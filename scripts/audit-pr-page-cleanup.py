#!/usr/bin/env python3
"""Audit pr's page-limit ownership correction and before/after evidence."""
# SPDX-License-Identifier: GPL-3.0-or-later
import hashlib
import importlib.util
import json
from pathlib import Path
import re
import sys

ROOT = Path(__file__).resolve().parents[1]
sys.path[:0] = [str(ROOT/'tests/gnu'), str(ROOT/'scripts')]
spec = importlib.util.spec_from_file_location('reviewed', ROOT/'evidence/raw/gnu-resource-tac-driver.py')
runner = importlib.util.module_from_spec(spec)
spec.loader.exec_module(runner)
target = ROOT/'evidence/pr-page-cleanup-validation.json'
assert not target.exists()
raw = {}


def pin(name, expected=None):
    path = Path(name)
    if not path.is_absolute(): path = ROOT/path
    value = hashlib.sha256(path.read_bytes()).hexdigest()
    assert expected is None or value == expected, str(path)
    raw[str(path.relative_to(ROOT)) if path.is_relative_to(ROOT) else str(path)] = value
    return value


before_source = ROOT/'evidence/raw/applet_pr-before-page-cleanup.rs'
after_source = ROOT/'src/generated/applet_pr.rs'
anchor = '    while print_page() {}\n'
addition = '''    let mut column = 0;
    while column < columns {
        close_file(column_vector.offset(column as isize));
        column += 1;
    }
'''
old = before_source.read_text()
assert old.count(anchor) == 1
assert after_source.read_text() == old.replace(anchor, anchor+addition)
for p in (before_source, after_source, 'scripts/cleanup.py', 'scripts/translate-coreutils.py',
          'scripts/postprocess.py', 'tests/pr-page-cleanup.py', 'tests/comparison_profile.py',
          'evidence/translation.json', 'build/pr-page-cleanup-build.log',
          'build/pr-page-cleanup-repro-build.log', 'build/pr-page-cleanup-translation.log'): pin(p)
candidate = ROOT/'target/pr-page-cleanup-candidate/release/rboxc'
candidate_hash = pin(candidate)
pin('target/pr-page-cleanup-repro/release/rboxc', candidate_hash)
before_hash = pin('target/expr-cleanup-candidate/release/rboxc')
baseline_path = ROOT/'evidence/raw/gnu-pr-expanded-valgrind.json'
baseline = json.loads(baseline_path.read_text()); pin(baseline_path)
row, = baseline['results']
assert row['state'] == 'open' and not row['pass']
assert row['rboxc']['binary_sha256'] == before_hash
assert row['rboxc']['status'] == row['gnu']['status'] == 0
assert row['rboxc']['case_count'] == row['gnu']['case_count'] == 717
findings = []
for implementation in ('gnu', 'rboxc'):
    outcome = row[implementation]; pin(outcome['log'])
    assert len(outcome['memory']) == 718
    for memory in outcome['memory']:
        path = ROOT/memory['log']; pin(path)
        parsed = runner.parse_memory_log(path.read_text(), path.stem)
        assert all(memory[k] == v for k, v in parsed.items())
        if implementation != 'rboxc': continue
        if parsed['errors'] or parsed['non_inherited_descriptors']:
            command, = re.findall(r'^==\d+== Command: (.*)$', path.read_text(), re.M)
            findings.append({'command': command, 'memory': parsed, 'log': memory['log']})
assert len(findings) == 6, 'expected six preserved page-range descriptor findings'
assert all(f['memory']['errors'] == f['memory']['non_inherited_descriptors'] == 1
           and not any(f['memory']['heap_bytes'].get(k, 0) for k in
                       ('definitely lost', 'indirectly lost', 'possibly lost'))
           for f in findings)
for suffix in ('original.json', 'original.progress.json', 'valgrind.progress.json'):
    pin('evidence/raw/gnu-pr-expanded-'+suffix)
checks = {}
for name, count in (('pr-page-cleanup-smoke', 428), ('pr-page-cleanup-dispatcher', 11),
                    ('pr-page-cleanup-focused', 10)):
    path = ROOT/f'evidence/{name}.json'; data = json.loads(path.read_text()); pin(path)
    assert data['passed'] == data['total'] == len(data['results']) == count
    assert data['binary_sha256'] == candidate_hash
    pin('build/gnu-coreutils/src/coreutils', data['gnu_binary_sha256'])
    for helper, value in data['runtime_helpers'].items(): pin(helper, value)
    for result in data['results']:
        assert result['pass']
        if 'log' not in result: continue
        log = ROOT/result['log']; pin(log)
        if name == 'pr-page-cleanup-focused':
            for filename, content in data['fixtures'].items():
                fixture = log.parent/filename; pin(fixture)
                assert fixture.read_text() == content
        memory = runner.parse_memory_log(log.read_text(), log.stem)
        assert memory['errors'] == memory['non_inherited_descriptors'] == 0
        assert not any(memory['heap_bytes'].get(k, 0) for k in
                       ('definitely lost', 'indirectly lost', 'possibly lost'))
    checks[name] = count
expanded_path = ROOT/'evidence/gnu-pr-expanded-validation.json'
expanded = json.loads(expanded_path.read_text()); pin(expanded_path)
assert expanded['accounting_pass']
assert expanded['results']['rboxc'] == {'images': 718, 'clean': 718}
target.write_text(json.dumps({'scope': 'Close each still-open pr column input after the page loop. The existing close_file status handling closes shared streams once and leaves stdin to its existing main cleanup.',
    'before_findings': findings, 'candidate_sha256': candidate_hash,
    'candidate_bytes': candidate.stat().st_size, 'rebuild_identical': True,
    'checks': checks, 'expanded_pr_clean_images': 718, 'raw': raw, 'pass': True}, indent=2)+'\n')
print('PASS: pr page ownership, identical rebuild, 717 original cases and ten focused checks')
