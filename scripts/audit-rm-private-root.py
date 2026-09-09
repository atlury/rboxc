#!/usr/bin/env python3
"""Audit the whole original rm root guard inside its disposable unprivileged root."""
# SPDX-License-Identifier: GPL-3.0-or-later
from collections import Counter
import copy
import hashlib
import json
from pathlib import Path
import re
ROOT=Path(__file__).resolve().parents[1]
raw={}
def pin(name,expected=None):
 p=Path(name);p=p if p.is_absolute() else ROOT/p
 value=hashlib.sha256(p.read_bytes()).hexdigest();assert expected is None or value==expected,str(p)
 raw[str(p.relative_to(ROOT)) if p.is_relative_to(ROOT) else str(p)]=value
 return value
def read(name):pin(name);return json.loads((ROOT/name).read_text())
target=ROOT/'evidence/gnu-rm-private-root-validation.json'
ledger=ROOT/'evidence/gnu-rm-private-root-coverage.json'
assert not target.exists() and not ledger.exists()
stem='gnu-rm-private-root-counted-original'
report=read(f'evidence/raw/{stem}.json');checkpoint=read(f'evidence/raw/{stem}.progress.json')
row,=report['results'];assert row['script']=='tests/rm/r-root.sh' and row['state']=='pass' and row['pass'] and row['full_suite']
saved=checkpoint['runs'][row['script']];context=saved['context'];assert not context['valgrind']
pin(Path(context['source'])/row['script'],row['sha256'])
assert context['definition']=={k:row[k] for k in context['definition']}
pin('evidence/raw/gnu-rm-root-counted-helper.py',context['root_driver'])
for path,value in context['drivers'].items():pin('evidence/raw/gnu-rm-root-counted-driver.py' if path=='tests/gnu/reviewed-original.py' else path,value)
for path,value in context['gnu_harness'].items():pin(Path(context['source'])/path,value)
for name,value in context['root_tools'].items():pin('/usr/bin/'+name,value)
expected=['dir','file','/','--preserve-root /','//','///','////','rootlink/','rootlink2/','rootlink3/','--no-preserve /',
 '--preserve-root file1 / file2','--preserve-root //.','--preserve-root /./','--preserve-root /.//','--preserve-root /../',
 '--preserve-root /.././','--preserve-root /etc/..','--preserve-root rootlink/..','--preserve-root rootlink2/.',
 '--preserve-root rootlink3/./','--interactive=never --no-preserve-root /']
profiles={}
for impl in ['gnu','rboxc']:
 result=row[impl];assert result==saved['outcomes'][impl]['result']
 assert result['status']==0 and result['case_count']==22 and result['case_count_pass']
 assert result['parent_root_unchanged']
 for path,value in saved['outcomes'][impl]['logs'].items():pin(path,value)
 text=(ROOT/result['log']).read_text();profile=result['private_root']
 emitted,=re.findall(r'^RBOXC_PRIVATE_ROOT_PROFILE (.+)$',text,re.M)
 assert json.loads(emitted)==profile
 assert text.count('RBOXC_PRIVATE_ROOT_PARENT_UNCHANGED')==1
 assert profile['uid']==profile['gid']==65534 and profile['groups']==[]
 assert profile['no_new_privileges'] and profile['private_null_device'] and profile['private_pid']>1
 assert profile['private_proc_options']=='rw,nosuid,nodev,noexec'
 assert profile['root_identity']!=profile['host_root_identity']
 assert profile['mount_namespace']!=profile['parent_mount_namespace']
 assert profile['pid_namespace']!=profile['parent_pid_namespace']
 assert profile['candidate_sha256']==result['binary_sha256']==context['binaries'][impl]
 pin('target/coreutils-debug-candidate/release/rboxc' if impl=='rboxc' else 'build/gnu-coreutils/src/coreutils',result['binary_sha256'])
 for path,value in profile['staged_files'].items():
  if path.startswith('/tmp/rboxc-upstream-') and path.endswith('/src/gdb'):
   wrapper='#!/bin/sh\nexec /usr/bin/gdb '+"'-iex=set disable-randomization off' '-iex=set auto-load off' "+"'-iex=set debuginfod enabled off' '-iex=set confirm off' \"$@\"\n"
   assert hashlib.sha256(wrapper.encode()).hexdigest()==value
  else:pin(path,value)
 for path in ['/usr/bin/gdb','/usr/bin/x86_64-linux-gnu-gcc-15','/usr/lib/x86_64-linux-gnu/libthread_db.so.1',
              '/opt/src/coreutils-9.11/src/remove.c']:
  assert path in profile['staged_files']
 calls=re.findall(r'^\+ (?:returns_ 1 )?(exercise_rm_r_root [^\n]*)$',text,re.M)
 assert calls==result['root_guard_commands']==['exercise_rm_r_root '+arg for arg in expected]
 assert '+ compare out_removed out\n' in text and '+ test -f excise.break\n' in text
 assert '+ __st=0\n' in text and re.search(r'^\+ Exit(?: 0)?$',text,re.M)
 profiles[impl]={'cases':22,'status':0,'staged_files':len(profile['staged_files']),
                 'root_identity':profile['root_identity'],'host_root_identity':profile['host_root_identity'],
                 'binary_sha256':result['binary_sha256']}
