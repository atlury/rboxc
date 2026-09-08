#!/usr/bin/env python3
"""Run reviewed, unchanged Bash scripts against their original expected output."""
# SPDX-License-Identifier: GPL-3.0-or-later
import importlib.util,json,os,re,shutil,subprocess,sys,tempfile
from pathlib import Path
from comparison_profile import ComparisonProfile,fingerprint

ROOT=Path(__file__).resolve().parents[1]
sys.path.insert(0,str(ROOT/'tests/gnu'))
spec=importlib.util.spec_from_file_location('reviewed',ROOT/'tests/gnu/reviewed-original.py')
runner=importlib.util.module_from_spec(spec);spec.loader.exec_module(runner)
profile=ComparisonProfile('bash-original',oracle=ROOT/'build/gnu-bash/bash',selections=True)
source=Path(json.loads((ROOT/'inventory/sources.json').read_text())['bash']['source'])/'tests'
manifest=ROOT/'inventory/bash-tests.json'
selected=[r for r in json.loads(manifest.read_text())['inputs'] if r['reviewed']]
if profile.options.commands:
    assert set(profile.options.commands)<={r['target'] for r in selected}
    selected=[r for r in selected if r['target'] in profile.options.commands]
assert selected
helpers={'sed':ROOT/'build/gnu-sed/sed/sed','grep':ROOT/'build/gnu-grep/src/grep'}
inputs={p:fingerprint(p) for p in [Path(__file__),manifest,profile.oracle,*helpers.values()]}
for row in selected:
    inputs[source/row['recipe']]=row['recipe_sha256']
    inputs.update({source/n:h for n,h in row['fixtures'].items()})
assert all(fingerprint(p)==h for p,h in inputs.items())
results=[]
for row in selected:
    outcomes={};name=row['target']
    for implementation,binary in [('gnu',profile.oracle),('rboxc',profile.binary)]:
        for instrument in (False,True):
            key=implementation+('-valgrind' if instrument else '')
            saved=profile.logs/(name+'-'+key);saved.mkdir()
            with tempfile.TemporaryDirectory(prefix='rboxc-bash-original-') as directory:
                work=Path(directory);(work/'exec').mkdir()
                alias=work/'exec/bash';alias.symlink_to(binary)
                for n,p in helpers.items():(work/'exec'/n).symlink_to(p if implementation=='gnu' else profile.binary)
                for n in row['fixtures']:shutil.copy2(source/n,work/n)
                argv=[str(alias),'--noprofile','--norc','./'+row['script']]
                if instrument:argv=['/usr/bin/valgrind','--leak-check=full','--show-leak-kinds=all',
                    '--track-fds=yes','--trace-children=yes','--log-file='+str(saved/'process-%p.log'),*argv]
                env={'PATH':str(work/'exec')+':/usr/bin:/bin','THIS_SH':str(alias),'HOME':directory,
                     'TMPDIR':directory,'LC_ALL':'C','LANGUAGE':'C','TZ':'UTC0'}
                done=subprocess.run(argv,cwd=work,env=env,stdin=subprocess.DEVNULL,stdout=subprocess.PIPE,
                    stderr=subprocess.PIPE if row['output_mode']=='stdout' else subprocess.STDOUT,timeout=180)
                actual=done.stdout
                # Match run-invert's original `grep -v '^expect'` filter.
                if row['output_mode']=='drop-expect':
                    actual=b''.join(line for line in actual.splitlines(keepends=True) if not line.startswith(b'expect'))
                (saved/'stdout').write_bytes(done.stdout);(saved/'stderr').write_bytes(done.stderr or b'')
                (saved/'actual').write_bytes(actual)
                logs=[]
                for log in sorted(saved.glob('process-*.log')):
                    text=log.read_text();pids=set(re.findall(r'^==([0-9]+)==',text,re.M));assert len(pids)==1
                    pid=pids.pop();parsed=runner.parse_memory_log(text,pid)
                    errors=list(re.finditer(r'ERROR SUMMARY:',text));fds=list(re.finditer(r'FILE DESCRIPTORS:',text))
                    images=list(re.finditer(r'^==[0-9]+== Command:',text,re.M))
                    complete=bool(errors) and len(errors)==len(fds) and (not images or errors[-1].start()>images[-1].start() and fds[-1].start()>images[-1].start())
                    clean=complete and parsed['errors']==0 and parsed['non_inherited_descriptors']==0 and not any(parsed['heap_bytes'].get(k,0) for k in ('definitely lost','indirectly lost','possibly lost'))
                    logs.append({'log':str(log.relative_to(ROOT)),'sha256':fingerprint(log),'pid':pid,'complete':complete,'clean':clean,**parsed})
                assert not instrument or logs
                outcomes[key]={'status':done.returncode,'expected_output_matches':actual==(source/row['expected']).read_bytes(),
                    'raw':{str(p.relative_to(ROOT)):fingerprint(p) for p in (saved/'stdout',saved/'stderr',saved/'actual')},
                    'memory':logs,'memory_clean':all(m['clean'] for m in logs) if instrument else None}
    passed=all(o['expected_output_matches'] and o['status']==outcomes['gnu']['status'] for o in outcomes.values()) and outcomes['rboxc-valgrind']['memory_clean']
    results.append({'selection':name,'pass':passed,'outcomes':outcomes})
    assert all(fingerprint(p)==h for p,h in inputs.items())
    report={**profile.metadata(),'inputs':{str(p):h for p,h in inputs.items()},
        'scope':'Reviewed complete Bash scripts execute unchanged in private directories. Compare their original expected files using the output mode/filter from the original run recipe, and compare exit status with GNU. Trace shell children and text helpers: native GNU Bash/sed/grep for the oracle, integrated Bash/sed/grep for the candidate. Preserve native GNU memory findings. Other original scripts remain open.',
        'complete':len(results)==len(selected),'planned_total':len(selected),'passed':sum(r['pass'] for r in results),'total':len(results),'results':results}
    profile.report.write_text(json.dumps(report,indent=2)+'\n')
    print('PASS' if passed else 'OPEN',name,flush=True)
raise SystemExit(report['passed']!=report['total'])
