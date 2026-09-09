#!/usr/bin/env python3
"""Audit explicit fixture-only stream cleanup without hiding unmodified findings."""
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
spec=importlib.util.spec_from_file_location('reviewed',ROOT/'evidence/raw/gnu-nproc-policy-closed-driver.py')
runner=importlib.util.module_from_spec(spec);spec.loader.exec_module(runner)
raw={}
def pin(name,expected=None):
 p=Path(name);p=p if p.is_absolute() else ROOT/p
 value=hashlib.sha256(p.read_bytes()).hexdigest();assert expected is None or value==expected,str(p)
 raw[str(p.relative_to(ROOT)) if p.is_relative_to(ROOT) else str(p)]=value;return value
def read(name):pin(name);return json.loads((ROOT/name).read_text())
profiles={
 'tests/dd/nocache_fail.sh':('gnu-dd-marker-closed','dd_marker_profile','  fopen ("called", "w");','  FILE *marker = fopen ("called", "w");\n  if (marker) fclose (marker);','k.marker-closed.c',2),
 'tests/nproc/nproc-quota.sh':('gnu-nproc-policy-closed','nproc_policy_profile','  if (pid == 0 && fscanfmt (policyf, "policy : %d", &policy) == 1)','  int matched = pid == 0 && fscanfmt (policyf, "policy : %d", &policy) == 1;\n  fclose (policyf);\n  if (matched)','k.policy-closed.c',20)}
target=ROOT/'evidence/gnu-fixture-stream-cleanup-validation.json';ledger=ROOT/'evidence/gnu-fixture-stream-cleanup-coverage.json'
assert not target.exists() and not ledger.exists()
summary={}
for script,(stem,key,before,after,adapted,images) in profiles.items():
 definitions={r['script']:r for r in read(f'evidence/raw/{stem}-inventory.json')}
 definition=definitions[script]
 for instrument in [False,True]:
  name=stem+('-valgrind' if instrument else '-original')
  report=read(f'evidence/raw/{name}.json');checkpoint=read(f'evidence/raw/{name}.progress.json')
  row,=report['results'];assert row['script']==script and row['pass'] and row['state']=='pass' and row['full_suite']
  saved=checkpoint['runs'][script];context=saved['context'];assert context['valgrind']==instrument
  assert context['definition']==definition and all(row[k]==v for k,v in definition.items())
  source=Path(context['source'])/script;pin(source,row['sha256'])
  stub=source.read_text().split("cat > k.c <<'EOF' || framework_failure_\n",1)[1].split('\nEOF\n',1)[0]+'\n'
  fixture=context[key]
  pin(f'evidence/raw/{stem}-helper.py',fixture['driver_sha256'])
  pin(fixture['compiler'],fixture['compiler_sha256'])
  assert hashlib.sha256(stub.encode()).hexdigest()==fixture['source_sha256']
  for path,value in context['drivers'].items():pin(f'evidence/raw/{stem}-driver.py' if path=='tests/gnu/reviewed-original.py' else path,value)
  for path,value in context['gnu_harness'].items():pin(Path(context['source'])/path,value)
  for path,value in (context.get('valgrind_runtime') or {}).items():pin(path,value)
  pin('/bin/sh',context['shell']);pin('build/gnu-coreutils/lib/config.h',context['config_header'])
  for impl in ['gnu','rboxc']:
   result=row[impl];assert result==saved['outcomes'][impl]['result'] and result['status']==0
   pin('target/pr-page-cleanup-candidate/release/rboxc' if impl=='rboxc' else 'build/gnu-coreutils/src/coreutils',result['binary_sha256'])
   for path,value in saved['outcomes'][impl]['logs'].items():pin(path,value)
   marker=result[key];pin(marker['log'],marker['sha256']);record=read(marker['log']);assert record==marker['record']
   assert record['original_source']==stub and stub.count(before)==1
   assert record['adapted_source']==stub.replace(before,after)
   assert record['original_sha256']==fixture['source_sha256']
   assert record['adapted_sha256']==hashlib.sha256(record['adapted_source'].encode()).hexdigest()
   assert record['effective_arguments']==[adapted if arg=='k.c' else arg for arg in record['original_arguments']]
   log=(ROOT/result['log']).read_text();assert '+ __st=0\n' in log and re.search(r'^\+ Exit(?: 0)?$',log,re.M)
   if key=='dd_marker_profile':
    assert '+ test -f called\n' in log and '+ test 1 = 1\n' in log
    assert "+ grep dd: failed to discard cache for: ifile err\n" in log
   elif instrument:
    pin('tests/gnu/nproc-quota-profile.py',context['quota_driver'])
    assert result['case_count_pass'] and result['quota_invocations']==len(result['quota_chroots'])==16
    for root in result['quota_chroots']:
     assert root['private_pid']==1 and root['nproc_sha256']==result['binary_sha256']
     assert root['private_mount_namespace']!=root['parent_mount_namespace'] and root['private_pid_namespace']!=root['parent_pid_namespace']
     for path,value in root['valgrind_runtime'].items():pin(path,value)
   commands=[]
   for memory in result.get('memory',[]):
    p=ROOT/memory['log'];text=p.read_text();parsed=runner.parse_memory_log(text,p.stem)
    assert all(memory[k]==v for k,v in parsed.items())
    assert parsed['errors']==parsed['non_inherited_descriptors']==0
    assert not any(parsed['heap_bytes'].get(k,0) for k in ['definitely lost','indirectly lost','possibly lost'])
    command,=re.findall(r'^==\d+== Command: (.*)$',text,re.M);commands.append(command)
   if instrument:assert len(commands)==images
   summary.setdefault(script,{})[f'{impl}-{"valgrind" if instrument else "native"}']={'status':0,'clean_images':len(commands),'commands':commands}
