#!/usr/bin/env python3
"""Compare finite pr page ranges across shared columns and multiple inputs."""
# SPDX-License-Identifier: GPL-3.0-or-later
import importlib.util
import json
from pathlib import Path
import subprocess
import sys
from comparison_profile import ComparisonProfile

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT/'tests/gnu'))
PROFILE = ComparisonProfile('pr-page-cleanup')
spec = importlib.util.spec_from_file_location('reviewed', ROOT/'tests/gnu/reviewed-original.py')
runner = importlib.util.module_from_spec(spec)
spec.loader.exec_module(runner)
fixtures = {
    'a': ''.join(f'alpha {i}\n' for i in range(1, 31)),
    'b': ''.join(f'beta {i}\n' for i in range(1, 21)),
    'short': 'one\n',
    'ff': 'one\ntwo\fthree\nfour\ffive\nsix\n',
}
for name, content in fixtures.items(): (PROFILE.logs/name).write_text(content)
cases = [
    (['+1:1', '-t', '-l2', 'a'], b''),
    (['+2:2', '-t', '-l2', 'a'], b''),
    (['+1:1', '-t', '-l2', '-2', 'a'], b''),
    (['+1:1', '-t', '-l2', '-a', '-3', 'a'], b''),
    (['+1:1', '-t', '-l2', '-m', 'a', 'b'], b''),
    (['+1:1', '-t', '-l2', 'a', 'b'], b''),
    (['+1:1', '-t', '-l2', '-m', 'short', 'a'], b''),
    (['+20:20', '-t', '-l2', 'a'], b''),
    (['+1:1', '-t', '-l2', 'ff'], b''),
    (['+1:1', '-t', '-l2', '-m', '-', 'a'], b'stdin 1\nstdin 2\nstdin 3\n'),
]
alias = PROFILE.logs/'pr'; alias.symlink_to(PROFILE.binary)
env = {'PATH': '/usr/bin:/bin', 'LC_ALL': 'C', 'LANGUAGE': 'C'}


def execute(args, data, executable=None):
    p = subprocess.run(args, executable=executable, cwd=PROFILE.logs, env=env,
                       input=data, capture_output=True, timeout=30)
    return {'status': p.returncode, 'stdout': p.stdout.hex(), 'stderr': p.stderr.hex()}


results = []
for index, (args, data) in enumerate(cases):
    expected = execute(['pr', *args], data, PROFILE.oracle)
    actual = execute(['pr', *args], data, PROFILE.binary)
    log = PROFILE.logs/f'pages-{index}.log'
    instrumented = execute(['valgrind', '--leak-check=full', '--show-leak-kinds=all',
        '--track-fds=yes', '--log-file='+str(log), str(alias), *args], data)
    memory = runner.parse_memory_log(log.read_text(), str(index))
    clean = (memory['errors'] == memory['non_inherited_descriptors'] == 0 and
             not any(memory['heap_bytes'].get(k, 0) for k in
                     ('definitely lost', 'indirectly lost', 'possibly lost')))
    results.append({'arguments': args, 'stdin': data.hex(), 'gnu': expected, 'rboxc': actual,
                    'instrumented': instrumented, 'memory': memory,
                    'log': str(log.relative_to(ROOT)),
                    'pass': expected == actual == instrumented and clean})
report = {**PROFILE.metadata(), 'fixtures': fixtures, 'results': results,
          'passed': sum(r['pass'] for r in results), 'total': len(results)}
PROFILE.report.write_text(json.dumps(report, indent=2)+'\n')
print(f'pr page cleanup: {report["passed"]}/{report["total"]} pass')
raise SystemExit(report['passed'] != report['total'])
