#!/usr/bin/env python3
"""Combine disjoint Findutils observations while preserving their provenance."""
# SPDX-License-Identifier: GPL-3.0-or-later
import argparse
import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('reports', nargs='+', type=Path)
parser.add_argument('--output', type=Path, required=True)
parser.add_argument('--expected-selections', type=int, required=True)
options = parser.parse_args()
digest = lambda p: hashlib.sha256(p.read_bytes()).hexdigest()
sources = []
rows = {}
inputs = {}
metadata = None
for path in options.reports:
    path = path.resolve(strict=True)
    data = json.loads(path.read_text())
    current = {k: data[k] for k in ('binary', 'binary_sha256', 'gnu_binary_sha256', 'runtime_helpers')}
    if metadata is None:
        metadata = current
    assert metadata == current, 'different executable/helper profiles'
    assert digest(Path(data['binary'])) == data['binary_sha256']
    assert len(data['results']) == data['total'] > 0
    origin = {'path': str(path.relative_to(ROOT)), 'sha256': digest(path), 'scope': data['scope']}
    sources.append(origin)
    for name, expected in {**data['inputs'], **data['runtime_helpers']}.items():
        assert digest(Path(name)) == expected, 'recorded input changed: ' + name
        assert name not in inputs or inputs[name] == expected
        inputs[name] = expected
    for row in data['results']:
        assert row['selection'] not in rows, 'overlapping selections'
        assert row['assertions_pass'] and row['applet_assertions_and_memory_passed']
        for outcome in row['outcomes'].values():
            assert outcome['status'] == 0 and outcome['assertions_pass']
            assert digest(ROOT/outcome['driver_log']) == outcome['driver_log_sha256']
            for log in outcome['memory']:
                assert digest(ROOT/log['log']) == log['sha256']
        rows[row['selection']] = {**row, 'observation_source': origin}
assert len(rows) == options.expected_selections
results = [rows[key] for key in sorted(rows)]
report = {
    'scope': 'Disjoint original Findutils selections on identical candidate bytes. Each row retains its original report, execution profile, and logs. Strict all-process passes remain separate from applet assertions and memory; declared child instrumentation limits remain in the source reports.',
    **metadata, 'inputs': inputs, 'source_reports': sources,
    'driver_sha256': digest(Path(__file__)),
    'passed': sum(r['pass'] for r in results), 'total': len(results),
    'applet_assertions_and_memory_passed': sum(r['applet_assertions_and_memory_passed'] for r in results),
    'assertions_passed': sum(r['expected_assertions'] for r in results), 'results': results,
}
assert not options.output.exists(), 'preserve existing observations under a new report name'
options.output.write_text(json.dumps(report, indent=2)+'\n')
print('Consolidated', len(results), 'selections;', report['passed'], 'strict all-process passes')
