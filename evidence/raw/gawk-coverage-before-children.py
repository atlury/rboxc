#!/usr/bin/env python3
"""Audit distinct current-candidate Gawk coverage across preserved checkpoints."""
# SPDX-License-Identifier: GPL-3.0-or-later
import argparse
from collections import Counter
import importlib.util
import json
from pathlib import Path
import re
import sys

ROOT = Path(__file__).resolve().parents[1]
sys.path[:0] = [str(ROOT/'tests'), str(ROOT/'tests/gnu')]
from comparison_profile import fingerprint
spec = importlib.util.spec_from_file_location('reviewed', ROOT/'tests/gnu/reviewed-original.py')
runner = importlib.util.module_from_spec(spec)
spec.loader.exec_module(runner)
parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('--report-name', default='gawk-current-coverage')
options = parser.parse_args()
assert re.fullmatch(r'[a-z0-9-]+', options.report_name)
target = ROOT/'evidence'/(options.report_name+'.json')
assert not target.exists(), 'preserve previous coverage audits'
manifest_path = ROOT/'inventory/gawk-tests.json'
manifest = json.loads(manifest_path.read_text())
reviewed = {r['target']: r for r in manifest['inputs'] if r['reviewed']}
source = Path(json.loads((ROOT/'inventory/sources.json').read_text())['gawk']['source'])
assert fingerprint(source/'test/Makefile.am') == manifest['registration_sha256']
for row in manifest['inputs']:
    assert fingerprint(source/row['path']) == row['sha256']
archives = [ROOT/'tests/gawk-original.py', ROOT/'evidence/raw/gawk-array-driver.py',
            ROOT/'evidence/raw/gawk-language-time-driver.py']
driver_versions = {fingerprint(p): str(p.relative_to(ROOT)) for p in archives}
focused_path = ROOT/'evidence/gawk-source-behavior.json'
focused = json.loads(focused_path.read_text())
binary_hash = fingerprint(Path(focused['binary']))
assert focused['binary_sha256'] == binary_hash
assert focused['complete'] and focused['passed'] == focused['total'] == 85
assert focused['driver_sha256'] == fingerprint(ROOT/'tests/gawk-behavior.py')
candidate_logs = {}
native_logs = {}
report_refs = {}
seen = set()

def check_inputs(data):
    for filename, expected in {**data.get('inputs', {}), **data['runtime_helpers']}.items():
        if filename == str(ROOT/'tests/gawk-original.py'):
            assert expected == data['driver_sha256'] and expected in driver_versions
        else:
            assert fingerprint(Path(filename)) == expected, filename

def check_memory(recorded, log, expected, candidate, case, focused_case=False):
    p = ROOT/log
    assert fingerprint(p) == expected
    text = p.read_text()
    commands = re.findall(r'^==[0-9]+== Command: (.*)$', text, re.M)
    assert len(commands) == 1
    argv = commands[0].split()
    if focused_case:
        assert re.fullmatch(r'/tmp/rboxc-gawk-[^/]+/exec/(awk|gawk|nawk|rboxc)', argv[0])
        if Path(argv[0]).name == 'rboxc':
            assert candidate and argv[1] in ('awk', 'gawk', 'nawk')
    else:
        assert argv[0] == 'gawk'
    pids = set(re.findall(r'^==([0-9]+)==', text, re.M))
    assert len(pids) == 1
    parsed = runner.parse_memory_log(text, pids.pop(), exec_only=True)
    assert parsed['complete_exec_log'] and parsed['exec_images'] == 1
    assert all(recorded[k] == v for k, v in parsed.items())
    if candidate:
        assert parsed['errors'] == parsed['non_inherited_descriptors'] == 0
        assert not any(parsed['heap_bytes'].get(k, 0)
                       for k in ('definitely lost', 'indirectly lost', 'possibly lost'))
    collection = candidate_logs if candidate else native_logs
    assert log not in collection, 'do not count shared process logs twice'
    collection[log] = {'case': case, 'sha256': expected, **parsed}

check_inputs(focused)
for item in focused['oracles'].values():
    assert fingerprint(Path(item['path'])) == item['sha256']
