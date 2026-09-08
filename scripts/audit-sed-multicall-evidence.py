#!/usr/bin/env python3
"""Audit Sed execution with native and integrated Coreutils dependencies."""
# SPDX-License-Identifier: GPL-3.0-or-later
import importlib.util
import json
from pathlib import Path
import re
import sys

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT/'tests'))
sys.path.insert(0, str(ROOT/'tests/gnu'))
from comparison_profile import fingerprint
from sed_dependencies import prepare

spec = importlib.util.spec_from_file_location('reviewed', ROOT/'tests/gnu/reviewed-original.py')
runner = importlib.util.module_from_spec(spec)
spec.loader.exec_module(runner)
target = ROOT/'evidence/sed-multicall-execution-audit.json'
assert not target.exists(), 'preserve prior audits'
paths = {name: ROOT/'evidence'/(filename+'.json') for name, filename in {
    'focused': 'sed-current-focused',
    'native_helpers': 'sed-execution-current-original',
    'multicall_helpers': 'sed-multicall-execution-original',
}.items()}
reports = {name: json.loads(path.read_text()) for name, path in paths.items()}
focused = reports['focused']
binary = Path(focused['binary'])
oracle = ROOT/'build/gnu-sed/sed/sed'
assert focused['passed'] == focused['total'] == len(focused['results']) == 56
inputs = {binary: fingerprint(binary), oracle: fingerprint(oracle)}
drivers = {
    'focused': ROOT/'tests/sed-behavior.py',
    'native_helpers': ROOT/'evidence/raw/sed-execution-before-multicall-driver.py',
    'multicall_helpers': ROOT/'tests/sed-original.py',
}
helpers, environment, prerequisites = prepare()
inputs.update({Path(h['path']): h['sha256'] for h in helpers.values()})
inputs[ROOT/prerequisites['path']] = prerequisites['sha256']
inputs[ROOT/'tests/sed_dependencies.py'] = prerequisites['adapter_sha256']
manifest = json.loads((ROOT/'inventory/sed-tests.json').read_text())
source = Path(json.loads((ROOT/'inventory/sources.json').read_text())['sed']['source'])
selection = next(r for r in manifest['scripts'] if r['script'] == 'testsuite/execute-tests.sh')
assert selection['reviewed']
inputs[source/selection['script']] = selection['source_sha256']
inputs[source/manifest['registration']['path']] = manifest['registration']['sha256']
processes = []
for name, report in reports.items():
    assert report['binary_sha256'] == inputs[binary]
    assert report['gnu_binary_sha256'] == inputs[oracle]
    assert report['driver_sha256'] == fingerprint(drivers[name])
    inputs[drivers[name]] = report['driver_sha256']
    inputs.update({Path(p): h for p, h in report['runtime_helpers'].items()})
    if name != 'focused':
        assert report['total'] == len(report['results']) == 1
        assert report['selected_scripts'] == ['execute-tests.sh']
        row = report['results'][0]
        assert row['script'] == selection['script'] and row['source_sha256'] == selection['source_sha256']
        assert row['native_pass'] and row['assertions_pass']
        assert all(o['status'] == 0 for o in row['outcomes'].values())
        recorded = dict(report['prerequisites'])
        if name == 'multicall_helpers':
            core = recorded.pop('coreutils')
            assert core['path'] == str(binary) and core['sha256'] == inputs[binary]
            assert core['commands'] == ['cat', 'touch', 'sleep', 'dd']
            recorded['coreutils'] = recorded.pop('native_coreutils')
            assert row['pass'] and row['memory_clean']
        else:
            assert not row['pass'] and not row['memory_clean']
        assert recorded == prerequisites
    for row in report['results']:
        if name == 'focused':
            assert row['pass'] and row['equivalent'] and row['memory_clean']
            assert all(all(o[k] == row['outcomes']['gnu'][k] for k in ('status', 'stdout', 'stderr', 'tree'))
                       for o in row['outcomes'].values())
        for implementation, outcome in row['outcomes'].items():
            if name != 'focused':
                assert fingerprint(ROOT/outcome['log']) == outcome['log_sha256']
                assert (ROOT/outcome['log']).read_bytes() == bytes.fromhex(outcome['stdout']) + bytes.fromhex(outcome['stderr'])
            if implementation.endswith('-valgrind'):
                assert outcome['memory']
            for log in outcome.get('memory', []):
                path = ROOT/log['log']
                assert fingerprint(path) == log['sha256']
                text = path.read_text()
                commands = re.findall(r'^==[0-9]+== Command: (.*)$', text, re.M)
                assert len(commands) == 1
                executable = commands[0].split()[0]
                if executable == 'sed' or re.fullmatch(r'/tmp/rboxc-sed-(?:original|behavior)-[^/]+/(?:exec|real|sed)/sed', executable):
                    role = 'sed'
                elif executable == '/bin/sh':
                    role = 'system-shell'
                elif re.fullmatch(r'/tmp/rboxc-sed-original-[^/]+/testsuite/cat', executable):
                    role = 'native-cat' if name == 'native_helpers' else 'multicall-cat'
                else:
                    raise AssertionError('unclassified process: '+commands[0])
                parsed = runner.parse_memory_log(text, path.stem, exec_only=True)
                assert all(log[k] == value for k, value in parsed.items())
                assert parsed['complete_exec_log']
                lost = {k: parsed['heap_bytes'].get(k, 0) for k in ('definitely lost', 'indirectly lost', 'possibly lost')}
                clean = parsed['errors'] == 0 and parsed['non_inherited_descriptors'] == 0 and not any(lost.values())
                if role == 'native-cat':
                    assert parsed['errors'] == 1 and lost == {'definitely lost': 262144, 'indirectly lost': 0, 'possibly lost': 0}
                    assert parsed['non_inherited_descriptors'] == 0
                elif implementation == 'rboxc-valgrind' or role != 'sed':
                    assert clean
                processes.append({'profile': name, 'implementation': implementation, 'role': role,
                                  'command': commands[0], 'log': log['log'], 'sha256': log['sha256'],
                                  'memory_clean': clean, **parsed})
