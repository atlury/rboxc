#!/usr/bin/env python3
"""Compare ordinary Sed editing, addressing, streams, and private file operations."""
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
profile=ComparisonProfile('sed-behavior',oracle=ROOT/'build/gnu-sed/sed/sed',selections=True)
data=b'alpha one\nbeta two\nalpha three\nlast\n'
cases=[]
def case(name,args,stdin=b'',output='pipe',locale='C'):
    cases.append((name,args,stdin,output,locale))
for name,args in [
    ('help',['--help']),('version',['--version']),('missing-script',[]),('option-error',['--unknown-option']),
    ('identity',['','input']),('substitute',['s/alpha/ALPHA/g','input']),
    ('extended',['-E',r's/(alpha) (.*)/\2: \1/','input']),
    ('basic-backreference',[r's/\(alpha\) \(.*\)/\2: \1/','input']),
    ('print',['-n','2,3p','input']),('delete',['/alpha/d','input']),
    ('range',['/beta/,/last/s/ /-/g','input']),('relative-range',['2,+1p','input']),
    ('negated-address',['-n','/alpha/!p','input']),('append',['2a appended','input']),
    ('insert',['2i inserted','input']),('change',['2c changed','input']),
    ('translate',['y/ab/AB/','input']),('number',['=','input']),('list',['l','input']),
    ('hold',['1h;2{G;};3x','input']),('join',['N;s/\n/ + /','input']),
    ('branch',['/alpha/b done;s/ /_/;:done','input']),
    ('test-branch',['s/alpha/ALPHA/;t done;s/ /_/;:done','input']),
    ('quit',['2q','input']),('quit-status',['2q7','input']),('quiet-quit',['2Q5','input']),
    ('script-file',['-f','script','input']),('several-scripts',['-e','s/alpha/A/','-e','s/beta/B/','input']),
    ('missing-file',['p','absent']),('directory',['p','tree']),('missing-script-file',['-f','absent','input']),
    ('script-directory',['-f','tree','input']),('invalid-script',['s/[//','input']),
    ('in-place',['-i','s/alpha/A/','input']),('backup',['-i.bak','s/alpha/A/','input']),
    ('follow-symlink',['--follow-symlinks','-i','s/alpha/A/','link']),
    ('separate',['-s','1s/^/FIRST /','input','other']),('combined',['1s/^/FIRST /','input','other']),
    ('read-file',['2r other','input']),('write-file',['-n','2w result','input']),
    ('sandbox',['--sandbox','s/alpha/A/','input']),('sandbox-read-rejection',['--sandbox','r other','input']),
    ('posix',['--posix','s/alpha/A/','input']),('debug',['--debug','s/alpha/A/','input']),
    ('unbuffered',['-u','s/alpha/A/','input']),('empty',['p','empty']),
    ('null-lines',['-z','s/alpha/A/g','binary']),('full-output',['p','input']),
]:case(name,args,output='full' if name=='full-output' else 'pipe')
case('stdin',['s/alpha/A/'],stdin=data)
case('script-stdin',['-f','-','input'],stdin=b's/alpha/A/\n')
case('unicode',['s/é/É/g','unicode'],locale='C.UTF-8')
case('reuse-regex',[r'/\(alpha\)/s//\1!/', 'input'])
case('regex-error',['s/(/x/', '-E', 'input'])
case('replacement-error',[r's/alpha/\2/', 'input'])
case('case-insensitive',['s/ALPHA/A/Ig', 'input'])
case('multiple-address-regex',['/alpha/,/last/s/[ae]/X/g', 'input'])
selected=set(profile.options.commands);assert selected<={r[0] for r in cases}
driver_sha256=fingerprint(Path(__file__));results=[]
for index,(name,args,stdin,output,locale) in enumerate(cases):
    if selected and name not in selected:continue
    outcomes={}
    for implementation in ('gnu','rboxc'):
        for instrument in (False,True):
            key=implementation+('-valgrind' if instrument else '')
            with tempfile.TemporaryDirectory(prefix='rboxc-sed-behavior-') as directory:
                work=Path(directory)
                for sub in ('exec','memory','tree'):(work/sub).mkdir()
                (work/'exec/sed').symlink_to(profile.oracle if implementation=='gnu' else profile.binary)
                for filename,contents in {'input':data,'other':b'other\nend\n','empty':b'',
                                          'script':b's/alpha/A/\n','unicode':'école\n'.encode(),'binary':b'alpha\0beta\0'}.items():
                    p=work/filename;p.write_bytes(contents);p.chmod(0o640);os.utime(p,ns=(946684800000000000,946684800000000000))
                (work/'link').symlink_to('input')
                invocation=[str(work/'exec/sed'),*args]
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
