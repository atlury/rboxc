#!/usr/bin/env python3
"""Recheck reviewed Bash outputs, process logs, and candidate provenance."""
# SPDX-License-Identifier: GPL-3.0-or-later
import argparse
import importlib.util
import json
from pathlib import Path
import re
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT/'tests'))
sys.path.insert(0, str(ROOT/'tests/gnu'))
from comparison_profile import fingerprint
spec = importlib.util.spec_from_file_location('reviewed', ROOT/'tests/gnu/reviewed-original.py')
runner = importlib.util.module_from_spec(spec)
spec.loader.exec_module(runner)
parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('--prefix', required=True)
parser.add_argument('--rebuild', type=Path, required=True)
parser.add_argument('--output', type=Path, required=True)
options = parser.parse_args()
assert re.fullmatch(r'[a-z0-9-]+', options.prefix) and not options.output.exists()
paths = {n: ROOT/'evidence'/(options.prefix+'-'+n+'.json')
         for n in ('original', 'behavior', 'adapters', 'smoke', 'dispatch')}
reports = {n: json.loads(p.read_text()) for n, p in paths.items()}
binary = Path(reports['original']['binary'])
expected_binary = fingerprint(binary)
assert fingerprint(options.rebuild) == expected_binary
assert len(subprocess.check_output([str(binary), '--list'], text=True).splitlines()) == 187
raw, inputs = {}, {}

def verify_raw(path, expected):
    assert fingerprint(ROOT/path) == expected, path
    raw[path] = expected

def walk(value):
    if isinstance(value, dict):
        for path, expected in value.get('raw', {}).items():
            verify_raw(path, expected)
        if isinstance(value.get('log'), str):
            path = value['log']
            verify_raw(path, value.get('sha256', value.get('log_sha256', fingerprint(ROOT/path))))
        for child in value.values():
            walk(child)
    elif isinstance(value, list):
        for child in value:
            walk(child)

for name, report in reports.items():
    assert report['binary_sha256'] == expected_binary
    assert report['passed'] == report['total'] and report.get('complete', True)
    inputs.update(report.get('inputs', {}))
    inputs.update(report['runtime_helpers'])
    walk(report)
assert [reports[n]['total'] for n in ('behavior', 'adapters', 'smoke', 'dispatch')] == [44, 69, 428, 11]
manifest = json.loads((ROOT/'inventory/bash-tests.json').read_text())
reviewed = {r['target']: r for r in manifest['inputs'] if r['reviewed']}
original = reports['original']
assert original['total'] == original['planned_total'] == len(reviewed)
assert {r['selection'] for r in original['results']} == reviewed.keys()
source = Path(json.loads((ROOT/'inventory/sources.json').read_text())['bash']['source'])/'tests'
candidate_processes = []
for result in original['results']:
    assert result['pass']
    row = reviewed[result['selection']]
    for implementation, outcome in result['outcomes'].items():
        assert outcome['status'] == result['outcomes']['gnu']['status']
        output = next(ROOT/p for p in outcome['raw'] if p.endswith('/stdout'))
        actual_path = next(ROOT/p for p in outcome['raw'] if p.endswith('/actual'))
        actual = output.read_bytes()
        if row['output_mode'] == 'drop-expect':
            actual = b''.join(line for line in actual.splitlines(keepends=True) if not line.startswith(b'expect'))
        assert actual == actual_path.read_bytes() == (source/row['expected']).read_bytes()
        assert outcome['expected_output_matches']
        assert bool(outcome['memory']) == implementation.endswith('-valgrind')
        for log in outcome['memory']:
            text = (ROOT/log['log']).read_text()
            pids = set(re.findall(r'^==([0-9]+)==', text, re.M))
            assert len(pids) == 1
            parsed = runner.parse_memory_log(text, pids.pop())
            assert all(log[k] == v for k, v in parsed.items())
            errors = list(re.finditer(r'ERROR SUMMARY:', text))
            fds = list(re.finditer(r'FILE DESCRIPTORS:', text))
            images = list(re.finditer(r'^==[0-9]+== Command:', text, re.M))
            complete = bool(errors) and len(errors) == len(fds) and (not images or
                errors[-1].start() > images[-1].start() and fds[-1].start() > images[-1].start())
            assert log['complete'] == complete
            if implementation == 'rboxc-valgrind':
                assert complete and parsed['errors'] == 0 and parsed['non_inherited_descriptors'] == 0
                assert not any(parsed['heap_bytes'].get(k, 0) for k in
                               ('definitely lost', 'indirectly lost', 'possibly lost'))
                assert log['clean'] and outcome['memory_clean']
                candidate_processes.append(log['log'])
for path, expected in inputs.items():
    assert fingerprint(Path(path)) == expected, path
sources = ['scripts/audit-bash-original.py', 'scripts/bash_cleanup.py',
           'scripts/entry_provider_helpers.py', 'src/shell_arguments.c',
           'src/generated/applet_bash.rs', 'tests/bash-original.py', 'tests/bash-behavior.py',
           'tests/bash-nss-files.c', 'inventory/bash-tests.json',
           'evidence/bash-native-cleanup.json', 'evidence/bash-link.json']
proof = {'scope': 'All currently reviewed complete Bash originals match GNU expected output and have clean candidate Valgrind process trees. Tilde uses the glibc files passwd provider in both implementations; host NSS findings remain in earlier reports. Focused Bash, shell-adapter, updatedb, smoke and dispatcher checks also pass. Other originals and broader GNU acceptance remain open.',
    'candidate': str(binary), 'candidate_sha256': expected_binary, 'candidate_bytes': binary.stat().st_size,
    'candidate_commands': 187, 'rebuild': str(options.rebuild.resolve()),
    'rebuild_sha256': fingerprint(options.rebuild), 'inputs': inputs,
    'reports': {str(p.relative_to(ROOT)): {'sha256': fingerprint(p), 'passed': reports[n]['passed'],
                'total': reports[n]['total']} for n, p in paths.items()},
    'source_files': {p: fingerprint(ROOT/p) for p in sources},
    'reviewed_originals': len(reviewed), 'clean_original_candidate_processes': len(candidate_processes),
    'verified_raw_files': len(raw), 'raw_files': raw, 'installed_release_changed': False,
    'full_acceptance_complete': False}
assert fingerprint(ROOT/'target/release/rboxc') == '8979835878e7dbe575440b5d0c7c86a01c75782ff5ddc735707ccada50e88e86'
options.output.write_text(json.dumps(proof, indent=2)+'\n')
print('Verified', len(reviewed), 'originals and', len(candidate_processes), 'clean candidate process logs')
