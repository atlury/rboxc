#!/usr/bin/env python3
"""Verify current Gawk comparisons and reparse every candidate process log."""
# SPDX-License-Identifier: GPL-3.0-or-later
import argparse
from collections import Counter
import importlib.util
import json
from pathlib import Path
import re
import sys

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT/'tests'))
sys.path.insert(0, str(ROOT/'tests/gnu'))
from comparison_profile import fingerprint
from gawk_child_profile import canonical_child
spec = importlib.util.spec_from_file_location('reviewed', ROOT/'tests/gnu/reviewed-original.py')
runner = importlib.util.module_from_spec(spec); spec.loader.exec_module(runner)
parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('--focused', type=Path, required=True)
parser.add_argument('--original', type=Path, required=True)
parser.add_argument('--report-name', required=True)
options = parser.parse_args()
assert re.fullmatch(r'[a-z0-9-]+', options.report_name)
target = ROOT/'evidence'/(options.report_name+'.json')
assert not target.exists(), 'preserve prior audits'
focused = json.loads(options.focused.read_text())
original = json.loads(options.original.read_text())
assert focused['binary_sha256'] == original['binary_sha256'] == fingerprint(Path(focused['binary']))
assert focused['complete'] and focused['passed'] == focused['total'] == focused['planned_total'] == 85
reviewed_rows={r['target']:r for r in json.loads((ROOT/'inventory/gawk-tests.json').read_text())['inputs'] if r['reviewed']}
requested=original.get('selected_targets',[])
if requested:
    assert len(set(requested))==len(requested) and set(requested)<=set(reviewed_rows)
    reviewed_rows={n:reviewed_rows[n] for n in requested}
reviewed=set(reviewed_rows)
baselines={n:r for n,r in reviewed_rows.items() if r.get('expected_baseline_output')}
assert reviewed
assert original['complete'] and original['total'] == original['planned_total'] == len(reviewed)
assert original['passed']==len(reviewed)-len(baselines)
assert original['passed']==sum(r['pass'] for r in original['results'])
assert original['baseline_failures_matched']==sum(r['baseline_failure_matches'] for r in original['results'])
assert original['matched']==original['passed']+original['baseline_failures_matched']
assert original['baseline_failures_matched']<=len(baselines)
assert focused['driver_sha256'] == fingerprint(ROOT/'tests/gawk-behavior.py')
assert original['inputs'][str(ROOT/'tests/gawk-original.py')] == fingerprint(ROOT/'tests/gawk-original.py')
assert {r['selection'] for r in original['results']} == reviewed
inputs = {}
for data in (focused, original):
    inputs.update(data.get('inputs', {})); inputs.update(data['runtime_helpers'])
for item in focused['oracles'].values():
    inputs[item['path']] = item['sha256']
for path, expected in inputs.items():
    assert fingerprint(Path(path)) == expected, 'recorded input changed: '+path
baseline_evidence={}
for recipe in baselines.values():
    previous_path=ROOT/recipe['baseline_report'];previous=json.loads(previous_path.read_text())
    assert previous['gnu_binary_sha256']==original['gnu_binary_sha256']
    selection=next(r for r in previous['results'] if r['selection']==recipe['target'])
    assert selection['source_sha256']==recipe['sha256']
    assert not selection['pass']
    for outcome in selection['outcomes'].values():
        assert not outcome['assertions_pass'] and outcome['driver_log_sha256']==recipe['expected_baseline_driver_sha256']
        assert fingerprint(ROOT/outcome['driver_log'])==outcome['driver_log_sha256']
        actual=(ROOT/outcome['driver_log']).parent/'actual-output'
        assert actual.read_bytes()==recipe['expected_baseline_output'].encode()
        baseline_evidence[str(actual.relative_to(ROOT))]=fingerprint(actual)
    archive=ROOT/recipe['baseline_driver_archive']
    assert fingerprint(archive)==previous['driver_sha256']
    baseline_evidence[str(archive.relative_to(ROOT))]=fingerprint(archive)
    baseline_evidence[recipe['baseline_report']]=fingerprint(previous_path)
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
for row in focused['results']:
    assert row['pass'] and row['equivalent'] and row['memory_clean']
    reference = row['outcomes']['gnu']
    for key, outcome in row['outcomes'].items():
        assert all(outcome[k] == reference[k] for k in ('status','stdout','stderr','tree'))
        if 'memory' in outcome:
            audit(outcome['log'],outcome['log_sha256'],outcome['memory'],key=='rboxc-valgrind',row['name'])
