#!/usr/bin/env python3
"""Verify trap restoration ownership, regression contracts and original exec baselines."""
# SPDX-License-Identifier: GPL-3.0-or-later
import hashlib
import importlib.util
import json
from pathlib import Path
import re
import sys
ROOT=Path(__file__).resolve().parents[1]
sys.path[:0]=[str(ROOT/'scripts'),str(ROOT/'tests/gnu')]
from bash_exec_trap_cleanup import adapt
spec=importlib.util.spec_from_file_location('reviewed',ROOT/'tests/gnu/reviewed-original.py');runner=importlib.util.module_from_spec(spec);spec.loader.exec_module(runner)
target=ROOT/'evidence/bash-exec-trap-validation.json';assert not target.exists()
raw={}
def pin(path,expected=None):
 p=Path(path);p=p if p.is_absolute() else ROOT/p
 value=hashlib.sha256(p.read_bytes()).hexdigest();assert expected is None or value==expected,str(p)
 raw[str(p.relative_to(ROOT)) if p.is_relative_to(ROOT) else str(p)]=value
 return value
def read(path):pin(path);return json.loads((ROOT/path).read_text())
def memory(m):
 pin(m['log'],m['sha256']);text=(ROOT/m['log']).read_text();pid=m.get('pid',Path(m['log']).stem.rsplit('-',1)[-1])
 parsed=runner.parse_memory_log(text,pid,'pid' not in m)
 assert all(m[k]==v for k,v in parsed.items()),m['log']
 return text
binary=ROOT/'target/bash-exec-trap-candidate/release/rboxc';repro=ROOT/'target/bash-exec-trap-repro/release/rboxc'
assert binary.read_bytes()==repro.read_bytes();binary_hash=pin(binary);pin(repro)
cleanup=read('evidence/bash-exec-trap-cleanup.json');pin('scripts/bash_exec_trap_cleanup.py',cleanup['driver_sha256'])
for k in ['original','source','object','log']:pin(cleanup[k],cleanup[k+'_sha256'])
assert Path(cleanup['source']).read_text()==adapt(Path(cleanup['original']).read_text())
old=read('build/bash-before-exec-trap-cleanup/bash-link.json');new=read('evidence/raw/bash-exec-trap-link.json')
assert old['rust_source_sha256']==new['rust_source_sha256'] and new['native_command_entries']==[]
pin('src/generated/applet_bash.rs',new['rust_source_sha256']);assert old['helper_inputs'].keys()==new['helper_inputs'].keys()
changed=[]
for path,value in old['helper_inputs'].items():
 pin('build/bash-before-exec-trap-cleanup/'+Path(path).name,value)
 pin('build/bash-exec-trap-evidence/'+Path(path).name,new['helper_inputs'][path])
 if value!=new['helper_inputs'][path]:changed.append(path)
assert len(changed)==1 and changed[0].endswith('-trap.o'),changed
contract_images={}
for stem,total in [('contract',16),('state-contract',32),('sigchld-contract',20),('baseline-contract',16)]:
 report=read('evidence/bash-exec-trap-'+stem+'.json');baseline=stem=='baseline-contract'
 assert report['complete'] and report['total']==report['planned_total']==total and report['passed']==(0 if baseline else total)
 pin(report['binary'],report['binary_sha256'])
 if not baseline:assert report['binary_sha256']==binary_hash
 if 'driver_sha256' in report:pin('tests/bash-exec-trap-contract.py',report['driver_sha256'])
 for path,value in report.get('inputs',{}).items():pin(path,value)
 clean=0
 for row in report['results']:
  assert row.get('behavior_pass',row.get('equivalent')) and row['pass']==(not baseline)
  reference=row['outcomes']['gnu']
  for impl,outcome in row['outcomes'].items():
   assert all(outcome[k]==reference[k] for k in ['status','stdout','stderr'])
   for path,value in outcome.get('raw',{}).items():pin(path,value)
   for m in outcome['memory']:
    text=memory(m)
    assert runner.parse_memory_log(text,m.get('pid',Path(m['log']).stem.rsplit('-',1)[-1]),True)['complete_exec_log']
    if not baseline and impl=='rboxc-valgrind':
     assert m['errors']==m['non_inherited_descriptors']==0
     assert not any(m['heap_bytes'].get(k,0) for k in ['definitely lost','indirectly lost','possibly lost']);clean+=1
 contract_images[stem]=clean
