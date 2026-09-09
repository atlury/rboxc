#!/usr/bin/env python3
"""Verify Tar child diagnostics, input ownership and complete child observations."""
# SPDX-License-Identifier: GPL-3.0-or-later
import importlib.util
import json
from pathlib import Path
import re
import sys

ROOT = Path(__file__).resolve().parents[1]
sys.path[:0] = [str(ROOT/'tests'), str(ROOT/'tests/gnu')]
from comparison_profile import fingerprint
spec = importlib.util.spec_from_file_location('reviewed', ROOT/'tests/gnu/reviewed-original.py')
runner = importlib.util.module_from_spec(spec)
spec.loader.exec_module(runner)
target = ROOT/'evidence/tar-child-input-validation.json'
assert not target.exists()
binary = ROOT/'target/tar-child-input-candidate/release/rboxc'
repro = ROOT/'target/tar-child-input-repro/release/rboxc'
assert fingerprint(binary) == fingerprint(repro)
before = ROOT/'evidence/raw/tar-before-child-prefix'
for row in json.loads((before/'manifest.json').read_text()):
    assert fingerprint(ROOT/row['preserved']) == row['sha256']
old_rust = (before/'applet_tar.rs').read_text()
new_rust = (ROOT/'src/generated/applet_tar.rs').read_text()
line = '    libc::fprintf(stderr.cast(), b"%s: \\0".as_ptr().cast(), RBOXC_INVOCATION);'
replacement = ('    let prefix = if program_name.is_null() { RBOXC_INVOCATION } else { program_name };\n'
               '    libc::fprintf(stderr.cast(), b"%s: \\0".as_ptr().cast(), prefix);')
assert old_rust.count(line) == 1 and old_rust.replace(line, replacement) == new_rust
translation = json.loads((ROOT/'evidence/tar-translation.json').read_text())
assert translation['raw_translation_sha256'] == json.loads((before/'tar-translation.json').read_text())['raw_translation_sha256']
assert translation['rust_sha256'] == fingerprint(ROOT/'src/generated/applet_tar.rs')
old_link = json.loads((ROOT/'evidence/raw/tar-before-child-input/tar-link.json').read_text())
new_link = json.loads((ROOT/'evidence/tar-link.json').read_text())
assert old_link['rust_source_sha256'] == new_link['rust_source_sha256'] == translation['rust_sha256']
changed = [p for p, h in new_link['helper_inputs'].items() if old_link['helper_inputs'][p] != h]
assert changed == ['build/helpers/tar-system.o']
for p, h in new_link['helper_inputs'].items():
    assert fingerprint(ROOT/p) == h
names = ['tar-pending-compression-original', 'tar-child-prefix-compression-original',
         'tar-child-input-compression-original', 'tar-child-input-compression-audited',
         'tar-child-prefix-behavior', 'tar-child-prefix-smoke', 'tar-child-prefix-full-dispatch',
         'tar-child-input-behavior', 'tar-child-input-smoke', 'tar-child-input-full-dispatch',
         'tar-child-input-contract-final', 'tar-child-input-contract-initial']
reports = {n: json.loads((ROOT/'evidence'/(n+'.json')).read_text()) for n in names}
processes = []
def parse(log):
    path = ROOT/log['log']
    assert fingerprint(path) == log['sha256']
    parsed = runner.parse_memory_log(path.read_text(), path.stem, exec_only=True)
    assert all(log[k] == value for k, value in parsed.items())
    return parsed

def clean(p):
    return (p['complete_exec_log'] and p['errors'] == p['non_inherited_descriptors'] == 0
            and not any(p['heap_bytes'].get(k, 0) for k in
                        ('definitely lost', 'indirectly lost', 'possibly lost')))

