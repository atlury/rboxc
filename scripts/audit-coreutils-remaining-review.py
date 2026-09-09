#!/usr/bin/env python3
"""Check completeness and source identity of the remaining read-only review."""
# SPDX-License-Identifier: GPL-3.0-or-later
from collections import Counter
import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SOURCE = Path('/opt/src/coreutils-9.11')
sha = lambda p: hashlib.sha256(Path(p).read_bytes()).hexdigest()
review_path = ROOT/'evidence/gnu-remaining-coreutils-review.json'
coverage_path = ROOT/'evidence/gnu-sort-selection-coverage.json'
target = ROOT/'evidence/gnu-remaining-coreutils-review-validation.json'
assert not target.exists()
review = json.loads(review_path.read_text())
coverage = json.loads(coverage_path.read_text())
base_path = ROOT/review['base_report']
assert sha(base_path) == review['base_report_sha256']
base = json.loads(base_path.read_text())
pending = {r['script'] for r in coverage['results'] if r['state'] == 'pending'}
assert pending == {r['script'] for r in base['results'] if r['state'] == 'pending'} - {'tests/sort/sort.pl'}
assert len(pending) == 31
assert len(review['results']) == len({r['script'] for r in review['results']}) == 31
assert {r['script'] for r in review['results']} == pending
pins = {}
for row in review['results']:
    source = SOURCE/row['script']
    assert sha(source) == row['sha256']
    assert len(source.read_bytes().splitlines()) == row['lines']
    assert row['review_state'] == 'source-reviewed' and row['execution_state'] == 'pending'
    assert row['reason']
    pins[str(source)] = row['sha256']
    old, = [r for r in base['results'] if r['script'] == row['script']]
    current, = [r for r in coverage['results'] if r['script'] == row['script']]
    assert old == current
assert review['counts'] == dict(sorted(Counter(r['category'] for r in review['results']).items()))
assert review['counts'] == {'historical-reproduction': 20, 'isolation-debugger-profile': 4,
                            'mixed-source-selection': 2, 'platform-resource-profile': 1,
                            'resource-profile': 4}
for path in (review_path, coverage_path, base_path, ROOT/review['separate_sort_review']):
    pins[str(path.relative_to(ROOT))] = sha(path)
target.write_text(json.dumps({
    'scope': 'All 31 remaining whole-script pending Coreutils entries have explicit source reviews and unchanged source hashes. No execution status or pass count is promoted by this review.',
    'reviewed_entries': 31, 'categories': review['counts'], 'inputs': pins,
    'accounting_pass': True, 'runtime_passes_added': 0, 'full_coreutils_complete': False,
}, indent=2)+'\n')
print('PASS: 31 source reviews; source identity verified; no runtime passes added')
