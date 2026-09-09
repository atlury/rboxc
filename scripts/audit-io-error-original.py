#!/usr/bin/env python3
"""Account for every image of the whole finite GNU write-error original."""
# SPDX-License-Identifier: GPL-3.0-or-later
from collections import Counter
import copy
import hashlib
import importlib.util
import json
from pathlib import Path
import re
import sys
ROOT=Path(__file__).resolve().parents[1]
sys.path[:0]=[str(ROOT/'tests/gnu')]
spec=importlib.util.spec_from_file_location('reviewed',ROOT/'evidence/raw/gnu-nproc-policy-closed-driver.py')
runner=importlib.util.module_from_spec(spec);spec.loader.exec_module(runner)
target=ROOT/'evidence/gnu-io-error-original-validation.json';ledger=ROOT/'evidence/gnu-io-error-original-coverage.json'
assert not target.exists() and not ledger.exists()
raw={}
def pin(path,expected=None):
 p=Path(path);p=p if p.is_absolute() else ROOT/p
 value=hashlib.sha256(p.read_bytes()).hexdigest();assert expected is None or value==expected,str(p)
 raw[str(p.relative_to(ROOT)) if p.is_relative_to(ROOT) else str(p)]=value
 return value
def read(path):pin(path);return json.loads((ROOT/path).read_text())
stem='gnu-io-errors-current-valgrind'
report=read('evidence/raw/'+stem+'.json');checkpoint=read('evidence/raw/'+stem+'.progress.json')
row,=report['results'];script=row['script'];assert script=='tests/misc/io-errors.sh' and row['state']=='open' and not row['pass']
saved=checkpoint['runs'][script];context=saved['context'];assert context['valgrind'] and row['trace_children']
pin(Path(context['source'])/script,row['sha256'])
for path,value in context['drivers'].items():pin('evidence/raw/gnu-nproc-policy-closed-driver.py' if path=='tests/gnu/reviewed-original.py' else path,value)
for path,value in context['gnu_harness'].items():pin(Path(context['source'])/path,value)
for path,value in context['valgrind_runtime'].items():pin(path,value)
pin('/bin/sh',context['shell'])
summaries={};commands={};writers={}
for impl in ['gnu','rboxc']:
 outcome=row[impl];assert outcome==saved['outcomes'][impl]['result'] and outcome['status']==0
 pin('target/pr-page-cleanup-candidate/release/rboxc' if impl=='rboxc' else 'build/gnu-coreutils/src/coreutils',outcome['binary_sha256'])
 for path,value in saved['outcomes'][impl]['logs'].items():pin(path,value)
 text=(ROOT/outcome['log']).read_text()
 assert '+ __st=0\n' in text and re.search(r'^\+ Exit(?: 0)?$',text,re.M)
 writers[impl]=re.findall(r'^\+ timeout 10 env --default-signal=PIPE (.*)$',text,re.M)
 assert len(writers[impl])==262
 assert len([w for w in writers[impl] if w.endswith('2>full.err >/dev/full)')])==131
 assert len([w for w in writers[impl] if w.endswith('2>pipe.err | :)')])==131
 commands[impl]=Counter();classes=Counter();findings=[]
 assert len(outcome['memory'])==1336
 for memory in outcome['memory']:
  path=ROOT/memory['log'];log=path.read_text();parsed=runner.parse_memory_log(log,path.stem)
  assert all(memory[k]==v for k,v in parsed.items())
  assert runner.parse_memory_log(log,path.stem,True)['complete_exec_log']
  command,=re.findall(r'^==\d+== Command: (.*)$',log,re.M)
  command=re.sub(r'/tmp/rboxc-upstream-[^/ ]+','<fixture>',command)
  command=re.sub(r'gt-io-errors.sh\.[A-Za-z0-9]+','<test>',command)
  commands[impl][command]+=1
  lost={k:parsed['heap_bytes'].get(k,0) for k in ['definitely lost','indirectly lost','possibly lost']}
  bad=parsed['errors'] or parsed['non_inherited_descriptors'] or any(lost.values())
  classification='native-oracle-finding' if bad else 'clean'
  if impl=='rboxc' and bad:
   assert not re.search(r'Invalid (?:read|write|free)|uninitiali[sz]ed|Syscall param|Mismatched',log)
   assert 'suppressed: 0 from 0' in log
   if command.startswith('/bin/sh '):
    assert parsed['errors']==1 and not any(lost.values())
    if parsed['non_inherited_descriptors']==0:
     assert len(re.findall(r'^==\d+== File descriptor -1 Invalid file descriptor$',log,re.M))==1
     assert re.search(r'^==\d+==    at 0x[0-9A-F]+: close \(',log,re.M)
     classification='native-shell-invalid-close'
    else:
     assert parsed['non_inherited_descriptors']==1
     assert len(re.findall(r'^==\d+== Open file descriptor 0:$',log,re.M))==1
     classification='native-shell-pipe-stdin'
   else:
    assert command in ['<fixture>/real/cat foo','<fixture>/real/dd status=none if=foo','<fixture>/real/tac']
    assert 'default action of signal 13 (SIGPIPE)' in log and parsed['non_inherited_descriptors']==1
    assert not lost['definitely lost'] and not lost['indirectly lost']
    assert parsed['errors']==(2 if command.endswith('/tac') else 1)
    assert lost['possibly lost']==(16388 if command.endswith('/tac') else 0)
    classification='default-sigpipe-live-resources'
  classes[classification]+=1
  if bad:findings.append({'command':command,'classification':classification,'memory':memory})
 summaries[impl]={'status':0,'process_images':1336,'writer_checks':262,'classes':dict(classes),'findings':findings}
assert writers['gnu']==writers['rboxc'] and commands['gnu']==commands['rboxc']
assert summaries['rboxc']['classes']=={'clean':1070,'native-shell-invalid-close':132,'native-shell-pipe-stdin':131,'default-sigpipe-live-resources':3}
base_name='evidence/gnu-sort-fd-profile-coverage.json';base=read(base_name);coverage=copy.deepcopy(base)
for entry in coverage['results']:
 if entry['script']==script:
  assert entry['state']=='passed' and entry['valgrind_state']=='open'
  entry['previous_valgrind']={k:v for k,v in entry.items() if k.startswith('valgrind')}
  entry.update(valgrind_evidence='evidence/raw/'+stem+'.json',valgrind_audit=str(target.relative_to(ROOT)),
   strict_valgrind_pass=False,latest_original_assertions_pass=True,latest_valgrind_classes=summaries['rboxc']['classes'])
assert all(a==b for a,b in zip(base['results'],coverage['results']) if a['script']!=script)
coverage.update(base_report=base_name,base_report_sha256=pin(base_name),scope='Whole finite I/O-error original revalidated with 262 matching writer checks and 1336 images per implementation. Native shell and original SIGPIPE resource findings are explicitly retained. No native or strict Valgrind pass-count changes.')
ledger.write_text(json.dumps(coverage,indent=2)+'\n')
target.write_text(json.dumps({'scope':coverage['scope'],'results':summaries,'command_inventory':dict(commands['rboxc']),
 'raw':raw,'coverage':str(ledger.relative_to(ROOT)),'coverage_sha256':pin(ledger),
 'original_assertions_pass':True,'accounting_pass':True,'strict_valgrind_pass':False,'full_suite_pass':False},indent=2)+'\n')
print('PASS: 262 original writer checks and 1336 matched images; 1070 clean candidate images; shell and SIGPIPE findings retained')
