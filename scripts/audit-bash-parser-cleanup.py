#!/usr/bin/env python3
"""Audit parser/expansion ownership repairs and the reviewed Bash regression."""
# SPDX-License-Identifier: GPL-3.0-or-later
import hashlib
import json
from pathlib import Path
import subprocess

ROOT=Path(__file__).resolve().parents[1]
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
target=ROOT/'evidence/bash-parser-cleanup-validation.json';assert not target.exists()
binary=ROOT/'target/bash-parser-cleanup-candidate/release/rboxc';digest=sha(binary)
assert sha(ROOT/'target/bash-parser-cleanup-repro/release/rboxc')==digest
assert len(subprocess.check_output([binary,'--list']).splitlines())==187
assert sha(ROOT/'target/release/rboxc')=='8979835878e7dbe575440b5d0c7c86a01c75782ff5ddc735707ccada50e88e86'
profile_path=ROOT/'evidence/bash-parser-cleanup.json';profile=json.loads(profile_path.read_text())
assert sha(ROOT/'scripts/bash_parser_cleanup.py')==profile['driver_sha256']
assert sha(ROOT/'evidence/bash-native-cleanup.json')==profile['baseline_profile_sha256']
raw={};reports={}
def pin(path,expected):
    p=Path(path)
    if not p.is_absolute():p=ROOT/p
    assert sha(p)==expected,str(p)
    raw[str(p.relative_to(ROOT)) if p.is_relative_to(ROOT) else str(p)]=expected
for record in profile['files']:
    for field in ('original','source','object','build_log'):pin(record[field],record[field+'_sha256'])
changes=json.loads((ROOT/'evidence/bash-parser-cleanup-helper-changes.json').read_text())
assert changes['current'].keys()==changes['previous'].keys()
assert changes['changed']==[p for p in changes['current'] if changes['current'][p]!=changes['previous'][p]]==['build/helpers/bash-001-y.tab.o','build/helpers/bash-013-subst.o']
for path,expected in changes['current'].items():pin(path,expected)
selections={};clean=[];open_processes=[];open_findings=[]
for i in range(4):
    report_path=ROOT/f'evidence/bash-parser-cleanup-batch-{i}.json'
    audit_path=ROOT/f'evidence/bash-parser-cleanup-batch-{i}-validation.json'
    report=json.loads(report_path.read_text());audit=json.loads(audit_path.read_text())
    assert report['complete'] and report['total']==len(report['results'])==report['planned_total']
    assert report['binary_sha256']==audit['binary_sha256']==digest
    assert audit['original_sha256']==sha(report_path)
    for result in report['results']:
        assert result['selection'] not in selections
        selections[result['selection']]=result['pass']
        assert all(o['expected_output_matches'] and not o['timed_out'] for o in result['outcomes'].values())
    for path,expected in audit['raw'].items():pin(path,expected)
    clean.extend(audit['processes']);open_processes.extend(audit['open_processes']);open_findings.extend(audit['open_findings'])
    reports[str(report_path.relative_to(ROOT))]=sha(report_path)
    reports[str(audit_path.relative_to(ROOT))]=sha(audit_path)
assert len(selections)==41 and sum(selections.values())==38
assert {name for name,passed in selections.items() if not passed}=={'lastpipe','posixexp2','set-e'}
assert selections['comsub-eof'] and selections['parser']
assert len({r['log'] for r in clean})==len(clean)
assert len({r['log'] for r in open_processes})==len(open_processes)
for name,total in [('behavior',44),('adapters',69),('smoke',428),('dispatch',11)]:
    path=ROOT/f'evidence/bash-parser-cleanup-{name}.json';r=json.loads(path.read_text())
    assert r.get('complete',True) and r['binary_sha256']==digest
    assert r['passed']==r['total']==total==len(r['results']) and all(x['pass'] for x in r['results'])
    for result in r['results']:
        for key,outcome in result.get('outcomes',{}).items():
            for memory in outcome['memory']:
                pin(memory['log'],memory['sha256'])
                if key=='rboxc-valgrind':assert memory['complete'] and memory['clean']
        if result.get('log'):pin(result['log'],sha(ROOT/result['log']))
    reports[str(path.relative_to(ROOT))]=sha(path)
target.write_text(json.dumps({
    'scope':'Two isolated Bash helpers changed. Lexer words are destroyed when Bison discards their owning tokens, failed compound-assignment parsing flushes only its abandoned saved state, and a heap WORD_DESC owns string expansion across nonlocal exits. Both affected original scripts now pass strictly. All 41 reviewed scripts match GNU assertions; 38 have clean candidate process families. The three earlier pipeline/subshell profiles remain open and outside strict counts. Full GNU acceptance is not complete.',
    'binary':str(binary),'binary_sha256':digest,'size_bytes':binary.stat().st_size,
    'byte_identical_rebuild':True,'changed_helpers':changes['changed'],
    'strict_original_scripts':38,'clean_candidate_processes':len(clean),
    'memory_open_scripts':[n for n,p in selections.items() if not p],
    'open_processes':open_processes,'open_findings':open_findings,
    'reports':reports,'raw':raw,'pass':True,
},indent=2)+'\n')
print('PASS: parser ownership; 38 strict originals;',len(clean),'clean processes; three prior profiles open')