# Original helper findings remain visible and cannot become strict passes.
baselines={}
for script,path in [('tests/dd/nocache_fail.sh','evidence/raw/gnu-dd-marker-original-valgrind.json'),('tests/nproc/nproc-quota.sh','evidence/gnu-reviewed-valgrind.json')]:
 report=read(path);row,=[r for r in report['results'] if r['script']==script]
 assert not row['pass']
 for impl in ['gnu','rboxc']:
  result=row[impl];assert result['status']==0;pin(result['log'])
  findings=[]
  for memory in result['memory']:
   pin(memory['log']);p=ROOT/memory['log'];parsed=runner.parse_memory_log(p.read_text(),p.stem)
   assert all(memory[k]==v for k,v in parsed.items())
   if parsed['non_inherited_descriptors']:
    assert parsed['non_inherited_descriptors']==parsed['errors']==1
    assert parsed['heap_bytes'].get('still reachable')==472
    findings.append(memory['log'])
  assert findings;baselines.setdefault(script,{})[impl]={'open_marker_or_policy_stream_logs':findings}
base_name='evidence/gnu-printf-bounded-output-coverage.json';base=read(base_name);coverage=copy.deepcopy(base)
for row in coverage['results']:
 if row['script'] not in profiles:continue
 stem=profiles[row['script']][0];row['previous_valgrind']={k:v for k,v in row.items() if k.startswith('valgrind')}
 assert row['state']=='passed' and row['valgrind_state'] in ['pending','open']
 row.update(valgrind_state='passed-adapted-fixture',valgrind_evidence=f'evidence/raw/{stem}-valgrind.json',
  adapted_fixture_native_evidence=f'evidence/raw/{stem}-original.json',adapted_fixture_audit=str(target.relative_to(ROOT)))
assert all(a==b for a,b in zip(base['results'],coverage['results']) if a['script'] not in profiles)
coverage['valgrind_counts']=dict(sorted(Counter(r['valgrind_state'] for r in coverage['results']).items()))
assert coverage['counts']==base['counts']
coverage.update(base_report=base_name,base_report_sha256=pin(base_name),scope='Original native coverage unchanged. Two explicit fixture-stream cleanup profiles pass instrumented assertions; unmodified helper findings remain recorded. Adapted-fixture passes are separate from unmodified whole-script passes.')
ledger.write_text(json.dumps(coverage,indent=2)+'\n')
target.write_text(json.dumps({'scope':coverage['scope'],'results':summary,'unmodified_baselines':baselines,'raw':raw,
 'coverage':str(ledger.relative_to(ROOT)),'coverage_sha256':pin(ledger),'clean_candidate_images':22,
 'accounting_pass':True,'adapted_fixture_pass':True,'unmodified_valgrind_pass':False,'full_suite_pass':False},indent=2)+'\n')
print('PASS: 2 explicitly adapted fixtures, 22 clean candidate images;',coverage['valgrind_counts'])
