#!/usr/bin/env python3
"""Drop privileges after the harness prepares its private fixtures and mounts."""
# SPDX-License-Identifier: GPL-3.0-or-later
import ctypes
import json
import os
from pathlib import Path
import sys
config=json.loads(Path(sys.argv[1]).read_text())
work=Path(config['work']).resolve(strict=True)
assert work.parent==Path('/var/tmp') and work.name.startswith('rboxc-bash-original-')
assert os.geteuid()==0 and Path.cwd()==work
os.setgroups([]);os.setgid(65534);os.setuid(65534)
assert os.geteuid()==os.getegid()==65534 and os.getgroups()==[]
assert ctypes.CDLL(None).prctl(38,1,0,0,0)==0
status=Path('/proc/self/status').read_text()
assert 'CapEff:\t0000000000000000\n' in status
profile={'uid':os.geteuid(),'gid':os.getegid(),'groups':os.getgroups(),'no_new_privileges':True,
 'cap_effective':'0000000000000000','work':str(work),'mount_namespace':os.readlink('/proc/self/ns/mnt')}
Path(config['journal']).write_text(json.dumps(profile,indent=2)+'\n')
os.execv(sys.argv[2],sys.argv[2:])
