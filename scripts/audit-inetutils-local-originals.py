#!/usr/bin/env python3
"""Audit complete original interface and local hostname command images."""
# SPDX-License-Identifier: GPL-3.0-or-later
from collections import Counter
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
target=ROOT/'evidence/inetutils-local-original-validation.json';assert not target.exists()
raw={}
def pin(path,expected=None):
 p=Path(path);p=p if p.is_absolute() else ROOT/p
 value=hashlib.sha256(p.read_bytes()).hexdigest();assert expected is None or expected==value,str(p)
 raw[str(p.relative_to(ROOT)) if p.is_relative_to(ROOT) else str(p)]=value
 return value
path=ROOT/'evidence/inetutils-local-original.json';pin(path);report=json.loads(path.read_text())
assert report['complete'] and len(report['results'])==4
pin(report['binary'],report['binary_sha256'])
for path,value in report['inputs'].items():pin(path,value)
summary={}
for row in report['results']:
 assert row['behavior_pass'];command=row['command'];expected=24 if command=='ifconfig' else 3
 commands={}
 for impl,outcome in row['outcomes'].items():
  assert outcome['status']==0 and outcome['behavior_pass']
  pin(outcome['log'],outcome['log_sha256']);text=(ROOT/outcome['log']).read_text()
  assert re.search(r'^\+ exit 0$',text,re.M)
  if command=='dnsdomainname':assert text.count('+ rc=0\n')==3
  ns=outcome['private_profile'];assert ns['uid']==ns['gid']==65534 and ns['groups']==[]
  assert ns['hostname']=='fixture.example.test' and [i['ifname'] for i in ns['interfaces']]==['lo']
  assert all(ns['namespaces'][k]!=ns['parent_namespaces'][k] for k in ['mnt','net','uts'])
  commands[impl]=Counter();bad=[]
  for memory in outcome['memory']:
   pin(memory['log'],memory['sha256']);path=ROOT/memory['log'];log=path.read_text()
   parsed=runner.parse_memory_log(log,path.stem.rsplit('-',1)[-1],True)
   assert all(memory[k]==v for k,v in parsed.items())
   assert parsed['complete_exec_log']
   if impl=='rboxc':
    assert parsed['errors']==parsed['non_inherited_descriptors']==0
    assert not any(parsed['heap_bytes'].get(k,0) for k in ['definitely lost','indirectly lost','possibly lost'])
   if parsed['errors'] or parsed['non_inherited_descriptors']:bad.append(memory)
   cmd,=re.findall(r'^==\d+== Command: \S+/real/'+command+r'(.*)$',log,re.M)
   commands[impl][cmd]+=1
  if row['instrumented']:
   assert len(outcome['memory'])==expected
   summary[command+'-'+impl]={'command_images':expected,'commands':dict(commands[impl]),'findings':bad}
 if row['instrumented']:
  assert row['strict_memory_pass'] and commands['gnu']==commands['rboxc']
  if command=='ifconfig':
   assert len([c for c in commands['rboxc'] if ' -A ' in c])==10
   for format in ['check','default','gnu','gnu-one-entry','netstat','net-tools','osf','unix']:
    assert commands['rboxc'][' --format='+format+' -i lo']>=1
target.write_text(json.dumps({'scope':report['scope'],'raw':raw,'binary_sha256':report['binary_sha256'],
 'results':summary,'original_scripts_passed':2,'clean_candidate_images':27,'full_provider_pass':False},indent=2)+'\n')
print('PASS: two whole GNU originals, matched command-image inventories, 27 clean candidate logs')
