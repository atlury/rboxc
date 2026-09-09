#!/usr/bin/env python3
"""Audit the unchanged ordinary GNU sort matrix and all instrumented images."""
# SPDX-License-Identifier: GPL-3.0-or-later
from collections import Counter
import copy
import hashlib
import importlib.util
import json
from pathlib import Path
import re
import sys

ROOT = Path(__file__).resolve().parents[1]
SOURCE = Path('/opt/src/coreutils-9.11')
sys.path[:0] = [str(ROOT/'tests/gnu'), str(ROOT/'scripts')]
driver = ROOT/'evidence/raw/gnu-sort-selection-driver.py'
spec = importlib.util.spec_from_file_location('reviewed', driver)
runner = importlib.util.module_from_spec(spec)
spec.loader.exec_module(runner)
sha = lambda p: hashlib.sha256(Path(p).read_bytes()).hexdigest()
target = ROOT/'evidence/gnu-sort-selection-validation.json'
coverage_target = ROOT/'evidence/gnu-sort-selection-coverage.json'
assert not target.exists() and not coverage_target.exists()
raw, reports, counts = {}, {}, {}


def pin(path, expected=None):
    path = Path(path)
    if not path.is_absolute():
        path = ROOT/path
    if expected and sha(path) != expected and path == ROOT/'tests/gnu/reviewed-original.py':
        path = driver
    value = sha(path)
    assert expected is None or value == expected, str(path)
    raw[str(path.relative_to(ROOT)) if path.is_relative_to(ROOT) else str(path)] = value
    return value


script = 'tests/sort/sort.pl'
snapshot = ROOT/'evidence/raw/gnu-sort-selection-inventory.json'
pin(snapshot)
definition, = [r for r in json.loads(snapshot.read_text()) if r['script'] == script]
review_path = ROOT/'evidence/gnu-sort-case-review.json'
review = json.loads(review_path.read_text())
pin(review_path)
pin(SOURCE/script, definition['sha256'])
assert definition['sha256'] == review['script_sha256']
pin(SOURCE/'tests/Coreutils.pm', review['generator_sha256'])
pin(review['collector'], review['collector_sha256'])
pin('build/sort-case-inventory.stdout')
pin('build/sort-case-inventory.stderr')
assert (ROOT/'build/sort-case-inventory.stdout').read_text().splitlines() == review['names']
assert len(review['names']) == len(set(review['names'])) == 457
held = [n for n in review['names'] if re.fullmatch(
    r'(unique-free-mem-read|17|obs-inval|output-is-input(?:-[23])?)(?:\.[rp])?', n)]
assert held == review['held'] == definition['excluded_cases'] and len(held) == 11
assert review['selected'] == definition['cases'] == [n for n in review['names'] if n not in held]
assert definition['expected_case_count'] == 446 and definition['original_case_count'] == 457
assert not definition['full_suite'] and definition['valgrind_multicall']
locale_path = ROOT/'evidence/test-locales.json'
locales = json.loads(locale_path.read_text())
pin(locale_path)
for locale in locales['locales']:
    for suffix, value in locale['files'].items():
        pin(ROOT/locales['path']/locale['name']/suffix, value)
        pin(Path(locales['runtime_path'])/locale['name']/suffix, value)

