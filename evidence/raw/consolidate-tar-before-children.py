#!/usr/bin/env python3
"""Consolidate original Tar selections and reparse every candidate process log."""
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
runner = importlib.util.module_from_spec(spec)
spec.loader.exec_module(runner)
parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('reports', nargs='+', type=Path)
parser.add_argument('--expected-selections', type=int, required=True)
parser.add_argument('--report-name', required=True)
parser.add_argument('--selection', action='append', default=[],
                    help='Audit only these named passing groups; retain source report and omitted outcomes')
parser.add_argument('--archived-driver', type=Path, action='append', default=[],
                    help='Preserved original driver bytes for reports predating allowlist expansion')
options = parser.parse_args()
archived_drivers = {}
for path in options.archived_driver:
    path = path.resolve(strict=True)
    archived_drivers[fingerprint(path)] = path
assert re.fullmatch(r'[a-z0-9-]+', options.report_name)
target = ROOT/'evidence'/(options.report_name+'.json')
assert not target.exists(), 'preserve existing observations'
metadata = None
inputs = {}
results = {}
provenance = []
audited = []
for path in options.reports:
    path = path.resolve(strict=True)
    data = json.loads(path.read_text())
    current = {k: data[k] for k in ('binary', 'binary_sha256', 'gnu_binary_sha256', 'runtime_helpers')}
    if metadata is None:
        metadata = current
    assert current == metadata, 'different executable/helper profiles'
    assert fingerprint(Path(data['binary'])) == data['binary_sha256']
    assert data['total'] == len(data['results']) > 0
    assert data['passed'] == sum(bool(row['pass']) for row in data['results'])
    if not options.selection:
        assert data['passed'] == data['total']
    if 'complete' in data:
        assert data['complete'] and data['planned_total'] == data['total'], 'original batch is incomplete'
        assert data['selected'] == [row['selection'] for row in data['results']]
    origin = {'path': str(path.relative_to(ROOT)), 'sha256': fingerprint(path), 'scope': data['scope']}
    provenance.append(origin)
    for name, expected in {**data['inputs'], **data['runtime_helpers']}.items():
        actual = Path(name)
        if name == str(ROOT/'tests/tar-original.py') and fingerprint(actual) != expected:
            assert expected in archived_drivers, 'preserved original driver required'
            actual = archived_drivers[expected]
            origin.setdefault('archived_inputs', []).append({
                'original_path': name, 'preserved_path': str(actual.relative_to(ROOT)),
                'sha256': expected,
            })
        assert fingerprint(actual) == expected, 'recorded input changed: '+name
        name = str(actual)
        assert name not in inputs or inputs[name] == expected
        inputs[name] = expected
    for row in data['results']:
        if options.selection and row['selection'] not in options.selection:
            origin.setdefault('omitted_selections', []).append({'selection': row['selection'], 'pass': row['pass'], 'assertions_pass': row['assertions_pass']})
            continue
        assert row['selection'] not in results and row['assertions_pass'] and row['pass']
        for outcome in row['outcomes'].values():
            assert outcome['assertions_pass'] and outcome['status'] == 0
            assert outcome['assertions'] == [[str(row['autotest_number']), 'ok']]
            nss = outcome.get('nss')
            if nss:
                assert nss['profile'] == 'private-mount-local-files'
                host = Path('/etc/nsswitch.conf')
                assert fingerprint(host) == data['inputs'][str(host)] == nss['host_sha256']
                private = ROOT/nss['private_path']
                assert fingerprint(private) == nss['private_sha256']
                expected = re.sub(r'^(passwd|group|shadow|gshadow|initgroups):.*$',
                                  r'\1: files', host.read_text(), flags=re.M)
                assert private.read_text() == expected
            copies = outcome.get('copied_executables', [])
            if copies:
                assert outcome['execution_uid'] == outcome['execution_gid'] == 65534
                known = {**data['inputs'], data['binary']: data['binary_sha256']}
                assert len({copy['private_path'] for copy in copies}) == len(copies)
                for copy in copies:
                    assert known[copy['source']] == copy['sha256']
                    assert fingerprint(Path(copy['source'])) == copy['sha256']
            assert fingerprint(ROOT/outcome['driver_log']) == outcome['driver_log_sha256']
            for log in outcome['memory']:
                assert fingerprint(ROOT/log['log']) == log['sha256']
        logs = row['outcomes']['rboxc-valgrind']['memory']
        assert logs
        for log in logs:
            path = ROOT/log['log']
            contents = path.read_text()
            parsed = runner.parse_memory_log(contents, path.stem, exec_only=True)
            assert all(log[k] == value for k, value in parsed.items())
            assert parsed['complete_exec_log'] and parsed['errors'] == 0 and parsed['non_inherited_descriptors'] == 0
            assert not any(parsed['heap_bytes'].get(k, 0) for k in ('definitely lost', 'indirectly lost', 'possibly lost'))
            commands = re.findall(r'^==\d+== Command: (.*)$', contents, re.M)
            assert commands and all(command == 'tar' or command.startswith('tar ') for command in commands), 'separate external child assessment required'
            audited.append({'selection': row['selection'], **log, 'command': commands[-1], 'pass': True})
        results[row['selection']] = {**row, 'observation_source': origin}
assert len(results) == options.expected_selections
if options.selection:
    assert len(set(options.selection)) == len(options.selection)
    assert set(results) == set(options.selection)
report = {'scope': 'Disjoint unchanged original GNU Tar selections on identical candidate bytes. All selected assertions pass, and every recorded candidate process log is hash-verified and reparsed. Original native findings remain unchanged. These are reviewed selections, not full-provider certification.',
          **metadata, 'inputs': inputs, 'source_reports': provenance,
          'driver_sha256': fingerprint(Path(__file__)), 'passed': len(results), 'total': len(results),
          'candidate_processes': len(audited), 'candidate_processes_clean': len(audited),
          'process_audit': audited, 'results': [results[key] for key in sorted(results)]}
target.write_text(json.dumps(report, indent=2)+'\n')
print('Consolidated', len(results), 'original selections with', len(audited), 'clean Tar processes')
