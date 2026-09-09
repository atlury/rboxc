#!/usr/bin/env python3
"""Compare ordinary multibyte splitting and command diagnostics with GNU Bash."""
# SPDX-License-Identifier: GPL-3.0-or-later
import importlib.util,json,os,re,shutil,subprocess,sys,tempfile
from pathlib import Path
from comparison_profile import ComparisonProfile,fingerprint
ROOT=Path(__file__).resolve().parents[1]
sys.path.insert(0,str(ROOT/'tests/gnu'))
spec=importlib.util.spec_from_file_location('reviewed',ROOT/'tests/gnu/reviewed-original.py')
runner=importlib.util.module_from_spec(spec);spec.loader.exec_module(runner)
profile=ComparisonProfile('bash-multibyte-contract',selections=True,oracle=ROOT/'build/gnu-bash/bash')
oracles={'gnu':ROOT/'build/gnu-bash/bash','rboxc':profile.binary}
inputs={str(p):fingerprint(p) for p in [*oracles.values(),Path(__file__).resolve()]}
cases=[]
scripts=[('unicode-ifs', 'export LC_ALL=en_US.UTF-8; IFS=é; v=\'aébéc\'; set -- $v; printf \'%s:<%s>\\n\' "$#" "$@"'), ('ascii-ifs', 'export LC_ALL=en_US.UTF-8; IFS=:; v=\'é:€:中\'; set -- $v; printf \'%s:<%s>\\n\' "$#" "$@"'), ('empty-ifs', 'export LC_ALL=en_US.UTF-8; IFS=; v=\'é:€:中\'; set -- $v; printf \'%s:<%s>\\n\' "$#" "$@"'), ('mixed-ifs', 'export LC_ALL=en_US.UTF-8; IFS=\' é\'; v=\' a ébéc \'; set -- $v; printf \'%s:<%s>\\n\' "$#" "$@"'), ('missing-ascii', 'command rboxc-missing-command; printf \'status:%s\\n\' "$?"'), ('missing-unicode', 'LC_ALL=C; command \'rboxc-missing-é\'; printf \'status:%s\\n\' "$?"')]
files={}
for alias in ('bash','sh','-bash','-sh'):
 for name,script in scripts:cases.append((alias+':'+name,alias,['-c',script],b'',files))
if profile.options.commands:cases=[c for c in cases if c[0] in profile.options.commands]
results=[]
for index,(name,command,args,data,files) in enumerate(cases):
 outcomes={}
 for implementation,binary in oracles.items():
  for instrument in (False,True):
   key=implementation+('-valgrind' if instrument else '')
   with tempfile.TemporaryDirectory(prefix='rboxc-bash-') as directory:
    work=Path(directory);(work/'exec').mkdir();(work/'files').mkdir()
    for file,(contents,mode) in files.items():
     p=work/'files'/file;p.write_bytes(contents);p.chmod(mode)
    alias=work/'exec'/command;alias.symlink_to(binary)
    argv=[str(alias),'--noprofile','--norc',*args]
    logs=profile.logs/(str(index)+'-'+key);logs.mkdir()
    if instrument:argv=['/usr/bin/valgrind','--leak-check=full','--show-leak-kinds=all','--track-fds=yes','--trace-children=yes','--log-file='+str(logs/'process-%p.log'),*argv]
    done=subprocess.run(argv,input=data,cwd=work/'files',capture_output=True,timeout=60,
      env={'LC_ALL':'C','LANGUAGE':'C','PATH':str(work/'exec')+':/usr/bin:/bin','HOME':directory,'TZ':'UTC0'})
    row={'status':done.returncode,'stdout':done.stdout.replace(directory.encode(),b'<fixture>').hex(),
         'stderr':done.stderr.replace(directory.encode(),b'<fixture>').hex(),'raw_stdout':done.stdout.hex(),'raw_stderr':done.stderr.hex(),
         'tree':{str(p.relative_to(work/'files')):{'sha256':fingerprint(p),'mode':p.stat().st_mode&0o7777} for p in sorted((work/'files').rglob('*')) if p.is_file()}}
    memory=[]
    for log in sorted(logs.glob('process-*.log')):
     contents=log.read_text();pids=set(re.findall(r'^==([0-9]+)==',contents,re.M));assert len(pids)==1
     pid=pids.pop();parsed=runner.parse_memory_log(contents,pid)
     errors=list(re.finditer(r'ERROR SUMMARY:',contents));fds=list(re.finditer(r'FILE DESCRIPTORS:',contents))
     images=list(re.finditer(r'^==[0-9]+== Command:',contents,re.M))
     complete=bool(errors) and len(errors)==len(fds) and (not images or errors[-1].start()>images[-1].start() and fds[-1].start()>images[-1].start())
     clean=complete and parsed['errors']==0 and parsed['non_inherited_descriptors']==0 and not any(parsed['heap_bytes'].get(k,0) for k in ('definitely lost','indirectly lost','possibly lost'))
     memory.append({'log':str(log.relative_to(ROOT)),'sha256':fingerprint(log),'pid':pid,'images':len(images),'complete':complete,'clean':clean,**parsed})
    assert not instrument or memory
    row['memory']=memory;outcomes[key]=row
 reference=outcomes['gnu'];equivalent=all(all(row[k]==reference[k] for k in ('status','stdout','stderr','tree')) for row in outcomes.values())
 clean=all(m['clean'] for m in outcomes['rboxc-valgrind']['memory'])
 results.append({'name':name,'command':command,'args':args,'input':data.hex(),'equivalent':equivalent,'memory_clean':clean,'pass':equivalent and clean,'outcomes':outcomes})
 for path,value in inputs.items():assert fingerprint(Path(path))==value
 report={**profile.metadata(),'inputs':inputs,'scope':'Six ordinary multibyte splitting and missing-command diagnostic cases through four shell aliases compare native GNU with the immutable Rboxc candidate. Includes valid Unicode delimiters, ASCII delimiters, empty/mixed IFS and borrowed or quoted command display names. Complete parent and child Valgrind summaries are required. Native findings remain baseline observations; this is focused ownership evidence, not full Bash acceptance.',
  'complete':len(results)==len(cases),'planned_total':len(cases),'passed':sum(r['pass'] for r in results),'total':len(results),'results':results}
 profile.report.write_text(json.dumps(report,indent=2)+'\n')
 print('PASS' if results[-1]['pass'] else 'OPEN',name,flush=True)
raise SystemExit(report['passed']!=report['total'])
