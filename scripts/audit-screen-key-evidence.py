#!/usr/bin/env python3
"""Audit Screen action ownership and original terminal acceptance findings."""
# SPDX-License-Identifier: GPL-3.0-or-later
import importlib.util
import json
from pathlib import Path
import re
import sys

ROOT=Path(__file__).resolve().parents[1]
sys.path[:0]=[str(ROOT/'tests'),str(ROOT/'tests/gnu'),str(ROOT/'scripts')]
from comparison_profile import fingerprint
from screen_key_cleanup import adapt
spec=importlib.util.spec_from_file_location('reviewed',ROOT/'tests/gnu/reviewed-original.py')
runner=importlib.util.module_from_spec(spec)
spec.loader.exec_module(runner)
target=ROOT/'evidence/screen-key-validation.json'
assert not target.exists()
binary=ROOT/'target/screen-key-candidate/release/rboxc'
repro=ROOT/'target/screen-key-repro/release/rboxc'
assert binary.read_bytes()==repro.read_bytes()
previous=ROOT/'target/less-keyboard-candidate/release/rboxc'
backup=ROOT/'build/screen-before-key-cleanup'
old_link=json.loads((backup/'screen-link.json').read_text())
new_link=json.loads((ROOT/'evidence/screen-link.json').read_text())
assert old_link['rust_source_sha256']==new_link['rust_source_sha256']==fingerprint(ROOT/'src/generated/applet_screen.rs')
assert old_link['helper_inputs'].keys()==new_link['helper_inputs'].keys()
changed=[]
for path,old_hash in old_link['helper_inputs'].items():
    assert fingerprint(backup/Path(path).name)==old_hash
    assert fingerprint(ROOT/path)==new_link['helper_inputs'][path]
    if old_hash!=new_link['helper_inputs'][path]:changed.append(path)
assert len(changed)==1 and changed[0].endswith('-process.o')
source=Path('/opt/src/screen-5.0.2/process.c')
pin=json.loads((ROOT/'inventory/sources.json').read_text())['screen']
assert fingerprint(source)==pin['source_and_header_sha256']['process.c']
adapted=ROOT/'build/screen-key-cleanup/process.c'
assert adapted.read_text()==adapt(source.read_text())
cleanup=json.loads((ROOT/'evidence/screen-key-native-cleanup.json').read_text())
assert cleanup['driver_sha256']==fingerprint(ROOT/'scripts/screen_key_cleanup.py')
assert cleanup['adapted_source_sha256']==fingerprint(adapted)
assert cleanup['object_sha256']==fingerprint(ROOT/'build/screen-key-cleanup/process.o')
assert fingerprint(ROOT/cleanup['log'])==cleanup['log_sha256']
report_names=['screen-key-focused','screen-key-contract','screen-key-smoke','screen-key-full-dispatch',
    'screen-original-detach','screen-original-detach-bounded','screen-original-detach-private-sockets',
    'screen-original-detach-gnu-helper','screen-key-original-detach','screen-key-native-cleanup']
reports={name:json.loads((ROOT/'evidence'/(name+'.json')).read_text()) for name in report_names}
for name,count in [('screen-key-focused',5),('screen-key-smoke',428),('screen-key-full-dispatch',11)]:
    d=reports[name];assert d['passed']==d['total']==count and d['binary_sha256']==fingerprint(binary)
contract=reports['screen-key-contract'];assert contract['passed']==contract['total']==6
for path,expected in contract['inputs'].items():assert fingerprint(Path(path))==expected
for r in contract['results']:
    assert fingerprint(ROOT/r['binary'])==r['binary_sha256']
    log=ROOT/r['log'];assert fingerprint(log)==r['log_sha256']
    pids=set(re.findall(r'^==([0-9]+)==',log.read_text(),re.M));assert len(pids)==1
    memory=runner.parse_memory_log(log.read_text(),pids.pop(),exec_only=True)
    assert memory==r['memory'] and memory['complete_exec_log']
    if r['implementation']=='adapted':
        assert memory['errors']==memory['non_inherited_descriptors']==0
        assert not any(memory['heap_bytes'].get(k,0) for k in ['definitely lost','indirectly lost','possibly lost'])
processes=[]
drivers={'screen-original-detach':ROOT/'evidence/raw/screen-original-detach-initial-driver.py',
    'screen-original-detach-bounded':ROOT/'evidence/raw/screen-original-detach-bounded-driver.py',
    'screen-original-detach-private-sockets':ROOT/'evidence/raw/screen-original-detach-private-sockets-driver.py',
    'screen-original-detach-gnu-helper':ROOT/'tests/screen-original-detach.py',
    'screen-key-original-detach':ROOT/'tests/screen-original-detach.py'}
