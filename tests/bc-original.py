#!/usr/bin/env python3
"""Compare reviewed original GNU BC arithmetic inputs and audit memory."""
# SPDX-License-Identifier: GPL-3.0-or-later
import importlib.util
import json
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import time
from comparison_profile import ComparisonProfile, fingerprint

ROOT=Path(__file__).resolve().parents[1]
sys.path.insert(0,str(ROOT/'tests/gnu'))
spec=importlib.util.spec_from_file_location('reviewed',ROOT/'tests/gnu/reviewed-original.py')
runner=importlib.util.module_from_spec(spec);spec.loader.exec_module(runner)
profile=ComparisonProfile('bc-original',oracle=ROOT/'build/gnu-bc/bc/bc',selections=True)
manifest_path=ROOT/'inventory/bc-tests.json'
manifest=json.loads(manifest_path.read_text());manifest_hash=fingerprint(manifest_path)
source=Path(json.loads((ROOT/'inventory/sources.json').read_text())['bc']['source'])
for name,digest in manifest['registration_sha256'].items():assert fingerprint(source/name)==digest
selected=set(profile.options.commands)
rows=[r for r in manifest['inputs'] if r['reviewed'] and r['state']=='reviewed']
assert selected<={Path(r['path']).name for r in rows}
driver_hash=fingerprint(Path(__file__));results=[]
for index,row in enumerate(rows):
    if selected and Path(row['path']).name not in selected:continue
    original=source/row['path'];assert fingerprint(original)==row['sha256']
    outcomes={}
    for implementation in ('gnu','rboxc'):
        for instrument in (False,True):
            key=implementation+('-valgrind' if instrument else '')
            with tempfile.TemporaryDirectory(prefix='rboxc-bc-original-') as directory:
                work=Path(directory);(work/'memory').mkdir()
                (work/'bc').symlink_to(profile.oracle if implementation=='gnu' else profile.binary)
                shutil.copy2(original,work/'input.bc')
                command=[str(work/'bc'),*row['arguments'],'input.bc']
                if instrument:command=['/usr/bin/valgrind','--leak-check=full','--show-leak-kinds=all','--track-fds=yes','--trace-children=yes','--log-file='+str(work/'memory/%p.log'),*command]
                started=time.monotonic()
                done=subprocess.run(command,cwd=work,input=row.get('stdin','').encode(),capture_output=True,
                    env={'PATH':'/usr/bin:/bin','HOME':directory,'LC_ALL':'C','TZ':'UTC0'},timeout=row['timeout_seconds'])
                outcome={'status':done.returncode,'stdout':done.stdout.hex(),'stderr':done.stderr.hex(),
                         'elapsed_seconds':time.monotonic()-started,'input_preserved':fingerprint(work/'input.bc')==row['sha256']}
                if instrument:
                    saved=profile.logs/f'{index:02}-{key}-memory';shutil.copytree(work/'memory',saved)
                    outcome['memory']=[{**runner.parse_memory_log(p.read_text(),p.stem,exec_only=True),'log':str(p.relative_to(ROOT)),'sha256':fingerprint(p)} for p in sorted(saved.glob('*.log'))]
                outcomes[key]=outcome
    equivalent=all(all(r[f]==outcomes['gnu'][f] for f in ('status','stdout','stderr')) for r in outcomes.values())
    expected=all(r['status']==0 and r['stderr']=='' and r['input_preserved'] and bool(r['stdout']) for r in outcomes.values())
    if 'expected_stdout' in row:expected=expected and all(r['stdout']==row['expected_stdout'].encode().hex() for r in outcomes.values())
    logs=outcomes['rboxc-valgrind']['memory']
    clean=bool(logs) and all(m['complete_exec_log'] and m['errors']==0 and m['non_inherited_descriptors']==0 and not any(m['heap_bytes'].get(k,0) for k in ('definitely lost','indirectly lost','possibly lost')) for m in logs)
    results.append({**row,'pass':equivalent and expected and clean,'equivalent':equivalent,'expected_output_pass':expected,'memory_clean':clean,'outcomes':outcomes})
    print('PASS' if results[-1]['pass'] else 'OPEN',row['path'],flush=True)
    assert fingerprint(Path(__file__))==driver_hash and fingerprint(manifest_path)==manifest_hash
    report={'scope':'Full reviewed original arithmetic inputs, with the declared invocation for the definition-only testfn file. Timing wrapper is accounted for separately. Output equivalence does not certify mathematical accuracy beyond original assertions.',
            **profile.metadata(),'driver_sha256':driver_hash,'manifest_sha256':manifest_hash,
            'passed':sum(r['pass'] for r in results),'total':len(results),'selected_inputs':sorted(selected),
            'registered_runtime_tests':0,'reviewed_inputs':len(rows),'results':results}
    profile.report.write_text(json.dumps(report,indent=2)+'\n')
raise SystemExit(any(not r['pass'] for r in results))
