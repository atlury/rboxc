#!/usr/bin/env python3
"""Run reviewed, unchanged GNU Less screen replays with isolated build profiles."""
# SPDX-License-Identifier: GPL-3.0-or-later
from concurrent.futures import ThreadPoolExecutor
import importlib.util
import json
import os
from pathlib import Path
import re
import shlex
import subprocess
import sys
import tempfile
from comparison_profile import ComparisonProfile, fingerprint
from less_fixture import describe

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT/'tests/gnu'))
spec = importlib.util.spec_from_file_location('reviewed', ROOT/'tests/gnu/reviewed-original.py')
runner = importlib.util.module_from_spec(spec)
spec.loader.exec_module(runner)
build_path = ROOT/os.environ.get('RBOXC_LESS_BUILD_REPORT', 'evidence/less-original-fixtures-build.json')
build = json.loads(build_path.read_text())
profile = ComparisonProfile('less-original', oracle=build['oracle'], selections=True)
assert str(profile.binary) == build['binary'] and profile.binary_sha256 == build['binary_sha256']
manifest = json.loads((ROOT/'inventory/less-tests.json').read_text())
source = Path(json.loads((ROOT/'inventory/sources.json').read_text())['less']['source'])
selected = [r for r in manifest['replays'] if r['reviewed']]
requested = set(profile.options.commands)
assert requested <= {r['name'] for r in selected}
if requested:
    selected = [r for r in selected if r['name'] in requested]
tool_dir = Path(build['oracle']).parent/'lesstest'
inputs = {Path(p): h for p, h in build['test_tools'].items()}
inputs.update({p: fingerprint(p) for p in [build_path, Path(__file__), ROOT/'tests/less_fixture.py',
              source/'lesstest/Makefile', source/'Makefile.in', Path('/usr/bin/perl'), Path('/bin/sh').resolve()]})
for row in selected:
    inputs[source/row['path']] = row['sha256']
    assert describe((source/row['path']).read_bytes()) == row['records']
assert all(fingerprint(p) == h for p, h in inputs.items())

def run(row):
    outcomes = {}
    for implementation, binary in [('gnu', profile.oracle), ('rboxc', profile.binary)]:
        for instrument in [False, True]:
            key = implementation+('-valgrind' if instrument else '')
            saved = profile.logs/(row['name']+'-'+key)
            saved.mkdir()
            (saved/'memory').mkdir()
            with tempfile.TemporaryDirectory(prefix='rboxc-less-replay-') as directory:
                work = Path(directory)
                wrapper = work/'less'
                argv = [str(binary)]
                if instrument:
                    argv = ['/usr/bin/valgrind', '--leak-check=full', '--show-leak-kinds=all',
                            '--track-fds=yes', '--trace-children=yes',
                            '--log-file='+str(saved/'memory/%p.log'), *argv]
                overlay = row.get('environment_overlay', {})
                assert not overlay or overlay == {'LESS_OSC8_OPEN_ANY': '', 'LESS_OSC8_OPEN_man': ''}
                environment_setup = ''.join('export '+n+'='+shlex.quote(v)+'\n' for n, v in overlay.items())
                wrapper.write_text('#!/bin/sh\n'+environment_setup+'exec '+shlex.join(argv)+' "$@"\n')
                wrapper.chmod(0o755)
                command = ['/usr/bin/perl', str(tool_dir/'runtest'), '-d', str(tool_dir),
                           '-l', str(wrapper), '-r', str(work/'run'), '-Od', str(source/row['path'])]
                done = subprocess.run(command, cwd=work, stdin=subprocess.DEVNULL,
                                      capture_output=True, timeout=120,
                                      env={'PATH': '/usr/bin:/bin', 'HOME': directory, 'TMPDIR': directory,
                                           'LC_ALL': 'C', 'LANGUAGE': 'C', 'TZ': 'UTC0'})
            for stream, contents in [('stdout', done.stdout), ('stderr', done.stderr)]:
                (saved/stream).write_bytes(contents)
            expected = f"PASS: {row['records']['files'][0]['name']} ({len(row['records']['keys'])} steps)\n".encode()
            assertions = done.returncode == 0 and done.stdout == expected and b'RAN  1 tests with 0 errors\n' in done.stderr
            memory = []
            for path in sorted((saved/'memory').glob('*.log')):
                text = path.read_text()
                commands = re.findall(r'^==[0-9]+== Command: (.*)$', text, re.M)
                assert len(commands) == 1 and commands[0].split()[0] == str(binary), 'unclassified child'
                memory.append({'log': str(path.relative_to(ROOT)), 'sha256': fingerprint(path),
                               'command': commands[0], **runner.parse_memory_log(text, path.stem, exec_only=True)})
            assert len(memory) == (2 if instrument else 0), 'original version probe plus replay'
            clean = bool(memory) and all(m['complete_exec_log'] and m['errors'] == 0 and
                m['non_inherited_descriptors'] == 0 and not any(m['heap_bytes'].get(k, 0)
                for k in ('definitely lost', 'indirectly lost', 'possibly lost')) for m in memory)
            outcomes[key] = {'status': done.returncode, 'assertions_pass': assertions,
                'stdout': done.stdout.hex(), 'stderr': done.stderr.hex(),
                'streams': {n: {'path': str((saved/n).relative_to(ROOT)), 'sha256': fingerprint(saved/n)}
                            for n in ('stdout', 'stderr')}, 'memory': memory, 'memory_clean': clean if instrument else None}
    return {'name': row['name'], 'source_sha256': row['sha256'], 'frames': row['records']['frames'],
            'environment_overlay': row.get('environment_overlay', {}),
            'assertions_pass': all(o['assertions_pass'] for o in outcomes.values()),
            'pass': all(o['assertions_pass'] for o in outcomes.values()) and outcomes['rboxc-valgrind']['memory_clean'],
            'outcomes': outcomes}

results = []
with ThreadPoolExecutor(max_workers=4) as pool:
    for result in pool.map(run, selected):
        results.append(result)
        assert all(fingerprint(p) == h for p, h in inputs.items())
        report = {**profile.metadata(), 'scope': 'Unchanged reviewed GNU screen-replay files and original runtest/lesstest assertions. '
                  'The standalone Rust entry uses GNU test-mode helpers; the release configuration remains separate. '
                  'Only Less version-probe and replay processes are instrumented, not the native test tools.',
                  'build_report': str(build_path.relative_to(ROOT)), 'build_report_sha256': fingerprint(build_path),
                  'inputs': {str(p): h for p, h in inputs.items()}, 'driver_sha256': fingerprint(Path(__file__)),
                  'selected_targets': sorted(requested), 'planned_total': len(selected),
                  'complete': len(results) == len(selected), 'passed': sum(r['pass'] for r in results),
                  'assertion_passes': sum(r['assertions_pass'] for r in results),
                  'total': len(results), 'results': results}
        profile.report.write_text(json.dumps(report, indent=2)+'\n')
        print('PASS' if result['pass'] else 'OPEN', result['name'], flush=True)
raise SystemExit(report['passed'] != report['total'])
