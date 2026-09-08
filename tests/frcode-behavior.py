#!/usr/bin/env python3
"""Compare the private GNU filename encoder on ordinary pathname lists."""
# SPDX-License-Identifier: GPL-3.0-or-later
import importlib.util,json,re,subprocess,sys,tempfile
from pathlib import Path
from comparison_profile import ComparisonProfile,fingerprint
ROOT=Path(__file__).resolve().parents[1];sys.path.insert(0,str(ROOT/'tests/gnu'))
spec=importlib.util.spec_from_file_location('reviewed',ROOT/'tests/gnu/reviewed-original.py');runner=importlib.util.module_from_spec(spec);spec.loader.exec_module(runner)
p=ComparisonProfile('frcode-behavior',oracle=ROOT/'build/gnu-findutils/locate/frcode')
paths=b'/alpha\n/alpha/one\n/alpha/two\n/beta\n'
cases=[('help',['--help'],b''),('version',['--version'],b''),('option',['--unknown'],b''),('extra',['extra'],b''),('empty',[],b''),('lines',[],paths),('nul',['-0'],paths.replace(b'\n',b'\0')),('slocate-0',['-S','0'],paths),('slocate-1',['-S','1'],paths),('prefix',[],b'/'+b'a'*160+b'/one\n/'+b'a'*160+b'/two\n/z\n')]
results=[];driver=fingerprint(Path(__file__))
for index,(name,args,data) in enumerate(cases):
 outcomes={}
 for candidate in (False,True):
  for instrument in (False,True):
   key=('rboxc' if candidate else 'gnu')+('-valgrind' if instrument else '')
   with tempfile.TemporaryDirectory(prefix='rboxc-frcode-') as directory:
    alias=Path(directory)/'frcode';alias.symlink_to(p.binary if candidate else p.oracle)
    argv=[str(alias),*args];log=p.logs/(str(index)+'-'+key+'.log')
    if instrument:argv=['valgrind','--leak-check=full','--show-leak-kinds=all','--track-fds=yes','--log-file='+str(log),*argv]
    done=subprocess.run(argv,input=data,capture_output=True,timeout=30,env={'LC_ALL':'C','PATH':'/usr/bin:/bin'})
    row={'status':done.returncode,'stdout':done.stdout.replace(directory.encode(),b'<fixture>').hex(),'stderr':done.stderr.replace(directory.encode(),b'<fixture>').hex()}
    if instrument:
     content=log.read_text();pid=re.search(r'^==([0-9]+)==',content)[1];m=runner.parse_memory_log(content,pid)
     row['memory']={'log':str(log.relative_to(ROOT)),'sha256':fingerprint(log),**m}
    outcomes[key]=row
 equivalent=all(all(v[k]==outcomes['gnu'][k] for k in ('status','stdout','stderr')) for v in outcomes.values())
 m=outcomes['rboxc-valgrind']['memory'];clean=m['errors']==0 and m['non_inherited_descriptors']==0 and not any(m['heap_bytes'].get(k,0) for k in ('definitely lost','indirectly lost','possibly lost'))
 results.append({'name':name,'args':args,'data':data.hex(),'equivalent':equivalent,'memory_clean':clean,'pass':equivalent and clean,'outcomes':outcomes})
assert fingerprint(Path(__file__))==driver
r={**p.metadata(),'driver_sha256':driver,'complete':True,'passed':sum(x['pass'] for x in results),'total':len(cases),'results':results};p.report.write_text(json.dumps(r,indent=2)+'\n')
print('frcode',r['passed'],r['total']);raise SystemExit(r['passed']!=r['total'])
