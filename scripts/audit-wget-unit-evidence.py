#!/usr/bin/env python3
"""Audit the separate original GNU Wget helper unit-test profile."""
# SPDX-License-Identifier: GPL-3.0-or-later
import argparse
import importlib.util
import json
from pathlib import Path
import re
import sys
ROOT=Path(__file__).resolve().parents[1]
sys.path.insert(0,str(ROOT/'tests'))
sys.path.insert(0,str(ROOT/'tests/gnu'))
from comparison_profile import fingerprint
spec=importlib.util.spec_from_file_location('reviewed',ROOT/'tests/gnu/reviewed-original.py')
runner=importlib.util.module_from_spec(spec);spec.loader.exec_module(runner)
parser=argparse.ArgumentParser(description=__doc__)
parser.add_argument('--report',type=Path,required=True)
parser.add_argument('--report-name',required=True)
args=parser.parse_args();assert re.fullmatch(r'[a-z0-9-]+',args.report_name)
target=ROOT/'evidence'/(args.report_name+'.json');assert not target.exists()
data=json.loads(args.report.read_text())
assert data['total']==data['passed']==4 and data['tests_per_profile']==17
for name,h in data['inputs'].items():assert fingerprint(Path(name))==h
for name,h in data['raw'].items():assert fingerprint(ROOT/name)==h
source=Path(json.loads((ROOT/'inventory/sources.json').read_text())['wget']['source'])
text=(source/'tests/unit-tests.c').read_text()
expected=set(re.findall(r'mu_run_test \((test_[a-z_]+)\)',text))
expected-={'test_has_key','test_find_key_value','test_find_key_values'}
assert len(expected)==17
assert {r['profile'] for r in data['results']}=={'gnu','gnu-valgrind','adapted-helpers','adapted-helpers-valgrind'}
processes=[];reference=None
for row in data['results']:
    assert row['pass'] and row['status']==0 and set(row['tests'])==expected and len(row['tests'])==17
    path=ROOT/row['output'];assert fingerprint(path)==row['output_sha256']
    output=path.read_bytes()
    assert set(n.decode() for n in re.findall(rb'^RUNNING TEST (test_[a-z_]+)\.\.\.$',output,re.M))==expected
    assert output.count(b'PASSED\n')==18 and b'ALL TESTS PASSED\nTests run: 17\n' in output
    if reference is None:reference=output
    assert output==reference
    if 'memory' in row:
        path=ROOT/row['memory_log'];assert fingerprint(path)==row['memory_log_sha256']
        contents=path.read_text();pids=set(re.findall(r'^==([0-9]+)==',contents,re.M));assert len(pids)==1
        parsed=runner.parse_memory_log(contents,pids.pop(),exec_only=True)
        assert parsed==row['memory'] and parsed['complete_exec_log']
        assert parsed['errors']==0 and parsed['non_inherited_descriptors']==0
        assert 'in use at exit: 0 bytes in 0 blocks' in contents
        assert re.findall(r'^==[0-9]+== Command: (.*)$',contents,re.M)==[row['binary']]
        processes.append({'profile':row['profile'],'log':row['memory_log'],'sha256':row['memory_log_sha256'],**parsed,'pass':True})
assert len(processes)==2
failed=ROOT/'evidence/raw/wget-unit-initial-run.log'
target.write_text(json.dumps({'scope':'All 17 unchanged configured GNU helper units pass in both native C TESTING executables, natively and under Valgrind. Both instrumented processes have zero errors, no new descriptors and zero heap bytes at exit. This is helper-profile evidence, not multicall/Rust entry coverage.', 'report':str(args.report),'report_sha256':fingerprint(args.report),'driver_sha256':fingerprint(Path(__file__)),'inputs':data['inputs'],'tests_per_profile':17,'passed':2,'total':2,'retained_initial_harness_failure':{'path':str(failed.relative_to(ROOT)),'sha256':fingerprint(failed),'reason':'Forced dry-run Make requested unavailable autotools regeneration; replaced with a targeted source prerequisite dry-run.'},'results':processes},indent=2)+'\n')
print('Audited all 17 helper units and two clean native test processes')
