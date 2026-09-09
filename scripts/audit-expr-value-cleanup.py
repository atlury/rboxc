#!/usr/bin/env python3
"""Verify the expr trailing-argument cleanup and its preserved comparisons."""
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
target = ROOT/'evidence/expr-value-cleanup-validation.json'
assert not target.exists()
raw = {}


def pin(name, expected=None):
    path = Path(name)
    if not path.is_absolute(): path = ROOT/path
    value = hashlib.sha256(path.read_bytes()).hexdigest()
    assert expected is None or value == expected, str(path)
    raw[str(path.relative_to(ROOT)) if path.is_relative_to(ROOT) else str(path)] = value
    return value


before_source = ROOT/'evidence/raw/applet_expr-before-unexpected-argument.rs'
after_source = ROOT/'src/generated/applet_expr.rs'
anchor = '    let mut v: *mut VALUE = eval(r#true != 0);\n    if !nomoreargs() {\n'
old = before_source.read_text()
assert old.count(anchor) == 1
assert after_source.read_text() == old.replace(anchor, anchor+'        freev(v);\n')
for p in (before_source, after_source, 'scripts/cleanup.py', 'scripts/translate-coreutils.py',
          'scripts/postprocess.py', 'tests/expr-value-cleanup.py', 'tests/comparison_profile.py',
          'evidence/translation.json', 'build/expr-cleanup-build.log',
          'build/expr-cleanup-repro-build.log', 'build/expr-cleanup-translation.log'): pin(p)
candidate = ROOT/'target/expr-cleanup-candidate/release/rboxc'
candidate_hash = pin(candidate)
pin('target/expr-cleanup-repro/release/rboxc', candidate_hash)
before_hash = pin('target/iconv-charmap-cleanup-candidate/release/rboxc')
baseline_path = ROOT/'evidence/raw/gnu-expr-expanded-valgrind.json'
baseline = json.loads(baseline_path.read_text()); pin(baseline_path)
row, = baseline['results']
assert row['state'] == 'open' and not row['pass']
assert row['rboxc']['binary_sha256'] == before_hash
assert row['rboxc']['status'] == row['gnu']['status'] == 0
assert row['rboxc']['case_count'] == row['gnu']['case_count'] == 205
findings = []
for implementation in ('gnu', 'rboxc'):
    outcome = row[implementation]; pin(outcome['log'])
    assert len(outcome['memory']) == 206
    for memory in outcome['memory']:
        path = ROOT/memory['log']; pin(path)
        parsed = runner.parse_memory_log(path.read_text(), path.stem)
        assert all(memory[k] == v for k, v in parsed.items())
        if implementation != 'rboxc': continue
        assert parsed['non_inherited_descriptors'] == 0
        if parsed['errors']:
            command, = re.findall(r'^==\d+== Command: (.*)$', path.read_text(), re.M)
            assert parsed['errors'] == 1
            assert parsed['heap_bytes'] == {'definitely lost': 24, 'indirectly lost': 2,
                                            'possibly lost': 0, 'still reachable': 0}
            findings.append(command)
assert sorted(findings) == ['expr 2 a', 'expr 9 9']
for suffix in ('original.json', 'original.progress.json', 'valgrind.progress.json'):
    pin('evidence/raw/gnu-expr-expanded-'+suffix)
checks = {}
for name, count in (('expr-cleanup-smoke', 428), ('expr-cleanup-dispatcher', 11),
                    ('expr-value-cleanup-fixed', 8)):
    path = ROOT/f'evidence/{name}.json'; data = json.loads(path.read_text()); pin(path)
    assert data['passed'] == data['total'] == len(data['results']) == count
    assert data['binary_sha256'] == candidate_hash
    pin('build/gnu-coreutils/src/coreutils', data['gnu_binary_sha256'])
    for helper, value in data['runtime_helpers'].items(): pin(helper, value)
    for result in data['results']:
        assert result['pass']
        if 'log' not in result: continue
        log = ROOT/result['log']; pin(log)
        memory = runner.parse_memory_log(log.read_text(), log.stem)
        assert memory['errors'] == memory['non_inherited_descriptors'] == 0
        assert not any(memory['heap_bytes'].get(k, 0) for k in
                       ('definitely lost', 'indirectly lost', 'possibly lost'))
    checks[name] = count
expanded_path = ROOT/'evidence/gnu-resource-partial-expansion-validation.json'
expanded = json.loads(expanded_path.read_text()); pin(expanded_path)
assert expanded['accounting_pass']
assert expanded['results']['tests/expr/expr.pl']['rboxc'] == {'images': 206, 'clean': 206}
target.write_text(json.dumps({'scope': 'Release the completed expr value before a fatal trailing-argument diagnostic; no change to evaluation, diagnostic text, or exit status.',
    'before_findings': findings, 'candidate_sha256': candidate_hash,
    'candidate_bytes': candidate.stat().st_size, 'rebuild_identical': True,
    'checks': checks, 'expanded_expr_clean_images': 206, 'raw': raw, 'pass': True}, indent=2)+'\n')
print('PASS: expr value cleanup, identical rebuild, 205 original cases and eight focused checks')
