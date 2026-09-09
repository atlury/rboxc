#!/usr/bin/env python3
"""Audit source-reviewed ordinary Perl expansions against immutable runs."""
# SPDX-License-Identifier: GPL-3.0-or-later
import argparse
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
PROFILES = {
    'pr': ('tests/pr/pr-tests.pl', 717, 739, 718,
           r'(narrow-1|neg-inp-pos[12]|smash-heap8?|padding2|page-length1|asan1)(\.[rp])?'),
    'numfmt': ('tests/numfmt/numfmt.pl', 556, 557, 561, r'delim-7'),
    'tr': ('tests/tr/tr.pl', 128, 134, 129, r'(empty-eq|empty-cc|no-abort-1)\.[rp]'),
    'date': ('tests/date/date.pl', 993, 996, 994, r'(dbg_)?wide-fmt|invalid-TZ-crash'),
    'unexpand': ('tests/unexpand/unexpand.pl', 54, 60, 55, r'infloop-[1-4]|blanks-ext[12]'),
}
parser = argparse.ArgumentParser()
parser.add_argument('command', choices=PROFILES)
parser.add_argument('--base', type=Path, required=True)
options = parser.parse_args()
command = options.command
script, selected, original_count, process_count, held_pattern = PROFILES[command]
stem = 'gnu-'+command+'-expanded'
runtime_stem = 'gnu-pr-page-cleanup' if command == 'pr' else stem
driver = ROOT/'evidence/raw/gnu-resource-tac-driver.py'
sys.path[:0] = [str(ROOT/'tests/gnu'), str(ROOT/'scripts')]
spec = importlib.util.spec_from_file_location('reviewed', driver)
runner = importlib.util.module_from_spec(spec)
spec.loader.exec_module(runner)
target = ROOT/f'evidence/{stem}-validation.json'
coverage_target = ROOT/f'evidence/{stem}-coverage.json'
assert not target.exists() and not coverage_target.exists()
raw = {}


def pin(name, expected=None):
    path = Path(name)
    if not path.is_absolute(): path = ROOT/path
    value = hashlib.sha256(path.read_bytes()).hexdigest()
    if expected and value != expected and path == ROOT/'tests/gnu/reviewed-original.py':
        return pin(driver, expected)
    assert expected is None or value == expected, str(path)
    raw[str(path.relative_to(ROOT)) if path.is_relative_to(ROOT) else str(path)] = value
    return value


definitions = ROOT/f'evidence/raw/{stem}-inventory.json'; pin(definitions)
definition, = [r for r in json.loads(definitions.read_text()) if r['script'] == script]
review_path = ROOT/f'evidence/{stem}-review.json'; pin(review_path)
review = json.loads(review_path.read_text())
pin(SOURCE/script, definition['sha256'])
assert review['script_sha256'] == definition['sha256']
pin(review['collector'], review['collector_sha256'])
pin(SOURCE/'tests/Coreutils.pm', review['generator_sha256'])
for suffix in ('stdout', 'stderr'): pin(f'build/{command}-case-inventory.{suffix}')
assert (ROOT/f'build/{command}-case-inventory.stdout').read_text().splitlines() == review['names']
assert len(review['names']) == len(set(review['names'])) == original_count
held = [n for n in review['names'] if re.fullmatch(held_pattern, n)]
assert held == review['held'] == definition['excluded_cases']
assert review['selected'] == definition['cases'] == [n for n in review['names'] if n not in held]
assert len(definition['cases']) == definition['expected_case_count'] == selected
assert definition['original_case_count'] == original_count and not definition['full_suite']
if command == 'pr':
    # This matrix reads expected outputs and input fixtures directly from the source tree.
    for path in sorted((SOURCE/'tests/pr').iterdir()):
        if path.is_file(): pin(path)
if definition.get('locale_profile'):
    path = ROOT/'evidence/test-locales.json'; pin(path)
    locales = json.loads(path.read_text())
    for locale in locales['locales']:
        for suffix, value in locale['files'].items():
            pin(ROOT/locales['path']/locale['name']/suffix, value)
            pin(Path(locales['runtime_path'])/locale['name']/suffix, value)
