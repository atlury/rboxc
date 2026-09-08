#!/usr/bin/env python3
"""Audit the original missing fixture and the separate private fixture alias."""
# SPDX-License-Identifier: GPL-3.0-or-later
from collections import Counter
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
path = ROOT/'evidence/sed-stdin-fixture-original.json'
target = ROOT/'evidence/sed-stdin-memory-audit.json'
assert not target.exists(), 'preserve prior audits'
report = json.loads(path.read_text())
source = Path(json.loads((ROOT/'inventory/sources.json').read_text())['sed']['source'])
assert report['script'] == 'testsuite/stdin.sh'
assert fingerprint(source/report['script']) == report['source_sha256']
assert fingerprint(Path(report['binary'])) == report['binary_sha256']
assert fingerprint(ROOT/'build/gnu-sed/sed/sed') == report['gnu_binary_sha256']
assert fingerprint(ROOT/'tests/sed-stdin-original.py') == report['driver_sha256']
assert prepare()[2] == report['prerequisites']
for filename, value in {**report['inputs'], **report['runtime_helpers']}.items():
    assert fingerprint(Path(filename)) == value
original = (source/'testsuite/init.sh').read_text()
anchor = 'setup_ "$@"\n'
addition = 'ln -s stdin-in stdin || framework_failure_ "cannot create private stdin fixture alias"\n'
assert report['framework_anchor'] == anchor and report['framework_addition'] == addition
assert original.count(anchor) == 1
adapted = original.replace(anchor, anchor+addition)
fixture = re.search(rb'cat << \\EOF > stdin-in[^\n]*\n(.*?)\nEOF\n',
                    (source/report['script']).read_bytes(), re.S)[1]+b'\n'
assert bytes.fromhex(report['fixture_bytes']) == fixture
assert report['passed'] == 1 and report['matched'] == report['total'] == 2
assert [r['fixture_profile'] for r in report['results']] == ['unchanged', 'private-alias']
processes = []
for row in report['results']:
    corrected = row['fixture_profile'] == 'private-alias'
    assert row['fixture_valid'] == row['pass'] == corrected and row['matched']
    assert set(row['outcomes']) == {'gnu', 'gnu-valgrind', 'rboxc', 'rboxc-valgrind'}
    for name, outcome in row['outcomes'].items():
        assert outcome['status'] == 0 and outcome['assertions_pass'] and outcome['diagnostic_matches']
        assert outcome['fixture_alias'] == ('stdin-in' if corrected else None)
        expected_error = b'' if corrected else b'cat: stdin: No such file or directory\n'
        assert bytes.fromhex(outcome['stderr']) == expected_error
        assert re.fullmatch(rb'Not removing temporary directory /tmp/rboxc-sed-stdin-[^/\n]+/gt-stdin.sh\.[^/\n]+\n',
                            bytes.fromhex(outcome['stdout']))
        for stream, entry in outcome['streams'].items():
            p = ROOT/entry['path']
            assert fingerprint(p) == entry['sha256'] and p.read_bytes() == bytes.fromhex(outcome[stream])
        init = ROOT/outcome['init']['path']
        assert fingerprint(init) == outcome['init']['sha256']
        assert init.read_text() == (adapted if corrected else original)
        assert set(outcome['snapshot']) == {'stdin-in', 'stdin-out1', 'stdin-out2'}
        for filename, entry in outcome['snapshot'].items():
            p = ROOT/entry['path']
            assert fingerprint(p) == entry['sha256']
            assert p.read_bytes() == bytes.fromhex(entry['contents']) == (fixture if filename == 'stdin-in' else b'')
        instrumented = name.endswith('-valgrind')
        assert len(outcome['memory']) == (6 if instrumented else 0)
        commands = Counter()
        clean = []
        for recorded in outcome['memory']:
            p = ROOT/recorded['log']
            assert fingerprint(p) == recorded['sha256']
            text = p.read_text()
            command = re.findall(r'^==[0-9]+== Command: (.*)$', text, re.M)
            assert len(command) == 1
            commands.update(command)
            parsed = runner.parse_memory_log(text, p.stem, exec_only=True)
            assert all(recorded[k] == v for k, v in parsed.items())
            assert parsed['complete_exec_log'] and parsed['exec_images'] == 1
            passed = parsed['errors'] == 0 and parsed['non_inherited_descriptors'] == 0 and not any(
                parsed['heap_bytes'].get(k, 0) for k in ('definitely lost', 'indirectly lost', 'possibly lost'))
            clean.append(passed)
            if name == 'rboxc-valgrind':
                assert passed
            processes.append({'fixture_profile': row['fixture_profile'], 'implementation': name,
                              'command': command[0], 'memory_clean': passed, **recorded})
        if instrumented:
            assert commands == Counter({'sed d': 2, 'sed G': 2,
                                        r'sed -i -e s/\\r//g stdin-out1': 1,
                                        r'sed -i -e s/\\r//g stdin-out2': 1})
            assert outcome['memory_clean'] == all(clean)
        else:
            assert outcome['memory_clean'] is None
result = {'scope': 'One original passes with an explicit private stdin -> stdin-in fixture alias. '
          'The unchanged original also exits zero but reads a missing fixture and is not counted as a valid-input pass. '
          'All 24 Sed logs are reparsed: six candidate logs per profile are clean. Native GNU findings are retained. '
          'Only Sed processes are instrumented; helper processes are outside memory coverage.',
          'binary': report['binary'], 'binary_sha256': report['binary_sha256'],
          'original_report': {'path': str(path.relative_to(ROOT)), 'sha256': fingerprint(path)},
          'fixture_adapted_original_passed': 1, 'unchanged_valid_input_passed': 0,
          'candidate_processes': 12, 'clean_candidate_processes': 12,
          'fixture_adapted_clean_processes': 6, 'baseline_clean_processes': 6,
          'driver_sha256': fingerprint(Path(__file__)),
          'memory_parser_sha256': fingerprint(ROOT/'tests/gnu/reviewed-original.py'),
          'results': processes}
target.write_text(json.dumps(result, indent=2)+'\n')
print('Audited one fixture-adapted original, its unchanged baseline, and 12 clean candidate Sed logs')
