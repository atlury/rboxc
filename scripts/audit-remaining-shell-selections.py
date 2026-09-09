#!/usr/bin/env python3
"""Independently audit ordinary selections without certifying whole scripts."""
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


def module(name, path):
    spec = importlib.util.spec_from_file_location(name, ROOT/path)
    result = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(result)
    return result


prefix = 'gnu-remaining-shell'
runner_path = 'evidence/raw/gnu-remaining-shell-driver.py'
selector_path = 'evidence/raw/gnu-remaining-shell-helper.py'
runner = module('reviewed', runner_path)
selector = module('selector', selector_path)
raw, reports, images, totals = {}, {}, {}, {}
sha = lambda p: hashlib.sha256(Path(p).read_bytes()).hexdigest()
target = ROOT/f'evidence/{prefix}-validation.json'
coverage_target = ROOT/f'evidence/{prefix}-coverage.json'
assert not target.exists() and not coverage_target.exists()
historic = {'tests/gnu/reviewed-original.py': runner_path,
            'scripts/gnu_shell_selections.py': selector_path}
expected = {'tests/ptx/ptx-overrun.sh': 4,
            'tests/ln/backup-suffix-traversal.sh': 2,
            'tests/tail/follow-stdin.sh': 3}

def report_stem(script):
    return 'gnu-remaining-shell'


def pin(path, expected=None):
    path = Path(path)
    if not path.is_absolute():
        path = ROOT/path
    if expected and sha(path) != expected and path.is_relative_to(ROOT):
        path = ROOT/historic[str(path.relative_to(ROOT))]
    value = sha(path)
    assert expected is None or value == expected, str(path)
    raw[str(path.relative_to(ROOT)) if path.is_relative_to(ROOT) else str(path)] = value
    return value


snapshot = ROOT/f'evidence/raw/{prefix}-inventory.json'
pin(snapshot)
definitions = {r['script']: r for r in json.loads(snapshot.read_text())}
review_path = ROOT/f'evidence/{prefix}-review.json'
pin(review_path)
review = json.loads(review_path.read_text())
assert {r['script'] for r in review['selections']} == set(expected) <= set(selector.EXCLUDED)
for selected in review['selections']:
    script = selected['script']
    definition = definitions[script]
    data, metadata = selector.selection(SOURCE, script)
    assert definition['shell_selection'] == metadata
    assert all(selected[k] == v for k, v in metadata.items())
    assert not definition['full_suite'] and not metadata['full_script']
    original = (SOURCE/script).read_bytes()
    pin(SOURCE/script, metadata['original_sha256'])
    # Reassemble directly from the reviewed byte intervals as a second check.
    parts, cursor = [], 0
    for interval in metadata['excluded_ranges']:
        a, b = interval['start_byte'], interval['end_byte_exclusive']
        assert cursor <= a < b <= len(original)
        assert hashlib.sha256(original[a:b]).hexdigest() == interval['sha256']
        parts.append(original[cursor:a])
        cursor = b
    assert b''.join(parts) + original[cursor:] == data
    pin(selected['selected_path'], metadata['selected_sha256'])
    assert (ROOT/selected['selected_path']).read_bytes() == data
    stem = report_stem(script)
    for instrument in (False, True):
        report = ROOT/f'evidence/raw/{stem}-{"valgrind" if instrument else "original"}.json'
        reports[str(report.relative_to(ROOT))] = pin(report)
        checkpoint = report.with_suffix('.progress.json')
        pin(checkpoint)
        saved = json.loads(checkpoint.read_text())['runs'][script]
        context = saved['context']
        assert context['definition'] == definition and context['valgrind'] == instrument
        result, = [r for r in json.loads(report.read_text())['results'] if r['script'] == script]
        assert result['pass'] and result['state'] == 'pass'
        assert all(result[k] == v for k, v in definition.items())
        pin('/bin/sh', context['shell'])
        pin('build/gnu-coreutils/lib/config.h', context['config_header'])
        pin('build/gnu-coreutils/src/getlimits', context['getlimits'])
        pin('scripts/gnu_shell_selections.py', context['shell_selection_driver'])
        pin(context['selected_script']['path'], context['selected_script']['sha256'])
        assert context['selected_script']['sha256'] == metadata['selected_sha256']
        for path, value in context['drivers'].items():
            pin(path, value)
        for path, value in context['gnu_harness'].items():
            pin(SOURCE/path, value)
        for path, value in (context.get('valgrind_runtime') or {}).items():
            pin(path, value)
        for implementation in ('gnu', 'rboxc'):
            outcome = result[implementation]
            assert saved['outcomes'][implementation]['result'] == outcome
            assert outcome['status'] == 0
            staged = outcome['selected_runtime_script']
            assert staged['sha256'] == metadata['selected_sha256']
            assert staged['launch_credentials'] == {}
            assert outcome['binary_sha256'] == context['binaries'][implementation]
            binary = ('build/gnu-coreutils/src/coreutils' if implementation == 'gnu'
                      else 'target/pr-page-cleanup-candidate/release/rboxc')
            pin(binary, outcome['binary_sha256'])
            pin(outcome['watchdog']['path'], outcome['watchdog']['sha256'])
            for path, value in saved['outcomes'][implementation]['logs'].items():
                pin(path, value)
            log = (ROOT/outcome['log']).read_bytes().decode('utf-8', 'backslashreplace')
            assert re.search(r'^\+ Exit(?: 0)?$', log, re.M)
            assert '+ __st=0\n' in log
            if not instrument:
                continue
            command_images = []
            clean = 0
            assert outcome['memory']
            for memory in outcome['memory']:
                path = ROOT/memory['log']
                text = path.read_text()
                parsed = runner.parse_memory_log(text, path.stem)
                assert all(memory[k] == v for k, v in parsed.items())
                assert set(re.findall(r'^==([0-9]+)==', text, re.M)) == {path.stem}
                image, = re.findall(r'^==[0-9]+== Command: (.*)$', text, re.M)
                assert image.split()[0] in definition['commands'], image
                command_images.append(image)
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
            images[script, implementation] = Counter(command_images)
            totals.setdefault(script, {})[implementation] = {
                'images': len(command_images), 'clean': clean,
                'commands': dict(sorted(Counter(command_images).items())),
            }
    assert images[script, 'gnu'] == images[script, 'rboxc']
    assert totals[script]['rboxc']['images'] == expected[script]
