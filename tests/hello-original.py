#!/usr/bin/env python3
"""Run all pinned GNU Hello originals, retaining the calendar prerequisite."""
# SPDX-License-Identifier: GPL-3.0-or-later
import importlib.util
import json
from pathlib import Path
import subprocess
import sys
import tempfile
from comparison_profile import ComparisonProfile, fingerprint

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT/'tests/gnu'))
spec = importlib.util.spec_from_file_location('reviewed_original', ROOT/'tests/gnu/reviewed-original.py')
runner = importlib.util.module_from_spec(spec)
spec.loader.exec_module(runner)
profile = ComparisonProfile('hello-original', oracle=ROOT/'build/gnu-hello/hello')
pin = json.loads((ROOT/'inventory/sources.json').read_text())['hello']
source = Path(pin['source'])
manifest = json.loads((ROOT/'inventory/hello-tests.json').read_text())
assert fingerprint(source/'Makefile.am') == manifest['registration_sha256']
date = ROOT/'build/gnu-coreutils/src/coreutils'
date_hash = fingerprint(date)
selections = [(row, 'ambient') for row in manifest['scripts']]
selections += [(next(r for r in manifest['scripts'] if r['script'] == 'tests/greeting-2'), 'fixed-calendar')]
results = []
for index, (row, calendar) in enumerate(selections):
    script = source/row['script']
    assert fingerprint(script) == row['source_sha256']
    outcomes = {}
    for implementation, binary in [('gnu', profile.oracle), ('rboxc', profile.binary)]:
        for instrument in (False, True):
            key = implementation + ('-valgrind' if instrument else '')
            with tempfile.TemporaryDirectory(prefix='rboxc-hello-original-') as directory:
                work = Path(directory)
                (work/'bin').mkdir()
                (work/'real').mkdir()
                (work/'real/hello').symlink_to(binary)
                memory = profile.logs/f'{index:02}-{key}'
                if instrument:
                    memory.mkdir()
                    wrapper = work/'bin/hello'
                    wrapper.write_text('#!/bin/sh\nPATH='+str(work/'real')+':$PATH\nexport PATH\n'
                        'exec /usr/bin/valgrind --leak-check=full '
                        '--show-leak-kinds=all --track-fds=yes --log-file='+str(memory/'%p.log')+
                        ' hello "$@"\n')
                    wrapper.chmod(0o755)
                else:
                    (work/'bin/hello').symlink_to(binary)
                if calendar == 'fixed-calendar':
                    # The original demonstration gates a greeting on lunar phase.
                    # Supply only its calendar input; leave the source unchanged.
                    wrapper = work/'bin/date'
                    wrapper.write_text('#!/bin/sh\n'
                        'test "$#" = 1 && test "$1" = "+%Y %j %H %M %S" || exit 99\n'
                        'exec '+str(date)+' --coreutils-prog=date --date=@948412800 "$1"\n')
                    wrapper.chmod(0o755)
                done = subprocess.run(['/bin/sh', str(script)], cwd=work,
                    stdin=subprocess.DEVNULL, capture_output=True, timeout=60,
                    env={'PATH': str(work/'bin')+':/opt/gnu/coreutils-9.11/bin:/usr/bin:/bin',
                         'HOME': directory, 'TMPDIR': directory, 'LC_ALL': 'C',
                         'LANGUAGE': 'C', 'TZ': 'UTC0', 'HELLO': 'hello', 'top_srcdir': str(source)})
                log = profile.logs/f'{index:02}-{key}.log'
                log.write_bytes(done.stdout+done.stderr)
                outcome = {'status': done.returncode, 'stdout': done.stdout.hex(),
                           'stderr': done.stderr.hex(), 'log': str(log.relative_to(ROOT)),
                           'log_sha256': fingerprint(log)}
                if instrument:
                    outcome['memory'] = [{**runner.parse_memory_log(p.read_text(), p.stem),
                        'log': str(p.relative_to(ROOT)), 'sha256': fingerprint(p)}
                        for p in sorted(memory.glob('*.log'))]
                outcomes[key] = outcome
    reference = outcomes['gnu']
    equivalent = all((r['status'], r['stdout'], r['stderr']) ==
        (reference['status'], reference['stdout'], reference['stderr']) for r in outcomes.values())
    skipped = equivalent and reference['status'] == 77
    logs = outcomes['rboxc-valgrind']['memory']
    clean = bool(logs) and all(m['errors'] == 0 and m['non_inherited_descriptors'] == 0
        and not any(m['heap_bytes'].get(k, 0) for k in
                    ('definitely lost', 'indirectly lost', 'possibly lost')) for m in logs)
    passed = equivalent and reference['status'] == 0 and clean
    results.append({**row, 'calendar_profile': calendar, 'pass': passed,
                    'skipped': skipped, 'equivalent': equivalent, 'memory_clean': clean,
                    'outcomes': outcomes})
    print('PASS' if passed else 'SKIP' if skipped else 'OPEN', row['script'], calendar, flush=True)
assert fingerprint(date) == date_hash
report = {'scope': 'All original scripts in native and Valgrind modes. Ambient calendar skips remain separate from the original long-greeting script with a declared fixed calendar input.',
          **profile.metadata(), 'gnu_binary': str(profile.oracle),
          'driver_sha256': fingerprint(Path(__file__)),
          'fixed_calendar': {'epoch': 948412800, 'date_command': str(date), 'date_command_sha256': date_hash},
          'passed': sum(r['pass'] for r in results), 'skipped': sum(r['skipped'] for r in results),
          'total': len(results), 'full_original_scripts_passed': len({r['script'] for r in results if r['pass']}),
          'registered_original_scripts': len(manifest['scripts']), 'results': results}
profile.report.write_text(json.dumps(report, indent=2)+'\n')
raise SystemExit(any(not (r['pass'] or r['skipped']) for r in results))
