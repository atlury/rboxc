#!/usr/bin/env python3
"""Audit copied child-signal trap cleanup and original process families."""
# SPDX-License-Identifier: GPL-3.0-or-later
import hashlib,importlib.util,json,re,subprocess,sys
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
sys.path.insert(0,str(ROOT/'tests/gnu'))
spec=importlib.util.spec_from_file_location('reviewed',ROOT/'tests/gnu/reviewed-original.py')
runner=importlib.util.module_from_spec(spec);spec.loader.exec_module(runner)
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
target=ROOT/'evidence/bash-sigchld-validation.json';assert not target.exists()
binary=ROOT/'target/bash-sigchld-cleanup-candidate/release/rboxc';digest=sha(binary)
assert sha(ROOT/'target/bash-sigchld-cleanup-repro/release/rboxc')==digest
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
profile=report('bash-sigchld-cleanup');pin('scripts/bash_sigchld_cleanup.py',profile['driver_sha256'])
pin('build/bash-trap-restart-cleanup/trap.c',profile['previous_source_sha256'])
for field in ('source','object','build_log'):pin(profile[field],profile[field+'_sha256'])
before=report('bash-before-sigchld-cleanup-link');after=report('bash-link')
assert before['original_inputs']==after['original_inputs']
assert before['helper_inputs'].keys()==after['helper_inputs'].keys()
changed=[p for p in before['helper_inputs'] if before['helper_inputs'][p]!=after['helper_inputs'][p]]
assert changed==['build/helpers/bash-017-trap.o']
for p,h in after['helper_inputs'].items():pin(p,h)
for name,total in [('contract',20),('return-contract',32),('behavior',44),('adapters',69),('smoke',428),('dispatch',11)]:
    r=report('bash-sigchld-'+name)
    assert r.get('complete',True) and r['binary_sha256']==digest
    assert r['passed']==r['total']==len(r['results'])==total and all(x['pass'] for x in r['results'])
    for p,h in r.get('inputs',{}).items():pin(p,h)
    for result in r['results']:
        for key,outcome in result.get('outcomes',{}).items():
            for m in outcome.get('memory',[]):memory(m,key=='rboxc-valgrind')
        if result.get('log'):pin(result['log'])
for name,total,passed in [('bash-sigchld-contract-before',16,12),('bash-sigchld-queued-before',4,0)]:
    r=report(name);assert r['complete'] and r['total']==total and r['passed']==passed
    pin(r['binary'],r['binary_sha256'])
    for p,h in r['inputs'].items():
        if name=='bash-sigchld-contract-before' and p==str(ROOT/'tests/bash-sigchld-contract.py'):
            p='evidence/raw/bash-sigchld-initial-contract-driver.py'
        pin(p,h)
    for result in r['results']:
        assert result['equivalent']
        findings=[]
        for key,outcome in result['outcomes'].items():
            for m in outcome['memory']:
                text=memory(m,result['pass'] and key=='rboxc-valgrind')
                if key=='rboxc-valgrind' and not m['clean']:
                    assert m['complete'] and m['errors'] in (1,2) and m['non_inherited_descriptors']==0
                    assert m['heap_bytes']['definitely lost']>0 and 'rboxc_bash_set_signal' in text
                    findings.append(m)
        expected=([2] if name=='bash-sigchld-queued-before' else
                  list(range(2,25,2)) if result['name']=='sh:repeat' else
                  [31] if result['name']=='sh:replace' else
                  [22] if result['name']=='sh:reset' else
                  [9] if result['name']=='sh:function' else [])
        assert sorted(m['heap_bytes']['definitely lost'] for m in findings)==expected
for name,audit_name,findings in [('bash-trap-original','bash-trap-validation',3),('bash-sigchld-trap-original','bash-sigchld-trap-validation',2),('bash-coproc-original','bash-coproc-validation',1)]:
    r=report(name);a=report(audit_name)
    assert r['complete'] and a['original_sha256']==reports[f'evidence/{name}.json']
    pin(r['binary'],r['binary_sha256']);assert len(a['open_findings'])==findings
    for p,h in a['raw'].items():pin(p,h)
    if name=='bash-sigchld-trap-original':
        assert r['binary_sha256']==digest and a['assertion_baselines']==['trap']
        logs=r['results'][0]['outcomes']['rboxc-valgrind']['memory']
        assert len(logs)==68 and sum(m['clean'] for m in logs)==66
        for m in logs:
            text=memory(m)
            assert not any(m['heap_bytes'].get(k,0) for k in ('definitely lost','indirectly lost','possibly lost'))
            if not m['clean']:
                assert 'default action of signal 13 (SIGPIPE)' in text
                assert m['errors']==m['non_inherited_descriptors']==2
    if name=='bash-coproc-original':
        assert not a['assertion_baselines']
        logs=r['results'][0]['outcomes']['rboxc-valgrind']['memory']
        assert len(logs)==13 and sum(m['clean'] for m in logs)==12
        m,=[m for m in logs if not m['clean']];text=memory(m)
        assert 'default action of signal 15 (SIGTERM)' in text
        assert m['errors']==m['non_inherited_descriptors']==2
coverage=report('bash-sigchld-coverage')
assert coverage['strict_original_scripts']==54 and coverage['original_scripts']==65
helper=report('bash-xcase-helper')
for field in ('source','binary','compiler','log'):pin(helper[field],helper[field+'_sha256'])
for p in ('scripts/entry_provider_helpers.py','tests/bash-original.py','scripts/audit-bash-reviewed-batch.py',
          'evidence/raw/bash-sigchld-original-inventory.json','evidence/raw/bash-sigchld-original-driver.py'):pin(p)
target.write_text(json.dumps({'scope':'Copied SIGCHLD command ownership is repaired. Four queued-handler comparisons demonstrate the previous leak; twenty child-signal and thirty-two RETURN contracts now pass, alongside shared checks and a byte-identical rebuild. Original trap output matches normally and retains the identical GNU instrumentation baseline; its heap leak is repaired, with two default-SIGPIPE families still open. Coprocess assertions match in all modes, retaining one intentionally terminated child finding. Entire open families remain outside strict coverage.',
    'binary':str(binary),'binary_sha256':digest,'size_bytes':binary.stat().st_size,
    'byte_identical_rebuild':True,'changed_helpers':changed,'focused_contracts':52,
    'reports':reports,'raw':raw,'pass':True},indent=2)+'\n')
print('PASS: copied SIGCHLD text; 52 contracts; original trap heap leak repaired')
