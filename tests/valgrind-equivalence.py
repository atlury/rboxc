#!/usr/bin/env python3
"""Assess saved instrumented observations separately from native arithmetic."""
# SPDX-License-Identifier: GPL-3.0-or-later
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
observations = json.loads((ROOT/'evidence/behavior.json').read_text())
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
report = {'scope': 'assessment of saved GNU/candidate Valgrind observations; native arithmetic checked separately',
          'passed': sum(row['pass'] for row in results), 'total': len(results), 'results': results}
(ROOT/'evidence/valgrind-equivalence.json').write_text(json.dumps(report, indent=2)+'\n')
print(f"Instrumented equivalence: {report['passed']}/{report['total']} pass")
for row in results:
    if not row['pass']:
        print('OPEN', row['name'], row['arguments'], row['differences'])
raise SystemExit(report['passed'] != report['total'])
