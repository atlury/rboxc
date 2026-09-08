#!/usr/bin/env python3
"""Audit original Findutils process logs without discarding traced child findings."""
# SPDX-License-Identifier: GPL-3.0-or-later
import argparse
import importlib.util
import json
from pathlib import Path
import re
import sys
ROOT=Path(__file__).resolve().parents[1]
sys.path.insert(0,str(ROOT/'tests'));sys.path.insert(0,str(ROOT/'tests/gnu'))
from comparison_profile import fingerprint
spec=importlib.util.spec_from_file_location('reviewed',ROOT/'tests/gnu/reviewed-original.py')
runner=importlib.util.module_from_spec(spec);spec.loader.exec_module(runner)
parser=argparse.ArgumentParser(description=__doc__)
parser.add_argument('--original',type=Path,required=True)
parser.add_argument('--expected-selections',type=int,required=True)
parser.add_argument('--report-name',required=True)
options=parser.parse_args();assert re.fullmatch(r'[a-z0-9-]+',options.report_name)
source=options.original.resolve(strict=True);data=json.loads(source.read_text())
assert len(data['results'])==data['total']==options.expected_selections>0
assert fingerprint(Path(data['binary']))==data['binary_sha256']
results=[];selections=[]
for row in data['results']:
 command=row['selection'].split('/')[0];expected=row['expected_assertions']
 for outcome in row['outcomes'].values():
  assert fingerprint(ROOT/outcome['driver_log'])==outcome['driver_log_sha256']
  assert outcome['assertions_pass'] and outcome['status']==0
  assert len(outcome['assertions'])==expected and all(a[0]=='PASS' for a in outcome['assertions'])
  for log in outcome['memory']:assert fingerprint(ROOT/log['log'])==log['sha256']
 assert row['assertions_pass']
 own=[]
 for log in row['outcomes']['rboxc-valgrind']['memory']:
  path=ROOT/log['log'];content=path.read_text();parsed=runner.parse_memory_log(content,path.stem,exec_only=True)
  assert all(log[k]==v for k,v in parsed.items())
  commands=re.findall(r'^==\d+== Command: (.*)$',content,re.M);assert commands
  executable=commands[-1].split(' ',1)[0]
  applet=bool(re.fullmatch(r'/tmp/rboxc-findutils-original-[^/]+/exec/'+command,executable))
  clean=parsed['complete_exec_log'] and parsed['errors']==0 and parsed['non_inherited_descriptors']==0 and not any(parsed['heap_bytes'].get(k,0) for k in ('definitely lost','indirectly lost','possibly lost'))
  assert log['command']==commands[-1] and log['applet_process']==applet and log['pass']==clean
  item={'selection':row['selection'],'path':log['log'],'sha256':log['sha256'],'command':commands[-1],'applet_process':applet,'pass':clean,**parsed}
  if not applet:
   if re.fullmatch(r'/tmp/rboxc-findutils-original-[^/]+/deps/[^/]+',executable):
    name=Path(executable).name
    if name in ('cmp','diff'):helper=ROOT/'build/gnu-diffutils/src'/name
    elif name=='sed':helper=ROOT/'build/gnu-sed/sed/sed'
    else:
     assert name in ('cat','rm','mkdir','chmod','touch','sort','echo','basename','cp','ln','true','false','sleep','ls','mv','mktemp','cut','id','date','printf','dd','rmdir')
     helper=ROOT/'build/gnu-coreutils/src/coreutils'
   else:helper=Path(executable)
   assert str(helper) in data['inputs'],'unrecorded child executable'
   assert fingerprint(helper)==data['inputs'][str(helper)]
   item['native_dependency']={'path':str(helper),'sha256':data['inputs'][str(helper)]}
  else:own.append(item)
  results.append(item)
 assert own and all(r['pass'] for r in own),'applet memory finding remains open'
 assert row['applet_assertions_and_memory_passed']
 selections.append({'selection':row['selection'],'assertions':expected,'applet_processes':len(own),'pass':True})
own=[r for r in results if r['applet_process']]
report={'scope':'Fresh parsing and hash verification of original Findutils logs. All registered assertions and applet processes must pass; traced native child findings remain explicit and do not become applet passes.',
        'binary_sha256':data['binary_sha256'],'original_report':str(source.relative_to(ROOT)),'original_report_sha256':fingerprint(source),
        'driver_sha256':fingerprint(Path(__file__)),'passed':len(own),'total':len(own),
        'selections_passed':len(selections),'selections_total':len(selections),'original_assertions_passed':sum(r['assertions'] for r in selections),
        'all_processes_passed':sum(r['pass'] for r in results),'all_processes_total':len(results),
        'external_child_findings':[r for r in results if not r['applet_process'] and not r['pass']],
        'selections':selections,'results':results}
(ROOT/'evidence'/(options.report_name+'.json')).write_text(json.dumps(report,indent=2)+'\n')
print('Audited',len(selections),'selections and',len(own),'clean applet processes;',len(report['external_child_findings']),'retained native child findings')
