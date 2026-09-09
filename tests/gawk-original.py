#!/usr/bin/env python3
"""Execute reviewed original GNU Gawk Make targets without changing recipes."""
# SPDX-License-Identifier: GPL-3.0-or-later
from concurrent.futures import ThreadPoolExecutor
from collections import Counter
import importlib.util
import json
import os
from pathlib import Path
import re
import shlex
import shutil
import signal
import time
import subprocess
import sys
import tempfile
from comparison_profile import ComparisonProfile,fingerprint
from gawk_child_profile import canonical_child

ROOT=Path(__file__).resolve().parents[1]
sys.path.insert(0,str(ROOT/'tests/gnu'))
spec=importlib.util.spec_from_file_location('reviewed',ROOT/'tests/gnu/reviewed-original.py')
runner=importlib.util.module_from_spec(spec);spec.loader.exec_module(runner)
profile=ComparisonProfile('gawk-original',oracle=ROOT/'build/gnu-gawk/gawk',selections=True)
source=Path(json.loads((ROOT/'inventory/sources.json').read_text())['gawk']['source'])
manifest=json.loads((ROOT/'inventory/gawk-tests.json').read_text())
selected=[r for r in manifest['inputs'] if r['reviewed']]
requested=set(profile.options.commands)
assert requested<={r['target'] for r in selected}
if requested:selected=[r for r in selected if r['target'] in requested]
assert selected and len({r['target'] for r in selected})==len(selected)
assert fingerprint(source/'test/Makefile.am')==manifest['registration_sha256']
makefile=ROOT/'build/gnu-gawk/test/Makefile'
helpers={'cmp':ROOT/'build/gnu-diffutils/src/cmp',
         'awk':ROOT/'build/gnu-gawk/gawk',
         'grep':ROOT/'build/gnu-grep/src/grep',
         'egrep':ROOT/'build/gnu-grep/src/egrep',
         'sed':ROOT/'build/gnu-sed/sed/sed',
         **{n:ROOT/'build/gnu-coreutils/src/coreutils' for n in ('rm','echo','od','tr','cp','sort','ls','stat','uname','basename','chmod','cat','sleep','wc','mkdir','touch')}}
helper_profile_path=ROOT/'evidence/gawk-sort-helper.json'
helper_profile=json.loads(helper_profile_path.read_text())
helpers['sort']=Path(helper_profile['binary'])
assert fingerprint(helpers['sort'])==helper_profile['binary_sha256']
cat_profile_path=ROOT/'evidence/gawk-cat-helper.json'
cat_profile=json.loads(cat_profile_path.read_text())
helpers['cat']=Path(cat_profile['binary'])
assert fingerprint(helpers['cat'])==cat_profile['binary_sha256']
inputs={p:fingerprint(p) for p in {makefile,source/'test/Makefile.am',source/'test/Makefile.in',
    Path(__file__),ROOT/'tests/gawk_child_profile.py',Path('/usr/bin/make'),Path('/bin/bash').resolve(),Path('/bin/sh').resolve(),*helpers.values(),profile.oracle}}
inputs[helper_profile_path]=fingerprint(helper_profile_path)
inputs.update({Path(p):h for p,h in helper_profile['inputs'].items()})
inputs[cat_profile_path]=fingerprint(cat_profile_path)
inputs.update({Path(p):h for p,h in cat_profile['inputs'].items()})
for row in selected:
    inputs[source/row['path']]=row['sha256']
    inputs.update({source/'test'/n:h for n,h in row['fixtures'].items()})
    inputs.update({Path(v['path']):v['sha256'] for v in row.get('working_files',{}).values()})
    for name, entry in row.get('child_dependencies', {}).items():
        assert name in helpers and str(helpers[name]) == entry['path']
        assert fingerprint(helpers[name]) == entry['sha256']
        inputs[helpers[name]] = entry['sha256']
    extension=row.get('extension_profile')
    if extension:
        inputs.update({Path(p):h for p,h in extension['inputs'].items()})
        inputs.update({Path(v['path']):v['sha256'] for v in extension['libraries'].values()})
locale_profiles={}
for row in selected:
    if row.get('locale_profile'):
        path=ROOT/row['locale_profile'];data=json.loads(path.read_text());inputs[path]=fingerprint(path)
        base=Path(data['runtime_path'])
        for name,h in data['files'].items():inputs[base/data['name']/name]=h
        locale_profiles[row['target']]=str(base)
assert all(fingerprint(p)==h for p,h in inputs.items())
for row in selected:
    if row.get('configured_recipe_sha256'):
        recipe=re.search('^'+re.escape(row['target'])+r':.*?(?=\n\S|\Z)',makefile.read_text(),re.M|re.S)[0]
        assert __import__('hashlib').sha256(recipe.encode()).hexdigest()==row['configured_recipe_sha256']
