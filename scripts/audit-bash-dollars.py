#!/usr/bin/env python3
"""Verify the complete positional-parameter recipe and all original child scripts."""
# SPDX-License-Identifier: GPL-3.0-or-later
import hashlib
import importlib.util
import json
from pathlib import Path
import re
import sys
ROOT=Path(__file__).resolve().parents[1]
sys.path[:0]=[str(ROOT/'tests/gnu')]
spec=importlib.util.spec_from_file_location('reviewed',ROOT/'tests/gnu/reviewed-original.py')
runner=importlib.util.module_from_spec(spec);spec.loader.exec_module(runner)
target=ROOT/'evidence/bash-dollars-validation.json';assert not target.exists()
raw={}
def pin(path,expected=None):
 p=Path(path);p=p if p.is_absolute() else ROOT/p
 value=hashlib.sha256(p.read_bytes()).hexdigest();assert expected is None or value==expected,str(p)
 raw[str(p.relative_to(ROOT)) if p.is_relative_to(ROOT) else str(p)]=value
 return value
original='evidence/bash-dollars-original.json';pin(original);report=json.loads((ROOT/original).read_text())
assert report['complete'] and report['passed']==report['total']==report['planned_total']==1
snapshot='evidence/raw/bash-dollars-inventory.json';pin(snapshot)
inventory=json.loads((ROOT/snapshot).read_text());row,=[r for r in inventory['inputs'] if r['target']=='dollars']
for path,value in report['inputs'].items():
 if path.endswith('/inventory/bash-tests.json'):path=snapshot
 elif path.endswith('/tests/bash-original.py'):path='evidence/raw/bash-dollars-driver.py'
 pin(path,value)
pin(report['binary'],report['binary_sha256'])
source=Path('/opt/src/bash-5.3/tests');main=(source/row['script']).read_text()
children=re.findall(r'^\$\{THIS_SH\} ./([^\s]+)',main,re.M)
assert len(children)==29 and set(children)=={p.name for p in source.glob('dollar*.sub')}
assert set(row['fixtures'])=={row['script'],row['expected'],*children}
expected=(source/row['expected']).read_bytes();processes=[];native=[]
result,=report['results'];assert result['pass'] and result['selection']=='dollars'
for impl,outcome in result['outcomes'].items():
 assert outcome['status']==0 and not outcome['timed_out'] and outcome['expected_output_matches']
 for path,value in outcome['raw'].items():pin(path,value)
 actual,=[ROOT/p for p in outcome['raw'] if p.endswith('/actual')]
 assert actual.read_bytes()==expected
 images=[]
 for memory in outcome['memory']:
  pin(memory['log'],memory['sha256']);text=(ROOT/memory['log']).read_text()
  parsed=runner.parse_memory_log(text,memory['pid'])
  assert all(memory[k]==v for k,v in parsed.items())
  assert memory['complete'] and runner.parse_memory_log(text,memory['pid'],True)['complete_exec_log']
  commands=re.findall(r'^==\d+== Command: (.*)$',text,re.M);images.extend(commands)
  if impl=='rboxc-valgrind':
   assert memory['clean'] and parsed['errors']==parsed['non_inherited_descriptors']==0
   assert not any(parsed['heap_bytes'].get(k,0) for k in ['definitely lost','indirectly lost','possibly lost'])
   processes.append({'selection':'dollars',**memory})
  elif impl=='gnu-valgrind':native.append(memory)
 if impl.endswith('-valgrind'):
  assert len(outcome['memory'])==420
  assert all(any(cmd.endswith(' ./'+child) for cmd in images) for child in children)
assert len(processes)==420
target.write_text(json.dumps({'scope':'Complete unchanged run-dollars recipe: dollar-at-star plus all 29 child scripts, original combined output, full child tracing and shared original recho helper. All 420 candidate process logs are clean.',
 'original':original,'original_sha256':pin(original),'binary_sha256':report['binary_sha256'],
 'inventory_snapshot':snapshot,'inventory_sha256':pin(snapshot),'raw':raw,'processes':processes,
 'open_processes':[],'assertion_baselines':[],'native_processes':native,'full_bash_complete':False},indent=2)+'\n')
current=ROOT/'inventory/bash-tests.json';data=json.loads(current.read_text());entry,=[r for r in data['inputs'] if r['target']=='dollars']
assert entry==row
entry.update(state='reviewed-original-pass',evidence=original,audit=str(target.relative_to(ROOT)))
current.write_text(json.dumps(data,indent=2)+'\n')
print('PASS: whole dollars recipe, all 29 child scripts, 420 clean candidate process logs')
