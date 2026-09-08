#!/usr/bin/env python3
"""Build a separate GNU C oracle for the Bash entry control-flow lowering."""
# SPDX-License-Identifier: GPL-3.0-or-later
import json,subprocess
from pathlib import Path
from split_entry import clean_args
from entry_provider_helpers import original_link,fingerprint
ROOT=Path(__file__).resolve().parents[1]
stage=ROOT/'build/translation/bash';source=Path('/opt/src/bash-5.3/shell.c')
records=[json.loads(p.read_text()) for p in (ROOT/'build/bash-cc-records').glob('*.json')]
records=[r for r in records if r.get('file')==str(source)];assert len(records)==1
record=records[0];args=clean_args(record)
outlined=stage/'outlined.c';obj=stage/'gated-oracle-check.o'
args=[str(outlined) if (Path(record['directory'])/s).resolve()==source else s for s in args]
args+=['-iquote',str(source.parent),'-c','-o',str(obj)]
link=original_link(ROOT,'bash');output=stage/'gated-bash-check'
command=list(link['arguments']);entry=ROOT/'build/gnu-bash/shell.o'
command=[str(obj) if (Path(link['directory'])/s).resolve()==entry else s for s in command]
command[command.index('-o')+1]=str(output)
with (stage/'gated-oracle-check-build.log').open('w') as log:
 subprocess.run(['gcc',*args],cwd=record['directory'],stdout=log,stderr=subprocess.STDOUT,check=True)
 subprocess.run(command,cwd=link['directory'],stdout=log,stderr=subprocess.STDOUT,check=True)
(ROOT/'evidence/bash-gated-oracle-build.json').write_text(json.dumps({'scope':'Independent C lowering oracle; this executable is never used as the multicall command implementation.','source_sha256':fingerprint(source),'outlined_sha256':fingerprint(outlined),'driver_sha256':fingerprint(Path(__file__)),'compile_arguments':['gcc',*args],'link_arguments':command,'binary':str(output),'binary_sha256':fingerprint(output)},indent=2)+'\n')
print(output)
