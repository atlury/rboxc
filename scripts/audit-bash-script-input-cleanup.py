#!/usr/bin/env python3
"""Audit script-open ownership and the unchanged invocation baseline."""
# SPDX-License-Identifier: GPL-3.0-or-later
import hashlib,importlib.util,json,re,subprocess,sys
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
sys.path.insert(0,str(ROOT/'tests/gnu'))
spec=importlib.util.spec_from_file_location('reviewed',ROOT/'tests/gnu/reviewed-original.py')
runner=importlib.util.module_from_spec(spec);spec.loader.exec_module(runner)
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
target=ROOT/'evidence/bash-script-input-validation.json';assert not target.exists()
binary=ROOT/'target/bash-script-input-cleanup-candidate/release/rboxc';digest=sha(binary)
assert sha(ROOT/'target/bash-script-input-cleanup-repro/release/rboxc')==digest
assert len(subprocess.check_output([binary,'--list']).splitlines())==187
assert sha(ROOT/'target/release/rboxc')=='8979835878e7dbe575440b5d0c7c86a01c75782ff5ddc735707ccada50e88e86'
raw={};reports={}
def pin(path,expected=None):
    p=Path(path)
    if not p.is_absolute():p=ROOT/p
    key=str(p.relative_to(ROOT)) if p.is_relative_to(ROOT) else str(p)
    h=raw.get(key) or sha(p)
    if expected is not None:assert h==expected,str(p)
    raw[key]=h
    return h
def report(name):
    p=ROOT/f'evidence/{name}.json';reports[str(p.relative_to(ROOT))]=pin(p)
    return json.loads(p.read_text())
def memory(m,strict=False):
    pin(m['log'],m['sha256']);text=(ROOT/m['log']).read_text()
    pid,=set(re.findall(r'^==([0-9]+)==',text,re.M))
    parsed=runner.parse_memory_log(text,pid)
    assert all(m[k]==v for k,v in parsed.items())
    if strict:
        assert m['complete'] and m['clean'] and parsed['errors']==parsed['non_inherited_descriptors']==0
        assert not any(parsed['heap_bytes'].get(k,0) for k in ('definitely lost','indirectly lost','possibly lost'))
    return text
profile=report('bash-script-input-cleanup');pin('scripts/bash_script_input_cleanup.py',profile['driver_sha256'])
for r in profile['files']:
    for field in ('original','source','object','log'):pin(r[field],r[field+'_sha256'])
before=report('bash-before-script-input-cleanup-link');after=report('bash-link')
assert before['original_inputs']==after['original_inputs']
assert before['helper_inputs'].keys()==after['helper_inputs'].keys()
changed=[p for p in before['helper_inputs'] if before['helper_inputs'][p]!=after['helper_inputs'][p]]
assert changed==['build/helpers/bash-047-native-helpers.o']
for p,h in after['helper_inputs'].items():pin(p,h)
for name,total in [('contract',16),('behavior',44),('adapters',69),('smoke',428),('dispatch',11)]:
    r=report('bash-script-input-'+name)
    assert r.get('complete',True) and r['binary_sha256']==digest
    assert r['passed']==r['total']==len(r['results'])==total and all(x['pass'] for x in r['results'])
    for p,h in r.get('inputs',{}).items():pin(p,h)
    for result in r['results']:
        for key,outcome in result.get('outcomes',{}).items():
            for m in outcome.get('memory',[]):memory(m,key=='rboxc-valgrind')
        if result.get('log'):pin(result['log'])
for name,audit_name,expected_findings in [('bash-invocation-private-original','bash-invocation-private-validation',3),
                                         ('bash-script-input-invocation-original','bash-script-input-invocation-validation',1)]:
    r=report(name);a=report(audit_name)
    assert r['complete'] and a['original_sha256']==reports[f'evidence/{name}.json']
    pin(r['binary'],r['binary_sha256'])
    assert a['assertion_baselines']==a['interpreter_baseline_open']==['invocation']
    assert len(a['open_findings'])==expected_findings
    for p,h in a['raw'].items():pin(p,h)
    for finding in a['open_findings']:
        m=finding['memory'];text=(ROOT/finding['log']).read_text()
        if m['errors'] is None:
            assert m['non_inherited_descriptors'] is None
        else:
            assert name=='bash-invocation-private-original'
            assert m['errors']==m['non_inherited_descriptors']==1
            assert re.findall(r'Open file descriptor ([0-9]+):',text)==['3']
            assert 'rboxc_bash_open_shell_script' in text
            assert not any(m['heap_bytes'].get(k,0) for k in ('definitely lost','indirectly lost','possibly lost'))
    if name=='bash-script-input-invocation-original':
        assert r['binary_sha256']==digest
        logs=r['results'][0]['outcomes']['rboxc-valgrind']['memory']
        assert len(logs)==48 and sum(m['clean'] for m in logs)==47
coverage=report('bash-script-input-coverage')
assert coverage['strict_original_scripts']==54 and coverage['original_scripts']==63
for p in ('scripts/entry_provider_helpers.py','tests/bash-original.py','tests/bash-script-input-contract.py',
          'scripts/audit-bash-reviewed-batch.py','evidence/raw/bash-invocation-original-driver.py',
          'evidence/raw/bash-invocation-original-inventory.json'):pin(p)
target.write_text(json.dumps({'scope':'The opened script descriptor retains ownership before startup validation. Sixteen valid/empty/directory/executable input contracts and all shared checks pass on a byte-identical rebuild. The unchanged invocation recipe matches normally; its Valgrind missing-interpreter diagnostic and one incomplete child image match GNU and remain outside strict coverage. The other 47 candidate logs are clean, including both repaired startup exits. Only the native helper object changes, and host profile/helper files remain unchanged.',
    'binary':str(binary),'binary_sha256':digest,'size_bytes':binary.stat().st_size,
    'byte_identical_rebuild':True,'changed_helpers':changed,'focused_contracts':16,
    'clean_original_processes_outside_strict_counts':47,'reports':reports,'raw':raw,'pass':True},indent=2)+'\n')
print('PASS: script input ownership; 16 contracts; 47 clean original logs and one interpreter baseline')