for name in names[:3]:
    data = reports[name]
    assert data['complete'] and data['total'] == 16
    assert fingerprint(Path(data['binary'])) == data['binary_sha256']
    final = name == 'tar-child-input-compression-original'
    assert data['passed'] == (15 if final else 13)
    driver = ROOT/('evidence/raw/tar-original-compression-driver.py'
                   if name == 'tar-pending-compression-original' else 'tests/tar-original.py')
    for p, h in data['inputs'].items():
        assert fingerprint(driver if p == str(ROOT/'tests/tar-original.py') else Path(p)) == h
    for row in data['results']:
        if name != 'tar-pending-compression-original':
            assert row['assertions_pass']
        for label, outcome in row['outcomes'].items():
            assert fingerprint(ROOT/outcome['driver_log']) == outcome['driver_log_sha256']
            if name != 'tar-pending-compression-original':
                assert not outcome['timed_out'] and not outcome['child_wait_timeout']
            for memory in outcome['memory']:
                parsed = parse(memory)
                if final and label == 'rboxc-valgrind':
                    assert parsed['complete_exec_log']
                    if not clean(parsed):
                        assert row['selection'] == 'compress-xz'
                        command = re.findall(r'^==\d+== Command: (.*)$', (ROOT/memory['log']).read_text(), re.M)
                        assert len(command) == 1 and re.fullmatch(r'/tmp/rboxc-tar-original-[\w]+/deps/xz(?: -d)?', command[0])
                        assert parsed['errors'] == 3 and parsed['non_inherited_descriptors'] == 2
                        assert parsed['heap_bytes']['possibly lost'] == 272
                processes.append({'report': name, 'selection': row['selection'],
                                  'implementation': label, **memory})
old_rows = {r['selection']: r for r in reports[names[0]]['results']}
new_rows = {r['selection']: r for r in reports[names[2]]['results']}
for name in ['remfiles01', 'remfiles02']:
    assert not old_rows[name]['assertions_pass'] and new_rows[name]['pass']
assert new_rows['remfiles02']['outcomes']['rboxc-valgrind']['waited_children']
audit = reports['tar-child-input-compression-audited']
assert audit['passed'] == audit['total'] == 15
assert audit['candidate_tar_processes_clean'] == 87 and audit['candidate_child_dependency_processes_clean'] == 61
for prefix in ['tar-child-prefix', 'tar-child-input']:
    for suffix, count in [('behavior', 57), ('smoke', 428), ('full-dispatch', 11)]:
        data = reports[prefix+'-'+suffix]
        assert data['passed'] == data['total'] == count
        assert fingerprint(Path(data['binary'])) == data['binary_sha256']
for row in reports['tar-child-input-behavior']['results']:
    for memory in row['outcomes']['rboxc-valgrind']['memory']:
        assert clean(parse(memory))
        processes.append({'report': 'tar-child-input-behavior', 'selection': row['name'],
                          'implementation': 'rboxc-valgrind', **memory})
contract = reports['tar-child-input-contract-final']
assert contract['passed'] == contract['total'] == 4 and contract['clean_processes'] == 5
for p, h in contract['inputs'].items():
    assert fingerprint(Path(p)) == h
for memory in contract['processes']:
    assert clean(parse(memory))
for p, h in reports['tar-child-input-contract-initial']['artifacts'].items():
    assert fingerprint(ROOT/p) == h
report = {
    'scope': 'GNU child/grandchild diagnostic names now follow program_name. A native helper closes only the owning child process input pipe and preserves inherited or reused descriptors. All 16 selected original assertions match GNU; 15 groups are strictly clean, while the host XZ helper retains its own findings. The harness waits for live child processes before preserving evidence. Full GNU-wide acceptance remains open.',
    'binary': str(binary), 'binary_sha256': fingerprint(binary), 'bytes': binary.stat().st_size,
    'rebuild': str(repro), 'rebuild_sha256': fingerprint(repro), 'byte_identical': True,
    'changed_helpers': changed, 'original_assertions_passed': 16, 'strict_original_groups': 15,
    'clean_tar_processes': 87, 'clean_native_child_processes': 61,
    'focused_passed': 57, 'coreutils_smoke': 428, 'dispatcher_passed': 11,
    'ownership_contracts': 4, 'clean_contract_processes': 5,
    'newly_resolved_originals': ['remfiles01', 'remfiles02'],
    'remaining_native_dependency_finding': 'compress-xz',
    'reports': {n: fingerprint(ROOT/'evidence'/(n+'.json')) for n in names},
    'processes': processes, 'driver_sha256': fingerprint(Path(__file__)),
    'translation_driver_sha256': fingerprint(ROOT/'scripts/translate-tar.py'),
    'cleanup_driver_sha256': fingerprint(ROOT/'scripts/tar_cleanup.py'),
    'full_acceptance_complete': False,
}
target.write_text(json.dumps(report, indent=2)+'\n')
print('Child diagnostics and ownership validated: 15 strict originals, 16 matching assertions, four lifetime contracts')
