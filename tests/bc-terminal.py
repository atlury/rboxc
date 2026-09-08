#!/usr/bin/env python3
"""Compare calculator terminal interaction and BC's normal interrupt handler."""
# SPDX-License-Identifier: GPL-3.0-or-later
import errno
import fcntl
import importlib.util
import json
import os
from pathlib import Path
import pty
import selectors
import signal
import subprocess
import sys
import tempfile
import termios
import time
from comparison_profile import ComparisonProfile, fingerprint

ROOT=Path(__file__).resolve().parents[1]
sys.path.insert(0,str(ROOT/'tests/gnu'))
spec=importlib.util.spec_from_file_location('reviewed',ROOT/'tests/gnu/reviewed-original.py')
runner=importlib.util.module_from_spec(spec);spec.loader.exec_module(runner)
profile=ComparisonProfile('bc-terminal',oracle=ROOT/'build/gnu-bc/bc/bc')
oracles={n:ROOT/'build/gnu-bc'/n/n for n in ('bc','dc')}
hashes={n:fingerprint(p) for n,p in oracles.items()}
cases=[
    ('bc-terminal','bc',{},b'2+3\n13579\n',b'quit\n',b'5\r\n',False),
    ('bc-environment-math','bc',{'BC_ENV_ARGS':'-q -l'},b'scale=4;sqrt(2)\n13579\n',b'quit\n',b'1.4142\r\n',False),
    ('bc-interrupt','bc',{},b'13579\n',b'',b'(interrupt) Exiting bc.',True),
    ('dc-terminal','dc',{},b'2 3 + p 13579p\n',b'q\n',b'5\r\n',False),
]
driver_hash=fingerprint(Path(__file__));results=[]
for index,(name,command,extra,initial,finish,expected,interrupt) in enumerate(cases):
    outcomes={}
    for implementation in ('gnu','rboxc'):
        for instrument in (False,True):
            key=implementation+('-valgrind' if instrument else '')
            with tempfile.TemporaryDirectory(prefix='rboxc-bc-terminal-') as directory:
                work=Path(directory);(work/command).symlink_to(oracles[command] if implementation=='gnu' else profile.binary)
                invocation=[str(work/command)]
                log=profile.logs/f'{index:02}-{key}.log'
                if instrument:invocation=['/usr/bin/valgrind','--leak-check=full','--show-leak-kinds=all','--track-fds=yes','--log-file='+str(log),*invocation]
                master,slave=pty.openpty()
                attributes=termios.tcgetattr(slave);attributes[3]&=~termios.ECHO;termios.tcsetattr(slave,termios.TCSANOW,attributes)
                def terminal_session():
                    os.setsid();fcntl.ioctl(slave,termios.TIOCSCTTY,0)
                process=None;output=bytearray();ready=False;timed_out=False
                try:
                    process=subprocess.Popen(invocation,stdin=slave,stdout=slave,stderr=slave,
                        preexec_fn=terminal_session,cwd=work,
                        env={'PATH':'/usr/bin:/bin','HOME':directory,'LC_ALL':'C','TERM':'dumb','TZ':'UTC0',**extra})
                    os.close(slave);slave=-1
                    assert os.write(master,initial)==len(initial)
                    deadline=time.monotonic()+30
                    with selectors.DefaultSelector() as selector:
                        selector.register(master,selectors.EVENT_READ)
                        while True:
                            if time.monotonic()>=deadline:
                                timed_out=True;break
                            if not selector.select(0.1):
                                if process.poll() is not None:break
                                continue
                            try:data=os.read(master,65536)
                            except OSError as error:
                                if error.errno==errno.EIO:break
                                raise
                            if not data:break
                            output.extend(data)
                            if not ready and b'13579\r\n' in output:
                                ready=True
                                if interrupt:os.kill(process.pid,signal.SIGINT)
                                else:assert os.write(master,finish)==len(finish)
                    if timed_out:os.killpg(process.pid,signal.SIGKILL)
                    status=process.wait(timeout=5)
                finally:
                    if process is not None and process.poll() is None:
                        os.killpg(process.pid,signal.SIGKILL);process.wait()
                    os.close(master)
                    if slave!=-1:os.close(slave)
                outcome={'status':status,'output':bytes(output).hex(),'ready':ready,'timed_out':timed_out,'expected_output':expected in output}
                if instrument:
                    memory=runner.parse_memory_log(log.read_text(),str(process.pid),exec_only=True)
                    outcome['memory']={**memory,'log':str(log.relative_to(ROOT)),'sha256':fingerprint(log)}
                outcomes[key]=outcome
    equivalent=all((r['status'],r['output'])==(outcomes['gnu']['status'],outcomes['gnu']['output']) for r in outcomes.values())
    expected_ok=all(r['ready'] and not r['timed_out'] and r['expected_output'] and r['status']==0 for r in outcomes.values())
    memory=outcomes['rboxc-valgrind']['memory']
    clean=memory['complete_exec_log'] and memory['errors']==0 and memory['non_inherited_descriptors']==0 and not any(memory['heap_bytes'].get(k,0) for k in ('definitely lost','indirectly lost','possibly lost'))
    results.append({'name':name,'command':command,'environment':extra,'pass':equivalent and expected_ok and clean,'equivalent':equivalent,'expected_output_pass':expected_ok,'memory_clean':clean,'outcomes':outcomes})
    print('PASS' if results[-1]['pass'] else 'OPEN',name,flush=True)
    assert fingerprint(Path(__file__))==driver_hash and all(fingerprint(p)==hashes[n] for n,p in oracles.items())
    profile.report.write_text(json.dumps({'scope':'Four controlling-terminal profiles with echo disabled: BC/DC arithmetic, bounded BC_ENV_ARGS math options, and BC SIGINT after an observed output marker. Status and complete terminal bytes are compared; candidate Valgrind logs remain strict.',
        **profile.metadata(),'driver_sha256':driver_hash,'gnu_binaries':hashes,'passed':sum(r['pass'] for r in results),'total':len(results),'results':results},indent=2)+'\n')
raise SystemExit(any(not r['pass'] for r in results))
