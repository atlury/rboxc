#!/usr/bin/env python3
"""Verify the unchanged Gzip empty-suffix original and its process logs."""
# SPDX-License-Identifier: GPL-3.0-or-later
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
target = ROOT/'evidence/gzip-empty-suffix-audited.json'
assert not target.exists()
path = ROOT/'evidence/gzip-empty-suffix-original.json'
data = json.loads(path.read_text())
assert data['passed'] == data['total'] == 1
assert data['selected_scripts'] == ['null-suffix-clobber']
assert fingerprint(Path(data['binary'])) == data['binary_sha256']
assert fingerprint(ROOT/'tests/gzip-original.py') == data['driver_sha256']
for oracle in data['gnu_binaries'].values():
    assert fingerprint(Path(oracle['path'])) == oracle['sha256']
pin = json.loads((ROOT/'inventory/sources.json').read_text())['gzip']
row = data['results'][0]
assert row['script'] == 'tests/null-suffix-clobber' and row['pass']
assert fingerprint(Path(pin['source'])/row['script']) == row['source_sha256']
processes = []
for implementation, outcome in row['outcomes'].items():
    assert outcome['status'] == 0
    assert fingerprint(ROOT/outcome['log']) == outcome['log_sha256']
    for memory in outcome.get('memory', []):
        path = ROOT/memory['log']
        assert fingerprint(path) == memory['sha256']
        parsed = runner.parse_memory_log(path.read_text(), path.stem, exec_only=True)
        assert all(memory[k] == value for k, value in parsed.items())
        if implementation == 'rboxc-valgrind':
            assert parsed['complete_exec_log'] and parsed['errors'] == parsed['non_inherited_descriptors'] == 0
            assert not any(parsed['heap_bytes'].get(k, 0) for k in
                           ('definitely lost', 'indirectly lost', 'possibly lost'))
            commands = re.findall(r'^==\d+== Command: (.*)$', path.read_text(), re.M)
            assert commands and all(c == 'gzip' or c.startswith('gzip ') for c in commands)
        processes.append({'implementation': implementation, **memory})
candidate = [p for p in processes if p['implementation'] == 'rboxc-valgrind']
assert len(candidate) == 2
target.write_text(json.dumps({
    'scope': 'One unchanged full GNU Gzip original rejects an empty suffix and preserves its private compressed input. Both candidate invocations are clean; earlier originals retain their own candidate hashes. Full GNU acceptance remains open.',
    'binary': data['binary'], 'binary_sha256': data['binary_sha256'],
    'source_report': 'evidence/gzip-empty-suffix-original.json',
    'source_report_sha256': fingerprint(ROOT/'evidence/gzip-empty-suffix-original.json'),
    'original_passed': 1, 'clean_candidate_processes': 2,
    'processes': processes, 'driver_sha256': fingerprint(Path(__file__)),
}, indent=2)+'\n')
print('Gzip empty-suffix original passed; two candidate processes audited clean')
