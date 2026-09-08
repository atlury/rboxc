#!/usr/bin/env python3
"""Reparse Ed's complete original-suite logs and retain child boundaries."""
# SPDX-License-Identifier: GPL-3.0-or-later
import argparse
import importlib.util
import json
from pathlib import Path
import re
import sys

ROOT=Path(__file__).resolve().parents[1]
sys.path.insert(0,str(ROOT/'tests'))
sys.path.insert(0,str(ROOT/'tests/gnu'))
from comparison_profile import fingerprint
spec=importlib.util.spec_from_file_location('reviewed',ROOT/'tests/gnu/reviewed-original.py')
runner=importlib.util.module_from_spec(spec);spec.loader.exec_module(runner)
parser=argparse.ArgumentParser(description=__doc__)
parser.add_argument('--original',required=True,type=Path)
parser.add_argument('--report-name',required=True)
options=parser.parse_args();assert re.fullmatch(r'[a-z0-9-]+',options.report_name)
source=options.original.resolve(strict=True);original=json.loads(source.read_text())
assert fingerprint(Path(original['binary']))==original['binary_sha256']
assert original['assertions_passed'] and original['editor_assertions_and_memory_passed']
for outcome in original['outcomes'].values():
    assert outcome['assertions_pass'] and fingerprint(ROOT/outcome['log'])==outcome['log_sha256']
    for log in outcome.get('memory',[]):assert fingerprint(ROOT/log['log'])==log['sha256']
results=[]
for log in original['outcomes']['rboxc-valgrind']['memory']:
    path=ROOT/log['log'];text=path.read_text()
    parsed=runner.parse_memory_log(text,path.stem,exec_only=True)
    clean=parsed['complete_exec_log'] and parsed['errors']==0 and parsed['non_inherited_descriptors']==0 and not any(parsed['heap_bytes'].get(k,0) for k in ('definitely lost','indirectly lost','possibly lost'))
    assert clean==log['pass']
    commands=re.findall(r'^==\d+== Command: (.*)$',text,re.M)
    assert commands and commands[-1]==log['command']
    executable=commands[-1].split(' ',1)[0]
    editor=bool(re.fullmatch(r'/tmp/rboxc-ed-original-[^/]+/real/ed',executable))
    assert editor==log['ed_process']
    item={'path':str(path.relative_to(ROOT)),'sha256':log['sha256'],'command':log['command'],'ed_process':editor,'pass':clean,**parsed}
    if '/deps/' in executable:
        helper=original['prerequisites']['commands'][Path(executable).name]
        assert fingerprint(Path(helper['path']))==helper['sha256'];item['native_dependency']=helper
    elif executable=='/bin/sh':
        helper=original['prerequisites']['shell']
        assert fingerprint(Path(helper['path']))==helper['sha256'];item['native_dependency']=helper
    results.append(item)
editor=[r for r in results if r['ed_process']]
assert len(editor)==original['outcomes']['rboxc-valgrind']['editor_processes']==242
assert all(r['pass'] for r in editor)
report={'scope':'Strict final-exec audit of all 242 Ed processes in the full original suite. Traced native child findings are retained separately and are not counted as clean Ed processes.',
    'binary_sha256':original['binary_sha256'],'original_report':str(source.relative_to(ROOT)),
    'original_report_sha256':fingerprint(source),'driver_sha256':fingerprint(Path(__file__)),
    'passed':len(editor),'total':len(editor),'all_processes_passed':sum(r['pass'] for r in results),
    'all_processes_total':len(results),'external_child_findings':[r for r in results if not r['ed_process'] and not r['pass']],
    'results':results}
(ROOT/'evidence'/(options.report_name+'.json')).write_text(json.dumps(report,indent=2)+'\n')
print('Audited',len(editor),'clean Ed processes;',len(report['external_child_findings']),'retained native child findings')
