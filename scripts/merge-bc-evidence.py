#!/usr/bin/env python3
"""Combine completed original arithmetic profiles for an immutable candidate."""
# SPDX-License-Identifier: GPL-3.0-or-later
import importlib.util
import json
from pathlib import Path
import sys

ROOT=Path(__file__).resolve().parents[1]
sys.path.insert(0,str(ROOT/'tests'))
sys.path.insert(0,str(ROOT/'tests/gnu'))
from comparison_profile import ComparisonProfile, fingerprint
spec=importlib.util.spec_from_file_location('reviewed',ROOT/'tests/gnu/reviewed-original.py')
runner=importlib.util.module_from_spec(spec);spec.loader.exec_module(runner)
profile=ComparisonProfile('bc-original',oracle=ROOT/'build/gnu-bc/bc/bc')
manifest=json.loads((ROOT/'inventory/bc-tests.json').read_text())
expected={r['path']:r for r in manifest['inputs'] if r['state']=='reviewed'}
results={};sources=[];audit=[]
for name in ('bc-original-initial','bc-original-signum-final'):
    path=ROOT/'evidence'/(name+'.json');report=json.loads(path.read_text())
    assert report['binary_sha256']==profile.binary_sha256 and report['gnu_binary_sha256']==profile.oracle_sha256
    sources.append({'path':str(path.relative_to(ROOT)),'sha256':fingerprint(path),'driver_sha256':report['driver_sha256']})
    for row in report['results']:
        if name=='bc-original-initial' and row['path']=='Test/signum':continue
        assert row['path'] not in results
        assert all(row[k]==v for k,v in expected[row['path']].items()), 'original invocation changed'
        assert row['pass'] and row['equivalent'] and row['expected_output_pass'] and row['memory_clean']
        for outcome in row['outcomes'].values():
            for log in outcome.get('memory',[]):assert fingerprint(ROOT/log['log'])==log['sha256']
        for log in row['outcomes']['rboxc-valgrind']['memory']:
            path=ROOT/log['log'];parsed=runner.parse_memory_log(path.read_text(),path.stem,exec_only=True)
            clean=parsed['complete_exec_log'] and parsed['errors']==0 and parsed['non_inherited_descriptors']==0 and not any(parsed['heap_bytes'].get(k,0) for k in ('definitely lost','indirectly lost','possibly lost'))
            assert clean
            audit.append({'input':row['path'],'path':str(path.relative_to(ROOT)),'sha256':log['sha256'],'pass':clean,**parsed})
        results[row['path']]={**row,'source_report':sources[-1]['path']}
assert set(results)==set(expected)
report={'scope':'Nineteen reviewed full original arithmetic inputs, including a declared invocation for the definition-only testfn file. Signum uses its default integer scale; the earlier math-library profile and its failed mathematical assertions are retained. The timing wrapper is not an additional runtime test.',
        **profile.metadata(),'driver_sha256':fingerprint(Path(__file__)),'source_reports':sources,
        'passed':len(results),'total':len(results),'registered_runtime_tests':0,
        'results':[results[n] for n in expected],
        'other_inputs':[r for r in manifest['inputs'] if r['state']!='reviewed']}
profile.report.write_text(json.dumps(report,indent=2)+'\n')
path=profile.report.with_name(profile.report.stem+'-memory-audit.json')
path.write_text(json.dumps({'scope':'Strict final-exec candidate memory and descriptor audit for the reviewed original calculator inputs.',
    'binary_sha256':profile.binary_sha256,'original_report':str(profile.report.relative_to(ROOT)),'original_report_sha256':fingerprint(profile.report),
    'passed':len(audit),'total':len(audit),'results':audit},indent=2)+'\n')
print('Merged',len(results),'original inputs;',len(audit),'complete clean candidate logs')
