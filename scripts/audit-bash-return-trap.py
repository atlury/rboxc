#!/usr/bin/env python3
"""Audit RETURN string cleanup, unchanged output, and separately open restart cases."""
# SPDX-License-Identifier: GPL-3.0-or-later
import importlib.util
import json
from pathlib import Path
import re
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[1]
sys.path[:0] = [str(ROOT/'tests'), str(ROOT/'tests/gnu')]
from comparison_profile import fingerprint
spec = importlib.util.spec_from_file_location('reviewed', ROOT/'tests/gnu/reviewed-original.py')
runner = importlib.util.module_from_spec(spec)
spec.loader.exec_module(runner)
target = ROOT/'evidence/bash-return-trap-validation.json'
assert not target.exists()
binary = ROOT/'target/bash-return-trap-candidate/release/rboxc'
rebuild = ROOT/'target/bash-return-trap-repro/release/rboxc'
digest = fingerprint(binary)
assert fingerprint(rebuild) == digest
assert len(subprocess.check_output([binary,'--list']).splitlines()) == 187
assert fingerprint(ROOT/'target/release/rboxc') == '8979835878e7dbe575440b5d0c7c86a01c75782ff5ddc735707ccada50e88e86'
profile_path = ROOT/'evidence/bash-return-trap-cleanup.json'
profile = json.loads(profile_path.read_text())
assert fingerprint(ROOT/'scripts/bash_return_trap_cleanup.py') == profile['driver_sha256']
assert fingerprint(ROOT/'evidence/bash-native-cleanup.json') == profile['baseline_profile_sha256']
assert fingerprint(ROOT/'evidence/bash-before-return-trap-native-cleanup.json') == profile['baseline_profile_sha256']
assert fingerprint(ROOT/'build/bash-cleanup/trap.c') == profile['baseline_adapted_source_sha256']
for path, key in (('source','source_sha256'),('object','object_sha256'),('build_log','build_log_sha256')):
    assert fingerprint(Path(profile[path])) == profile[key]
changes_path = ROOT/'evidence/bash-return-trap-helper-changes.json'
changes = json.loads(changes_path.read_text())
assert changes['changed'] == [p for p in changes['previous'] if changes['previous'][p]!=changes['current'][p]] == ['build/helpers/bash-017-trap.o']
assert changes['previous'].keys() == changes['current'].keys()
for path, expected in changes['current'].items(): assert fingerprint(ROOT/path) == expected
raw = {}
reports = {}


def verify_raw(path, expected):
    assert fingerprint(ROOT/path) == expected, path
    raw[path] = expected


def walk(value):
    if isinstance(value, list):
        for child in value: walk(child)
    elif isinstance(value, dict):
        for path, expected in value.get('raw', {}).items(): verify_raw(path,expected)
        if isinstance(value.get('log'), str):
            path=value['log']
            verify_raw(path,value.get('sha256',value.get('log_sha256',fingerprint(ROOT/path))))
        for child in value.values(): walk(child)


def load(name, current=True):
    path=ROOT/'evidence'/(name+'.json')
    data=json.loads(path.read_text())
    assert data.get('complete',True)
    if current: assert data['binary_sha256']==digest
    assert fingerprint(Path(data['binary']))==data['binary_sha256']
    for filename,expected in {**data.get('inputs',{}),**data['runtime_helpers']}.items():
        p=Path(filename)
        if p==ROOT/'tests/bash-return-trap-contract.py' and name=='bash-return-trap-contract':
            p=ROOT/'evidence/raw/bash-return-trap-initial-contract-driver.py'
        assert fingerprint(p)==expected, str(p)
    walk(data)
    reports[name]={'sha256':fingerprint(path),'passed':data['passed'],'total':data['total']}
    return data


