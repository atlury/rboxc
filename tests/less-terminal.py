#!/usr/bin/env python3
"""Check production-mode Less on a private terminal, including mode restoration."""
# SPDX-License-Identifier: GPL-3.0-or-later
import errno
import fcntl
import importlib.util
import json
import os
from pathlib import Path
import pty
import re
import select
import subprocess
import sys
import tempfile
import termios
import time
from comparison_profile import ComparisonProfile, fingerprint

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT/'tests/gnu'))
spec = importlib.util.spec_from_file_location('reviewed', ROOT/'tests/gnu/reviewed-original.py')
runner = importlib.util.module_from_spec(spec)
spec.loader.exec_module(runner)
profile = ComparisonProfile('less-terminal', oracle=ROOT/'build/gnu-less/less')
fixture = b''.join(f'ordinary terminal line {n}\n'.encode() for n in range(1, 81))
outcomes = {}
for implementation, binary in [('gnu', profile.oracle), ('rboxc', profile.binary)]:
    for instrument in [False, True]:
        key = implementation+('-valgrind' if instrument else '')
        saved = profile.logs/key
        saved.mkdir()
        with tempfile.TemporaryDirectory(prefix='rboxc-less-terminal-') as directory:
            work = Path(directory)
            (work/'less').symlink_to(binary)
            (work/'input').write_bytes(fixture)
            master, slave = pty.openpty()
            fcntl.ioctl(slave, termios.TIOCSWINSZ, __import__('struct').pack('HHHH', 24, 80, 0, 0))
            initial = termios.tcgetattr(slave)
            def terminal_child():
                os.setsid()
                fcntl.ioctl(0, termios.TIOCSCTTY, 0)
            log = saved/'memory.log'
            argv = [str(work/'less'), 'input']
            if instrument:
                argv = ['/usr/bin/valgrind', '--leak-check=full', '--show-leak-kinds=all',
                        '--track-fds=yes', '--trace-children=yes', '--log-file='+str(log), *argv]
            process = subprocess.Popen(argv, cwd=work, stdin=slave, stdout=slave, stderr=slave,
                close_fds=True, preexec_fn=terminal_child,
                env={'PATH': '/usr/bin:/bin', 'HOME': directory, 'TERM': 'xterm',
                     'LC_ALL': 'C', 'LANGUAGE': 'C', 'LESSHISTFILE': '-', 'TZ': 'UTC0'})
            output = bytearray()
            quit_sent = False
            deadline = time.monotonic()+30
            try:
                while process.poll() is None:
                    assert time.monotonic() < deadline, 'terminal replay deadline'
                    if select.select([master], [], [], 0.1)[0]:
                        output.extend(os.read(master, 65536))
                    if not quit_sent and b'ordinary terminal line 23' in output and b'input' in output:
                        os.write(master, b'q')
                        quit_sent = True
                while select.select([master], [], [], 0)[0]:
                    output.extend(os.read(master, 65536))
                status = process.wait()
                restored = termios.tcgetattr(slave) == initial
            finally:
                if process.poll() is None:
                    os.killpg(process.pid, 9)
                    process.wait()
                os.close(master)
                os.close(slave)
            assert status == 0 and quit_sent and restored
            (saved/'terminal-output').write_bytes(output)
            memory = None
            clean = None
            if instrument:
                text = log.read_text()
                pids = set(re.findall(r'^==([0-9]+)==', text, re.M))
                assert len(pids) == 1
                memory = runner.parse_memory_log(text, pids.pop(), exec_only=True)
                clean = memory['complete_exec_log'] and memory['errors'] == 0 and memory['non_inherited_descriptors'] == 0 and not any(
                    memory['heap_bytes'].get(k, 0) for k in ('definitely lost', 'indirectly lost', 'possibly lost'))
            outcomes[key] = {'status': status, 'quit_sent': quit_sent, 'terminal_restored': restored,
                'output': bytes(output).hex(), 'output_path': str((saved/'terminal-output').relative_to(ROOT)),
                'output_sha256': fingerprint(saved/'terminal-output'), 'memory': memory, 'memory_clean': clean,
                'memory_log': str(log.relative_to(ROOT)) if instrument else None,
                'memory_log_sha256': fingerprint(log) if instrument else None}
reference = outcomes['gnu']['output']
equivalent = all(o['output'] == reference for o in outcomes.values())
passed = equivalent and outcomes['rboxc-valgrind']['memory_clean']
report = {**profile.metadata(), 'scope': 'Production-mode GNU and rboxc Less display 80 ordinary lines through '
          'a private controlling terminal, receive q after the first screen, and restore the original terminal '
          'settings. Output bytes are compared exactly across native and Valgrind profiles. GNU ownership '
          'findings are retained; candidate memory and owned descriptors must be clean.',
          'driver_sha256': fingerprint(Path(__file__)), 'fixture': fixture.hex(),
          'passed': int(passed), 'total': 1, 'equivalent': equivalent, 'outcomes': outcomes}
profile.report.write_text(json.dumps(report, indent=2)+'\n')
print('Production terminal:', 'PASS' if passed else 'OPEN')
raise SystemExit(not passed)
