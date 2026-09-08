#!/usr/bin/env python3
"""Verify Less replay, production terminal, ownership and build evidence."""
# SPDX-License-Identifier: GPL-3.0-or-later
import importlib.util
import json
from pathlib import Path
import re
import sys

ROOT = Path(__file__).resolve().parents[1]
sys.path[:0] = [str(ROOT/'tests'), str(ROOT/'tests/gnu')]
from comparison_profile import fingerprint
from less_fixture import describe
spec = importlib.util.spec_from_file_location('reviewed', ROOT/'tests/gnu/reviewed-original.py')
runner = importlib.util.module_from_spec(spec)
spec.loader.exec_module(runner)
target = ROOT/'evidence/less-keyboard-validation.json'
assert not target.exists(), 'preserve prior audits'
names = ['less-screen-keyboard-original', 'less-screen-initial-original', 'less-keyboard-focused',
         'less-keyboard-terminal', 'less-keyboard-contract', 'less-keyboard-rebuild',
         'less-keyboard-smoke', 'less-keyboard-full-dispatch', 'less-keyboard-fixtures-build',
         'less-original-fixtures-build', 'less-native-cleanup', 'less-link']
reports = {n: json.loads((ROOT/'evidence'/(n+'.json')).read_text()) for n in names}
replay = reports['less-screen-keyboard-original']
focused = reports['less-keyboard-focused']
terminal = reports['less-keyboard-terminal']
build = reports['less-keyboard-fixtures-build']
production_hash = fingerprint(Path(focused['binary']))
assert focused['binary_sha256'] == terminal['binary_sha256'] == production_hash
assert replay['binary_sha256'] == build['binary_sha256'] == fingerprint(Path(build['binary']))
assert build['keyboard_cleanup'] and build['translated_entry_identical']
source_pin = json.loads((ROOT/'inventory/sources.json').read_text())['less']
source = Path(source_pin['source'])
for name, expected in source_pin['source_and_header_sha256'].items():
    assert fingerprint(source/name) == expected
build_archives = {fingerprint(p): p for p in [ROOT/'scripts/prepare-less-original-profile.py',
                  ROOT/'evidence/raw/less-initial-fixtures-build-driver.py']}
for name in ['less-keyboard-fixtures-build', 'less-original-fixtures-build']:
    profile = reports[name]
    assert profile['driver_sha256'] in build_archives
    for filename, expected in profile['inputs'].items():
        path = Path(filename)
        if path == ROOT/'scripts/prepare-less-original-profile.py':
            assert expected == profile['driver_sha256']
        else:
            assert fingerprint(path) == expected
    for filename, expected in {**profile['helper_inputs'], **profile['test_tools'], **profile['artifacts']}.items():
        assert fingerprint(Path(filename)) == expected
    assert fingerprint(Path(profile['oracle'])) == profile['oracle_sha256']
for filename, expected in replay['inputs'].items():
    assert fingerprint(Path(filename)) == expected
assert replay['driver_sha256'] == fingerprint(ROOT/'tests/less-original.py')
assert focused['driver_sha256'] == fingerprint(ROOT/'tests/entry-behavior.py')
assert terminal['driver_sha256'] == fingerprint(ROOT/'tests/less-terminal.py')
assert reports['less-screen-initial-original']['driver_sha256'] == fingerprint(ROOT/'evidence/raw/less-initial-original-driver.py')
manifest = json.loads((ROOT/'inventory/less-tests.json').read_text())
reviewed = {r['name']: r for r in manifest['replays'] if r['reviewed']}
for row in manifest['replays']:
    path = source/row['path']
    assert fingerprint(path) == row['sha256'] and describe(path.read_bytes()) == row['records']
assert replay['complete'] and replay['passed'] == replay['assertion_passes'] == replay['total'] == 17
assert {r['name'] for r in replay['results']} == set(reviewed)
processes = []
def memory_check(recorded, filename, expected, require_clean, scope):
    path = ROOT/filename
    assert fingerprint(path) == expected
    text = path.read_text()
    pids = set(re.findall(r'^==([0-9]+)==', text, re.M))
    assert len(pids) == 1
    parsed = runner.parse_memory_log(text, pids.pop(), exec_only=True)
    assert parsed['complete_exec_log'] and parsed['exec_images'] == 1
    assert all(recorded[k] == v for k, v in parsed.items())
    clean = parsed['errors'] == parsed['non_inherited_descriptors'] == 0 and not any(
        parsed['heap_bytes'].get(k, 0) for k in ('definitely lost', 'indirectly lost', 'possibly lost'))
    if require_clean:
        assert clean
    processes.append({'scope': scope, 'log': filename, 'sha256': expected, 'memory_clean': clean, **parsed})

for name in ['less-screen-keyboard-original', 'less-screen-initial-original']:
    for row in reports[name]['results']:
        assert row['assertions_pass']
        if name == 'less-screen-keyboard-original':
            assert row['pass'] and row['frames'] == reviewed[row['name']]['records']['frames']
        for key, outcome in row['outcomes'].items():
            assert outcome['status'] == 0 and outcome['assertions_pass']
            for stream, entry in outcome['streams'].items():
                path = ROOT/entry['path']
                assert fingerprint(path) == entry['sha256'] and path.read_bytes() == bytes.fromhex(outcome[stream])
            assert len(outcome['memory']) == (2 if key.endswith('-valgrind') else 0)
            for entry in outcome['memory']:
                memory_check(entry, entry['log'], entry['sha256'],
                             key == 'rboxc-valgrind' and name == 'less-screen-keyboard-original', name+'/'+key)
