#!/usr/bin/env python3
"""Check ownership of completed expr values on success and trailing arguments."""
# SPDX-License-Identifier: GPL-3.0-or-later
import importlib.util
import json
from pathlib import Path
import subprocess
import sys

from comparison_profile import ComparisonProfile

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT/'tests/gnu'))
PROFILE = ComparisonProfile('expr-value-cleanup')
spec = importlib.util.spec_from_file_location('reviewed', ROOT/'tests/gnu/reviewed-original.py')
runner = importlib.util.module_from_spec(spec)
spec.loader.exec_module(runner)
cases = [
    ['9', '9'], ['2', 'a'], ['2', '+', '3', 'extra'],
    ['123456789012345678901234567890', '*', '98765432109876543210', 'extra'],
    ['abc', ':', r'\(a\)', 'extra'],
    ['2', '+', '3'], ['abc', ':', r'\(a\)'], ['0'],
]
env = {'PATH': '/usr/bin:/bin', 'LC_ALL': 'C', 'LANGUAGE': 'C'}


def execute(args, executable=None):
    p = subprocess.run(args, executable=executable, env=env, capture_output=True, timeout=30)
    return {'status': p.returncode, 'stdout': p.stdout.hex(), 'stderr': p.stderr.hex()}


results = []
alias = PROFILE.logs/'expr'
alias.symlink_to(PROFILE.binary)
for index, args in enumerate(cases):
    expected = execute(['expr', *args], PROFILE.oracle)
    actual = execute(['expr', *args], PROFILE.binary)
    log = PROFILE.logs/f'value-{index}.log'
    instrumented = execute(['valgrind', '--leak-check=full', '--show-leak-kinds=all',
        '--track-fds=yes', '--log-file='+str(log), str(alias), *args])
    # Both process images retain the GNU diagnostic basename "expr".
    memory = runner.parse_memory_log(log.read_text(), str(index))
    clean = (memory['errors'] == memory['non_inherited_descriptors'] == 0 and
             not any(memory['heap_bytes'].get(k, 0) for k in
                     ('definitely lost', 'indirectly lost', 'possibly lost')))
    results.append({'arguments': args, 'gnu': expected, 'rboxc': actual,
                    'instrumented': instrumented, 'memory': memory,
                    'log': str(log.relative_to(ROOT)),
                    'pass': expected == actual == instrumented and clean})
report = {**PROFILE.metadata(), 'results': results,
          'passed': sum(r['pass'] for r in results), 'total': len(results)}
PROFILE.report.write_text(json.dumps(report, indent=2)+'\n')
print(f'expr value cleanup: {report["passed"]}/{report["total"]} pass')
raise SystemExit(report['passed'] != report['total'])
