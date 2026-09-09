#!/usr/bin/env python3
"""Map GNU Tar's registered test groups to reviewed source inputs."""
# SPDX-License-Identifier: GPL-3.0-or-later
from collections import Counter
import hashlib
import json
from pathlib import Path
import re
import subprocess

ROOT = Path(__file__).resolve().parents[1]
digest = lambda p: hashlib.sha256(p.read_bytes()).hexdigest()
manifest = json.loads((ROOT/'inventory/tar-tests.json').read_text())
source = Path(json.loads((ROOT/'inventory/sources.json').read_text())['tar']['source'])
assert digest(source/'tests/testsuite.at') == manifest['registration_sha256']
by_basename = {}
for row in manifest['inputs']:
    assert digest(source/row['path']) == row['sha256']
    basename = Path(row['path']).name
    assert basename not in by_basename
    by_basename[basename] = row
done = subprocess.run(['/bin/bash', str(source/'tests/testsuite'), '--list'],
    capture_output=True, check=True, env={'PATH':'/usr/bin:/bin','LC_ALL':'C'})
assert not done.stderr
listing = ROOT/'evidence/raw/tar-registered-groups.txt'
if listing.exists():
    assert listing.read_bytes() == done.stdout
else:
    listing.write_bytes(done.stdout)
groups = []
for number, basename, line, title in re.findall(r'^\s*(\d+):\s+([^ :]+):(\d+)\s+([^\n]+)$', done.stdout.decode(), re.M):
    number = int(number)
    row = by_basename.get(basename)
    if row is None:
        group = {'number':number, 'source':'tests/'+basename, 'line':int(line), 'title':title.strip(),
                 'source_sha256':None, 'state':'pending-generated-source-review', 'evidence':None}
        generated = manifest.get('generated_groups', {}).get(str(number))
        if generated:
            assert generated['registered_source'] == group['source']
            execution = source/generated['execution_source']
            assert digest(execution) == generated['generated_suite_sha256']
            body = execution.read_text().split('#AT_START_'+str(number)+'\n', 1)[1].split('#AT_STOP_'+str(number)+'\n', 1)[0]
            assert hashlib.sha256(body.encode()).hexdigest() == generated['group_sha256']
            group.update(state=generated['state'], execution_source=generated['execution_source'],
                         generated_suite_sha256=generated['generated_suite_sha256'],
                         group_sha256=generated['group_sha256'], evidence=generated.get('evidence'))
        groups.append(group)
        continue
    assert 0 < int(line) <= len((source/row['path']).read_text().splitlines())
    covered = row.get('autotest_numbers', [row['autotest_number']] if 'autotest_number' in row else [])
    validated = row['state'] == 'original-validated' and number in covered
    groups.append({'number':number, 'source':row['path'], 'line':int(line), 'title':title.strip(),
        'source_sha256':row['sha256'], 'state':'original-validated' if validated else row['state'] if row['state']!='original-validated' else 'pending-group-validation',
        'evidence':row.get('evidence') if validated else None})
assert [r['number'] for r in groups] == list(range(1, len(groups)+1))
assert len(groups) > 0
registered = {r['source'] for r in groups}
result = {'provider':'GNU Tar 1.35',
    'scope':'Read-only --list metadata mapped to each exact original source. Source input counts and registered group counts are separate. A group is validated only when its source inventory records both a passing observation and that exact group number. No tests execute during discovery.',
    'inputs':{str(p):digest(p) for p in [source/'tests/testsuite', source/'tests/testsuite.at', ROOT/'inventory/tar-tests.json', Path(__file__), Path('/bin/bash')]},
    'listing':str(listing.relative_to(ROOT)), 'listing_sha256':digest(listing),
    'source_input_files':len(manifest['inputs']), 'registered_groups':len(groups),
    'group_states':dict(Counter(r['state'] for r in groups)),
    'inputs_without_own_group':sorted(set(r['path'] for r in manifest['inputs'])-registered),
    'registered_inputs_absent_from_distribution':sorted({r['source'] for r in groups if r['source_sha256'] is None}),
    'groups':groups}
(ROOT/'inventory/tar-groups.json').write_text(json.dumps(result,indent=2)+'\n')
print('Registered groups:',len(groups),'; states:',result['group_states'])
print('Inputs without their own group:',result['inputs_without_own_group'])
