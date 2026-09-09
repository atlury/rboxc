#!/usr/bin/env python3
"""Verify finite EPIPE cleanup separately from default SIGPIPE termination."""
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
target=ROOT/'evidence/coreutils-finite-pipe-exit-validation.json';assert not target.exists()
raw={}
def pin(path,expected=None):
 p=Path(path);p=p if p.is_absolute() else ROOT/p
 value=hashlib.sha256(p.read_bytes()).hexdigest();assert expected is None or value==expected,str(p)
 raw[str(p.relative_to(ROOT)) if p.is_relative_to(ROOT) else str(p)]=value
 return value
path=ROOT/'evidence/coreutils-finite-pipe-exit.json';pin(path);report=json.loads(path.read_text())
assert report['complete'] and len(report['results'])==6
pin(report['binary'],report['binary_sha256']);pin('build/gnu-coreutils/src/coreutils',report['gnu_binary_sha256'])
pin('tests/coreutils-pipe-exit.py',report['driver_sha256'])
summaries=[];clean=0
for row in report['results']:
 assert row['behavior_pass'];ignored=row['sigpipe_ignored'];baseline=row['outcomes']['gnu-native']
 assert baseline['status']==(1 if ignored else -13)
 if ignored:assert b'Broken pipe' in bytes.fromhex(baseline['stderr'])
 else:assert not baseline['stderr']
 expected=b''.join(str(i).encode()+b'\n' for i in range(1,10001)) if row['command']=='tac' else b''
 images={}
 for label,outcome in row['outcomes'].items():
  assert outcome['status']==baseline['status'] and outcome['stderr']==baseline['stderr']
  assert outcome['input_bytes']==len(expected) and outcome['input_sha256']==hashlib.sha256(expected).hexdigest()
  if 'memory' in outcome:
   memory=outcome['memory'];path=ROOT/memory['log'];pin(path,memory['sha256']);text=path.read_text()
   pid,=re.findall(r'^==(\d+)== Command:',text,re.M);parsed=runner.parse_memory_log(text,pid,True)
   assert all(memory[k]==v for k,v in parsed.items()) and parsed['complete_exec_log']
   if not ignored:assert 'default action of signal 13 (SIGPIPE)' in text
   if ignored and label=='rboxc-valgrind':
    assert outcome['strict_memory_pass'] and parsed['errors']==parsed['non_inherited_descriptors']==0
    assert not any(parsed['heap_bytes'].get(k,0) for k in ['definitely lost','indirectly lost','possibly lost'])
    clean+=1
   images[label]=memory
 summaries.append({'command':row['command'],'sigpipe_ignored':ignored,'status':baseline['status'],'images':images})
assert clean==3
target.write_text(json.dumps({'scope':report['scope'],'raw':raw,'results':summaries,'clean_epipe_candidate_images':3,
 'comparisons':6,'whole_original_memory_pass':False,'full_suite_pass':False},indent=2)+'\n')
print('PASS: six GNU comparisons; three clean EPIPE cleanup paths; default signal profiles remain separate')
