#!/usr/bin/env python3
"""Retain native/instrumentation baselines for the complete execscript recipe."""
# SPDX-License-Identifier: GPL-3.0-or-later
import hashlib
import importlib.util
import json
from pathlib import Path
import sys
ROOT=Path(__file__).resolve().parents[1]
sys.path[:0]=[str(ROOT/'tests/gnu')]
spec=importlib.util.spec_from_file_location('reviewed',ROOT/'tests/gnu/reviewed-original.py')
runner=importlib.util.module_from_spec(spec);spec.loader.exec_module(runner)
target=ROOT/'evidence/bash-execscript-validation.json';assert not target.exists()
raw={}
def pin(path,expected=None):
 p=Path(path);p=p if p.is_absolute() else ROOT/p
 value=hashlib.sha256(p.read_bytes()).hexdigest();assert expected is None or value==expected,str(p)
 raw[str(p.relative_to(ROOT)) if p.is_relative_to(ROOT) else str(p)]=value
 return value
original='evidence/bash-execscript-original.json';report=json.loads((ROOT/original).read_text());pin(original)
snapshot='evidence/raw/bash-execscript-inventory.json';pin(snapshot)
inventory=json.loads((ROOT/snapshot).read_text());row,=[r for r in inventory['inputs'] if r['target']=='execscript']
for path,value in report['inputs'].items():
 if path.endswith('/inventory/bash-tests.json'):path=snapshot
 elif path.endswith('/tests/bash-original.py'):path='evidence/raw/bash-execscript-driver.py'
 elif path.endswith('/tests/bash-unprivileged-profile.py'):path='evidence/raw/bash-unprivileged-ready-helper.py'
 pin(path,value)
pin(report['binary'],report['binary_sha256'])
assert report['complete'] and report['total']==report['planned_total']==1 and report['passed']==0
result,=report['results'];assert not result['pass'] and result['selection']=='execscript'
outputs={};processes=[];bad=[];native=[]
for impl,outcome in result['outcomes'].items():
 assert outcome['status']==0 and not outcome['timed_out'] and not outcome['expected_output_matches']
 profile=outcome['unprivileged_profile'];assert profile['uid']==profile['gid']==65534 and profile['cap_effective']=='0000000000000000' and profile['groups']==[]
 for path,value in outcome['raw'].items():pin(path,value)
 actual,=[ROOT/p for p in outcome['raw'] if p.endswith('/actual')]
 outputs[impl]=actual.read_text().replace(profile['work'],'<WORK>')
 for m in outcome['memory']:
  pin(m['log'],m['sha256']);text=(ROOT/m['log']).read_text();parsed=runner.parse_memory_log(text,m['pid'])
  assert all(m[k]==v for k,v in parsed.items())
  if impl=='gnu-valgrind':native.append({'selection':'execscript',**m})
  if impl=='rboxc-valgrind':
   processes.append({'selection':'execscript',**m})
   if not m['clean']:
    bad.append(m)
    if m['complete']:
     assert m['errors']==2 and m['non_inherited_descriptors']==0 and m['heap_bytes']['definitely lost']==30
     assert 'rboxc_bash_restore_traps' in text
    else:assert 'EXEC FAILED: I can\'t recover from execve() failing' in text and 'errno 7' in text
 assert not impl.endswith('-valgrind') or len(outcome['memory'])==286
assert outputs['gnu']==outputs['rboxc'] and outputs['gnu-valgrind']==outputs['rboxc-valgrind']
assert len(bad)==2
for text in outputs.values():assert "trap -- '' SIGPIPE" in text and "trap -- '' SIGXFSZ" in text
assert "E2BIG: error message mismatch:" in outputs['gnu-valgrind'] and "this is <WORK>/exec/bash" in outputs['gnu-valgrind']
target.write_text(json.dumps({'scope':'Entire unchanged command-execution recipe and all 17 children. GNU and rboxc outputs agree within native/instrumented modes after only work-directory substitution for comparison; original expected output is not altered and this is an assertion baseline, not a pass. Native trap order differs from the original platform; the Python privilege helper adds ignored PIPE/XFSZ. Valgrind adds RTMAX, rewrites argv[0], and cannot recover from the ordinary E2BIG exec failure. A 30-byte trap-restoration leak and incomplete exec-failure image remain open.',
 'original':original,'original_sha256':pin(original),'binary_sha256':report['binary_sha256'],
 'inventory_snapshot':snapshot,'inventory_sha256':pin(snapshot),'raw':raw,'processes':[],
 'open_processes':processes,'native_processes':native,'assertion_baselines':['execscript'],
 'native_differential_pass':True,'instrumented_differential_pass':True,'open_profiles':bad,'full_bash_complete':False},indent=2)+'\n')
p=ROOT/'inventory/bash-tests.json';data=json.loads(p.read_text());entry,=[r for r in data['inputs'] if r['target']=='execscript'];assert entry==row
entry.update(state='reviewed-original-baseline-open',evidence=original,audit=str(target.relative_to(ROOT)))
p.write_text(json.dumps(data,indent=2)+'\n')
print('Audited execscript: 284 clean candidate images, 2 open, native and instrumented GNU baselines retained')
