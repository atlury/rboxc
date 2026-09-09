#!/usr/bin/env python3
"""Audit full debugger originals and the expanded ordinary terminal selection."""
# SPDX-License-Identifier: GPL-3.0-or-later
from collections import Counter
import copy
import hashlib
import importlib.util
import json
from pathlib import Path
import re

ROOT = Path(__file__).resolve().parents[1]
SOURCE = Path('/opt/src/coreutils-9.11')
PREFIX = 'gnu-tail-profiles'
raw = {}
def pin(name, expected=None):
    p = Path(name)
    if not p.is_absolute():
        p = ROOT/p
    value = hashlib.sha256(p.read_bytes()).hexdigest()
    assert expected is None or value == expected, str(p)
    raw[str(p.relative_to(ROOT)) if p.is_relative_to(ROOT) else str(p)] = value
    return value

def read(name):
    pin(name)
    return json.loads((ROOT/name).read_text())

def module(name, path):
    import sys
    sys.path[:0] = [str(ROOT/'tests/gnu'), str(ROOT/'scripts')]
    spec = importlib.util.spec_from_file_location(name, ROOT/path)
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod

output = ROOT/f'evidence/{PREFIX}-validation.json'
ledger = ROOT/f'evidence/{PREFIX}-coverage.json'
assert not output.exists() and not ledger.exists()
runner = module('reviewed', 'evidence/raw/gnu-tail-terminal-driver.py')
selector = module('selector', 'evidence/raw/gnu-tail-terminal-helper.py')
definitions = {r['script']: r for r in read('evidence/raw/gnu-tail-debug-ready-inventory.json')}
terminal = 'tests/tail/follow-stdin.sh'
debug_scripts = ['tests/tail/inotify-race.sh', 'tests/tail/inotify-race2.sh']
summaries = {}
for stem, scripts, instrument in [
    ('gnu-tail-debug-ready-original', debug_scripts, False),
    ('gnu-tail-terminal-original', [terminal], False),
    ('gnu-tail-terminal-valgrind', [terminal], True),
]:
    report = read(f'evidence/raw/{stem}.json')
    checkpoint = read(f'evidence/raw/{stem}.progress.json')
    for script in scripts:
        result, = [r for r in report['results'] if r['script'] == script]
        saved = checkpoint['runs'][script]
        context = saved['context']
        definition = definitions[script]
        assert context['definition'] == definition
        assert all(result[k] == v for k, v in definition.items())
        assert context['valgrind'] == instrument
        assert result['state'] == ('open' if instrument else 'pass')
        assert result['pass'] == (not instrument)
        pin(SOURCE/script, definition['sha256'])
        for path, value in context['drivers'].items():
            if path == 'tests/gnu/reviewed-original.py':
                path = ('evidence/raw/gnu-tail-terminal-driver.py' if script == terminal
                        else 'evidence/raw/gnu-tail-debug-ready-driver.py')
            pin(path, value)
        for path, value in context['gnu_harness'].items():
            pin(SOURCE/path, value)
        for path, value in (context.get('valgrind_runtime') or {}).items():
            pin(path, value)
        pin('/bin/sh', context['shell'])
        pin('build/gnu-coreutils/lib/config.h', context['config_header'])
        pin('build/gnu-coreutils/src/getlimits', context['getlimits'])
        if script == terminal:
            selected, metadata = selector.selection(SOURCE, script)
            assert definition['shell_selection'] == metadata
            assert not definition['full_suite'] and definition['terminal']
            assert len(metadata['excluded_ranges']) == 1
            original = (SOURCE/script).read_bytes()
            interval, = metadata['excluded_ranges']
            a, b = interval['start_byte'], interval['end_byte_exclusive']
            assert original[:a] + original[b:] == selected
            assert hashlib.sha256(original[a:b]).hexdigest() == interval['sha256']
            pin('evidence/raw/gnu-tail-terminal-helper.py', context['shell_selection_driver'])
            pin(context['selected_script']['path'], metadata['selected_sha256'])
        else:
            profile = context['gdb_tail_profile']
            assert definition['full_suite'] and not definition['valgrind']
            pin('evidence/raw/gnu-tail-debug-ready-helper.py', profile['driver_sha256'])
            pin('/usr/bin/gdb', profile['gdb_sha256'])
            for impl, mapping in profile['sources'].items():
                pin(mapping['path'], mapping['sha256'])
                line = Path(mapping['path']).read_text().splitlines()[mapping['line']-1]
                assert (line.startswith('tail_forever_inotify') if impl == 'gnu'
                        else line == '    wd_to_name = hash_initialize(')
        for impl in ('gnu', 'rboxc'):
            outcome = result[impl]
            assert saved['outcomes'][impl]['result'] == outcome and outcome['status'] == 0
            assert outcome['binary_sha256'] == context['binaries'][impl]
            candidate = ('target/pr-page-cleanup-candidate/release/rboxc' if script == terminal
                         else 'target/coreutils-debug-candidate/release/rboxc')
            pin('build/gnu-coreutils/src/coreutils' if impl == 'gnu' else candidate,
                outcome['binary_sha256'])
            pin(outcome['watchdog']['path'], outcome['watchdog']['sha256'])
            for path, value in saved['outcomes'][impl]['logs'].items():
                pin(path, value)
            log = (ROOT/outcome['log']).read_text()
            assert re.search(r'^\+ Exit(?: 0)?$', log, re.M) and '+ __st=0\n' in log
            summary = {'status': outcome['status'], 'binary_sha256': outcome['binary_sha256']}
            if script == terminal:
                assert outcome['terminal_profile'] == {'private': True, 'controlling_stdin': True}
                pin('tests/gnu/terminal-profile.py', outcome['terminal_runner_sha256'])
                assert outcome['selected_runtime_script']['sha256'] == metadata['selected_sha256']
                assert log.count('+ returns_ 124 timeout 0.1 tail -f') == 3
                assert '+ tail -f - /dev/null' in log
            else:
                events = outcome['gdb_tail_profile']
                pin(events['log'], events['sha256'])
                assert read(events['log']) == events['events'] and len(events['events']) == 3
                mapping = profile['sources'][impl]
                for i, event in enumerate(events['events']):
                    assert event['implementation'] == impl
                    assert event['break_source'] == mapping['path'] and event['break_line'] == mapping['line']
                    orig = event['original_arguments']
                    if i == 0:
                        assert orig == event['effective_arguments'] == ['--version']
                        continue
                    assert orig[:3] == ['-nx', '--batch-silent', '--eval-command=break '+str(profile['gnu_line'])]
                    assert orig[-2:] == ['--eval-command=quit', 'tail']
                    expected = ['-iex=set disable-randomization off', '-iex=set auto-load off',
                                '-iex=set debuginfod enabled off', '-iex=set confirm off',
                                *orig[:2], '--eval-command=break '+mapping['path']+':'+str(mapping['line']), *orig[3:]]
                    assert event['effective_arguments'] == expected
                    assert re.fullmatch(r'--eval-command=run --pid=\d+ (-f file|-F file 2>tail.err)( >> tail.out| >>tail.out)?', orig[3])
                    assert len(orig) == (6 if i == 1 else 8)
                    if i == 2:
                        assert orig[4] == ('--eval-command=shell echo never-seen-with-tail-7.5 >> file'
                            if script == debug_scripts[0] else '--eval-command=shell mv file.new file')
                        assert orig[5] == '--eval-command=continue'
                assert '+ compare /dev/null gdb.out\n' in log
                assert '+ compare /dev/null tail.out\n' in log and '+ test -s tail.out\n' in log
                summary['debugger_invocations'] = 3
            if instrument:
                images = {}
                for memory in outcome['memory']:
                    path = ROOT/memory['log']; pin(path)
                    text = path.read_text()
                    parsed = runner.parse_memory_log(text, path.stem)
                    assert all(memory[k] == v for k, v in parsed.items())
                    command, = re.findall(r'^==\d+== Command: (.*)$', text, re.M)
                    assert command not in images
                    images[command] = parsed
                assert len(images) == 7
                for command in ['tail --version', 'tail -f -s.1 --max-unchanged-stats=1',
                                'tail ---disable-inotify -f -s.1 --max-unchanged-stats=1']:
                    m = images[command]
                    assert m['errors'] == m['non_inherited_descriptors'] == 0
                    assert not any(m['heap_bytes'].get(k, 0) for k in ['definitely lost','indirectly lost','possibly lost'])
                if impl == 'rboxc':
                    for command in ['tail -f', 'tail -f -', 'tail -f /dev/tty']:
                        assert images[command]['errors'] is None and images[command]['non_inherited_descriptors'] is None
                m = images['tail -f - /dev/null']
                assert m['errors'] == 2 and m['non_inherited_descriptors'] == 0
                assert m['heap_bytes'] == {'definitely lost':16432,'indirectly lost':0,'possibly lost':0,'still reachable':4386}
                summary['images'] = images
                summary['strict_valgrind_pass'] = False
            summaries.setdefault(script, {}).setdefault('valgrind' if instrument else 'native', {})[impl] = summary

