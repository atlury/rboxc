#!/usr/bin/env python3
"""Audit the Gawk parser-source ownership candidate and its preserved baselines."""
# SPDX-License-Identifier: GPL-3.0-or-later
import importlib.util
import json
from pathlib import Path
import re
import subprocess
import sys
ROOT=Path(__file__).resolve().parents[1]
sys.path.insert(0,str(ROOT/'tests'));sys.path.insert(0,str(ROOT/'tests/gnu'))
from comparison_profile import fingerprint
spec=importlib.util.spec_from_file_location('reviewed',ROOT/'tests/gnu/reviewed-original.py')
runner=importlib.util.module_from_spec(spec);spec.loader.exec_module(runner)
target=ROOT/'evidence/gawk-source-validation.json';assert not target.exists()
paths=['gawk-source-memory-audit','gawk-source-descriptor-contract','gawk-source-smoke','gawk-source-full-dispatch']
reports={n:json.loads((ROOT/'evidence'/(n+'.json')).read_text()) for n in paths}
candidate=ROOT/'target/gawk-source-ownership-candidate/release/rboxc';digest=fingerprint(candidate)
assert all(r['binary_sha256']==digest for r in reports.values())
assert reports['gawk-source-memory-audit']['passed']==reports['gawk-source-memory-audit']['total']==190
for name,driver,total in [('gawk-source-descriptor-contract','gawk-source-ownership.py',7),('gawk-source-smoke','coreutils-smoke.py',428),('gawk-source-full-dispatch','dispatcher.py',11)]:
    data=reports[name]
    assert data['passed']==data['total']==total and all(r['pass'] for r in data['results'])
    if 'driver_sha256' in data:assert data['driver_sha256']==fingerprint(ROOT/'tests'/driver)
    else:assert (ROOT/'tests'/driver).read_bytes()==subprocess.check_output(['git','show','HEAD:tests/'+driver],cwd=ROOT)
processes=[];inspections=[]
for row in reports['gawk-source-descriptor-contract']['results']:
    assert row['equivalent']
    reference=row['outcomes']['gnu']
    for key,o in row['outcomes'].items():
        assert all(o[k]==reference[k] for k in ('status','stdout','stderr'))
        for path,h in o['raw'].items():assert fingerprint(ROOT/path)==h
        if 'memory' not in o:continue
        contents=(ROOT/o['memory_log']).read_text();pids=set(re.findall(r'^==([0-9]+)==',contents,re.M));assert len(pids)==1
        parsed=runner.parse_memory_log(contents,pids.pop(),exec_only=True);assert parsed==o['memory'] and parsed['complete_exec_log']
        if key.startswith('rboxc'):
            assert parsed['non_inherited_descriptors']==0
            assert not any(parsed['heap_bytes'].get(k,0) for k in ('definitely lost','indirectly lost','possibly lost'))
            record={'case':row['name'],'log':o['memory_log'],'sha256':fingerprint(ROOT/o['memory_log']),**parsed}
            if key.endswith('descriptors'):
                assert parsed['errors']==4 and o['preserved_inherited'] and o['preserved_stdin']
                fds=re.findall(r'Open file descriptor (\d+):[^\n]*\n==[0-9]+==\s+<inherited from parent>',contents)
                assert sorted(map(int,fds))==sorted([0,1,2,o['inherited_fd']])
                inspections.append(record)
            else:
                assert parsed['errors']==0 and o['memory_clean'];processes.append(record)
assert len(processes)==len(inspections)==7
for row in reports['gawk-source-full-dispatch']['results']:
    if 'log' not in row:continue
    contents=(ROOT/row['log']).read_text();pids=set(re.findall(r'^==([0-9]+)==',contents,re.M));assert len(pids)==1
    parsed=runner.parse_memory_log(contents,pids.pop(),exec_only=True)
    assert parsed['complete_exec_log'] and parsed['errors']==parsed['non_inherited_descriptors']==0
inputs={}
for name in ('gawk-native-cleanup','gawk-source-cleanup'):
    path=ROOT/'evidence'/(name+'.json');data=json.loads(path.read_text());inputs[str(path)]=fingerprint(path)
    assert data['driver_sha256']==fingerprint(ROOT/'scripts/gawk_cleanup.py')
    args=data['compiler_arguments'];obj=Path(args[args.index('-o')+1]);adapted=obj.with_suffix('.c')
    assert fingerprint(obj)==data['object_sha256'] and fingerprint(adapted)==data['adapted_source_sha256']
    assert fingerprint(ROOT/data['log'])==data['log_sha256']
    original=Path('/opt/src/gawk-5.4.1')/adapted.name
    assert fingerprint(original)==data['original_sha256']
    inputs.update({str(p):fingerprint(p) for p in (obj,adapted,original)})
link=json.loads((ROOT/'evidence/gawk-link.json').read_text())
for key in ('original_inputs','helper_inputs'):
    for p,h in link[key].items():assert fingerprint(ROOT/p)==h;inputs[str(ROOT/p)]=h
translation=json.loads((ROOT/'evidence/gawk-translation.json').read_text())
assert fingerprint(ROOT/translation['rust_file'])==translation['rust_sha256']==link['rust_source_sha256']
repro=ROOT/'target/gawk-source-ownership-repro/release/rboxc';assert fingerprint(repro)==digest
helper=candidate.parent/'libstdbuf.so';assert fingerprint(helper)==fingerprint(repro.parent/'libstdbuf.so')==fingerprint(ROOT/'target/release/libstdbuf.so')
assert fingerprint(ROOT/'target/release/rboxc')=='8979835878e7dbe575440b5d0c7c86a01c75782ff5ddc735707ccada50e88e86'
names=subprocess.check_output([str(candidate),'--list'],text=True).splitlines();assert len(names)==len(set(names))==187
assert len(subprocess.check_output([str(ROOT/'target/release/rboxc'),'--list'],text=True).splitlines())==133
retained={}
for name in ('gawk-format-original','gawk-source-contract','gawk-source-dispatch'):
    p=ROOT/'evidence'/(name+'.json');retained[str(p.relative_to(ROOT))]=fingerprint(p)
report={'scope':'Parser source ownership candidate: all 105 original scripts and 85 focused comparisons pass with 190 clean process logs. Seven additional contracts pass with seven strict zero-error process logs and seven separate descriptor inspections; track-fds=all deliberately counts the four preserved inherited descriptors. The initial three source-FD failures, initial descriptor-inspection interpretation and installed-provider dispatcher mismatch remain preserved. Full Gawk acceptance remains open.',
    'binary':str(candidate),'binary_sha256':digest,'binary_bytes':candidate.stat().st_size,'command_count':187,'installed_command_count':133,'installed_unchanged':True,
    'rebuild':str(repro),'rebuild_sha256':fingerprint(repro),'runtime_helper_sha256':fingerprint(helper),
    'reports':{str(ROOT/'evidence'/(n+'.json')):fingerprint(ROOT/'evidence'/(n+'.json')) for n in paths},
    'inputs':inputs,'driver_sha256':fingerprint(Path(__file__)),'retained_baselines':retained,'clean_processes':197,'descriptor_inspections':inspections,'additional_clean_processes':processes,'smoke_passed':428,'dispatcher_passed':11,'full_acceptance_complete':False}
target.write_text(json.dumps(report,indent=2)+'\n')
print('Audited 197 clean Gawk processes, seven inherited-descriptor inspections, 428 smoke checks, 11 dispatcher checks and identical rebuild')
