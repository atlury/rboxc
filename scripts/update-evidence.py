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
time_reports = [ROOT/'evidence/time-original.json', ROOT/'evidence/time-behavior.json']
if all(path.exists() for path in time_reports):
    original, focused = [json.loads(path.read_text()) for path in time_reports]
    entry = read('evidence/time-translation.json')
    linked = read('evidence/time-link.json')
    manifest = read('inventory/time-tests.json')
    expected = {r['script']: r['source_sha256'] for r in manifest['scripts']}
    assert all(expected.get(r['script']) == r['source_sha256'] for r in original['results'])
    current = all(report['binary_sha256'] == binary_sha256 for report in (original, focused))
    source_current = (entry['rust_sha256'] == linked['rust_source_sha256'] ==
                      hashlib.sha256((ROOT/entry['rust_file']).read_bytes()).hexdigest())
    listed = subprocess.check_output([binary, '--list'], text=True).splitlines()
    active = current and source_current and entry['translated'] and 'time' in listed and not linked['native_command_entries']
    scripts_pass = active and {r['script'] for r in original['results'] if r['pass']} == set(expected)
    behavior_pass = active and focused['passed'] == focused['total'] and bool(focused['results'])
    help_checks = [r for r in focused['results'] if r['name'] in ('help', 'version')]
    assert {r['name'] for r in help_checks} == {'help', 'version'}
    row = next(r for r in inventory if r['name'] == 'time')
    row.update(translated=entry['translated'], compiles=active, active_rust=active,
               state='compiled-rust-entry' if active else 'queued',
               help_version_pass=active and all(r['pass'] for r in help_checks),
               valgrind_help_pass=active and all(r['memory_clean'] for r in help_checks),
               behavior_fixture_count=focused['total'], behavior_fixture_pass=behavior_pass,
               valgrind_fixture_pass=behavior_pass and all(r['memory_clean'] for r in focused['results']),
               gnu_tests_pass=scripts_pass, valgrind_pass=False, complete=False,
               completion_scope='Open: excluded original format test and instrumented max-RSS assertion. Memory evidence assesses Time exits; successful child exec boundaries remain separate.')
    extra_providers['time'] = {'active_rust_entries': int(active),
        'original_native_passed': original['native_passed'], 'original_scripts_passed': original['passed'],
        'original_scripts_executed': original['total'], 'original_scripts': len(expected),
        'behavior_passed': focused['passed'], 'behavior_total': focused['total'],
        'complete': False, 'completion_scope': row['completion_scope']}
which_report = ROOT/'evidence/which-behavior.json'
if which_report.exists():
    focused = json.loads(which_report.read_text())
    entry = read('evidence/which-translation.json')
    linked = read('evidence/which-link.json')
    manifest = read('inventory/which-tests.json')
    assert not manifest['scripts'], 'original test registration requires a new assessment'
    source_current = (entry['rust_sha256'] == linked['rust_source_sha256'] ==
                      hashlib.sha256((ROOT/entry['rust_file']).read_bytes()).hexdigest())
    listed = subprocess.check_output([binary, '--list'], text=True).splitlines()
    active = (focused['binary_sha256'] == binary_sha256 and source_current and entry['translated']
              and 'which' in listed and not linked['native_command_entries'])
    behavior_pass = active and focused['passed'] == focused['total'] and bool(focused['results'])
    help_checks = [r for r in focused['results'] if r['name'] in ('help', 'version')]
    assert {r['name'] for r in help_checks} == {'help', 'version'}
    row = next(r for r in inventory if r['name'] == 'which')
    row.update(translated=entry['translated'], compiles=active, active_rust=active,
               state='compiled-rust-entry' if active else 'queued',
               help_version_pass=active and all(r['pass'] for r in help_checks),
               valgrind_help_pass=active and all(r['memory_clean'] for r in help_checks),
               behavior_fixture_count=focused['total'], behavior_fixture_pass=behavior_pass,
               valgrind_fixture_pass=behavior_pass and all(r['memory_clean'] for r in focused['results']),
               gnu_tests_pass=False, valgrind_pass=False, complete=False,
               completion_scope='Focused compatibility coverage; the source distribution registers no runtime test suite. No full-suite completion claim.')
    extra_providers['which'] = {'active_rust_entries': int(active), 'original_scripts': 0,
        'behavior_passed': focused['passed'], 'behavior_total': focused['total'],
        'complete': False, 'completion_scope': row['completion_scope']}
