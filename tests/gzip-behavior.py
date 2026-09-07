#!/usr/bin/env python3
"""Compare ordinary compression, decompression, aliases, and local file behavior."""
# SPDX-License-Identifier: GPL-3.0-or-later
import importlib.util
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
from comparison_profile import ComparisonProfile, fingerprint

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0,str(ROOT/'tests/gnu'))
spec=importlib.util.spec_from_file_location('reviewed',ROOT/'tests/gnu/reviewed-original.py')
runner=importlib.util.module_from_spec(spec);spec.loader.exec_module(runner)
profile=ComparisonProfile('gzip-behavior',oracle=ROOT/'build/gnu-gzip/gzip',selections=True)
commands=('gzip','gunzip','uncompress','zcat')
oracles={name:ROOT/'build/gnu-gzip'/name for name in commands}
hashes={name:fingerprint(path) for name,path in oracles.items()}
data=b'alpha beta gamma\n'*1000+b'0123456789\n'
compressed=subprocess.run([str(oracles['gzip']),'-nc'],input=data,capture_output=True,check=True).stdout
cases=[]
def case(name,args,command='gzip',stdin=b'',output='pipe'):
    cases.append((name,command,args,stdin,output))
for command in commands:
    case(command+'-help',['--help'],command)
    case(command+'-version',['--version'],command)
    case(command+'-missing',['absent'],command)
    case(command+'-option-error',['--unknown-option'],command)
    if command!='gzip':
        case(command+'-stdout',['-c','input.gz'],command)
        case(command+'-stdin',[],command,compressed)
        case(command+'-full-help',['--help'],command,output='full')
for name,args in [
    ('compress',['-nc','input']),('compress-fast',['-1nc','input']),('compress-best',['-9nc','input']),
    ('decompress',['-dc','input.gz']),('test',['-t','input.gz']),('list',['-l','input.gz']),
    ('verbose',['-nv','-c','input']),('empty',['-nc','empty']),('keep',['-nk','input']),
    ('in-place',['-n','input']),('decode-in-place',['-d','input.gz']),
    ('recursive',['-nr','tree']),('suffix',['-nS','.packed','input']),
    ('concatenated',['-dc','concatenated.gz']),('full-output',['-nc','input']),
    ('full-decompress',['-dc','input.gz']),('directory',['tree']),('several-files',['-nc','input','other']),
    ('keep-decode',['-dk','input.gz']),('named-header',['-c','input']),('rsyncable',['--rsyncable','-nc','input']),
]:case(name,args,output='full' if name in ('full-output','full-decompress') else 'pipe')
case('compress-stdin',['-nc'],stdin=data)
selected=set(profile.options.commands);assert selected<={r[0] for r in cases}
results=[]
for index,(name,command,args,stdin,output) in enumerate(cases):
    if selected and name not in selected:continue
    outcomes={}
    for implementation in ('gnu','rboxc'):
        for instrument in (False,True):
            key=implementation+('-valgrind' if instrument else '')
            with tempfile.TemporaryDirectory(prefix='rboxc-gzip-behavior-') as directory:
                work=Path(directory)
                for sub in ('exec','memory','tree'):(work/sub).mkdir()
                for entry in commands:(work/'exec'/entry).symlink_to(oracles[entry] if implementation=='gnu' else profile.binary)
                fixtures={'input':data,'other':b'other input\n','empty':b'', 'input.gz':compressed,
                          'concatenated.gz':compressed+compressed,'tree/first':data,'tree/second':b'another file\n'}
                # Output destinations for in-place cases must initially be absent.
                if name in ('keep','in-place'):del fixtures['input.gz']
                if name in ('decode-in-place','keep-decode'):del fixtures['input']
                for filename,contents in fixtures.items():
                    path=work/filename;path.write_bytes(contents);path.chmod(0o640);os.utime(path,(946684800,946684800))
                invocation=[str(work/'exec'/command),*args]
                if instrument:
                    if implementation=='gnu' and command!='gzip':invocation=['/bin/sh',*invocation]
                    invocation=['/usr/bin/valgrind','--leak-check=full','--show-leak-kinds=all','--track-fds=yes','--trace-children=yes','--log-file='+str(work/'memory/%p.log'),*invocation]
                with open('/dev/full' if output=='full' else os.devnull,'wb') as sink:
                    done=subprocess.run(invocation,cwd=work,input=stdin,env={'PATH':str(work/'exec')+':/usr/bin:/bin','HOME':directory,'LC_ALL':'C','TZ':'UTC0'},stdout=sink if output=='full' else subprocess.PIPE,stderr=subprocess.PIPE,timeout=30)
                def normalize(value):return value.replace(directory.encode(),b'<fixture>').hex()
                tree={str(p.relative_to(work)):{'sha256':fingerprint(p),'bytes':p.stat().st_size,'mode':p.stat().st_mode&0o777,'mtime_ns':p.stat().st_mtime_ns} for p in sorted(work.rglob('*')) if p.is_file() and p.relative_to(work).parts[0] not in ('exec','memory')}
                row={'status':done.returncode,'stdout':normalize(done.stdout or b''),'stderr':normalize(done.stderr),'tree':tree,'raw_stdout':(done.stdout or b'').hex(),'raw_stderr':done.stderr.hex()}
                if instrument:
                    saved=profile.logs/f'{index:02}-{key}-memory';shutil.copytree(work/'memory',saved)
                    row['memory']=[{**runner.parse_memory_log(p.read_text(),p.stem,exec_only=True),'log':str(p.relative_to(ROOT)),'sha256':fingerprint(p)} for p in sorted(saved.glob('*.log'))]
                outcomes[key]=row
    fields=('status','stdout','stderr','tree')
    equivalent=all(all(r[f]==outcomes['gnu'][f] for f in fields) for r in outcomes.values())
    memory=outcomes['rboxc-valgrind']['memory']
    clean=bool(memory) and all(m['complete_exec_log'] and m['errors']==0 and m['non_inherited_descriptors']==0 and not any(m['heap_bytes'].get(k,0) for k in ('definitely lost','indirectly lost','possibly lost')) for m in memory)
    results.append({'name':name,'command':command,'arguments':args,'stdin':stdin.hex(),'output':output,'pass':equivalent and clean,'equivalent':equivalent,'memory_clean':clean,'outcomes':outcomes})
    print('PASS' if results[-1]['pass'] else 'OPEN',name,flush=True)
    report={'scope':'Ordinary bounded valid compression/decompression, aliases, local metadata, and I/O errors; historical vulnerability reproductions are not executed.',**profile.metadata(),'gnu_binaries':{n:{'path':str(p),'sha256':hashes[n]} for n,p in oracles.items()},'driver_sha256':fingerprint(Path(__file__)),'passed':sum(r['pass'] for r in results),'total':len(results),'results':results}
    assert all(fingerprint(p)==hashes[n] for n,p in oracles.items())
    profile.report.write_text(json.dumps(report,indent=2)+'\n')
raise SystemExit(any(not r['pass'] for r in results))
