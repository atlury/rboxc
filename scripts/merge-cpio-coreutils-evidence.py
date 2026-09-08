#!/usr/bin/env python3
"""Replace only missing-companion stdbuf observations, preserving both runs."""
# SPDX-License-Identifier: GPL-3.0-or-later
import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
digest = lambda p: hashlib.sha256(p.read_bytes()).hexdigest()
base_path = ROOT/'evidence/cpio-combined-coreutils-behavior.json'
retry_path = ROOT/'evidence/cpio-staged-stdbuf-behavior.json'
base = json.loads(base_path.read_text()); retry = json.loads(retry_path.read_text())
assert base['passed'] == 314 and base['total'] == 318
assert retry['passed'] == retry['total'] == 4
assert base['binary_sha256'] == retry['binary_sha256'] == digest(Path(base['binary']))
assert base['gnu_binary_sha256'] == retry['gnu_binary_sha256']
assert all(row['name'] == 'stdbuf' for row in base['results'] if not row['pass'])
assert all(row['name'] == 'stdbuf' and row['pass'] for row in retry['results'])
companion = str(Path(base['binary']).parent/'libstdbuf.so')
assert companion not in base['runtime_helpers']
assert retry['runtime_helpers'] == {**base['runtime_helpers'], companion:digest(Path(companion))}
assert digest(Path(companion)) == digest(ROOT/'target/release/libstdbuf.so')
for path, expected in retry['runtime_helpers'].items():
    assert digest(Path(path)) == expected
key = lambda row: (row['name'],tuple(row['arguments']))
replacements = {key(row):row for row in retry['results']}
assert len(replacements) == 4
assert set(replacements) == {key(row) for row in base['results'] if row['name']=='stdbuf'}
results = []
logs = {}
for row in base['results']:
    replaced = row['name'] == 'stdbuf'
    selected = replacements[key(row)] if replaced else row
    assert selected['pass'] and selected['behavior_pass'] and selected['valgrind_pass']
    for implementation in ('gnu_valgrind','rboxc_valgrind'):
        log = selected[implementation]['log']
        assert log
        logs[log] = digest(ROOT/log)
    results.append({**selected,'source_report':str((retry_path if replaced else base_path).relative_to(ROOT))})
report = {**retry,
    'scope':'318 Coreutils comparisons on unchanged executable bytes: retain 314 successful cases; rerun all four stdbuf cases after staging its byte-identical installed libstdbuf.so companion. Only stdbuf loads this command-specific companion. The missing-companion report is retained. This is a combined assessment, not a second full-suite run.',
    'passed':318,'behavior_passed':318,'valgrind_passed':318,'total':318,'results':results,
    'source_reports':[{ 'path':str(p.relative_to(ROOT)),'sha256':digest(p)} for p in (base_path,retry_path)],
    'selected_log_sha256':logs,'driver_sha256':digest(Path(__file__)),
    'comparison_driver_sha256':digest(ROOT/'tests/coreutils-behavior.py')}
target = ROOT/'evidence/cpio-staged-coreutils-behavior.json'
assert not target.exists(), 'preserve previous assessment'
target.write_text(json.dumps(report,indent=2)+'\n')
print('Combined 314 unchanged passing cases with four corrected stdbuf cases: 318/318')
