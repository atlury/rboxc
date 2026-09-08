#!/usr/bin/env python3
"""Execute reviewed original GNU Patch scripts without changing assertions."""
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
profile=ComparisonProfile('patch-original',oracle=ROOT/'build/gnu-patch/src/patch')
source=Path(json.loads((ROOT/'inventory/sources.json').read_text())['patch']['source'])
manifest=json.loads((ROOT/'inventory/patch-tests.json').read_text())
selected=[r for r in manifest['inputs'] if r['reviewed']]
assert len(selected)==8
helpers={n:ROOT/'build/gnu-coreutils/src/coreutils' for n in ('cat','rm','echo','chmod','ls','cut','mktemp','touch','ln','mkdir','expr','seq')}
helpers.update(diff=ROOT/'build/gnu-diffutils/src/diff',sed=ROOT/'build/gnu-sed/sed/sed')
inputs={p:fingerprint(p) for p in {source/'tests/Makefile.am',source/'tests/test-lib.sh',Path(__file__),Path('/bin/sh').resolve(),*helpers.values(),profile.oracle}}
for row in selected:inputs[source/row['path']]=row['sha256']
results=[]
for row in selected:
    name=row['target'];outcomes={}
    for implementation in ('gnu','rboxc'):
        for instrument in (False,True):
            key=implementation+('-valgrind' if instrument else '')
            saved=profile.logs/(name+'-'+key);saved.mkdir()
            with tempfile.TemporaryDirectory(prefix='rboxc-patch-original-') as directory:
                work=Path(directory)
                for n in ('exec','deps','memory','tests'): (work/n).mkdir()
                for n,p in helpers.items():(work/'deps'/n).symlink_to(p)
                (work/'exec/patch').symlink_to(profile.oracle if implementation=='gnu' else profile.binary)
                argv=['patch']
                if instrument:argv=['/usr/bin/valgrind','--leak-check=full','--show-leak-kinds=all',
                    '--track-fds=yes','--trace-children=yes','--log-file='+str(work/'memory/%p.log'),*argv]
                wrapper=work/'awk-wrapper'
                wrapper.write_text('#!/bin/sh\nexec '+shlex.join(argv)+' "$@"\n');wrapper.chmod(0o755)
                command=['/bin/sh',str(source/row['path'])]
                env={'PATH':str(work/'exec')+':'+str(work/'deps')+':/usr/bin:/bin','HOME':directory,
                     'TMPDIR':directory,'LC_ALL':'C','LANGUAGE':'C','TZ':'UTC0',
                     'srcdir':str(source/'tests'),'abs_top_builddir':directory,'PATCH':str(wrapper)}
                done=subprocess.run(command,cwd=work,env=env,stdin=subprocess.DEVNULL,
                    stdout=subprocess.PIPE,stderr=subprocess.STDOUT,timeout=90)
                (saved/'driver.log').write_bytes(done.stdout)
                counts=re.findall(rb'(\d+) tests \((\d+) passed, (\d+) failed\)',done.stdout)
                passed=(done.returncode==0 and len(counts)==1 and int(counts[0][0])>0
                        and counts[0][0]==counts[0][1] and int(counts[0][2])==0)
                shutil.copytree(work/'memory',saved/'memory')
                logs=[]
                for p in sorted((saved/'memory').glob('*.log')):
                    text=p.read_text();commands=re.findall(r'^==[0-9]+== Command: (.*)$',text,re.M)
                    assert len(commands)==1 and commands[0].split()[0]=='patch'
                    logs.append({**runner.parse_memory_log(text,p.stem,exec_only=True),
                        'log':str(p.relative_to(ROOT)),'sha256':fingerprint(p)})
                clean=bool(logs) and all(m['complete_exec_log'] and m['errors']==0
                    and m['non_inherited_descriptors']==0 and not any(m['heap_bytes'].get(k,0)
                    for k in ('definitely lost','indirectly lost','possibly lost')) for m in logs)
                outcomes[key]={'status':done.returncode,'assertions_pass':passed,'assertion_counts':[[int(n) for n in row] for row in counts],
                    'driver_log':str((saved/'driver.log').relative_to(ROOT)),'driver_log_sha256':fingerprint(saved/'driver.log'),
                    'memory':logs,'memory_clean':clean if instrument else None}
    passed=all(o['assertions_pass'] for o in outcomes.values()) and outcomes['rboxc-valgrind']['memory_clean']
    results.append({'selection':name,'source':row['path'],'source_sha256':row['sha256'],
                    'pass':passed,'outcomes':outcomes})
    assert all(fingerprint(p)==h for p,h in inputs.items())
    report={**profile.metadata(),'scope':'Eight reviewed unchanged GNU Patch scripts exercise edits, backups, file modes, empty files, unmatched input and whitespace. Every Patch invocation is instrumented; original assertion totals and native findings are preserved.',
        'inputs':{str(p):h for p,h in inputs.items()},'driver_sha256':fingerprint(Path(__file__)),
        'planned_total':len(selected),'complete':len(results)==len(selected),
        'passed':sum(r['pass'] for r in results),'total':len(results),'results':results}
    profile.report.write_text(json.dumps(report,indent=2)+'\n')
    print('PASS' if passed else 'OPEN',name,flush=True)
raise SystemExit(report['passed']!=report['total'])
