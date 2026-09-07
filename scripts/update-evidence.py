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
binary = ROOT/'target/release/rboxc'
binary_sha256 = hashlib.sha256(binary.read_bytes()).hexdigest()
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
extra_providers = {}
hello_reports = [ROOT/'evidence/hello-original.json', ROOT/'evidence/hello-behavior.json']
if all(path.exists() for path in hello_reports):
    original, focused = [json.loads(path.read_text()) for path in hello_reports]
    entry = read('evidence/hello-translation.json')
    linked = read('evidence/hello-link.json')
    manifest = read('inventory/hello-tests.json')
    expected = {row['script']: row['source_sha256'] for row in manifest['scripts']}
    assert all(expected.get(row['script']) == row['source_sha256'] for row in original['results'])
    current = all(report['binary_sha256'] == binary_sha256 for report in (original, focused))
    source_current = (entry['rust_sha256'] == linked['rust_source_sha256'] ==
                      hashlib.sha256((ROOT/entry['rust_file']).read_bytes()).hexdigest())
    listed = subprocess.check_output([binary, '--list'], text=True).splitlines()
    active = current and source_current and entry['translated'] and 'hello' in listed and not linked['native_command_entries']
    scripts_pass = active and {row['script'] for row in original['results'] if row['pass']} == set(expected)
    behavior_pass = active and focused['passed'] == focused['total'] and bool(focused['results'])
    memory_pass = scripts_pass and behavior_pass and all(row['memory_clean'] for row in focused['results'])
    complete = scripts_pass and behavior_pass and memory_pass
    help_checks = [r for r in focused['results'] if r['name'] in ('help', 'version')]
    assert {r['name'] for r in help_checks} == {'help', 'version'}
    row = next(row for row in inventory if row['name'] == 'hello')
    row.update(translated=entry['translated'], compiles=active, active_rust=active,
               state='compiled-rust-entry' if active else 'queued',
               help_version_pass=active and all(r['pass'] for r in help_checks),
               valgrind_help_pass=active and all(r['memory_clean'] for r in help_checks),
               behavior_fixture_count=focused['total'], behavior_fixture_pass=behavior_pass,
               valgrind_fixture_pass=memory_pass, gnu_tests_pass=scripts_pass,
               valgrind_pass=memory_pass, complete=complete,
               completion_scope='GNU Hello 2.12.3 on the recorded Linux x86-64/glibc profile; includes a declared fixed-calendar input for the original long-greeting test.')
    extra_providers['hello'] = {'active_rust_entries': int(active), 'original_scripts_passed':
        len({r['script'] for r in original['results'] if r['pass']}), 'original_scripts': len(expected),
        'ambient_calendar_skips': original['skipped'], 'behavior_passed': focused['passed'],
        'behavior_total': focused['total'], 'complete': complete, 'completion_scope': row['completion_scope']}
(ROOT/'inventory/applets.json').write_text(json.dumps(inventory, indent=2)+'\n')
reviewed_valgrind = read('evidence/gnu-reviewed-valgrind.json')
assessments = {'clean': 0, 'assertions_passed_memory_open': 0,
               'prerequisite_skip': 0, 'assertions_open': 0, 'interrupted': 0}
for row in reviewed_valgrind['results']:
    statuses = [row[implementation]['status'] for implementation in ('gnu', 'rboxc')]
    assertions_pass = statuses == [0, 0] and all(
        row[implementation].get('case_count_pass', True) for implementation in ('gnu', 'rboxc'))
    if row['pass']:
        assert assertions_pass, 'clean memory evidence requires passing original assertions'
        assert not row.get('execution_interruption'), 'interrupted execution is not a clean pass'
        assessment = 'clean'
    elif row.get('execution_interruption') or 124 in statuses:
        # The runner's GNU timeout watchdog uses 124 for an expired deadline.
        assessment = 'interrupted'
    elif statuses == [77, 77]:
        assessment = 'prerequisite_skip'
    elif assertions_pass:
        assessment = 'assertions_passed_memory_open'
    else:
        assessment = 'assertions_open'
    assessments[assessment] += 1
assert sum(assessments.values()) == reviewed_valgrind['total']
assert assessments['clean'] == reviewed_valgrind['passed']
summary = {
    'binary': {'path': str(binary.relative_to(ROOT)), 'bytes': binary.stat().st_size,
               'sha256': binary_sha256},
    'toolchain': subprocess.check_output(['rustc', '--version'], cwd=ROOT, text=True).strip(),
    'valgrind': subprocess.check_output(['valgrind', '--version'], text=True).strip(),
    'active_rust_entries': sum(r['active_rust'] for r in translation.values()) + sum(r['active_rust_entries'] for r in extra_providers.values()),
    'coreutils_rust_entries': sum(r['active_rust'] for r in translation.values()),
    'extra_providers': extra_providers,
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
        'passed_selections': reviewed_valgrind['passed'],
        'total_selections': reviewed_valgrind['total'],
        'selected_perl_cases': sum(row.get('expected_case_count', len(row.get('cases', []))) for row in reviewed_valgrind['results']),
        'candidate_processes': sum(len(row['rboxc']['memory']) for row in reviewed_valgrind['results']),
        'assessment_counts': assessments,
    },
    'allocation_adapter': read('evidence/aligned-alloc.json'),
    'descriptor_probe_adapter': {key: read('evidence/freopen-safer.json')[key]
                                 for key in ('passed', 'total')},
    'standard_stream_adapters': {key: read('evidence/standard-streams.json')[key]
                                 for key in ('passed', 'total')},
    'stdbuf_buffer_adapter': {key: read('evidence/stdbuf-lifetime.json')[key]
                               for key in ('passed', 'total', 'libc')},
    'runtime_helpers': read('evidence/link.json').get('runtime_helpers', []),
    'external_dependency_observations': len(read('evidence/host-dependency-findings.json')['results']),
    'dispatcher': {key: read('evidence/dispatcher.json')[key] for key in ('passed', 'total')},
    'gnu_suite': {key: read('evidence/gnu-suite-coverage.json')[key] for key in ('total', 'counts', 'valgrind_counts')},
    'complete_commands': sum(row['complete'] for row in inventory),
}
(ROOT/'evidence/status.json').write_text(json.dumps(summary, indent=2)+'\n')
