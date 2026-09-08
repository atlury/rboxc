#!/usr/bin/env python3
"""Compare GNU Findutils traversal, argument construction, database lookup, and diagnostics."""
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
profile=ComparisonProfile('findutils-behavior',oracle=ROOT/'build/gnu-findutils/find/find',selections=True)
oracles={name:ROOT/'build/gnu-findutils'/name/name for name in ('find','xargs','locate')}
frcode=ROOT/'build/gnu-findutils/locate/frcode';frcode_hash=fingerprint(frcode)
core=ROOT/'build/gnu-coreutils/src/coreutils';core_hash=fingerprint(core)
hashes={name:fingerprint(path) for name,path in oracles.items()}
cases=[]
def case(name,command,args,stdin=b'',output='pipe'):
    cases.append((name,command,args,stdin,output))
for command in ('find','xargs','locate'):
    for option in ('help','version'):
        case(command+'-'+option,command,['--'+option])
        case(command+'-'+option+'-full',command,['--'+option],output='full')
    case(command+'-unknown',command,['--not-an-option'])
for name,args in [
    ('walk',['tree']),('depth',['tree','-depth']),('maxdepth',['tree','-maxdepth','1']),
    ('files',['tree','-type','f']),('directories',['tree','-type','d']),('name',['tree','-name','*.txt']),
    ('iname',['tree','-iname','*.TXT']),('path',['tree','-path','tree/sub/*']),
    ('boolean',['tree','(','-name','*.txt','-o','-name','*.log',')','-type','f']),
    ('negate',['tree','!','-name','*.txt']),('size',['tree','-type','f','-size','6c']),
    ('permissions',['tree','-perm','0640']),('empty',['tree','-empty']),
    ('printf',['tree','-printf','%y %m %p\\n']),('null',['tree','-type','f','-print0']),
    ('follow',['-L','tree','-type','f']),('samefile',['tree','-samefile','tree/a.txt']),
    ('prune',['tree','-path','tree/sub','-prune','-o','-print']),
    ('missing',['absent']),('invalid-predicate',['tree','-not-a-predicate']),
    ('files0',['-files0-from','paths0','-type','f']),
    ('delete',['tree','-name','*.txt','-delete','-print']),
    ('exec',['tree','-type','f','-exec','/bin/sh','-c','echo "$1"','sh','{}',';']),
    ('execdir',['tree','-type','f','-execdir','/bin/sh','-c','echo "$1"','sh','{}',';']),
    ('execdir-batch',['tree','-type','f','-execdir','echo','{}','+']),
    ('samefile-twice',['tree','-samefile','tree/a.txt','-o','-samefile','tree/b.log']),
    ('samefile-parse-error',['tree','-samefile','tree/a.txt','-not-a-predicate']),
    ('execdir-root',['/','-maxdepth','0','-execdir','echo','==','{}','+']),
    ('fprint-parse-error',['tree','-fprint','output','-not-a-predicate']),
]:case('find-'+name,'find',args)
for name,args,data in [
    ('default',[],b'alpha beta\n'),('quotes',[],b'"two words" \'three words\' four\\ five\n'),
    ('empty',[],b''),('no-empty',['-r'],b''),('null',['-0'],b'a b\x00c\nd\x00'),
    ('delimiter',['-d',':'],b'a:b c:d:'),('maxargs',['-n','2'],b'a b c d e\n'),
    ('lines',['-L','1'],b'a b\nc d\n'),('replace',['-I','{}','echo','prefix-{}-suffix'],b'one\ntwo words\n'),
    ('file',['-a','arguments'],b''),('missing',['-a','absent'],b''),
    ('bad-quote',[],b'"unterminated\n'),('bad-limit',['-n','0'],b'a\n'),
    ('missing-command',['missing-program'],b'a\n'),
    ('child-failure',['/bin/sh','-c','exit 1'],b'a\n'),
    ('parallel',['-P','2','-n','1','/bin/sh','-c','exit 0','sh'],b'a b c d\n'),
    ('file-missing-command',['-a','arguments','missing-program'],b''),
    ('replace-file',['-a','arguments','-I','{}','echo','{}'],b''),
    ('replace-empty',['-I','{}','echo','{}'],b''),
]:case('xargs-'+name,'xargs',args,data)
for name,args in [
    ('match',['txt']),('glob',['*.txt']),('basename',['-b','*.txt']),
    ('ignore-case',['-i','A.TXT']),('null',['-0','txt']),('limit',['-l','1','txt']),
    ('regex',['--regex','^tree/.*\\.txt$']),('count',['-c','txt']),
    ('existing',['-e','txt']),('nonexisting',['-E','txt']),('statistics',['-S']),
    ('missing-database',['-d','absent','txt']),('no-match',['never-matches']),
    ('two-databases',['-d','database:database','txt']),
    ('two-databases-regex',['-d','database:database','-r','txt$']),
    ('missing-second-database',['-d','database:absent','txt']),
    ('invalid-regex',['-r','[']),
    ('multiple-patterns',['-A','tree','txt']),
]:case('locate-'+name,'locate',args)
case('find-walk-full','find',['tree'],output='full')
case('locate-search-full','locate',['txt'],output='full')
selected=set(profile.options.commands);assert selected<={r[0] for r in cases}
driver_sha256=fingerprint(Path(__file__));results=[]
for index,(name,command,args,stdin,output) in enumerate(cases):
    if selected and name not in selected:continue
    outcomes={}
    for implementation in ('gnu','rboxc'):
        for instrument in (False,True):
            key=implementation+('-valgrind' if instrument else '')
            with tempfile.TemporaryDirectory(prefix='rboxc-findutils-behavior-') as directory:
                work=Path(directory)
                for sub in ('exec','memory','deps','tree','tree/sub','tree/emptydir'):(work/sub).mkdir()
                (work/'exec'/command).symlink_to(oracles[command] if implementation=='gnu' else profile.binary)
                (work/'deps/echo').symlink_to(core)
                fixtures={'tree/a.txt':b'alpha\n','tree/b.log':b'beta\n','tree/sub/c.txt':b'gamma\n',
                          'tree/.hidden':b'hidden\n','arguments':b'a b\n"c d"\n',
                          'paths0':b'tree/a.txt\x00tree/sub\x00'}
                database_input=b'tree/a.txt\ntree/absent.txt\ntree/b.log\ntree/sub/c.txt\n'
                fixtures['database']=subprocess.check_output([frcode],input=database_input,env={'LC_ALL':'C'})
                for filename,contents in fixtures.items():
                    p=work/filename;p.write_bytes(contents);p.chmod(0o640);os.utime(p,ns=(946684800000000000,946684800000000000))
                for fixture_dir in ('tree','tree/sub','tree/emptydir'):(work/fixture_dir).chmod(0o750)
                (work/'tree/link').symlink_to('a.txt');(work/'tree/dangling').symlink_to('absent')
                invocation=[str(work/'exec'/command),*args]
                if instrument:invocation=['/usr/bin/valgrind','--leak-check=full','--show-leak-kinds=all','--track-fds=yes','--trace-children=yes','--log-file='+str(work/'memory/%p.log'),*invocation]
                started=time.time_ns()
                with open('/dev/full' if output=='full' else os.devnull,'wb') as sink:
                    done=subprocess.run(invocation,cwd=work,input=stdin,env={'PATH':str(work/'exec')+':'+str(work/'deps')+':/usr/bin:/bin','HOME':directory,'LOCATE_PATH':'database','LC_ALL':'C','LANGUAGE':'C','TZ':'UTC0'},stdout=sink if output=='full' else subprocess.PIPE,stderr=subprocess.PIPE,timeout=30)
                finished=time.time_ns()
                def normalize(value):return value.replace(directory.encode(),b'<fixture>').hex()
                tree={}
                for path in sorted(work.rglob('*')):
                    relative=path.relative_to(work)
                    if relative.parts[0] in ('exec','memory','deps'):continue
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
    report={'scope':'Bounded GNU Findutils comparisons in private trees, including command batching and a database generated by the pinned native frcode fixture helper. The encoder is not counted as a port.',**profile.metadata(),'driver_sha256':driver_sha256,'normalization':'Replace private fixture directory; classify file mtimes as preserved fixture time or within the current operation interval (one-second filesystem tolerance). Compare bytes, modes, symlinks, streams, and statuses.','passed':sum(r['pass'] for r in results),'total':len(results),'results':results}
    assert fingerprint(frcode)==frcode_hash and fingerprint(core)==core_hash
    report['prerequisites']={'frcode':{'path':str(frcode),'sha256':frcode_hash},'echo':{'path':str(core),'sha256':core_hash}}
    report['gnu_binaries']={n:{'path':str(p),'sha256':hashes[n]} for n,p in oracles.items()}
    assert all(fingerprint(p)==hashes[n] for n,p in oracles.items())
    assert fingerprint(Path(__file__))==driver_sha256
    profile.report.write_text(json.dumps(report,indent=2)+'\n')
raise SystemExit(any(not r['pass'] for r in results))
