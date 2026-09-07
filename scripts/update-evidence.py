#!/usr/bin/env python3
"""Update inventory progress without equating smoke coverage with completion."""
import hashlib
import json
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[1]
read = lambda path: json.loads((ROOT/path).read_text())
translation = {row['name']: row for row in read('evidence/translation.json')}
smoke = read('evidence/smoke.json')
valgrind = read('evidence/valgrind.json')
behavior = read('evidence/behavior.json')
inventory = read('inventory/applets.json')
for row in inventory:
    if row['name'] not in translation:
        continue
    unit = translation[row['name']]
    checks = [r for r in behavior['results'] if r['name'] == row['name']]
    row.update(translated=unit['translated'], compiles=True, active_rust=unit['active_rust'],
               state='compiled-rust-entry' if unit['active_rust'] else 'temporary-C-entry',
               help_version_pass=all(r['pass'] for r in smoke['results'] if r['name'] == row['name']),
               valgrind_help_pass=all(r['pass'] for r in valgrind['results'] if r['name'] == row['name']),
               behavior_fixture_count=len(checks),
               behavior_fixture_pass=bool(checks) and all(r['behavior_pass'] for r in checks),
               valgrind_fixture_pass=bool(checks) and all(r['valgrind_pass'] for r in checks),
               gnu_tests_pass=False, valgrind_pass=False, complete=False)
(ROOT/'inventory/applets.json').write_text(json.dumps(inventory, indent=2)+'\n')
binary = ROOT/'target/release/rboxc'
summary = {
    'binary': {'path': str(binary.relative_to(ROOT)), 'bytes': binary.stat().st_size,
               'sha256': hashlib.sha256(binary.read_bytes()).hexdigest()},
    'toolchain': subprocess.check_output(['rustc', '--version'], cwd=ROOT, text=True).strip(),
    'valgrind': subprocess.check_output(['valgrind', '--version'], text=True).strip(),
    'active_rust_entries': sum(r['active_rust'] for r in translation.values()),
    'temporary_C_entries': [r['name'] for r in translation.values() if not r['active_rust']],
    'help_version': {'passed': smoke['passed'], 'total': smoke['total']},
    'valgrind_help': {'passed': valgrind['passed'], 'total': valgrind['total']},
    'behavior': {**{key: behavior[key] for key in ('behavior_passed', 'valgrind_passed', 'total')},
                 'commands_covered': len({row['name'] for row in behavior['results']})},
    'instrumented_equivalence': {key: read('evidence/valgrind-equivalence.json')[key]
                                 for key in ('passed', 'total')},
    'gnu_cp_original': read('evidence/gnu-cp-original.json')['counts'],
    'reviewed_original': {
        'passed_selections': read('evidence/gnu-reviewed-original.json')['passed'],
        'total_selections': read('evidence/gnu-reviewed-original.json')['total'],
        'selected_perl_cases': sum(row.get('expected_case_count', len(row.get('cases', []))) for row in read('evidence/gnu-reviewed-original.json')['results']),
        'full_perl_scripts': sum(bool(row.get('full_suite')) for row in read('evidence/gnu-reviewed-original.json')['results']),
        'shell_scripts': sum(row['script'].endswith('.sh') for row in read('evidence/gnu-reviewed-original.json')['results']),
    },
    'reviewed_original_valgrind': {
        'passed_selections': read('evidence/gnu-reviewed-valgrind.json')['passed'],
        'total_selections': read('evidence/gnu-reviewed-valgrind.json')['total'],
        'selected_perl_cases': sum(row.get('expected_case_count', len(row.get('cases', []))) for row in read('evidence/gnu-reviewed-valgrind.json')['results']),
        'candidate_processes': sum(len(row['rboxc']['memory']) for row in read('evidence/gnu-reviewed-valgrind.json')['results']),
    },
    'allocation_adapter': read('evidence/aligned-alloc.json'),
    'descriptor_probe_adapter': {key: read('evidence/freopen-safer.json')[key]
                                 for key in ('passed', 'total')},
    'standard_stream_adapters': {key: read('evidence/standard-streams.json')[key]
                                 for key in ('passed', 'total')},
    'runtime_helpers': read('evidence/link.json').get('runtime_helpers', []),
    'external_dependency_observations': len(read('evidence/host-dependency-findings.json')['results']),
    'dispatcher': {key: read('evidence/dispatcher.json')[key] for key in ('passed', 'total')},
    'gnu_suite': {key: read('evidence/gnu-suite-coverage.json')[key] for key in ('total', 'counts', 'valgrind_counts')},
    'complete_commands': 0,
}
(ROOT/'evidence/status.json').write_text(json.dumps(summary, indent=2)+'\n')
