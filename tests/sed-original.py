#!/usr/bin/env python3
"""Run individually reviewed, unchanged GNU Sed original shell assertions."""
# SPDX-License-Identifier: GPL-3.0-or-later
import importlib.util
import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import sys
import tempfile
from comparison_profile import ComparisonProfile, fingerprint
from sed_dependencies import prepare

ROOT=Path(__file__).resolve().parents[1]
sys.path.insert(0,str(ROOT/'tests/gnu'))
spec=importlib.util.spec_from_file_location('reviewed',ROOT/'tests/gnu/reviewed-original.py')
runner=importlib.util.module_from_spec(spec);spec.loader.exec_module(runner)
profile=ComparisonProfile('sed-original',oracle=ROOT/'build/gnu-sed/sed/sed',selections=True)
pin=json.loads((ROOT/'inventory/sources.json').read_text())['sed'];source=Path(pin['source'])
manifest=json.loads((ROOT/'inventory/sed-tests.json').read_text())
assert fingerprint(source/manifest['registration']['path'])==manifest['registration']['sha256']
driver_sha256=fingerprint(Path(__file__))
native_dependencies=prepare()
helpers,prerequisite_environment,prerequisites=native_dependencies
helper_mode=os.environ.get('RBOXC_SED_MULTICALL_HELPERS','0')
assert helper_mode in ('0','1')
if helper_mode=='1':
    # An additional interoperation profile uses the same translated Coreutils
    # helpers for both Sed implementations. Keep the native-helper run separate.
    helpers={**helpers}
    prerequisite_environment={**prerequisite_environment}
    prerequisites={**prerequisites}
    coreutils={'path':str(profile.binary),'sha256':profile.binary_sha256,
               'commands':['cat','touch','sleep','dd'],
               'scope':'Integrated rboxc Coreutils test dependencies, shared by GNU and rboxc Sed'}
    for name in coreutils['commands']:
        helpers[name]={'path':coreutils['path'],'sha256':coreutils['sha256'],'scope':coreutils['scope']}
    prerequisites['native_coreutils']=prerequisites['coreutils']
    prerequisites['coreutils']=coreutils
