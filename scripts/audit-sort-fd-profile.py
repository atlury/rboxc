#!/usr/bin/env python3
"""Audit seven effective sort descriptors after a declared instrumented startup allowance."""
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
sys.path[:0]=[str(ROOT/'tests/gnu'),str(ROOT/'scripts')]
spec=importlib.util.spec_from_file_location('reviewed',ROOT/'evidence/raw/gnu-sort-fd-final-driver.py')
runner=importlib.util.module_from_spec(spec);spec.loader.exec_module(runner)
raw={}
def pin(name,expected=None):
 p=Path(name);p=p if p.is_absolute() else ROOT/p
 value=hashlib.sha256(p.read_bytes()).hexdigest();assert expected is None or value==expected,str(p)
 raw[str(p.relative_to(ROOT)) if p.is_relative_to(ROOT) else str(p)]=value;return value
def read(name):pin(name);return json.loads((ROOT/name).read_text())
target=ROOT/'evidence/gnu-sort-fd-profile-validation.json';ledger=ROOT/'evidence/gnu-sort-fd-profile-coverage.json'
assert not target.exists() and not ledger.exists()
script='tests/sort/sort-continue.sh';summary={}
for instrument,stem in [(False,'gnu-sort-seven-fd-native'),(True,'gnu-sort-seven-soft-fd-valgrind')]:
 report=read(f'evidence/raw/{stem}.json');checkpoint=read(f'evidence/raw/{stem}.progress.json')
 row,=report['results'];assert row['script']==script and row['pass'] and row['state']=='pass' and row['full_suite']
 saved=checkpoint['runs'][script];context=saved['context'];assert context['valgrind']==instrument
 pin(Path(context['source'])/script,row['sha256'])
 for path,value in context['drivers'].items():pin('evidence/raw/gnu-sort-fd-final-driver.py' if path=='tests/gnu/reviewed-original.py' else path,value)
 for path,value in context['gnu_harness'].items():pin(Path(context['source'])/path,value)
 for path,value in (context.get('valgrind_runtime') or {}).items():pin(path,value)
 pin('/bin/bash',context['shell'])
 if instrument:
  config=context['sort_fd_profile']
  pin('tests/gnu/sort-fd-profile.c',config['source_sha256']);pin('tests/gnu/sort-fd-profile.bash',config['bash_sha256'])
  pin('build/sort-fd-profile-verified.so',config['library_sha256'])
 for impl in ['gnu','rboxc']:
  outcome=row[impl];assert outcome==saved['outcomes'][impl]['result'] and outcome['status']==0
  pin('target/pr-page-cleanup-candidate/release/rboxc' if impl=='rboxc' else 'build/gnu-coreutils/src/coreutils',outcome['binary_sha256'])
  for path,value in saved['outcomes'][impl]['logs'].items():pin(path,value)
  text=(ROOT/outcome['log']).read_text()
  assert text.count('+ ulimit -n 7\n')==3
  assert text.count('+ compare in out\n')==2
  assert '+ __st=0\n' in text and re.search(r'^\+ Exit(?: 0)?$',text,re.M)
  assert '+ seq 31\n' in text
  data={'status':0,'original_limit_requests':3,'merge_comparisons':2}
  if instrument:
   profile=outcome['sort_fd_profile'];pin(profile['log'],profile['sha256'])
   assert read(profile['log'])==profile['records'] and len(profile['records'])==3 and outcome['case_count_pass']
   assert all(r['soft']==7 and r['hard']==64 and r['before_main'] for r in profile['records'])
   assert len(outcome['memory'])==4
   recorded={str(r['pid']) for r in profile['records']};observed=set();commands=[];memories=[]
   for memory in outcome['memory']:
    p=ROOT/memory['log'];log=p.read_text();parsed=runner.parse_memory_log(log,p.stem)
    assert all(memory[k]==v for k,v in parsed.items())
    memories.append(parsed)
    if impl=='rboxc':
     assert parsed['errors']==parsed['non_inherited_descriptors']==0
     assert not any(parsed['heap_bytes'].get(k,0) for k in ['definitely lost','indirectly lost','possibly lost'])
    cmd,=re.findall(r'^==\d+== Command: (.*)$',log,re.M);commands.append(cmd)
    if cmd!='sort --version':observed.add(p.stem)
   assert observed==recorded and 'sort' in commands
   merges=[cmd for cmd in commands if cmd.startswith('sort -n -m ')]
   assert len(merges)==2
   expected=sorted('__test.'+str(i) for i in range(1,32))
   assert sorted([cmd.split()[3:] for cmd in merges],key=len)==[expected,expected+['-']]
   data.update(soft_limit=7,hard_limit=64,clean_processes=sum(m['errors']==m['non_inherited_descriptors']==0 for m in memories),memory=memories,limited_client_pids=sorted(recorded))
  else:
   assert 'sort_fd_profile' not in outcome
  summary[f'{impl}-{"valgrind" if instrument else "native"}']=data
