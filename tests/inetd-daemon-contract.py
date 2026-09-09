#!/usr/bin/env python3
"""Exercise real daemon forks, cwd, descriptor ownership and redirection."""
# SPDX-License-Identifier: GPL-3.0-or-later
import ctypes
import importlib.util
import itertools
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import time
from comparison_profile import fingerprint
ROOT=Path(__file__).resolve().parents[1]
sys.path[:0]=[str(ROOT/'tests/gnu')]
spec=importlib.util.spec_from_file_location('reviewed',ROOT/'tests/gnu/reviewed-original.py')
runner=importlib.util.module_from_spec(spec);spec.loader.exec_module(runner)
report=ROOT/'evidence/inetd-daemon-contract.json';assert not report.exists()
logs=ROOT/'evidence/raw/inetd-daemon-contract';logs.mkdir(exist_ok=True)
logs=Path(tempfile.mkdtemp(prefix='run-',dir=logs))
source=ROOT/'tests/gnu/inetd-daemon-contract.c'
objects={'gnu':ROOT/'build/gnu-inetutils/libinetutils/daemon.o','adapted':ROOT/'build/inetd-daemon-cleanup/daemon.o'}
library=ROOT/'build/gnu-inetutils/lib/libgnu.a'
inputs={str(p):fingerprint(p) for p in [source,Path(__file__),library,*objects.values()]}
compiler=['cc','-O2','-Wall','-Wextra','-Werror',str(source)]
executables={}
for impl,object_file in objects.items():
 executable=logs/impl
 subprocess.run([*compiler,str(object_file),str(library),'-o',str(executable)],check=True)
 executables[impl]=executable
 inputs[str(executable)]=fingerprint(executable)
# Adopt only descendants of this test process so daemon grandchildren can be reaped.
assert ctypes.CDLL(None).prctl(36,1,0,0,0)==0
rows=[]
for instrument,nochdir,noclose,sparse in itertools.product([False,True],range(2),range(2),range(2)):
 outcomes={}
 for impl,executable in executables.items():
  name=f'{impl}-{int(instrument)}-{nochdir}-{noclose}-{sparse}'
  with tempfile.TemporaryDirectory(prefix='rboxc-daemon-contract-') as directory:
   work=Path(directory);output=work/'result.json'
   owned=os.open(work/'owned',os.O_CREAT|os.O_RDWR,0o600)
   assert owned<9
   try:os.fstat(9)
   except OSError:pass
   else:raise AssertionError('fixture descriptor 9 already occupied')
   os.dup2(owned,9);os.close(owned)
   args=[str(executable),str(output),str(nochdir),str(noclose),str(sparse)]
   if instrument:args=['/usr/bin/valgrind','--leak-check=full','--show-leak-kinds=all','--track-fds=yes','--log-file='+str(logs/(name+'-%p.log')),*args]
   try:
    done=subprocess.run(args,cwd=work,stdin=subprocess.DEVNULL,stdout=subprocess.DEVNULL,stderr=subprocess.PIPE,pass_fds=(9,),timeout=15)
   finally:os.close(9)
   deadline=time.monotonic()+15;children=[]
   while time.monotonic()<deadline:
    try:pid,status=os.waitpid(-1,os.WNOHANG)
    except ChildProcessError:break
    if pid:children.append(os.waitstatus_to_exitcode(status))
    else:time.sleep(.02)
   else:raise AssertionError('daemon contract descendants did not terminate')
   assert done.returncode==0 and children==[0,0] and output.exists(),(name,done,children)
   result=json.loads(output.read_text());assert result['pass']
   memory=[]
   for path in sorted(logs.glob(name+'-*.log')):
    pid=path.stem.rsplit('-',1)[1];parsed=runner.parse_memory_log(path.read_text(),pid,True)
    memory.append({'log':str(path.relative_to(ROOT)),'sha256':fingerprint(path),**parsed})
   if instrument:
    assert len(memory)==3
    if impl=='adapted':
     assert all(m['complete_exec_log'] and m['errors']==m['non_inherited_descriptors']==0 and not any(m['heap_bytes'].get(k,0) for k in ['definitely lost','indirectly lost','possibly lost']) for m in memory),(name,memory)
   outcomes[impl]={'status':done.returncode,'reaped_statuses':children,'result':result,'memory':memory}
 assert outcomes['gnu']['result']==outcomes['adapted']['result']
 rows.append({'instrumented':instrument,'nochdir':nochdir,'noclose':noclose,'sparse':sparse,'outcomes':outcomes,'pass':True})
 report.write_text(json.dumps({'inputs':inputs,'results':rows,'passed':len(rows),'total':16,'complete':len(rows)==16},indent=2)+'\n')
 print('PASS',int(instrument),nochdir,noclose,sparse,flush=True)
assert all(fingerprint(Path(p))==h for p,h in inputs.items())
