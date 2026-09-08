#!/usr/bin/env python3
"""Combine completed Gzip selections for one immutable candidate and audit its logs."""
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
profile=ComparisonProfile('gzip-original',oracle=ROOT/'build/gnu-gzip/gzip',selections=True)
manifest=json.loads((ROOT/'inventory/gzip-tests.json').read_text())
# Each completed batch covers a disjoint reviewed selection.
names=profile.options.commands or ['gzip-original-initial','gzip-original-large','gzip-original-help']
rows={};sources=[];seen_logs=set();audit=[]
for name in names:
    assert re.fullmatch(r'[a-z0-9-]+',name)
    path=ROOT/f'evidence/{name}.json'
    report=json.loads(path.read_text())
    assert report['binary_sha256']==profile.binary_sha256
    assert report['gnu_binary_sha256']==profile.oracle_sha256
    if report['selected_scripts']:
        assert {Path(r['script']).name for r in report['results']}==set(report['selected_scripts']), 'batch is incomplete'
    sources.append({'path':str(path.relative_to(ROOT)),'sha256':fingerprint(path),
                    'driver_sha256':report['driver_sha256'],
                    'prerequisites':report.get('prerequisites'),
                    'locale_environment':report.get('locale_environment')})
    for row in report['results']:
        for outcome in row['outcomes'].values():
            assert fingerprint(ROOT/outcome['log'])==outcome['log_sha256']
            for log in outcome.get('memory',[]):
                assert fingerprint(ROOT/log['log'])==log['sha256']
        assert row['script'] not in rows, 'duplicate selection'
        rows[row['script']]={**row,'source_report':str(path.relative_to(ROOT))}
expected={r['script']:r for r in manifest['scripts'] if r['reviewed']}
assert set(rows)==set(expected), (set(expected)-set(rows),set(rows)-set(expected))
for script,row in rows.items():
    assert row['source_sha256']==expected[script]['source_sha256']
    assert row.get('built_programs')==expected[script].get('built_programs')
    clean=True
    for log in row['outcomes']['rboxc-valgrind']['memory']:
        path=ROOT/log['log'];pid=path.stem
        parsed=runner.parse_memory_log(path.read_text(),pid,exec_only=True)
        passed=(parsed['complete_exec_log'] and parsed['errors']==0 and parsed['non_inherited_descriptors']==0
                and not any(parsed['heap_bytes'].get(k,0) for k in ('definitely lost','indirectly lost','possibly lost')))
        clean=clean and passed
        if str(path) not in seen_logs:
            seen_logs.add(str(path));audit.append({'script':script,'path':str(path.relative_to(ROOT)),
                                                 'sha256':log['sha256'],'pass':passed,**parsed})
    assert row['outcomes']['rboxc-valgrind']['memory']
    assert row['memory_clean']==clean, 'strict audit differs from source assessment'
    assert row['pass'] and clean, 'unresolved selection requires a separate assessment'
results=[rows[r['script']] for r in manifest['scripts'] if r['reviewed']]
report={'scope':'Completed individually reviewed originals on one binary. Sixteen full scripts and one declared three-program help/version selection; auxiliary ports and remaining originals stay open.',
        **profile.metadata(),'gnu_binaries':report['gnu_binaries'],
        'driver_sha256':fingerprint(Path(__file__)),'source_reports':sources,
        'passed':len(results),'native_passed':len(results),'assertions_passed':len(results),
        'total':len(results),'full_original_scripts_passed':sum(not r.get('built_programs') for r in results),
        'partial_original_scripts':sum(bool(r.get('built_programs')) for r in results),
        'selected_perl_cases':sum(len(r.get('cases',[])) for r in results),
        'registered_original_scripts':len(manifest['scripts']),
        'reviewed_scripts':len(expected),'state_counts':{'passed':len(results)},
        'expected_failures':0,'prerequisite_skips':0,
        'remaining':[r for r in manifest['scripts'] if not r['reviewed']],
        'results':results}
profile.report.write_text(json.dumps(report,indent=2)+'\n')
audit_path=profile.report.with_name(profile.report.stem+'-memory-audit.json')
audit_path.write_text(json.dumps({'scope':'Strict retained candidate process-log audit for the selected original results. Every final exec image requires descriptor and error summaries; no lost/possibly-lost heap or unexpected descriptors.','binary_sha256':profile.binary_sha256,'original_report':str(profile.report.relative_to(ROOT)),'original_report_sha256':fingerprint(profile.report),'passed':sum(r['pass'] for r in audit),'total':len(audit),'results':audit},indent=2)+'\n')
print('Merged',len(results),'selections;',len(audit),'complete clean candidate process logs')
