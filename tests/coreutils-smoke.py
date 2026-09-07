#!/usr/bin/env python3
"""Check every registered GNU command through both multicall entry forms."""
from concurrent.futures import ThreadPoolExecutor
import json
import os
from pathlib import Path
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[1]
from comparison_profile import ComparisonProfile
PROFILE = ComparisonProfile('smoke')
BINARY = PROFILE.binary
GNU = ROOT/'build/gnu-coreutils/src/coreutils'
environment = {**os.environ, 'LC_ALL':'C', 'LANGUAGE':'C'}
environment.pop('POSIXLY_CORRECT', None)
rows = json.loads((ROOT/'evidence/translation.json').read_text())
results = []
with tempfile.TemporaryDirectory(prefix='rboxc-smoke-') as work:
    directory = Path(work)
    for row in rows:
        (directory/row['name']).symlink_to(BINARY)
    def run_case(case):
        name, option, form = case
        argv0 = name if form == 'multicall' else str(directory/name)
        expected = subprocess.run([argv0, option], executable=GNU, capture_output=True,
                                  env=environment, timeout=15)
        args = [BINARY, name, option] if form == 'multicall' else [directory/name, option]
        actual = subprocess.run(args, capture_output=True, env=environment, timeout=15)
        matches = (expected.returncode, expected.stdout, expected.stderr) == (actual.returncode, actual.stdout, actual.stderr)
        result = {'name':name, 'option':option, 'entry_form':form, 'pass':matches,
                  'expected_status':expected.returncode, 'actual_status':actual.returncode}
        if not matches:
            result.update({key:value.decode(errors='backslashreplace') for key,value in
                          [('expected_stdout',expected.stdout), ('actual_stdout',actual.stdout),
                           ('expected_stderr',expected.stderr), ('actual_stderr',actual.stderr)]})
        return result
    cases = [(row['name'], option, form) for row in rows
             for option in ['--help','--version'] for form in ['multicall','symlink']]
    with ThreadPoolExecutor(max_workers=4) as pool:
        results = list(pool.map(run_case, cases))
report = {**PROFILE.metadata(), 'scope':'help/version smoke only; not full applet certification',
          'passed':sum(row['pass'] for row in results), 'total':len(results), 'results':results}
PROFILE.report.write_text(json.dumps(report,indent=2)+'\n')
print(f'GNU multicall help/version: {report["passed"]}/{report["total"]} pass')
for row in results:
    if not row['pass']: print('FAIL', row['name'], row['option'], row['entry_form'])
raise SystemExit(report['passed'] != report['total'])
