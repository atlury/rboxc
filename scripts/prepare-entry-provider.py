#!/usr/bin/env python3
"""Build pinned native oracles and capture authoritative entry link inputs."""
# SPDX-License-Identifier: GPL-3.0-or-later
import argparse
import hashlib
import json
import os
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[1]
parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('provider',choices=['gawk','bash','patch','binutils','inetutils'])
name = parser.parse_args().provider
pin = json.loads((ROOT/'inventory/sources.json').read_text())[name]
source = Path(pin['source']); build = ROOT/'build'/('gnu-'+name)
build.mkdir(exist_ok=True)
digest = lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
for path, expected in pin['entry_source_sha256'].items():
    assert digest(source/path)==expected
records = ROOT/'build'/(name+'-cc-records');records.mkdir(exist_ok=True)
environment = {**os.environ,'CFLAGS':pin.get('build_cflags','-g -O2 -std=gnu17'),'RBOXC_CC_RECORDS':str(records)}
if not (build/'Makefile').exists():
    args=[str(source/'configure'),'--prefix='+str(ROOT/'build/oracle'/name)]
    if name=='bash':args.append('--without-bash-malloc')
    if name=='binutils':args += ['--disable-gdb','--disable-gdbserver','--disable-gprofng','--disable-sim']
    with (ROOT/f'evidence/raw/{name}-configure.log').open('w') as log:
        subprocess.run(args,cwd=build,env=environment,stdout=log,stderr=subprocess.STDOUT,check=True)
with (ROOT/f'evidence/raw/{name}-native-build.log').open('w') as log:
    targets=['all-binutils'] if name=='binutils' else []
    subprocess.run(['make','-j8','CC=python3 '+str(ROOT/'scripts/record-provider-cc.py'),*targets],cwd=build,
                   env=environment,stdout=log,stderr=subprocess.STDOUT,check=True)
binary = build/{'patch':'src/patch','binutils':'binutils/ar','inetutils':'src/dnsdomainname'}.get(name,name)
links=[json.loads(p.read_text()) for p in records.glob('*.json')]
links=[r for r in links if r['kind']=='link' and (Path(r['directory'])/r['output']).resolve()==binary]
assert len(links)==1
profile={'provider':name,'version':pin['version'],'scope':'Pinned native oracle and recorded GNU compile/link inputs. Native helpers and runtime feature configuration are retained; this is not port certification.',
         'configure_arguments':subprocess.check_output(['./config.status','--config'],cwd=build,text=True).strip(),
         'config_header_sha256':digest(build/('binutils/config.h' if name=='binutils' else 'config.h')),
         'oracle':{'path':str(binary),'sha256':digest(binary),'version':subprocess.check_output([binary,'--version'],text=True).splitlines()[0]},
         'link_record':links[0],
         'compiler':subprocess.check_output(['gcc','--version'],text=True).splitlines()[0]}
(ROOT/f'evidence/{name}-build-profile.json').write_text(json.dumps(profile,indent=2)+'\n')
print('Built pinned',name,pin['version'],'native oracle and captured its link inputs')
