#!/usr/bin/env python3
"""Compare GNU BC and DC arithmetic, state, file input, and diagnostics."""
# SPDX-License-Identifier: GPL-3.0-or-later
import importlib.util
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import time
from comparison_profile import ComparisonProfile, fingerprint

ROOT=Path(__file__).resolve().parents[1]
sys.path.insert(0,str(ROOT/'tests/gnu'))
spec=importlib.util.spec_from_file_location('reviewed',ROOT/'tests/gnu/reviewed-original.py')
runner=importlib.util.module_from_spec(spec);spec.loader.exec_module(runner)
profile=ComparisonProfile('bc-behavior',oracle=ROOT/'build/gnu-bc/bc/bc',selections=True)
oracles={name:ROOT/'build/gnu-bc'/name/name for name in ('bc','dc')}
hashes={name:fingerprint(path) for name,path in oracles.items()}
cases=[]
def case(name,command,args,stdin=b'',output='pipe'):
    cases.append((name,command,args,stdin,output))
for command in ('bc','dc'):
    for option in ('help','version'):
        case(command+'-'+option,command,['--'+option])
        case(command+'-'+option+'-full',command,['--'+option],output='full')
    case(command+'-unknown',command,['--not-an-option'])
    case(command+'-missing',command,['absent'])
    case(command+'-directory',command,['tree'])
    case(command+'-empty',command,[],b'')