for row in original['results']:
    baseline=baselines.get(row['selection'])
    assert row['pass']==(baseline is None)
    if baseline:
        # A discovery run records an ordinary assertion failure before its
        # identical GNU baseline is reviewed. Audit its original bytes here;
        # do not rewrite the report or rerun unchanged programs just to relabel it.
        assert row['baseline_failure_matches'] or (ROOT/baseline['baseline_report']).resolve()==options.original.resolve()
    else:
        assert not row['baseline_failure_matches']
    assert row['locale_profile']==reviewed_rows[row['selection']].get('locale_profile')
    self_exec_images=reviewed_rows[row['selection']].get('self_exec_images')
    assert row.get('self_exec_images')==self_exec_images
    children=reviewed_rows[row['selection']].get('child_commands',{})
    dependencies=reviewed_rows[row['selection']].get('child_dependencies',{})
    assert row.get('child_dependencies',{}) == dependencies
    for entry in dependencies.values():
        assert original['inputs'][entry['path']] == entry['sha256']
    assert row.get('child_commands',{})==children
    working_files=reviewed_rows[row['selection']].get('working_files',{})
    assert row.get('working_files',{})==working_files
    for filename,entry in working_files.items():
        assert re.fullmatch(r'[A-Za-z0-9_][A-Za-z0-9_.-]*',filename)
        assert original['inputs'][entry['path']]==entry['sha256']
    extension=reviewed_rows[row['selection']].get('extension_profile')
    assert row.get('extension_profile')==extension
    if extension:
        for path,h in extension['inputs'].items():assert original['inputs'][path]==h
        for library,entry in extension['libraries'].items():
            assert Path(entry['path']).name==library
            assert original['inputs'][entry['path']]==entry['sha256']
    if row['locale_profile']:
        locale=json.loads((ROOT/row['locale_profile']).read_text())
        for path,h in locale['inputs'].items():assert fingerprint(Path(path))==h
        assert locale['driver_sha256']==fingerprint(ROOT/locale.get('preparation_driver','scripts/prepare-gawk-locales.py'))
        assert fingerprint(ROOT/locale['build_log'])==locale['build_log_sha256']
        base=Path(locale['runtime_path'])
        assert (base/locale['alias']).readlink()==Path(locale['name'])
        for name,h in locale['files'].items():assert fingerprint(base/locale['name']/name)==h
        assert locale['probe']=={'status':0,'stdout':'UTF-8\n','stderr':''}
    for key,outcome in row['outcomes'].items():
        assert not outcome.get('timed_out') and not outcome.get('child_wait_timeout')
        assert outcome['status']==0
        if baseline:
            assert not outcome['assertions_pass']
            assert outcome['baseline_failure_matches']==row['baseline_failure_matches']
            output=ROOT/outcome['actual_output']
            assert fingerprint(output)==outcome['actual_output_sha256']
            assert output.read_bytes()==baseline['expected_baseline_output'].encode()
            assert outcome['driver_log_sha256']==baseline['expected_baseline_driver_sha256']
        else:
            assert outcome['assertions_pass'] and not outcome['baseline_failure_matches']
            assert outcome['actual_output'] is None and outcome['actual_output_sha256'] is None
        assert fingerprint(ROOT/outcome['driver_log'])==outcome['driver_log_sha256']
        observed_children=Counter()
        gawk_processes=0
        for log in outcome['memory']:
            contents=(ROOT/log['log']).read_text()
            commands=re.findall(r'^==[0-9]+== Command: (.*)$',contents,re.M)
            assert len(commands)==(self_exec_images or 1)
            if self_exec_images:
                assert self_exec_images==2 and commands[0].startswith('gawk ')
                directory=outcome['private_work_directory']
                assert re.fullmatch(r'/tmp/rboxc-gawk-original-[A-Za-z0-9_-]+(?:/test)?',directory)
                assert commands[1]==directory+'/exec/gawk'+commands[0][4:]
            assert log.get('commands',commands)==commands
            command=commands[0]
            if command.split()[0]=='gawk':
                gawk_processes+=1
                assert log.get('role','gawk')=='gawk'
            else:
                canonical=canonical_child(command,children,outcome.get('private_work_directory'),dependencies)
                assert log.get('canonical_command',canonical)==canonical
                assert log['role']=='child-dependency'
                observed_children[canonical]+=1
            assert log.get('command',command)==command
            audit(log['log'],log['sha256'],log,key=='rboxc-valgrind',row['source'])
        if key.endswith('-valgrind'):
            assert gawk_processes>0 and observed_children==children
original_processes=sum(len(r['outcomes']['rboxc-valgrind']['memory']) for r in original['results'])
assert original_processes>0 and len(processes)==85+original_processes
report = {'scope':'All 85 focused comparisons pass. Original assertion passes and exact failures shared with the pinned native GNU baseline are counted separately; baseline matches do not count as passing original tests. Every input and raw log is hash-checked and every candidate process summary is reparsed. GNU native findings remain baseline observations.',
          'binary':focused['binary'],'binary_sha256':focused['binary_sha256'],
          'runtime_helpers':focused['runtime_helpers'],'inputs':inputs,
          'focused_report':{'path':str(options.focused),'sha256':fingerprint(options.focused)},
          'original_report':{'path':str(options.original),'sha256':fingerprint(options.original)},
          'passed':len(processes),'total':len(processes),'focused_cases':85,'original_selections':len(reviewed),'original_processes':original_processes,
          'original_assertion_passes':original['passed'],'baseline_failures_matched':len(baselines),'baseline_selections':sorted(baselines),'baseline_evidence':baseline_evidence,'driver_sha256':fingerprint(Path(__file__)),'results':processes}
report['selected_targets']=requested
report['extension_selections']=[n for n,r in reviewed_rows.items() if r.get('extension_profile')]
report['child_dependency_processes']=sum(sum(r.get('child_commands',{}).values()) for r in reviewed_rows.values())
report['original_gawk_processes']=original_processes-report['child_dependency_processes']
report['original_gawk_execution_images']=sum(m['exec_images'] for r in original['results'] for m in r['outcomes']['rboxc-valgrind']['memory'] if m['role']=='gawk')
target.write_text(json.dumps(report,indent=2)+'\n')
print('Audited 85 focused cases,',len(reviewed),'GNU original selections, and',len(processes),'clean candidate processes')