# Retain the initial missing-timeout fixture skips; they add no passes.
initial = read('evidence/raw/gnu-tail-debug-original.json')
read('evidence/raw/gnu-tail-debug-original.progress.json')
for row in initial['results']:
    assert row['script'] in debug_scripts and not row['pass']
    for impl in ('gnu', 'rboxc'):
        assert row[impl]['status'] == 77
        pin(row[impl]['log'])
        assert 'timeout: not built' in (ROOT/row[impl]['log']).read_text()
candidate = 'target/coreutils-debug-candidate/release/rboxc'
repro = 'target/coreutils-debug-repro/release/rboxc'
assert pin(candidate) == pin(repro)
for path in ['build/coreutils-debug-build.log', 'build/coreutils-debug-repro-build.log',
             'evidence/raw/gnu-tail-debug-ready-driver.py', 'evidence/raw/gnu-tail-debug-ready-helper.py',
             'evidence/raw/gnu-tail-debug-initial-driver.py', 'evidence/raw/gnu-tail-debug-inventory.json',
             'evidence/raw/gnu-before-tail-debug-inventory.json', 'evidence/raw/gnu-before-gdb-tail-driver.py',
             'evidence/raw/gnu-before-tail-terminal-inventory.json', 'evidence/raw/gnu-tail-terminal-inventory.json',
             'evidence/gnu-tail-terminal-review.json']:
    pin(path)