case('bc-arithmetic','bc',[],b'2+3*4\n2^80\nscale=8;22/7\n')
case('bc-negative','bc',[],b'-17%5\n(-3)^5\nscale=3; -22/7\n')
case('bc-base','bc',[],b'obase=16;255\nibase=16;FF\n')
case('bc-math','bc',['-l'],b'scale=12;s(1);c(1);a(1);e(1);l(2);j(0,1)\n')
case('bc-loop','bc',[],b'for(i=1;i<=8;i++) i*i\n')
case('bc-function','bc',[],b'define f(x) { return (x*x+1) }\nf(12)\n')
case('bc-array','bc',[],b'a[1]=17;a[2]=23;a[1]+a[2]\n')
case('bc-compile','bc',['-c'],b'2+3\n')
case('bc-quiet','bc',['-q'],b'42\n')
case('bc-file','bc',['program.bc'])
case('bc-multiple-files','bc',['program.bc','program.bc'],b'7\n')
case('bc-file-quit','bc',['quit.bc'])
case('bc-runtime-error','bc',[],b'1/0\n5\n')
case('bc-syntax-error','bc',[],b'1+\n')
case('bc-full','bc',[],b'1+1\n',output='full')
case('dc-arithmetic','dc',['-e','2 3 4 * + p 2 80 ^ p 8k 22 7 / p'])
case('dc-negative','dc',['-e','_17 5 % p _3 5 ^ p'])
case('dc-base','dc',['-e','16o 255p 16i FFp'])
case('dc-stack','dc',['-e','1 2 3 f z p c 7 d * p'])
case('dc-register','dc',['-e','17sa 23sb la lb + p'])
case('dc-array','dc',['-e','17 1:a 23 2:a 1;a 2;a + p'])
case('dc-macro','dc',['-e','[2 3 + p]sa lax'])
case('dc-string','dc',['-e','[hello]P'])
case('dc-root','dc',['-e','12k 2 v p'])
case('dc-file','dc',['-f','program.dc'])
case('dc-stdin','dc',[],b'2 3 + p\n')
case('dc-runtime-error','dc',['-e','1 0 / p'])
case('dc-full','dc',['-e','2p'],output='full')
selected=set(profile.options.commands);assert selected<={r[0] for r in cases}
driver_sha256=fingerprint(Path(__file__));results=[]
for index,(name,command,args,stdin,output) in enumerate(cases):
    if selected and name not in selected:continue
    outcomes={}
    for implementation in ('gnu','rboxc'):
        for instrument in (False,True):
            key=implementation+('-valgrind' if instrument else '')
            with tempfile.TemporaryDirectory(prefix='rboxc-bc-behavior-') as directory:
                work=Path(directory)
                for sub in ('exec','memory','tree'):(work/sub).mkdir()
                (work/'exec'/command).symlink_to(oracles[command] if implementation=='gnu' else profile.binary)
                for filename,contents in {'program.bc':b'2+3\n', 'program.dc':b'2 3 + p\n', 'quit.bc':b'42\nquit\n'}.items():
                    p=work/filename;p.write_bytes(contents);p.chmod(0o640);os.utime(p,ns=(946684800000000000,946684800000000000))
                invocation=[str(work/'exec'/command),*args]
                if instrument:invocation=['/usr/bin/valgrind','--leak-check=full','--show-leak-kinds=all','--track-fds=yes','--trace-children=yes','--log-file='+str(work/'memory/%p.log'),*invocation]
                started=time.time_ns()
                with open('/dev/full' if output=='full' else os.devnull,'wb') as sink:
                    done=subprocess.run(invocation,cwd=work,input=stdin,env={'PATH':str(work/'exec')+':/usr/bin:/bin','HOME':directory,'LC_ALL':'C','LANGUAGE':'C','TZ':'UTC0'},stdout=sink if output=='full' else subprocess.PIPE,stderr=subprocess.PIPE,timeout=30)
                finished=time.time_ns()
                def normalize(value):return value.replace(directory.encode(),b'<fixture>').hex()
                tree={}
                for path in sorted(work.rglob('*')):
                    relative=path.relative_to(work)
                    if relative.parts[0] in ('exec','memory'):continue
                    if path.is_symlink():tree[str(relative)]={'symlink':str(path.readlink())}
                    elif path.is_file():
                        st=path.stat();mtime=st.st_mtime_ns
                        tree[str(relative)]={'sha256':fingerprint(path),'bytes':st.st_size,'mode':st.st_mode&0o777,
                                             'mtime':'fixture' if mtime==946684800000000000 else 'current-operation' if started-1000000000<=mtime<=finished+1000000000 else mtime}
                row={'status':done.returncode,'stdout':normalize(done.stdout or b''),'stderr':normalize(done.stderr),'tree':tree,'raw_stdout':(done.stdout or b'').hex(),'raw_stderr':done.stderr.hex()}
                if instrument:
                    saved=profile.logs/f'{index:02}-{key}-memory';shutil.copytree(work/'memory',saved)
                    row['memory']=[{**runner.parse_memory_log(p.read_text(),p.stem,exec_only=True),'log':str(p.relative_to(ROOT)),'sha256':fingerprint(p)} for p in sorted(saved.glob('*.log'))]
                outcomes[key]=row
    equivalent=all(all(r[f]==outcomes['gnu'][f] for f in ('status','stdout','stderr','tree')) for r in outcomes.values())
    logs=outcomes['rboxc-valgrind']['memory']
    clean=bool(logs) and all(m['complete_exec_log'] and m['errors']==0 and m['non_inherited_descriptors']==0 and not any(m['heap_bytes'].get(k,0) for k in ('definitely lost','indirectly lost','possibly lost')) for m in logs)
    results.append({'name':name,'command':command,'arguments':args,'stdin':stdin.hex(),'output':output,'pass':equivalent and clean,'equivalent':equivalent,'memory_clean':clean,'outcomes':outcomes})
    print('PASS' if results[-1]['pass'] else 'OPEN',name,flush=True)
    report={'scope':'Ordinary bounded BC/DC arithmetic, calculator state, private file input, and diagnostics.',**profile.metadata(),'driver_sha256':driver_sha256,'normalization':'Replace private fixture directory; classify file mtimes as preserved fixture time or within the current operation interval (one-second filesystem tolerance). Compare bytes, modes, symlinks, streams, and statuses.','passed':sum(r['pass'] for r in results),'total':len(results),'results':results}
    report['gnu_binaries']={n:{'path':str(p),'sha256':hashes[n]} for n,p in oracles.items()}
    assert all(fingerprint(p)==hashes[n] for n,p in oracles.items())
    assert fingerprint(Path(__file__))==driver_sha256
    profile.report.write_text(json.dumps(report,indent=2)+'\n')
raise SystemExit(any(not r['pass'] for r in results))
