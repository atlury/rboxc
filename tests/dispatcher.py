#!/usr/bin/env python3
"""Compare GNU multicall alias and argument dispatch, including Valgrind."""
# SPDX-License-Identifier: GPL-3.0-or-later
import hashlib
import json
import os
from pathlib import Path
import re
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[1]
from comparison_profile import ComparisonProfile
PROFILE = ComparisonProfile('dispatcher')
BINARY = PROFILE.binary
GNU = ROOT/'build/gnu-coreutils/src/coreutils'
CASES = [
    ('coreutils', ['--version']),
    ('coreutils', ['--coreutils-prog=printf', '%s', 'one two']),
    ('coreutils', ['--coreutils-prog-shebang=printf', 'script-name', '%s', 'one two']),
    ('ginstall', ['--version']),
    ('blah', []),
    ('blah', ['--version']),
    ('blah', ['--coreutils-prog=printf', '%s', 'one two']),
    ('prefix-coreutils', ['--version']),
    ('coreutils', ['--coreutils-prog=unknown-command']),
]

def main():
    results = []
    with tempfile.TemporaryDirectory(prefix='rboxc-dispatch-') as directory:
        run = Path(directory)
        environment = {'PATH': '/usr/bin:/bin', 'HOME': str(run), 'LC_ALL': 'C', 'LANGUAGE': 'C'}
        for alias in {name for name, _ in CASES} | {'rboxc', 'rbox'}:
            (run/alias).symlink_to(BINARY)
        def execute(args, executable=None):
            p = subprocess.run(args, executable=executable, cwd=run, env=environment,
                               capture_output=True, timeout=30)
            return {'status': p.returncode, 'stdout': p.stdout.hex(), 'stderr': p.stderr.hex()}
        for index, (alias, arguments) in enumerate(CASES):
            # Both implementations see identical argv[0], preserving diagnostic names.
            args = [str(run/alias), *arguments]
            expected = execute(args, GNU)
            actual = execute(args)
            log = PROFILE.logs/f'dispatch-valgrind-{index}.log'
            instrumented = execute(['valgrind', '--leak-check=full', '--show-leak-kinds=all',
                '--track-fds=yes', '--log-file='+str(log), *args])
            report = log.read_text()
            errors = re.search(r'ERROR SUMMARY: ([\d,]+) errors', report)
            fds = re.search(r'FILE DESCRIPTORS: (\d+) open \((\d+) (?:inherited|std)\)', report)
            clean = errors is not None and errors[1] == '0' and fds is not None and fds[1] == fds[2]
            results.append({'alias': alias, 'arguments': arguments,
                'gnu': expected, 'rboxc': actual, 'valgrind': instrumented,
                'memory_clean': clean, 'log': str(log.relative_to(ROOT)),
                'pass': expected == actual == instrumented and clean})
        names = sorted(r['name'] for r in json.loads((ROOT/'evidence/translation.json').read_text()))
        for alias in ('rboxc', 'rbox'):
            listed = execute([str(run/alias), '--list'])
            unknown = execute([str(run/alias), 'unknown-command'])
            passed = (listed['status'] == 0 and not listed['stderr']
                and bytes.fromhex(listed['stdout']).decode().splitlines() == names
                and unknown == {'status': 127, 'stdout': '', 'stderr': b'rboxc: unknown program\n'.hex()})
            results.append({'alias': alias, 'scope': 'rbox command selection', 'pass': passed})
    report = {**PROFILE.metadata(),
              'passed': sum(r['pass'] for r in results), 'total': len(results), 'results': results}
    PROFILE.report.write_text(json.dumps(report, indent=2)+'\n')
    print(f"Dispatcher: {report['passed']}/{report['total']} pass")
    for row in results:
        if not row['pass']:
            print('OPEN', row)
    return report['passed'] != report['total']

if __name__ == '__main__':
    raise SystemExit(main())
