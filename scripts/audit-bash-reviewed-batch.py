#!/usr/bin/env python3
"""Audit a bounded original Bash batch without changing installed-release evidence."""
# SPDX-License-Identifier: GPL-3.0-or-later
import argparse
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
parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('--original', type=Path, required=True)
parser.add_argument('--inventory-snapshot', type=Path, required=True)
parser.add_argument('--driver-snapshot', type=Path)
parser.add_argument('--report-name', required=True)
args = parser.parse_args()
assert re.fullmatch(r'[a-z0-9-]+', args.report_name)
target = ROOT/'evidence'/(args.report_name+'.json')
assert not target.exists()
report = json.loads(args.original.read_text())
assert report['complete'] and report['passed'] == report['total'] == report['planned_total']
assert fingerprint(Path(report['binary'])) == report['binary_sha256']
assert fingerprint(ROOT/'build/gnu-bash/bash') == report['gnu_binary_sha256']
assert fingerprint(args.inventory_snapshot) == report['inputs'][str(ROOT/'inventory/bash-tests.json')]
inventory = json.loads(args.inventory_snapshot.read_text())
rows = {r['target']: r for r in inventory['inputs'] if r['reviewed']}
for path, digest in {**report['inputs'], **report['runtime_helpers']}.items():
    actual = Path(path)
    if actual == ROOT/'inventory/bash-tests.json': actual = args.inventory_snapshot
    if actual == ROOT/'tests/bash-original.py' and args.driver_snapshot: actual = args.driver_snapshot
    assert fingerprint(actual) == digest, path
source = Path('/opt/src/bash-5.3/tests')
clean = []
raw = {}
for result in report['results']:
    row = rows[result['selection']]
    assert result['pass']
    expected = source/row['expected']
    assert fingerprint(expected) == row['fixtures'][row['expected']]
    for key, outcome in result['outcomes'].items():
        assert outcome['expected_output_matches']
        assert outcome['status'] == result['outcomes']['gnu']['status']
        for path, digest in outcome['raw'].items():
            assert fingerprint(ROOT/path) == digest
            raw[path] = digest
        output = next(ROOT/p for p in outcome['raw'] if p.endswith('/stdout')).read_bytes()
        if row['output_mode'] == 'drop-expect':
            output = b''.join(line for line in output.splitlines(keepends=True) if not line.startswith(b'expect'))
        assert output == expected.read_bytes()
        assert next(ROOT/p for p in outcome['raw'] if p.endswith('/actual')).read_bytes() == output
        mounts = outcome.get('private_mounts', [])
        assert [m['original'] for m in mounts] == row.get('absolute_helpers', [])
        for mount in mounts:
            assert mount['original'] in ('/bin/echo','/bin/mkdir','/bin/touch','/bin/chmod','/bin/rm')
            assert str(Path(mount['original']).resolve()) == mount['destination']
            assert fingerprint(Path(mount['destination'])) == row['host_inputs'][mount['destination']]
            replacement = ROOT/'build/gnu-coreutils/src/coreutils' if key.startswith('gnu') else Path(report['binary'])
            assert mount['source'] == str(replacement) and fingerprint(replacement) == mount['source_sha256']
        assert bool(outcome['memory']) == key.endswith('-valgrind')
        for memory in outcome['memory']:
            log = ROOT/memory['log']
            assert fingerprint(log) == memory['sha256']
            text = log.read_text()
            pids = set(re.findall(r'^==([0-9]+)==', text, re.M))
            assert len(pids) == 1
            parsed = runner.parse_memory_log(text, pids.pop())
            assert all(memory[k] == v for k, v in parsed.items())
            errors = list(re.finditer(r'ERROR SUMMARY:', text))
            fds = list(re.finditer(r'FILE DESCRIPTORS:', text))
            images = list(re.finditer(r'^==[0-9]+== Command:', text, re.M))
            complete = bool(errors) and len(errors) == len(fds) and (not images or
                errors[-1].start() > images[-1].start() and fds[-1].start() > images[-1].start())
            assert memory['complete'] == complete
            if key == 'rboxc-valgrind':
                assert complete and memory['clean'] and outcome['memory_clean']
                assert parsed['errors'] == parsed['non_inherited_descriptors'] == 0
                assert not any(parsed['heap_bytes'].get(k,0) for k in ('definitely lost','indirectly lost','possibly lost'))
                clean.append({'selection': result['selection'], 'log': memory['log'], 'sha256': memory['sha256']})
            raw[memory['log']] = memory['sha256']
assert len({p['log'] for p in clean}) == len(clean)
target.write_text(json.dumps({'scope':'Selected complete original Bash scripts and nested fixtures match unchanged GNU expected output and GNU exit status. Every raw process log is reparsed. Explicit absolute helper paths use private mounts; their original host files are unchanged. Native helper findings are retained separately. This is batch evidence, not GNU-wide completion.',
    'binary':report['binary'], 'binary_sha256':report['binary_sha256'],
    'original':str(args.original), 'original_sha256':fingerprint(args.original),
    'inventory_snapshot':str(args.inventory_snapshot), 'inventory_sha256':fingerprint(args.inventory_snapshot),
    'original_scripts':report['total'], 'clean_candidate_processes':len(clean),
    'driver_sha256':fingerprint(Path(__file__)), 'processes':clean, 'raw':raw}, indent=2)+'\n')
print('Audited', report['total'], 'original Bash scripts and', len(clean), 'clean candidate processes')