candidate = ROOT/('target/pr-page-cleanup-candidate/release/rboxc' if command == 'pr'
                  else 'target/expr-cleanup-candidate/release/rboxc')
candidate_hash = pin(candidate)
images, totals = {}, {}
instrumentation_baseline = ['large-2a', 'large-10', 'large-14', 'large-15', 'large-16']
for instrument in (False, True):
    report = ROOT/f'evidence/raw/{runtime_stem}-{"valgrind" if instrument else "original"}.json'
    pin(report)
    result, = json.loads(report.read_text())['results']
    baseline = instrument and command == 'numfmt'
    assert result['pass'] == (not baseline)
    assert result['state'] == ('open' if baseline else 'pass')
    assert all(result[k] == v for k, v in definition.items())
    checkpoint = report.with_suffix('.progress.json'); pin(checkpoint)
    saved = json.loads(checkpoint.read_text())['runs'][script]; context = saved['context']
    assert context['definition'] == definition and context['valgrind'] == instrument
    pin('/bin/sh', context['shell']); pin('build/gnu-coreutils/lib/config.h', context['config_header'])
    pin('build/gnu-coreutils/src/getlimits', context['getlimits'])
    for path, value in context['drivers'].items(): pin(path, value)
    for path, value in context['gnu_harness'].items(): pin(SOURCE/path, value)
    for path, value in (context.get('valgrind_runtime') or {}).items(): pin(path, value)
    if definition.get('locale_profile'): pin('evidence/test-locales.json', context['locales'])
    for implementation in ('gnu', 'rboxc'):
        outcome = result[implementation]
        assert saved['outcomes'][implementation]['result'] == outcome
        assert outcome['status'] == int(baseline)
        assert outcome['binary_sha256'] == context['binaries'][implementation]
        pin('build/gnu-coreutils/src/coreutils' if implementation == 'gnu' else candidate,
            outcome['binary_sha256'])
        pin(outcome['watchdog']['path'], outcome['watchdog']['sha256'])
        for path, value in saved['outcomes'][implementation]['logs'].items(): pin(path, value)
        log = (ROOT/outcome['log']).read_text()
        assert outcome['case_count'] == selected and outcome['case_count_pass']
        assert log.count(f'RBOXC_SELECTION {selected} of {original_count}\n') == 1
        failures = re.findall(r'^-e: test (.*)$', log, re.M)
        assert failures == ([f'{name}: stdout mismatch, comparing {name}.1 (expected) and {name}.O (actual)'
                             for name in instrumentation_baseline] if baseline else [])
        if not instrument: continue
        assert len(outcome['memory']) == process_count, (command, implementation, len(outcome['memory']))
        calls, clean = [], 0
        for memory in outcome['memory']:
            path = ROOT/memory['log']; text = path.read_text()
            parsed = runner.parse_memory_log(text, path.stem)
            assert all(memory[k] == v for k, v in parsed.items())
            assert set(re.findall(r'^==([0-9]+)==', text, re.M)) == {path.stem}
            image, = re.findall(r'^==[0-9]+== Command: (.*)$', text, re.M)
            assert image.split()[0] in definition.get('commands', [command]); calls.append(image)
            for summary in ('ERROR SUMMARY:', 'FILE DESCRIPTORS:'):
                assert text.count(summary) == 1 and text.index(summary) > text.index('Command:')
            ok = (parsed['errors'] == parsed['non_inherited_descriptors'] == 0
                  and not any(parsed['heap_bytes'].get(k, 0) for k in
                              ('definitely lost', 'indirectly lost', 'possibly lost')))
            if implementation == 'rboxc': assert ok, memory['log']
            clean += ok
        images[implementation] = Counter(calls)
        totals[implementation] = {'images': process_count, 'clean': clean}