for stem,total in [('behavior',44),('adapters',69),('smoke',428),('dispatch',11)]:
 report=read('evidence/bash-exec-trap-'+stem+'.json');assert report['passed']==report['total']==total and report['binary_sha256']==binary_hash
original='evidence/bash-exec-trap-original.json';report=read(original)
snapshot='evidence/raw/bash-exec-trap-inventory.json';inventory=read(snapshot)
assert report['complete'] and report['total']==report['planned_total']==1 and report['passed']==0 and report['binary_sha256']==binary_hash
for path,value in report['inputs'].items():
 if path.endswith('/inventory/bash-tests.json'):path=snapshot
 elif path.endswith('/tests/bash-original.py'):path='evidence/raw/bash-exec-trap-driver.py'
 elif path.endswith('/tests/bash-unprivileged-profile.py'):path='evidence/raw/bash-exec-trap-helper.py'
 pin(path,value)
result,=report['results'];assert not result['pass'] and result['selection']=='execscript'
processes=[];bad=[];outputs={};native=[]
for impl,outcome in result['outcomes'].items():
 assert outcome['status']==0 and not outcome['timed_out'] and not outcome['expected_output_matches']
 profile=outcome['unprivileged_profile'];assert profile['uid']==profile['gid']==65534 and profile['groups']==[] and profile['cap_effective']=='0000000000000000'
 for path,value in outcome['raw'].items():pin(path,value)
 actual,=[ROOT/p for p in outcome['raw'] if p.endswith('/actual')];outputs[impl]=actual.read_text().replace(profile['work'],'<WORK>')
 assert "trap -- '' SIGPIPE" not in outputs[impl] and "trap -- '' SIGXFSZ" not in outputs[impl]
 for m in outcome['memory']:
  text=memory(m)
  if impl=='gnu-valgrind':native.append({'selection':'execscript',**m})
  if impl=='rboxc-valgrind':
   processes.append({'selection':'execscript',**m})
   if m['clean']:
    assert m['complete'] and m['errors']==m['non_inherited_descriptors']==0
    assert not any(m['heap_bytes'].get(k,0) for k in ['definitely lost','indirectly lost','possibly lost'])
   else:
    bad.append(m);assert not m['complete'] and 'EXEC FAILED: I can\'t recover from execve() failing' in text and 'errno 7' in text
 if impl.endswith('-valgrind'):assert len(outcome['memory'])==286
assert len(bad)==1 and len(processes)==286
assert outputs['gnu']==outputs['rboxc'] and outputs['gnu-valgrind']==outputs['rboxc-valgrind']
assert 'E2BIG: error message mismatch:' in outputs['gnu-valgrind'] and 'this is <WORK>/exec/bash' in outputs['gnu-valgrind']
stable=read('evidence/bash-failglob-stable-validation.json');assert pin(stable['predecessor'])==stable['predecessor_sha256']
for path,digest in stable['raw'].items():pin(path,digest)
target.write_text(json.dumps({'scope':'Restore ordinary-signal trap string ownership after failed exec; original Rust entry unchanged. All 16 new contracts plus 32 trap-state and 20 child-signal comparisons pass. Whole original execscript matches native/instrumented GNU baselines and has 285 clean candidate images; only Valgrind\'s unrecoverable E2BIG client image remains incomplete. Original expected-output differences in platform trap order and instrumentation argv/signal behavior are retained, not counted as passes. Python privilege launcher now restores ordinary PIPE/XFSZ defaults.',
 'original':original,'original_sha256':pin(original),'binary_sha256':binary_hash,'binary_size':binary.stat().st_size,'reproducible':True,
 'inventory_snapshot':snapshot,'inventory_sha256':pin(snapshot),'raw':raw,'processes':[],'open_processes':processes,'assertion_baselines':['execscript'],'native_processes':native,
 'contract_clean_images':contract_images,'changed_helpers':changed,'open_profiles':bad,'full_bash_complete':False},indent=2)+'\n')
p=ROOT/'inventory/bash-tests.json';data=json.loads(p.read_text());entry,=[r for r in data['inputs'] if r['target']=='execscript'];saved,=[r for r in inventory['inputs'] if r['target']=='execscript'];assert entry==saved
entry.update(state='reviewed-original-baseline-open',evidence=original,audit=str(target.relative_to(ROOT)))
p.write_text(json.dumps(data,indent=2)+'\n')
print('PASS: 68 trap comparisons, original leak resolved, 285 clean original images; E2BIG instrumentation baseline retained')
