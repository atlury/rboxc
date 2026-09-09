#!/usr/bin/env python3
"""Close only the cache-advice test stub's marker stream before it returns."""
# SPDX-License-Identifier: GPL-3.0-or-later
import hashlib
import json
import os
from pathlib import Path
import sys
config=json.loads(Path(sys.argv[1]).read_text())
args=sys.argv[2:]
run=Path(config['run']).resolve(strict=True)
assert Path.cwd().resolve().is_relative_to(run)
compiler=Path(config['compiler'])
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
assert sha(compiler)==config['compiler_sha256']
assert args[:5]==['-Wall','-shared','--std=gnu99','-fPIC','-O2']
if 'k.c' in args:
    assert args[5:8]==['k.c','-o','k.so'] and args[8:] in ([],['-ldl'])
    source=Path('k.c');original=source.read_bytes()
    assert sha(source)==config['source_sha256']
    before=b'  fopen ("called", "w");'
    after=b'  FILE *marker = fopen ("called", "w");\n  if (marker) fclose (marker);'
    assert original.count(before)==1
    changed=original.replace(before,after)
    output=Path('k.marker-closed.c');assert not output.exists()
    output.write_bytes(changed)
    record={'original_source':original.decode(),'adapted_source':changed.decode(),
            'original_sha256':sha(source),'adapted_sha256':sha(output),
            'original_arguments':args,'fixture':str(Path.cwd())}
    args=[str(output) if a=='k.c' else a for a in args]
    record['effective_arguments']=args
    journal=run/'dd-marker-profile.json'
    assert not journal.exists();journal.write_text(json.dumps(record,indent=2)+'\n')
else:
    assert args[5:]==['-xc','-','-o','d.so','-ldl'] or args[5:]==['-xc','-','-o','d.so']
os.execv(compiler,[str(compiler),*args])
