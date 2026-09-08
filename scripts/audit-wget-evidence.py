#!/usr/bin/env python3
"""Verify current Wget comparisons and reparse every candidate process log."""
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
assert focused['complete'] and focused['passed'] == focused['total'] == focused['planned_total'] == 14
manifest=json.loads((ROOT/'inventory/wget-tests.json').read_text())
reviewed_rows={r['target']:r for r in manifest['inputs'] if r['reviewed']}
reviewed=set(reviewed_rows)
source=Path(json.loads((ROOT/'inventory/sources.json').read_text())['wget']['source'])
expected_skips={r['target']:r['expected_feature_skip'] for r in manifest['inputs'] if r['reviewed'] and r.get('expected_feature_skip')}
assert original['complete'] and original['passed'] == original['total'] == original['planned_total'] == len(reviewed)
assert focused['driver_sha256'] == fingerprint(ROOT/'tests/entry-behavior.py')
assert original['inputs'][str(ROOT/'tests/wget-original.py')] == fingerprint(ROOT/'tests/wget-original.py')
assert {r['selection'] for r in original['results']} == reviewed
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
focused_rows=[r for r in focused['results'] if r['command']=='wget']
assert len(focused_rows)==14
for row in focused_rows:
    assert row['pass'] and row['equivalent'] and row['memory_clean']
    reference = row['outcomes']['gnu']
    for key, outcome in row['outcomes'].items():
        assert all(outcome[k] == reference[k] for k in ('status','stdout','stderr','tree'))
        if 'memory' in outcome:
            audit(outcome['log'],outcome['log_sha256'],outcome['memory'],key=='rboxc-valgrind',row['name'])
for row in original['results']:
    assert row['pass']
    fixture=reviewed_rows[row['selection']].get('fixture_profile')
    assert row.get('fixture_profile')==fixture
    expected_skip=expected_skips.get(row['selection'])
    assert row['expected_feature_skip']==expected_skip
    for key,outcome in row['outcomes'].items():
        assert outcome['expectation_matches'] and not outcome.get('timed_out',False)
        private=outcome.get('private_inputs',{})
        raw=outcome.get('fixture_raw',{})
        for path,expected in raw.items():assert fingerprint(ROOT/path)==expected
        if fixture=='private-tls-log':
            expected_files={str(p.relative_to(source/'tests')) for p in (source/'tests').glob('*.pm')}
            expected_files.update(str(p.relative_to(source/'tests')) for p in (source/'tests/certs').rglob('*') if p.is_file())
            assert set(private)==expected_files
            helper_paths=[ROOT/p for p in raw if Path(p).name=='SSLServer.pm']
            assert len(helper_paths)==1 and fingerprint(helper_paths[0])==private['SSLServer.pm']
            adapted=helper_paths[0].read_text()
            paths=re.findall(r'/tmp/rboxc-wget-original-[A-Za-z0-9_]+/server\.log',adapted)
            assert len(paths)==1
            pristine=(source/'tests/SSLServer.pm').read_text()
            assert pristine.count('/tmp/wgetserver.log')==1
            assert adapted==pristine.replace('/tmp/wgetserver.log',paths[0])
            for name,expected in private.items():
                if name!='SSLServer.pm':assert inputs[str(source/'tests'/name)]==expected
        else:assert not private and not raw
        output=(ROOT/outcome['driver_log']).read_bytes()
        if expected_skip:
            assert outcome['status']==77 and outcome['feature_skip_matches'] and not outcome['assertions_pass']
            assert ("Skipped test: Wget misses feature '"+expected_skip+"'").encode() in output
            assert re.search(rb'^\s+'+expected_skip.encode()+rb'=0$',output,re.M)
            assert output.count(b'Test successful.')==0
        else:
            assert outcome['status']==0 and outcome['assertions_pass'] and not outcome['feature_skip_matches']
            assert output.count(b'Test successful.')==1 and b'Test failed:' not in output
        assert len(outcome['assertion_counts'])==1
        total,passed,failed=outcome['assertion_counts'][0]
        assert total==passed==(0 if expected_skip else 1) and failed==0
        assert fingerprint(ROOT/outcome['driver_log'])==outcome['driver_log_sha256']
        for log in outcome['memory']:
            contents=(ROOT/log['log']).read_text()
            commands=re.findall(r'^==[0-9]+== Command: (.*)$',contents,re.M)
            assert len(commands)==1 and commands[0].split()[0]=='wget', 'unclassified child process'
            audit(log['log'],log['sha256'],log,key=='rboxc-valgrind',row['source'])
original_processes=sum(len(r['outcomes']['rboxc-valgrind']['memory']) for r in original['results'])
assert original_processes>0 and len(processes)==14+original_processes
assert original['ordinary_passed']==len(reviewed)-len(expected_skips)
assert original['feature_skips_matched']==len(expected_skips)
report = {'scope':'All 14 focused comparisons pass and all currently reviewed unchanged GNU original selections match their expected outcomes. Optional-feature skips are reported separately and do not certify the skipped behavior. Every input and raw log is hash-checked and every candidate process summary is reparsed. GNU native findings remain baseline observations.',
          'binary':focused['binary'],'binary_sha256':focused['binary_sha256'],
          'runtime_helpers':focused['runtime_helpers'],'inputs':inputs,
          'focused_report':{'path':str(options.focused),'sha256':fingerprint(options.focused)},
          'original_report':{'path':str(options.original),'sha256':fingerprint(options.original)},
          'passed':len(processes),'total':len(processes),'focused_cases':14,'original_selections':len(reviewed),'original_processes':original_processes,
          'ordinary_passed':original['ordinary_passed'],'feature_skips_matched':original['feature_skips_matched'],
          'driver_sha256':fingerprint(Path(__file__)),'results':processes}
target.write_text(json.dumps(report,indent=2)+'\n')
print('Audited 14 focused cases,',len(reviewed),'GNU original selections, and',len(processes),'clean candidate processes')
