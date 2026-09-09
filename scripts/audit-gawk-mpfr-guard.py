#!/usr/bin/env python3
"""Verify the configured MPFR guard and its exact unavailable recipe inventory."""
# SPDX-License-Identifier: GPL-3.0-or-later
import hashlib
import importlib.util
import json
from pathlib import Path
import re
import sys

ROOT=Path(__file__).resolve().parents[1]
sys.path.insert(0,str(ROOT/'tests/gnu'))
digest=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
spec=importlib.util.spec_from_file_location('reviewed',ROOT/'tests/gnu/reviewed-original.py')
runner=importlib.util.module_from_spec(spec);spec.loader.exec_module(runner)
target=ROOT/'evidence/gawk-mpfr-guard-validation.json';assert not target.exists()
path=ROOT/'evidence/gawk-mpfr-guard.json';data=json.loads(path.read_text())
assert data['complete'] and data['pass'] and digest(Path(data['binary']))==data['binary_sha256']
for filename,expected in data['inputs'].items():assert digest(Path(filename))==expected
assert len(data['unavailable_recipes'])==21
programs={}
for name,row in data['unavailable_recipes'].items():
    assert not row['executed']
    if row['program']:
        assert digest(Path(row['program']))==row['program_sha256']
        programs[name]=row['program_sha256']
assert len(programs)==20
candidate=[]
for key,outcome in data['outcomes'].items():
    assert outcome['status']==0 and outcome['assertions_pass']
    assert outcome['stdout']=='MPFR tests not supported on this system\n'
    assert digest(ROOT/outcome['driver_log'])==outcome['driver_log_sha256']
    commands=[]
    for recorded in outcome['memory']:
        p=ROOT/recorded['log'];assert digest(p)==recorded['sha256'];text=p.read_text()
        command,=re.findall(r'^==[0-9]+== Command: (.*)$',text,re.M);commands.append(command)
        assert command==recorded['command'] and '-f ' not in command
        pids=set(re.findall(r'^==([0-9]+)==',text,re.M));assert len(pids)==1
        parsed=runner.parse_memory_log(text,pids.pop(),exec_only=True)
        assert all(recorded[k]==v for k,v in parsed.items()) and parsed['complete_exec_log']
        if key=='rboxc-valgrind':
            assert parsed['errors']==parsed['non_inherited_descriptors']==0
            assert not any(parsed['heap_bytes'].get(k,0) for k in ('definitely lost','indirectly lost','possibly lost'))
            candidate.append({'command':command,'log':recorded['log'],'sha256':recorded['sha256'],**parsed})
    if key.endswith('-valgrind'):
        assert len(commands)==2 and 'gawk --version' in commands
        assert sum('/MPFR/' in c for c in commands)==1
assert len(candidate)==2
target.write_text(json.dumps({'scope':'The unchanged GNU capability guard reports MPFR unavailable for native GNU and Rboxc. The two candidate guard invocations are clean under Valgrind. Twenty distributed programs and one inline recipe are unavailable in the current configuration; none is executed or counted as passing. This is configuration accounting, not implementation of MPFR support.',
    'original_report':{'path':str(path.relative_to(ROOT)),'sha256':digest(path)},'binary_sha256':data['binary_sha256'],
    'unavailable_programs':programs,'inline_recipe':'mpfruplus','candidate_logs':candidate,'full_acceptance':False,'driver_sha256':digest(Path(__file__))},indent=2)+'\n')
print('Verified MPFR guard: two clean processes; twenty programs and one inline recipe unavailable')
