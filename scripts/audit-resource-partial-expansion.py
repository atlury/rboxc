#!/usr/bin/env python3
"""Audit a whole od resource profile and expanded tac/expr/seq selections."""
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
driver = ROOT/'evidence/raw/gnu-resource-tac-driver.py'
spec = importlib.util.spec_from_file_location('reviewed', driver)
runner = importlib.util.module_from_spec(spec)
spec.loader.exec_module(runner)
sha = lambda p: hashlib.sha256(Path(p).read_bytes()).hexdigest()
target = ROOT/'evidence/gnu-resource-partial-expansion-validation.json'
coverage_target = ROOT/'evidence/gnu-resource-partial-expansion-coverage.json'
assert not target.exists() and not coverage_target.exists()
raw, reports, totals, binaries = {}, {}, {}, {}


def pin(path, expected=None):
    path = Path(path)
    if not path.is_absolute(): path = ROOT/path
    if expected and sha(path) != expected and path == ROOT/'tests/gnu/reviewed-original.py':
        path = driver
    value = sha(path)
    assert expected is None or value == expected, str(path)
    raw[str(path.relative_to(ROOT)) if path.is_relative_to(ROOT) else str(path)] = value
    return value


definitions = {r['script']: r for r in json.loads((ROOT/'evidence/raw/gnu-seq-expanded-inventory.json').read_text())}
for p in ('evidence/raw/gnu-expr-expanded-inventory.json', 'evidence/raw/gnu-resource-tac-inventory.json',
          'evidence/raw/gnu-before-resource-tac-inventory.json', 'evidence/raw/gnu-seq-expanded-inventory.json'): pin(p)
cases = {'tests/tac/tac.pl': (85, 90, 86, 'gnu-resource-tac'),
         'tests/od/big-w.sh': (None, None, 7, 'gnu-resource-tac'),
         'tests/expr/expr.pl': (205, 207, 206, 'gnu-expr-cleanup'),
         'tests/seq/seq.pl': (77, 78, 78, 'gnu-seq-expanded')}
for script, (selected, original_count, process_count, stem) in cases.items():
    definition = definitions[script]
    candidate = ('target/expr-cleanup-candidate/release/rboxc' if script == 'tests/expr/expr.pl'
                 else 'target/iconv-charmap-cleanup-candidate/release/rboxc')
    binaries[script] = pin(candidate)
    pin(SOURCE/script, definition['sha256'])
    if selected:
        command = definition['command']
        review_path = ROOT/f'evidence/gnu-{command}-expanded-review.json'
        review = json.loads(review_path.read_text()); pin(review_path)
        assert review['script_sha256'] == definition['sha256']
        pin(review['collector'], review['collector_sha256'])
        pin(SOURCE/'tests/Coreutils.pm', review['generator_sha256'])
        for suffix in ('stdout', 'stderr'): pin(f'build/{command}-case-inventory.{suffix}')
        assert (ROOT/f'build/{command}-case-inventory.stdout').read_text().splitlines() == review['names']
        assert len(review['names']) == len(set(review['names'])) == original_count
        held = ([n for n in review['names'] if re.fullmatch(r'(segfault2?|double-free)(?:\.[rp])?', n)]
                if command == 'tac' else ['emptysub', 'emptysub-mb'] if command == 'expr' else ['long-1'])
        assert held == review['held'] == definition['excluded_cases']
        assert review['selected'] == definition['cases'] == [n for n in review['names'] if n not in held]
        assert len(definition['cases']) == definition['expected_case_count'] == selected
        assert definition['original_case_count'] == original_count and not definition['full_suite']
    else:
        assert definition['full_suite'] and not definition.get('shell_selection')
        assert definition['address_space_limit_bytes'] == 1024**3 and definition['very_expensive']
    if definition.get('locale_profile'):
        p = ROOT/'evidence/test-locales.json'; locales = json.loads(p.read_text()); pin(p)
        for locale in locales['locales']:
            for suffix, value in locale['files'].items():
                pin(ROOT/locales['path']/locale['name']/suffix, value)
                pin(Path(locales['runtime_path'])/locale['name']/suffix, value)
    images = {}
    for instrument in (False, True):
        report = ROOT/f'evidence/raw/{stem}-{"valgrind" if instrument else "original"}.json'
        reports[str(report.relative_to(ROOT))] = pin(report)
        data = json.loads(report.read_text())
        result, = [r for r in data['results'] if r['script'] == script]
        assert result['pass'] and result['state'] == 'pass'
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
            assert outcome['status'] == 0 and outcome['binary_sha256'] == context['binaries'][implementation]
            pin('build/gnu-coreutils/src/coreutils' if implementation == 'gnu'
                else candidate, outcome['binary_sha256'])
            pin(outcome['watchdog']['path'], outcome['watchdog']['sha256'])
            for path, value in saved['outcomes'][implementation]['logs'].items(): pin(path, value)
            log = (ROOT/outcome['log']).read_text()
            if selected:
                assert outcome['case_count'] == selected and outcome['case_count_pass']
                assert log.count(f'RBOXC_SELECTION {selected} of {original_count}\n') == 1
            else:
                profile = outcome['address_space_profile']
                assert profile == context['address_space_profile'] and profile['bytes'] == 1024**3
                pin(profile['limiter'], profile['limiter_sha256'])
                assert log.count('+ test ! -s out\n') == 2
                assert '+ test 185381 -eq 185381\n' in log
                assert '+ test 185385 -eq 185385\n' in log
                assert re.search(r'^\+ Exit(?: 0)?$', log, re.M) and '+ __st=0\n' in log
            if not instrument: continue
            if script == 'tests/tac/tac.pl':
                pin('tests/gnu/valgrind-tmpdir.c', outcome['tmpdir_adapter_source_sha256'])
            assert len(outcome['memory']) == process_count
            clean, calls = 0, []
            for memory in outcome['memory']:
                path = ROOT/memory['log']; text = path.read_text()
                parsed = runner.parse_memory_log(text, path.stem)
                assert all(memory[k] == v for k, v in parsed.items())
                assert set(re.findall(r'^==([0-9]+)==', text, re.M)) == {path.stem}
                image, = re.findall(r'^==[0-9]+== Command: (.*)$', text, re.M)
                assert image.split()[0] == definition['command']; calls.append(image)
                errors = list(re.finditer('ERROR SUMMARY:', text)); fds = list(re.finditer('FILE DESCRIPTORS:', text))
                assert len(errors) == len(fds) == 1
                assert min(errors[0].start(), fds[0].start()) > text.index('Command:')
                ok = (parsed['errors'] == parsed['non_inherited_descriptors'] == 0
                      and not any(parsed['heap_bytes'].get(k, 0) for k in ('definitely lost', 'indirectly lost', 'possibly lost')))
                if implementation == 'rboxc': assert ok, memory['log']
                clean += ok
            images[implementation] = Counter(calls)
            totals.setdefault(script, {})[implementation] = {'images': process_count, 'clean': clean}
    assert images['gnu'] == images['rboxc']
    if script == 'tests/od/big-w.sh':
        assert images['rboxc'] == Counter({'od --version': 1, 'od -w46340 -tcz': 2,
            'od -w46341 -tcz': 2, 'od -w3037000500 -tcz': 1, 'od -w3037000501 -tcz': 1})

