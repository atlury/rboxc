#!/usr/bin/env python3
"""Verify the final screen replay with explicitly disabled external openers."""
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
target = ROOT/'evidence/less-screen-combined-validation.json'
assert not target.exists(), 'preserve prior audits'
path = ROOT/'evidence/less-private-links-original.json'
data = json.loads(path.read_text())
prior_path = ROOT/'evidence/less-keyboard-validation.json'
prior = json.loads(prior_path.read_text())
for name, expected in prior['reports'].items():
    assert fingerprint(ROOT/'evidence'/(name+'.json')) == expected
old_manifest = ROOT/'evidence/raw/less-keyboard-replay-manifest.json'
assert fingerprint(old_manifest) == prior['manifest_sha256']
old_original = json.loads((ROOT/'evidence/less-screen-keyboard-original.json').read_text())
old_driver = ROOT/'evidence/raw/less-keyboard-original-driver.py'
assert fingerprint(old_driver) == old_original['driver_sha256']
assert data['complete'] and data['passed'] == data['assertion_passes'] == data['total'] == 1
assert data['selected_targets'] == ['osc8']
assert data['binary_sha256'] == prior['test_profile_sha256'] == fingerprint(Path(data['binary']))
assert fingerprint(Path(prior['binary'])) == prior['binary_sha256']
assert fingerprint(ROOT/'tests/less-original.py') == data['driver_sha256']
for filename, expected in data['inputs'].items():
    assert fingerprint(Path(filename)) == expected
manifest_path = ROOT/'inventory/less-tests.json'
manifest = json.loads(manifest_path.read_text())
row = next(r for r in manifest['replays'] if r['name'] == 'osc8')
source = Path('/opt/src/less-704')/row['path']
assert fingerprint(source) == row['sha256']
assert describe(source.read_bytes()) == row['records']
result = data['results'][0]
assert result['pass'] and result['assertions_pass'] and result['frames'] == 223
assert result['environment_overlay'] == row['environment_overlay'] == {
    'LESS_OSC8_OPEN_ANY': '', 'LESS_OSC8_OPEN_man': ''}
processes = []
for name, outcome in result['outcomes'].items():
    assert outcome['status'] == 0 and outcome['assertions_pass']
    assert bytes.fromhex(outcome['stdout']) == b'PASS: osc8 (223 steps)\n'
    assert bytes.fromhex(outcome['stderr']) == b'TEST osc8.lt\nRAN  1 tests with 0 errors\n'
    for stream, entry in outcome['streams'].items():
        p = ROOT/entry['path']
        assert fingerprint(p) == entry['sha256'] and p.read_bytes() == bytes.fromhex(outcome[stream])
    assert len(outcome['memory']) == (2 if name.endswith('-valgrind') else 0)
    for recorded in outcome['memory']:
        p = ROOT/recorded['log']
        assert fingerprint(p) == recorded['sha256']
        text = p.read_text()
        commands = re.findall(r'^==[0-9]+== Command: (.*)$', text, re.M)
        assert commands == [recorded['command']]
        expected_binary = data['binary'] if name == 'rboxc-valgrind' else str(Path(data['binary']).parents[1]/'less')
        assert commands[0] in (expected_binary+' -V', expected_binary+' --tty /dev/stdin osc8')
        parsed = runner.parse_memory_log(text, p.stem, exec_only=True)
        assert parsed['complete_exec_log'] and parsed['exec_images'] == 1
        assert all(recorded[k] == v for k, v in parsed.items())
        if name == 'rboxc-valgrind':
            assert parsed['errors'] == parsed['non_inherited_descriptors'] == 0
            assert not any(parsed['heap_bytes'].get(k, 0) for k in ('definitely lost', 'indirectly lost', 'possibly lost'))
        processes.append({'implementation': name, **recorded})
assert len(manifest['replays']) == 18 and all(r['reviewed'] for r in manifest['replays'])
assert prior['original_passed'] == 17 and prior['screen_assertions'] == 2022
report = {'scope': 'All 18 distributed screen replays pass across declared private profiles: 17 retain their '
          'original environment; the last retains original file bytes, keystrokes and 223 screen assertions '
          'while disabling external link handlers. Its complete logs contain only the version probe and Less '
          'replay, with no executed opener child. This covers in-document hyperlink behavior and does not '
          'validate external browser/man integration. Original GNU descriptor findings are retained.',
          'binary': prior['binary'], 'binary_sha256': prior['binary_sha256'],
          'test_profile_binary': data['binary'], 'test_profile_sha256': data['binary_sha256'],
          'original_passed': 18, 'original_environment_passes': 17, 'private_opener_environment_passes': 1,
          'screen_assertions': 2245, 'clean_replay_processes': 36,
          'clean_production_processes': prior['clean_production_processes'], 'ownership_contracts': 18,
          'prior_validation': {'path': str(prior_path.relative_to(ROOT)), 'sha256': fingerprint(prior_path)},
          'additional_original': {'path': str(path.relative_to(ROOT)), 'sha256': fingerprint(path)},
          'preserved_driver': {'path': str(old_driver.relative_to(ROOT)), 'sha256': fingerprint(old_driver)},
          'preserved_manifest': {'path': str(old_manifest.relative_to(ROOT)), 'sha256': fingerprint(old_manifest)},
          'manifest_sha256': fingerprint(manifest_path), 'driver_sha256': fingerprint(Path(__file__)),
          'results': processes}
target.write_text(json.dumps(report, indent=2)+'\n')
print('Audited all 18 original replays across declared profiles: 2245 assertions and 36 clean Less logs')
