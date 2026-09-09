#!/usr/bin/env python3
"""Verify the printf ownership fix, immutable baselines and candidate validation."""
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
target = ROOT/'evidence/gawk-format-lifetime-validation.json'
assert not target.exists(), 'preserve earlier validation'
names = ['gawk-format-lifetime-contract', 'gawk-format-lifetime-baseline-contract',
         'gawk-format-lifetime-behavior', 'gawk-format-lifetime-smoke',
         'gawk-format-lifetime-dispatch', 'gawk-format-lifetime-full-dispatch',
         'gawk-table-format-source-original', 'gawk-format-lifetime-original',
         'gawk-format-lifetime-reviewed-original', 'gawk-format-lifetime-coverage']
reports = {n:json.loads((ROOT/'evidence'/(n+'.json')).read_text()) for n in names}
candidate = ROOT/'target/gawk-format-lifetime-candidate/release/rboxc'
digest = fingerprint(candidate)
repro = ROOT/'target/gawk-format-lifetime-repro/release/rboxc'
assert fingerprint(repro) == digest
for name, total in [('gawk-format-lifetime-contract',18), ('gawk-format-lifetime-behavior',85),
                    ('gawk-format-lifetime-smoke',428), ('gawk-format-lifetime-full-dispatch',11)]:
    d = reports[name]
    assert d['binary_sha256'] == digest and d['passed'] == d['total'] == total
    assert all(r['pass'] for r in d['results'])
coverage = reports['gawk-format-lifetime-coverage']
assert coverage['binary_sha256'] == digest
assert coverage['manifest_sha256'] == fingerprint(ROOT/'inventory/gawk-tests.json')
assert coverage['driver_sha256'] == fingerprint(ROOT/'scripts/audit-gawk-current-coverage.py')
assert coverage['original_selections'] == 539 and coverage['original_passed'] == 525
assert coverage['baseline_failures'] == 6 and len(coverage['open_originals']) == 8
assert coverage['clean_candidate_processes'] == 668
clean_contracts = []
baseline_findings = []
for name in ('gawk-format-lifetime-contract', 'gawk-format-lifetime-baseline-contract'):
    d = reports[name]
    assert d['complete'] and d['total'] == 18
    assert d['passed'] == (18 if name == 'gawk-format-lifetime-contract' else 6)
    for p, h in d['inputs'].items():
        assert fingerprint(Path(p)) == h
    for row in d['results']:
        assert row['equivalent']
        reference = row['outcomes']['gnu']
        for key, o in row['outcomes'].items():
            assert all(o[k] == reference[k] for k in ('status', 'stdout', 'stderr'))
            for p, h in o['raw'].items():
                assert fingerprint(ROOT/p) == h
            if 'memory' not in o:
                continue
            log = ROOT/o['memory_log']
            text = log.read_text()
            pids = set(re.findall(r'^==([0-9]+)==', text, re.M))
            assert len(pids) == 1
            parsed = runner.parse_memory_log(text, pids.pop(), exec_only=True)
            assert parsed == o['memory'] and parsed['complete_exec_log']
            if key != 'rboxc-valgrind':
                continue
            if name == 'gawk-format-lifetime-contract':
                assert parsed['errors'] == parsed['non_inherited_descriptors'] == 0
                assert not any(parsed['heap_bytes'].get(k,0) for k in ('definitely lost','indirectly lost','possibly lost'))
                clean_contracts.append({'log':o['memory_log'], 'sha256':fingerprint(log), **parsed})
            elif not o['memory_clean']:
                assert parsed['heap_bytes']['definitely lost'] > 0
                baseline_findings.append({'log':o['memory_log'], 'sha256':fingerprint(log), **parsed})
assert len(clean_contracts) == 18 and len(baseline_findings) == 12
before = json.loads((ROOT/'evidence/gawk-before-format-link.json').read_text())
after = json.loads((ROOT/'evidence/gawk-link.json').read_text())
assert before['original_inputs'] == after['original_inputs']
assert before['link_inputs'] == after['link_inputs']
assert before['rust_source_sha256'] == after['rust_source_sha256']
changed = [p for p, h in after['helper_inputs'].items() if before['helper_inputs'][p] != h]
assert changed == ['build/helpers/gawk-017-printf.o']
assert fingerprint(ROOT/'build/gawk-printf-before-format.o') == before['helper_inputs'][changed[0]]
for p, h in {**after['original_inputs'], **after['helper_inputs']}.items():
    assert fingerprint(ROOT/p) == h
cleanup = json.loads((ROOT/'evidence/gawk-format-cleanup.json').read_text())
for p, h in cleanup['inputs'].items():
    assert fingerprint(Path(p)) == h
for pkey, hkey in [('adapted_source','adapted_source_sha256'), ('object','object_sha256'), ('log','log_sha256')]:
    assert fingerprint(ROOT/cleanup[pkey]) == cleanup[hkey]
before_fmt = next(r for r in reports['gawk-table-format-source-original']['results'] if r['selection'] == 'fmtspcl')
after_fmt = next(r for r in reports['gawk-format-lifetime-reviewed-original']['results'] if r['selection'] == 'fmtspcl')
assert not before_fmt['outcomes']['rboxc-valgrind']['memory_clean']
assert after_fmt['baseline_failure_matches'] and after_fmt['outcomes']['rboxc-valgrind']['memory_clean']
for key in before_fmt['outcomes']:
    a, b = before_fmt['outcomes'][key], after_fmt['outcomes'][key]
    assert a['actual_output_sha256'] == b['actual_output_sha256']
    assert fingerprint(ROOT/a['actual_output']) == fingerprint(ROOT/b['actual_output']) == a['actual_output_sha256']
installed = ROOT/'target/release/rboxc'
assert fingerprint(installed) == '8979835878e7dbe575440b5d0c7c86a01c75782ff5ddc735707ccada50e88e86'
assert len(subprocess.check_output([str(candidate),'--list'], text=True).splitlines()) == 187
assert len(subprocess.check_output([str(installed),'--list'], text=True).splitlines()) == 133
assert fingerprint(candidate.parent/'libstdbuf.so') == fingerprint(installed.parent/'libstdbuf.so') == fingerprint(repro.parent/'libstdbuf.so')
target.write_text(json.dumps({
    'scope':'Only the isolated native GNU printf helper changes. The temporary-string cleanup preserves output and borrowed arguments. All reviewed Gawk originals rerun on this candidate, with six GNU assertion baselines and eight native-child/extension profiles retained separately. Full GNU acceptance remains open.',
    'binary':str(candidate), 'binary_sha256':digest, 'bytes':candidate.stat().st_size,
    'repro':str(repro), 'repro_sha256':fingerprint(repro), 'installed_unchanged':True,
    'changed_helpers':changed, 'clean_original_and_focused_processes':668,
    'additional_clean_contracts':clean_contracts, 'initial_contract_findings':baseline_findings,
    'original_passed':525, 'original_baselines':6, 'open_profiles':8,
    'smoke_passed':428, 'dispatcher_passed':11,
    'reports':{str((ROOT/'evidence'/(n+'.json')).relative_to(ROOT)):fingerprint(ROOT/'evidence'/(n+'.json')) for n in names},
    'cleanup_report_sha256':fingerprint(ROOT/'evidence/gawk-format-cleanup.json'),
    'link_report_sha256':fingerprint(ROOT/'evidence/gawk-link.json'),
    'driver_sha256':fingerprint(Path(__file__)), 'full_acceptance_complete':False,
}, indent=2)+'\n')
print('Verified printf ownership fix, 539 reviewed originals, 686 clean processes and identical rebuild')