base_path = ROOT/'evidence/gnu-sort-selection-coverage.json'; pin(base_path)
base = json.loads(base_path.read_text()); coverage = copy.deepcopy(base)
for row in coverage['results']:
    if row['script'] not in cases: continue
    selected, original_count, _, stem = cases[row['script']]
    if selected:
        assert row['state'] == 'partial' and row['selected_case_count'] < selected
        row.update(selected_case_count=selected, original_case_count=original_count,
                   held_cases=definitions[row['script']]['excluded_cases'], valgrind_state='passed-selection')
    else:
        assert row['state'] == row['valgrind_state'] == 'pending'
        row.update(state='passed', execution_coverage='whole-script', valgrind_state='passed-script',
                   address_space_limit_bytes=1024**3,
                   execution_note='Whole unmodified script under a 1 GiB ceiling: both small widths produce their expected output and both large widths take the original accepted allocation-failure branch.')
    row.pop('outcomes', None)
    row.update(evidence=f'evidence/raw/{stem}-original.json', valgrind_evidence=f'evidence/raw/{stem}-valgrind.json',
               audit=str(target.relative_to(ROOT)), binary_sha256=binaries[row['script']])
assert all(a == b for a, b in zip(base['results'], coverage['results']) if a['script'] not in cases)
coverage['counts'] = dict(sorted(Counter(r['state'] for r in coverage['results']).items()))
coverage['valgrind_counts'] = dict(sorted(Counter(r['valgrind_state'] for r in coverage['results']).items()))
coverage.update(base_report=str(base_path.relative_to(ROOT)), base_report_sha256=sha(base_path),
                scope='Pinned 733-script ledger: whole od width script passes under its recorded resource profile; tac, expr and seq selections expand with all historical omissions retained. Other rows and installed release are unchanged.')
coverage_target.write_text(json.dumps(coverage, indent=2)+'\n')
target.write_text(json.dumps({'scope': coverage['scope'], 'results': totals, 'raw': raw, 'reports': reports,
    'clean_candidate_processes': sum(r['rboxc']['clean'] for r in totals.values()),
    'coverage': str(coverage_target.relative_to(ROOT)), 'coverage_sha256': sha(coverage_target),
    'accounting_pass': True, 'full_coreutils_complete': False}, indent=2)+'\n')
print('PASS: od whole resource profile; expanded tac/expr/seq selections;', sum(r['rboxc']['clean'] for r in totals.values()), 'clean candidate images')