diffutils_reports = [ROOT/'evidence/diffutils-original.json', ROOT/'evidence/diffutils-behavior.json']
if all(path.exists() for path in diffutils_reports):
    original, focused = [json.loads(path.read_text()) for path in diffutils_reports]
    linked = read('evidence/diffutils-link.json')
    manifest = read('inventory/diffutils-tests.json')
    expected = {r['script']: r['source_sha256'] for r in manifest['scripts']}
    assert all(expected.get(r['script']) == r['source_sha256'] for r in original['results'])
    current = all(report['binary_sha256'] == binary_sha256 for report in (original, focused))
    listed = subprocess.check_output([binary, '--list'], text=True).splitlines()
    active_entries = 0
    for command in ('cmp', 'diff', 'diff3', 'sdiff'):
        entry = read(f'evidence/diffutils-{command}-translation.json')
        source_current = (entry['rust_sha256'] == linked['rust_source_sha256'][command] ==
                          hashlib.sha256((ROOT/entry['rust_file']).read_bytes()).hexdigest())
        active = current and source_current and entry['translated'] and command in listed and not linked['native_command_entries']
        checks = [r for r in focused['results'] if r['command'] == command]
        help_checks = [r for r in checks if r['name'] in (command+'-help', command+'-version')]
        assert len(help_checks) == 2
        behavior_pass = active and bool(checks) and all(r['pass'] for r in checks)
        row = next(r for r in inventory if r['name'] == command)
        row.update(translated=entry['translated'], compiles=active, active_rust=active,
                   provider_confirmed=True, state='compiled-rust-entry' if active else 'queued',
                   help_version_pass=active and all(r['pass'] for r in help_checks),
                   valgrind_help_pass=active and all(r['memory_clean'] for r in help_checks),
                   behavior_fixture_count=len(checks), behavior_fixture_pass=behavior_pass,
                   valgrind_fixture_pass=behavior_pass and all(r['memory_clean'] for r in checks),
                   gnu_tests_pass=False, valgrind_pass=False, complete=False,
                   completion_scope='Pinned GNU Diffutils 3.12 Linux profile. Provider suite retains three excluded originals, a prerequisite skip, an upstream expected failure, and an instrumented cmp deadline assertion; no full-suite completion claim.')
        active_entries += int(active)
    extra_providers['diffutils'] = {'active_rust_entries': active_entries,
        'original_native_passed': original['native_passed'], 'original_scripts_passed': original['passed'],
        'original_scripts_executed': original['total'], 'original_scripts': len(expected),
        'original_states': original['state_counts'], 'behavior_passed': focused['passed'],
        'behavior_total': focused['total'], 'complete': False, 'completion_scope': row['completion_scope']}
