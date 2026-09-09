#!/usr/bin/env python3
"""Pin current ordinary lifecycle results without promoting open originals."""
# SPDX-License-Identifier: GPL-3.0-or-later
from collections import Counter
import hashlib
import importlib.util
import json
from pathlib import Path
import re
import sys
ROOT=Path(__file__).resolve().parents[1]
sys.path[:0]=[str(ROOT/'tests/gnu'),str(ROOT/'scripts')]
spec=importlib.util.spec_from_file_location('reviewed',ROOT/'evidence/raw/gnu-before-rm-root-driver.py')
runner=importlib.util.module_from_spec(spec);spec.loader.exec_module(runner)
raw={}
def pin(name,expected=None):
 p=Path(name);p=p if p.is_absolute() else ROOT/p
 value=hashlib.sha256(p.read_bytes()).hexdigest()
 assert expected is None or expected==value,str(p)
 raw[str(p.relative_to(ROOT)) if p.is_relative_to(ROOT) else str(p)]=value
 return value
def read(name):
 pin(name);return json.loads((ROOT/name).read_text())
target=ROOT/'evidence/gnu-current-lifecycle-validation.json'
assert not target.exists()
summary={}
for stem in ['gnu-tail-current-lifecycle-valgrind','gnu-install-stat-current-valgrind']:
 report=read(f'evidence/raw/{stem}.json')
 checkpoint=read(f'evidence/raw/{stem}.progress.json')
 for row in report['results']:
  assert row['state']=='open' and not row['pass']
  saved=checkpoint['runs'][row['script']];context=saved['context']
  assert context['valgrind']
  pin(Path(context['source'])/row['script'],row['sha256'])
  for path,value in context['drivers'].items():
   pin('evidence/raw/gnu-before-rm-root-driver.py' if path=='tests/gnu/reviewed-original.py' else path,value)
  for path,value in context['gnu_harness'].items():pin(Path(context['source'])/path,value)
  for path,value in context['valgrind_runtime'].items():pin(path,value)
  per_impl={}
  for impl in ['gnu','rboxc']:
   result=row[impl];assert saved['outcomes'][impl]['result']==result
   assert result['status']==(1 if row['script']=='tests/ls/stat-free-symlinks.sh' else 0)
   pin('target/pr-page-cleanup-candidate/release/rboxc' if impl=='rboxc' else 'build/gnu-coreutils/src/coreutils',result['binary_sha256'])
   for path,value in saved['outcomes'][impl]['logs'].items():pin(path,value)
   images=[]
   for m in result['memory']:
    p=ROOT/m['log'];text=p.read_text();parsed=runner.parse_memory_log(text,p.stem)
    assert all(m[k]==v for k,v in parsed.items())
    cmd,=re.findall(r'^==\d+== Command: (.*)$',text,re.M)
    clean=parsed['errors']==parsed['non_inherited_descriptors']==0 and not any(parsed['heap_bytes'].get(k,0) for k in ['definitely lost','indirectly lost','possibly lost'])
    images.append({'command':cmd,'memory':parsed,'clean':clean,'log':m['log']})
   per_impl[impl]={'status':result['status'],'images':images,'clean':sum(i['clean'] for i in images)}
  summary[row['script']]=per_impl
assert len(summary)==8
assert all(i['clean'] for i in summary['tests/ls/stat-free-symlinks.sh']['rboxc']['images'])
for i in summary['tests/install/strip-program.sh']['rboxc']['images']:
 if '/real/ginstall ' in i['command']:assert i['clean']
focused=read('evidence/tail-natural-exit-status.json')
assert focused['passed']==focused['total']==13
pin('tests/tail-normal-lifecycle.py',focused['driver_sha256'])
focused_clean=0
for row in focused['results']:
 assert row['pass']
 behavior=[]
 for impl,result in row['outcomes'].items():
  assert result['pass'] and result['normal_exit']
  assert result['status']==(1 if row['case'] in ['missing','headers'] else 0)
  assert result['stdout']==result['expected_stdout']
  behavior.append([result[k] for k in ['status','stdout','stderr']])
  for path,value in result['logs'].items():pin(path,value)
  if impl.endswith('valgrind'):
   log,=[path for path in result['logs'] if path.endswith('/memory.log')]
   parsed=runner.parse_memory_log((ROOT/log).read_text(),row['case'])
   assert parsed==result['memory']
   if impl.startswith('rboxc'):
    assert result['memory_clean'] and parsed['errors']==parsed['non_inherited_descriptors']==0
    assert not any(parsed['heap_bytes'].get(k,0) for k in ['definitely lost','indirectly lost','possibly lost'])
    focused_clean+=1
 assert all(r==behavior[0] for r in behavior)
initial=read('evidence/tail-natural-exit-current.json')
pin('evidence/raw/tail-natural-exit-initial-driver.py',initial['driver_sha256'])
assert initial['passed']==9 and initial['total']==13
assert {r['case'] for r in initial['results'] if not r['pass']}=={'missing','headers'}
for row in initial['results']:
 for result in row['outcomes'].values():
  for path,value in result['logs'].items():pin(path,value)
pin('evidence/gnu-tail-profiles-coverage.json')
target.write_text(json.dumps({'scope':'Eight current original Valgrind profiles remain open. Focused natural exits pass separately; no whole-script or Valgrind ledger counts change.',
 'original_results':summary,'focused_report':'evidence/tail-natural-exit-status.json',
 'focused_candidate_clean':focused_clean,'accounting_pass':True,'whole_original_valgrind_pass':False,'raw':raw},indent=2)+'\n')
print('PASS accounting: 8 open originals; 13 clean focused candidate exits')
