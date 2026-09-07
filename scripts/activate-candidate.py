#!/usr/bin/env python3
"""Activate a hash-verified candidate while retaining binaries and observations."""
# SPDX-License-Identifier: GPL-3.0-or-later
import argparse
import datetime
import hashlib
import json
import os
from pathlib import Path
import re
import shutil

ROOT = Path(__file__).resolve().parents[1]


def fingerprint(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def preserve(source, destination):
    destination.parent.mkdir(parents=True, exist_ok=True)
    if destination.exists():
        assert fingerprint(source) == fingerprint(destination), 'backup already contains different bytes'
    else:
        shutil.copy2(source, destination)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('proof', type=Path)
    parser.add_argument('--name', required=True)
    args = parser.parse_args()
    assert re.fullmatch(r'[a-z0-9][a-z0-9-]*', args.name)
    proofpath = args.proof.resolve(strict=True)
    proof = json.loads(proofpath.read_text())
    candidate = ROOT/proof['candidate']['path']
    digest = fingerprint(candidate)
    assert digest == proof['candidate']['sha256']
    assert candidate.stat().st_size == proof['candidate']['bytes']
    for path, expected in proof.get('source_evidence', {}).items():
        assert fingerprint(ROOT/path) == expected, 'recorded source evidence changed'
    installed = ROOT/'target/release/rboxc'
    old = fingerprint(installed)
    assert old != digest, 'candidate is already active'
    record = ROOT/f'evidence/{args.name}-activation.json'
    assert not record.exists(), 'activation record already exists'
    prepared = []
    assert {'smoke', 'valgrind', 'behavior', 'dispatcher', 'valgrind-equivalence'} <= set(proof['reports'])
    for canonical, item in proof['reports'].items():
        assert re.fullmatch(r'[a-z0-9][a-z0-9-]*', canonical)
        source = ROOT/item['path']
        assert fingerprint(source) == item['sha256']
        data = json.loads(source.read_text())
        assert data['binary_sha256'] == digest
        assert (data['passed'], data['total']) == (item['passed'], item['total'])
        if canonical in ('smoke', 'valgrind', 'behavior', 'dispatcher', 'valgrind-equivalence'):
            assert data['passed'] == data['total'] > 0
        for path, expected in data.get('runtime_helpers', {}).items():
            assert fingerprint(Path(path)) == expected, 'runtime helper changed'
        origin = {'source_report': str(source.relative_to(ROOT)),
                  'source_report_sha256': item['sha256'], 'candidate_binary': str(candidate),
                  'same_executable_bytes': True, 'rerun': False}
        if 'binary' in data:
            data['binary'] = str(installed)
        if 'runtime_helpers' in data:
            data['runtime_helpers'] = {
                str(installed.parent/Path(p).name) if Path(p).parent == candidate.parent else p: value
                for p, value in data['runtime_helpers'].items()}
        data['activation'] = origin
        prepared.append((ROOT/f'evidence/{canonical}.json', data, origin))
    assert fingerprint(candidate.parent/'libstdbuf.so') == fingerprint(installed.parent/'libstdbuf.so')
    backup = ROOT/'target/history'/old
    for name in ('rboxc', 'libstdbuf.so'):
        preserve(installed.parent/name, backup/name)
    reportbackup = ROOT/'evidence/raw'/f'before-{args.name}-activation-{old}'
    baseline = {}
    for destination, data, origin in prepared:
        item = {'destination': str(destination.relative_to(ROOT)), 'source': origin}
        if destination.exists():
            saved = reportbackup/destination.name
            preserve(destination, saved)
            item['previous'] = {'path': str(saved.relative_to(ROOT)), 'sha256': fingerprint(saved)}
        baseline[destination.stem] = item
    temporary = installed.with_name('rboxc.activate')
    shutil.copy2(candidate, temporary)
    assert fingerprint(temporary) == digest
    os.replace(temporary, installed)
    for destination, data, origin in prepared:
        temporary = destination.with_suffix('.activate.json')
        temporary.write_text(json.dumps(data, indent=2)+'\n')
        os.replace(temporary, destination)
        baseline[destination.stem]['sha256'] = fingerprint(destination)
    activation = {'scope': 'Hash-verified activation; observations reused by executable/helper byte identity, not rerun. Original reports and previous binaries are retained.',
                  'activated_utc': datetime.datetime.now(datetime.timezone.utc).isoformat(),
                  'proof': {'path': str(proofpath.relative_to(ROOT)), 'sha256': fingerprint(proofpath)},
                  'previous_binary_sha256': old, 'binary_backup': str((backup/'rboxc').relative_to(ROOT)),
                  'activated_binary': {'path': str(installed.relative_to(ROOT)), 'bytes': installed.stat().st_size, 'sha256': digest},
                  'baseline': baseline}
    record.write_text(json.dumps(activation, indent=2)+'\n')
    print('Activated', digest, installed.stat().st_size, 'bytes')


if __name__ == '__main__':
    main()
