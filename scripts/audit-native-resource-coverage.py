#!/usr/bin/env python3
"""Audit three native memory budgets and a finite timer-parameter profile."""
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
spec = importlib.util.spec_from_file_location('reviewed', ROOT/'tests/gnu/reviewed-original.py')
runner = importlib.util.module_from_spec(spec); spec.loader.exec_module(runner)
target = ROOT/'evidence/gnu-native-resource-validation.json'
coverage_target = ROOT/'evidence/gnu-native-resource-coverage.json'
assert not target.exists() and not coverage_target.exists()
raw, results = {}, {}


def pin(name, expected=None):
    path = Path(name)
    if not path.is_absolute(): path = ROOT/path
    value = hashlib.sha256(path.read_bytes()).hexdigest()
    assert expected is None or value == expected, str(path)
    raw[str(path.relative_to(ROOT)) if path.is_relative_to(ROOT) else str(path)] = value
    return value


definitions_path = ROOT/'evidence/raw/gnu-timeout-large-inventory.json'; pin(definitions_path)
definitions = {r['script']: r for r in json.loads(definitions_path.read_text())}
for name in ('gnu-before-native-resource-inventory', 'gnu-native-resource-inventory',
             'gnu-before-timeout-large-inventory'): pin('evidence/raw/'+name+'.json')
candidate_hash = pin('target/pr-page-cleanup-candidate/release/rboxc')
for stem, instrument in (('gnu-native-resource-original', False),
                         ('gnu-timeout-large-original', False), ('gnu-timeout-large-valgrind', True)):
    report = ROOT/f'evidence/raw/{stem}.json'; pin(report)
    checkpoint = report.with_suffix('.progress.json'); pin(checkpoint)
    saved_runs = json.loads(checkpoint.read_text())['runs']
    for result in json.loads(report.read_text())['results']:
        script = result['script']; definition = definitions[script]
        assert definition['full_suite'] and not definition.get('shell_selection')
        assert result['pass'] and result['state'] == 'pass'
        assert all(result[k] == v for k, v in definition.items())
        pin(SOURCE/script, definition['sha256'])
        saved = saved_runs[script]; context = saved['context']
        assert context['definition'] == definition and context['valgrind'] == instrument
        pin('/bin/sh', context['shell']); pin('build/gnu-coreutils/lib/config.h', context['config_header'])
        pin('build/gnu-coreutils/src/getlimits', context['getlimits'])
        for path, value in context['drivers'].items(): pin(path, value)
        for path, value in context['gnu_harness'].items(): pin(SOURCE/path, value)
        for path, value in (context.get('valgrind_runtime') or {}).items(): pin(path, value)
        if definition.get('locale_profile'):
            p = ROOT/'evidence/test-locales.json'; pin(p, context['locales'])
            locales = json.loads(p.read_text())
            for locale in locales['locales']:
                for suffix, value in locale['files'].items():
                    pin(ROOT/locales['path']/locale['name']/suffix, value)
                    pin(Path(locales['runtime_path'])/locale['name']/suffix, value)
        for implementation in ('gnu', 'rboxc'):
            outcome = result[implementation]
            assert saved['outcomes'][implementation]['result'] == outcome
            assert outcome['status'] == 0
            pin('build/gnu-coreutils/src/coreutils' if implementation == 'gnu'
                else 'target/pr-page-cleanup-candidate/release/rboxc', outcome['binary_sha256'])
            pin(outcome['watchdog']['path'], outcome['watchdog']['sha256'])
            for path, value in saved['outcomes'][implementation]['logs'].items(): pin(path, value)
            log = (ROOT/outcome['log']).read_text()
            assert '+ __st=0\n' in log and re.search(r'^\+ Exit(?: 0)?$', log, re.M)
            entry = results.setdefault(script, {}).setdefault(implementation, {})
            if script.startswith('tests/timeout/'):
                assert '+ test 0 = 124\n' in log
                assert '+ returns_ 125 timeout -- -' in log
            else:
                # Only trace lines from the main test follow the assigned calibration.
                vms = [int(v) for v in re.findall(r'^\+ vm=(\d+)$', log, re.M)]
                assert vms and 1000 < vms[0] <= 50004
                body = log[log.index('+ vm='):]
                limits = [int(v) for v in re.findall(r'^\+ ulimit -v (\d+)$', body, re.M)]
                if definition['command'] == 'csplit':
                    assert limits == [vms[0]+40000]
                    assert '+ head -n2500000\n' in body and '+ csplit -z - %n%1\n' in body
                elif definition['command'] == 'cut':
                    assert vms == [vms[0], vms[0]+1000]
                    assert limits == [vms[-1]]*4
                    assert '+ compare /dev/null err\n' in body
                else:
                    assert limits == [vms[0]+6000]*3
                    assert body.count('+ test 1 = 124\n') == 3
                    assert log.count('+ test 6553 -eq 6553\n') == 2
                    assert log.count('+ test 0 -eq 0\n') >= 3
                entry.update(calibration_kib=vms[0], exercised_limits_kib=limits)
            if not instrument: continue
            assert len(outcome['memory']) == 12
            calls = Counter()
            for memory in outcome['memory']:
                p = ROOT/memory['log']; text = p.read_text()
                parsed = runner.parse_memory_log(text, p.stem)
                assert all(memory[k] == v for k, v in parsed.items())
                assert parsed['errors'] == parsed['non_inherited_descriptors'] == 0
                assert not any(parsed['heap_bytes'].get(k, 0) for k in
                               ('definitely lost', 'indirectly lost', 'possibly lost'))
                assert text.count('ERROR SUMMARY:') == text.count('FILE DESCRIPTORS:') == 1
                image, = re.findall(r'^==\d+== Command: (.*)$', text, re.M)
                parts = image.split(); parts[0] = Path(parts[0]).name; calls[' '.join(parts)] += 1
            assert sum(n for name, n in calls.items() if name.startswith('timeout ')) == 7
            assert calls['timeout --version'] == 1 and calls['sleep 3'] == 1 and calls['sleep 0'] == 4
            entry.update(clean_process_images=12, commands=dict(calls))
