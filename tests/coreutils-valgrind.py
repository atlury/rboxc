#!/usr/bin/env python3
"""Initial Valgrind coverage; preserve retained/lost allocations separately."""
from concurrent.futures import ThreadPoolExecutor
import json
import os
from pathlib import Path
import re
import subprocess

ROOT = Path(__file__).resolve().parents[1]
from comparison_profile import ComparisonProfile
PROFILE = ComparisonProfile('valgrind')
BINARY = PROFILE.binary
rows = json.loads((ROOT/'evidence/translation.json').read_text())
environment = {**os.environ, 'LC_ALL':'C', 'LANGUAGE':'C'}
environment.pop('POSIXLY_CORRECT', None)
def check(row):
    log = PROFILE.logs/f'valgrind-{row["entry"]}.log'
    oracle = subprocess.run([row['name'], '--help'], executable=ROOT/'build/gnu-coreutils/src/coreutils',
                            capture_output=True, env=environment, timeout=15)
    result = subprocess.run(['valgrind', '--error-exitcode=97', '--leak-check=full',
                             '--show-leak-kinds=all', '--errors-for-leak-kinds=definite,indirect,possible',
                             '--track-fds=yes', '--log-file='+str(log), BINARY, row['name'], '--help'],
                            capture_output=True, env=environment, timeout=30)
    text = log.read_text()
    errors = re.search(r'ERROR SUMMARY: ([\d,]+) errors', text)
    descriptors = re.search(r'FILE DESCRIPTORS: (\d+) open \((\d+) (?:inherited|std)\)', text)
    retained_descriptors = int(descriptors[1])-int(descriptors[2]) if descriptors else None
    lost = {kind:int(match[1].replace(',','')) for kind in ['definitely lost','indirectly lost','possibly lost','still reachable']
            if (match := re.search(re.escape(kind)+r': ([\d,]+) bytes', text))}
    if 'All heap blocks were freed -- no leaks are possible' in text:
        lost = dict.fromkeys(['definitely lost','indirectly lost','possibly lost','still reachable'],0)
    return {'name':row['name'], 'scope':'--help only', 'active_rust':row['active_rust'],
            'status':result.returncode, 'oracle_status':oracle.returncode,
            'pass':(result.returncode, result.stdout, result.stderr) == (oracle.returncode, oracle.stdout, oracle.stderr)
                   and errors is not None and errors[1] == '0' and retained_descriptors == 0,
            'errors':int(errors[1].replace(',','')) if errors else None,
            'non_inherited_descriptors': retained_descriptors, 'heap_bytes':lost, 'log':str(log.relative_to(ROOT))}
with ThreadPoolExecutor(max_workers=4) as pool:
    results = list(pool.map(check, rows))
report = {**PROFILE.metadata(), 'scope':'help-path memory/descriptor smoke; normal and error-path coverage remains open',
          'passed':sum(row['pass'] for row in results), 'total':len(results),'results':results}
PROFILE.report.write_text(json.dumps(report,indent=2)+'\n')
print(f'GNU command Valgrind help paths: {report["passed"]}/{report["total"]} pass')
for row in results:
    if not row['pass']: print('FAIL', row['name'], row['status'], row['errors'])
raise SystemExit(report['passed'] != report['total'])
