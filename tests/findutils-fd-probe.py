#!/usr/bin/env python3
"""Compare the original and adapted GNU fd scan, including O_PATH and callback stop."""
# SPDX-License-Identifier: GPL-3.0-or-later
import importlib.util
import json
import re
from pathlib import Path
import subprocess
import sys
from comparison_profile import fingerprint
ROOT=Path(__file__).resolve().parents[1]
sys.path.insert(0,str(ROOT/'tests/gnu'))
spec=importlib.util.spec_from_file_location('reviewed',ROOT/'tests/gnu/reviewed-original.py')
runner=importlib.util.module_from_spec(spec);spec.loader.exec_module(runner)
pin=json.loads((ROOT/'inventory/sources.json').read_text())['findutils']
original=Path(pin['source'])/'lib/fdleak.c';adapted=ROOT/'build/findutils-cleanup/fdleak.c'
assert fingerprint(original)==pin['helper_source_sha256']['fdleak']
cleanup=json.loads((ROOT/'evidence/findutils-native-cleanup.json').read_text())
assert fingerprint(adapted)==cleanup['adapted_source_sha256']
records=[json.loads(p.read_text()) for p in (ROOT/'build/findutils-cc-records').glob('*.json')]
record=next(r for r in records if Path(r['file'])==original)
raw=ROOT/'evidence/raw/findutils-fd-probe';raw.mkdir(exist_ok=True)
source=ROOT/'tests/findutils-fd-probe.c';results={}
for kind,include in [('gnu',original),('adapted',adapted)]:
 binary=raw/kind
 args=record['arguments'].copy();args.remove('-c')
 args[args.index('-o')+1]=str(binary);args[args.index(str(original))]=str(source)
 args+=['-I'+str(original.parent),'-DRBOXC_FDLEAK_SOURCE="'+str(include)+'"',str(ROOT/'build/gnu-findutils/lib/libfind.a'),str(ROOT/'build/gnu-findutils/gl/lib/libgnulib.a'),'-lm']
 with (raw/(kind+'-build.log')).open('w') as out:subprocess.run(args,cwd=record['directory'],stdout=out,stderr=subprocess.STDOUT,check=True)
 for vg in (False,True):
  key=kind+('-valgrind' if vg else '');cmd=[str(binary)];log=raw/(key+'.log')
  if vg:cmd=['/usr/bin/valgrind','--leak-check=full','--show-leak-kinds=all','--track-fds=yes','--log-file='+str(log),*cmd]
  done=subprocess.run(cmd,cwd=raw,stdout=subprocess.PIPE,stderr=subprocess.PIPE,timeout=30)
  row={'status':done.returncode,'stdout':done.stdout.decode(),'stderr':done.stderr.decode(),'binary_sha256':fingerprint(binary)}
  if vg:
   content=log.read_text();pids=set(re.findall(r'^==([0-9]+)==',content,re.M));assert len(pids)==1
   row['memory']={**runner.parse_memory_log(content,pids.pop(),exec_only=True),'log':str(log.relative_to(ROOT)),'sha256':fingerprint(log)}
  results[key]=row
same=all(all(r[k]==results['gnu'][k] for k in ('status','stdout','stderr')) for r in results.values())
m=results['adapted-valgrind']['memory'];clean=m['complete_exec_log'] and m['errors']==0 and m['non_inherited_descriptors']==0 and not any(m['heap_bytes'].get(k,0) for k in ('definitely lost','indirectly lost','possibly lost'))
passed=same and results['gnu']['status']==0 and clean
report={'scope':'Original and adapted private GNU descriptor iteration with a regular descriptor, two closed holes, O_PATH, a live pipe, callback early return, and preserved errno. The original poll warnings remain recorded; only the adapted helper must be memory/descriptor clean.',
        'passed':int(passed),'total':1,'equivalent':same,'memory_clean':clean,'inputs':{str(p):fingerprint(p) for p in (Path(__file__),source,original,adapted)},'results':results}
(ROOT/'evidence/findutils-fd-probe.json').write_text(json.dumps(report,indent=2)+'\n')
print('PASS' if passed else 'OPEN','GNU descriptor callback equivalence')
raise SystemExit(not passed)
