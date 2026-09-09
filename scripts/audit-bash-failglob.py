#!/usr/bin/env python3
"""Audit failglob ownership cleanup and unchanged unprivileged Bash originals."""
# SPDX-License-Identifier: GPL-3.0-or-later
import hashlib
import importlib.util
import json
from pathlib import Path
import re
import sys
ROOT=Path(__file__).resolve().parents[1]
sys.path[:0]=[str(ROOT/'scripts'),str(ROOT/'tests/gnu')]
from bash_failglob_cleanup import adapt
spec=importlib.util.spec_from_file_location('reviewed',ROOT/'tests/gnu/reviewed-original.py')
runner=importlib.util.module_from_spec(spec);spec.loader.exec_module(runner)
target=ROOT/'evidence/bash-failglob-validation.json';assert not target.exists()
raw={}
def pin(path,expected=None):
 p=Path(path);p=p if p.is_absolute() else ROOT/p
 value=hashlib.sha256(p.read_bytes()).hexdigest();assert expected is None or value==expected,str(p)
 raw[str(p.relative_to(ROOT)) if p.is_relative_to(ROOT) else str(p)]=value
 return value
def read(path):pin(path);return json.loads((ROOT/path).read_text())
def memory(m,original=False):
 pin(m['log'],m['sha256']);text=(ROOT/m['log']).read_text()
 pid=m.get('pid',Path(m['log']).stem.rsplit('-',1)[-1])
 parsed=runner.parse_memory_log(text,pid,not original)
 assert all(m[k]==v for k,v in parsed.items()),m['log']
 return text
binary=ROOT/'target/bash-failglob-candidate/release/rboxc'
repro=ROOT/'target/bash-failglob-repro/release/rboxc'
assert binary.read_bytes()==repro.read_bytes();binary_hash=pin(binary);pin(repro)
cleanup=read('evidence/bash-failglob-cleanup.json')
for k in ['original','source','object','log']:pin(cleanup[k],cleanup[k+'_sha256'])
pin('scripts/bash_failglob_cleanup.py',cleanup['driver_sha256'])
assert Path(cleanup['source']).read_text()==adapt(Path(cleanup['original']).read_text())
old=read('build/bash-before-failglob-cleanup/bash-link.json');new=read('evidence/bash-link.json')
assert old['rust_source_sha256']==new['rust_source_sha256'] and new['native_command_entries']==[]
pin('src/generated/applet_bash.rs',new['rust_source_sha256'])
assert old['helper_inputs'].keys()==new['helper_inputs'].keys()
changed=[]
for path,value in old['helper_inputs'].items():
 pin('build/bash-before-failglob-cleanup/'+Path(path).name,value)
 pin(path,new['helper_inputs'][path])
 if value!=new['helper_inputs'][path]:changed.append(path)
assert len(changed)==1 and changed[0].endswith('-subst.o'),changed
contracts={};clean_contract=0
for stem,passed in [('bash-failglob-contract',16),('bash-failglob-baseline-contract',0)]:
 report=read('evidence/'+stem+'.json');assert report['complete'] and report['total']==report['planned_total']==16 and report['passed']==passed
 pin('tests/bash-failglob-contract.py',report['driver_sha256']);pin(report['binary'],report['binary_sha256'])
 if passed:assert report['binary_sha256']==binary_hash
 contracts[stem]=report
 for row in report['results']:
  assert row['behavior_pass'] and row['pass']==bool(passed)
  reference=row['outcomes']['gnu']
  for impl,outcome in row['outcomes'].items():
   assert all(outcome[k]==reference[k] for k in ['status','stdout','stderr'])
   for path,value in outcome['raw'].items():pin(path,value)
   for m in outcome['memory']:
    memory(m);assert m['complete_exec_log']
    if passed and impl=='rboxc-valgrind':
     assert m['errors']==m['non_inherited_descriptors']==0
     assert not any(m['heap_bytes'].get(k,0) for k in ['definitely lost','indirectly lost','possibly lost'])
     clean_contract+=1
   if impl=='rboxc-valgrind':assert outcome['clean']==bool(passed)
assert [r['script'] for r in contracts['bash-failglob-contract']['results']]==[r['script'] for r in contracts['bash-failglob-baseline-contract']['results']]
for stem,total in [('behavior',44),('adapters',69),('smoke',428),('dispatch',11)]:
 report=read('evidence/bash-failglob-'+stem+'.json')
 assert report['passed']==report['total']==total and report['binary_sha256']==binary_hash
original='evidence/bash-failglob-original.json';report=read(original)
assert report['complete'] and report['total']==report['planned_total']==2 and report['passed']==0 and report['binary_sha256']==binary_hash
snapshot='evidence/raw/bash-unprivileged-ready-inventory.json';inventory=read(snapshot)
for path,value in report['inputs'].items():
 if path.endswith('/inventory/bash-tests.json'):path=snapshot
 elif path.endswith('/tests/bash-original.py'):path='evidence/raw/bash-unprivileged-ready-driver.py'
 elif path.endswith('/tests/bash-unprivileged-profile.py'):path='evidence/raw/bash-unprivileged-ready-helper.py'
 pin(path,value)