selected=set(profile.options.commands)
assert selected<={Path(r['script']).name for r in manifest['scripts'] if r['reviewed']}
results=[]
for index,row in enumerate(manifest['scripts']):
    script=source/row['script'];assert fingerprint(script)==row['source_sha256']
    if not row['reviewed'] or (selected and script.name not in selected):continue
    for name,expected in row.get('fixture_sha256',{}).items():
        assert fingerprint(source/'testsuite'/name)==expected
    if script.suffix=='.pl':
        assert row.get('full_suite') and row.get('expected_case_count')
    outcomes={}
    for implementation in ('gnu','rboxc'):
        for instrument in (False,True):
            key=implementation+('-valgrind' if instrument else '')
            with tempfile.TemporaryDirectory(prefix='rboxc-sed-original-') as directory:
                work=Path(directory)
                for sub in ('sed','real','memory','testsuite'):(work/sub).mkdir()
                binary=profile.oracle if implementation=='gnu' else profile.binary
                credentials=row.get('credentials')
                if credentials:
                    assert os.geteuid()==0
                    shutil.copy2(binary,work/'real/sed')
                    assert fingerprint(work/'real/sed')==fingerprint(binary)
                else:(work/'real/sed').symlink_to(binary)
                for name,helper in helpers.items():
                    if credentials:
                        shutil.copy2(helper['path'],work/'testsuite'/name)
                        assert fingerprint(work/'testsuite'/name)==helper['sha256']
                    else:(work/'testsuite'/name).symlink_to(helper['path'])
                if instrument:
                    wrapper=work/'sed/sed'
                    wrapper.write_text('#!/bin/sh\nPATH='+str(work/'real')+':$PATH\nexport PATH\nexec /usr/bin/valgrind --leak-check=full --show-leak-kinds=all --track-fds=yes --trace-children=yes --log-file='+str(work/'memory/%p.log')+' sed "$@"\n')
                    wrapper.chmod(0o755)
                else:(work/'sed/sed').symlink_to(work/'real/sed')
                if script.suffix=='.pl':
                    command=['/usr/bin/perl','-I'+str(source/'testsuite'),'-MCuSkip','-MCoreutils',
                             '-e',runner.PERL_SELECTION,str(script)]
                else:command=['/bin/bash','-c','exec 9>&2; exec /bin/bash "$1"','sed-test',str(script)]
                if credentials:
                    for path in [work,*work.rglob('*')]:os.chown(path,credentials['uid'],credentials['gid'],follow_symlinks=False)
                    command=['/usr/bin/setpriv','--reuid='+str(credentials['uid']),'--regid='+str(credentials['gid']),'--clear-groups',*command]
                done=subprocess.run(command,
                    cwd=work,stdin=subprocess.DEVNULL,capture_output=True,timeout=row.get('timeout_seconds',300),
                    env={**prerequisite_environment,'RBOXC_FULL_SUITE':'1' if row.get('full_suite') else '',
                         'RUN_VERY_EXPENSIVE_TESTS':'yes' if row.get('requires_very_expensive_tests') else 'no',
                         'RBOXC_APPROVED_CASES':'','PATH':str(work/'sed')+':'+str(work/'testsuite')+':/usr/bin:/bin','HOME':directory,'TMPDIR':directory,
                         'LC_ALL':'C','LANGUAGE':'C','TZ':'UTC0','srcdir':str(source),
                         'top_srcdir':str(source),'abs_top_srcdir':str(source),'abs_srcdir':str(source),
                         'abs_top_builddir':directory,'VERSION':pin['version'],'PACKAGE_BUGREPORT':'bug-sed@gnu.org',
                         'built_programs':'sed','PERL':'/usr/bin/perl','SHELL':'/bin/bash'})
                log=profile.logs/f'{index:02}-{key}.log';log.write_bytes(done.stdout+done.stderr)
                outcome={'status':done.returncode,'stdout':done.stdout.hex(),'stderr':done.stderr.hex(),
                         'log':str(log.relative_to(ROOT)),'log_sha256':fingerprint(log)}
                if script.suffix=='.pl':
                    counts=re.findall(rb'RBOXC_SELECTION ([0-9]+) of ([0-9]+)',done.stdout)
                    count=str(row['expected_case_count']).encode()
                    outcome['case_count_pass']=counts==[(count,count)]
                    outcome['case_count']=row['expected_case_count'] if outcome['case_count_pass'] else None
                if instrument:
                    saved=profile.logs/f'{index:02}-{key}-memory';shutil.copytree(work/'memory',saved)
                    outcome['memory']=[{**runner.parse_memory_log(p.read_text(),p.stem,exec_only=True),'log':str(p.relative_to(ROOT)),'sha256':fingerprint(p)} for p in sorted(saved.glob('*.log'))]
                outcomes[key]=outcome
    native_pass=all(outcomes[n]['status']==0 and outcomes[n].get('case_count_pass',True) for n in ('gnu','rboxc'))
    assertions_pass=native_pass and all(r['status']==0 and r.get('case_count_pass',True) for r in outcomes.values())
    logs=outcomes['rboxc-valgrind']['memory']
    clean=bool(logs) and all(m['complete_exec_log'] and m['errors']==0 and m['non_inherited_descriptors']==0 and not any(m['heap_bytes'].get(k,0) for k in ('definitely lost','indirectly lost','possibly lost')) for m in logs)
    skipped=all(r['status']==77 for r in outcomes.values())
    passed=assertions_pass and clean
    state='passed' if passed else 'prerequisite_skip' if skipped else 'assertions_passed_memory_open' if assertions_pass else 'assertions_open'
    results.append({**row,'pass':passed,'native_pass':native_pass,'assertions_pass':assertions_pass,'memory_clean':clean,'skipped':skipped,'state':state,'outcomes':outcomes})
    print(state.upper(),row['script'],flush=True)
    report={'scope':'Individually reviewed unchanged original shell assertions on the pinned native GNU and Rust candidate, with strict final-exec Valgrind checks.',**profile.metadata(),'driver_sha256':driver_sha256,'passed':sum(r['pass'] for r in results),'native_passed':sum(r['native_pass'] for r in results),'total':len(results),'state_counts':{s:sum(r['state']==s for r in results) for s in sorted({r['state'] for r in results})},'selected_scripts':sorted(selected),'registered_original_scripts':len(manifest['scripts']),'remaining':[r for r in manifest['scripts'] if not r['reviewed']],'results':results}
    assert fingerprint(Path(__file__))==driver_sha256
    assert prepare()==native_dependencies
    report['prerequisites']=prerequisites
    profile.report.write_text(json.dumps(report,indent=2)+'\n')
raise SystemExit(any(not r['pass'] for r in results))
