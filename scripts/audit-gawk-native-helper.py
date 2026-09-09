#!/usr/bin/env python3
"""Audit GNU sort helper provenance and preserved before/after recipe findings."""
# SPDX-License-Identifier: GPL-3.0-or-later
import hashlib
import importlib.util
import json
from pathlib import Path
import re
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0,str(ROOT/'tests/gnu'))
digest = lambda p: hashlib.sha256(p.read_bytes()).hexdigest()
target = ROOT/'evidence/gawk-native-helper-validation.json'
assert not target.exists()
spec = importlib.util.spec_from_file_location('reviewed', ROOT/'tests/gnu/reviewed-original.py')
runner = importlib.util.module_from_spec(spec); spec.loader.exec_module(runner)
profile_path = ROOT/'evidence/gawk-sort-helper.json'
profile = json.loads(profile_path.read_text())
for p,h in profile['inputs'].items(): assert digest(Path(p)) == h, p
for p,h in profile['artifacts'].items(): assert digest(ROOT/p) == h, p
original = ROOT/'build/gnu-coreutils/src/libsinglebin_sort.a'
adapted = ROOT/'build/gawk-sort-helper/libsinglebin_sort.a'
members = subprocess.check_output(['ar','t',original],text=True).splitlines()
assert members == subprocess.check_output(['ar','t',adapted],text=True).splitlines()
changed = []
for member in members:
    if subprocess.check_output(['ar','p',original,member]) != subprocess.check_output(['ar','p',adapted,member]):
        changed.append(member)
assert changed == ['libsinglebin_sort_a-sort.o']
source = Path('/opt/src/coreutils-9.11/src/sort.c').read_text()
anchor = '  main_exit (EXIT_SUCCESS);\n}'
assert source.count(anchor) == 1
assert (ROOT/'build/gawk-sort-helper/sort.c').read_text() == source.replace(anchor,'  if (!files_from)\n    free (files);\n\n'+anchor)
reports = {}
old_findings = []
new_processes = []
for name in ('gawk-native-child-original','gawk-native-child-clean-original'):
    path = ROOT/'evidence'/(name+'.json'); data = json.loads(path.read_text())
    assert data['complete'] and data['total'] == data['planned_total'] == 6
    assert data['binary_sha256'] == digest(Path(data['binary']))
    for filename,expected in data['inputs'].items():
        p = Path(filename)
        if p == ROOT/'tests/gawk-original.py' and name == 'gawk-native-child-original':
            p = ROOT/'evidence/raw/gawk-first-native-children-driver.py'
        assert digest(p) == expected, str(p)
    for row in data['results']:
        for key,outcome in row['outcomes'].items():
            assert outcome['assertions_pass'] and outcome['status'] == 0
            assert digest(ROOT/outcome['driver_log']) == outcome['driver_log_sha256']
            for recorded in outcome['memory']:
                p = ROOT/recorded['log']; assert digest(p) == recorded['sha256']
                text = p.read_text(); pids = set(re.findall(r'^==([0-9]+)==',text,re.M)); assert len(pids)==1
                parsed = runner.parse_memory_log(text,pids.pop(),exec_only=True)
                assert all(recorded[k] == v for k,v in parsed.items()) and parsed['complete_exec_log']
                if key != 'rboxc-valgrind': continue
                clean = parsed['errors']==parsed['non_inherited_descriptors']==0 and not any(parsed['heap_bytes'].get(k,0) for k in ('definitely lost','indirectly lost','possibly lost'))
                if name == 'gawk-native-child-clean-original':
                    assert clean
                    new_processes.append({'selection':row['selection'],'log':recorded['log'],'sha256':recorded['sha256'],'role':recorded['role']})
                elif not clean:
                    assert row['selection'] in ('longwrds','nsidentifier','spacere')
                    assert recorded['role']=='child-dependency' and recorded['canonical_command']=='sort'
                    assert parsed['errors']==1 and parsed['non_inherited_descriptors']==0
                    assert parsed['heap_bytes']['definitely lost']==8 and parsed['heap_bytes']['indirectly lost']==parsed['heap_bytes']['possibly lost']==0
                    old_findings.append({'selection':row['selection'],'log':recorded['log'],'sha256':recorded['sha256'],**parsed})
    reports[name] = {'path':str(path.relative_to(ROOT)),'sha256':digest(path),'binary_sha256':data['binary_sha256'],'passed':data['passed']}
assert len({r['binary_sha256'] for r in reports.values()}) == 1
assert len(old_findings)==3 and len(new_processes)==16
assert sum(p['role']=='gawk' for p in new_processes)==6
assert reports['gawk-native-child-original']['passed']==3
assert reports['gawk-native-child-clean-original']['passed']==6
target.write_text(json.dumps({'scope':'All six unchanged original recipes pass their assertions both before and after private native helper cleanup. The three earlier candidate failures were exactly eight lost bytes each in the pinned native GNU sort helper. Only its native sort object changes; the Rboxc candidate is unchanged. All sixteen final candidate and child logs are complete and clean. Native GNU Gawk baseline findings remain preserved separately.',
    'helper_profile':{'path':str(profile_path.relative_to(ROOT)),'sha256':digest(profile_path)},
    'changed_archive_members':changed,'reports':reports,'preserved_native_helper_findings':old_findings,
    'clean_processes':new_processes,'driver_sha256':digest(Path(__file__))},indent=2)+'\n')
print('Audited six originals, six clean Gawk processes and ten clean child processes')