grep_reports = [ROOT/'evidence/grep-original.json', ROOT/'evidence/grep-behavior.json']
if all(path.exists() for path in grep_reports):
    original, focused = [json.loads(path.read_text()) for path in grep_reports]
    entry = read('evidence/grep-translation.json')
    linked = read('evidence/grep-link.json')
    manifest = read('inventory/grep-tests.json')
    expected = {r['script']: r['source_sha256'] for r in manifest['scripts']}
    assert all(expected.get(r['script']) == r['source_sha256'] for r in original['results'])
    original_current = original['binary_sha256'] == binary_sha256
    retained_verified = False
    reuse_path = ROOT/'evidence/grep-original-reuse.json'
    if not original_current and reuse_path.exists():
        reuse = json.loads(reuse_path.read_text())
        retained_verified = (reuse['binary_sha256'] == binary_sha256 and reuse['passed'] == reuse['total'] == 1
                             and reuse['original_observations']['sha256'] == hashlib.sha256(grep_reports[0].read_bytes()).hexdigest())
        for name, expected_hash in {**reuse['unchanged_compiler_inputs'], **reuse['unchanged_provider_inputs']}.items():
            retained_verified = retained_verified and hashlib.sha256((ROOT/name).read_bytes()).hexdigest() == expected_hash
        retained_verified = retained_verified and all(not (ROOT/name).exists() for name in reuse['absent_compiler_configs'])
        focused_source = ROOT/reuse['focused_observations']['path']
        retained_verified = retained_verified and hashlib.sha256(focused_source.read_bytes()).hexdigest() == reuse['focused_observations']['sha256']
    current = focused['binary_sha256'] == binary_sha256 and (original_current or retained_verified)
    source_current = (entry['rust_sha256'] == linked['rust_source_sha256'] ==
                      hashlib.sha256((ROOT/entry['rust_file']).read_bytes()).hexdigest())
    listed = subprocess.check_output([binary, '--list'], text=True).splitlines()
    active_entries = 0
    for command in ('grep', 'egrep', 'fgrep'):
        active = current and source_current and entry['translated'] and command in listed and not linked['native_command_entries']
        checks = [r for r in focused['results'] if r['command'] == command]
        help_names = ('help', 'version') if command == 'grep' else (command+'-help', command+'-version')
        help_checks = [r for r in checks if r['name'] in help_names]
        assert len(help_checks) == 2
        behavior_pass = active and bool(checks) and all(r['pass'] for r in checks)
        row = next(r for r in inventory if r['name'] == command)
        row.update(translated=True, compiles=active, active_rust=active, provider_confirmed=True,
                   state='compiled-rust-entry' if active else 'queued',
                   help_version_pass=active and all(r['pass'] for r in help_checks),
                   valgrind_help_pass=active and all(r['memory_clean'] for r in help_checks),
                   behavior_fixture_count=len(checks), behavior_fixture_pass=behavior_pass,
                   valgrind_fixture_pass=behavior_pass and all(r['memory_clean'] for r in checks),
                   gnu_tests_pass=False, valgrind_pass=False, complete=False,
                   completion_scope='Pinned GNU Grep 3.12 Linux/glibc/PCRE2 profile. Reviewed originals and focused comparisons pass; one partial Perl script, excluded originals, and pending resource profiles prevent a full-suite completion claim.')
        active_entries += int(active)
    extra_providers['grep'] = {'active_rust_entries':active_entries,
        'original_binary_sha256':original['binary_sha256'],
        'originals_on_current_binary':original_current,'retained_original_inputs_verified':retained_verified,
        'original_native_passed':original['native_passed'],'original_selections_passed':original['passed'],
        'original_selections_executed':original['total'],'original_scripts':len(expected),
        'full_original_scripts_passed':original['full_original_scripts_passed'],
        'partial_original_scripts':original['partial_original_scripts'],
        'selected_perl_cases':original['selected_perl_cases'],
        'original_states':original['state_counts'],'behavior_passed':focused['passed'],
        'behavior_total':focused['total'],'complete':False,'completion_scope':row['completion_scope']}