assert focused['complete'] and focused['passed'] == focused['total'] == 10
for row in focused['results']:
    assert row['pass'] and row['equivalent'] and row['memory_clean']
    reference = row['outcomes']['gnu']
    for key, outcome in row['outcomes'].items():
        assert all(outcome[k] == reference[k] for k in ('status', 'stdout', 'stderr', 'tree'))
        if 'memory' in outcome:
            memory_check(outcome['memory'], outcome['log'], outcome['log_sha256'],
                         key == 'rboxc-valgrind', 'production-focused/'+key)
assert terminal['passed'] == terminal['total'] == 1 and terminal['equivalent']
for key, outcome in terminal['outcomes'].items():
    assert outcome['status'] == 0 and outcome['quit_sent'] and outcome['terminal_restored']
    assert outcome['output'] == terminal['outcomes']['gnu']['output']
    assert fingerprint(ROOT/outcome['output_path']) == outcome['output_sha256']
    assert (ROOT/outcome['output_path']).read_bytes() == bytes.fromhex(outcome['output'])
    if outcome['memory']:
        memory_check(outcome['memory'], outcome['memory_log'], outcome['memory_log_sha256'],
                     key == 'rboxc-valgrind', 'production-terminal/'+key)
contract = reports['less-keyboard-contract']
assert contract['passed'] == contract['total'] == 18
assert contract['driver_sha256'] == fingerprint(ROOT/'tests/less-keyboard-contract.py')
assert contract['adapter_sha256'] == fingerprint(ROOT/'scripts/less_cleanup.py')
for filename, expected in contract['artifacts'].items():
    assert fingerprint(ROOT/filename) == expected
for row in contract['results']:
    assert row['pass']
    memory_check(row['memory'], row['log'], row['log_sha256'], True, 'ownership-contract/'+row['implementation'])
for name, total in [('less-keyboard-smoke', 428), ('less-keyboard-full-dispatch', 11)]:
    report = reports[name]
    assert report['passed'] == report['total'] == total and report['binary_sha256'] == production_hash
    assert all(r['pass'] for r in report['results'])
rebuild = reports['less-keyboard-rebuild']
assert rebuild['identical'] and rebuild['binary_sha256'] == rebuild['rebuild_sha256'] == production_hash
assert fingerprint(ROOT/rebuild['rebuild']) == production_hash
old_link_path = ROOT/'build/less-before-keyboard-cleanup/link.json'
old_link = json.loads(old_link_path.read_text())
link = reports['less-link']
assert old_link['rust_source_sha256'] == link['rust_source_sha256'] == fingerprint(ROOT/'src/generated/applet_less.rs')
changes = [p for p, h in link['helper_inputs'].items() if h != old_link['helper_inputs'][p]]
assert len(changes) == 1 and changes[0].endswith('-ttyin.o')
for filename, expected in link['helper_inputs'].items():
    assert fingerprint(ROOT/filename) == expected
for filename, expected in old_link['helper_inputs'].items():
    assert fingerprint(old_link_path.parent/Path(filename).name) == expected
cleanup = reports['less-native-cleanup']
assert fingerprint(Path(cleanup['adapted_source'])) == cleanup['adapted_sha256']
assert fingerprint(Path(cleanup['object'])) == cleanup['object_sha256']
assert fingerprint(ROOT/'scripts/less_cleanup.py') == cleanup['driver_sha256']
assert sum(p['scope'] == 'less-screen-keyboard-original/rboxc-valgrind' for p in processes) == 34
assert sum(p['scope'].startswith('production-') and p['scope'].endswith('/rboxc-valgrind') for p in processes) == 11
result = {'scope': 'All 17 reviewed unchanged screen replays pass (2022 screen assertions) with 34 clean '
          'standalone test-profile Less processes. Production candidate: ten focused comparisons and one '
          'private terminal check pass with 11 clean Less processes. Eighteen ownership contracts pass separately. '
          'Only the namespaced keyboard helper changed; Rust entry is unchanged. Native GNU descriptor findings '
          'and the initial failing candidate remain recorded. One hyperlink opener replay is pending; full GNU acceptance is open.',
          'binary': focused['binary'], 'binary_sha256': production_hash,
          'test_profile_binary': replay['binary'], 'test_profile_sha256': replay['binary_sha256'],
          'original_passed': 17, 'screen_assertions': sum(r['frames'] for r in replay['results']),
          'clean_replay_processes': 34, 'clean_production_processes': 11, 'ownership_contracts': 18,
          'smoke_passed': 428, 'dispatcher_passed': 11, 'byte_identical_rebuild': True,
          'reports': {n: fingerprint(ROOT/'evidence'/(n+'.json')) for n in names},
          'manifest_sha256': fingerprint(ROOT/'inventory/less-tests.json'), 'changed_helpers': changes,
          'build_driver_archives': {h: str(p.relative_to(ROOT)) for h, p in build_archives.items()},
          'old_link_sha256': fingerprint(old_link_path),
          'driver_sha256': fingerprint(Path(__file__)), 'results': processes}
target.write_text(json.dumps(result, indent=2)+'\n')
print('Audited 17 original replays, 11 production Less checks, 18 ownership contracts and identical rebuild')
