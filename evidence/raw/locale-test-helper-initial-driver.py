#!/usr/bin/env python3
"""Compare native locale enumeration with finite private original-format data."""
# SPDX-License-Identifier: GPL-3.0-or-later
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
ROOT=Path(__file__).resolve().parents[1]
sys.path[:0]=[str(ROOT/'tests/gnu')]
spec=importlib.util.spec_from_file_location('reviewed',ROOT/'tests/gnu/reviewed-original.py')
runner=importlib.util.module_from_spec(spec);spec.loader.exec_module(runner)
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
report_path=ROOT/'evidence/locale-test-helper-contract.json';assert not report_path.exists()
build=json.loads((ROOT/'evidence/locale-test-helper-build.json').read_text())
base=ROOT/'evidence/raw/locale-test-helper-contract';base.mkdir(exist_ok=True)
logs=Path(tempfile.mkdtemp(prefix='run-',dir=base))
archive=ROOT/'build/bash-intl-locales/usr/lib/locale/locale-archive'
inputs={str(p):sha(p) for p in [Path(__file__),ROOT/'evidence/locale-test-helper-build.json',archive,Path('/usr/bin/locale'),Path('/usr/bin/valgrind'),Path('/usr/bin/unshare'),Path('/usr/bin/mount'),Path('/bin/sh').resolve()]}
results=[]
for fixture in ['system','archive-directory-aliases','directory-aliases','empty']:
 with tempfile.TemporaryDirectory(prefix='locale-contract-',dir=ROOT/'build') as directory:
  work=Path(directory);loc=work/'locale';loc.mkdir();aliases=work/'aliases';aliases.mkdir()
  (aliases/'locale.alias').write_text('C C\nPOSIX C\ncopy C\ncopy C\nignored unavailable\n')
  mounts=[]
  if fixture!='system':
   if fixture!='empty':
    shutil.copytree('/usr/lib/locale/C.utf8',loc/'C.utf8')
    for p in Path('/usr/lib/locale/C.utf8').rglob('*'):
     if p.is_file():inputs[str(p)]=sha(p)
   mounts=[str(loc),'/usr/lib/locale',str(aliases),'/usr/share/locale']
   if fixture=='archive-directory-aliases':
    (loc/'locale-archive').touch();mounts += [str(archive),str(loc/'locale-archive')]
  for args in [['-a'],['-v','-a']]:
   name=fixture+('-verbose' if '-v' in args else '-names');outcomes={}
   for impl,binary in [('host',Path('/usr/bin/locale')),*[(n,Path(d['binary'])) for n,d in build['builds'].items()]]:
    for instrument in [False,True]:
     key=impl+('-valgrind' if instrument else '');prefix=logs/(name+'-'+key)
     command=[str(binary),*args]
     if instrument:command=['/usr/bin/valgrind','--leak-check=full','--show-leak-kinds=all','--track-fds=yes','--trace-children=yes','--log-file='+str(prefix)+'-%p.log',*command]
     if mounts:
      command=['/usr/bin/unshare','--mount','--propagation','private','/bin/sh','-c','while [ "$1" != -- ]; do /usr/bin/mount --bind "$1" "$2" || exit 77; shift 2; done; shift; exec "$@"','locale-test-mounts',*mounts,'--',*command]
     done=subprocess.run(command,cwd=work,env={'LC_ALL':'C','PATH':'/usr/bin:/bin'},stdin=subprocess.DEVNULL,stdout=subprocess.PIPE,stderr=subprocess.PIPE,timeout=60)
     raw={}
     for suffix,data in [('stdout',done.stdout),('stderr',done.stderr)]:
      p=Path(str(prefix)+'-'+suffix);p.write_bytes(data);raw[str(p.relative_to(ROOT))]=sha(p)
     memory=[]
     for log in sorted(logs.glob(prefix.name+'-*.log')):
      pid=log.stem.rsplit('-',1)[-1];parsed=runner.parse_memory_log(log.read_text(),pid,True)
      memory.append({'log':str(log.relative_to(ROOT)),'sha256':sha(log),**parsed})
     assert bool(memory)==instrument
     clean=all(m['complete_exec_log'] and m['errors']==m['non_inherited_descriptors']==0 and not any(m['heap_bytes'].get(k,0) for k in ['definitely lost','indirectly lost','possibly lost']) for m in memory)
     outcomes[key]={'status':done.returncode,'stdout':done.stdout.hex(),'stderr':done.stderr.hex(),'raw':raw,'memory':memory,'clean':clean if instrument else None}
   reference=outcomes['host']
   behavior=all(all(o[k]==reference[k] for k in ['status','stdout','stderr']) for o in outcomes.values())
   passed=behavior and outcomes['cleanup-valgrind']['clean']
   results.append({'name':name,'arguments':args,'behavior_pass':behavior,'pass':passed,'outcomes':outcomes})
   report_path.write_text(json.dumps({'scope':'Native test-helper enumeration only: original and adapted GNU glibc sources use identical configured host data paths and match host locale. Private known-good archive/directory/duplicate-alias fixtures; no installed helper changes.', 'inputs':inputs,'build_sha256':sha(ROOT/'evidence/locale-test-helper-build.json'),'results':results,'passed':sum(r['pass'] for r in results),'total':len(results),'planned_total':8,'complete':len(results)==8},indent=2)+'\n')
   print(('PASS' if passed else 'OPEN'),name,flush=True)
raise SystemExit(0 if all(r['pass'] for r in results) else 1)
