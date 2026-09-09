#!/usr/bin/env python3
"""Exercise owned tail followers through natural --pid and terminal EOF exits."""
# SPDX-License-Identifier: GPL-3.0-or-later
import fcntl
import importlib.util
import json
import os
from pathlib import Path
import subprocess
import sys
import termios
import time
from comparison_profile import ComparisonProfile, fingerprint

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT/'tests/gnu'))
spec = importlib.util.spec_from_file_location('reviewed', ROOT/'tests/gnu/reviewed-original.py')
runner = importlib.util.module_from_spec(spec); spec.loader.exec_module(runner)
profile = ComparisonProfile('tail-normal-lifecycle')
env = {'PATH':'/usr/bin:/bin','LC_ALL':'C','LANGUAGE':'C'}
results = []

def wait_for(proc, condition):
    deadline = time.monotonic()+12
    while not condition():
        assert proc.poll() is None, 'tail exited before expected output'
        assert time.monotonic() < deadline, 'tail output deadline exceeded'
        time.sleep(.02)

def terminal_session():
    os.setsid()
    fcntl.ioctl(0, termios.TIOCSCTTY, 0)

for case in ['initial', 'append', 'rename', 'missing', 'headers', 'terminal-stdin', 'terminal-dash', 'terminal-path']:
    modes = [''] if case.startswith('terminal') else ['', '---disable-inotify']
    for mode in modes:
        outcomes = {}
        for impl, instrument in [('gnu',False), ('rboxc',False), ('gnu',True), ('rboxc',True)]:
            tag = f'{case}-{bool(mode)}-{impl}-{instrument}'
            run = profile.logs/tag; run.mkdir()
            binary = profile.oracle if impl == 'gnu' else profile.binary
            (run/'tail').symlink_to(binary)
            memory_log = run/'memory.log'
            args = [str(run/'tail')]
            owner = None; master = slave = None
            if case.startswith('terminal'):
                args += ['-n', '1', *({'terminal-stdin':[], 'terminal-dash':['-'], 'terminal-path':['/dev/tty']}[case])]
                master, slave = os.openpty()
            else:
                owner = subprocess.Popen([sys.executable,'-c','import sys; sys.stdin.buffer.read()'],
                    stdin=subprocess.PIPE, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
                if case not in ('missing','headers'):
                    (run/'a').write_text('first\n')
                args += ['-s.05', '--max-unchanged-stats=1', f'--pid={owner.pid}']
                if mode: args.append(mode)
                args += ['-F' if case in ('missing','headers') else '-f', 'a']
                if case == 'headers': args.append('b')
            if instrument:
                args = ['/usr/bin/valgrind','--leak-check=full','--show-leak-kinds=all',
                        '--track-fds=yes','--log-file='+str(memory_log), *args]
            with (run/'stdout').open('wb') as out, (run/'stderr').open('wb') as err:
                proc = subprocess.Popen(args, cwd=run, env=env,
                    stdin=slave if slave is not None else subprocess.DEVNULL,
                    stdout=out, stderr=err, preexec_fn=terminal_session if slave is not None else None)
                if slave is not None: os.close(slave)
                try:
                    if case.startswith('terminal'):
                        # Canonical EOF after complete lines, without sending a signal.
                        os.write(master, b'first\nlast\n\x04')
                        expected = b'last\n'
                    else:
                        stdout = lambda: (run/'stdout').read_bytes()
                        stderr = lambda: (run/'stderr').read_bytes()
                        if case in ('missing','headers'):
                            wait_for(proc, lambda: b'cannot open' in stderr())
                            (run/'a').write_text('first\n')
                            wait_for(proc, lambda: b'first\n' in stdout())
                            if case == 'headers':
                                (run/'b').write_text('second\n')
                                wait_for(proc, lambda: b'second\n' in stdout())
                                expected = b'==> a <==\nfirst\n\n==> b <==\nsecond\n'
                            else: expected = b'first\n'
                        else:
                            wait_for(proc, lambda: stdout() == b'first\n')
                            if case == 'rename': (run/'a').rename(run/'b')
                            if case in ('append','rename'):
                                with (run/('b' if case=='rename' else 'a')).open('a') as f: f.write('second\n')
                                wait_for(proc, lambda: stdout() == b'first\nsecond\n')
                                expected = b'first\nsecond\n'
                            else: expected = b'first\n'
                        owner.stdin.close(); owner.wait(timeout=5)
                    status = proc.wait(timeout=15)
                finally:
                    if owner is not None:
                        if owner.poll() is None: owner.kill()
                        owner.wait()
                    if proc.poll() is None: proc.kill()
                    proc.wait()
                    if master is not None: os.close(master)
            stdout = (run/'stdout').read_bytes(); stderr = (run/'stderr').read_bytes()
            expected_status = 1 if case in ('missing','headers') else 0
            outcome = {'status':status,'stdout':stdout.hex(),'stderr':stderr.hex(),
                       'expected_stdout':expected.hex(),'expected_status':expected_status,'normal_exit':status>=0,
                       'logs':{str(p.relative_to(ROOT)):fingerprint(p) for p in [run/'stdout',run/'stderr']}}
            if instrument:
                memory = runner.parse_memory_log(memory_log.read_text(), tag)
                clean = memory['errors'] == memory['non_inherited_descriptors'] == 0 and not any(
                    memory['heap_bytes'].get(k,0) for k in ['definitely lost','indirectly lost','possibly lost'])
                outcome.update(memory=memory, memory_clean=clean)
                outcome['logs'][str(memory_log.relative_to(ROOT))] = fingerprint(memory_log)
            outcome['pass'] = status==expected_status and stdout==expected and (not instrument or impl=='gnu' or clean)
            outcomes[f'{impl}-{"valgrind" if instrument else "native"}'] = outcome
        behavioral = [{k:r[k] for k in ['status','stdout','stderr']} for r in outcomes.values()]
        equivalent = all(r==behavioral[0] for r in behavioral)
        results.append({'case':case,'mode':mode,'outcomes':outcomes,'pass':equivalent and all(r['pass'] for r in outcomes.values())})
        print(case,mode or 'inotify/default', 'PASS' if results[-1]['pass'] else 'OPEN',flush=True)
report = {**profile.metadata(), 'scope':'Focused natural --pid/terminal EOF exits; does not replace signal-terminated originals.',
          'driver_sha256':fingerprint(Path(__file__)), 'results':results,
          'passed':sum(r['pass'] for r in results),'total':len(results)}
profile.report.write_text(json.dumps(report,indent=2)+'\n')
raise SystemExit(report['passed'] != report['total'])