# Preserve setup skips and the first passing execution's incomplete trace counter.
for old,driver,helper,status,count in [
 ('gnu-rm-private-root-original','gnu-rm-root-driver.py','gnu-rm-root-initial-helper.py',77,1),
 ('gnu-rm-private-root-ready-original','gnu-rm-root-ready-driver.py','gnu-rm-root-ready-helper.py',0,3)]:
 oldreport=read(f'evidence/raw/{old}.json');oldcheckpoint=read(f'evidence/raw/{old}.progress.json')
 oldrow,=oldreport['results'];assert not oldrow['pass']
 oldsaved=oldcheckpoint['runs'][row['script']]
 pin('evidence/raw/'+driver,oldsaved['context']['drivers']['tests/gnu/reviewed-original.py'])
 pin('evidence/raw/'+helper,oldsaved['context']['root_driver'])
 for impl in ['gnu','rboxc']:
  assert oldrow[impl]['status']==status and oldrow[impl]['case_count']==count
  assert not oldrow[impl]['case_count_pass'] and oldrow[impl]['parent_root_unchanged']
  for path,value in oldsaved['outcomes'][impl]['logs'].items():pin(path,value)
base_name='evidence/gnu-tail-profiles-coverage.json';base=read(base_name);coverage=copy.deepcopy(base)
for entry in coverage['results']:
 if entry['script']==row['script']:
  assert entry['state']==entry['valgrind_state']=='pending'
  entry.update(state='passed',execution_coverage='full-script',evidence=f'evidence/raw/{stem}.json',
   profile='private-root',binary_sha256=row['rboxc']['binary_sha256'],audit=str(target.relative_to(ROOT)))
assert all(a==b for a,b in zip(base['results'],coverage['results']) if a['script']!=row['script'])
coverage['counts']=dict(sorted(Counter(r['state'] for r in coverage['results']).items()))
assert coverage['counts']=={'excluded':29,'partial':32,'passed':647,'pending':20,'skipped':5}
assert coverage['valgrind_counts']==base['valgrind_counts']
coverage.update(base_report=base_name,base_report_sha256=pin(base_name),scope='733-script ledger: whole rm root-guard original passes inside private unprivileged copies-only root. Separate optimized debug candidate; Valgrind remains pending.')
ledger.write_text(json.dumps(coverage,indent=2)+'\n')
target.write_text(json.dumps({'scope':coverage['scope'],'results':profiles,'raw':raw,
 'coverage':str(ledger.relative_to(ROOT)),'coverage_sha256':pin(ledger),
 'accounting_pass':True,'original_native_pass':True,'valgrind_pass':False,'full_suite_pass':False},indent=2)+'\n')
print('PASS: 22 original root-guard cases per implementation; 647 whole scripts passed')
