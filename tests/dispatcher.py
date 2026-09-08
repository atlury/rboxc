#!/usr/bin/env python3
"""Compare GNU multicall alias and argument dispatch, including Valgrind."""
# SPDX-License-Identifier: GPL-3.0-or-later
import argparse
import sys
import hashlib
import json
import os
from pathlib import Path
import re
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[1]
from comparison_profile import ComparisonProfile
provider_commands = {name: data['commands'] for name, data in json.loads((ROOT/'inventory/sources.json').read_text()).items()
                     if name != 'coreutils' and isinstance(data, dict) and 'commands' in data}
parser = argparse.ArgumentParser(add_help=False)
parser.add_argument('--providers', nargs='*', choices=sorted(provider_commands))
parser.add_argument('--extra-commands', nargs='*', default=[],
                    choices=sorted({command for commands in provider_commands.values() for command in commands}),
                    help='Explicit commands from partially integrated providers, in addition to the selected full providers')
selection, remaining = parser.parse_known_args()
sys.argv = [sys.argv[0], *remaining]
expected_providers = selection.providers
if expected_providers is None:
    status = json.loads((ROOT/'evidence/status.json').read_text())
    expected_providers = [name for name, data in status['extra_providers'].items() if data['active_rust_entries']]
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
        for provider in expected_providers:
            names = sorted([*names, *provider_commands[provider]])
        assert not set(names) & set(selection.extra_commands), 'duplicate full-provider and partial-command selection'
        assert len(selection.extra_commands) == len(set(selection.extra_commands))
        names = sorted([*names, *selection.extra_commands])
        for alias in ('rboxc', 'rbox'):
            listed = execute([str(run/alias), '--list'])
            unknown = execute([str(run/alias), 'unknown-command'])
            passed = (listed['status'] == 0 and not listed['stderr']
                and bytes.fromhex(listed['stdout']).decode().splitlines() == names
                and unknown == {'status': 127, 'stdout': '', 'stderr': b'rboxc: unknown program\n'.hex()})
            results.append({'alias': alias, 'scope': 'rbox command selection', 'pass': passed})
    report = {**PROFILE.metadata(), 'expected_providers': expected_providers,
              'expected_extra_commands': selection.extra_commands,
              'driver_sha256': hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
              'passed': sum(r['pass'] for r in results), 'total': len(results), 'results': results}
    PROFILE.report.write_text(json.dumps(report, indent=2)+'\n')
    print(f"Dispatcher: {report['passed']}/{report['total']} pass")
    for row in results:
        if not row['pass']:
            print('OPEN', row)
    return report['passed'] != report['total']

if __name__ == '__main__':
    raise SystemExit(main())
