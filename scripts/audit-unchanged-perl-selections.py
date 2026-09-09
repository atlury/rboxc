#!/usr/bin/env python3
"""Verify that five existing Perl selections already include all ordinary cases."""
# SPDX-License-Identifier: GPL-3.0-or-later
import hashlib
import json
from pathlib import Path
import re

ROOT = Path(__file__).resolve().parents[1]
SOURCE = Path('/opt/src/coreutils-9.11')
target = ROOT/'evidence/gnu-unchanged-perl-selection-review.json'
assert not target.exists()
raw, results = {}, []


def pin(path, expected=None):
    path = Path(path)
    if not path.is_absolute(): path = ROOT/path
    value = hashlib.sha256(path.read_bytes()).hexdigest()
    assert expected is None or value == expected, str(path)
    raw[str(path.relative_to(ROOT)) if path.is_relative_to(ROOT) else str(path)] = value
    return value


manifest = ROOT/'evidence/raw/gnu-before-unexpand-expanded-inventory.json'; pin(manifest)
definitions = {r['script']: r for r in json.loads(manifest.read_text())}
ledger = ROOT/'evidence/gnu-numfmt-expanded-coverage.json'; pin(ledger)
coverage = {r['script']: r for r in json.loads(ledger.read_text())['results']}
pin('build/partial-names-only.pl')
pin(SOURCE/'tests/Coreutils.pm')
pin(SOURCE/'tests/CuSkip.pm')
pin('build/gnu-coreutils/src/getlimits')
profiles = {
    'tests/cksum/cksum-base64.pl': (26, 27, r'nul', 'Historical read-buffer reproduction'),
    'tests/cksum/md5sum.pl': (33, 34, r'bsd-segv', 'Historical crash reproduction'),
    'tests/cksum/sha1sum.pl': (17, 18, r'bsd-segv', 'Historical crash reproduction'),
    'tests/od/od.pl': (17, 20, r'invalid-w-[1-3]', 'Historical abort reproductions'),
    'tests/paste/paste.pl': (26, 28, r'delim-bs[12]', 'Historical delimiter memory-error reproductions'),
}
for script, (selected_count, original_count, held_pattern, reason) in profiles.items():
    definition = definitions[script]; pin(SOURCE/script, definition['sha256'])
    name = Path(script).stem
    out = ROOT/f'build/{name}-remaining-names.stdout'; pin(out)
    pin(out.with_suffix('.stderr'))
    names = out.read_text().splitlines()
    assert len(names) == len(set(names)) == original_count
    held = [n for n in names if re.fullmatch(held_pattern, n)]
    selected = [n for n in names if n not in held]
    assert set(selected) == set(definition['cases']) and len(selected) == selected_count
    row = coverage[script]
    assert row['state'] == 'partial' and row['valgrind_state'] == 'passed-selection'
    assert row['selected_case_count'] == selected_count
    for key in ('evidence', 'valgrind_evidence'): pin(row[key])
    results.append({'script': script, 'source_sha256': definition['sha256'],
        'names': names, 'selected': selected, 'held': held, 'held_reason': reason,
        'ordinary_cases_not_selected': [], 'existing_evidence': row['evidence'],
        'existing_valgrind_evidence': row['valgrind_evidence']})
target.write_text(json.dumps({'scope': 'Source/name review only: these five selections already include all 119 ordinary cases. Eight historical cases remain held. The collector exits inside run_tests without running any original test case. No runtime passes, exclusions or ledger states are added.',
    'results': results, 'raw': raw, 'ordinary_selected_cases': 119,
    'held_cases': 8, 'accounting_pass': True}, indent=2)+'\n')
print('PASS: five existing selections cover all 119 ordinary cases; eight held cases unchanged')
