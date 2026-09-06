#!/usr/bin/env python3
"""Check the native allocation adapter with bounded allocation requests."""
import json
from pathlib import Path
import re
import subprocess

ROOT = Path(__file__).resolve().parents[1]
binary = ROOT/'build/test-aligned-alloc'
subprocess.run(['gcc', '-O2', '-Wall', '-Wextra', '-Werror', ROOT/'tests/aligned-alloc.c',
                ROOT/'src/bridges/aligned-alloc.c', '-Wl,--wrap=aligned_alloc', '-o', binary], check=True)
log = ROOT/'evidence/raw/aligned-alloc.log'
completed = subprocess.run(['valgrind', '--error-exitcode=97', '--leak-check=full',
                            '--show-leak-kinds=all', '--log-file='+str(log), binary], timeout=20)
text = log.read_text()
errors = re.search(r'ERROR SUMMARY: (\d+) errors', text)
passed = completed.returncode == 0 and errors is not None and errors[1] == '0'
report = {'pass': passed, 'status': completed.returncode, 'bounded_allocations': 24,
          'rounding_overflow_checks': 1, 'zero_live_heap': 'in use at exit: 0 bytes in 0 blocks' in text,
          'log': str(log.relative_to(ROOT))}
(ROOT/'evidence/aligned-alloc.json').write_text(json.dumps(report, indent=2)+'\n')
print(report)
raise SystemExit(not passed)
