#!/usr/bin/env python3
"""Preserve matching GNU/Rboxc numeric differences introduced by Valgrind."""
# SPDX-License-Identifier: GPL-3.0-or-later
import importlib.util
import json
from pathlib import Path
import subprocess
import sys
from comparison_profile import ComparisonProfile

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT/'tests/gnu'))
PROFILE = ComparisonProfile('numfmt-instrumentation')
spec = importlib.util.spec_from_file_location('reviewed', ROOT/'tests/gnu/reviewed-original.py')
runner = importlib.util.module_from_spec(spec)
spec.loader.exec_module(runner)
cases = [
    ('large-2a', ['915339622755539213'], b'915339622755539213\n'),
    ('large-10', ['--from=si', '--to=si', '999Q'], b'999Q\n'),
    ('large-14', ['--from=si', '--to=si', '999Q'], b'999Q\n'),
    ('large-15', ['9223372036854775808'], b'9223372036854775808\n'),
    ('large-16', ['9.300000000000000000'], b'9.300000000000000000\n'),
]
env = {'PATH': '/usr/bin:/bin', 'LC_ALL': 'C', 'LANGUAGE': 'C'}


def execute(args, executable=None):
    p = subprocess.run(args, executable=executable, env=env, capture_output=True, timeout=30)
    return {'status': p.returncode, 'stdout': p.stdout.hex(), 'stderr': p.stderr.hex()}


aliases = {}
for implementation, binary in (('gnu', PROFILE.oracle), ('rboxc', PROFILE.binary)):
    directory = PROFILE.logs/implementation; directory.mkdir()
    aliases[implementation] = directory/'numfmt'
    aliases[implementation].symlink_to(binary)
results = []
for name, args, expected in cases:
    row = {'case': name, 'arguments': args, 'expected_stdout': expected.hex()}
    for implementation, binary in (('gnu', PROFILE.oracle), ('rboxc', PROFILE.binary)):
        native = execute(['numfmt', *args], binary)
        log = PROFILE.logs/implementation/(name+'.log')
        instrumented = execute(['valgrind', '--leak-check=full', '--show-leak-kinds=all',
            '--track-fds=yes', '--log-file='+str(log), str(aliases[implementation]), *args])
        memory = runner.parse_memory_log(log.read_text(), name)
        row[implementation] = {'native': native, 'instrumented': instrumented, 'memory': memory,
                               'log': str(log.relative_to(ROOT))}
    assert row['gnu']['native'] == row['rboxc']['native'] == {
        'status': 0, 'stdout': expected.hex(), 'stderr': ''}
    assert row['gnu']['instrumented'] == row['rboxc']['instrumented']
    assert row['rboxc']['instrumented']['stdout'] != expected.hex()
    memory = row['rboxc']['memory']
    assert memory['errors'] == memory['non_inherited_descriptors'] == 0
    assert not any(memory['heap_bytes'].get(k, 0) for k in
                   ('definitely lost', 'indirectly lost', 'possibly lost'))
    results.append(row)
PROFILE.report.write_text(json.dumps({**PROFILE.metadata(),
    'scope': 'Five original numfmt numeric cases match their expected output normally. Both binaries produce identical differences only under Valgrind; original assertions remain failed under instrumentation.',
    'results': results, 'accounting_pass': True, 'strict_valgrind_assertions_pass': False}, indent=2)+'\n')
print('PASS: five matching instrumentation differences documented; strict assertions remain open')
