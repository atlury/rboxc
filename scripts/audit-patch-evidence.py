#!/usr/bin/env python3
"""Verify current Patch comparisons and reparse every candidate process log."""
# SPDX-License-Identifier: GPL-3.0-or-later
import argparse
import importlib.util
import json
from pathlib import Path
import re
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT/'tests'))
sys.path.insert(0, str(ROOT/'tests/gnu'))
from comparison_profile import fingerprint
spec = importlib.util.spec_from_file_location('reviewed', ROOT/'tests/gnu/reviewed-original.py')
runner = importlib.util.module_from_spec(spec); spec.loader.exec_module(runner)
parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('--focused', type=Path, required=True)
parser.add_argument('--original', type=Path, required=True)
parser.add_argument('--report-name', required=True)
for name in ('smoke','dispatch','rebuild'):
    parser.add_argument('--'+name, type=Path)
options = parser.parse_args()
assert re.fullmatch(r'[a-z0-9-]+', options.report_name)
target = ROOT/'evidence'/(options.report_name+'.json')
assert not target.exists(), 'preserve prior audits'
focused = json.loads(options.focused.read_text())
original = json.loads(options.original.read_text())
assert focused['binary_sha256'] == original['binary_sha256'] == fingerprint(Path(focused['binary']))
assert focused['complete'] and focused['passed'] == focused['total'] == focused['planned_total'] == 23
manifest=json.loads((ROOT/'inventory/patch-tests.json').read_text())
reviewed={r['target'] for r in manifest['inputs'] if r['reviewed']}
reviewed_rows={r['target']:r for r in manifest['inputs'] if r['reviewed']}
assert original['complete'] and original['passed'] == original['total'] == original['planned_total'] == len(reviewed)
assert focused['driver_sha256'] == fingerprint(ROOT/'tests/entry-behavior.py')
assert original['inputs'][str(ROOT/'tests/patch-original.py')] == fingerprint(ROOT/'tests/patch-original.py')
assert {r['selection'] for r in original['results']} == reviewed
inputs = {}
for data in (focused, original):
    inputs.update(data.get('inputs', {})); inputs.update(data['runtime_helpers'])
for item in focused['oracles'].values():
    inputs[item['path']] = item['sha256']
for path, expected in inputs.items():
    assert fingerprint(Path(path)) == expected, 'recorded input changed: '+path
processes = []
def audit(log, expected_hash, recorded, candidate, case):
    path = ROOT/log
    assert fingerprint(path) == expected_hash, 'recorded log changed: '+log
    contents = path.read_text()
    pids = set(re.findall(r'^==([0-9]+)==', contents, re.M)); assert len(pids) == 1
    parsed = runner.parse_memory_log(contents, pids.pop(), exec_only=True)
    assert all(recorded[k] == v for k,v in parsed.items())
    assert parsed['complete_exec_log']
    if candidate:
        assert parsed['errors'] == 0 and parsed['non_inherited_descriptors'] == 0
        assert not any(parsed['heap_bytes'].get(k,0) for k in ('definitely lost','indirectly lost','possibly lost'))
        processes.append({'case':case,'log':log,'sha256':expected_hash,**parsed,'pass':True})
focused_rows=[r for r in focused['results'] if r['command']=='patch']
assert len(focused_rows)==23
for row in focused_rows:
    assert row['pass'] and row['equivalent'] and row['memory_clean']
    reference = row['outcomes']['gnu']
    for key, outcome in row['outcomes'].items():
        assert all(outcome[k] == reference[k] for k in ('status','stdout','stderr','tree'))
        if 'memory' in outcome:
            audit(outcome['log'],outcome['log_sha256'],outcome['memory'],key=='rboxc-valgrind',row['name'])
expected_failures={'context-format','dash-o-append'}
assert set(original['expected_failure_selections'])==expected_failures
assert original['ordinary_passed']==len(reviewed)-len(expected_failures)
assert original['expected_failures_matched']==len(expected_failures)
for row in original['results']:
    assert row['pass'] and row['expected_failure']==(row['selection'] in expected_failures)
    selection=reviewed_rows[row['selection']]
    assert row['unprivileged']==selection.get('unprivileged',False)
    assert row['native_editor']==selection.get('native_editor',False)
    reference=row['outcomes']['gnu']
    for key,outcome in row['outcomes'].items():
        assert outcome['identity']==({'user':65534,'group':65534,'extra_groups':[]} if row['unprivileged'] else {})
        for path,copy in outcome['copied_inputs'].items():
            assert fingerprint(Path(path))==copy['sha256']
        if row['unprivileged']:
            expected_binary=original['binary'] if key.startswith('rboxc') else str(ROOT/'build/gnu-patch/src/patch')
            assert expected_binary in outcome['copied_inputs']
        assert outcome['expectation_matches']
        if row['expected_failure']:
            assert outcome['status']==reference['status']==1 and not outcome['assertions_pass']
            assert outcome['assertion_counts']==reference['assertion_counts']
            assert (ROOT/outcome['driver_log']).read_bytes()==(ROOT/reference['driver_log']).read_bytes()
        else:
            assert outcome['status']==0 and outcome['assertions_pass']
        assert len(outcome['assertion_counts'])==1
        total,passed,failed=outcome['assertion_counts'][0]
        assert total>0 and total==passed+failed
        assert failed>0 if row['expected_failure'] else failed==0
        counts=re.findall(rb'(\d+) tests \((\d+) passed, (\d+) failed\)',(ROOT/outcome['driver_log']).read_bytes())
        assert [[int(n) for n in c] for c in counts]==outcome['assertion_counts']
        assert fingerprint(ROOT/outcome['driver_log'])==outcome['driver_log_sha256']
        for log in outcome['memory']:
            contents=(ROOT/log['log']).read_text()
            commands=re.findall(r'^==[0-9]+== Command: (.*)$',contents,re.M)
            assert len(commands)==1, 'unclassified process image'
            name=commands[0].split()[0]
            role='patch' if Path(name).name=='patch' else 'native-test-helper'
            assert role==log['role']
            if role!='patch':
                if row['launch_profile']=='instrument-original-driver':
                    assert Path(name).name in {'sh','cat','diff','mkdir','expr','chmod','rm','touch','sed','ed'}
                else:
                    assert row['native_editor'] and name in {'/bin/sh','/usr/bin/ed'}
            audit(log['log'],log['sha256'],log,key=='rboxc-valgrind' and role=='patch',row['source'])
