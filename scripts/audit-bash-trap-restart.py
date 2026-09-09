#!/usr/bin/env python3
"""Audit trap ownership, original GNU assertions and complete process logs."""
# SPDX-License-Identifier: GPL-3.0-or-later
import importlib.util
import json
from pathlib import Path
import re
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[1]
sys.path[:0] = [str(ROOT/'tests'), str(ROOT/'tests/gnu')]
from comparison_profile import fingerprint
spec = importlib.util.spec_from_file_location('reviewed', ROOT/'tests/gnu/reviewed-original.py')
runner = importlib.util.module_from_spec(spec)
spec.loader.exec_module(runner)
target = ROOT/'evidence/bash-trap-restart-validation.json'
assert not target.exists()
binary = ROOT/'target/bash-trap-restart-final-candidate/release/rboxc'
digest = fingerprint(binary)
assert fingerprint(ROOT/'target/bash-trap-restart-final-repro/release/rboxc') == digest
assert len(subprocess.check_output([binary, '--list']).splitlines()) == 187
assert fingerprint(ROOT/'target/release/rboxc') == '8979835878e7dbe575440b5d0c7c86a01c75782ff5ddc735707ccada50e88e86'
raw = {}
reports = {}


def verify(path, expected):
    path = Path(path)
    if not path.is_absolute(): path = ROOT/path
    assert fingerprint(path) == expected, str(path)
    raw[str(path.relative_to(ROOT)) if path.is_relative_to(ROOT) else str(path)] = expected


profile_path = ROOT/'evidence/bash-trap-restart-cleanup.json'
profile = json.loads(profile_path.read_text())
verify('scripts/bash_trap_restart_cleanup.py', profile['driver_sha256'])
verify('evidence/bash-return-trap-cleanup.json', profile['previous_profile_sha256'])
verify('build/bash-return-trap-cleanup/trap.c', profile['previous_source_sha256'])
for field in ('source', 'object', 'build_log'): verify(profile[field], profile[field+'_sha256'])
for name, driver in [
    ('initial', 'initial'),
    ('owners-initial', 'owners-initial'),
    ('attached', 'attached'),
]:
    archived = json.loads((ROOT/f'evidence/bash-trap-restart-{name}-cleanup.json').read_text())
    verify(f'evidence/raw/bash-trap-restart-{driver}-driver.py', archived['driver_sha256'])
    for field in ('source', 'object', 'build_log'): verify(archived[field], archived[field+'_sha256'])
changes = json.loads((ROOT/'evidence/bash-trap-restart-helper-changes.json').read_text())
assert changes['previous'].keys() == changes['current'].keys()
assert changes['changed'] == [p for p in changes['previous'] if changes['previous'][p] != changes['current'][p]] == ['build/helpers/bash-017-trap.o']
for path, expected in changes['current'].items(): verify(path, expected)


def load(name, current=True):
    path = ROOT/'evidence'/(name+'.json')
    data = json.loads(path.read_text())
    assert data.get('complete', True)
    assert data['passed'] == sum(row['pass'] for row in data['results'])
    assert data['total'] == len(data['results'])
    if current: assert data['binary_sha256'] == digest
    verify(data['binary'], data['binary_sha256'])
    for filename, expected in {**data.get('inputs', {}), **data['runtime_helpers']}.items():
        if filename == str(ROOT/'inventory/bash-tests.json'):
            filename = 'evidence/raw/bash-trap-restart-final-inventory.json'
        verify(filename, expected)
    reports[name] = {'sha256': fingerprint(path), 'passed': data['passed'], 'total': data['total']}
    return data


