#!/usr/bin/env python3
"""Record the immutable Findutils candidate and its completed validation inputs."""
# SPDX-License-Identifier: GPL-3.0-or-later
import hashlib
import json
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[1]
digest = lambda p: hashlib.sha256(p.read_bytes()).hexdigest()
candidate = ROOT/'target/findutils-exit-cleanup/release/rboxc'
binary_hash = digest(candidate)
assert len(subprocess.check_output([candidate, '--list'], text=True).splitlines()) == 128
original = json.loads((ROOT/'evidence/findutils-original-final.json').read_text())
audit = json.loads((ROOT/'evidence/findutils-original-final-memory-audit.json').read_text())
assert original['total'] == original['applet_assertions_and_memory_passed'] == 233
assert audit['selections_total'] == audit['selections_passed'] == 233
assert audit['passed'] == audit['total'] > 0
assert audit['original_report_sha256'] == digest(ROOT/'evidence/findutils-original-final.json')
mapping = {
    'smoke': 'findutils-combined-smoke', 'valgrind': 'findutils-combined-valgrind',
    'behavior': 'findutils-combined-coreutils-behavior',
    'valgrind-equivalence': 'findutils-combined-coreutils-equivalence',
    'dispatcher': 'findutils-combined-dispatcher',
    'findutils-behavior': 'findutils-behavior-exit-cleanup',
    'findutils-original': 'findutils-original-final',
    'findutils-original-memory-audit': 'findutils-original-final-memory-audit',
    'bc-terminal': 'findutils-combined-bc-terminal',
}
providers = ('hello', 'time', 'which', 'diffutils', 'grep', 'gzip', 'sed', 'bc', 'ed')
for provider in providers:
    mapping[provider+'-behavior'] = 'findutils-combined-'+provider+'-behavior'
    if provider != 'which':
        mapping[provider+'-original-reuse'] = 'findutils-'+provider+'-original-reuse'
reports = {}
for canonical, name in mapping.items():
    path = ROOT/'evidence'/(name+'.json')
    data = json.loads(path.read_text())
    assert data['binary_sha256'] == binary_hash
    if canonical != 'findutils-original':
        assert data['passed'] == data['total'] > 0
    reports[canonical] = {'path': str(path.relative_to(ROOT)), 'sha256': digest(path),
                          'passed': data['passed'], 'total': data['total']}
sources = {}
for provider in (*providers, 'findutils'):
    paths = [ROOT/f'evidence/{provider}-link.json', ROOT/f'evidence/{provider}-build-profile.json', ROOT/f'inventory/{provider}-tests.json']
    paths += list((ROOT/'evidence').glob(provider+'*-translation.json'))
    cleanup = ROOT/f'evidence/{provider}-native-cleanup.json'
    if cleanup.exists():
        paths.append(cleanup)
    for path in paths:
        sources[str(path.relative_to(ROOT))] = digest(path)
linked = json.loads((ROOT/'evidence/findutils-link.json').read_text())
assert not linked['native_command_entries']
for path, expected in {**linked['helper_inputs'], **linked['original_inputs']}.items():
    assert digest(ROOT/path) == expected
    sources[path] = expected
for command in ('find', 'xargs', 'locate'):
    entry = json.loads((ROOT/f'evidence/findutils-{command}-translation.json').read_text())
    assert entry['rust_sha256'] == linked['rust_source_sha256'][command] == digest(ROOT/entry['rust_file'])
    sources[entry['rust_file']] = entry['rust_sha256']
for name in ('inventory/sources.json', 'src/registry.rs', 'src/main.rs', 'Cargo.toml', 'Cargo.lock', 'build.rs',
             'build/rust-link-inputs.txt', 'scripts/validate-retained-provider.py', 'scripts/validate-original-reuse.py',
             'scripts/audit-findutils-evidence.py', 'scripts/consolidate-findutils-original.py',
             'scripts/prove-findutils-candidate.py', 'evidence/findutils-build-exit-cleanup.json'):
    sources[name] = digest(ROOT/name)
proof = {
    'scope': 'Activate three GNU Findutils Rust entries after 233 reviewed original selections with clean applet processes, 83 focused checks, and complete combined regressions. Four excluded originals, external native child findings, and the declared large-exec child-instrumentation limit remain explicit. updatedb/frcode are pending. Prior providers retain their real original binary hashes under unchanged-input validation, not a claimed rerun.',
    'candidate': {'path': str(candidate.relative_to(ROOT)), 'sha256': binary_hash, 'bytes': candidate.stat().st_size},
    'reports': reports, 'source_evidence': sources,
}
path = ROOT/'evidence/findutils-candidate-proof.json'
assert not path.exists(), 'preserve existing proof'
path.write_text(json.dumps(proof, indent=2)+'\n')
print('Proved candidate', binary_hash, 'with', len(reports), 'validation reports')
