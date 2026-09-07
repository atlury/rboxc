#!/usr/bin/env python3
"""Compare Hello options, Unicode conversion, and ordinary output errors."""
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
profile = ComparisonProfile('hello-behavior', oracle=ROOT/'build/gnu-hello/hello')
cases = [('default', [], 'C', 'pipe', 0),
         ('traditional', ['--traditional'], 'C', 'pipe', 0),
         ('empty', ['--greeting='], 'C', 'pipe', 0),
         ('custom', ['-g', 'one two'], 'C', 'pipe', 0),
         ('last-option', ['-g', 'first', '-t', '-g', 'last'], 'C', 'pipe', 0),
         ('unicode', ['-g', 'héllo 日本'], 'C.utf8', 'pipe', 0),
         ('help', ['--help'], 'C', 'pipe', 0),
         ('version', ['--version'], 'C', 'pipe', 0),
         ('extra-operands', ['first', 'second'], 'C', 'pipe', 1),
         ('missing-greeting', ['--greeting'], 'C', 'pipe', 1),
         ('unknown-option', ['--unknown-option'], 'C', 'pipe', 1),
         ('invalid-utf8', ['-g', b'\xff'], 'C.utf8', 'pipe', 1),
         ('unicode-C-locale', ['-g', '日本'], 'C', 'pipe', 0),
         ('relative-path-error', ['first'], 'C', 'pipe', 1),
         ('absolute-path-error', ['first'], 'C', 'pipe', 1),
         ('full-output', [], 'C', 'full', 1),
         ('full-long-output', ['-g', 'x'*5000], 'C', 'full', 1)]
results = []
for index, (name, operands, locale, sink, expected_status) in enumerate(cases):
    outcomes = {}
    for implementation, binary in [('gnu', profile.oracle), ('rboxc', profile.binary)]:
        for instrument in (False, True):
            key = implementation + ('-valgrind' if instrument else '')
            command = (['./hello', *operands] if name == 'relative-path-error' else
                       ['hello', *operands] if implementation == 'gnu' else
                       [str(binary), 'hello', *operands])
            log = profile.logs/f'{index:02}-{key}.log'
            with tempfile.TemporaryDirectory(prefix='rboxc-hello-behavior-') as directory:
                (Path(directory)/'hello').symlink_to(binary)
                if name == 'absolute-path-error':
                    command = [str(Path(directory)/'hello'), *operands]
                if instrument:
                    command = ['/usr/bin/valgrind', '--leak-check=full', '--show-leak-kinds=all',
                               '--track-fds=yes', '--log-file='+str(log), *command]
                output = open('/dev/full', 'wb') if sink == 'full' else None
                try:
                    done = subprocess.run(command, cwd=directory, stdin=subprocess.DEVNULL,
                        stdout=output if output is not None else subprocess.PIPE,
                        stderr=subprocess.PIPE, timeout=30,
                        env={'PATH': directory+':/usr/bin:/bin', 'LC_ALL': locale, 'LANGUAGE': 'C', 'HOME': directory})
                finally:
                    if output is not None:
                        output.close()
                row = {'status': done.returncode, 'stdout': (done.stdout or b'').hex(),
                       'stderr': done.stderr.hex()}
                if name == 'absolute-path-error':
                    row.update(raw_stdout=row['stdout'], raw_stderr=row['stderr'],
                               fixture=directory, normalization='Replace only the private fixture directory with <fixture>.')
                    row['stdout'] = (done.stdout or b'').replace(directory.encode(), b'<fixture>').hex()
                    row['stderr'] = done.stderr.replace(directory.encode(), b'<fixture>').hex()
                if instrument:
                    row.update(memory=runner.parse_memory_log(log.read_text(), log.stem),
                               log=str(log.relative_to(ROOT)), log_sha256=fingerprint(log))
                outcomes[key] = row
    reference = outcomes['gnu']
    equivalent = reference['status'] == expected_status and all(
        (r['status'], r['stdout'], r['stderr']) == (reference['status'], reference['stdout'], reference['stderr'])
        for r in outcomes.values())
    memory = outcomes['rboxc-valgrind']['memory']
    clean = memory['errors'] == 0 and memory['non_inherited_descriptors'] == 0 and not any(
        memory['heap_bytes'].get(k, 0) for k in ('definitely lost', 'indirectly lost', 'possibly lost'))
    results.append({'name': name, 'arguments_hex': [(x if isinstance(x, bytes) else x.encode()).hex() for x in operands],
                    'locale': locale, 'sink': sink, 'expected_status': expected_status,
                    'pass': equivalent and clean, 'equivalent': equivalent, 'memory_clean': clean, 'outcomes': outcomes})
    print('PASS' if equivalent and clean else 'OPEN', name, flush=True)
report = {'scope': 'Focused native/Valgrind comparison through rboxc hello, including owned greeting storage on conversion and output errors.',
          **profile.metadata(), 'gnu_binary': str(profile.oracle), 'driver_sha256': fingerprint(Path(__file__)),
          'passed': sum(r['pass'] for r in results), 'total': len(results), 'results': results}
profile.report.write_text(json.dumps(report, indent=2)+'\n')
raise SystemExit(report['passed'] != report['total'])
