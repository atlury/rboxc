#!/usr/bin/env python3
"""Record compilation and final linker arguments for new GNU providers."""
# SPDX-License-Identifier: GPL-3.0-or-later
import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys

args = sys.argv[1:]
directory = Path(os.environ['RBOXC_CC_RECORDS'])
directory.mkdir(parents=True, exist_ok=True)
sources = [arg for arg in args if arg.endswith('.c') and not arg.startswith('-')]
if '-o' in args or ('-c' in args and len(sources) == 1):
    output = args[args.index('-o')+1] if '-o' in args else Path(sources[0]).stem+'.o'
    kind = 'compile' if '-c' in args else 'link'
    key = hashlib.sha256((os.getcwd()+'\0'+kind+'\0'+output).encode()).hexdigest()
    record = {'directory':os.getcwd(), 'arguments':['gcc',*args], 'output':output, 'kind':kind}
    if kind=='compile':
        assert len(sources)==1
        record['file']=str(Path(sources[0]).resolve())
    temporary = directory/f'{key}.{os.getpid()}.tmp'
    temporary.write_text(json.dumps(record,indent=2)+'\n')
    temporary.replace(directory/(key+'.json'))
raise SystemExit(subprocess.call(['gcc',*args]))
