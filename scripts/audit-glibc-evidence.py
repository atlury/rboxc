#!/usr/bin/env python3
"""Recheck glibc utility comparisons and every recorded instrumented process."""
# SPDX-License-Identifier: GPL-3.0-or-later
import argparse
import importlib.util
import json
from pathlib import Path
import re
import sys
ROOT=Path(__file__).resolve().parents[1]
sys.path.insert(0,str(ROOT/'tests'));sys.path.insert(0,str(ROOT/'tests/gnu'))
from comparison_profile import fingerprint
spec=importlib.util.spec_from_file_location('reviewed',ROOT/'tests/gnu/reviewed-original.py')
runner=importlib.util.module_from_spec(spec);spec.loader.exec_module(runner)
parser=argparse.ArgumentParser(description=__doc__)
parser.add_argument('--focused',type=Path,required=True)
parser.add_argument('--original',type=Path,required=True)
parser.add_argument('--limits',type=Path,required=True)
parser.add_argument('--report-name',required=True)
options=parser.parse_args()
assert re.fullmatch(r'[a-z0-9-]+',options.report_name)
target=ROOT/'evidence'/(options.report_name+'.json');assert not target.exists()
focused=json.loads(options.focused.read_text())
original=json.loads(options.original.read_text())
limits=json.loads(options.limits.read_text())
assert focused['complete'] and focused['passed']==focused['total']==focused['planned_total']==50
assert focused['driver_sha256']==fingerprint(ROOT/'tests/glibc-behavior.py')
expected=fingerprint(Path(focused['binary']));inputs={}
for report in (focused,original,limits):
 assert report['binary_sha256']==expected
 assert report['complete'] and report['passed']==report['total']==report['planned_total']
 inputs.update(report.get('inputs',{}));inputs.update(report['runtime_helpers'])
for oracle in focused['oracles'].values():inputs[oracle['path']]=oracle['sha256']
profile=json.loads((ROOT/'evidence/glibc-build-profile.json').read_text())
inputs.update(profile['runtime'])
for path,value in inputs.items():assert fingerprint(Path(path))==value,path
processes=[]
def audit(recorded,log,sha,candidate,case):
 path=ROOT/log;assert fingerprint(path)==sha
 contents=path.read_text();pids=set(re.findall(r'^==([0-9]+)==',contents,re.M));assert len(pids)==1
 parsed=runner.parse_memory_log(contents,pids.pop(),exec_only=True)
 assert all(recorded[k]==v for k,v in parsed.items()) and parsed['complete_exec_log']
 if candidate:
  assert parsed['errors']==0 and parsed['non_inherited_descriptors']==0
  assert not any(parsed['heap_bytes'].get(k,0) for k in ('definitely lost','indirectly lost','possibly lost'))
  processes.append({'case':case,'log':log,'sha256':sha,**parsed,'pass':True})
for row in focused['results']:
 assert row['pass'] and row['memory_clean'] and row['equivalent']
 for key,outcome in row['outcomes'].items():
  assert all(outcome[k]==row['outcomes']['gnu'][k] for k in ('status','stdout','stderr','tree'))
  if 'memory' in outcome:audit(outcome['memory'],outcome['log'],outcome['log_sha256'],key=='rboxc-valgrind',row['name'])
assert original['expected_queries_per_mode']==191
for key,outcome in original['outcomes'].items():
 assert outcome['pass'] and outcome['status']==0 and outcome['queries']==191
 assert fingerprint(ROOT/outcome['log'])==outcome['log_sha256']
 assert len(outcome['memory'])==(191 if key.endswith('-valgrind') else 0)
 for log in outcome['memory']:audit(log,log['log'],log['sha256'],key=='rboxc-valgrind','tst-getconf.sh')
for key,outcome in limits['outcomes'].items():
 assert outcome['pass'] and outcome['status']==0 and not outcome['stdout'] and not outcome['stderr']
 if key.endswith('-valgrind'):
  log=outcome['memory'];audit(log,log['log'],log['sha256'],key=='rboxc-valgrind','tst-getconf-limits.py')
assert len(processes)==242
report={'scope':'50 focused GNU utility comparisons and both reviewed original getconf tests pass on the recorded host glibc 2.43 profile. Raw logs and inputs are hash-checked; every candidate memory summary is reparsed. Iconv original scripts, charmaps, and other runtime profiles remain open.',
 'binary':focused['binary'],'binary_sha256':expected,'runtime_helpers':focused['runtime_helpers'],
 'inputs':inputs,'reports':{str(p):fingerprint(p) for p in (options.focused,options.original,options.limits)},
 'focused_cases':50,'original_getconf_tests':2,'passed':len(processes),'total':len(processes),
 'driver_sha256':fingerprint(Path(__file__)),'results':processes}
target.write_text(json.dumps(report,indent=2)+'\n')
print('Audited 50 focused cases, two original getconf tests, and 242 clean candidate process logs')
