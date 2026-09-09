#!/usr/bin/env python3
"""Audit complete Bash glob and internationalization originals with clean locale discovery."""
# SPDX-License-Identifier: GPL-3.0-or-later
import hashlib
import importlib.util
import json
from pathlib import Path
import re
import sys
ROOT=Path(__file__).resolve().parents[1]
sys.path[:0]=[str(ROOT/'tests/gnu')]
spec=importlib.util.spec_from_file_location('reviewed',ROOT/'tests/gnu/reviewed-original.py');runner=importlib.util.module_from_spec(spec);spec.loader.exec_module(runner)
target=ROOT/'evidence/bash-locale-helper-validation.json';assert not target.exists()
raw={}
def pin(path,expected=None):
 p=Path(path);p=p if p.is_absolute() else ROOT/p
 value=hashlib.sha256(p.read_bytes()).hexdigest();assert expected is None or value==expected,str(p)
 raw[str(p.relative_to(ROOT)) if p.is_relative_to(ROOT) else str(p)]=value
 return value
original='evidence/bash-locale-helper-original.json';pin(original);report=json.loads((ROOT/original).read_text())
snapshot='evidence/raw/bash-locale-helper-inventory.json';pin(snapshot);inventory=json.loads((ROOT/snapshot).read_text())
assert report['complete'] and report['passed']==report['total']==report['planned_total']==2
for path,value in report['inputs'].items():
 if path.endswith('/inventory/bash-tests.json'):path=snapshot
 elif path.endswith('/tests/bash-original.py'):path='evidence/raw/bash-locale-helper-driver.py'
 elif path.endswith('/tests/bash-unprivileged-profile.py'):path='evidence/raw/bash-unprivileged-ready-helper.py'
 pin(path,value)
pin(report['binary'],report['binary_sha256'])
helper=json.loads((ROOT/'evidence/locale-test-helper-validation.json').read_text());assert helper['validation_pass'] and helper['comparisons']==helper['clean_images']==8 and not helper['applet_added']
for path,value in helper['raw'].items():pin(path,value)
processes=[];native=[];counts={};locale_images={}
source=Path('/opt/src/bash-5.3/tests')
for result in report['results']:
 name=result['selection'];entry,=[r for r in inventory['inputs'] if r['target']==name]
 assert result['pass'] and name in ['glob-test','intl']
 expected=(source/entry['expected']).read_bytes();locale_count=0
 for impl,outcome in result['outcomes'].items():
  assert outcome['status']==0 and not outcome['timed_out'] and outcome['expected_output_matches']
  for path,value in outcome['raw'].items():pin(path,value)
  actual,=[ROOT/p for p in outcome['raw'] if p.endswith('/actual')];assert actual.read_bytes()==expected
  assert outcome['private_locale']['source_sha256']==entry['locale_archive']['sha256']
  assert outcome['private_locale']['native_locale_helper']==str(ROOT/entry['locale_helper']['binary'])
  if name=='glob-test':
   profile=outcome['unprivileged_profile'];assert profile['uid']==profile['gid']==65534 and profile['groups']==[] and profile['cap_effective']=='0000000000000000'
   assert outcome['staged_executables']['locale']['sha256']==entry['locale_helper']['binary_sha256']
  commands=[]
  for m in outcome['memory']:
   pin(m['log'],m['sha256']);text=(ROOT/m['log']).read_text();parsed=runner.parse_memory_log(text,m['pid'])
   assert all(m[k]==v for k,v in parsed.items())
   assert m['complete'] and runner.parse_memory_log(text,m['pid'],True)['complete_exec_log']
   commands.extend(re.findall(r'^==\d+== Command: (.*)$',text,re.M))
   if impl=='rboxc-valgrind':
    assert m['clean'] and m['errors']==m['non_inherited_descriptors']==0
    assert not any(m['heap_bytes'].get(k,0) for k in ['definitely lost','indirectly lost','possibly lost'])
    processes.append({'selection':name,**m})
    locale_count+=bool(re.search(r'^==\d+== Command: .*?/exec/locale -a$',text,re.M))
   else:native.append({'selection':name,**m})
  if impl.endswith('-valgrind'):
   assert len(outcome['memory'])=={'glob-test':163,'intl':87}[name]
   children=[f for f in entry['fixtures'] if f.endswith('.sub')]
   assert all(any('./'+child in cmd for cmd in commands) for child in children)
 counts[name]=len(result['outcomes']['rboxc-valgrind']['memory']);locale_images[name]=locale_count
assert counts=={'glob-test':163,'intl':87} and locale_images=={'glob-test':1,'intl':4}
target.write_text(json.dumps({'scope':'Two complete unchanged original Bash recipes: glob and internationalization. All 250 candidate process images are clean, including five invocations of the explicitly adapted native GNU locale test helper. The helper is independently compared with unmodified GNU and host locale; no applet is added. Original assertions and locale test counts are unchanged.',
 'original':original,'original_sha256':pin(original),'binary_sha256':report['binary_sha256'],
 'inventory_snapshot':snapshot,'inventory_sha256':pin(snapshot),'raw':raw,'processes':processes,'open_processes':[],'assertion_baselines':[],'native_processes':native,
 'clean_images':counts,'locale_helper_images':locale_images,'native_helper_adapted':True,'full_bash_complete':False},indent=2)+'\n')
p=ROOT/'inventory/bash-tests.json';data=json.loads(p.read_text())
for name in ['glob-test','intl']:
 entry,=[r for r in data['inputs'] if r['target']==name];saved,=[r for r in inventory['inputs'] if r['target']==name];assert entry==saved
 entry.update(state='reviewed-original-pass',evidence=original,audit=str(target.relative_to(ROOT)))
p.write_text(json.dumps(data,indent=2)+'\n')
print('PASS: complete glob and intl originals; 250 clean process images, including 5 native helper images')
