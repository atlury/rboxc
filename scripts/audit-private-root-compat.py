#!/usr/bin/env python3
"""Check that adding rm isolation preserves the existing ownership root profile."""
# SPDX-License-Identifier: GPL-3.0-or-later
import hashlib
import importlib.util
import json
from pathlib import Path
import re
import sys
ROOT=Path(__file__).resolve().parents[1]
sys.path[:0]=[str(ROOT/'tests/gnu'),str(ROOT/'scripts')]
spec=importlib.util.spec_from_file_location('reviewed',ROOT/'evidence/raw/gnu-rm-root-counted-driver.py')
runner=importlib.util.module_from_spec(spec);spec.loader.exec_module(runner)
raw={}
def pin(name,expected=None):
 p=Path(name);p=p if p.is_absolute() else ROOT/p
 value=hashlib.sha256(p.read_bytes()).hexdigest();assert expected is None or value==expected,str(p)
 raw[str(p.relative_to(ROOT)) if p.is_relative_to(ROOT) else str(p)]=value
 return value
def read(name):pin(name);return json.loads((ROOT/name).read_text())
target=ROOT/'evidence/gnu-private-root-compat-validation.json';assert not target.exists()
summary={}
for kind in ['original','valgrind']:
 stem='gnu-private-root-compat-'+kind
 report=read(f'evidence/raw/{stem}.json');checkpoint=read(f'evidence/raw/{stem}.progress.json')
 row,=report['results'];assert row['script']=='tests/chown/preserve-root.sh' and row['pass'] and row['state']=='pass'
 saved=checkpoint['runs'][row['script']];context=saved['context']
 pin(Path(context['source'])/row['script'],row['sha256'])
 pin('evidence/raw/gnu-rm-root-counted-helper.py',context['root_driver'])
 for path,value in context['drivers'].items():pin('evidence/raw/gnu-rm-root-counted-driver.py' if path=='tests/gnu/reviewed-original.py' else path,value)
 for path,value in context['gnu_harness'].items():pin(Path(context['source'])/path,value)
 for path,value in context.get('root_valgrind_runtime',{}).items():pin(path,value)
 for impl in ['gnu','rboxc']:
  result=row[impl];assert result==saved['outcomes'][impl]['result']
  assert result['status']==0 and result['case_count']==7 and result['case_count_pass']
  assert result['root_guard_commands']==runner.ROOT_GUARD_COMMANDS and result['parent_root_unchanged']
  profile=result['private_root']
  assert profile['uid']==profile['gid']==65534 and not profile['groups'] and profile['no_new_privileges']
  assert profile['root_identity']!=profile['host_root_identity']
  assert profile['mount_namespace']!=profile['parent_mount_namespace'] and profile['pid_namespace']!=profile['parent_pid_namespace']
  assert profile['private_proc_options']=='ro,nosuid,nodev,noexec'
  for path,value in profile['staged_files'].items():pin(path,value)
  for path,value in saved['outcomes'][impl]['logs'].items():pin(path,value)
  clean=0
  for m in result.get('memory',[]):
   p=ROOT/m['log'];parsed=runner.parse_memory_log(p.read_text(),p.stem,True)
   assert all(m[k]==v for k,v in parsed.items())
   ok=parsed['errors']==parsed['non_inherited_descriptors']==0 and not any(parsed['heap_bytes'].get(k,0) for k in ['definitely lost','indirectly lost','possibly lost'])
   if impl=='rboxc':assert ok
   clean+=ok
  if kind=='valgrind':
   assert len(result['memory'])==9
   assert result['instrumented_root_guard_commands']==sorted(runner.ROOT_GUARD_COMMANDS)
  summary[f'{kind}-{impl}']={'cases':7,'status':0,'images':len(result.get('memory',[])),'clean':clean}
target.write_text(json.dumps({'scope':'Existing ownership private-root profile still passes unchanged original assertions normally and under Valgrind. Nine clean candidate process logs; no new suite passes.',
 'results':summary,'raw':raw,'accounting_pass':True,'regression_pass':True},indent=2)+'\n')
print('PASS existing root profile: 7 cases per implementation, 9 clean candidate process logs')
