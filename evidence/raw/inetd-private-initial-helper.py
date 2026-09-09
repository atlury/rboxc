#!/usr/bin/env python3
"""Run one owned inetd reload original in private network and PID namespaces."""
# SPDX-License-Identifier: GPL-3.0-or-later
import json
import os
from pathlib import Path
import signal
import subprocess
import sys
import time
config=json.loads(Path(sys.argv[1]).read_text())
assert os.geteuid()==0 and os.getpid()==1
namespaces={name:os.readlink('/proc/self/ns/'+name) for name in ['mnt','net','pid']}
assert all(namespaces[k]!=config['parent_namespaces'][k] for k in namespaces)
subprocess.run(['/usr/bin/mount','--make-rprivate','/'],check=True)
subprocess.run(['/usr/bin/mount','-t','proc','-o','nosuid,nodev,noexec','proc','/proc'],check=True)
subprocess.run(['/usr/sbin/ip','link','set','dev','lo','up'],check=True)
interfaces=json.loads(subprocess.check_output(['/usr/sbin/ip','-j','address','show'],text=True))
assert [r['ifname'] for r in interfaces]==['lo']
addresses={a['local'] for r in interfaces for a in r.get('addr_info',[])}
assert {'127.0.0.1','::1'} <= addresses
work=Path(config['work']).resolve(strict=True)
assert work.name.startswith('rboxc-inetd-')
def unprivileged():
 os.setgroups([]);os.setgid(65534);os.setuid(65534)
 assert os.geteuid()==os.getegid()==65534
profile={'namespaces':namespaces,'parent_namespaces':config['parent_namespaces'],
         'interfaces':interfaces,'uid':65534,'gid':65534,'groups':[]}
print('RBOXC_INETD_PRIVATE_PROFILE '+json.dumps(profile),flush=True)
result=subprocess.run(['/bin/sh',str(work/'inetd.sh')],cwd=work,env=config['environment'],preexec_fn=unprivileged)
# PID 1 reaps the daemon and helpers after the original's owned-PID cleanup.
deadline=time.monotonic()+10
reaped=[]
while time.monotonic()<deadline:
 try:
  pid,status=os.waitpid(-1,os.WNOHANG)
 except ChildProcessError:break
 if pid:reaped.append({'pid':pid,'status':os.waitstatus_to_exitcode(status)})
 else:time.sleep(.05)
else:
 raise AssertionError('original left child processes alive')
print('RBOXC_INETD_CHILDREN_REAPED '+json.dumps(reaped),flush=True)
raise SystemExit(result.returncode)
