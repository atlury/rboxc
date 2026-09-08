#!/usr/bin/env python3
"""Verify original iconv recipes, their raw logs, and a reproducible candidate."""
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
for name in ('original', 'focused', 'smoke', 'dispatch', 'rebuild', 'output'):
    parser.add_argument('--'+name, type=Path, required=True)
options = parser.parse_args()
assert not options.output.exists()
reports = {name: json.loads(getattr(options, name).read_text())
           for name in ('original', 'focused', 'smoke', 'dispatch')}
original, focused = reports['original'], reports['focused']
binary = Path(original['binary'])
binary_hash = fingerprint(binary)
assert fingerprint(options.rebuild) == binary_hash
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
    assert report['binary_sha256'] == binary_hash
    assert report['passed'] == report['total'] and report.get('complete', True)
    inputs.update(report.get('inputs', {}))
    inputs.update(report['runtime_helpers'])
    walk(report)
assert reports['smoke']['total'] == 428 and reports['dispatch']['total'] == 11
assert focused['total'] == focused['planned_total'] == 50
assert focused['driver_sha256'] == fingerprint(ROOT/'tests/glibc-behavior.py')
assert original['total'] == original['planned_total'] == 3
assert [(r['variant'], r['arguments']) for r in original['results']] == [
    ('default', ['', '0']), ('tiny', ['--buffer-size=1', '0']), ('large', ['', '22'])]
processes = []

def audit_memory(recorded, path, candidate):
    text = (ROOT/path).read_text()
    pids = set(re.findall(r'^==([0-9]+)==', text, re.M))
    assert len(pids) == 1
    parsed = runner.parse_memory_log(text, pids.pop(), exec_only=True)
    assert all(recorded[k] == v for k, v in parsed.items())
    assert parsed['complete_exec_log']
    if candidate:
        assert parsed['errors'] == 0 and parsed['non_inherited_descriptors'] == 0
        assert not any(parsed['heap_bytes'].get(k, 0) for k in
                       ('definitely lost', 'indirectly lost', 'possibly lost'))
        processes.append(path)

for row in original['results']:
    assert row['pass']
    for key, outcome in row['outcomes'].items():
        assert outcome['pass'] and outcome['status'] == 0 and outcome['calls'] == 59
        output = next(ROOT/p for p in outcome['raw'] if p.endswith('/stdout'))
        assert len(re.findall(rb'^.*tst-iconv_prog-buffer\.sh:[0-9]+: iconv ', output.read_bytes(), re.M)) == 59
        assert len(outcome['memory']) == (59 if key.endswith('-valgrind') else 0)
        for log in outcome['memory']:
            audit_memory(log, log['log'], key == 'rboxc-valgrind')
for row in focused['results']:
    assert row['pass'] and row['equivalent'] and row['memory_clean']
    for key, outcome in row['outcomes'].items():
        assert all(outcome[k] == row['outcomes']['gnu'][k] for k in ('status', 'stdout', 'stderr', 'tree'))
        if 'memory' in outcome:
            audit_memory(outcome['memory'], outcome['log'], key == 'rboxc-valgrind')
for oracle in focused['oracles'].values():
    inputs[oracle['path']] = oracle['sha256']
for path, expected in inputs.items():
    assert fingerprint(Path(path)) == expected, path
assert len(processes) == 227
sources = ['scripts/audit-iconv-original.py', 'scripts/glibc_entry_adapters.py',
           'scripts/translate-entry-provider.py', 'src/generated/applet_iconv.rs',
           'tests/iconv-original.py', 'tests/glibc-behavior.py',
           'inventory/glibc-utility-tests.json', 'evidence/iconv-translation.json']
proof = {'scope': 'All three original GNU iconv buffering recipes pass, including their original large-file arguments. The 50 focused glibc utility checks, smoke checks and dispatcher checks also pass. Candidate memory logs are reparsed and raw/input hashes verified. This does not complete broader GNU acceptance.',
    'candidate': str(binary), 'candidate_sha256': binary_hash, 'candidate_bytes': binary.stat().st_size,
    'candidate_commands': 187, 'rebuild': str(options.rebuild.resolve()),
    'rebuild_sha256': fingerprint(options.rebuild), 'inputs': inputs,
    'reports': {str(getattr(options, n)): fingerprint(getattr(options, n)) for n in reports},
    'source_files': {p: fingerprint(ROOT/p) for p in sources},
    'verified_raw_files': len(raw), 'raw_files': raw, 'clean_candidate_processes': len(processes),
    'installed_release_changed': False, 'full_acceptance_complete': False}
assert fingerprint(ROOT/'target/release/rboxc') == '8979835878e7dbe575440b5d0c7c86a01c75782ff5ddc735707ccada50e88e86'
options.output.write_text(json.dumps(proof, indent=2)+'\n')
print('Verified three original recipes, 50 focused cases and 227 clean candidate process logs')
