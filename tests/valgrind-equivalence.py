#!/usr/bin/env python3
"""Assess saved instrumented observations separately from native arithmetic."""
# SPDX-License-Identifier: GPL-3.0-or-later
import argparse
import hashlib
import json
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('--observations', type=Path, default=ROOT/'evidence/behavior.json')
parser.add_argument('--report-name')
options = parser.parse_args()
source = options.observations.resolve(strict=True)
if source != ROOT/'evidence/behavior.json' and not options.report_name:
    parser.error('alternate observations require --report-name')
if options.report_name and (not re.fullmatch(r'[a-z0-9][a-z0-9-]*', options.report_name)
                            or options.report_name in {'smoke', 'valgrind', 'dispatcher', 'behavior', 'valgrind-equivalence'}):
    parser.error('named assessments must use a separate report name')
destination = ROOT/'evidence'/((options.report_name or 'valgrind-equivalence')+'.json')
observations = json.loads(source.read_text())
results = []
for row in observations['results']:
    actual, expected = row['rboxc_valgrind'], row['gnu_valgrind']
    differences = [key for key in ('stdout', 'stderr', 'tree') if actual[key] != expected[key]]
    # GNU's own leak findings can replace its exit code with Valgrind's 97.
    # Require the candidate to preserve its native exit status and stay clean.
    if actual['status'] != row['rboxc']['status']:
        differences.append('native exit status')
    clean = actual['errors'] == 0 and actual['non_inherited_descriptors'] == 0
    results.append({'name': row['name'], 'arguments': row['arguments'],
                    'pass': not differences and clean, 'differences': differences,
                    'gnu_native_vs_instrumented_streams': [
                        key for key in ('stdout', 'stderr') if expected[key] != row['gnu'][key]]})
report = {'observations': str(source), 'observations_sha256': hashlib.sha256(source.read_bytes()).hexdigest(),
          'binary_sha256': observations.get('binary_sha256'), 'scope': 'assessment of saved GNU/candidate Valgrind observations; native arithmetic checked separately',
          'passed': sum(row['pass'] for row in results), 'total': len(results), 'results': results}
destination.write_text(json.dumps(report, indent=2)+'\n')
print(f"Instrumented equivalence: {report['passed']}/{report['total']} pass")
for row in results:
    if not row['pass']:
        print('OPEN', row['name'], row['arguments'], row['differences'])
raise SystemExit(report['passed'] != report['total'])
