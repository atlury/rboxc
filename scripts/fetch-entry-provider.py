#!/usr/bin/env python3
"""Fetch fixed GNU release archives, verify signatures, and record source pins."""
# SPDX-License-Identifier: GPL-3.0-or-later
import argparse
import hashlib
import json
from pathlib import Path
import re
import subprocess
import tarfile
import urllib.request

ROOT=Path(__file__).resolve().parents[1]
RELEASES={'less':('704','gz',['less'],['main.c']),
          'screen':('5.0.2','gz',['screen'],['screen.c']),
          'wget':('1.25.0','gz',['wget'],['src/main.c']),
          'glibc':('2.43','xz',['getconf','iconv'],['posix/getconf.c','iconv/iconv_prog.c'])}
parser=argparse.ArgumentParser(description=__doc__)
parser.add_argument('provider',choices=RELEASES)
name=parser.parse_args().provider
version,suffix,commands,entries=RELEASES[name]
archive=f'{name}-{version}.tar.{suffix}'
base=f'https://ftp.gnu.org/gnu/{name}/'
stage=ROOT/'build/downloads';stage.mkdir(exist_ok=True)
digest=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
for filename in (archive,archive+'.sig'):
    target=stage/filename
    if not target.exists():
        temporary=target.with_suffix(target.suffix+'.tmp')
        with urllib.request.urlopen(base+filename,timeout=60) as response:
            temporary.write_bytes(response.read())
        temporary.replace(target)
keyring=ROOT/'build/gnu-keyring.gpg'
result=subprocess.run(['gpgv','--keyring',str(keyring),'--status-fd','1',str(stage/(archive+'.sig')),str(stage/archive)],capture_output=True)
log=ROOT/f'evidence/raw/{name}-release-signature.log'
log.write_bytes(result.stdout+result.stderr)
assert result.returncode==0,'GNU release signature verification failed'
signers=re.findall(rb'\[GNUPG:\] VALIDSIG ([0-9A-F]+) ',result.stdout)
assert signers
source=Path('/opt/src')/(name+'-'+version)
if not source.exists():
    with tarfile.open(stage/archive) as data:
        assert all(m.name==source.name or m.name.startswith(source.name+'/') for m in data)
        data.extractall(source.parent,filter='data')
pin={'version':version,'source':str(source),'archive':base+archive,'archive_sha256':digest(stage/archive),
     'signature':base+archive+'.sig','signature_sha256':digest(stage/(archive+'.sig')),
     'signer_fingerprint':signers[0].decode(),'signature_evidence':str(log.relative_to(ROOT)),
     'signature_evidence_sha256':digest(log),'keyring_sha256':digest(keyring),'commands':commands,
     'entry_source_sha256':{p:digest(source/p) for p in entries},
     'source_and_header_sha256':{str(p.relative_to(source)):digest(p) for p in sorted(source.rglob('*')) if p.is_file() and p.suffix in ('.c','.h','.y')},
     'scope':'Verified fixed GNU release source baseline; pinning alone is not a translated or tested command.'}
# Workers write separate results; the caller merges into the shared inventory.
(ROOT/f'build/{name}-source-pin.json').write_text(json.dumps(pin,indent=2)+'\n')
print('Verified GNU',name,version,'source signature and hashes')
