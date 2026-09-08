#!/usr/bin/env python3
"""Compare ordinary Ed editing, addressing, streams, and private file operations."""
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
profile=ComparisonProfile('ed-behavior',oracle=ROOT/'build/gnu-ed/ed',selections=True)
data=b'alpha one\nbeta two\nalpha three\nlast\n'
cases=[]
def case(name,args,stdin=b'',output='pipe',locale='C'):
    cases.append((name,args,stdin,output,locale))
for name,args in [('help',['--help']),('version',['--version']),('unknown',['--not-an-option']),('missing',['absent']),('directory',['tree']),('start-line',['-s','+2','input']),('start-search',['-s','+/beta','input']),('start-reverse',['-s','+?alpha','input']),('start-invalid',['-s','+0','input'])]:
    case(name,args,stdin=b'p\nQ\n')
for name,commands in [
    ('print',b'1,$p\nQ\n'),('number',b'1,$n\nQ\n'),('list',b'1,$l\nQ\n'),
    ('append',b'a\nnew\n.\n1,$p\nQ\n'),('insert',b'1i\nfirst\n.\n1,$p\nQ\n'),
    ('change',b'2c\nchanged\n.\n1,$p\nQ\n'),('delete',b'2d\n1,$p\nQ\n'),
    ('join',b'1,2j\n1,$p\nQ\n'),('move',b'1m$\n1,$p\nQ\n'),('copy',b'1t$\n1,$p\nQ\n'),
    ('substitute',b'1,$s/alpha/A/g\n1,$p\nQ\n'),('undo',b'2d\nu\n1,$p\nQ\n'),
    ('mark',b'2ka\n1p\n\'ap\nQ\n'),('global',b'g/alpha/p\nQ\n'),('inverse',b'v/alpha/p\nQ\n'),
    ('write',b'w result\nq\n'),('append-file',b'W other\nQ\n'),('read',b'r other\n1,$p\nQ\n'),
    ('edit',b'e other\n1,$p\nQ\n'),('filename',b'f result\nf\nQ\n'),
    ('empty-pattern-reuse',b'/alpha/p\ns//A/\np\nQ\n'),
    ('invalid-regex',b's/[//\nQ\n'),('bad-address',b'99p\nQ\n'),
    ('modified-quit',b'1d\nq\nq\n'),('modified-eof',b'1d\n'),
]:case(name,['-s','input'],stdin=commands)
case('empty',['-s','empty'],stdin=b'=\nQ\n')
case('binary',['-s','binary'],stdin=b'1,$l\nQ\n')
case('unicode',['-s','unicode'],stdin='s/é/É/g\np\nQ\n'.encode(),locale='C.UTF-8')
case('extended',['-E','-s','input'],stdin=b'1,$s/(alpha|beta)/X/g\n1,$p\nQ\n')
case('prompt',['-p','ed> ','input'],stdin=b'p\nQ\n')
case('restricted-file',['-r','-s','tree/file'],stdin=b'Q\n')
case('restricted-shell',['-r','-s','input'],stdin=b'!echo hello\nQ\n')
case('help-full',['--help'],output='full')
case('version-full',['--version'],output='full')
case('full-output',['-s','input'],stdin=b'1,$p\nQ\n',output='full')
selected=set(profile.options.commands);assert selected<={r[0] for r in cases}
driver_sha256=fingerprint(Path(__file__));results=[]
for index,(name,args,stdin,output,locale) in enumerate(cases):
    if selected and name not in selected:continue
    outcomes={}
    for implementation in ('gnu','rboxc'):
        for instrument in (False,True):
            key=implementation+('-valgrind' if instrument else '')
            with tempfile.TemporaryDirectory(prefix='rboxc-ed-behavior-') as directory:
                work=Path(directory)
                for sub in ('exec','memory','tree'):(work/sub).mkdir()
                (work/'exec/ed').symlink_to(profile.oracle if implementation=='gnu' else profile.binary)
                for filename,contents in {'input':data,'other':b'other\nend\n','empty':b'',
                                          'script':b's/alpha/A/\n','unicode':'école\n'.encode(),'binary':b'alpha\0beta\0'}.items():
                    p=work/filename;p.write_bytes(contents);p.chmod(0o640);os.utime(p,ns=(946684800000000000,946684800000000000))
                (work/'link').symlink_to('input')
                invocation=[str(work/'exec/ed'),*args]
                if instrument:invocation=['/usr/bin/valgrind','--leak-check=full','--show-leak-kinds=all','--track-fds=yes','--trace-children=yes','--log-file='+str(work/'memory/%p.log'),*invocation]
                started=time.time_ns()
                with open('/dev/full' if output=='full' else os.devnull,'wb') as sink:
                    done=subprocess.run(invocation,cwd=work,input=stdin,env={'PATH':str(work/'exec')+':/usr/bin:/bin','HOME':directory,'LC_ALL':locale,'LANGUAGE':'C','TZ':'UTC0'},stdout=sink if output=='full' else subprocess.PIPE,stderr=subprocess.PIPE,timeout=30)
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
    results.append({'name':name,'command':'sed','arguments':args,'stdin':stdin.hex(),'output':output,'locale':locale,'pass':equivalent and clean,'equivalent':equivalent,'memory_clean':clean,'outcomes':outcomes})
    print('PASS' if results[-1]['pass'] else 'OPEN',name,flush=True)
    report={'scope':'Ordinary bounded editing, input/output, private file operations, and option diagnostics. No historical vulnerability reproductions.',**profile.metadata(),'driver_sha256':driver_sha256,'normalization':'Replace private fixture directory; classify file mtimes as preserved fixture time or within the current operation interval (one-second filesystem tolerance). Compare bytes, modes, symlinks, streams, and statuses.','passed':sum(r['pass'] for r in results),'total':len(results),'results':results}
    assert fingerprint(Path(__file__))==driver_sha256
    profile.report.write_text(json.dumps(report,indent=2)+'\n')
raise SystemExit(any(not r['pass'] for r in results))
