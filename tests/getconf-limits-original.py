#!/usr/bin/env python3
"""Run GNU's unchanged getconf limits-header consistency test."""
# SPDX-License-Identifier: GPL-3.0-or-later
import importlib.util
import json
import os
from pathlib import Path
import re
import shlex
import subprocess
import sys
import tempfile
from comparison_profile import ComparisonProfile,fingerprint

ROOT=Path(__file__).resolve().parents[1]
sys.path.insert(0,str(ROOT/'tests/gnu'))
spec=importlib.util.spec_from_file_location('reviewed',ROOT/'tests/gnu/reviewed-original.py')
runner=importlib.util.module_from_spec(spec);spec.loader.exec_module(runner)
profile=ComparisonProfile('getconf-limits-original')
pin=json.loads((ROOT/'inventory/sources.json').read_text())['glibc']
source=Path(pin['source'])/'posix/tst-getconf-limits.py'
oracle=ROOT/'build/gnu-glibc/posix/getconf'
record=json.loads((ROOT/'build/translation/getconf/compile_commands.json').read_text())[0]
args=['gcc'];skip=False
for word in record['arguments'][1:]:
 if skip:skip=False;continue
 if word in ('-o','-MF','-MT'):skip=True;continue
 if word in ('-c','-MD','-MP') or word.endswith('.c') or word.startswith('-Dmain='):continue
 args.append(word)
inputs={str(p):fingerprint(p) for p in [source,oracle,Path(__file__).resolve(),
        Path(pin['source'])/'scripts/glibcextract.py']}
outcomes={}
for implementation in ('gnu','rboxc'):
 for instrument in (False,True):
  key=implementation+('-valgrind' if instrument else '')
  with tempfile.TemporaryDirectory(prefix='rboxc-getconf-limits-') as directory:
   work=Path(directory);alias=work/'getconf';alias.symlink_to(oracle if implementation=='gnu' else profile.binary)
   log=profile.logs/(key+'.log');command=[str(alias)]
   if instrument:command=['/usr/bin/valgrind','--leak-check=full','--show-leak-kinds=all',
                          '--track-fds=yes','--log-file='+str(log),*command]
   wrapper=work/'run-getconf';wrapper.write_text('#!/bin/sh\nexec '+shlex.join(command)+' "$@"\n');wrapper.chmod(0o755)
   done=subprocess.run(['python3',str(source),'--cc',shlex.join(args),str(wrapper)],capture_output=True,
      cwd=record['directory'],env={**os.environ,'PYTHONPATH':str(Path(pin['source'])/'scripts'),
      'LC_ALL':'C','LANGUAGE':'C','HOME':directory,'TMPDIR':directory},timeout=60)
   outcome={'status':done.returncode,'stdout':done.stdout.hex(),'stderr':done.stderr.hex()}
   clean=True
   if instrument:
    contents=log.read_text();pids=set(re.findall(r'^==([0-9]+)==',contents,re.M));assert len(pids)==1
    parsed=runner.parse_memory_log(contents,pids.pop(),exec_only=True)
    clean=parsed['complete_exec_log'] and parsed['errors']==0 and parsed['non_inherited_descriptors']==0 and not any(parsed['heap_bytes'].get(k,0) for k in ('definitely lost','indirectly lost','possibly lost'))
    outcome['memory']={'log':str(log.relative_to(ROOT)),'sha256':fingerprint(log),**parsed,'clean':clean}
   outcome['pass']=done.returncode==0 and not done.stdout and not done.stderr and (implementation!='rboxc' or clean)
   outcomes[key]=outcome
   for path,expected in inputs.items():assert fingerprint(Path(path))==expected
   report={**profile.metadata(),'scope':'Unchanged GNU 2.43 getconf limits-header consistency test with pinned GNU compile flags and headers. Four native/Valgrind modes; runtime libraries remain the recorded host profile.',
      'inputs':inputs,'c_arguments':args,'c_directory':record['directory'],'source':str(source),
      'complete':len(outcomes)==4,'passed':sum(o['pass'] for o in outcomes.values()),'total':len(outcomes),
      'planned_total':4,'outcomes':outcomes}
   profile.report.write_text(json.dumps(report,indent=2)+'\n')
   print(key,'PASS' if outcome['pass'] else 'OPEN',flush=True)
raise SystemExit(report['passed']!=4)