base_path = ROOT/'evidence/gnu-native-resource-coverage.json'
base = json.loads(base_path.read_text())
pin(base_path)
coverage = copy.deepcopy(base)
for row in coverage['results']:
    if row['script'] not in expected:
        continue
    assert row['state'] == row['valgrind_state'] == 'pending'
    script = row['script']
    stem = report_stem(script)
    row.update(state='partial', execution_coverage='selected-shell-sections',
               shell_selection=definitions[script]['shell_selection'],
               valgrind_state='passed-selection',
               evidence=f'evidence/raw/{stem}-original.json',
               valgrind_evidence=f'evidence/raw/{stem}-valgrind.json',
               audit=str(target.relative_to(ROOT)),
               binary_sha256=sha(ROOT/'target/pr-page-cleanup-candidate/release/rboxc'))
assert all(a == b for a, b in zip(base['results'], coverage['results'])
           if a['script'] not in expected)
coverage['counts'] = dict(sorted(Counter(r['state'] for r in coverage['results']).items()))
coverage['valgrind_counts'] = dict(sorted(Counter(r['valgrind_state'] for r in coverage['results']).items()))
coverage.update(base_report=str(base_path.relative_to(ROOT)), base_report_sha256=sha(base_path),
                scope=f'Pinned 733-script ledger extended by {len(expected)} ordinary shell selections. Excluded intervals remain unexecuted; full script and release acceptance remain open.')
coverage_target.write_text(json.dumps(coverage, indent=2)+'\n')
target.write_text(json.dumps({
    'scope': 'Original assertions pass normally and under Valgrind for all selected scripts. Every candidate image is clean; native GNU findings remain recorded. These are partial script results.',
    'results': totals, 'raw': raw, 'reports': reports,
    'coverage': str(coverage_target.relative_to(ROOT)),
    'coverage_sha256': sha(coverage_target),
    'clean_candidate_processes': sum(r['rboxc']['clean'] for r in totals.values()),
    'accounting_pass': True, 'selected_sections_pass': True, 'full_suite_pass': False,
}, indent=2)+'\n')
print('PASS:', len(expected), 'ordinary shell selections;', sum(r['rboxc']['clean'] for r in totals.values()), 'clean candidate images')