for row in focused['results']:
    assert row['pass'] and row['equivalent'] and row['memory_clean']
    reference = row['outcomes']['gnu']
    for key, outcome in row['outcomes'].items():
        assert all(outcome[k] == reference[k] for k in ('status', 'stdout', 'stderr', 'tree'))
        if 'memory' in outcome:
            check_memory(outcome['memory'], outcome['log'], outcome['log_sha256'],
                         key == 'rboxc-valgrind', row['name'], focused_case=True)
for filename in sorted({r['evidence'] for r in reviewed.values()}):
    path = ROOT/filename
    data = json.loads(path.read_text())
    assert data['complete'] and data['total'] == data['planned_total'] == len(data['results'])
    assert data['binary_sha256'] == binary_hash
    assert fingerprint(ROOT/'build/gnu-gawk/gawk') == data['gnu_binary_sha256']
    assert data['driver_sha256'] in driver_versions
    check_inputs(data)
    names = {r['selection'] for r in data['results']}
    assert names == {n for n, r in reviewed.items() if r['evidence'] == filename}
    assert not names & seen
    seen.update(names)
    for result in data['results']:
        row = reviewed[result['selection']]
        assert result['source'] == row['path'] and result['source_sha256'] == row['sha256']
        for name, expected in row['fixtures'].items():
            assert fingerprint(source/'test'/name) == expected
        assert result.get('working_files', {}) == row.get('working_files', {})
        assert result.get('extension_profile') == row.get('extension_profile')
        baseline = row.get('expected_baseline_output')
        assert result['pass'] == (baseline is None)
        for key, outcome in result['outcomes'].items():
            assert outcome['status'] == 0 and outcome['assertions_pass'] == result['pass']
            log = ROOT/outcome['driver_log']
            assert fingerprint(log) == outcome['driver_log_sha256']
            if baseline is not None:
                actual = ROOT/outcome['actual_output']
                assert fingerprint(actual) == outcome['actual_output_sha256']
                assert actual.read_bytes() == baseline.encode()
                assert fingerprint(log) == row['expected_baseline_driver_sha256']
            else:
                assert outcome['actual_output'] is None and outcome['actual_output_sha256'] is None
                assert result['selection'] in log.read_text().splitlines() and b'Error ' not in log.read_bytes()
            if key.endswith('-valgrind'):
                assert outcome['memory']
                if key == 'rboxc-valgrind':
                    assert outcome['memory_clean']
                for memory in outcome['memory']:
                    check_memory(memory, memory['log'], memory['sha256'],
                                 key == 'rboxc-valgrind', result['selection'])
    report_refs[filename] = {'sha256': fingerprint(path), 'passed': data['passed'],
                             'total': data['total'], 'driver_archive': driver_versions[data['driver_sha256']]}
assert seen == set(reviewed)
assert len(candidate_logs) == len(native_logs)
counts = Counter(r['state'] for r in manifest['inputs'])
result = {'scope': 'Distinct reviewed Gawk recipes on one immutable candidate. Every current inventory '
          'input, report dependency and raw Valgrind log is verified. Historical driver versions are '
          'matched to preserved archives. The 85 focused process logs are counted once. Original failures '
          'shared with GNU, native findings, exclusions and pending inputs remain separate; full acceptance is open.',
          'binary': focused['binary'], 'binary_sha256': binary_hash,
          'manifest_sha256': fingerprint(manifest_path), 'input_states': dict(counts),
          'original_selections': len(reviewed),
          'original_passed': sum(r['passed'] for r in report_refs.values()),
          'baseline_failures': sum(bool(r.get('expected_baseline_output')) for r in reviewed.values()),
          'focused_cases': 85, 'clean_candidate_processes': len(candidate_logs),
          'clean_original_processes': len(candidate_logs)-85,
          'original_reports': report_refs,
          'focused_report': {'path': str(focused_path.relative_to(ROOT)), 'sha256': fingerprint(focused_path)},
          'driver_archives': driver_versions, 'driver_sha256': fingerprint(Path(__file__)),
          'memory_parser_sha256': fingerprint(ROOT/'tests/gnu/reviewed-original.py'),
          'candidate_logs': candidate_logs, 'native_logs': native_logs}
target.write_text(json.dumps(result, indent=2)+'\n')
print('Audited', result['original_passed'], 'distinct passing originals,', result['baseline_failures'],
      'GNU baseline failures and', len(candidate_logs), 'clean candidate process logs')