baseline=load('bash-return-trap-contract-baseline',False)
contract=load('bash-return-trap-contract')
assert baseline['total']==contract['total']==32
assert baseline['passed']==16 and contract['passed']==28
assert [r['name'] for r in baseline['results']]==[r['name'] for r in contract['results']]
clean_contract_logs=[]
open_restart_logs=[]
improved=[]
for before,after in zip(baseline['results'],contract['results']):
    name=after['name']
    assert before['args']==after['args'] and before['command']==after['command']
    assert before['equivalent'] and after['equivalent']
    assert set(after['outcomes'])=={'gnu','gnu-valgrind','rboxc','rboxc-valgrind'}
    for data in (before,after):
        reference=data['outcomes']['gnu']
        for key,outcome in data['outcomes'].items():
            assert all(outcome[k]==reference[k] for k in ('status','stdout','stderr','tree'))
            for memory in outcome['memory']:
                text=(ROOT/memory['log']).read_text()
                pids=set(re.findall(r'^==([0-9]+)==',text,re.M)); assert len(pids)==1
                parsed=runner.parse_memory_log(text,pids.pop())
                assert all(memory[k]==v for k,v in parsed.items())
                assert memory['complete']
                if key=='rboxc-valgrind': assert parsed['non_inherited_descriptors']==0
                if key=='rboxc-valgrind' and name.endswith(':handler-restart'):
                    assert parsed['errors']==4 and parsed['heap_bytes']['definitely lost']==580
                    assert parsed['heap_bytes']['indirectly lost']==132
                    if data is after: open_restart_logs.append(memory['log'])
                elif key=='rboxc-valgrind' and data is after:
                    assert memory['clean'] and parsed['errors']==0
                    assert not any(parsed['heap_bytes'].get(k,0) for k in ('definitely lost','indirectly lost','possibly lost'))
                    clean_contract_logs.append(memory['log'])
    if not before['pass'] and after['pass']: improved.append(name)
assert len(improved)==12 and len(open_restart_logs)==4
for suffix,total in (('behavior',44),('adapters',69),('smoke',428),('dispatch',11)):
    data=load('bash-return-trap-'+suffix)
    assert data['passed']==data['total']==total
original=load('bash-return-trap-reviewed-original')
assert original['total']==27 and original['passed']==26 and original['original_groups']==26
audit_path=ROOT/'evidence/bash-return-trap-reviewed-validation.json'
audit=json.loads(audit_path.read_text())
assert audit['original_sha256']==reports['bash-return-trap-reviewed-original']['sha256']
assert audit['binary_sha256']==digest and audit['strict_original_passes']==26
assert len(audit['open_findings'])==1 and audit['open_findings'][0]['selection']=='set-x'
for path,expected in audit['raw'].items(): verify_raw(path,expected)
target.write_text(json.dumps({'scope':'Active RETURN trap strings remain borrowed until GNU releases them; only superseded restoration copies are freed. One isolated trap object changes and the rebuild is identical. All reviewed original assertions match; the deliberate already-closed trace descriptor stays outside strict counts. Twelve formerly leaking contracts now pass. Four restart cases retain the same 580 direct and 132 indirect lost bytes as the prior candidate and remain open. Contract comparisons use GNU and Rboxc only; the initial contract report reused generic scope text mentioning a C oracle, which was not executed in those cases.',
    'binary':str(binary),'binary_sha256':digest,'binary_bytes':binary.stat().st_size,
    'rebuild':str(rebuild),'rebuild_sha256':fingerprint(rebuild),
    'helper_profile_sha256':fingerprint(profile_path),'helper_changes_sha256':fingerprint(changes_path),
    'changed_helpers':changes['changed'],'reports':reports,
    'original_audit_sha256':fingerprint(audit_path),'strict_original_scripts':26,
    'original_processes':audit['clean_candidate_processes'],
    'improved_contracts':improved,'clean_contract_processes':len(clean_contract_logs),
    'open_restart_logs':open_restart_logs,'raw':raw,'driver_sha256':fingerprint(Path(__file__)),
    'installed_release_changed':False,'full_acceptance_complete':False},indent=2)+'\n')
print('Audited RETURN string fix, 26 strict original scripts and 12 repaired contracts; restart cleanup remains open')