gzip_reports = [ROOT/'evidence/gzip-original.json', ROOT/'evidence/gzip-behavior.json']
if all(path.exists() for path in gzip_reports):
    original, focused = [json.loads(path.read_text()) for path in gzip_reports]
    entry = read('evidence/gzip-translation.json')
    linked = read('evidence/gzip-link.json')
    manifest = read('inventory/gzip-tests.json')
    expected = {r['script']: r for r in manifest['scripts'] if r['reviewed']}
    assert set(expected) == {r['script'] for r in original['results']}
    assert all(expected[r['script']]['source_sha256'] == r['source_sha256'] and
               expected[r['script']].get('built_programs') == r.get('built_programs')
               for r in original['results'])
    current = all(report['binary_sha256'] == binary_sha256 for report in (original, focused))
    source_current = (entry['rust_sha256'] == linked['rust_source_sha256'] ==
                      hashlib.sha256((ROOT/entry['rust_file']).read_bytes()).hexdigest())
    listed = subprocess.check_output([binary, '--list'], text=True).splitlines()
    active_entries = 0
    for command in ('gzip', 'gunzip', 'uncompress', 'zcat'):
        active = current and source_current and entry['translated'] and command in listed and not linked['native_command_entries']
        checks = [r for r in focused['results'] if r['command'] == command]
        help_checks = [r for r in checks if r['name'] in (command+'-help', command+'-version')]
        assert len(help_checks) == 2
        behavior_pass = active and bool(checks) and all(r['pass'] for r in checks)
        row = next(r for r in inventory if r['name'] == command)
        row.update(translated=entry['translated'], compiles=active, active_rust=active,
                   provider_confirmed=True, state='compiled-rust-entry' if active else 'queued',
                   help_version_pass=active and all(r['pass'] for r in help_checks),
                   valgrind_help_pass=active and all(r['memory_clean'] for r in help_checks),
                   behavior_fixture_count=len(checks), behavior_fixture_pass=behavior_pass,
                   valgrind_fixture_pass=behavior_pass and all(r['memory_clean'] for r in checks),
                   gnu_tests_pass=False, valgrind_pass=False, complete=False,
                   completion_scope='GNU Gzip 1.14 Linux/glibc with configured Bash alias diagnostics. Sixteen full originals and one three-program help/version selection pass; auxiliary ports and remaining originals are open.')
        active_entries += int(active)
    extra_providers['gzip'] = {'active_rust_entries': active_entries,
        'original_native_passed': original['native_passed'], 'original_selections_passed': original['passed'],
        'original_selections_executed': original['total'], 'original_scripts': len(manifest['scripts']),
        'full_original_scripts_passed': original['full_original_scripts_passed'],
        'partial_original_scripts': original['partial_original_scripts'],
        'original_states': original['state_counts'], 'behavior_passed': focused['passed'],
        'behavior_total': focused['total'], 'complete': False, 'completion_scope': row['completion_scope']}
sed_reports = [ROOT/'evidence/sed-original.json', ROOT/'evidence/sed-behavior.json']
if all(path.exists() for path in sed_reports):
    original, focused = [json.loads(path.read_text()) for path in sed_reports]
    entry = read('evidence/sed-translation.json')
    linked = read('evidence/sed-link.json')
    manifest = read('inventory/sed-tests.json')
    expected = {r['script']: r for r in manifest['scripts'] if r['reviewed']}
    assert set(expected) == {r['script'] for r in original['results']}
    assert all(expected[r['script']]['source_sha256'] == r['source_sha256'] for r in original['results'])
    current = all(r['binary_sha256'] == binary_sha256 for r in (original, focused))
    source_current = entry['rust_sha256'] == linked['rust_source_sha256'] == hashlib.sha256((ROOT/entry['rust_file']).read_bytes()).hexdigest()
    listed = subprocess.check_output([binary, '--list'], text=True).splitlines()
    active = current and source_current and entry['translated'] and 'sed' in listed and not linked['native_command_entries']
    checks = focused['results']; help_checks = [r for r in checks if r['name'] in ('help','version')]
    assert len(help_checks) == 2
    row = next(r for r in inventory if r['name'] == 'sed')
    row.update(translated=entry['translated'], compiles=active, active_rust=active,
        provider_confirmed=True, state='compiled-rust-entry' if active else 'queued',
        help_version_pass=active and all(r['pass'] for r in help_checks),
        valgrind_help_pass=active and all(r['memory_clean'] for r in help_checks),
        behavior_fixture_count=len(checks), behavior_fixture_pass=active and all(r['pass'] for r in checks),
        valgrind_fixture_pass=active and all(r['memory_clean'] for r in checks),
        gnu_tests_pass=False, valgrind_pass=False, complete=False,
        completion_scope='GNU Sed 4.10 Linux/glibc reviewed original selections and focused comparisons. External child findings, platform skips, excluded originals, and dedicated remaining profiles are recorded separately; full-suite completion is not claimed.')
    extra_providers['sed'] = {'active_rust_entries':int(active),'original_native_passed':original['native_passed'],
        'original_selections_passed':original['passed'],'original_selections_executed':original['total'],
        'original_scripts':len(manifest['scripts']),'original_states':original['state_counts'],
        'sed_assertions_and_memory_passed':original['sed_assertions_and_memory_passed'],
        'behavior_passed':focused['passed'],'behavior_total':focused['total'],
        'complete':False,'completion_scope':row['completion_scope']}
