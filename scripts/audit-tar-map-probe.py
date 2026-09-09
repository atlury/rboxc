#!/usr/bin/env python3
"""Audit the Tar map-buffer and compression-probe cleanup checkpoint."""
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
target = ROOT/'evidence/tar-map-probe-validation.json'
assert not target.exists()
binary = ROOT/'target/tar-map-probe-candidate/release/rboxc'
repro = ROOT/'target/tar-map-probe-repro/release/rboxc'
assert fingerprint(binary) == fingerprint(repro)
retained = ROOT/'evidence/raw/tar-before-map-probe-cleanup'
for row in json.loads((retained/'manifest.json').read_text()):
    assert fingerprint(ROOT/row['preserved']) == row['sha256']
old = json.loads((retained/'tar-link.json').read_text())
new = json.loads((ROOT/'evidence/tar-link.json').read_text())
assert old['original_inputs'] == new['original_inputs']
assert old['rust_source_sha256'] == new['rust_source_sha256']
assert old['symbol_map_sha256'] == new['symbol_map_sha256']
changed = [p for p, h in new['helper_inputs'].items() if old['helper_inputs'][p] != h]
assert changed == ['build/helpers/tar-buffer.o', 'build/helpers/tar-map.o']
for p, h in new['helper_inputs'].items():
    assert fingerprint(ROOT/p) == h
names = ['tar-pending-links-metadata-original', 'tar-map-probe-metadata-original',
         'tar-map-probe-metadata-audited', 'tar-map-probe-behavior',
         'tar-map-probe-smoke', 'tar-map-probe-full-dispatch', 'tar-map-probe-dispatcher']
reports = {n: json.loads((ROOT/'evidence'/(n+'.json')).read_text()) for n in names}
initial = reports['tar-pending-links-metadata-original']
final = reports['tar-map-probe-metadata-original']
assert initial['complete'] and initial['passed'] == 12 and initial['total'] == 14
assert final['complete'] and final['passed'] == 26 and final['total'] == 27
processes = []
def parse(log):
    path = ROOT/log['log']
    assert fingerprint(path) == log['sha256']
    result = runner.parse_memory_log(path.read_text(), path.stem, exec_only=True)
    assert all(log[k] == value for k, value in result.items())
    return result

def clean(m):
    return (m['complete_exec_log'] and m['errors'] == m['non_inherited_descriptors'] == 0
            and not any(m['heap_bytes'].get(k, 0) for k in
                        ('definitely lost', 'indirectly lost', 'possibly lost')))

for name, data, driver in [
    ('initial', initial, ROOT/'evidence/raw/tar-original-metadata-driver.py'),
    ('final', final, ROOT/'tests/tar-original.py'),
]:
    assert fingerprint(Path(data['binary'])) == data['binary_sha256']
    for p, h in data['inputs'].items():
        actual = driver if p == str(ROOT/'tests/tar-original.py') else Path(p)
        assert fingerprint(actual) == h, p
    for row in data['results']:
        for label, outcome in row['outcomes'].items():
            assert fingerprint(ROOT/outcome['driver_log']) == outcome['driver_log_sha256']
            for log in outcome['memory']:
                parsed = parse(log)
                if label == 'rboxc-valgrind':
                    if name == 'final' or row['selection'] not in {'map', 'shortupd'}:
                        assert clean(parsed)
                processes.append({'observation': name, 'selection': row['selection'],
                                  'implementation': label, **log})
    assert {row['selection'] for row in data['results'] if not row['pass']} == (
        {'map', 'shortupd'} if name == 'initial' else {'capabs_raw01'})

capability = next(row for row in final['results'] if row['selection'] == 'capabs_raw01')
for outcome in capability['outcomes'].values():
    assert outcome['status'] == 1 and outcome['assertions'] == [['233', 'FAILED']]
    path = (ROOT/outcome['driver_log']).parent/'suite/233/testsuite.log'
    text = path.read_text()
    assert text.count('-dir/file = cap_chown=ei\n+dir/file cap_chown=ei\n') == 1
    assert text.count('@@ -1,2 +1,2 @@') == 1
audited = reports['tar-map-probe-metadata-audited']
assert audited['passed'] == audited['total'] == 26 and audited['candidate_processes_clean'] == 400
assert audited['source_reports'][0]['omitted_selections'] == [
    {'selection': 'capabs_raw01', 'pass': False, 'assertions_pass': False}]
for name, total in [('tar-map-probe-behavior', 57), ('tar-map-probe-smoke', 428),
                    ('tar-map-probe-full-dispatch', 11)]:
    data = reports[name]
    assert data['binary_sha256'] == fingerprint(binary)
    assert data['passed'] == data['total'] == total
for row in reports['tar-map-probe-behavior']['results']:
    assert row['pass'] and row['equivalent']
    for log in row['outcomes']['rboxc-valgrind']['memory']:
        assert clean(parse(log))
        processes.append({'observation': 'focused', 'selection': row['name'],
                          'implementation': 'rboxc-valgrind', **log})
dispatch_logs = []
for row in reports['tar-map-probe-full-dispatch']['results']:
    if 'log' in row:
        path = ROOT/row['log']
        contents = path.read_text()
        pids = set(re.findall(r'^==(\d+)==', contents, re.M))
        assert len(pids) == 1
        parsed = runner.parse_memory_log(contents, pids.pop(), exec_only=True)
        assert clean(parsed)
        dispatch_logs.append({'log': row['log'], 'sha256': fingerprint(path), **parsed})
assert len(dispatch_logs) == 9
wrong_profile = reports['tar-map-probe-dispatcher']
assert wrong_profile['passed'] == 9 and wrong_profile['total'] == 11
assert {r['alias'] for r in wrong_profile['results'] if not r['pass']} == {'rbox', 'rboxc'}
report = {
    'scope': 'Two private native Tar helper fixes; the Rust entry and original GNU objects remain unchanged. '
             '26 unchanged original groups and 57 focused comparisons pass. One original capability '
             'assertion fails identically in GNU because host getcap omits the historical equals prefix; '
             'its candidate processes are clean and it adds no passing group. The initial dispatcher '
             'used the installed command set; the explicit 187-command profile passes. Full acceptance remains open.',
    'binary': str(binary), 'binary_sha256': fingerprint(binary), 'bytes': binary.stat().st_size,
    'rebuild': str(repro), 'rebuild_sha256': fingerprint(repro), 'byte_identical': True,
    'changed_helpers': changed, 'original_groups_passed': 26, 'clean_original_processes': 400,
    'focused_passed': 57, 'coreutils_smoke': 428, 'dispatcher_passed': 11,
    'matched_baseline_failures': ['capabs_raw01'], 'reports': {
        n: fingerprint(ROOT/'evidence'/(n+'.json')) for n in names},
    'processes': processes, 'dispatcher_logs': dispatch_logs,
    'driver_sha256': fingerprint(Path(__file__)),
    'cleanup_driver_sha256': fingerprint(ROOT/'scripts/tar_cleanup.py'),
    'link_report_sha256': fingerprint(ROOT/'evidence/tar-link.json'),
    'full_acceptance_complete': False,
}
target.write_text(json.dumps(report, indent=2)+'\n')
print('Verified two helper fixes, 26 original groups, 57 focused checks and the 187-command dispatcher')
