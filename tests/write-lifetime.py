#!/usr/bin/env python3
"""Compare bounded cat/tac output and fatal-write resource ownership with GNU."""
# SPDX-License-Identifier: GPL-3.0-or-later
import argparse
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT/'tests/gnu'))
spec = importlib.util.spec_from_file_location('reviewed_original', ROOT/'tests/gnu/reviewed-original.py')
runner = importlib.util.module_from_spec(spec)
spec.loader.exec_module(runner)
parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('--candidate', type=Path, default=ROOT/'target/release/rboxc')
parser.add_argument('--report-name', default='write-lifetime')
options = parser.parse_args()
assert options.report_name.replace('-', '').replace('_', '').isalnum()
binaries = {'gnu': ROOT/'build/gnu-coreutils/src/coreutils', 'rboxc': options.candidate.resolve(strict=True)}
raw = ROOT/'evidence/raw'/options.report_name
raw.mkdir(exist_ok=True)
cases = []
for command, args in [('cat', []), ('cat', ['-n']), ('cat', ['-A']),
                      ('tac', []), ('tac', ['-s', '::']), ('tac', ['-r', '-s', '[\n:]'])]:
    for source in ('named', 'pipe'):
        for sink in ('capture', 'full'):
            cases.append((command, args, source, sink))
# Long, bounded records grow tac's sentinel-offset buffer. Multiple operands
# exercise normal close followed by descriptor reuse and a later fatal write.
for command in ('cat', 'tac'):
    cases += [(command, [], 'long', sink) for sink in ('capture', 'full')]
    cases += [(command, [], 'multiple', sink) for sink in ('capture', 'full')]
    cases += [(command, [], 'named', 'closed-pipe')]
results = []
for index, (command, flags, source, sink) in enumerate(cases):
    observations = {}
    data = b'alpha\t\x01\n\nbeta::gamma\n' * 4096
    if source == 'long':
        data = b'x' * 196608 + b'\ny\n'
    for implementation, binary in binaries.items():
        for instrument in (False, True):
            key = implementation + ('-valgrind' if instrument else '')
            with tempfile.TemporaryDirectory(prefix='rboxc-write-lifetime-') as directory:
                work = Path(directory)
                (work/'input').write_bytes(data)
                (work/'empty').write_bytes(b'')
                operands = [] if source == 'pipe' else ['input']
                if source == 'multiple':
                    operands = ['empty', 'input', 'empty', 'input']
                arguments = [str(binary), '--coreutils-prog='+command if implementation == 'gnu' else command, *flags, *operands]
                log = raw/f'{index:02}-{key}.log'
                if instrument:
                    arguments = ['/usr/bin/valgrind', '--leak-check=full', '--show-leak-kinds=all',
                                 '--track-fds=yes', '--log-file='+str(log), *arguments]
                environment = {'PATH': '/usr/bin:/bin', 'LC_ALL': 'C', 'HOME': directory, 'TMPDIR': directory}
                output = subprocess.PIPE
                handle = None
                if sink == 'full':
                    handle = open('/dev/full', 'wb')
                    output = handle
                elif sink == 'closed-pipe':
                    read_fd, write_fd = os.pipe()
                    os.close(read_fd)
                    handle = os.fdopen(write_fd, 'wb')
                    output = handle
                try:
                    process = subprocess.run(arguments, input=data if source == 'pipe' else b'',
                                             stdout=output, stderr=subprocess.PIPE, cwd=work,
                                             env=environment, timeout=60)
                finally:
                    if handle:
                        handle.close()
                stdout = process.stdout or b''
                row = {'status': process.returncode, 'stdout_bytes': len(stdout),
                       'stdout_sha256': hashlib.sha256(stdout).hexdigest(),
                       'stderr': process.stderr.decode(errors='replace')}
                if instrument:
                    row['memory'] = runner.parse_memory_log(log.read_text(), log.stem)
                    row['log'] = str(log.relative_to(ROOT))
                    row['log_sha256'] = hashlib.sha256(log.read_bytes()).hexdigest()
                observations[key] = row
    reference = observations['gnu']
    equal = all(all(row[field] == reference[field] for field in
                    ('status', 'stdout_bytes', 'stdout_sha256', 'stderr')) for row in observations.values())
    memory = observations['rboxc-valgrind']['memory']
    clean = (memory['errors'] == 0 and memory['non_inherited_descriptors'] == 0
             and all(memory['heap_bytes'].get(kind, 0) == 0 for kind in
                     ('definitely lost', 'indirectly lost', 'possibly lost')))
    expected_status = {'capture': 0, 'full': 1, 'closed-pipe': -13}[sink]
    # SIGPIPE exits bypass atexit in GNU and rboxc. Retain the findings and
    # check signal equivalence separately; never label these memory-clean.
    passed = equal and reference['status'] == expected_status and (clean or sink == 'closed-pipe')
    results.append({'command': command, 'flags': flags, 'source': source, 'sink': sink,
                    'pass': passed, 'equivalent': equal, 'memory_clean': clean,
                    'assessment': 'signal-equivalence' if sink == 'closed-pipe' else 'ownership',
                    **observations})
    print('PASS' if passed else 'OPEN', command, flags, source, sink, flush=True)
report = {'scope': 'Bounded native/Valgrind GNU comparisons; SIGPIPE checks preserve signal behavior and retain memory findings',
          'binaries': {name: {'path': str(path), 'sha256': hashlib.sha256(path.read_bytes()).hexdigest()}
                       for name, path in binaries.items()},
          'test_sha256': hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
          'passed': sum(row['pass'] for row in results), 'total': len(results), 'results': results}
(ROOT/'evidence'/f'{options.report_name}.json').write_text(json.dumps(report, indent=2)+'\n')
raise SystemExit(report['passed'] != report['total'])
