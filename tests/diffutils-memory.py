#!/usr/bin/env python3
"""Audit saved Diffutils logs for a complete final process image."""
# SPDX-License-Identifier: GPL-3.0-or-later
import argparse
import importlib.util
import json
from pathlib import Path
import sys
from comparison_profile import fingerprint

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT/'tests/gnu'))
spec = importlib.util.spec_from_file_location('reviewed', ROOT/'tests/gnu/reviewed-original.py')
runner = importlib.util.module_from_spec(spec); spec.loader.exec_module(runner)
parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('observations', type=Path, nargs='+')
parser.add_argument('--report', type=Path, required=True)
options = parser.parse_args()
results = []
reports = {}
for source in options.observations:
    data = json.loads(source.read_text()); reports[str(source)] = fingerprint(source)
    for case in data['results']:
        for log in case['outcomes']['rboxc-valgrind']['memory']:
            path = ROOT/log['log']
            assert fingerprint(path) == log['sha256']
            assessed = runner.parse_memory_log(path.read_text(), path.stem, exec_only=True)
            clean = (assessed['complete_exec_log'] and assessed['errors'] == 0
                and assessed['non_inherited_descriptors'] == 0
                and not any(assessed['heap_bytes'].get(k, 0) for k in
                            ('definitely lost', 'indirectly lost', 'possibly lost')))
            results.append({'source': str(source), 'case': case.get('script', case.get('name')),
                            'log': log['log'], 'sha256': log['sha256'], 'pass': clean, **assessed})
report = {'scope': 'Saved candidate memory logs: all final images must have matching-PID error and descriptor summaries after the last exec header. This does not convert failed or skipped original assertions into passes.',
    'observations': reports, 'driver_sha256': fingerprint(Path(__file__)),
    'passed': sum(r['pass'] for r in results), 'total': len(results), 'results': results}
options.report.write_text(json.dumps(report, indent=2)+'\n')
print(report['passed'], '/', report['total'], 'complete clean logs')
raise SystemExit(any(not row['pass'] for row in results))
