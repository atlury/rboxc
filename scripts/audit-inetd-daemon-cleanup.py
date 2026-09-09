#!/usr/bin/env python3
"""Audit the bounded daemon descriptor change and original service results."""
# SPDX-License-Identifier: GPL-3.0-or-later
import hashlib
import importlib.util
import json
from pathlib import Path
import re
import subprocess
import sys
ROOT=Path(__file__).resolve().parents[1]
sys.path[:0]=[str(ROOT/'scripts'),str(ROOT/'tests/gnu')]
from inetd_daemon_cleanup import adapt
spec=importlib.util.spec_from_file_location('reviewed',ROOT/'tests/gnu/reviewed-original.py')
runner=importlib.util.module_from_spec(spec);spec.loader.exec_module(runner)
target=ROOT/'evidence/inetd-daemon-cleanup-validation.json';assert not target.exists()
raw={}
def pin(path,expected=None):
 p=Path(path);p=p if p.is_absolute() else ROOT/p
 value=hashlib.sha256(p.read_bytes()).hexdigest();assert expected is None or value==expected,str(p)
 raw[str(p.relative_to(ROOT)) if p.is_relative_to(ROOT) else str(p)]=value
 return value
def read(path):pin(path);return json.loads((ROOT/path).read_text())
def memory(row):
 path=ROOT/row['log'];pin(path,row['sha256'])
 pid=path.stem.rsplit('-',1)[-1]
 parsed=runner.parse_memory_log(path.read_text(),pid,True)
 assert all(row[k]==v for k,v in parsed.items()),str(path)
 assert parsed['complete_exec_log']
 assert not any(parsed['heap_bytes'].get(k,0) for k in ['definitely lost','indirectly lost','possibly lost'])
 return parsed
binary=ROOT/'target/inetd-daemon-candidate/release/rboxc'
repro=ROOT/'target/inetd-daemon-repro/release/rboxc'
assert binary.read_bytes()==repro.read_bytes();binary_hash=pin(binary);pin(repro)
native=read('evidence/inetd-daemon-native-cleanup.json')
source=Path('/opt/src/inetutils-2.8/libinetutils/daemon.c');pin(source,native['original_sha256'])
adapted=ROOT/'build/inetd-daemon-cleanup/daemon.c';pin(adapted,native['adapted_sha256'])
assert adapted.read_text()==adapt(source.read_text())
pin('scripts/inetd_daemon_cleanup.py',native['driver_sha256'])
pin('build/inetd-daemon-cleanup/daemon.o',native['object_sha256'])
pin(native['log'],native['log_sha256'])
old_archive=ROOT/'build/gnu-inetutils/libinetutils/libinetutils.a'
new_archive=ROOT/'build/inetd-daemon-cleanup/libinetutils.a'
pin(old_archive,native['original_archive_sha256']);pin(new_archive,native['adapted_archive_sha256'])
members=subprocess.check_output(['ar','t',old_archive],text=True).splitlines()
assert members==subprocess.check_output(['ar','t',new_archive],text=True).splitlines()
changed=[m for m in members if subprocess.check_output(['ar','p',old_archive,m])!=subprocess.check_output(['ar','p',new_archive,m])]
assert changed==['daemon.o'],changed
old=read('build/inetd-before-daemon-close/inetd-link.json');new=read('evidence/inetd-link.json')
assert new['native_command_entries']==[] and old['rust_source_sha256']==new['rust_source_sha256']
pin('src/generated/applet_inetd.rs',new['rust_source_sha256'])
changed_helpers=[]
for path,value in old['helper_inputs'].items():
 pin('build/inetd-before-daemon-close/'+Path(path).name,value)
 pin(path,new['helper_inputs'][path])
 if value!=new['helper_inputs'][path]:changed_helpers.append(path)
