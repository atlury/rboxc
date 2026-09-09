#!/usr/bin/env python3
"""Compare custom argv[0] through shell and printenv executable aliases."""
# SPDX-License-Identifier: GPL-3.0-or-later
import importlib.util,json,os,re,shutil,subprocess,sys,tempfile
from pathlib import Path
from comparison_profile import ComparisonProfile,fingerprint
ROOT=Path(__file__).resolve().parents[1]
sys.path.insert(0,str(ROOT/'tests/gnu'))
spec=importlib.util.spec_from_file_location('reviewed',ROOT/'tests/gnu/reviewed-original.py')
runner=importlib.util.module_from_spec(spec);spec.loader.exec_module(runner)
profile=ComparisonProfile('executable-name-contract',selections=True,oracle=ROOT/'build/gnu-bash/bash')
oracles={'gnu':ROOT/'build/gnu-bash/bash','rboxc':profile.binary}
printenv=ROOT/'build/bash-printenv-helper/printenv'
inputs={str(p):fingerprint(p) for p in [*oracles.values(),printenv,Path(__file__).resolve()]}
cases=[]
scripts=[('custom', 'exec -a specialname "$THIS_SH" --noprofile --norc -c \'printf "%s\\n" "$0"; set -o | while read option value; do case $option in posix) printf "%s:%s\\n" "$option" "$value";; esac; done\''), ('known-applet', 'exec -a cat "$THIS_SH" --noprofile --norc -c \'printf "%s\\n" "$0"; set -o | while read option value; do case $option in posix) printf "%s:%s\\n" "$option" "$value";; esac; done\''), ('dispatcher-name', 'exec -a rboxc "$THIS_SH" --noprofile --norc -c \'printf "%s\\n" "$0"; set -o | while read option value; do case $option in posix) printf "%s:%s\\n" "$option" "$value";; esac; done\''), ('login', 'exec -a -specialname "$THIS_SH" --noprofile --norc -c \'printf "%s\\n" "$0"; set -o | while read option value; do case $option in posix) printf "%s:%s\\n" "$option" "$value";; esac; done\''), ('posix-name', 'exec -a sh "$THIS_SH" --noprofile --norc -c \'printf "%s\\n" "$0"; set -o | while read option value; do case $option in posix) printf "%s:%s\\n" "$option" "$value";; esac; done\''), ('printenv-login', 'export RBOXC_VALUE=fixture; exec -l printenv RBOXC_VALUE'), ('printenv-clean-login', 'export RBOXC_VALUE=fixture; exec -c -l printenv RBOXC_VALUE')]
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
    (work/'exec/printenv').symlink_to(printenv if implementation=='gnu' else profile.binary)
    argv=[str(alias),'--noprofile','--norc',*args]
    logs=profile.logs/(str(index)+'-'+key);logs.mkdir()
    if instrument:argv=['/usr/bin/valgrind','--leak-check=full','--show-leak-kinds=all','--track-fds=yes','--trace-children=yes','--log-file='+str(logs/'process-%p.log'),*argv]
    done=subprocess.run(argv,input=data,cwd=work/'files',capture_output=True,timeout=60,
      env={'THIS_SH':str(alias),'LC_ALL':'C','LANGUAGE':'C','PATH':str(work/'exec')+':/usr/bin:/bin','HOME':directory,'TZ':'UTC0'})
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
 report={**profile.metadata(),'inputs':inputs,'scope':'Twenty-eight custom invocation comparisons across four shell aliases retain argv[0], login and POSIX behavior, known-applet name collisions and printenv with a login name and clean environment. Pinned standalone native GNU printenv is the oracle; candidate helpers remain integrated. Every traced candidate image requires complete clean heap and descriptor summaries.',
  'complete':len(results)==len(cases),'planned_total':len(cases),'passed':sum(r['pass'] for r in results),'total':len(results),'results':results}
 profile.report.write_text(json.dumps(report,indent=2)+'\n')
 print('PASS' if results[-1]['pass'] else 'OPEN',name,flush=True)
raise SystemExit(report['passed']!=report['total'])
