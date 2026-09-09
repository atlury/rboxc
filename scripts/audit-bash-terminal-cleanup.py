#!/usr/bin/env python3
"""Verify terminal ownership and preserve the original environment findings."""
# SPDX-License-Identifier: GPL-3.0-or-later
import hashlib,importlib.util,json,re,subprocess,sys
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
sys.path.insert(0,str(ROOT/'tests/gnu'))
spec=importlib.util.spec_from_file_location('reviewed',ROOT/'tests/gnu/reviewed-original.py')
runner=importlib.util.module_from_spec(spec);spec.loader.exec_module(runner)
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
target=ROOT/'evidence/bash-terminal-cleanup-validation.json';assert not target.exists()
binary=ROOT/'target/bash-terminal-cleanup-candidate/release/rboxc';digest=sha(binary)
assert sha(ROOT/'target/bash-terminal-cleanup-repro/release/rboxc')==digest
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
profile=report('bash-terminal-cleanup');pin('scripts/bash_terminal_cleanup.py',profile['driver_sha256'])
for r in profile['files']:
    for field in ('original','source','object','log'):pin(r[field],r[field+'_sha256'])
before=report('bash-before-terminal-cleanup-link');after=report('bash-link')
assert before['original_inputs']==after['original_inputs']
assert before['helper_inputs'].keys()==after['helper_inputs'].keys()
changed=[p for p in before['helper_inputs'] if before['helper_inputs'][p]!=after['helper_inputs'][p]]
assert changed==['build/helpers/bash-012-jobs.o']
for p,h in after['helper_inputs'].items():pin(p,h)
for name,total in [('contract',8),('behavior',44),('adapters',69),('smoke',428),('dispatch',11)]:
    r=report('bash-terminal-cleanup-'+name)
    assert r.get('complete',True) and r['binary_sha256']==digest
    assert r['passed']==r['total']==len(r['results'])==total and all(x['pass'] for x in r['results'])
    for p,h in r.get('inputs',{}).items():pin(p,h)
    for result in r['results']:
        for key,outcome in result.get('outcomes',{}).items():
            for m in outcome.get('memory',[]):memory(m,key=='rboxc-valgrind')
            if name=='contract':
                assert outcome['status']==7
                assert b'terminal-open-at-exit\n' in bytes.fromhex(outcome['raw_stdout'])
        if result.get('log'):pin(result['log'])
strict_original=0;clean_original=0
for name,audit_name in [('bash-terminal-history-original','bash-terminal-history-validation'),
                        ('bash-terminal-functions-original','bash-terminal-functions-validation'),
                        ('bash-next-four-scope','bash-next-four-scope-validation')]:
    r=report(name);a=report(audit_name)
    assert r['complete'] and a['original_sha256']==reports[f'evidence/{name}.json']
    pin(r['binary'],r['binary_sha256'])
    for p,h in a['raw'].items():pin(p,h)
    strict_original+=a['strict_original_passes'];clean_original+=len(a['processes'])
    for finding in a['open_findings']:
        m=finding['memory'];text=(ROOT/finding['log']).read_text()
        assert finding['selection']=='histexpand'
        assert m['errors']==m['non_inherited_descriptors']==1
        assert not any(m['heap_bytes'].get(k,0) for k in ('definitely lost','indirectly lost','possibly lost'))
        assert 'default action of signal 13 (SIGPIPE)' in text
        assert re.findall(r'Open file descriptor ([0-9]+):',text)==['1']
assert strict_original==3 and clean_original==142
# Preserve the initial missing-CTTY run, including every native and candidate
# log. It is diagnostic evidence, never original-suite acceptance.
initial=report('bash-next-four-history')
assert initial['complete'] and initial['passed']==0 and initial['total']==2
pin(initial['binary'],initial['binary_sha256'])
for p,h in initial['inputs'].items():
    replacement={'/root/rboxc/tests/bash-original.py':'evidence/raw/bash-history-original-driver.py',
        '/root/rboxc/inventory/bash-tests.json':'evidence/raw/bash-history-original-inventory.json'}.get(p,p)
    pin(replacement,h)
initial_terminal_findings=0
for r in initial['results']:
    for key,o in r['outcomes'].items():
        assert not o['timed_out'] and o['status']==0
        assert o['expected_output_matches']==(r['selection']=='histexpand')
        for p,h in o['raw'].items():pin(p,h)
        for m in o['memory']:
            text=memory(m)
            if key=='rboxc-valgrind' and r['selection']=='history' and not m['clean']:
                initial_terminal_findings+=1
                assert 'initialize_job_control' in text and 'move_to_high_fd' in text
                assert re.findall(r'Open file descriptor ([0-9]+):',text)==['255']
                assert not any(m['heap_bytes'].get(k,0) for k in ('definitely lost','indirectly lost','possibly lost'))
assert initial_terminal_findings==11
for p in ('scripts/entry_provider_helpers.py','tests/bash-original.py','tests/bash-terminal-contract.py',
          'scripts/audit-bash-reviewed-batch.py','evidence/raw/bash-terminal-build-data-original-driver.py',
          'evidence/raw/bash-terminal-build-data-original-inventory.json'):pin(p)
coverage=report('bash-terminal-cleanup-coverage')
assert coverage['strict_original_scripts']==53 and coverage['clean_candidate_processes']==12275
target.write_text(json.dumps({'scope':'Only the interactive jobs helper changes. The owned terminal remains visible through EXIT and is released at process exit; borrowed stderr and GNU job-control decisions remain unchanged. Eight contracts and shared checks pass on a byte-identical rebuild. Three new original scripts pass strictly. History expansion signal exits and the variable/grep diagnostic instrumentation baselines stay outside strict coverage. Initial missing-terminal findings and every native log remain preserved. Full Bash/GNU coverage remains unfinished.',
    'binary':str(binary),'binary_sha256':digest,'size_bytes':binary.stat().st_size,
    'byte_identical_rebuild':True,'changed_helpers':changed,'focused_contracts':8,
    'new_strict_original_scripts':strict_original,'new_clean_original_processes':clean_original,
    'initial_terminal_findings':initial_terminal_findings,'reports':reports,'raw':raw,'pass':True},indent=2)+'\n')
print('PASS: terminal ownership; eight contracts; three new strict originals; 142 clean original processes')
