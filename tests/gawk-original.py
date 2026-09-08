#!/usr/bin/env python3
"""Execute reviewed original GNU Gawk Make targets without changing recipes."""
# SPDX-License-Identifier: GPL-3.0-or-later
import importlib.util
import json
import os
from pathlib import Path
import re
import shlex
import shutil
import subprocess
import sys
import tempfile
from comparison_profile import ComparisonProfile,fingerprint

ROOT=Path(__file__).resolve().parents[1]
sys.path.insert(0,str(ROOT/'tests/gnu'))
spec=importlib.util.spec_from_file_location('reviewed',ROOT/'tests/gnu/reviewed-original.py')
runner=importlib.util.module_from_spec(spec);spec.loader.exec_module(runner)
profile=ComparisonProfile('gawk-original',oracle=ROOT/'build/gnu-gawk/gawk')
source=Path(json.loads((ROOT/'inventory/sources.json').read_text())['gawk']['source'])
manifest=json.loads((ROOT/'inventory/gawk-tests.json').read_text())
selected=[r for r in manifest['inputs'] if r['reviewed']]
assert selected and len({r['target'] for r in selected})==len(selected)
assert fingerprint(source/'test/Makefile.am')==manifest['registration_sha256']
makefile=ROOT/'build/gnu-gawk/test/Makefile'
helpers={'cmp':ROOT/'build/gnu-diffutils/src/cmp',
         'grep':ROOT/'build/gnu-grep/src/grep',
         'sed':ROOT/'build/gnu-sed/sed/sed',
         **{n:ROOT/'build/gnu-coreutils/src/coreutils' for n in ('rm','echo','od','tr')}}
inputs={p:fingerprint(p) for p in {makefile,source/'test/Makefile.am',source/'test/Makefile.in',
    Path(__file__),Path('/usr/bin/make'),Path('/bin/sh').resolve(),*helpers.values(),profile.oracle}}
for row in selected:
    inputs[source/row['path']]=row['sha256']
    inputs.update({source/'test'/n:h for n,h in row['fixtures'].items()})
assert all(fingerprint(p)==h for p,h in inputs.items())
for row in selected:
    if row.get('configured_recipe_sha256'):
        recipe=re.search('^'+re.escape(row['target'])+r':.*?(?=\n\S|\Z)',makefile.read_text(),re.M|re.S)[0]
        assert __import__('hashlib').sha256(recipe.encode()).hexdigest()==row['configured_recipe_sha256']
results=[]
for row in selected:
    name=row['target'];outcomes={}
    for implementation in ('gnu','rboxc'):
        for instrument in (False,True):
            key=implementation+('-valgrind' if instrument else '')
            saved=profile.logs/(name+'-'+key);saved.mkdir()
            with tempfile.TemporaryDirectory(prefix='rboxc-gawk-original-') as directory:
                work=Path(directory)
                for n in ('exec','deps','memory'): (work/n).mkdir()
                for n,p in helpers.items():(work/'deps'/n).symlink_to(p)
                (work/'exec/gawk').symlink_to(profile.oracle if implementation=='gnu' else profile.binary)
                argv=['gawk']
                if instrument:argv=['/usr/bin/valgrind','--leak-check=full','--show-leak-kinds=all',
                    '--track-fds=yes','--trace-children=yes','--log-file='+str(work/'memory/%p.log'),*argv]
                wrapper=work/'awk-wrapper'
                wrapper.write_text('#!/bin/sh\nexec '+shlex.join(argv)+' "$@"\n');wrapper.chmod(0o755)
                command=['/usr/bin/make','--no-print-directory','-f',str(makefile),
                    'top_builddir='+str(ROOT/'build/gnu-gawk'),'top_srcdir='+str(source),
                    'srcdir='+str(source/'test'),'AWKPROG='+str(wrapper),'CMP='+str(helpers['cmp']),name]
                env={'PATH':str(work/'exec')+':'+str(work/'deps')+':/usr/bin:/bin','HOME':directory,
                     'TMPDIR':directory,'LC_ALL':'C','LANGUAGE':'C','TZ':'UTC0'}
                done=subprocess.run(command,cwd=work,env=env,stdin=subprocess.DEVNULL,
                    stdout=subprocess.PIPE,stderr=subprocess.STDOUT,timeout=90)
                (saved/'driver.log').write_bytes(done.stdout)
                residual=work/('_'+name)
                if residual.exists():shutil.copy2(residual,saved/'actual-output')
                # GNU's recipes ignore Make failures but leave _TARGET on a
                # comparison failure. Require the success cleanup and target echo.
                passed=(done.returncode==0 and not residual.exists()
                        and name in done.stdout.decode(errors='replace').splitlines()
                        and b'Error ' not in done.stdout)
                shutil.copytree(work/'memory',saved/'memory')
                logs=[]
                for p in sorted((saved/'memory').glob('*.log')):
                    text=p.read_text();commands=re.findall(r'^==[0-9]+== Command: (.*)$',text,re.M)
                    assert len(commands)==1 and commands[0].split()[0]=='gawk'
                    logs.append({**runner.parse_memory_log(text,p.stem,exec_only=True),
                        'log':str(p.relative_to(ROOT)),'sha256':fingerprint(p)})
                clean=bool(logs) and all(m['complete_exec_log'] and m['errors']==0
                    and m['non_inherited_descriptors']==0 and not any(m['heap_bytes'].get(k,0)
                    for k in ('definitely lost','indirectly lost','possibly lost')) for m in logs)
                outcomes[key]={'status':done.returncode,'assertions_pass':passed,
                    'driver_log':str((saved/'driver.log').relative_to(ROOT)),'driver_log_sha256':fingerprint(saved/'driver.log'),
                    'memory':logs,'memory_clean':clean if instrument else None}
    passed=all(o['assertions_pass'] for o in outcomes.values()) and outcomes['rboxc-valgrind']['memory_clean']
    results.append({'selection':name,'source':row['path'],'source_sha256':row['sha256'],
                    'pass':passed,'outcomes':outcomes})
    assert all(fingerprint(p)==h for p,h in inputs.items())
    report={**profile.metadata(),'scope':'Reviewed unchanged GNU Make recipes compare original supplied programs and input against GNU expected output in private directories. Every selected Gawk invocation is instrumented; native findings are preserved.',
        'inputs':{str(p):h for p,h in inputs.items()},'driver_sha256':fingerprint(Path(__file__)),
        'planned_total':len(selected),'complete':len(results)==len(selected),
        'passed':sum(r['pass'] for r in results),'total':len(results),'results':results}
    profile.report.write_text(json.dumps(report,indent=2)+'\n')
    print('PASS' if passed else 'OPEN',name,flush=True)
raise SystemExit(report['passed']!=report['total'])
