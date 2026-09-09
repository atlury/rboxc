#!/usr/bin/env python3
"""Verify the cat helper build and its narrowly resolved original pipe case."""
# SPDX-License-Identifier: GPL-3.0-or-later
import hashlib
import importlib.util
import json
from pathlib import Path
import re
import subprocess
import sys

ROOT=Path(__file__).resolve().parents[1]
sys.path.insert(0,str(ROOT/'tests/gnu'))
digest=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
target=ROOT/'evidence/gawk-cat-helper-validation.json'
assert not target.exists()
spec=importlib.util.spec_from_file_location('reviewed',ROOT/'tests/gnu/reviewed-original.py')
runner=importlib.util.module_from_spec(spec);spec.loader.exec_module(runner)
profile_path=ROOT/'evidence/gawk-cat-helper.json';profile=json.loads(profile_path.read_text())
for filename,expected in profile['inputs'].items():assert digest(Path(filename))==expected
for filename,expected in profile['artifacts'].items():assert digest(ROOT/filename)==expected
assert profile['compile_command'].count('-Dlint')==1
source=Path('/opt/src/coreutils-9.11/src/cat.c')
assert profile['compile_command'][-1]==str(source)
assert '#ifdef lint\n  alignfree (outbuf);\n  alignfree (inbuf);\n#endif' in source.read_text()
original=ROOT/'build/gnu-coreutils/src/libsinglebin_cat.a';adapted=ROOT/'build/gawk-cat-helper/libsinglebin_cat.a'
members=subprocess.check_output(['ar','t',original],text=True).splitlines()
assert members==subprocess.check_output(['ar','t',adapted],text=True).splitlines()
changed=[m for m in members if subprocess.check_output(['ar','p',original,m])!=subprocess.check_output(['ar','p',adapted,m])]
assert changed==['libsinglebin_cat_a-cat.o']
observations={}
for name in ('gawk-pipe-lifetime-baseline-original','gawk-pipe-cat-lint-original'):
    path=ROOT/'evidence'/(name+'.json');data=json.loads(path.read_text())
    assert data['complete'] and data['total']==data['planned_total']==11
    assert digest(Path(data['binary']))==data['binary_sha256']
    for filename,expected in data['inputs'].items():
        p=Path(filename)
        if p==ROOT/'tests/gawk-original.py' and name=='gawk-pipe-lifetime-baseline-original':p=ROOT/'evidence/raw/gawk-pipe-before-cat-helper-driver.py'
        assert digest(p)==expected,str(p)
    row=next(r for r in data['results'] if r['selection']=='childin')
    assert all(o['assertions_pass'] for o in row['outcomes'].values())
    records=[]
    for key,outcome in row['outcomes'].items():
        assert digest(ROOT/outcome['driver_log'])==outcome['driver_log_sha256']
        for memory in outcome['memory']:
            p=ROOT/memory['log'];assert digest(p)==memory['sha256']
            text=p.read_text();pids=set(re.findall(r'^==([0-9]+)==',text,re.M));assert len(pids)==1
            parsed=runner.parse_memory_log(text,pids.pop(),exec_only=True)
            assert all(memory[k]==v for k,v in parsed.items()) and parsed['complete_exec_log']
            if key!='rboxc-valgrind':continue
            if name=='gawk-pipe-lifetime-baseline-original' and memory['canonical_command']=='cat':
                assert parsed['errors']==1 and parsed['heap_bytes']['definitely lost']==262144
            else:
                assert parsed['errors']==parsed['non_inherited_descriptors']==0
                assert not any(parsed['heap_bytes'].get(k,0) for k in ('definitely lost','indirectly lost','possibly lost'))
            records.append({'command':memory['command'],'log':memory['log'],'sha256':memory['sha256'],**parsed})
    assert len(records)==3
    observations[name]={'path':str(path.relative_to(ROOT)),'sha256':digest(path),'candidate_sha256':data['binary_sha256'],'childin_pass':row['pass'],'candidate_processes':records}
assert len({r['candidate_sha256'] for r in observations.values()})==1
assert not observations['gawk-pipe-lifetime-baseline-original']['childin_pass']
assert observations['gawk-pipe-cat-lint-original']['childin_pass']
coverage_path=ROOT/'evidence/gawk-pipe-lifetime-coverage.json';coverage=json.loads(coverage_path.read_text())
assert len(coverage['open_originals'])==6
target.write_text(json.dumps({'scope':'GNU cat source is unchanged and only its existing lint normal-exit buffer cleanup is enabled. The unchanged childin original gains a strict pass: Gawk, shell and cat logs are complete and clean. The earlier 262144-byte native cat buffer leak is preserved. Six other original profiles still have descriptor/probe findings and are not claimed fixed; candidate code and bytes are unchanged.',
    'helper_profile':{'path':str(profile_path.relative_to(ROOT)),'sha256':digest(profile_path)},'changed_archive_members':changed,
    'observations':observations,'coverage':{'path':str(coverage_path.relative_to(ROOT)),'sha256':digest(coverage_path)},
    'open_originals':sorted(coverage['open_originals']),'driver_sha256':digest(Path(__file__))},indent=2)+'\n')
print('Verified unchanged GNU cat cleanup flag and three clean childin processes; six other profiles remain open')
