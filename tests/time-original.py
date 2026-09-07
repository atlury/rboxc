#!/usr/bin/env python3
"""Run reviewed GNU Time originals with unchanged assertions and Valgrind."""
# SPDX-License-Identifier: GPL-3.0-or-later
import importlib.util
import json
from pathlib import Path
import subprocess
import sys
import tempfile
from comparison_profile import ComparisonProfile, fingerprint
from time_memory import observations, clean_time_exits

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT/'tests/gnu'))
spec = importlib.util.spec_from_file_location('reviewed_original', ROOT/'tests/gnu/reviewed-original.py')
runner = importlib.util.module_from_spec(spec)
spec.loader.exec_module(runner)
profile = ComparisonProfile('time-original', oracle=ROOT/'build/gnu-time/src/time')
pin = json.loads((ROOT/'inventory/sources.json').read_text())['time']
source = Path(pin['source'])
manifest = json.loads((ROOT/'inventory/time-tests.json').read_text())
assert fingerprint(source/manifest['registration']['path']) == manifest['registration']['sha256']
aux = ROOT/'build/gnu-time/tests/time-aux'
aux_hash = fingerprint(aux)
results = []
for index, row in enumerate(manifest['scripts']):
    script = source/row['script']
    assert fingerprint(script) == row['source_sha256']
    if not row['reviewed']:
        continue
    outcomes = {}
    for implementation, binary in [('gnu', profile.oracle), ('rboxc', profile.binary)]:
        for instrument in (False, True):
            key = implementation + ('-valgrind' if instrument else '')
            with tempfile.TemporaryDirectory(prefix='rboxc-time-original-') as directory:
                work = Path(directory)
                (work/'src').mkdir()
                (work/'real').mkdir()
                (work/'real/time').symlink_to(binary)
                (work/'src/time-aux').symlink_to(aux)
                memory = profile.logs/f'{index:02}-{key}'
                if instrument:
                    memory.mkdir()
                    wrapper = work/'src/time'
                    wrapper.write_text('#!/bin/sh\nPATH='+str(work/'real')+':$PATH\nexport PATH\n'
                        'exec /usr/bin/valgrind --leak-check=full '
                        '--show-leak-kinds=all --track-fds=yes --log-file='+str(memory/'%p.log')+
                        ' time "$@"\n')
                    wrapper.chmod(0o755)
                else:
                    (work/'src/time').symlink_to(binary)
                done = subprocess.run(['/bin/bash', str(script)], cwd=work,
                    stdin=subprocess.DEVNULL, capture_output=True, timeout=120,
                    env={'PATH': str(work/'src')+':/usr/bin:/bin', 'HOME': directory,
                         'TMPDIR': directory, 'LC_ALL': 'C', 'LANGUAGE': 'C', 'TZ': 'UTC0',
                         'srcdir': str(source), 'top_srcdir': str(source), 'VERSION': pin['version']})
                log = profile.logs/f'{index:02}-{key}.log'
                log.write_bytes(done.stdout+done.stderr)
                outcome = {'status': done.returncode, 'stdout': done.stdout.hex(),
                           'stderr': done.stderr.hex(), 'log': str(log.relative_to(ROOT)),
                           'log_sha256': fingerprint(log)}
                if instrument:
                    outcome['memory'] = observations(memory, runner.parse_memory_log, ROOT)
                outcomes[key] = outcome
    native_pass = outcomes['gnu']['status'] == outcomes['rboxc']['status'] == 0
    assertions_pass = native_pass and all(r['status'] == 0 for r in outcomes.values())
    logs = outcomes['rboxc-valgrind']['memory']
    clean = clean_time_exits(logs)
    passed = assertions_pass and clean
    results.append({**row, 'pass': passed, 'native_pass': native_pass,
                    'assertions_pass': assertions_pass, 'memory_clean': clean, 'outcomes': outcomes})
    print('PASS' if passed else 'OPEN', row['script'], flush=True)
assert fingerprint(aux) == aux_hash
report = {'scope': 'Reviewed original scripts with unchanged assertions in native and Valgrind modes. Timing/resource measurements vary; script exit status evaluates the original assertions. Child programs execute natively after exec.',
          **profile.metadata(), 'gnu_binary': str(profile.oracle),
          'memory_scope': 'Time parent exits and completed pre-exec child exits. Header-only fork records are retained as unassessed exec boundaries; child executable memory is outside this assessment.',
          'memory_parser_sha256': fingerprint(ROOT/'tests/time_memory.py'),
          'driver_sha256': fingerprint(Path(__file__)), 'test_helper_sha256': aux_hash,
          'passed': sum(r['pass'] for r in results), 'native_passed': sum(r['native_pass'] for r in results),
          'assertions_passed': sum(r['assertions_pass'] for r in results),
          'total': len(results), 'registered_original_scripts': len(manifest['scripts']),
          'excluded': [r for r in manifest['scripts'] if not r['reviewed']], 'results': results}
profile.report.write_text(json.dumps(report, indent=2)+'\n')
raise SystemExit(report['passed'] != report['total'])