def memory(row, strict):
    verify(row['log'], row['sha256'])
    text = (ROOT/row['log']).read_text()
    pids = set(re.findall(r'^==([0-9]+)==', text, re.M))
    assert len(pids) == 1
    parsed = runner.parse_memory_log(text, pids.pop())
    assert all(row[k] == value for k, value in parsed.items())
    ends = list(re.finditer(r'ERROR SUMMARY:', text))
    fds = list(re.finditer(r'FILE DESCRIPTORS:', text))
    images = list(re.finditer(r'^==[0-9]+== Command:', text, re.M))
    complete = bool(ends) and len(ends) == len(fds) and (not images or ends[-1].start() > images[-1].start() and fds[-1].start() > images[-1].start())
    assert row['complete'] == complete
    clean = complete and parsed['errors'] == 0 and parsed['non_inherited_descriptors'] == 0 and not any(parsed['heap_bytes'].get(k, 0) for k in ('definitely lost', 'indirectly lost', 'possibly lost'))
    assert row['clean'] == clean
    if strict: assert clean, row['log']


contract_logs = []
for suffix, total in [('return', 32), ('lifetimes', 20), ('state', 32)]:
    data = load('bash-trap-restart-final-'+suffix)
    assert data['passed'] == data['total'] == total
    for row in data['results']:
        assert row['equivalent'] and row['memory_clean']
        assert set(row['outcomes']) == {'gnu', 'gnu-valgrind', 'rboxc', 'rboxc-valgrind'}
        reference = row['outcomes']['gnu']
        for key, outcome in row['outcomes'].items():
            assert all(outcome[k] == reference[k] for k in ('status', 'stdout', 'stderr', 'tree'))
            for record in outcome['memory']:
                memory(record, key == 'rboxc-valgrind')
                if key == 'rboxc-valgrind': contract_logs.append(record['log'])

# Keep every intermediate failure visible; none contributes to strict counts.
for name, passes, total in [
    ('bash-trap-restart-contract', 32, 32),
    ('bash-trap-restart-lifetimes', 4, 20),
    ('bash-trap-restart-owners-lifetimes', 16, 20),
    ('bash-trap-restart-owners-state', 28, 32),
]:
    data = load(name, False)
    assert (data['passed'], data['total']) == (passes, total)
    for row in data['results']:
        assert row['equivalent']
        for outcome in row['outcomes'].values():
            for record in outcome['memory']: memory(record, False)

for suffix, total in [('behavior', 44), ('adapters', 69)]:
    data = load('bash-trap-restart-final-'+suffix)
    assert data['passed'] == data['total'] == total
    for row in data['results']:
        assert row['equivalent'] and row['memory_clean']
        for key, outcome in row['outcomes'].items():
            for record in outcome['memory']: memory(record, key == 'rboxc-valgrind')
for suffix, total in [('smoke', 428), ('dispatch', 11)]:
    data = load('bash-trap-restart-final-'+suffix)
    assert data['passed'] == data['total'] == total

original = load('bash-trap-restart-final-original')
assert original['passed'] == 26 and original['total'] == 27
audit_path = ROOT/'evidence/bash-trap-restart-final-original-validation.json'
audit = json.loads(audit_path.read_text())
assert audit['strict_original_passes'] == 26
for path, expected in audit['raw'].items(): verify(path, expected)
reports[audit_path.stem] = {'sha256': fingerprint(audit_path)}

result = {
    'scope': 'Only the Bash trap helper changed. All 84 bounded ownership/control-flow contracts pass with complete clean candidate summaries. Original GNU output is unchanged in all 27 reviewed scripts; the deliberate set-x duplicate-close profile remains separately open. This unit does not complete Bash or GNU-wide validation.',
    'binary': str(binary), 'binary_sha256': digest, 'size_bytes': binary.stat().st_size,
    'byte_identical_rebuild': True, 'changed_helpers': changes['changed'],
    'strict_contracts': 84, 'clean_contract_processes': len(set(contract_logs)),
    'strict_original_scripts': 26, 'reports': reports, 'raw': raw,
    'pass': True,
}
target.write_text(json.dumps(result, indent=2)+'\n')
print('PASS: trap restart ownership, 84 contracts, reviewed originals and shared checks')
