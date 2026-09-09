#!/usr/bin/env python3
"""Preserve and verify the missing-prerequisite run against its completed rerun."""
# SPDX-License-Identifier: GPL-3.0-or-later
import importlib.util
import json
from pathlib import Path
import re
import sys
import tarfile

ROOT = Path(__file__).resolve().parents[1]
sys.path[:0] = [str(ROOT/'tests'), str(ROOT/'tests/gnu')]
from comparison_profile import fingerprint
spec = importlib.util.spec_from_file_location('reviewed', ROOT/'tests/gnu/reviewed-original.py')
runner = importlib.util.module_from_spec(spec)
spec.loader.exec_module(runner)
target = ROOT/'evidence/tar-incremental-recovery.json'
assert not target.exists()
paths = {n:ROOT/'evidence'/(n+'.json') for n in ('tar-pending-incremental-original',
    'tar-pending-incremental-ready-original', 'tar-test-prerequisites', 'tar-pending-current-audited')}
reports = {n:json.loads(p.read_text()) for n,p in paths.items()}
initial = reports['tar-pending-incremental-original']
final = reports['tar-pending-incremental-ready-original']
assert initial['complete'] and initial['passed']==6 and initial['total']==21
assert final['complete'] and final['passed']==final['total']==21
assert initial['binary_sha256']==final['binary_sha256']==fingerprint(Path(final['binary']))
archive = ROOT/'evidence/raw/tar-original-before-ckmtime.py'
retained = {str(archive.relative_to(ROOT)):fingerprint(archive)}
for name,expected in initial['inputs'].items():
    actual = archive if name==str(ROOT/'tests/tar-original.py') else Path(name)
    assert fingerprint(actual)==expected
skips = []
native_findings = []
for row in initial['results']:
    skipped = not row['pass']
    assert skipped == (not row['selection'].startswith('rename'))
    for key,outcome in row['outcomes'].items():
        log = ROOT/outcome['driver_log']
        assert fingerprint(log)==outcome['driver_log_sha256'] and outcome['status']==0
        status = re.findall(r'^\s*(\d+):\s+.*?\s+(ok|skipped)(?: \([^\n]*\))?\s*$',log.read_text(),re.M)
        assert status==[(str(row['autotest_number']),'skipped' if skipped else 'ok')]
        retained[str(log.relative_to(ROOT))]=fingerprint(log)
        if skipped:
            suite=log.parent/'suite'/str(row['autotest_number'])/'testsuite.log'
            assert 'ckmtime: command not found' in suite.read_text()
            retained[str(suite.relative_to(ROOT))]=fingerprint(suite)
        for memory in outcome['memory']:
            path=ROOT/memory['log'];assert fingerprint(path)==memory['sha256']
            parsed=runner.parse_memory_log(path.read_text(),path.stem,exec_only=True)
            assert all(memory[k]==v for k,v in parsed.items())
            assert parsed['complete_exec_log']
            if key=='rboxc-valgrind':
                assert parsed['errors']==parsed['non_inherited_descriptors']==0
                assert not any(parsed['heap_bytes'].get(k,0) for k in ('definitely lost','indirectly lost','possibly lost'))
            elif parsed['errors'] or parsed['non_inherited_descriptors']:
                native_findings.append({'selection':row['selection'], 'log':memory['log'], 'sha256':memory['sha256'], **parsed})
    if skipped:skips.append(row['selection'])
assert len(skips)==15
prerequisites=reports['tar-test-prerequisites']
assert prerequisites['passed']==prerequisites['clean_helper_processes']==2
for name,expected in prerequisites['inputs'].items():assert fingerprint(Path(name))==expected
for row in prerequisites['results']:
    memory=row['outcomes']['valgrind'];assert fingerprint(ROOT/memory['log'])==memory['sha256']
combined=reports['tar-pending-current-audited']
assert combined['passed']==combined['total']==34 and combined['candidate_processes_clean']==401
pin=json.loads((ROOT/'inventory/sources.json').read_text())['tar']
distribution=ROOT/'build/tar-1.35.tar.xz'
assert fingerprint(distribution)==pin['archive_sha256']
with tarfile.open(distribution) as source_archive:
    names=set(source_archive.getnames())
    assert all('tar-1.35/tests/'+n not in names for n in ('exclude17.at','exclude18.at'))
target.write_text(json.dumps({'scope':'Fifteen initial selections skipped because the original ckmtime prerequisite was absent; the six rename passes remain historical observations. The completed unchanged rerun passes all 21 groups after building the original helper. No skipped run adds passing coverage. The disjoint current-candidate consolidation contains 34 groups and 401 clean Tar processes.',
    'reports':{str(p.relative_to(ROOT)):fingerprint(p) for p in paths.values()},
    'retained_initial_artifacts':retained, 'initial_skipped_selections':skips,
    'native_findings_preserved':native_findings,
    'original_source_archive_sha256':pin['archive_sha256'],
    'registered_sources_absent_from_signed_archive':['tests/exclude17.at','tests/exclude18.at'],
    'driver_sha256':fingerprint(Path(__file__)), 'full_acceptance_complete':False},indent=2)+'\n')
print('Verified 15 prerequisite skips, the completed 21-group rerun and 34-group consolidation')
