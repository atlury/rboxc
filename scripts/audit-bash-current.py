#!/usr/bin/env python3
"""Account for every Bash recipe without treating open profiles as passes."""
# SPDX-License-Identifier: GPL-3.0-or-later
import argparse
from collections import Counter
import hashlib
import json
from pathlib import Path

ROOT=Path(__file__).resolve().parents[1]
parser=argparse.ArgumentParser(description=__doc__)
parser.add_argument('--report-name',required=True)
args=parser.parse_args()
assert args.report_name.replace('-','').isalnum() and args.report_name.isascii()
target=ROOT/'evidence'/(args.report_name+'.json');assert not target.exists()
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
manifest=ROOT/'inventory/bash-tests.json';inventory=json.loads(manifest.read_text())
source=Path('/opt/src/bash-5.3/tests')
assert {r['recipe'] for r in inventory['inputs']}=={p.name for p in source.glob('run-*') if p.is_file()}
reports={};audits={};raw={};results=[];processes={};open_processes={}
for row in inventory['inputs']:
    assert sha(source/row['recipe'])==row['recipe_sha256']
    if not row['reviewed']:continue
    assert row['state'] in ('reviewed-original-pass','reviewed-original-assertions-memory-open','reviewed-original-baseline-open')
    report_path=ROOT/row['evidence'];audit_path=ROOT/row['audit']
    if str(report_path) not in reports:
        report=json.loads(report_path.read_text());audit=json.loads(audit_path.read_text())
        assert report['complete'] and report['total']==report['planned_total']==len(report['results'])
        assert report['passed']==sum(r['pass'] for r in report['results'])
        assert sha(report_path)==audit['original_sha256']
        assert report['binary_sha256']==audit['binary_sha256']==sha(Path(report['binary']))
        assert sha(ROOT/audit['inventory_snapshot'])==audit['inventory_sha256']
        for path,digest in audit['raw'].items():
            if path not in raw:assert sha(ROOT/path)==digest;raw[path]=digest
            else:assert raw[path]==digest
        reports[str(report_path)]={**report,'report_sha256':sha(report_path)}
        audits[str(audit_path)]={**audit,'audit_sha256':sha(audit_path)}
    report=reports[str(report_path)];audit=audits[str(audit_path)]
    for case in row.get('script_cases',[{}]):
        selection=row['target']+':'+case['script'] if row.get('script_cases') else row['target']
        result,=[r for r in report['results'] if r['selection']==selection]
        assert result['pass']==(row['state']=='reviewed-original-pass')
        baseline=selection in audit.get('assertion_baselines',[])
        assert baseline==(row['state']=='reviewed-original-baseline-open')
        assert all((baseline or o['expected_output_matches']) and not o.get('timed_out',False) for o in result['outcomes'].values())
        collection=processes if result['pass'] else open_processes
        entries=audit['processes'] if result['pass'] else audit['open_processes']
        selected=[r for r in entries if r['selection']==selection];assert selected
        for record in selected:assert record['log'] not in collection;collection[record['log']]=record
        results.append({'selection':selection,'pass':result['pass'],'assertion_baseline':baseline,'binary_sha256':report['binary_sha256'],'evidence':row['evidence'],'audit':row['audit']})
assert len({r['selection'] for r in results})==len(results)
states=Counter(row['state'] for row in inventory['inputs'])
target.write_text(json.dumps({
    'scope':'Every top-level Bash recipe is source-pinned. Reviewed script results refer to their actual immutable candidates and independently audited original assertions/raw logs. Entire memory-open process families remain outside strict counts. Recipe dispatchers are orchestration, not runtime passes. Pending and held work is not complete.',
    'inventory_sha256':sha(manifest),'recipes':len(inventory['inputs']),'recipe_states':dict(states),
    'original_scripts':len(results),'strict_original_scripts':sum(r['pass'] for r in results),
    'memory_open_scripts':[r['selection'] for r in results if not r['pass'] and not r['assertion_baseline']],
    'assertion_baseline_scripts':[r['selection'] for r in results if r['assertion_baseline']],
    'clean_candidate_processes':len(processes),'open_candidate_processes':len(open_processes),
    'candidate_script_counts':dict(Counter(r['binary_sha256'] for r in results)),
    'reports':{p:r['report_sha256'] for p,r in reports.items()},
    'audits':{p:r['audit_sha256'] for p,r in audits.items()},
    'results':results,'raw':raw,'accounting_pass':True,'full_bash_complete':False,
},indent=2)+'\n')
print('Audited',len(results),'original scripts;',sum(r['pass'] for r in results),'strict;',len(processes),'clean processes')
