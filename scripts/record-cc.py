#!/usr/bin/env python3
"""Capture real compilation commands while building the pinned GNU sources."""
import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys

args = sys.argv[1:]
if '-c' in args:
    files = [arg for arg in args if arg.endswith('.c') and not arg.startswith('-')]
    if len(files) == 1:
        output = args[args.index('-o') + 1] if '-o' in args else files[0]
        key = hashlib.sha256((os.getcwd() + '\0' + output).encode()).hexdigest()
        directory = Path(os.environ['RBOXC_CC_RECORDS'])
        directory.mkdir(parents=True, exist_ok=True)
        record = {'directory': os.getcwd(), 'file': str(Path(files[0]).resolve()),
                  'arguments': ['gcc', *args], 'output': output}
        temporary = directory / f'{key}.{os.getpid()}.tmp'
        temporary.write_text(json.dumps(record, indent=2) + '\n')
        temporary.replace(directory / f'{key}.json')
raise SystemExit(subprocess.call(['gcc', *args]))
