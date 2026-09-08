#!/usr/bin/env python3
"""Run GNU Wget's separate TESTING helper profile; no multicall entry coverage."""
# SPDX-License-Identifier: GPL-3.0-or-later
import argparse
import importlib.util
import json
import os
from pathlib import Path
import re
import shlex
import subprocess
import sys
import tempfile
from comparison_profile import fingerprint

ROOT=Path(__file__).resolve().parents[1]
sys.path.insert(0,str(ROOT/'tests/gnu'))
spec=importlib.util.spec_from_file_location('reviewed',ROOT/'tests/gnu/reviewed-original.py')
runner=importlib.util.module_from_spec(spec);spec.loader.exec_module(runner)
parser=argparse.ArgumentParser(description=__doc__)
parser.add_argument('--report-name',required=True)
args=parser.parse_args();assert re.fullmatch(r'[a-z0-9-]+',args.report_name)
report=ROOT/'evidence'/(args.report_name+'.json');assert not report.exists()
logs=ROOT/'evidence/raw'/args.report_name;logs.mkdir()
build=ROOT/'build/gnu-wget';source=Path(json.loads((ROOT/'inventory/sources.json').read_text())['wget']['source'])
cleanup=json.loads((ROOT/'evidence/wget-native-cleanup.json').read_text())
adapted=ROOT/'build/wget-cleanup/http.c'
assert fingerprint(adapted)==cleanup['adapted_source_sha256']
assert fingerprint(source/'src/http.c')==cleanup['original_sha256']
inputs={p:fingerprint(p) for p in [Path(__file__),ROOT/'evidence/wget-native-cleanup.json',adapted,build/'src/wget',ROOT/'target/release/rboxc',ROOT/'target/patch-merge-initialization-candidate/release/rboxc',build/'tests/Makefile',build/'src/Makefile',build/'src/config.h',build/'lib/libgnu.a',source/'tests/unit-tests.c',source/'tests/unit-tests.h',Path('/usr/bin/gcc').resolve(),Path('/usr/bin/make'),Path('/usr/bin/valgrind')]}
for folder in ('src','lib'):
    for p in (source/folder).iterdir():
        if p.suffix in ('.c','.h'):inputs[p]=fingerprint(p)
commands=[]
def execute(command,cwd,log):
    commands.append({'arguments':command,'directory':str(cwd),'log':str(log.relative_to(ROOT))})
    with log.open('w') as out:subprocess.run(command,cwd=cwd,stdout=out,stderr=subprocess.STDOUT,check=True)
execute(['/usr/bin/make','-j4','V=1','unit-tests'],build/'tests',logs/'build.log')
# Use GNU's exact TESTING compilation flags with the already recorded production
# HTTP ownership adapter. No command implementation is translated by this test.
plan=subprocess.check_output(['/usr/bin/make','-n','-W',str(source/'src/http.c'),'libunittest_a-http.o'],cwd=build/'src',text=True)
lines=[line for line in plan.splitlines() if ' -c -o libunittest_a-http.o ' in line]
assert len(lines)==1
compile_line=lines[0].split(';')[-1]
source_expression="`test -f 'http.c' || echo '"+str(source/'src')+"/'`http.c"
assert compile_line.count(source_expression)==1
command=shlex.split(compile_line.replace(source_expression,shlex.quote(str(source/'src/http.c'))))
assert command[0]=='gcc' and '-DTESTING' in command and command[-1]==str(source/'src/http.c')
command[0]='/usr/bin/gcc';command[-1]=str(adapted)
command[command.index('-o')+1]=str(logs/'http.o')
command[command.index('-MF')+1]=str(logs/'http.d')
command+=['-iquote'+str(source/'src')]
execute(command,build/'src',logs/'adapted-build.log')
link_plan=subprocess.check_output(['/usr/bin/make','-n','-W','unit-tests.o','unit-tests'],cwd=build/'tests',text=True)
links=[line for line in link_plan.splitlines() if ' -o unit-tests unit-tests.o ' in line]
assert len(links)==1
link=shlex.split(links[0].split(';')[-1]);assert link[0]=='gcc'
link[0]='/usr/bin/gcc';link[link.index('-o')+1]=str(logs/'adapted-unit-tests')
link.insert(link.index('../src/libunittest.a'),str(logs/'http.o'))
execute(link,build/'tests',logs/'adapted-link.log')
for p in [build/'tests/unit-tests',build/'tests/unit-tests.o',build/'src/libunittest.a',logs/'http.o',logs/'adapted-unit-tests']:inputs[p]=fingerprint(p)
results=[]
for name,binary in [('gnu',build/'tests/unit-tests'),('adapted-helpers',logs/'adapted-unit-tests')]:
    for instrument in (False,True):
        key=name+('-valgrind' if instrument else '')
        memory=logs/(key+'-memory.log')
        command=[str(binary)]
        if instrument:command=['/usr/bin/valgrind','--leak-check=full','--show-leak-kinds=all','--track-fds=yes','--trace-children=yes','--log-file='+str(memory),*command]
        with tempfile.TemporaryDirectory(prefix='rboxc-wget-unit-') as directory:
            env={'PATH':'/usr/bin:/bin','HOME':directory,'TMPDIR':directory,'LC_ALL':'C','LANGUAGE':'C','TZ':'UTC0'}
            done=subprocess.run(command,cwd=directory,env=env,stdin=subprocess.DEVNULL,stdout=subprocess.PIPE,stderr=subprocess.STDOUT,timeout=90)
        output=logs/(key+'.log');output.write_bytes(done.stdout)
        tests=re.findall(rb'^RUNNING TEST (test_[a-z_]+)\.\.\.$',done.stdout,re.M)
        passed=done.returncode==0 and len(tests)==17 and done.stdout.count(b'PASSED\n')==18 and b'ALL TESTS PASSED\nTests run: 17\n' in done.stdout
        row={'profile':key,'binary':str(binary),'status':done.returncode,'pass':passed,'tests':[n.decode() for n in tests],'output':str(output.relative_to(ROOT)),'output_sha256':fingerprint(output)}
        if instrument:
            text=memory.read_text();pids=set(re.findall(r'^==([0-9]+)==',text,re.M));assert len(pids)==1
            row.update(memory=runner.parse_memory_log(text,pids.pop(),exec_only=True),memory_log=str(memory.relative_to(ROOT)),memory_log_sha256=fingerprint(memory))
        results.append(row);print(key,'PASS' if passed else 'OPEN',flush=True)
assert all(fingerprint(p)==h for p,h in inputs.items())
report.write_text(json.dumps({'scope':'Original 17 GNU Wget TESTING helper unit functions run unchanged against GNU helper objects and the same helpers with the recorded production HTTP ownership adaptation. These are separate native C test executables, not the multicall candidate and not Rust entry coverage. Test-only configuration and all memory findings are retained separately. No suppressions or fixture edits.', 'inputs':{str(p):h for p,h in inputs.items()},'commands':commands,'raw':{str(p.relative_to(ROOT)):fingerprint(p) for p in logs.iterdir() if p.suffix=='.log'},'total':len(results),'passed':sum(r['pass'] for r in results),'tests_per_profile':17,'results':results},indent=2)+'\n')
raise SystemExit(not all(r['pass'] for r in results))
