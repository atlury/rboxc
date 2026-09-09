#!/usr/bin/env python3
"""Audit native printf output status under the original finite memory budget."""
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
 raw[str(p.relative_to(ROOT)) if p.is_relative_to(ROOT) else str(p)]=value;return value
def read(name):pin(name);return json.loads((ROOT/name).read_text())
target=ROOT/'evidence/gnu-printf-bounded-output-validation.json'
ledger=ROOT/'evidence/gnu-printf-bounded-output-coverage.json'
assert not target.exists() and not ledger.exists()
stem='gnu-printf-bounded-output-original'
report=read(f'evidence/raw/{stem}.json');checkpoint=read(f'evidence/raw/{stem}.progress.json')
row,=report['results'];assert row['script']=='tests/printf/printf-surprise.sh' and row['pass'] and row['state']=='pass' and row['full_suite']
saved=checkpoint['runs'][row['script']];context=saved['context'];assert not context['valgrind']
pin(Path(context['source'])/row['script'],row['sha256'])
for path,value in context['drivers'].items():pin('evidence/raw/gnu-rm-root-counted-driver.py' if path=='tests/gnu/reviewed-original.py' else path,value)
for path,value in context['gnu_harness'].items():pin(Path(context['source'])/path,value)
summary={}
for impl in ['gnu','rboxc']:
 outcome=row[impl];assert outcome==saved['outcomes'][impl]['result'] and outcome['status']==0
 pin('target/pr-page-cleanup-candidate/release/rboxc' if impl=='rboxc' else 'build/gnu-coreutils/src/coreutils',outcome['binary_sha256'])
 for path,value in saved['outcomes'][impl]['logs'].items():pin(path,value)
 text=(ROOT/outcome['log']).read_text()
 vm,=map(int,re.findall(r'^\+ vm=(\d+)$',text,re.M))
 limit,=map(int,re.findall(r'^\+ ulimit -v (\d+)$',text,re.M))
 assert vm==(14004 if impl=='gnu' else 31004) and limit==vm+4000
 for assertion in ['+ export MALLOC_PERTURB_=0','+ head -c 10 fifo','+ env printf %20000000f 0',
                   '+ exit=1','+ err_msg=printf: write error:','+ diagnostic=y','+ n_out=0','+ __st=0']:
  assert assertion+'\n' in text,assertion
 assert re.search(r'^\+ Exit(?: 0)?$',text,re.M)
 summary[impl]={'status':0,'calibrated_kib':vm,'exercised_kib':limit,'printf_status':1,'output_bytes':0,'diagnostic':'printf: write error','binary_sha256':outcome['binary_sha256']}
base_name='evidence/gnu-rm-private-root-coverage.json';base=read(base_name);coverage=copy.deepcopy(base)
for entry in coverage['results']:
 if entry['script']==row['script']:
  assert entry['state']==entry['valgrind_state']=='pending'
  entry.update(state='passed',execution_coverage='full-script',evidence=f'evidence/raw/{stem}.json',
    binary_sha256=row['rboxc']['binary_sha256'],audit=str(target.relative_to(ROOT)))
assert all(a==b for a,b in zip(base['results'],coverage['results']) if a['script']!=row['script'])
coverage['counts']=dict(sorted(Counter(r['state'] for r in coverage['results']).items()))
assert coverage['counts']=={'excluded':29,'partial':32,'passed':648,'pending':19,'skipped':5}
assert coverage['valgrind_counts']==base['valgrind_counts']
coverage.update(base_report=base_name,base_report_sha256=pin(base_name),scope='733-script ledger extended by the whole original finite printf allocation/output-status profile. Native limits are below Valgrind startup needs; instrumented resource acceptance remains pending.')
ledger.write_text(json.dumps(coverage,indent=2)+'\n')
target.write_text(json.dumps({'scope':coverage['scope'],'results':summary,'raw':raw,
 'coverage':str(ledger.relative_to(ROOT)),'coverage_sha256':pin(ledger),
 'accounting_pass':True,'original_native_pass':True,'valgrind_pass':False,'full_suite_pass':False},indent=2)+'\n')
print('PASS: finite printf ENOMEM output/status; 648 whole scripts passed')