assert changed_helpers==['build/helpers/inetd-000-libinetutils.a']
contract=read('evidence/inetd-daemon-contract.json');assert contract['complete'] and contract['passed']==contract['total']==16
for path,value in contract['inputs'].items():pin(path,value)
clean_contract=0
for row in contract['results']:
 assert row['pass'] and row['outcomes']['gnu']['result']==row['outcomes']['adapted']['result']
 for impl,outcome in row['outcomes'].items():
  assert outcome['status']==0 and outcome['reaped_statuses']==[0,0]
  for m in outcome['memory']:
   parsed=memory(m)
   if impl=='adapted':
    assert parsed['errors']==parsed['non_inherited_descriptors']==0
    clean_contract+=1
assert clean_contract==24
original=read('evidence/inetd-daemon-closed-original.json')
assert original['complete'] and original['binary_sha256']==binary_hash
for path,value in original['inputs'].items():pin(path,value)
daemon_profiles={}
for row in original['results']:
 assert row['behavior_pass']
 for impl,outcome in row['outcomes'].items():
  assert outcome['status']==0 and outcome['service_connections']==10 and outcome['reload_signals']==5
  pin(outcome['log'],outcome['log_sha256'])
  ns=outcome['private_profile'];assert ns['uid']==ns['gid']==65534 and ns['groups']==[]
  assert [n['ifname'] for n in ns['interfaces']]==['lo']
  assert all(ns['namespaces'][k]!=ns['parent_namespaces'][k] for k in ns['namespaces'])
  assert sorted(x['status'] for x in outcome['children_reaped'])==[-15,0]
  bad=[]
  for m in outcome['memory']:
   parsed=memory(m)
   if parsed['errors'] or parsed['non_inherited_descriptors']:bad.append(m)
  if row['instrumented']:
   assert len(outcome['memory'])==14 and len(bad)==1
   assert bad[0]['errors']==(5 if impl=='rboxc' else 6) and bad[0]['non_inherited_descriptors']==5
   text=(ROOT/bad[0]['log']).read_text()
   assert 'default action of signal 15 (SIGTERM)' in text
   if impl=='rboxc':
    assert 'is already closed' not in text
    assert len(re.findall(r'^==\d+== Open file descriptor [012]: /dev/null$',text,re.M))==3
    assert len(re.findall(r'^==\d+== Open AF_INET6? socket ',text,re.M))==2
    assert not re.search(r'Invalid (?:read|write|free)|uninitialised|Syscall param',text)
   daemon_profiles[impl]=bad[0]
for stem,total in [('inetd-daemon-focused',5),('inetd-daemon-smoke',428),('inetd-daemon-dispatch',11)]:
 report=read('evidence/'+stem+'.json')
 assert report['passed']==report['total']==total and report['binary_sha256']==binary_hash
# Retain the initial contract where fixture-created descriptors were still open in fork parents.
old_contract=read('evidence/inetd-daemon-contract-owned.json');assert not old_contract['complete'] and old_contract['passed']==8
for path,value in old_contract['inputs'].items():
 if path.endswith('/tests/inetd-daemon-contract.py'):path='evidence/raw/inetd-daemon-contract-owned-driver.py'
 elif path.endswith('/tests/gnu/inetd-daemon-contract.c'):path='evidence/raw/inetd-daemon-contract-owned.c'
 pin(path,value)
target.write_text(json.dumps({'scope':native['scope'],'raw':raw,'binary':str(binary.relative_to(ROOT)),
 'binary_sha256':binary_hash,'binary_size':binary.stat().st_size,'reproducible':True,
 'contract_comparisons':16,'clean_contract_images':clean_contract,'original_behavior_pass':True,
 'clean_original_images':13,'daemon_signal_exit_profiles':daemon_profiles,
 'startup_close_fixed':True,'strict_original_memory_pass':False,'full_provider_pass':False},indent=2)+'\n')
print('PASS: 16 daemon comparisons, 24 clean helper images, original IPv4/IPv6 reloads; signal-exit resources retained')