assert all(fingerprint(path) == h for path, h in inputs.items())
for name in ('native_helpers', 'multicall_helpers'):
    for implementation in ('gnu-valgrind', 'rboxc-valgrind'):
        group = [p for p in processes if p['profile'] == name and p['implementation'] == implementation]
        assert len(group) == 25
        assert sum(p['role'] == 'sed' for p in group) == 20
        assert sum(p['role'] == 'system-shell' for p in group) == 4
        assert sum(p['role'].endswith('-cat') for p in group) == 1
candidate = [p for p in processes if p['implementation'] == 'rboxc-valgrind']
result = {
    'scope': 'All 56 focused comparisons and the unchanged execute-tests.sh assertions pass. The native-helper profile retains its GNU cat allocation finding in both implementations. The candidate integrated-helper profile has 25 clean processes, including Sed, shell children and translated cat. Native GNU Sed memory findings are preserved separately. Other Sed originals and platform profiles retain their prior status; this is not full GNU acceptance.',
    'binary': str(binary), 'binary_sha256': inputs[binary],
    'source_reports': {name: {'path': str(path.relative_to(ROOT)), 'sha256': fingerprint(path)} for name, path in paths.items()},
    'inputs': {str(path): h for path, h in inputs.items()},
    'focused_passed': 56, 'multicall_original_selections': 1,
    'candidate_processes': len(candidate),
    'clean_candidate_processes': sum(p['memory_clean'] for p in candidate),
    'retained_native_helper_findings': sum(p['role'] == 'native-cat' and not p['memory_clean'] for p in processes),
    'retained_gnu_sed_findings': sum(p['implementation'] == 'gnu-valgrind' and p['role'] == 'sed' and not p['memory_clean'] for p in processes),
    'driver_sha256': fingerprint(Path(__file__)), 'results': processes,
}
target.write_text(json.dumps(result, indent=2)+'\n')
print('Audited', len(processes), 'process logs;', result['clean_candidate_processes'], 'clean candidate-profile processes; native GNU cat findings retained')
