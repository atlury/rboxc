#!/usr/bin/env python3
"""Compare GNU preload-buffer behavior and verify candidate buffer cleanup."""
# SPDX-License-Identifier: GPL-3.0-or-later
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
candidate = Path(sys.argv[1]).resolve() if len(sys.argv) > 1 else ROOT/'target/release/libstdbuf.so'
libraries = {'gnu': ROOT/'build/gnu-coreutils/src/libstdbuf.so', 'rboxc': candidate}
client = ROOT/'build/stdbuf-lifetime'
subprocess.run(['cc', '-O2', '-Wall', '-Wextra', '-Werror',
                ROOT/'tests/stdbuf-lifetime.c', '-o', client], check=True)
results = []
for mode in ('empty', 'normal', 'close', 'replace', 'reopen', 'atexit'):
    for sizes in [('1', '1', '1'), ('1024', '1024', '1024'), ('7', '31', '17'), ('0', 'L', 'L')]:
        observations = {}
        for implementation, library in libraries.items():
            for instrument in (False, True):
                with tempfile.TemporaryDirectory(prefix='rboxc-stdbuf-lifetime-') as directory:
                    work = Path(directory)
                    (work/'input').write_bytes(b'input\n')
                    environment = {'PATH': '/usr/bin:/bin', 'LC_ALL': 'C', 'HOME': directory,
                                   'TMPDIR': directory, 'LD_PRELOAD': str(library),
                                   **dict(zip(('_STDBUF_I', '_STDBUF_O', '_STDBUF_E'), sizes))}
                    arguments = [str(client), mode]
                    log = ROOT/'evidence/raw'/f'stdbuf-lifetime-{mode}-{sizes[0]}-{implementation}.log'
                    if instrument:
                        arguments = ['valgrind', '--leak-check=full', '--show-leak-kinds=all',
                                     '--track-fds=yes', '--log-file='+str(log), *arguments]
                    process = subprocess.run(arguments, input=b'input\n', capture_output=True,
                                             cwd=work, env=environment, timeout=30)
                    observation = {'status': process.returncode,
                                   'stdout': process.stdout.decode(), 'stderr': process.stderr.decode(),
                                   'files': {p.name: p.read_text() for p in sorted(work.iterdir()) if p.is_file()}}
                    if instrument:
                        observation['memory'] = runner.parse_memory_log(log.read_text(), log.stem)
                        observation['log'] = str(log.relative_to(ROOT))
                    observations[implementation+('-valgrind' if instrument else '')] = observation
        reference = observations['gnu']
        equal = all(all(row[key] == reference[key] for key in ('status', 'stdout', 'stderr', 'files'))
                    for row in observations.values())
        memory = observations['rboxc-valgrind']['memory']
        clean = (memory['errors'] == 0 and memory['non_inherited_descriptors'] == 0
                 and all(memory['heap_bytes'].get(kind, 0) == 0 for kind in
                         ('definitely lost', 'indirectly lost', 'possibly lost')))
        passed = equal and clean and reference['status'] == 0
        results.append({'mode': mode, 'sizes': sizes, 'pass': passed, **observations})
        print('PASS' if passed else 'OPEN', mode, sizes, flush=True)
report = {'scope': 'Bounded GNU libstdbuf behavior and stream ownership on this GNU libc profile',
          'libc': subprocess.check_output(['getconf', 'GNU_LIBC_VERSION'], text=True).strip(),
          'libraries': {name: {'path': str(path), 'sha256': hashlib.sha256(path.read_bytes()).hexdigest()}
                        for name, path in libraries.items()},
          'client_sha256': hashlib.sha256(client.read_bytes()).hexdigest(),
          'client_source_sha256': hashlib.sha256((ROOT/'tests/stdbuf-lifetime.c').read_bytes()).hexdigest(),
          'passed': sum(row['pass'] for row in results), 'total': len(results), 'results': results}
(ROOT/'evidence/stdbuf-lifetime.json').write_text(json.dumps(report, indent=2)+'\n')
raise SystemExit(report['passed'] != report['total'])
