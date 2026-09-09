#!/usr/bin/env python3
"""Audit close ownership and unchanged original Bash assertions."""
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
target = ROOT/'evidence/bash-owned-close-validation.json'
assert not target.exists()
binary = ROOT/'target/bash-owned-close-candidate/release/rboxc'
digest = fingerprint(binary)
assert fingerprint(ROOT/'target/bash-owned-close-repro/release/rboxc') == digest
assert len(subprocess.check_output([binary, '--list']).splitlines()) == 187
assert fingerprint(ROOT/'target/release/rboxc') == '8979835878e7dbe575440b5d0c7c86a01c75782ff5ddc735707ccada50e88e86'
helper_proof = json.loads((ROOT/'evidence/bash-trap-restart-helper-changes.json').read_text())
assert all(fingerprint(ROOT/p) == h for p, h in helper_proof['current'].items())
raw = {}
reports = {}


def pin(path, expected=None):
    path = Path(path)
    if not path.is_absolute(): path = ROOT/path
    actual = fingerprint(path)
    if expected is not None: assert actual == expected, str(path)
    raw[str(path.relative_to(ROOT)) if path.is_relative_to(ROOT) else str(path)] = actual
    return actual


def load(name, total):
    path = ROOT/'evidence'/(name+'.json')
    data = json.loads(path.read_text())
    assert data.get('complete', True)
    assert data['passed'] == data['total'] == total == len(data['results'])
    assert all(r['pass'] for r in data['results'])
    assert data['binary_sha256'] == digest
    for p, h in {**data.get('inputs', {}), **data['runtime_helpers']}.items():
        if p == str(ROOT/'inventory/bash-tests.json'):
            p = 'evidence/raw/bash-owned-close-original-inventory.json'
        pin(p, h)
    reports[name] = {'sha256': pin(path), 'passed': total}
    return data


for name, total in [('behavior', 44), ('adapters', 69), ('smoke', 428), ('dispatch', 11)]:
    data = load('bash-owned-close-'+name, total)
    for result in data['results']:
        for key, outcome in result.get('outcomes', {}).items():
            for record in outcome.get('memory', []):
                pin(record['log'], record['sha256'])
                contents = (ROOT/record['log']).read_text()
                pids = set(re.findall(r'^==([0-9]+)==', contents, re.M))
                assert len(pids) == 1
                parsed = runner.parse_memory_log(contents, pids.pop())
                assert all(record[k] == v for k, v in parsed.items())
                if key == 'rboxc-valgrind':
                    assert record['complete'] and record['clean'] and parsed['errors'] == 0
                    assert parsed['non_inherited_descriptors'] == 0
                    assert not any(parsed['heap_bytes'].get(k, 0) for k in ('definitely lost', 'indirectly lost', 'possibly lost'))
        if result.get('log'): pin(result['log'])
original = load('bash-owned-close-reviewed-original', 29)
original_audit_path = ROOT/'evidence/bash-owned-close-original-validation.json'
audit = json.loads(original_audit_path.read_text())
assert audit['original_sha256'] == reports['bash-owned-close-reviewed-original']['sha256']
assert audit['strict_original_passes'] == audit['original_scripts'] == 29
assert audit['original_groups'] == 28
assert not audit['open_processes'] and not audit['open_findings']
for p, h in audit['raw'].items(): pin(p, h)
reports[original_audit_path.stem] = {'sha256': pin(original_audit_path)}

contract_path = ROOT/'evidence/bash-owned-close-contract.json'
contract = json.loads(contract_path.read_text())
assert contract['passed'] == contract['total'] == len(contract['results']) == 7
pin('tests/bash-backup-contract.py', contract['driver_sha256'])
pin('src/shell_arguments.c', contract['runtime_source_sha256'])
for result in contract['results']:
    assert result['pass']
    for outcome in result['outcomes']:
        assert outcome['pass'] and outcome['status'] == 0
        assert outcome['stdout'] == b'backup ownership passed\n'.hex() and not outcome['stderr']
        if outcome['instrumented']:
            pin(outcome['log'], outcome['sha256'])
            text = (ROOT/outcome['log']).read_text()
            pids = set(re.findall(r'^==([0-9]+)==', text, re.M)); assert len(pids) == 1
            parsed = runner.parse_memory_log(text, pids.pop(), exec_only=True)
            assert outcome['memory'] == parsed and outcome['memory_clean']
            assert parsed['complete_exec_log'] and parsed['errors'] == 0
            assert parsed['non_inherited_descriptors'] == 0
reports[contract_path.stem] = {'sha256': pin(contract_path), 'passed': 7}
before = ROOT/'evidence/raw/bash-before-owned-close-source.c'
source = ROOT/'src/shell_arguments.c'
pin(before); pin(source)
result = {
    'scope': 'The shared Bash close wrapper preserves close results and errno while avoiding a second syscall for an already closed descriptor. Backup ownership is forgotten after every close result. Seven native/Valgrind ownership contracts pass; all 29 reviewed original scripts match unchanged GNU assertions with clean candidate process logs. Native GNU duplicate-close findings remain preserved. Two recipe dispatchers are accounted as orchestration, not runtime passes. Full Bash and GNU-wide validation remain open.',
    'binary': str(binary), 'binary_sha256': digest, 'size_bytes': binary.stat().st_size,
    'byte_identical_rebuild': True, 'unchanged_namespaced_helpers': True,
    'runtime_source': str(source), 'runtime_source_sha256': fingerprint(source),
    'previous_runtime_source': str(before), 'previous_runtime_source_sha256': fingerprint(before),
    'strict_original_scripts': 29, 'clean_original_processes': audit['clean_candidate_processes'],
    'reports': reports, 'raw': raw, 'pass': True,
}
target.write_text(json.dumps(result, indent=2)+'\n')
print('PASS: owned close, seven contracts, 29 originals and shared comparisons')
