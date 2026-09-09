#!/usr/bin/env python3
"""Distinguish original SIGPIPE exits from recoverable EPIPE cleanup on finite input."""
# SPDX-License-Identifier: GPL-3.0-or-later
import importlib.util
import json
import os
from pathlib import Path
import re
import signal
import subprocess
import sys
import tempfile
from comparison_profile import ComparisonProfile,fingerprint
ROOT=Path(__file__).resolve().parents[1]
sys.path[:0]=[str(ROOT/'tests/gnu')]
spec=importlib.util.spec_from_file_location('reviewed',ROOT/'tests/gnu/reviewed-original.py')
runner=importlib.util.module_from_spec(spec);spec.loader.exec_module(runner)
profile=ComparisonProfile('coreutils-pipe-exit')
driver=fingerprint(Path(__file__));results=[]
with tempfile.TemporaryDirectory(prefix='rboxc-pipe-exit-') as directory:
 work=Path(directory);(work/'foo').write_bytes(b'foo\n')
 for command,args,data in [('cat',['foo'],b''),('dd',['status=none','if=foo'],b''),('tac',[],b''.join(str(i).encode()+b'\n' for i in range(1,10001)))]:
  for ignored in [False,True]:
   outcomes={}
   for impl,binary in [('gnu',profile.oracle),('rboxc',profile.binary)]:
    alias=work/command;alias.unlink(missing_ok=True);alias.symlink_to(binary)
    for instrument in [False,True]:
     label=f'{command}-{int(ignored)}-{impl}-{int(instrument)}'
     log=profile.logs/(label+'.log');argv=[str(alias),*args]
     if instrument:argv=['/usr/bin/valgrind','--leak-check=full','--show-leak-kinds=all','--track-fds=yes','--log-file='+str(log),*argv]
     read_fd,write_fd=os.pipe();os.close(read_fd)
     def disposition():signal.signal(signal.SIGPIPE,signal.SIG_IGN if ignored else signal.SIG_DFL)
     try:
      done=subprocess.run(argv,cwd=work,env={'PATH':'/usr/bin:/bin','LC_ALL':'C','HOME':directory,'TMPDIR':directory},input=data,stdout=write_fd,stderr=subprocess.PIPE,preexec_fn=disposition,restore_signals=False,timeout=30)
     finally:os.close(write_fd)
     outcome={'status':done.returncode,'stderr':done.stderr.hex(),'input_bytes':len(data),'input_sha256':__import__('hashlib').sha256(data).hexdigest()}
     assert done.returncode==(1 if ignored else -signal.SIGPIPE),(label,outcome)
     if instrument:
      text=log.read_text();pid,=re.findall(r'^==(\d+)== Command:',text,re.M)
      memory={'log':str(log.relative_to(ROOT)),'sha256':fingerprint(log),**runner.parse_memory_log(text,pid,True)}
      assert memory['complete_exec_log'];outcome['memory']=memory
      if impl=='rboxc' and ignored:
       outcome['strict_memory_pass']=memory['errors']==memory['non_inherited_descriptors']==0 and not any(memory['heap_bytes'].get(k,0) for k in ['definitely lost','indirectly lost','possibly lost'])
     outcomes[impl+('-valgrind' if instrument else '-native')]=outcome
   baseline=outcomes['gnu-native']
   assert all(o['status']==baseline['status'] and o['stderr']==baseline['stderr'] for o in outcomes.values()),(command,ignored,outcomes)
   results.append({'command':command,'arguments':args,'sigpipe_ignored':ignored,'outcomes':outcomes,'behavior_pass':True})
   profile.report.write_text(json.dumps({**profile.metadata(),'driver_sha256':driver,'scope':'Finite cat/dd file and tac stream writes to a pipe with no reader. Default SIGPIPE and explicitly ignored SIGPIPE are separate profiles; ignored signal exercises ordinary EPIPE cleanup, without changing production signal policy.',
    'results':results,'complete':len(results)==6},indent=2)+'\n')
   print(command,'EPIPE' if ignored else 'SIGPIPE',outcomes['rboxc-valgrind'].get('strict_memory_pass'),flush=True)
assert fingerprint(Path(__file__))==driver
raise SystemExit(not all(r['outcomes']['rboxc-valgrind']['strict_memory_pass'] for r in results if r['sigpipe_ignored']))
