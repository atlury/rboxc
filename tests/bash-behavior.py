#!/usr/bin/env python3
"""Compare GNU Bash, the C control-flow lowering, and the translated Rust entry."""
# SPDX-License-Identifier: GPL-3.0-or-later
import importlib.util,json,os,re,shutil,subprocess,sys,tempfile
from pathlib import Path
from comparison_profile import ComparisonProfile,fingerprint
ROOT=Path(__file__).resolve().parents[1]
sys.path.insert(0,str(ROOT/'tests/gnu'))
spec=importlib.util.spec_from_file_location('reviewed',ROOT/'tests/gnu/reviewed-original.py')
runner=importlib.util.module_from_spec(spec);spec.loader.exec_module(runner)
profile=ComparisonProfile('bash-behavior')
oracles={'gnu':ROOT/'build/gnu-bash/bash','gated-c':ROOT/'build/translation/bash/gated-bash','rboxc':profile.binary}
inputs={str(p):fingerprint(p) for p in [*oracles.values(),Path(__file__).resolve()]}
cases=[]
for name in ('bash','sh','-bash','-sh'):
 for opt in ('--help','--version'):
  cases.append((name+opt,name,[opt],b'',{}))
 cases.append((name+'-stdin',name,[],b'printf "stdin:%s\\n" "$((6*7))"\n',{}))
scripts=[
 ('print','printf "hello\\n"'),
 ('arithmetic','x=4; printf "%s\\n" "$((x+3))"'),
 ('variables','declare -a a=(alpha beta); declare -A m=([key]=value); printf "%s:%s\\n" "${a[1]}" "${m[key]}"'),
 ('loops','for x in a b c; do if [[ $x == b ]]; then continue; fi; printf "%s\\n" "$x"; done'),
 ('functions','f(){ local x=inner; printf "%s\\n" "$x"; return 7; }; x=outer; f; printf "%s:%s\\n" "$?" "$x"'),
 ('errexit','set -e; false; printf "unexpected\\n"'),
 ('conditional','[[ alpha == a* && 17 -gt 4 ]]; printf "%s\\n" "$?"'),
 ('regex','[[ ab12 =~ ^[a-z]+([0-9]+)$ ]]; printf "%s:%s\\n" "$?" "${BASH_REMATCH[1]}"'),
 ('subshell','(printf "child\\n"); printf "parent\\n"'),
 ('command-substitution','x=$(printf value); printf "%s\\n" "$x"'),
 ('exit-trap',"trap 'printf \"exit-trap\\n\"' EXIT; printf 'body\\n'"),
 ('interrupt-trap',"trap 'printf \"interrupt-trap\\n\"' INT; kill -s INT $$; printf 'after\\n'"),
 ('source','source ./input one two; printf "source-status:%s\\n" "$?"'),
 ('exec-no-shebang','./plain alpha; printf "outer\\n"'),
 ('exec-no-shebang-subshell','(exec ./plain beta); printf "parent\\n"'),
 ('job-wait','(exit 3) & p=$!; wait "$p"; printf "wait:%s\\n" "$?"'),
 ('pipeline','printf "alpha\\nbeta\\n" | while IFS= read -r line; do printf "line:%s\\n" "$line"; done'),
 ('syntax-error','if then'),
 ('return-outside-function','return 3; printf "after:%s\\n" "$?"'),
 ('shift','set -- one two three; shift; printf "%s:%s\\n" "$#" "$1"')]
files={'input':(b'printf "sourced:%s:%s\\n" "$1" "$2"\nreturn 4\n',0o640),
       'plain':(b'printf "plain:%s\\n" "$1"\n',0o755)}
for name,script in scripts:cases.append((name,'bash',['-c',script],b'',files))
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
 report={**profile.metadata(),'inputs':inputs,'scope':'Original GNU, separately lowered C, and Rust Bash entries on bounded shell fixtures, with per-process Valgrind logs including forked children. Complete parent and child summaries are required; native GNU findings remain baseline observations. Full Bash original suites, interactive job control, and loadable builtins remain open.',
  'complete':len(results)==len(cases),'planned_total':len(cases),'passed':sum(r['pass'] for r in results),'total':len(results),'results':results}
 profile.report.write_text(json.dumps(report,indent=2)+'\n')
 print('PASS' if results[-1]['pass'] else 'OPEN',name,flush=True)
raise SystemExit(report['passed']!=report['total'])