timeout = results['tests/timeout/timeout-large-parameters.sh']
assert timeout['gnu']['commands'] == timeout['rboxc']['commands']
base_path = ROOT/'evidence/gnu-date-expanded-coverage.json'; base_hash = pin(base_path)
base = json.loads(base_path.read_text()); coverage = copy.deepcopy(base)
for row in coverage['results']:
    if row['script'] not in results: continue
    assert row['state'] == row['valgrind_state'] == 'pending'
    is_timeout = row['script'].startswith('tests/timeout/')
    row.update(state='passed', execution_coverage='whole-script', binary_sha256=candidate_hash,
        evidence='evidence/raw/gnu-timeout-large-original.json' if is_timeout else 'evidence/raw/gnu-native-resource-original.json',
        audit=str(target.relative_to(ROOT)))
    if is_timeout:
        row.update(valgrind_state='passed-script', valgrind_evidence='evidence/raw/gnu-timeout-large-valgrind.json')
    else:
        row['execution_note'] = 'Whole original native memory-budget script passes. Its calibration is bounded below Valgrind startup needs; instrumented resource validation remains pending.'
assert all(a == b for a, b in zip(base['results'], coverage['results']) if a['script'] not in results)
coverage['counts'] = dict(sorted(Counter(r['state'] for r in coverage['results']).items()))
coverage['valgrind_counts'] = dict(sorted(Counter(r['valgrind_state'] for r in coverage['results']).items()))
coverage.update(base_report=str(base_path.relative_to(ROOT)), base_report_sha256=base_hash,
    scope='Four whole original scripts pass: native csplit/cut/fold memory budgets and finite timeout parameters. Timeout also passes under Valgrind; the three native resource profiles do not claim instrumented passes.')
coverage_target.write_text(json.dumps(coverage, indent=2)+'\n')
target.write_text(json.dumps({'scope': coverage['scope'], 'results': results, 'raw': raw,
    'coverage': str(coverage_target.relative_to(ROOT)), 'coverage_sha256': pin(coverage_target),
    'accounting_pass': True, 'full_coreutils_complete': False}, indent=2)+'\n')
print('PASS: four whole native scripts, 12 clean timeout/sleep candidate images; three instrumented budgets remain pending')
