#!/usr/bin/env python3
"""Run original local name/interface checks with an isolated loopback and hosts file."""
# SPDX-License-Identifier: GPL-3.0-or-later
import json
import os
from pathlib import Path
import subprocess
import sys
config=json.loads(Path(sys.argv[1]).read_text())
assert os.geteuid()==0
namespaces={k:os.readlink('/proc/self/ns/'+k) for k in ['mnt','net','uts']}
assert all(namespaces[k]!=config['parent_namespaces'][k] for k in namespaces)
work=Path(config['work']).resolve(strict=True);assert work.name.startswith('rboxc-inetutils-local-')
subprocess.run(['/usr/bin/mount','--make-rprivate','/'],check=True)
for name in ['hosts','nsswitch.conf']:
 subprocess.run(['/usr/bin/mount','--bind',str(work/name),'/etc/'+name],check=True)
 subprocess.run(['/usr/bin/mount','-o','remount,bind,ro','/etc/'+name],check=True)
subprocess.run(['/usr/bin/hostname','fixture.example.test'],check=True)
subprocess.run(['/usr/sbin/ip','link','set','dev','lo','up'],check=True)
interfaces=json.loads(subprocess.check_output(['/usr/sbin/ip','-j','address','show'],text=True))
assert [r['ifname'] for r in interfaces]==['lo']
os.setgroups([]);os.setgid(65534);os.setuid(65534)
assert os.geteuid()==os.getegid()==65534
print('RBOXC_INETUTILS_LOCAL_PROFILE '+json.dumps({'namespaces':namespaces,'parent_namespaces':config['parent_namespaces'],
 'interfaces':interfaces,'uid':os.geteuid(),'gid':os.getegid(),'groups':os.getgroups(),'hostname':os.uname().nodename}),flush=True)
result=subprocess.run(['/bin/sh','-x',str(work/config['script'])],cwd=work,env=config['environment'])
raise SystemExit(result.returncode)