bc_reports = [ROOT/'evidence/bc-original.json', ROOT/'evidence/bc-behavior.json', ROOT/'evidence/bc-terminal.json']
if all(path.exists() for path in bc_reports):
    original, focused, terminal = [json.loads(path.read_text()) for path in bc_reports]
    linked = read('evidence/bc-link.json'); manifest = read('inventory/bc-tests.json')
    expected = {r['path']:r for r in manifest['inputs'] if r['state']=='reviewed'}
    assert set(expected) == {r['path'] for r in original['results']}
    assert all(all(r[k]==v for k,v in expected[r['path']].items()) for r in original['results'])
    current = all(r['binary_sha256'] == binary_sha256 for r in (original, focused, terminal))
    listed = subprocess.check_output([binary, '--list'], text=True).splitlines(); active_entries = 0
    for command in ('bc','dc'):
        entry = read('evidence/bc-'+command+'-translation.json')
        source_current = entry['rust_sha256'] == linked['rust_source_sha256'][command] == hashlib.sha256((ROOT/entry['rust_file']).read_bytes()).hexdigest()
        active = current and source_current and entry['translated'] and command in listed and not linked['native_command_entries']
        checks = [r for r in focused['results'] if r['command']==command]
        help_checks = [r for r in checks if r['name'] in (command+'-help',command+'-version')]
        assert len(help_checks)==2
        row = next(r for r in inventory if r['name']==command)
        row.update(translated=entry['translated'], compiles=active, active_rust=active,
            provider_confirmed=True, state='compiled-rust-entry' if active else 'queued',
            help_version_pass=active and all(r['pass'] for r in help_checks),
            valgrind_help_pass=active and all(r['memory_clean'] for r in help_checks),
            behavior_fixture_count=len(checks), behavior_fixture_pass=active and all(r['pass'] for r in checks),
            valgrind_fixture_pass=active and all(r['memory_clean'] for r in checks),
            gnu_tests_pass=False, valgrind_pass=False, complete=False,
            completion_scope='GNU BC 1.08.2 bc/dc entries with separate native helpers. All reviewed historical arithmetic inputs, focused checks, and four controlling-terminal profiles pass; no automated upstream runtime suite is registered. Broader terminal, locale, and signal profiles remain open.')
        active_entries += int(active)
    extra_providers['bc'] = {'active_rust_entries':active_entries,'original_inputs_passed':original['passed'],
        'original_inputs_executed':original['total'],'registered_runtime_tests':original['registered_runtime_tests'],
        'behavior_passed':focused['passed'],'behavior_total':focused['total'],
        'terminal_passed':terminal['passed'],'terminal_total':terminal['total'],
        'complete':False,'completion_scope':row['completion_scope']}
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