base_name = 'evidence/gnu-remaining-shell-coverage.json'
base = read(base_name)
coverage = copy.deepcopy(base)
for row in coverage['results']:
    script = row['script']
    if script in debug_scripts:
        assert row['state'] == row['valgrind_state'] == 'pending'
        row.update(state='passed', execution_coverage='full-script',
                   evidence='evidence/raw/gnu-tail-debug-ready-original.json',
                   binary_sha256=pin(candidate), profile='optimized-debug-source-breakpoint')
    elif script == terminal:
        assert row['state'] == 'partial' and row['valgrind_state'] == 'passed-selection'
        row['previous_selection'] = copy.deepcopy(row)
        row.update(evidence='evidence/raw/gnu-tail-terminal-original.json',
                   valgrind_evidence='evidence/raw/gnu-tail-terminal-valgrind.json',
                   shell_selection=definitions[script]['shell_selection'], valgrind_state='open')
    else:
        continue
    row['audit'] = str(output.relative_to(ROOT))
assert all(a == b for a,b in zip(base['results'], coverage['results'])
           if a['script'] not in [terminal, *debug_scripts])
coverage['counts'] = dict(sorted(Counter(r['state'] for r in coverage['results']).items()))
coverage['valgrind_counts'] = dict(sorted(Counter(r['valgrind_state'] for r in coverage['results']).items()))
assert coverage['counts'] == {'excluded':29,'partial':32,'passed':646,'pending':21,'skipped':5}
coverage.update(base_report=base_name, base_report_sha256=pin(base_name),
    scope='733-script ledger: two full debug-profile passes and expanded ordinary terminal assertions. Terminal Valgrind remains open; narrower clean selection preserved. Profiles use distinct pinned binaries, not installed-release certification.')
ledger.write_text(json.dumps(coverage, indent=2)+'\n')
output.write_text(json.dumps({'scope': coverage['scope'], 'results': summaries, 'raw':raw,
    'debug_candidate': {'path':candidate,'sha256':pin(candidate),'bytes':(ROOT/candidate).stat().st_size,
        'reproduction':repro,'byte_identical':True, 'release_debug':2,'release_strip':'none',
        'toolchain':'nightly-2026-01-22'},
    'coverage':str(ledger.relative_to(ROOT)), 'coverage_sha256':pin(ledger),
    'accounting_pass':True,'full_suite_pass':False,'terminal_valgrind_pass':False},indent=2)+'\n')
print('PASS audit:', coverage['counts'], coverage['valgrind_counts'])
