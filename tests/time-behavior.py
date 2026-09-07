#!/usr/bin/env python3
"""Compare GNU Time command control, output ownership, and timing formats."""
# SPDX-License-Identifier: GPL-3.0-or-later
import importlib.util
import json
from pathlib import Path
import re
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
profile = ComparisonProfile('time-behavior', oracle=ROOT/'build/gnu-time/src/time')
cases = [
    ('help', ['--help'], 0), ('version', ['--version'], 0),
    ('missing-command', [], 125), ('unknown-option', ['--unknown-option'], 125),
    ('missing-format', ['--format'], 125), ('relative-path-error', [], 125),
    ('absolute-path-error', [], 125),
    ('command-arguments', ['-f', '%C|%x', 'printf', '%s', 'one two'], 0),
    ('exit-status', ['-q', '-f', '%Tt|%Tx|%To', 'sh', '-c', 'exit 7'], 7),
    ('normal-status', ['-f', '%Tt|%Tx|%To|%Tn|%Ts', 'true'], 0),
    ('missing-executable', ['-q', '-f', '%x', 'missing-rboxc-time-command'], 127),
    ('non-executable', ['-q', '-f', '%x', './non-executable'], 126),
    ('output-file', ['-o', 'out', '-f', '%C|%x', 'true'], 0),
    ('output-append', ['-a', '-o', 'out', '-f', '%C|%x', 'true'], 0),
    ('output-directory', ['-o', '.', 'true'], 125),
    ('file-exec-failure', ['-o', 'out', '-q', '-f', '%x', 'missing-rboxc-time-command'], 127),
    ('environment-format', ['true'], 0),
    ('format-precedence', ['-f', '%x', 'true'], 0),
    ('escaped-format', ['-f', 'x\\ny\\tz\\\\%%', 'true'], 0),
    ('literal-format', ['-f', 'literal', 'true'], 0),
    ('full-small-output', ['-o', '/dev/full', '-f', 'literal', 'true'], 0),
    ('full-large-output', ['-o', '/dev/full', '-f', 'x'*5000, 'true'], 1),
    ('elapsed-sleep', ['-f', '%e', 'sleep', '0.05'], 0),
]
results = []
for index, (name, operands, expected_status) in enumerate(cases):
    outcomes = {}
    for implementation, binary in [('gnu', profile.oracle), ('rboxc', profile.binary)]:
        for instrument in (False, True):
            key = implementation + ('-valgrind' if instrument else '')
            logs = profile.logs/f'{index:02}-{key}'
            if instrument:
                logs.mkdir()
            with tempfile.TemporaryDirectory(prefix='rboxc-time-behavior-') as directory:
                work = Path(directory)
                (work/'time').symlink_to(binary)
                (work/'non-executable').write_text('fixture\n')
                (work/'out').write_text('existing\n')
                command = (['time', *operands] if implementation == 'gnu' else
                           [str(binary), 'time', *operands])
                if name == 'relative-path-error':
                    command = ['./time', *operands]
                if name == 'absolute-path-error':
                    command = [str(work/'time'), *operands]
                if instrument:
                    command = ['/usr/bin/valgrind', '--leak-check=full', '--show-leak-kinds=all',
                               '--track-fds=yes', '--log-file='+str(logs/'%p.log'), *command]
                environment = {'PATH': directory+':/usr/bin:/bin', 'LC_ALL': 'C', 'LANGUAGE': 'C', 'HOME': directory}
                if name in ('environment-format', 'format-precedence'):
                    environment['TIME'] = '%C'
                done = subprocess.run(command, cwd=directory, stdin=subprocess.DEVNULL,
                                      capture_output=True, timeout=30, env=environment)
                row = {'status': done.returncode, 'stdout': done.stdout.hex(), 'stderr': done.stderr.hex(),
                       'output_file': (work/'out').read_bytes().hex()}
                if name == 'absolute-path-error':
                    row.update(raw_stderr=row['stderr'], fixture=directory,
                               normalization='Replace only the private fixture directory with <fixture>.')
                    row['stderr'] = done.stderr.replace(directory.encode(), b'<fixture>').hex()
                if name == 'elapsed-sleep':
                    text = done.stderr.decode()
                    valid = re.fullmatch(r'\d+\.\d{2}\n', text) is not None and 0.04 <= float(text) < 5
                    row.update(elapsed_seconds=text.strip(), elapsed_valid=valid, raw_stderr=row['stderr'],
                               normalization='Validate the elapsed format and 0.04 <= seconds < 5 independently, then compare the remaining streams/status.')
                    row['stderr'] = b'<elapsed>\n'.hex()
                if instrument:
                    row['memory'] = observations(logs, runner.parse_memory_log, ROOT)
                outcomes[key] = row
    reference = outcomes['gnu']
    equivalent = reference['status'] == expected_status and all(
        (r['status'], r['stdout'], r['stderr'], r['output_file']) ==
        (reference['status'], reference['stdout'], reference['stderr'], reference['output_file'])
        and r.get('elapsed_valid', True) for r in outcomes.values())
    logs = outcomes['rboxc-valgrind']['memory']
    clean = clean_time_exits(logs)
    results.append({'name': name, 'arguments': operands, 'expected_status': expected_status,
                    'pass': equivalent and clean, 'equivalent': equivalent, 'memory_clean': clean, 'outcomes': outcomes})
    print('PASS' if equivalent and clean else 'OPEN', name, flush=True)
report = {'scope': 'Focused native/Valgrind comparison through rboxc time. Elapsed values are validated against declared format and bounds; all other compared streams and file contents are exact.',
          **profile.metadata(), 'gnu_binary': str(profile.oracle), 'driver_sha256': fingerprint(Path(__file__)),
          'memory_scope': 'Time parent exits and completed pre-exec child exits; header-only fork records remain unassessed exec boundaries.',
          'memory_parser_sha256': fingerprint(ROOT/'tests/time_memory.py'),
          'passed': sum(r['pass'] for r in results), 'total': len(results), 'results': results}
profile.report.write_text(json.dumps(report, indent=2)+'\n')
raise SystemExit(report['passed'] != report['total'])
