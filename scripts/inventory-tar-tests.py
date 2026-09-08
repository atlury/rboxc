#!/usr/bin/env python3
"""Discover every original Tar Autotest input while preserving review evidence."""
# SPDX-License-Identifier: GPL-3.0-or-later
import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
source = Path(json.loads((ROOT/'inventory/sources.json').read_text())['tar']['source'])
target = ROOT/'inventory/tar-tests.json'
data = json.loads(target.read_text())
fingerprint = lambda path: hashlib.sha256(path.read_bytes()).hexdigest()
assert fingerprint(source/'tests/testsuite.at') == data['registration_sha256']
known = {row['path']: row for row in data['inputs']}
assert len(known) == len(data['inputs']), 'duplicate original input'
found = {str(path.relative_to(source)): fingerprint(path)
         for path in (source/'tests').rglob('*.at')}
assert known.keys() <= found.keys(), 'preserve previous input records'
for name, expected in found.items():
    if name in known:
        assert known[name]['sha256'] == expected, 'source upgrade needs explicit review'
    else:
        known[name] = {'path': name, 'sha256': expected,
                       'reviewed': False, 'state': 'pending-review'}
data['scope'] = ('Original Autotest inputs discovered recursively, with per-input review and '
                 'validation evidence preserved. Discovery does not execute tests. Each selection '
                 'requires review and a bounded fixture profile. SELinux excluded from this port.')
data['discovery'] = {'pattern': 'tests/**/*.at', 'files': len(found),
                     'registration': 'tests/testsuite.at'}
data['inputs'] = [known[name] for name in sorted(known)]
target.write_text(json.dumps(data, indent=2)+'\n')
print('Inventoried', len(found), 'original Tar inputs, preserving existing reviews')
