#!/usr/bin/env python3
"""Preserve native descriptor-limit coverage and Valgrind startup limitations."""
# SPDX-License-Identifier: GPL-3.0-or-later
import hashlib
import importlib.util
import json
from pathlib import Path
import re
import sys

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0,str(ROOT/'tests/gnu'))
digest = lambda p: hashlib.sha256(p.read_bytes()).hexdigest()
spec = importlib.util.spec_from_file_location('reviewed',ROOT/'tests/gnu/reviewed-original.py')
runner = importlib.util.module_from_spec(spec); spec.loader.exec_module(runner)
target = ROOT/'evidence/tar-scarce-descriptors-validation.json'
assert not target.exists()
versions = {digest(p):p for p in [ROOT/'tests/tar-original.py',
    ROOT/'evidence/raw/tar-original-scarce-shell-driver.py',
    ROOT/'evidence/raw/tar-original-scarce-bash-driver.py']}
reports = {}
candidate_logs = []
for name in ('tar-scarce-descriptors-original','tar-scarce-bash-original','tar-scarce-direct-valgrind-original'):
    path = ROOT/'evidence'/(name+'.json'); data = json.loads(path.read_text())
    assert data['complete'] and data['total']==data['planned_total']==1 and data['passed']==0
    assert digest(Path(data['binary'])) == data['binary_sha256']
    for filename,expected in data['inputs'].items():
        p = Path(filename)
        if p == ROOT/'tests/tar-original.py': p = versions[expected]
        assert digest(p)==expected, str(p)
    row, = data['results']; assert row['selection']=='extrac11' and row['autotest_number']==94
    observations = {}
    for key,outcome in row['outcomes'].items():
        assert not outcome['timed_out'] and not outcome['child_wait_timeout']
        p = ROOT/outcome['driver_log']; assert digest(p)==outcome['driver_log_sha256']
        detail = p.parent/'suite/094/testsuite.log'; text = detail.read_text()
        native_pass = name!='tar-scarce-descriptors-original' and not key.endswith('-valgrind')
        assert outcome['assertions_pass']==native_pass
        assert outcome['status']==(0 if native_pass else 1)
        if not native_pass:
            if name=='tar-scarce-direct-valgrind-original':
                assert 'Valgrind: FATAL: Private file creation failed.' in text
                assert 'current file descriptor limit is -2.' in text
            else:
                assert '/bin/sh: 0: 3: Invalid argument' in text
        commands = []
        for memory in outcome['memory']:
            log = ROOT/memory['log']; assert digest(log)==memory['sha256']
            contents = log.read_text(); pids = set(re.findall(r'^==([0-9]+)==',contents,re.M)); assert len(pids)==1
            parsed = runner.parse_memory_log(contents,pids.pop(),exec_only=True)
            assert all(memory[k]==v for k,v in parsed.items()) and parsed['complete_exec_log']
            command, = re.findall(r'^==[0-9]+== Command: (.*)$',contents,re.M)
            commands.append(command)
            if key=='rboxc-valgrind':
                assert parsed['errors']==parsed['non_inherited_descriptors']==0
                assert not any(parsed['heap_bytes'].get(k,0) for k in ('definitely lost','indirectly lost','possibly lost'))
                candidate_logs.append({'profile':name,'command':command,'log':memory['log'],'sha256':memory['sha256'],**parsed})
        if key.endswith('-valgrind'):
            assert sorted(commands)==sorted(['tar --version','tar -cf archive1.tar a','tar -xf archive1.tar -C dest1 a'])
        observations[key]={'native_original_pass':native_pass,'status':outcome['status'],
            'detail_log':str(detail.relative_to(ROOT)),'detail_log_sha256':digest(detail),'instrumented_commands':commands}
    reports[name]={'path':str(path.relative_to(ROOT)),'sha256':digest(path),'binary_sha256':data['binary_sha256'],'observations':observations}
assert len({r['binary_sha256'] for r in reports.values()})==1
assert len(candidate_logs)==9
target.write_text(json.dumps({'scope':'The unchanged original descriptor-limit assertions pass for native GNU and Rboxc using a Bash launcher. Dash wrapper startup fails at the original low limit. Bypassing the distro Valgrind shell wrapper preserves its environment but reveals Valgrind itself cannot start at limit 10. Instrumented coverage includes only the version prerequisite and limit-100 create/extract pair; neither limit-4 nor limit-10 Tar execution is claimed. Three distinct attempts each retain three clean candidate logs. This is partial coverage, not a strict original/Valgrind pass.',
    'native_original_pass':True,'strict_original_valgrind_pass':False,'full_acceptance':False,
    'reports':reports,'candidate_logs':candidate_logs,'driver_sha256':digest(Path(__file__))},indent=2)+'\n')
print('Verified native original pass; Valgrind limit-4/10 coverage remains unavailable')
