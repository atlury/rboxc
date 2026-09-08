#!/usr/bin/env python3
"""Audit original Sed batches without hiding external child findings."""
# SPDX-License-Identifier: GPL-3.0-or-later
import importlib.util
import json
from pathlib import Path
import re
import sys

ROOT=Path(__file__).resolve().parents[1]
sys.path.insert(0,str(ROOT/'tests'))
sys.path.insert(0,str(ROOT/'tests/gnu'))
from comparison_profile import ComparisonProfile, fingerprint
spec=importlib.util.spec_from_file_location('reviewed',ROOT/'tests/gnu/reviewed-original.py')
runner=importlib.util.module_from_spec(spec);spec.loader.exec_module(runner)
profile=ComparisonProfile('sed-original',oracle=ROOT/'build/gnu-sed/sed/sed',selections=True)
names=profile.options.commands or ['sed-original-prerequisite-final','sed-original-large-final','sed-original-remainder-final']
manifest=json.loads((ROOT/'inventory/sed-tests.json').read_text())
expected={r['script']:r for r in manifest['scripts'] if r['reviewed']}
rows={};sources=[];audit=[]
for name in names:
    assert re.fullmatch(r'[a-z0-9-]+',name)
    path=ROOT/'evidence'/(name+'.json');report=json.loads(path.read_text())
    assert report['binary_sha256']==profile.binary_sha256 and report['gnu_binary_sha256']==profile.oracle_sha256
    if report['selected_scripts']:assert {Path(r['script']).name for r in report['results']}==set(report['selected_scripts'])
    sources.append({'path':str(path.relative_to(ROOT)),'sha256':fingerprint(path),'driver_sha256':report['driver_sha256'],'prerequisites':report['prerequisites']})
    for row in report['results']:
        assert row['script'] not in rows
        assert row['source_sha256']==expected[row['script']]['source_sha256']
        for outcome in row['outcomes'].values():
            assert fingerprint(ROOT/outcome['log'])==outcome['log_sha256']
            for log in outcome.get('memory',[]):assert fingerprint(ROOT/log['log'])==log['sha256']
        candidate=[];children=[]
        for log in row['outcomes']['rboxc-valgrind']['memory']:
            path=ROOT/log['log'];text=path.read_text()
            parsed=runner.parse_memory_log(text,path.stem,exec_only=True)
            commands=re.findall(r'^==\d+== Command: (.*)$',text,re.M)
            command=commands[-1] if commands else ''
            executable=command.split(' ',1)[0]
            # Accept only the actual Sed invocation forms established by the
            # driver. Unknown images remain visible external processes.
            is_sed=executable=='sed' or bool(re.fullmatch(r'/tmp/rboxc-sed-original-[^/]+/(?:real|sed)/sed',executable))
            clean=parsed['complete_exec_log'] and parsed['errors']==0 and parsed['non_inherited_descriptors']==0 and not any(parsed['heap_bytes'].get(k,0) for k in ('definitely lost','indirectly lost','possibly lost'))
            item={'script':row['script'],'path':str(path.relative_to(ROOT)),'sha256':log['sha256'],'command':command,'sed_process':is_sed,'pass':clean,**parsed}
            if re.fullmatch(r'/tmp/rboxc-sed-original-[^/]+/testsuite/(cat|touch|sleep|dd)',executable):
                item['dependency']=report['prerequisites']['coreutils']
            (candidate if is_sed else children).append(item);audit.append(item)
        strict=bool(candidate or children) and all(r['pass'] for r in candidate+children)
        assert row['memory_clean']==strict
        rows[row['script']]={**row,'source_report':sources[-1]['path'],
            'sed_memory_clean':bool(candidate) and all(r['pass'] for r in candidate),
            'sed_process_logs':len(candidate),'external_child_logs':len(children),
            'external_child_findings':[r for r in children if not r['pass']]}
assert set(rows)==set(expected),(set(expected)-set(rows),set(rows)-set(expected))
results=[rows[r['script']] for r in manifest['scripts'] if r['reviewed']]
report={'scope':'Reviewed original selections on one immutable binary. Strict all-process results are retained; Sed process memory and external child findings are also reported separately. Platform skips and unreviewed selections are not passes.',
        **profile.metadata(),'driver_sha256':fingerprint(Path(__file__)),'source_reports':sources,
        'passed':sum(r['pass'] for r in results),'total':len(results),
        'native_passed':sum(r['native_pass'] for r in results),'assertions_passed':sum(r['assertions_pass'] for r in results),
        'sed_assertions_and_memory_passed':sum(r['assertions_pass'] and r['sed_memory_clean'] for r in results),
        'state_counts':{s:sum(r['state']==s for r in results) for s in sorted({r['state'] for r in results})},
        'registered_original_scripts':len(manifest['scripts']),'reviewed_scripts':len(expected),
        'remaining':[r for r in manifest['scripts'] if not r['reviewed']],'results':results}
profile.report.write_text(json.dumps(report,indent=2)+'\n')
path=profile.report.with_name(profile.report.stem+'-memory-audit.json')
path.write_text(json.dumps({'scope':'Reparsed final-exec logs, retaining strict checks for every process and attributing known native dependencies explicitly.',
    'binary_sha256':profile.binary_sha256,'original_report':str(profile.report.relative_to(ROOT)),'original_report_sha256':fingerprint(profile.report),
    'passed':sum(r['pass'] for r in audit),'total':len(audit),'sed_processes':sum(r['sed_process'] for r in audit),
    'sed_processes_passed':sum(r['sed_process'] and r['pass'] for r in audit),'results':audit},indent=2)+'\n')
print('Merged',len(results),'selections:',report['passed'],'strict passes;',report['sed_assertions_and_memory_passed'],'Sed assertions/memory passes')
