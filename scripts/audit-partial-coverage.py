#!/usr/bin/env python3
"""Consolidate normal/instrumented accounting for every partial Coreutils entry."""
# SPDX-License-Identifier: GPL-3.0-or-later
from collections import Counter
import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
target = ROOT/'evidence/gnu-partial-coverage-validation.json'
assert not target.exists()
raw = {}


def read(name):
    path = ROOT/name
    raw[name] = hashlib.sha256(path.read_bytes()).hexdigest()
    return json.loads(path.read_text())


ledger = read('evidence/gnu-date-expanded-coverage.json')
definitions = {r['script']: r for r in read('evidence/raw/gnu-partial-current-inventory.json')}
rows = [r for r in ledger['results'] if r['state'] == 'partial']
assert len(rows) == 29
assert Counter(r['valgrind_state'] for r in rows) == {'passed-selection': 27, 'open': 2}
assert {r['script'] for r in rows if r['valgrind_state'] == 'open'} == {
    'tests/env/env-S.pl', 'tests/numfmt/numfmt.pl'}
reviews = {}
for command in ('tac', 'expr', 'seq', 'tr', 'numfmt', 'pr', 'date', 'unexpand'):
    review = read(f'evidence/gnu-{command}-expanded-review.json')
    script = 'tests/pr/pr-tests.pl' if command == 'pr' else f'tests/{command}/{command}.pl'
    reviews[script] = review
for command in ('cut', 'ptx', 'sort'):
    reviews[f'tests/{command}/{command}.pl'] = read(f'evidence/gnu-{command}-case-review.json')
for review in read('evidence/gnu-unchanged-perl-selection-review.json')['results']:
    reviews[review['script']] = review
env = read('evidence/gnu-env-s-selection-validation.json')
assert env['accounting_pass'] and env['selected_cases'] == 197 and env['original_cases'] == 199
shells = {}
for group in ('shell', 'file', 'option'):
    for review in read(f'evidence/gnu-{group}-selection-review.json')['selections']:
        shells[review['script']] = review
assert len(shells) == 12
results, counts = [], Counter()
for row in rows:
    script = row['script']; definition = definitions[script]
    path = Path('/opt/src/coreutils-9.11')/script
    digest = hashlib.sha256(path.read_bytes()).hexdigest()
    assert digest == definition['sha256']; raw[str(path)] = digest
    item = {'script': script, 'source_sha256': digest, 'state': 'partial',
            'valgrind_state': row['valgrind_state']}
    for key in ('evidence', 'valgrind_evidence'):
        report = read(row[key]); outcome, = [r for r in report['results'] if r['script'] == script]
        assert outcome['gnu']['status'] == outcome['rboxc']['status'] == (1 if key == 'valgrind_evidence' and row['valgrind_state'] == 'open' else 0)
        item[key] = row[key]
    if script.endswith('.pl'):
        counts['perl_selections'] += 1
        if script == 'tests/env/env-S.pl':
            selected = definition['cases']; held = env['held_cases']; original_count = env['original_cases']
        else:
            review = reviews[script]
            assert review.get('script_sha256', review.get('source_sha256')) == digest
            assert set(review['selected']) == set(definition['cases'])
            assert set(review['selected']).isdisjoint(review['held'])
            assert set(review['selected']) | set(review['held']) == set(review['names'])
            selected, held, original_count = review['selected'], review['held'], len(review['names'])
        assert len(selected) == row['selected_case_count']
        assert len(selected) + len(held) == original_count
        counts['selected_perl_cases'] += len(selected); counts['held_perl_cases'] += len(held)
        item.update(selected_cases=len(selected), original_cases=original_count, held_cases=held)
    else:
        counts['shell_selections'] += 1
        review = shells[script]
        assert review['original_sha256'] == digest and not review['full_script']
        assert definition['shell_selection']['selected_sha256'] == review['selected_sha256']
        item['excluded_ranges'] = review['excluded_ranges']
    results.append(item)
assert counts['perl_selections'] == 17 and counts['shell_selections'] == 12
target.write_text(json.dumps({'scope': 'Every one of the 29 partial Coreutils entries has normal and Valgrind execution evidence. All selected normal assertions pass; 27 instrumented selections pass and two retain documented instrumentation differences. Source-reviewed omissions remain held and no partial entry becomes a whole-script pass.',
    'counts': dict(counts), 'results': results, 'raw': raw,
    'pending_partial_execution_profiles': 0, 'accounting_pass': True,
    'full_coreutils_complete': False}, indent=2)+'\n')
print('PASS: all 29 partial entries have normal and Valgrind evidence;', dict(counts))
