#!/usr/bin/env python3
"""Run GNU Screen's unchanged attach/detach integration test in private sessions."""
# SPDX-License-Identifier: GPL-3.0-or-later
import importlib.util
import json
import os
from pathlib import Path
import re
import resource
import shlex
import subprocess
import sys
import tempfile
import time
from comparison_profile import ComparisonProfile, fingerprint

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT/'tests/gnu'))
spec = importlib.util.spec_from_file_location('reviewed', ROOT/'tests/gnu/reviewed-original.py')
runner = importlib.util.module_from_spec(spec)
spec.loader.exec_module(runner)
profile = ComparisonProfile('screen-original-detach', oracle=ROOT/'build/gnu-screen/screen')
source = Path('/opt/src/screen-5.0.2')
inputs = {p:fingerprint(p) for p in [Path(__file__), source/'tests/integration-detach.c',
    source/'tests/macros.h', source/'Makefile.in', Path('/etc/screenrc'),
    ROOT/'build/gnu-screen/config.h', Path('/usr/bin/script'), Path('/bin/sh').resolve(),
    ROOT/'build/gnu-coreutils/src/coreutils']}
# The pinned configuration disables utmp; the reviewed system configuration only
# sets display options and key bindings. Every socket and user config is private.
assert '/* #undef ENABLE_UTMP */' in (ROOT/'build/gnu-screen/config.h').read_text()
binary = profile.logs/'test-detach-cli'
argv = ['gcc', '-g', '-O2', str(source/'tests/integration-detach.c'), '-o', str(binary)]
done = subprocess.run(argv, capture_output=True, timeout=60)
(profile.logs/'build.log').write_bytes(done.stdout+done.stderr)
assert done.returncode == 0
outcomes = {}
for implementation, provider in [('gnu', profile.oracle), ('rboxc', profile.binary)]:
    for instrument in [False, True]:
        key = implementation+('-valgrind' if instrument else '')
        work = profile.logs/key
        work.mkdir()
        socket_directory = tempfile.TemporaryDirectory(prefix='rboxc-screen-')
        socket_path = Path(socket_directory.name)
        (work/'memory').mkdir()
        (work/'helpers').mkdir()
        sleep = work/'helpers/sleep'
        sleep.write_text('#!/bin/sh\nexec '+shlex.join([
            str(ROOT/'build/gnu-coreutils/src/coreutils'), '--coreutils-prog=sleep'])+' "$@"\n')
        sleep.chmod(0o755)
        (work/'real').mkdir()
        (work/'real/screen').symlink_to(provider)
        command = [str(work/'real/screen')]
        if instrument:
            command = ['/usr/bin/valgrind', '--leak-check=full', '--show-leak-kinds=all',
                '--track-fds=yes', '--trace-children=yes', '--log-file='+str(work/'memory/%p.log'), *command]
        (work/'screen').write_text('#!/bin/sh\nexec '+shlex.join(command)+' "$@"\n')
        (work/'screen').chmod(0o755)
        env = {'PATH':str(work/'helpers')+':/usr/bin:/bin', 'HOME':str(work), 'SCREENDIR':str(socket_path),
            'SCREENRC':'/dev/null', 'LC_ALL':'C', 'LANGUAGE':'C', 'TERM':'xterm', 'SHELL':'/bin/sh'}
        def private_limits():
            resource.setrlimit(resource.RLIMIT_NOFILE, (256, 256))
        with (work/'stdout').open('wb') as stdout, (work/'stderr').open('wb') as stderr:
            process = subprocess.Popen([str(binary)], cwd=work, env=env, stdin=subprocess.DEVNULL,
                stdout=stdout, stderr=stderr, start_new_session=True, preexec_fn=private_limits)
            timed_out = False
            try:
                status = process.wait(timeout=120)
            except subprocess.TimeoutExpired:
                timed_out = True
                os.killpg(process.pid, 9)
                status = process.wait()
            finally:
                # Only this profile's private socket directory is visible here.
                if list(socket_path.iterdir()):
                    subprocess.run([str(work/'real/screen'), '-X', 'quit'], cwd=work, env=env,
                        stdin=subprocess.DEVNULL, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, timeout=10)
                if process.poll() is None:
                    os.killpg(process.pid, 9)
                    process.wait()
        deadline = time.monotonic()+10
        while list(socket_path.iterdir()) and time.monotonic()<deadline:
            time.sleep(.1)
        memory = []
        if instrument:
            # Daemon cleanup can finish just after the original C test returns.
            while time.monotonic()<deadline:
                logs = sorted((work/'memory').glob('*.log'))
                if logs and all('ERROR SUMMARY:' in p.read_text() for p in logs): break
                time.sleep(.1)
            for log in logs:
                text = log.read_text()
                commands = re.findall(r'^==[0-9]+== Command: (.*)$',text,re.M)
                parsed = runner.parse_memory_log(text,log.stem,exec_only=True)
                memory.append({'log':str(log.relative_to(ROOT)), 'sha256':fingerprint(log),
                    'commands':commands, **parsed})
        clean = bool(memory) and all(m['complete_exec_log'] and m['errors']==m['non_inherited_descriptors']==0 and not any(
            m['heap_bytes'].get(k,0) for k in ('definitely lost','indirectly lost','possibly lost')) for m in memory)
        outcomes[key] = {'status':status, 'timed_out':timed_out, 'descriptor_limit':256,
            'sockets_remaining':[p.name for p in socket_path.iterdir()],
            'streams':{n:{'path':str((work/n).relative_to(ROOT)), 'sha256':fingerprint(work/n),
                'bytes':(work/n).read_bytes().hex()} for n in ['stdout','stderr']},
            'environment':env, 'memory':memory, 'memory_clean':clean if instrument else None}
        print(key, 'status', status, 'memory clean', clean if instrument else 'unassessed', flush=True)
        assert all(fingerprint(p)==v for p,v in inputs.items())
        passed = len(outcomes)==4 and all(o['status']==0 and not o['sockets_remaining'] for o in outcomes.values()) and outcomes['rboxc-valgrind']['memory_clean']
        report = {**profile.metadata(), 'scope':'Unchanged GNU Screen original C integration assertions: create '
            'a private detached session with a bounded local sleep, attach through util-linux script, '
            'observe Attached, remotely detach, observe Detached, wait for client exit, and quit. '
            'Screen invocations and their executed shell/sleep descendants are instrumented; '
            'the original C test controller and external script PTY driver are outside instrumentation. '
            'System configuration is reviewed and fingerprinted, user configuration is empty, utmp is disabled. '
            'All profiles use a 256-descriptor process limit to bound the native GNU descriptor sweep. '
            'Short private socket paths fit the Unix-domain address limit. The original sleep command '
            'uses pinned GNU Coreutils with an explicit command selector in both implementations.',
            'driver_sha256':fingerprint(Path(__file__)), 'inputs':{str(p):v for p,v in inputs.items()},
            'test_binary_sha256':fingerprint(binary), 'build_command':argv,
            'passed':int(passed), 'total':1, 'complete':len(outcomes)==4, 'outcomes':outcomes}
        profile.report.write_text(json.dumps(report,indent=2)+'\n')
        socket_directory.cleanup()
raise SystemExit(not passed)