def run_selection(row):
    name=row['target'];outcomes={}
    for implementation in ('gnu','rboxc'):
        for instrument in (False,True):
            key=implementation+('-valgrind' if instrument else '')
            saved=profile.logs/(name+'-'+key);saved.mkdir()
            with tempfile.TemporaryDirectory(prefix='rboxc-gawk-original-') as directory:
                work=Path(directory)
                if row.get('extension_profile'):
                    # Preserve GNU's relative AWKLIBPATH=../extension/.libs.
                    libraries=work/'extension/.libs';libraries.mkdir(parents=True)
                    for library,entry in row['extension_profile']['libraries'].items():
                        assert re.fullmatch(r'[a-z0-9_]+\.so',library)
                        (libraries/library).symlink_to(entry['path'])
                    work=work/'test';work.mkdir()
                for n in ('exec','deps','memory'): (work/n).mkdir()
                for filename,entry in row.get('working_files',{}).items():
                    assert re.fullmatch(r'[A-Za-z0-9_][A-Za-z0-9_.-]*',filename)
                    shutil.copy2(entry['path'],work/filename)
                    assert fingerprint(work/filename)==entry['sha256']
                for n,p in helpers.items():(work/'deps'/n).symlink_to(p)
                for n,entry in row.get('child_dependencies',{}).items():
                    assert (work/'deps'/n).resolve() == Path(entry['path']).resolve()
                (work/'exec/gawk').symlink_to(profile.oracle if implementation=='gnu' else profile.binary)
                argv=['gawk']
                if instrument:argv=['/usr/bin/valgrind','--leak-check=full','--show-leak-kinds=all',
                    '--track-fds=yes','--trace-children=yes',
                    '--log-fd=9' if row.get('self_exec_images') else '--log-file='+str(work/'memory/%p.log'),*argv]
                if instrument and row.get('self_exec_images'):
                    argv[1:1]=['--exit-on-first-error=yes','--error-exitcode=125']
                wrapper=work/'awk-wrapper'
                append_log = ('exec 9>>'+shlex.quote(str(work/'memory'))+'/"$$.log"\n') if instrument and row.get('self_exec_images') else ''
                wrapper.write_text('#!/bin/sh\n'+append_log+'exec '+shlex.join(argv)+' "$@"\n');wrapper.chmod(0o755)
                command=['/usr/bin/make','--no-print-directory','-f',str(makefile),
                    'top_builddir='+str(ROOT/'build/gnu-gawk'),'top_srcdir='+str(source),
                    'srcdir='+str(source/'test'),'AWKPROG='+str(wrapper),'CMP='+str(helpers['cmp']),name]
                env={'PATH':str(work/'exec')+':'+str(work/'deps')+':/usr/bin:/bin','HOME':directory,
                     'TMPDIR':directory,'LC_ALL':'C','LANGUAGE':'C','TZ':'UTC0'}
                if name in locale_profiles:env['LOCPATH']=locale_profiles[name]
                process=subprocess.Popen(command,cwd=work,env=env,stdin=subprocess.DEVNULL,
                    stdout=subprocess.PIPE,stderr=subprocess.STDOUT,start_new_session=True)
                timed_out=False
                try:
                    output,_=process.communicate(timeout=90)
                except subprocess.TimeoutExpired:
                    timed_out=True
                    os.killpg(process.pid,signal.SIGKILL)
                    output,_=process.communicate()
                done=subprocess.CompletedProcess(command,process.returncode,output)
                waited_children=set(); child_wait_timeout=False
                deadline=time.monotonic()+15
                while True:
                    active=set()
                    for entry in Path('/proc').iterdir():
                        if not entry.name.isdecimal():continue
                        try: fields=(entry/'stat').read_text().rpartition(') ')[2].split()
                        except (FileNotFoundError,ProcessLookupError):continue
                        if len(fields)>2 and int(fields[2])==process.pid and fields[0]!='Z':active.add(int(entry.name))
                    if not active:break
                    waited_children.update(active)
                    if time.monotonic()>=deadline:
                        assert not child_wait_timeout, 'private test children did not finish after termination'
                        child_wait_timeout=True
                        os.killpg(process.pid,signal.SIGKILL)
                        deadline=time.monotonic()+5
                    time.sleep(0.05)
                (saved/'driver.log').write_bytes(done.stdout)
                residual=work/('_'+name)
                if residual.exists():shutil.copy2(residual,saved/'actual-output')
                # Keep multi-output recipe failures as well as the conventional _TARGET.
                for output_file in work.glob('_*'):
                    if output_file.is_file():
                        (saved/'outputs').mkdir(exist_ok=True)
                        shutil.copy2(output_file,saved/'outputs'/output_file.name)
                # GNU's recipes ignore Make failures but leave _TARGET on a
                # comparison failure. Require the success cleanup and target echo.
                passed=(not timed_out and not child_wait_timeout and done.returncode==0 and not residual.exists()
                        and name in done.stdout.decode(errors='replace').splitlines()
                        and b'Error ' not in done.stdout)
                baseline_output=row.get('expected_baseline_output')
                baseline_matches=baseline_output is not None and not passed and done.returncode==0 and residual.exists() and residual.read_bytes()==baseline_output.encode() and fingerprint(saved/'driver.log')==row['expected_baseline_driver_sha256']
                shutil.copytree(work/'memory',saved/'memory')
                logs=[]
                for p in sorted((saved/'memory').glob('*.log')):
                    text=p.read_text();commands=re.findall(r'^==[0-9]+== Command: (.*)$',text,re.M)
                    if row.get('self_exec_images'):
                        assert row['self_exec_images']==2 and len(commands)==2
                        assert commands[0].startswith('gawk ')
                        assert commands[1]==str(work/'exec/gawk')+commands[0][4:]
                    else:
                        assert len(commands)==1
                    command=commands[0]
                    role='gawk' if command.split()[0]=='gawk' else 'child-dependency'
                    canonical = command if role=='gawk' else canonical_child(command, row.get('child_commands',{}), str(work), row.get('child_dependencies'))
                    logs.append({**runner.parse_memory_log(text,p.stem,exec_only=True),
                        'log':str(p.relative_to(ROOT)),'sha256':fingerprint(p),
                        'command':command,'commands':commands,'canonical_command':canonical,'role':role})
                if instrument:
                    if row.get('self_exec_images'):
                        assert len(logs)==1 and logs[0]['exec_images']==row['self_exec_images']
                    assert any(m['role']=='gawk' for m in logs)
                    assert Counter(m['canonical_command'] for m in logs if m['role']=='child-dependency')==row.get('child_commands',{})
                clean=bool(logs) and all(m['complete_exec_log'] and m['errors']==0
                    and m['non_inherited_descriptors']==0 and not any(m['heap_bytes'].get(k,0)
                    for k in ('definitely lost','indirectly lost','possibly lost')) for m in logs)
                outcomes[key]={'process_group':process.pid,'timed_out':timed_out,'child_wait_timeout':child_wait_timeout,'waited_children':sorted(waited_children),'private_work_directory':str(work),'status':done.returncode,'assertions_pass':passed,
                    'baseline_failure_matches':baseline_matches,'actual_output':str((saved/'actual-output').relative_to(ROOT)) if residual.exists() else None,'actual_output_sha256':fingerprint(saved/'actual-output') if residual.exists() else None,
                    'driver_log':str((saved/'driver.log').relative_to(ROOT)),'driver_log_sha256':fingerprint(saved/'driver.log'),
                    'memory':logs,'memory_clean':clean if instrument else None}
    passed=all(o['assertions_pass'] for o in outcomes.values()) and outcomes['rboxc-valgrind']['memory_clean']
    baseline_matches=bool(row.get('expected_baseline_output')) and all(o['baseline_failure_matches'] for o in outcomes.values()) and outcomes['rboxc-valgrind']['memory_clean']
    return {'self_exec_images':row.get('self_exec_images'),'child_dependencies':row.get('child_dependencies',{}),'baseline_failure_matches':baseline_matches,'child_commands':row.get('child_commands',{}),'locale_profile':row.get('locale_profile'),'extension_profile':row.get('extension_profile'),'working_files':row.get('working_files',{}),'selection':name,'source':row['path'],'source_sha256':row['sha256'],
            'pass':passed,'outcomes':outcomes}

results=[]
with ThreadPoolExecutor(max_workers=4) as pool:
    for result in pool.map(run_selection,selected):
        results.append(result)
        assert all(fingerprint(p)==h for p,h in inputs.items())
        report={**profile.metadata(),'scope':'Reviewed unchanged GNU Make recipes compare original supplied programs and input against GNU expected output in private directories. Every selected Gawk invocation is instrumented; native findings are preserved.',
            'inputs':{str(p):h for p,h in inputs.items()},'driver_sha256':fingerprint(Path(__file__)),
            'selected_targets':sorted(requested),
            'baseline_failures_matched':sum(r['baseline_failure_matches'] for r in results),'matched':sum(r['pass'] or r['baseline_failure_matches'] for r in results),'parallel_selections':4,'planned_total':len(selected),'complete':len(results)==len(selected),
            'passed':sum(r['pass'] for r in results),'total':len(results),'results':results}
        profile.report.write_text(json.dumps(report,indent=2)+'\n')
        print('PASS' if result['pass'] else 'BASELINE-FAILURE-MATCH' if result['baseline_failure_matches'] else 'OPEN',result['selection'],flush=True)
raise SystemExit(report['matched']!=report['total'])
