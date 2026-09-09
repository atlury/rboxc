#!/usr/bin/env python3
"""Compare recovery of a removed private Screen socket and normal daemon exit."""
# SPDX-License-Identifier: GPL-3.0-or-later
import importlib.util,json,os,re,resource,shlex,signal,subprocess,sys,tempfile,time
from pathlib import Path
from comparison_profile import ComparisonProfile,fingerprint
ROOT=Path(__file__).resolve().parents[1]
sys.path.insert(0,str(ROOT/'tests/gnu'))
spec=importlib.util.spec_from_file_location('reviewed',ROOT/'tests/gnu/reviewed-original.py')
runner=importlib.util.module_from_spec(spec);spec.loader.exec_module(runner)
profile=ComparisonProfile('screen-socket-recovery',oracle=ROOT/'build/gnu-screen/screen')
results={}
for implementation,binary in [('gnu',profile.oracle),('rboxc',profile.binary)]:
 for instrument in [False,True]:
  key=implementation+('-valgrind' if instrument else '')
  saved=profile.logs/key;saved.mkdir();(saved/'memory').mkdir()
  with tempfile.TemporaryDirectory(prefix='rboxc-recover-') as directory:
   work=Path(directory);(work/'sockets').mkdir(mode=0o700);(work/'bin').mkdir();(work/'screen').symlink_to(binary)
   sleep=work/'bin/sleep';sleep.write_text('#!/bin/sh\nexec '+shlex.join([str(ROOT/'build/gnu-coreutils/src/coreutils'),'--coreutils-prog=sleep'])+' "$@"\n');sleep.chmod(0o755)
   env={'PATH':str(work/'bin')+':/usr/bin:/bin','HOME':directory,'SCREENDIR':str(work/'sockets'),
        'SCREENRC':'/dev/null','TERM':'xterm','LC_ALL':'C','SHELL':'/bin/sh'}
   command=[str(work/'screen'),'-D','-m','-S','recovery','sh','-c','sleep 30']
   if instrument:command=['/usr/bin/valgrind','--leak-check=full','--show-leak-kinds=all','--track-fds=yes','--trace-children=yes','--log-file='+str(saved/'memory/%p.log'),*command]
   def limits():resource.setrlimit(resource.RLIMIT_NOFILE,(256,256))
   with (saved/'stdout').open('wb') as out,(saved/'stderr').open('wb') as err:
    process=subprocess.Popen(command,env=env,cwd=work,stdin=subprocess.DEVNULL,stdout=out,stderr=err,start_new_session=True,preexec_fn=limits)
    try:
     deadline=time.monotonic()+15
     while not list((work/'sockets').iterdir()) and process.poll() is None and time.monotonic()<deadline:time.sleep(.05)
     sockets=list((work/'sockets').iterdir());assert len(sockets)==1,(key,process.poll())
     socket=sockets[0];assert socket.name.startswith(str(process.pid)+'.')
     ready=subprocess.run([str(work/'screen'),'-S','recovery','-Q','windows'],env=env,cwd=work,capture_output=True,timeout=15)
     (saved/'ready.stdout').write_bytes(ready.stdout);(saved/'ready.stderr').write_bytes(ready.stderr)
     assert ready.returncode==0
     socket.unlink();os.kill(process.pid,signal.SIGCHLD)
     deadline=time.monotonic()+15
     while not socket.exists() and process.poll() is None and time.monotonic()<deadline:time.sleep(.05)
     recovered=socket.is_socket();assert recovered
     done=subprocess.run([str(work/'screen'),'-S','recovery','-X','quit'],env=env,cwd=work,capture_output=True,timeout=15)
     (saved/'control.stdout').write_bytes(done.stdout);(saved/'control.stderr').write_bytes(done.stderr)
     assert done.returncode==0
     status=process.wait(timeout=15);assert status==0 and not socket.exists()
    finally:
     if process.poll() is None:os.killpg(process.pid,9);process.wait()
  memory=[]
  if instrument:
   deadline=time.monotonic()+5
   while time.monotonic()<deadline:
    logs=sorted((saved/'memory').glob('*.log'))
    if logs and all('ERROR SUMMARY:' in p.read_text() for p in logs):break
    time.sleep(.05)
   for log in logs:
    parsed=runner.parse_memory_log(log.read_text(),log.stem,exec_only=True)
    memory.append({'path':str(log.relative_to(ROOT)),'sha256':fingerprint(log),**parsed})
  clean=bool(memory) and all(m['complete_exec_log'] and m['errors']==m['non_inherited_descriptors']==0 and not any(m['heap_bytes'].get(k,0) for k in ['definitely lost','indirectly lost','possibly lost']) for m in memory)
  results[key]={'status':status,'recovered':recovered,'control_status':done.returncode,'memory':memory,'memory_clean':clean if instrument else None,
   'streams':{p.name:{'path':str(p.relative_to(ROOT)),'sha256':fingerprint(p)} for p in saved.iterdir() if p.is_file()}}
  print(key,status,'clean',clean if instrument else 'unassessed',flush=True)
report={**profile.metadata(),'scope':'Each process owns a private detached Screen session. Remove only its Unix socket, '
 'after a successful windows query confirms daemon readiness; deliver SIGCHLD to trigger its original socket recovery path, confirm recreation and quit '
 'through the recovered socket. The control client is outside instrumentation. GNU findings are retained.',
 'driver_sha256':fingerprint(Path(__file__)),'passed':int(results['rboxc-valgrind']['memory_clean']),'total':1,'results':results}
profile.report.write_text(json.dumps(report,indent=2)+'\n')
raise SystemExit(not report['passed'])