open_processes=[];bad_profiles={};native=[];clean_original=0
for row in report['results']:
 selection=row['selection'];entry,=[r for r in inventory['inputs'] if r['target']==selection]
 expected=(Path('/opt/src/bash-5.3/tests')/entry['expected']).read_bytes()
 assert not row['pass'] and entry['unprivileged'] and entry['private_tmp']
 for impl,outcome in row['outcomes'].items():
  assert outcome['status']==0 and not outcome['timed_out'] and outcome['expected_output_matches']
  profile=outcome['unprivileged_profile']
  assert profile['uid']==profile['gid']==65534 and profile['groups']==[] and profile['no_new_privileges'] and profile['cap_effective']=='0000000000000000'
  assert {s['fd'] for s in profile['stdio']}==({0,1,2} if selection=='test' else {1,2})
  assert all(s['uid']==s['gid']==65534 for s in profile['stdio'])
  assert outcome['private_tmp']['destination']=='/tmp' and outcome['private_tmp']['mode']==0o1777
  assert outcome['stdin_terminal']==(selection=='test')
  for path,value in outcome['raw'].items():pin(path,value)
  actual,=[ROOT/p for p in outcome['raw'] if p.endswith('/actual')];assert actual.read_bytes()==expected
  nss,=[ROOT/p for p in outcome['raw'] if p.endswith('/nsswitch.conf')]
  assert all(line.endswith(': files') for line in nss.read_text().splitlines())
  mounts,=[ROOT/p for p in outcome['raw'] if p.endswith('/mountinfo')]
  assert ' /tmp ' in mounts.read_text() and ' /etc/nsswitch.conf ' in mounts.read_text()
  for staged in outcome['staged_executables'].values():assert pin(staged['source'],staged['sha256'])==staged['copy_sha256']
  if impl.startswith('rboxc'):assert outcome['staged_executables']['bash']['sha256']==binary_hash
  for m in outcome['memory']:
   text=memory(m,True)
   if impl=='gnu-valgrind':native.append({'selection':selection,**m})
   if impl!='rboxc-valgrind':continue
   open_processes.append({'selection':selection,**m})
   if m['clean']:
    assert m['complete'] and m['errors']==m['non_inherited_descriptors']==0
    assert not any(m['heap_bytes'].get(k,0) for k in ['definitely lost','indirectly lost','possibly lost'])
    clean_original+=1
   else:
    assert selection not in bad_profiles
    bad_profiles[selection]=m
    if selection=='glob-test':
     assert '/exec/locale -a\n' in text and m['complete'] and m['errors']==3 and m['non_inherited_descriptors']==0
     assert m['heap_bytes']['definitely lost']==306 and m['heap_bytes']['indirectly lost']==271
    else:
     assert '/exec/bash ./test1.sub\n' in text and not m['complete'] and m['errors'] is None
     assert 'ERROR SUMMARY:' not in text and 'FILE DESCRIPTORS:' not in text
  if impl.endswith('-valgrind'):assert len(outcome['memory'])==({'glob-test':163,'test':47}[selection])
assert clean_original==208 and len(open_processes)==210 and set(bad_profiles)=={'glob-test','test'}
# Preserve the initial harness findings and uncorrected candidate observations.
initial=read('evidence/bash-unprivileged-original.json');assert initial['complete'] and initial['passed']==0
for path,value in initial['inputs'].items():
 if path.endswith('/inventory/bash-tests.json'):path='evidence/raw/bash-unprivileged-inventory.json'
 elif path.endswith('/tests/bash-original.py'):path='evidence/raw/bash-unprivileged-driver.py'
 elif path.endswith('/tests/bash-unprivileged-profile.py'):path='evidence/raw/bash-unprivileged-initial-helper.py'
 pin(path,value)
for row in initial['results']:
 for impl,outcome in row['outcomes'].items():
  assert outcome['status']==0 and outcome['expected_output_matches']==(row['selection']=='glob-test')
  for path,value in outcome['raw'].items():pin(path,value)
  for m in outcome['memory']:memory(m,True)
contract=read('evidence/bash-unprivileged-profile-contract.json')
pin('evidence/raw/bash-unprivileged-initial-helper.py',contract['source_sha256'])
assert contract['status']==0 and contract['owned_unreadable_file_denied']
target.write_text(json.dumps({'scope':'Unchanged glob and file-predicate originals pass all assertions under real unprivileged execution. All 210 candidate process images remain outside strict recipe counts: native locale leaks and one early-terminated child has no complete Valgrind summary. Failglob work-list cleanup passes 16 differential contracts, preserving GNU behavior.',
 'original':original,'original_sha256':pin(original),'binary_sha256':binary_hash,'binary_size':binary.stat().st_size,'reproducible':True,
 'inventory_snapshot':snapshot,'inventory_sha256':pin(snapshot),'raw':raw,'processes':[],
 'open_processes':open_processes,'assertion_baselines':[],'native_processes':native,
 'open_profiles':bad_profiles,'clean_original_images':clean_original,'clean_contract_images':clean_contract,
 'changed_helpers':changed,'full_bash_complete':False},indent=2)+'\n')
current=ROOT/'inventory/bash-tests.json';data=json.loads(current.read_text())
for selection in ['glob-test','test']:
 entry,=[r for r in data['inputs'] if r['target']==selection]
 saved,=[r for r in inventory['inputs'] if r['target']==selection];assert entry==saved
 entry.update(state='reviewed-original-assertions-memory-open',evidence=original,audit=str(target.relative_to(ROOT)))
current.write_text(json.dumps(data,indent=2)+'\n')
print('PASS:',clean_contract,'clean contract images; 208 clean original images, 2 open; both full original assertions pass')
