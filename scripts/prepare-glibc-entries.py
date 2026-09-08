#!/usr/bin/env python3
"""Prepare pinned glibc utility objects without installing a replacement libc."""
# SPDX-License-Identifier: GPL-3.0-or-later
import hashlib
import json
import os
from pathlib import Path
import subprocess

ROOT=Path(__file__).resolve().parents[1]
digest=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
pin=json.loads((ROOT/'inventory/sources.json').read_text())['glibc']
source=Path(pin['source']);build=ROOT/'build/gnu-glibc';build.mkdir(exist_ok=True)
for path,expected in pin['entry_source_sha256'].items():assert digest(source/path)==expected
# Reuse the recorded isolated tool packages. This step never installs packages.
tool_pin=json.loads((ROOT/'evidence/glibc-build-tools.json').read_text())
for path,key in [(ROOT/'build'/tool_pin['bison_package'],'package_sha256'),
                 (ROOT/'build'/tool_pin['m4_package'],'m4_package_sha256'),
                 (ROOT/'.tools/bison-package/usr/bin/bison','bison_sha256'),
                 (ROOT/'.tools/m4-package/usr/bin/m4','m4_binary_sha256')]:
 assert digest(path)==tool_pin[key], 'prepare the recorded isolated tool: '+str(path)
bin_dir=ROOT/'.tools/build-tools';bin_dir.mkdir(exist_ok=True)
for name,target in {'bison':ROOT/'.tools/bison-package/usr/bin/bison',
                    'm4':ROOT/'.tools/m4-package/usr/bin/m4',
                    'gawk':ROOT/'build/gnu-gawk/gawk'}.items():
 path=bin_dir/name
 if path.is_symlink():assert path.resolve()==target.resolve()
 else:assert not path.exists();path.symlink_to(target)
env={**os.environ,'CFLAGS':'-g -O2','PATH':str(bin_dir)+':'+os.environ['PATH'],
     'BISON_PKGDATADIR':str(ROOT/'.tools/bison-package/usr/share/bison'),
     'M4':str(ROOT/'.tools/m4-package/usr/bin/m4'),
     'RBOXC_CC_RECORDS':str(ROOT/'build/glibc-cc-records')}
configure=[str(source/'configure'),'--prefix='+str(ROOT/'build/oracle/glibc'),'--disable-werror']
if not (build/'Makefile').exists():
 with (ROOT/'evidence/raw/glibc-configure-recorded.log').open('w') as log:
  subprocess.run(configure,cwd=build,env=env,stdout=log,stderr=subprocess.STDOUT,check=True)
args=['make','-j8','CC=python3 '+str(ROOT/'scripts/record-provider-cc.py')]
with (ROOT/'evidence/raw/glibc-native-recorded.log').open('w') as log:
 subprocess.run(args,cwd=build,env=env,stdout=log,stderr=subprocess.STDOUT,check=True)
runtime={Path('/usr/lib/x86_64-linux-gnu/ld-linux-x86-64.so.2'),Path('/usr/lib/x86_64-linux-gnu/libc.so.6')}
prior=json.loads((ROOT/'evidence/glibc-build-profile.json').read_text())
for path in runtime:assert digest(path)==prior['runtime'][str(path)], 'review changed host runtime'
loader=ROOT/'build/oracle/glibc/lib/ld-linux-x86-64.so.2';loader.parent.mkdir(parents=True,exist_ok=True)
host_loader=Path('/usr/lib/x86_64-linux-gnu/ld-linux-x86-64.so.2')
if loader.is_symlink():assert loader.resolve()==host_loader.resolve()
else:assert not loader.exists();loader.symlink_to(host_loader)
oracles={}
for name,path in {'getconf':build/'posix/getconf','iconv':build/'iconv/iconv_prog'}.items():
 version=subprocess.check_output([path,'--version'],text=True).splitlines()[0]
 assert version==name+' (GNU libc) '+pin['version']
 oracles[name]={'path':str(path),'sha256':digest(path),'version':version}
report={'scope':'Pinned GNU utility and helper objects; original utilities use the recorded existing host runtime via a private interpreter symlink. The full native build is preparation only. No make install, CRT/libc archive integration, or host library replacement.',
        'version':pin['version'],'configure_arguments':configure,'make_arguments':args,
        'build_cflags':env['CFLAGS'],'driver_sha256':digest(Path(__file__)),
        'runtime':{str(p):digest(p) for p in sorted(runtime)},'oracles':oracles,
        'tool_profile_sha256':digest(ROOT/'evidence/glibc-build-tools.json')}
(ROOT/'evidence/glibc-preparation.json').write_text(json.dumps(report,indent=2)+'\n')
print('Prepared GNU getconf/iconv utility objects with unchanged host libc')
