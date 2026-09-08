#!/usr/bin/env python3
"""Check parser source cleanup while preserving inherited descriptors."""
# SPDX-License-Identifier: GPL-3.0-or-later
import importlib.util
import json
import os
from pathlib import Path
import re
import subprocess
import sys
import tempfile
from comparison_profile import ComparisonProfile,fingerprint
ROOT=Path(__file__).resolve().parents[1]
sys.path.insert(0,str(ROOT/'tests/gnu'))
spec=importlib.util.spec_from_file_location('reviewed',ROOT/'tests/gnu/reviewed-original.py')
runner=importlib.util.module_from_spec(spec);spec.loader.exec_module(runner)
profile=ComparisonProfile('gawk-source-ownership',oracle=ROOT/'build/gnu-gawk/gawk')
driver_hash=fingerprint(Path(__file__))
cases=[('normal-file',b'BEGIN {print 42}\n',False,False),
       ('fatal-file',b'BEGIN {delete FUNCTAB}\n',False,False),
       ('syntax-file',b'function x(a, b, c , ,) {}\n',False,False),
       ('fatal-source-stdin',b'BEGIN {delete FUNCTAB}\n',True,False),
       ('normal-one-line',b'BEGIN {print 42}\n',False,True),
       ('fatal-one-line',b'BEGIN {delete FUNCTAB}\n',False,True),
       ('syntax-one-line',b'function x(a, b, c , ,) {}\n',False,True)]
results=[]
for name,program,use_stdin,one_line in cases:
    outcomes={}
    for implementation in ('gnu','rboxc'):
        for instrument in (False,True,'descriptors'):
            key=implementation+('-descriptors' if instrument=='descriptors' else '-valgrind' if instrument else '')
            saved=profile.logs/(name+'-'+key);saved.mkdir()
            with tempfile.TemporaryDirectory(prefix='rboxc-gawk-source-') as directory:
                work=Path(directory);(work/'gawk').symlink_to(profile.oracle if implementation=='gnu' else profile.binary)
                (work/'program.awk').write_bytes(program)
                with (work/'inherited-data').open('w+b') as inherited:
                    inherited.write(b'preserve this descriptor\n');inherited.flush()
                    fd=inherited.fileno()
                    argv=[str(work/'gawk'),'-f','-' if use_stdin else 'program.awk']
                    memory=saved/'memory.log'
                    if instrument:argv=['/usr/bin/valgrind','--leak-check=full','--show-leak-kinds=all','--track-fds='+('all' if instrument=='descriptors' else 'yes'),'--trace-children=yes','--log-file='+str(memory),*argv]
                    env={'PATH':'/usr/bin:/bin','HOME':directory,'TMPDIR':directory,'LC_ALL':'C','LANGUAGE':'C','TZ':'UTC0'}
                    if one_line:env['AWKREADFUNC']='1'
                    done=subprocess.run(argv,cwd=work,env=env,input=program if use_stdin else b'',stdout=subprocess.PIPE,stderr=subprocess.PIPE,pass_fds=(fd,),timeout=90)
                (saved/'stdout').write_bytes(done.stdout);(saved/'stderr').write_bytes(done.stderr)
                status={'status':done.returncode,'stdout':done.stdout.decode(),'stderr':done.stderr.decode()}
                outcome={**status,'raw':{str(p.relative_to(ROOT)):fingerprint(p) for p in saved.iterdir()},'inherited_fd':fd}
                if instrument:
                    contents=memory.read_text();pids=set(re.findall(r'^==([0-9]+)==',contents,re.M));assert len(pids)==1
                    parsed=runner.parse_memory_log(contents,pids.pop(),exec_only=True)
                    preserved=bool(re.search(r'Open file descriptor '+str(fd)+r': [^\n]*/inherited-data\n==[0-9]+==\s+<inherited from parent>',contents))
                    stdin_preserved=bool(re.search(r'Open file descriptor 0:',contents))
                    outcome.update(memory=parsed,memory_log=str(memory.relative_to(ROOT)),preserved_inherited=preserved,preserved_stdin=stdin_preserved,
                        memory_clean=parsed['complete_exec_log'] and parsed['errors']==0 and parsed['non_inherited_descriptors']==0 and not any(parsed['heap_bytes'].get(k,0) for k in ('definitely lost','indirectly lost','possibly lost')))
                outcomes[key]=outcome
    reference=outcomes['gnu']
    equivalent=all(all(o[k]==reference[k] for k in ('status','stdout','stderr')) for o in outcomes.values())
    candidate=outcomes['rboxc-valgrind']
    inspection=outcomes['rboxc-descriptors']
    passed=equivalent and candidate['memory_clean'] and inspection['preserved_inherited'] and inspection['preserved_stdin'] and inspection['memory']['errors']==4 and inspection['memory']['non_inherited_descriptors']==0
    results.append({'name':name,'program':program.decode(),'use_stdin':use_stdin,'one_line':one_line,'pass':passed,'equivalent':equivalent,'outcomes':outcomes})
    print('PASS' if passed else 'OPEN',name,flush=True)
assert fingerprint(Path(__file__))==driver_hash
profile.report.write_text(json.dumps({**profile.metadata(),'scope':'Seven normal, fatal and syntax-diagnostic source inputs compare exact status/stdout/stderr with pinned GNU. The debug one-line reader is included. Valgrind checks both parser-owned cleanup and preservation of inherited stdin and an unrelated open data descriptor. No suppressions. Separate track-fds=all inspection intentionally reports the four inherited descriptors as four errors; strict track-fds=yes memory runs must have zero errors.', 'driver_sha256':driver_hash,'passed':sum(r['pass'] for r in results),'total':len(results),'results':results},indent=2)+'\n')
raise SystemExit(not all(r['pass'] for r in results))