for instrument in (False, True):
    report = ROOT/f'evidence/raw/gnu-sort-selected-{"valgrind" if instrument else "original"}.json'
    reports[str(report.relative_to(ROOT))] = pin(report)
    data = json.loads(report.read_text())
    assert data['passed'] == data['total'] == 1
    result, = data['results']
    assert all(result[k] == v for k, v in definition.items())
    checkpoint = report.with_suffix('.progress.json')
    pin(checkpoint)
    saved = json.loads(checkpoint.read_text())['runs'][script]
    context = saved['context']
    assert context['definition'] == definition and context['valgrind'] == instrument
    pin('/bin/sh', context['shell'])
    pin('build/gnu-coreutils/lib/config.h', context['config_header'])
    pin('build/gnu-coreutils/src/getlimits', context['getlimits'])
    pin(locale_path, context['locales'])
    for path, value in context['drivers'].items():
        pin(path, value)
    for path, value in context['gnu_harness'].items():
        pin(SOURCE/path, value)
    for path, value in (context.get('valgrind_runtime') or {}).items():
        pin(path, value)
    for implementation in ('gnu', 'rboxc'):
        outcome = result[implementation]
        assert saved['outcomes'][implementation]['result'] == outcome
        assert outcome['status'] == 0 and outcome['case_count'] == 446 and outcome['case_count_pass']
        assert outcome['binary_sha256'] == context['binaries'][implementation]
        binary = ('build/gnu-coreutils/src/coreutils' if implementation == 'gnu'
                  else 'target/iconv-charmap-cleanup-candidate/release/rboxc')
        pin(binary, outcome['binary_sha256'])
        pin(outcome['watchdog']['path'], outcome['watchdog']['sha256'])
        assert outcome['locale_evidence_sha256'] == sha(locale_path)
        for path, value in saved['outcomes'][implementation]['logs'].items():
            pin(path, value)
        log = (ROOT/outcome['log']).read_text()
        assert log.count('RBOXC_SELECTION 446 of 457\n') == 1
        assert not any(re.search('^' + re.escape(n) + r'\.\.\.$', log, re.M) for n in held)
        if not instrument:
            continue
        assert len(outcome['memory']) == 447
        clean, versions = 0, 0
        for memory in outcome['memory']:
            path = ROOT/memory['log']
            text = path.read_text()
            parsed = runner.parse_memory_log(text, path.stem)
            assert all(memory[k] == v for k, v in parsed.items())
            assert set(re.findall(r'^==([0-9]+)==', text, re.M)) == {path.stem}
            image, = re.findall(r'^==[0-9]+== Command: (.*)$', text, re.M)
            assert re.match(r'coreutils --coreutils-prog=sort(?: |$)', image)
            versions += image == 'coreutils --coreutils-prog=sort --version'
            errors = list(re.finditer('ERROR SUMMARY:', text))
            fds = list(re.finditer('FILE DESCRIPTORS:', text))
            assert len(errors) == len(fds) == 1
            assert min(errors[0].start(), fds[0].start()) > text.index('Command:')
            ok = (parsed['errors'] == parsed['non_inherited_descriptors'] == 0
                  and not any(parsed['heap_bytes'].get(k, 0) for k in
                              ('definitely lost', 'indirectly lost', 'possibly lost')))
            if implementation == 'rboxc':
                assert ok, memory['log']
            clean += ok
        assert versions == 1
        counts[implementation] = {'images': 447, 'clean': clean}

base_path = ROOT/'evidence/gnu-option-selection-coverage.json'
base = json.loads(base_path.read_text())
pin(base_path)
coverage = copy.deepcopy(base)
row, = [r for r in coverage['results'] if r['script'] == script]
assert row['state'] == row['valgrind_state'] == 'pending'
row.update(state='partial', execution_coverage='selected-cases', selected_case_count=446,
           original_case_count=457, held_cases=held, valgrind_state='passed-selection',
           evidence='evidence/raw/gnu-sort-selected-original.json',
           valgrind_evidence='evidence/raw/gnu-sort-selected-valgrind.json',
           audit=str(target.relative_to(ROOT)), binary_sha256=context['binaries']['rboxc'])
assert all(a == b for a, b in zip(base['results'], coverage['results']) if a['script'] != script)
coverage['counts'] = dict(sorted(Counter(r['state'] for r in coverage['results']).items()))
coverage['valgrind_counts'] = dict(sorted(Counter(r['valgrind_state'] for r in coverage['results']).items()))
coverage.update(base_report=str(base_path.relative_to(ROOT)), base_report_sha256=sha(base_path),
                scope='Pinned 733-script ledger extended by 446 unchanged ordinary sort cases. Eleven historical cases remain held; full-script and release acceptance remain open.')
coverage_target.write_text(json.dumps(coverage, indent=2)+'\n')
target.write_text(json.dumps({
    'scope': 'All 446 unchanged selected GNU sort cases pass normally and under Valgrind, including original file/stdin/pipe and locale variants. Every candidate image is clean; native findings remain recorded. Eleven historical cases remain held.',
    'selected_cases': 446, 'original_cases': 457, 'held_cases': held,
    'processes': counts, 'clean_candidate_processes': counts['rboxc']['clean'],
    'reports': reports, 'raw': raw, 'coverage': str(coverage_target.relative_to(ROOT)),
    'coverage_sha256': sha(coverage_target), 'accounting_pass': True,
    'selected_cases_pass': True, 'full_suite_pass': False,
}, indent=2)+'\n')
print('PASS: 446 selected sort cases; 447 clean candidate images; 11 held inputs')
