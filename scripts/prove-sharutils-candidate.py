#!/usr/bin/env python3
"""Bind the Tar/Sharutils candidate to complete current regression evidence."""
# SPDX-License-Identifier: GPL-3.0-or-later
import hashlib
import json
from pathlib import Path
import re
import subprocess

ROOT = Path(__file__).resolve().parents[1]
digest = lambda p: hashlib.sha256(p.read_bytes()).hexdigest()
candidate = ROOT/'target/sharutils-stream-fixed-candidate/release/rboxc'
binary_hash = digest(candidate)
rebuilt = ROOT/'target/sharutils-repro-check/release/rboxc'
assert digest(rebuilt) == binary_hash, 'fresh source rebuild differs from tested executable'
assert len(subprocess.check_output([candidate,'--list'],text=True).splitlines()) == 131
mapping = {
    'smoke':'sharutils-recovered-coreutils-smoke',
    'valgrind':'sharutils-recovered-coreutils-valgrind',
    'behavior':'sharutils-recovered-coreutils-behavior',
    'valgrind-equivalence':'sharutils-recovered-coreutils-equivalence',
    'dispatcher':'sharutils-recovered-dispatcher-all',
    'bc-terminal':'sharutils-recovered-bc-terminal',
    'tar-original':'sharutils-tar-original-final-audited',
    'tar-behavior':'sharutils-recovered-tar-behavior',
    'sharutils-original':'sharutils-original-stream-fixed',
    'sharutils-behavior':'sharutils-behavior-private-output',
    'sharutils-memory-audit':'sharutils-recovered-memory-audit',
}
providers = ('hello','time','which','diffutils','grep','gzip','sed','bc','ed','findutils')
expected_focused = dict(zip(providers,(17,23,46,56,68,56,56,44,44,83)))
for provider in providers:
    mapping[provider+'-behavior'] = 'sharutils-recovered-'+provider+'-behavior'
    if provider != 'which':
        mapping[provider+'-original-reuse'] = 'sharutils-'+provider+'-original-reuse'
reports = {}
for canonical,name in mapping.items():
    path = ROOT/'evidence'/(name+'.json'); data = json.loads(path.read_text())
    assert data['binary_sha256'] == binary_hash
    assert data['passed'] == data['total'] > 0, 'incomplete or failed '+canonical
    if canonical.endswith('-behavior'):
        provider = canonical.removesuffix('-behavior')
        expected = expected_focused.get(provider, {'tar':57,'sharutils':72}.get(provider))
        assert data['total'] == len(data['results']) == expected
        assert all(row['pass'] and row['memory_clean'] for row in data['results'])
        assert data['driver_sha256'] == digest(ROOT/f'tests/{provider}-behavior.py')
    if canonical == 'tar-original':
        assert data['total'] == 84 and data['candidate_processes'] == data['candidate_processes_clean'] > 0
    if canonical == 'sharutils-original':
        assert data['complete'] and data['total'] == 2 and data['candidate_processes'] == 14
    if canonical == 'sharutils-memory-audit':
        assert data['total'] == 86
        assert data['focused_report']['sha256'] == digest(ROOT/'evidence/sharutils-behavior-private-output.json')
        assert data['original_report']['sha256'] == digest(ROOT/'evidence/sharutils-original-stream-fixed.json')
    for helper,expected in data.get('runtime_helpers',{}).items():
        assert digest(Path(helper)) == expected
    reports[canonical] = {'path':str(path.relative_to(ROOT)),'sha256':digest(path),
                          'passed':data['passed'],'total':data['total']}
for canonical,total in {'smoke':428,'valgrind':107,'behavior':318,'valgrind-equivalence':318,'dispatcher':11,'bc-terminal':4}.items():
    assert reports[canonical]['total'] == total
sources = {}
for provider in (*providers,'tar','sharutils'):
    paths = [ROOT/f'evidence/{provider}-link.json',ROOT/f'evidence/{provider}-build-profile.json',ROOT/f'inventory/{provider}-tests.json']
    paths += list((ROOT/'evidence').glob(provider+'*-translation.json'))
    cleanup = ROOT/f'evidence/{provider}-native-cleanup.json'
    if cleanup.exists(): paths.append(cleanup)
    for path in paths: sources[str(path.relative_to(ROOT))] = digest(path)
    linked = json.loads((ROOT/f'evidence/{provider}-link.json').read_text())
    assert not linked['native_command_entries']
    for name,expected in {**linked.get('helper_inputs',{}),**linked.get('original_inputs',{})}.items():
        assert digest(ROOT/name) == expected
        sources[name] = expected
    if linked.get('helper_archive'):
        assert digest(ROOT/linked['helper_archive']) == linked['helper_archive_sha256']
        sources[linked['helper_archive']] = linked['helper_archive_sha256']
    for path in paths:
        if not path.name.endswith('-translation.json'): continue
        entry = json.loads(path.read_text())
        assert entry['translated'] and digest(ROOT/entry['rust_file']) == entry['rust_sha256']
        sources[entry['rust_file']] = entry['rust_sha256']
for name in ('inventory/sources.json','src/registry.rs','src/main.rs','src/bridges/sharutils-owned.rs',
             'Cargo.toml','Cargo.lock','rust-toolchain.toml','build.rs','build/rust-link-inputs.txt',
             'scripts/prove-sharutils-candidate.py','scripts/activate-candidate.py','scripts/update-evidence.py',
             'scripts/audit-sharutils-evidence.py','scripts/validate-retained-provider.py','scripts/validate-original-reuse.py'):
    sources[name] = digest(ROOT/name)
needed = lambda p: sorted(re.findall(r'Shared library: \[([^]]+)\]',subprocess.check_output(['readelf','-d',p],text=True)))
assert needed(candidate) == needed(ROOT/'target/release/rboxc'), 'review added shared dependencies'
proof = {'scope':'Activate Tar and two Sharutils Rust entries after fresh combined regressions. Tar has 84 reviewed original selections, not full-suite certification. Both assigned Sharutils originals and 72 focused checks pass. Earlier providers retain their actual original executable hashes, including Findutils external-child findings, under verified unchanged inputs and fresh focused checks.',
         'candidate':{'path':str(candidate.relative_to(ROOT)),'sha256':binary_hash,'bytes':candidate.stat().st_size},
         'reproducibility':{'rebuilt_binary':str(rebuilt.relative_to(ROOT)),'sha256':digest(rebuilt),
                            'build_log':'evidence/raw/sharutils-repro-check-build.log',
                            'build_log_sha256':digest(ROOT/'evidence/raw/sharutils-repro-check-build.log')},
         'needed_libraries':needed(candidate),'reports':reports,'source_evidence':sources}
path = ROOT/'evidence/sharutils-candidate-proof.json'
assert not path.exists(), 'preserve previous proof'
path.write_text(json.dumps(proof,indent=2)+'\n')
print('Proved 131-command candidate with',len(reports),'current validation reports')
