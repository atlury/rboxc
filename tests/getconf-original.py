#!/usr/bin/env python3
"""Run GNU's unchanged getconf shell test against native and translated entries."""
# SPDX-License-Identifier: GPL-3.0-or-later
import importlib.util
import json
import os
from pathlib import Path
import re
import shlex
import shutil
import subprocess
import sys
import tempfile
from comparison_profile import ComparisonProfile,fingerprint

ROOT=Path(__file__).resolve().parents[1]
sys.path.insert(0,str(ROOT/'tests/gnu'))
spec=importlib.util.spec_from_file_location('reviewed',ROOT/'tests/gnu/reviewed-original.py')
runner=importlib.util.module_from_spec(spec);spec.loader.exec_module(runner)
profile=ComparisonProfile('getconf-original')
pin=json.loads((ROOT/'inventory/sources.json').read_text())['glibc']
source=Path(pin['source'])/'posix/tst-getconf.sh'
source_hash=fingerprint(source)
# All invocations in this source query configuration or filesystem limits.
blocks=re.findall(r'done <<EOF\n(.*?)\nEOF',source.read_text(),re.S)
assert len(blocks)==2
expected_queries=sum(len(b.splitlines()) for b in blocks)
oracle=ROOT/'build/gnu-glibc/posix/getconf'
inputs={str(source):source_hash,str(oracle):fingerprint(oracle),
        str(Path(__file__).resolve()):fingerprint(Path(__file__))}
outcomes={}
for implementation in ('gnu','rboxc'):
 for instrument in (False,True):
  key=implementation+('-valgrind' if instrument else '')
  with tempfile.TemporaryDirectory(prefix='rboxc-getconf-original-') as directory:
   work=Path(directory);(work/'posix').mkdir();(work/'exec').mkdir()
   alias=work/'exec/getconf';alias.symlink_to(oracle if implementation=='gnu' else profile.binary)
   logs=profile.logs/key;logs.mkdir()
   command=[str(alias)]
   if instrument:command=['/usr/bin/valgrind','--leak-check=full','--show-leak-kinds=all',
      '--track-fds=yes','--log-file='+str(logs/'process-%p.log'),*command]
   wrapper=work/'run-getconf'
   wrapper.write_text('#!/bin/sh\nexec '+shlex.join(command)+' "$@"\n');wrapper.chmod(0o755)
   done=subprocess.run(['/bin/sh',str(source),str(work),str(wrapper)],capture_output=True,
     env={'PATH':'/usr/bin:/bin','LC_ALL':'C','LANGUAGE':'C','HOME':directory},cwd=work,timeout=600)
   raw=work/'posix/tst-getconf.out';saved=logs/'original-output.log';shutil.copy2(raw,saved)
   output=raw.read_text();queries=sum(line.startswith('getconf ') for line in output.splitlines())
   memory=[]
   for path in sorted(logs.glob('process-*.log')):
    contents=path.read_text();pids=set(re.findall(r'^==([0-9]+)==',contents,re.M));assert len(pids)==1
    parsed=runner.parse_memory_log(contents,pids.pop(),exec_only=True)
    clean=parsed['complete_exec_log'] and parsed['errors']==0 and parsed['non_inherited_descriptors']==0 and not any(parsed['heap_bytes'].get(k,0) for k in ('definitely lost','indirectly lost','possibly lost'))
    memory.append({'log':str(path.relative_to(ROOT)),'sha256':fingerprint(path),**parsed,'clean':clean})
   assert not instrument or len(memory)==expected_queries
   passed=done.returncode==0 and queries==expected_queries and 'FAILED' not in output
   if implementation=='rboxc' and instrument:passed=passed and all(r['clean'] for r in memory)
   outcomes[key]={'status':done.returncode,'stdout':done.stdout.hex(),'stderr':done.stderr.hex(),
    'queries':queries,'log':str(saved.relative_to(ROOT)),'log_sha256':fingerprint(saved),'memory':memory,'pass':passed}
   for path,expected in inputs.items():assert fingerprint(Path(path))==expected
   report={**profile.metadata(),'scope':'Unchanged GNU 2.43 tst-getconf.sh configuration and path-limit checks on the host runtime. Native and Valgrind outcomes are retained; every candidate query must have a complete clean memory log. This is one original shell test, not the glibc library suite.',
    'inputs':inputs,'source':str(source),'expected_queries_per_mode':expected_queries,
    'complete':len(outcomes)==4,'planned_total':4,'passed':sum(o['pass'] for o in outcomes.values()),'total':len(outcomes),'outcomes':outcomes}
   profile.report.write_text(json.dumps(report,indent=2)+'\n')
   print(key,queries,'queries','PASS' if passed else 'OPEN',flush=True)
raise SystemExit(report['passed']!=4)
