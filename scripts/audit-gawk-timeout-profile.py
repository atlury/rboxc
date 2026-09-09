#!/usr/bin/env python3
"""Verify the supplemental timeout profile without changing strict coverage."""
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
path = ROOT/'evidence/gawk-timeout-native-children.json'
report = json.loads(path.read_text())
target = ROOT/'evidence/gawk-timeout-native-children-validation.json'
assert not target.exists()
assert report['complete'] and report['passed'] == report['total'] == 1
assert fingerprint(Path(report['binary'])) == report['binary_sha256']
assert fingerprint(ROOT/'build/gnu-gawk/gawk') == report['gnu_binary_sha256']
for p, h in {**report['inputs'], **report['runtime_helpers']}.items():
    assert fingerprint(Path(p)) == h
row, = report['results']
assert row['name'] == row['original_make_target'] == 'timeout'
assert row['pass'] and row['equivalent']
logs = {}
for key, outcome in row['outcomes'].items():
    assert outcome['original_assertions_pass'] and outcome['status'] == 0
    for p, h in outcome['raw'].items():
        f = ROOT/p
        assert fingerprint(f) == h
        if f.name == 'stdout': assert f.read_bytes() == b'timeout\n'
        elif f.name == 'stderr': assert f.read_bytes() == b''
        else: assert f.name == 'memory.log'
    if not key.endswith('-valgrind'): continue
    log = ROOT/outcome['memory_log']
    text = log.read_text()
    assert re.findall(r'^==[0-9]+== Command: (.*)$', text, re.M) == ['gawk -f timeout.awk']
    pids = set(re.findall(r'^==([0-9]+)==', text, re.M))
    assert len(pids) == 1
    parsed = runner.parse_memory_log(text, pids.pop(), exec_only=True)
    assert parsed == outcome['memory'] and parsed['complete_exec_log']
    assert parsed['errors'] == parsed['non_inherited_descriptors'] == 0
    assert not any(parsed['heap_bytes'].get(k, 0) for k in ('definitely lost', 'indirectly lost', 'possibly lost'))
    logs[key] = {'path': outcome['memory_log'], 'sha256': fingerprint(log), **parsed}
assert len(logs) == 2
target.write_text(json.dumps({'scope': 'Original timeout assertions pass in four profiles with native children. Only the two GNU/Rboxc Gawk logs are instrumented and audited; native child memory is outside this supplemental profile. No strict full-child original count is added.',
    'binary_sha256': report['binary_sha256'], 'report_sha256': fingerprint(path),
    'clean_candidate_processes': 1, 'additional_strict_original_passes': 0,
    'driver_sha256': fingerprint(Path(__file__)), 'logs': logs}, indent=2)+'\n')
print('Audited unchanged timeout output and two Gawk memory logs with native children')
