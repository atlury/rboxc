#!/usr/bin/env python3
"""Account for intentionally closed stdout without suppressing any Valgrind error."""
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
target=ROOT/'evidence/gnu-close-stdout-profile-validation.json';ledger=ROOT/'evidence/gnu-close-stdout-profile-coverage.json'
assert not target.exists() and not ledger.exists()
stem='gnu-close-stdout-current-valgrind'
report=read(f'evidence/raw/{stem}.json');checkpoint=read(f'evidence/raw/{stem}.progress.json')
row,=report['results'];assert row['script']=='tests/misc/close-stdout.sh' and row['state']=='open' and not row['pass']
saved=checkpoint['runs'][row['script']];context=saved['context'];assert context['valgrind']
source=Path(context['source'])/row['script'];pin(source,row['sha256'])
for path,value in context['drivers'].items():pin('evidence/raw/gnu-nproc-policy-closed-driver.py' if path=='tests/gnu/reviewed-original.py' else path,value)
for path,value in context['gnu_harness'].items():pin(Path(context['source'])/path,value)
for path,value in context['valgrind_runtime'].items():pin(path,value)
expected={'printf foo','cp --verbose a b','mktemp tmpfile-XXXXXX','mktemp tmpfile-XXXXXX -q'}
for snippet in ["returns_ 1 \"$p/src/printf\" 'foo' >&-",'returns_ 1 cp --verbose a b >&-',
                'returns_ 1 mktemp tmpfile-XXXXXX >&-','returns_ 1 mktemp tmpfile-XXXXXX -q >&-']:
 assert snippet in source.read_text()
results={}
for impl in ['gnu','rboxc']:
 outcome=row[impl];assert outcome==saved['outcomes'][impl]['result'] and outcome['status']==0
 pin('target/pr-page-cleanup-candidate/release/rboxc' if impl=='rboxc' else 'build/gnu-coreutils/src/coreutils',outcome['binary_sha256'])
 for path,value in saved['outcomes'][impl]['logs'].items():pin(path,value)
 log=(ROOT/outcome['log']).read_text()
 assert '+ __st=0\n' in log and re.search(r'^\+ Exit(?: 0)?$',log,re.M)
 images=[];expected_images=[]
 for memory in outcome['memory']:
  p=ROOT/memory['log'];text=p.read_text();parsed=runner.parse_memory_log(text,p.stem)
  assert all(memory[k]==v for k,v in parsed.items())
  command,=re.findall(r'^==\d+== Command: (.*)$',text,re.M)
  classification='native-oracle-finding' if parsed['errors'] else 'clean'
  if impl=='rboxc':
   assert parsed['non_inherited_descriptors']==0 and parsed['errors'] is not None
   assert not any(parsed['heap_bytes'].get(k,0) for k in ['definitely lost','indirectly lost','possibly lost'])
   assert not re.search(r'Invalid (?:read|write|free)|uninitiali[sz]ed|Syscall param|Mismatched',text)
   if parsed['errors']:
    assert command in expected and parsed['errors']==3
    headers=list(re.finditer(r'^==\d+== File descriptor (.*)$',text,re.M))
    assert len(headers)==3
    assert all(re.fullmatch(r'1 (?:was closed already|is already closed)',h[1]) or
               re.fullmatch(r'1: .+ is already closed',h[1]) for h in headers)
    frames=[re.search(r'^==\d+==    at 0x[0-9A-F]+: ([^\n]+)',text[h.end():],re.M)[1] for h in headers]
    assert frames[0].startswith('fstat (') and frames[1].startswith('write (') and frames[2].startswith('__close_nocancel (')
    # All three reported error contexts are accounted for. No suppression is added.
    assert 'suppressed: 0 from 0' in text
    classification='expected-closed-stdout-error'
    expected_images.append(command)
  images.append({'command':command,'classification':classification,'memory':parsed,'log':memory['log']})
 if impl=='rboxc':
  assert len(images)==28 and set(expected_images)==expected and len(expected_images)==4
 results[impl]=images
base_name='evidence/gnu-fixture-stream-cleanup-coverage.json';base=read(base_name);coverage=copy.deepcopy(base)
for entry in coverage['results']:
 if entry['script']==row['script']:
  assert entry['state']=='passed' and entry['valgrind_state']=='open'
  entry['previous_valgrind']={k:v for k,v in entry.items() if k.startswith('valgrind')}
  entry.update(valgrind_state='validated-expected-fd-errors',valgrind_evidence=f'evidence/raw/{stem}.json',
   valgrind_audit=str(target.relative_to(ROOT)),strict_valgrind_pass=False)
assert all(a==b for a,b in zip(base['results'],coverage['results']) if a['script']!=row['script'])
coverage['valgrind_counts']=dict(sorted(Counter(r['valgrind_state'] for r in coverage['results']).items()))
coverage.update(base_report=base_name,base_report_sha256=pin(base_name),scope='Native whole-script totals unchanged. One closed-stdout error profile fully accounted for: 24 clean candidate logs and four intentional descriptor-1 failure logs, each with three fd-only contexts. This is not a strict zero-error pass.')
ledger.write_text(json.dumps(coverage,indent=2)+'\n')
target.write_text(json.dumps({'scope':coverage['scope'],'results':results,'raw':raw,
 'coverage':str(ledger.relative_to(ROOT)),'coverage_sha256':pin(ledger),
 'candidate_clean_images':24,'candidate_expected_fd_error_images':4,'expected_error_contexts':12,
 'accounting_pass':True,'behavior_pass':True,'strict_valgrind_pass':False,'full_suite_pass':False},indent=2)+'\n')
print('PASS accounting: 24 clean logs, 4 intentional closed-stdout logs; strict zero-error result unchanged')
