#!/usr/bin/env python3
"""Audit standard pipe ownership against unchanged originals and shared checks."""
# SPDX-License-Identifier: GPL-3.0-or-later
import hashlib,importlib.util,json,re,subprocess,sys
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
sys.path.insert(0,str(ROOT/'tests/gnu'))
spec=importlib.util.spec_from_file_location('reviewed',ROOT/'tests/gnu/reviewed-original.py')
runner=importlib.util.module_from_spec(spec);spec.loader.exec_module(runner)
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
target=ROOT/'evidence/bash-pipe-owner-validation.json';assert not target.exists()
binary=ROOT/'target/bash-pipe-owner-final-candidate/release/rboxc';digest=sha(binary)
assert sha(ROOT/'target/bash-pipe-owner-final-repro/release/rboxc')==digest
assert len(subprocess.check_output([binary,'--list']).splitlines())==187
assert sha(ROOT/'target/release/rboxc')=='8979835878e7dbe575440b5d0c7c86a01c75782ff5ddc735707ccada50e88e86'
raw={};reports={}
def pin(path,expected=None):
    path=Path(path)
    if not path.is_absolute():path=ROOT/path
    actual=sha(path)
    if expected is not None:assert actual==expected,str(path)
    raw[str(path.relative_to(ROOT)) if path.is_relative_to(ROOT) else str(path)]=actual
    return actual
def memory(record,strict):
    p=ROOT/record['log'];pin(p,record['sha256']);text=p.read_text()
    pids=set(re.findall(r'^==([0-9]+)==',text,re.M));assert len(pids)==1
    parsed=runner.parse_memory_log(text,pids.pop())
    assert all(record[k]==v for k,v in parsed.items())
    if strict:
        assert record['complete'] and record['clean'] and parsed['errors']==0
        assert parsed['non_inherited_descriptors']==0
        assert not any(parsed['heap_bytes'].get(k,0) for k in ('definitely lost','indirectly lost','possibly lost'))
for name,total in [('behavior',44),('adapters',69),('smoke',428),('final-dispatch',11),('final-original',1)]:
    path=ROOT/f'evidence/bash-pipe-owner-{name}.json';r=json.loads(path.read_text())
    assert r.get('complete',True) and r['passed']==r['total']==len(r['results'])==total and all(x['pass'] for x in r['results'])
    assert r['binary_sha256']==digest
    for result in r['results']:
        for key,outcome in result.get('outcomes',{}).items():
            for m in outcome.get('memory',[]):memory(m,key=='rboxc-valgrind')
        if result.get('log'):pin(result['log'])
    reports[str(path.relative_to(ROOT))]=pin(path)
audit_path=ROOT/'evidence/bash-pipe-owner-original-validation.json';audit=json.loads(audit_path.read_text())
assert audit['original_sha256']==reports['evidence/bash-pipe-owner-final-original.json']
assert audit['strict_original_passes']==1 and len(audit['processes'])==50
assert not audit['open_findings'] and not audit['open_processes']
for p,h in audit['raw'].items():pin(p,h)
reports[str(audit_path.relative_to(ROOT))]=pin(audit_path)
contract_path=ROOT/'evidence/bash-pipe-owner-lifetime-contract.json';r=json.loads(contract_path.read_text())
assert r['passed']==r['total']==len(r['results'])==17
pin('tests/bash-pipe-contract.py',r['driver_sha256']);pin('src/shell_arguments.c',r['runtime_source_sha256']);pin(r['harness'],r['harness_sha256'])
for case in r['results']:
    assert case['pass']
    for o in case['outcomes']:
        assert o['pass'] and o['status']==0 and not o['stdout'] and not o['stderr']
        if o['instrumented']:
            p=ROOT/o['log'];pin(p,o['sha256']);text=p.read_text();pids=set(re.findall(r'^==([0-9]+)==',text,re.M));assert len(pids)==1
            parsed=runner.parse_memory_log(text,pids.pop(),exec_only=True);assert parsed==o['memory'] and o['memory_clean']
            assert parsed['complete_exec_log'] and parsed['errors']==0 and parsed['non_inherited_descriptors']==0
reports[str(contract_path.relative_to(ROOT))]=pin(contract_path)
a=json.loads((ROOT/'evidence/bash-before-pipe-owner-link.json').read_text())['helper_inputs']
b=json.loads((ROOT/'evidence/bash-link.json').read_text())['helper_inputs'];assert a.keys()==b.keys()
changed=[p for p in a if a[p]!=b[p]]
assert changed==['build/helpers/bash-002-general.o','build/helpers/bash-006-execute_cmd.o','build/helpers/bash-012-jobs.o','build/helpers/bash-013-subst.o','build/helpers/bash-036-redir.o']
for p,h in b.items():pin(p,h)
for p in ['scripts/entry_provider_helpers.py','evidence/raw/bash-before-pipe-owner-source.c','evidence/raw/bash-pipe-owner-initial-source.c','evidence/raw/bash-before-pipe-owner-helper-driver.py']:pin(p)
target.write_text(json.dumps({'scope':'Bash pipe references now record newly owned standard slots; normal close clears their ownership. Unchanged posixexp2 passes all original assertions with 50 clean candidate process logs. Seventeen isolated descriptor lifecycle contracts and shared checks pass. Other Bash originals retain their recorded candidate hashes; full Bash/GNU acceptance remains open.',
 'binary':str(binary),'binary_sha256':digest,'size_bytes':binary.stat().st_size,'byte_identical_rebuild':True,'changed_helpers':changed,'strict_original_scripts':1,'clean_candidate_processes':50,'ownership_contracts':17,'reports':reports,'raw':raw,'pass':True},indent=2)+'\n')
print('PASS: owned pipe standards; original 50-process profile; 17 contracts; shared checks')