for name,driver in drivers.items():
    data=reports[name]
    assert fingerprint(driver)==data['driver_sha256']
    assert fingerprint(Path(data['binary']))==data['binary_sha256']
    for path,expected in data['inputs'].items():
        p=driver if path==str(ROOT/'tests/screen-original-detach.py') else Path(path)
        assert fingerprint(p)==expected
    for key,outcome in data['outcomes'].items():
        for stream in outcome['streams'].values():
            p=ROOT/stream['path'];assert fingerprint(p)==stream['sha256'] and p.read_bytes()==bytes.fromhex(stream['bytes'])
        for m in outcome['memory']:
            log=ROOT/m['log'];assert fingerprint(log)==m['sha256']
            parsed=runner.parse_memory_log(log.read_text(),log.stem,exec_only=True)
            assert all(m[k]==v for k,v in parsed.items())
            if name=='screen-key-original-detach':
                assert parsed['complete_exec_log']
                assert len(m['commands'])==1
                command=m['commands'][0]
                is_screen='/real/screen ' in command
                assert is_screen or command in ['/usr/bin/sh -c sleep\\ 30',
                    str(ROOT/'build/gnu-coreutils/src/coreutils')+' --coreutils-prog=sleep 30']
                clean=parsed['errors']==parsed['non_inherited_descriptors']==0 and not any(
                    parsed['heap_bytes'].get(k,0) for k in ['definitely lost','indirectly lost','possibly lost'])
                processes.append({'implementation':key,'provider':'screen' if is_screen else 'test-dependency',
                    'clean':clean,**m})
final=reports['screen-key-original-detach']
assert final['complete'] and all(o['status']==0 and not o['timed_out'] and not o['sockets_remaining'] for o in final['outcomes'].values())
candidate=[p for p in processes if p['implementation']=='rboxc-valgrind']
assert len(candidate)==10 and sum(p['clean'] for p in candidate)==9
open_processes=[p for p in candidate if not p['clean']]
assert len(open_processes)==1
finding=open_processes[0]
assert finding['heap_bytes']['definitely lost']==finding['heap_bytes']['indirectly lost']==0
assert finding['heap_bytes']['possibly lost']==418 and finding['non_inherited_descriptors']==4
baseline=reports['screen-original-detach-gnu-helper']['outcomes']['rboxc-valgrind']['memory']
assert sum(m['heap_bytes'].get('definitely lost',0) for m in baseline)==8
# Preserve all raw partial-run logs too; a timeout never becomes a passed test.
initial_run=next((ROOT/'evidence/raw/screen-original-detach').glob('run-*'))
raw_observations={str(p.relative_to(ROOT)):fingerprint(p) for p in initial_run.rglob('*') if p.is_file() and not p.is_symlink()}
report={'scope':'Only the namespaced Screen process helper changed, freeing the action length array before '
    'discarding its pointer. Six contracts and five focused comparisons pass. Original attach/detach assertions '
    'pass in all four declared profiles; the candidate daemon no longer loses the eight action bytes. '
    'Terminal acceptance remains open: the daemon retains four owned descriptors and 418 possibly-lost '
    'terminal-library bytes. Nine of ten candidate process logs are clean, including two local test-dependency '
    'processes. Raw native findings, test-environment failures and the timed-out profile remain preserved.',
    'binary':str(binary),'binary_sha256':fingerprint(binary),'bytes':binary.stat().st_size,
    'rebuild':str(repro),'rebuild_sha256':fingerprint(repro),'byte_identical':True,
    'previous_binary':str(previous),'previous_sha256':fingerprint(previous),
    'size_delta':binary.stat().st_size-previous.stat().st_size,
    'changed_helpers':changed,'original_terminal_assertion_profiles':4,
    'terminal_full_acceptance':False,'candidate_terminal_processes':10,
    'clean_screen_terminal_processes':7,'clean_test_dependency_processes':2,
    'remaining_owned_descriptors':4,'remaining_possibly_lost_bytes':418,
    'ownership_contract_outcomes':6,'clean_adapted_contract_processes':3,
    'coreutils_smoke':428,'dispatcher_checks':11,'focused_checks':5,
    'reports':{n:fingerprint(ROOT/'evidence'/(n+'.json')) for n in report_names},
    'prior_link':{'path':str((backup/'screen-link.json').relative_to(ROOT)),'sha256':fingerprint(backup/'screen-link.json')},
    'link_report_sha256':fingerprint(ROOT/'evidence/screen-link.json'),
    'driver_sha256':fingerprint(Path(__file__)), 'preserved_partial_run':raw_observations,'processes':processes}
target.write_text(json.dumps(report,indent=2)+'\n')
print('Screen key cleanup audited: leak removed; original terminal assertions pass; daemon cleanup remains open')