# The algorithm cannot use the larger tool hard ceiling: neither source raises a limit.
for source in ['/opt/src/coreutils-9.11/src/sort.c','src/generated/applet_sort.rs']:
 pin(source);assert not re.search(r'\b(?:setrlimit|prlimit|prlimit64)\s*\(',Path(source if source.startswith('/') else ROOT/source).read_text())
for stem,driver,c_source,bash_source in [
 ('gnu-sort-seven-fd-valgrind','gnu-sort-fd-initial-driver.py','evidence/raw/gnu-sort-fd-initial-helper.c','evidence/raw/gnu-sort-fd-initial-bash'),
 ('gnu-sort-seven-fd-ready-valgrind','gnu-sort-fd-ready-driver.py','evidence/raw/gnu-sort-fd-initial-helper.c','evidence/raw/gnu-sort-fd-ready-bash'),
 ('gnu-sort-seven-fd-diagnostic-valgrind','gnu-sort-fd-ready-driver.py','evidence/raw/gnu-sort-fd-diagnostic-helper.c','evidence/raw/gnu-sort-fd-ready-bash')]:
 report=read(f'evidence/raw/{stem}.json');checkpoint=read(f'evidence/raw/{stem}.progress.json');old,=report['results']
 assert not old['pass'];saved=checkpoint['runs'][script];context=saved['context']
 pin('evidence/raw/'+driver,context['drivers']['tests/gnu/reviewed-original.py'])
 pin(c_source,context['sort_fd_profile']['source_sha256']);pin(bash_source,context['sort_fd_profile']['bash_sha256'])
 for impl in ['gnu','rboxc']:
  assert old[impl]['status']==77 and not old[impl]['case_count_pass']
  for path,value in saved['outcomes'][impl]['logs'].items():pin(path,value)
base_name='evidence/gnu-close-stdout-profile-coverage.json';base=read(base_name);coverage=copy.deepcopy(base)
for entry in coverage['results']:
 if entry['script']==script:
  assert entry['state']=='passed' and entry['valgrind_state']=='pending'
  entry['previous_valgrind']={k:v for k,v in entry.items() if k.startswith('valgrind')}
  entry.update(valgrind_state='passed-adapted-fixture',valgrind_evidence='evidence/raw/gnu-sort-seven-soft-fd-valgrind.json',
   adapted_fixture_native_evidence='evidence/raw/gnu-sort-seven-fd-native.json',adapted_fixture_audit=str(target.relative_to(ROOT)),
   limit_profile={'effective_soft':7,'instrumented_hard':64,'original_native_soft':7,'original_native_hard':7})
assert all(a==b for a,b in zip(base['results'],coverage['results']) if a['script']!=script)
coverage['valgrind_counts']=dict(sorted(Counter(r['valgrind_state'] for r in coverage['results']).items()))
coverage.update(base_report=base_name,base_report_sha256=pin(base_name),scope='One explicit sort startup profile passes with an effective seven-descriptor soft cap, verified before main. Valgrind hard ceiling remains 64; original native soft/hard seven-descriptor result retained. No native whole-script count changes.')
ledger.write_text(json.dumps(coverage,indent=2)+'\n')
target.write_text(json.dumps({'scope':coverage['scope'],'results':summary,'raw':raw,
 'coverage':str(ledger.relative_to(ROOT)),'coverage_sha256':pin(ledger),
 'accounting_pass':True,'adapted_fixture_pass':True,'unmodified_valgrind_pass':False,'full_suite_pass':False},indent=2)+'\n')
print('PASS: original merges, effective descriptor cap 7, four clean candidate logs; hard ceiling 64 explicitly retained')