original_processes=sum(sum(m['role']=='patch' for m in r['outcomes']['rboxc-valgrind']['memory']) for r in original['results'])
assert original_processes>0 and len(processes)==23+original_processes
report = {'scope':'All 23 focused comparisons and all reviewed unchanged GNU original selections match their registered expectations. The two upstream XFAIL selections retain identical native/candidate failing assertions and output; they are not ordinary passes. Every input and raw log is hash-checked and every candidate process summary is reparsed. GNU native findings remain baseline observations.',
          'binary':focused['binary'],'binary_sha256':focused['binary_sha256'],
          'runtime_helpers':focused['runtime_helpers'],'inputs':inputs,
          'focused_report':{'path':str(options.focused),'sha256':fingerprint(options.focused)},
          'original_report':{'path':str(options.original),'sha256':fingerprint(options.original)},
          'passed':len(processes),'total':len(processes),'focused_cases':23,'original_selections':len(reviewed),'ordinary_passed':original['ordinary_passed'],'expected_failures_matched':original['expected_failures_matched'],'original_processes':original_processes,
          'driver_sha256':fingerprint(Path(__file__)),'results':processes}
if any(getattr(options,n) is not None for n in ('smoke','dispatch','rebuild')):
    assert all(getattr(options,n) is not None for n in ('smoke','dispatch','rebuild'))
    assert fingerprint(options.rebuild)==original['binary_sha256']
    binary=Path(original['binary'])
    commands=subprocess.check_output([str(binary),'--list'],text=True).splitlines()
    assert len(commands)==len(set(commands))==187
    for name,total in (('smoke',428),('dispatch',11)):
        path=getattr(options,name);data=json.loads(path.read_text())
        assert data['binary_sha256']==original['binary_sha256']
        assert data['passed']==data['total']==total
        report[name+'_report']={'path':str(path),'sha256':fingerprint(path),'passed':total,'total':total}
    installed=ROOT/'target/release/rboxc'
    assert fingerprint(installed)=='8979835878e7dbe575440b5d0c7c86a01c75782ff5ddc735707ccada50e88e86'
    translation_path=ROOT/'evidence/patch-translation.json'
    translation=json.loads(translation_path.read_text())
    assert fingerprint(ROOT/translation['rust_file'])==translation['rust_sha256']
    assert fingerprint(ROOT/translation['log'])==translation['log_sha256']
    native_path=ROOT/'evidence/patch-native-cleanup.json'
    native=json.loads(native_path.read_text())
    assert native['driver_sha256']==fingerprint(ROOT/'scripts/patch_cleanup.py')
    donor=native['merge_threshold_reference']
    assert fingerprint(Path(donor['path']))==donor['sha256']
    pin=json.loads((ROOT/'inventory/sources.json').read_text())['patch']
    for item in native['adaptations']:
        args=item['compiler_arguments'];obj=Path(args[args.index('-o')+1])
        adapted=obj.with_suffix('.c');original_source=Path(pin['source'])/'src'/adapted.name
        assert fingerprint(obj)==item['object_sha256']
        assert fingerprint(adapted)==item['adapted_source_sha256']
        assert fingerprint(original_source)==item['original_sha256']==pin['source_and_header_sha256']['src/'+adapted.name]
        assert fingerprint(ROOT/item['log'])==item['log_sha256']
    link_path=ROOT/'evidence/patch-link.json';link=json.loads(link_path.read_text())
    assert link['rust_source_sha256']==translation['rust_sha256']
    for path,expected in link['helper_inputs'].items():
        assert fingerprint(ROOT/path)==expected
    report.update(candidate_bytes=binary.stat().st_size,candidate_commands=187,
        rebuild=str(options.rebuild),rebuild_sha256=fingerprint(options.rebuild),
        installed_binary_sha256=fingerprint(installed),full_acceptance_complete=False,
        build_evidence={str(p.relative_to(ROOT)):fingerprint(p) for p in (translation_path,native_path,link_path)},
        held_out=[r['target'] for r in manifest['inputs'] if r['state']=='held-out'])
target.write_text(json.dumps(report,indent=2)+'\n')
print('Audited 23 focused cases,',len(reviewed),'GNU original selections, and',len(processes),'clean candidate processes')