assert images['gnu'] == images['rboxc']
assert images['rboxc'][command+' --version'] == 1
if command == 'numfmt':
    assert sum(n for call, n in images['rboxc'].items() if call.startswith('printf ')) == 3
    assert images['rboxc']['numfmt ---debug 1'] == 1
    path = ROOT/'evidence/numfmt-expanded-instrumentation.json'; pin(path)
    profile = json.loads(path.read_text())
    assert profile['accounting_pass'] and not profile['strict_valgrind_assertions_pass']
    assert profile['binary_sha256'] == candidate_hash
    assert [r['case'] for r in profile['results']] == instrumentation_baseline
    pin('tests/numfmt-instrumentation.py')
    manual = Path('/usr/share/doc/valgrind/html/manual-core.html'); pin(manual)
    assert 'Precision: There is no support for 80 bit arithmetic.' in manual.read_text()
    for case in profile['results']:
        assert case['gnu']['native'] == case['rboxc']['native'] == {
            'status': 0, 'stdout': case['expected_stdout'], 'stderr': ''}
        assert case['gnu']['instrumented'] == case['rboxc']['instrumented']
        assert case['rboxc']['instrumented']['stdout'] != case['expected_stdout']
        for implementation in ('gnu', 'rboxc'):
            log = ROOT/case[implementation]['log']; pin(log)
            parsed = runner.parse_memory_log(log.read_text(), case['case'])
            assert parsed == case[implementation]['memory']
            if implementation == 'rboxc':
                assert parsed['errors'] == parsed['non_inherited_descriptors'] == 0
                assert not any(parsed['heap_bytes'].get(k, 0) for k in
                               ('definitely lost', 'indirectly lost', 'possibly lost'))
base_path = options.base.resolve(strict=True); base_hash = pin(base_path)
base = json.loads(base_path.read_text()); coverage = copy.deepcopy(base)
row, = [r for r in coverage['results'] if r['script'] == script]
assert row['state'] == 'partial' and row['selected_case_count'] < selected
row.update(selected_case_count=selected, original_case_count=original_count, held_cases=held,
           valgrind_state='open' if command == 'numfmt' else 'passed-selection', binary_sha256=candidate_hash,
           evidence=f'evidence/raw/{runtime_stem}-original.json',
           valgrind_evidence=f'evidence/raw/{runtime_stem}-valgrind.json', audit=str(target.relative_to(ROOT)))
row.pop('outcomes', None)
if command == 'numfmt':
    row.update(valgrind_memory_clean=True, instrumentation_baseline_cases=instrumentation_baseline,
               instrumentation_baseline_evidence='evidence/numfmt-expanded-instrumentation.json',
               execution_note='All selected original assertions pass normally. Five numeric assertions fail identically for GNU and Rboxc under Valgrind; the strict instrumented profile remains open with all candidate memory logs clean.')
assert all(a == b for a, b in zip(base['results'], coverage['results']) if a['script'] != script)
coverage['counts'] = dict(sorted(Counter(r['state'] for r in coverage['results']).items()))
coverage['valgrind_counts'] = dict(sorted(Counter(r['valgrind_state'] for r in coverage['results']).items()))
coverage.update(base_report=str(base_path.relative_to(ROOT)), base_report_sha256=base_hash,
    scope=f'Pinned 733-script ledger with {selected}/{original_count} ordinary {command} cases; historical omissions remain held. Other rows and installed release unchanged.')
if command == 'numfmt':
    coverage['scope'] += ' Five matching numeric differences under Valgrind remain explicitly open.'
coverage_target.write_text(json.dumps(coverage, indent=2)+'\n')
coverage_hash = pin(coverage_target)
target.write_text(json.dumps({'scope': coverage['scope'], 'script': script,
    'selected_cases': selected, 'held_cases': held, 'results': totals, 'raw': raw,
    'coverage': str(coverage_target.relative_to(ROOT)), 'coverage_sha256': coverage_hash,
    'accounting_pass': True, 'full_script_pass': False}, indent=2)+'\n')
print(f'PASS: {command} {selected}/{original_count} cases, {process_count} clean candidate images')
