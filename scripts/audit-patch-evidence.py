#!/usr/bin/env python3
"""Verify current Patch comparisons and reparse every candidate process log."""
# SPDX-License-Identifier: GPL-3.0-or-later
import argparse
import importlib.util
import json
from pathlib import Path
import re
import sys

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT/'tests'))
sys.path.insert(0, str(ROOT/'tests/gnu'))
from comparison_profile import fingerprint
spec = importlib.util.spec_from_file_location('reviewed', ROOT/'tests/gnu/reviewed-original.py')
runner = importlib.util.module_from_spec(spec); spec.loader.exec_module(runner)
parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('--focused', type=Path, required=True)
parser.add_argument('--original', type=Path, required=True)
parser.add_argument('--report-name', required=True)
options = parser.parse_args()
assert re.fullmatch(r'[a-z0-9-]+', options.report_name)
target = ROOT/'evidence'/(options.report_name+'.json')
assert not target.exists(), 'preserve prior audits'
focused = json.loads(options.focused.read_text())
original = json.loads(options.original.read_text())
assert focused['binary_sha256'] == original['binary_sha256'] == fingerprint(Path(focused['binary']))
assert focused['complete'] and focused['passed'] == focused['total'] == focused['planned_total'] == 108
assert original['complete'] and original['passed'] == original['total'] == original['planned_total'] == 8
assert focused['driver_sha256'] == fingerprint(ROOT/'tests/entry-behavior.py')
assert original['inputs'][str(ROOT/'tests/patch-original.py')] == fingerprint(ROOT/'tests/patch-original.py')
assert {r['selection'] for r in original['results']} == {'backup-prefix-suffix','no-backup','inname','file-create-modes','empty-files','unmodified-files','unusual-blanks','file-modes'}
inputs = {}
for data in (focused, original):
    inputs.update(data.get('inputs', {})); inputs.update(data['runtime_helpers'])
for item in focused['oracles'].values():
    inputs[item['path']] = item['sha256']
for path, expected in inputs.items():
    assert fingerprint(Path(path)) == expected, 'recorded input changed: '+path
processes = []
def audit(log, expected_hash, recorded, candidate, case):
    path = ROOT/log
    assert fingerprint(path) == expected_hash, 'recorded log changed: '+log
    contents = path.read_text()
    pids = set(re.findall(r'^==([0-9]+)==', contents, re.M)); assert len(pids) == 1
    parsed = runner.parse_memory_log(contents, pids.pop(), exec_only=True)
    assert all(recorded[k] == v for k,v in parsed.items())
    assert parsed['complete_exec_log']
    if candidate:
        assert parsed['errors'] == 0 and parsed['non_inherited_descriptors'] == 0
        assert not any(parsed['heap_bytes'].get(k,0) for k in ('definitely lost','indirectly lost','possibly lost'))
        processes.append({'case':case,'log':log,'sha256':expected_hash,**parsed,'pass':True})
focused_rows=[r for r in focused['results'] if r['command']=='patch']
assert len(focused_rows)==23
for row in focused_rows:
    assert row['pass'] and row['equivalent'] and row['memory_clean']
    reference = row['outcomes']['gnu']
    for key, outcome in row['outcomes'].items():
        assert all(outcome[k] == reference[k] for k in ('status','stdout','stderr','tree'))
        if 'memory' in outcome:
            audit(outcome['log'],outcome['log_sha256'],outcome['memory'],key=='rboxc-valgrind',row['name'])
for row in original['results']:
    assert row['pass']
    for key,outcome in row['outcomes'].items():
        assert outcome['status']==0 and outcome['assertions_pass']
        assert len(outcome['assertion_counts'])==1
        total,passed,failed=outcome['assertion_counts'][0]
        assert total==passed and total>0 and failed==0
        assert fingerprint(ROOT/outcome['driver_log'])==outcome['driver_log_sha256']
        for log in outcome['memory']:
            contents=(ROOT/log['log']).read_text()
            commands=re.findall(r'^==[0-9]+== Command: (.*)$',contents,re.M)
            assert len(commands)==1 and commands[0].split()[0]=='patch', 'unclassified child process'
            audit(log['log'],log['sha256'],log,key=='rboxc-valgrind',row['source'])
original_processes=sum(len(r['outcomes']['rboxc-valgrind']['memory']) for r in original['results'])
assert original_processes>0 and len(processes)==23+original_processes
report = {'scope':'All 23 focused comparisons and eight reviewed unchanged GNU original selections pass. Every input and raw log is hash-checked and every candidate process summary is reparsed. GNU native findings remain baseline observations.',
          'binary':focused['binary'],'binary_sha256':focused['binary_sha256'],
          'runtime_helpers':focused['runtime_helpers'],'inputs':inputs,
          'focused_report':{'path':str(options.focused),'sha256':fingerprint(options.focused)},
          'original_report':{'path':str(options.original),'sha256':fingerprint(options.original)},
          'passed':len(processes),'total':len(processes),'focused_cases':23,'original_selections':8,'original_processes':original_processes,
          'driver_sha256':fingerprint(Path(__file__)),'results':processes}
target.write_text(json.dumps(report,indent=2)+'\n')
print('Audited 23 focused cases, 8 GNU original selections, and',len(processes),'clean candidate processes')
