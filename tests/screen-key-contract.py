#!/usr/bin/env python3
"""Validate Screen action replacement and cleanup using its original functions."""
# SPDX-License-Identifier: GPL-3.0-or-later
import importlib.util
import json
from pathlib import Path
import re
import subprocess
import sys
from comparison_profile import fingerprint

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0,str(ROOT/'tests/gnu'))
spec=importlib.util.spec_from_file_location('reviewed',ROOT/'tests/gnu/reviewed-original.py')
runner=importlib.util.module_from_spec(spec)
spec.loader.exec_module(runner)
stage=ROOT/'build/screen-key-contract-strings'
stage.mkdir()
source=Path('/opt/src/screen-5.0.2')
records=[json.loads(p.read_text()) for p in (ROOT/'build/screen-cc-records').glob('*.json')]
inputs={p:fingerprint(p) for p in [Path(__file__),source/'process.c',source/'misc.c',ROOT/'build/screen-key-cleanup/process.c']}
results=[]
body=r'''
#include <assert.h>
char strnomem[] = "Out of memory.";
void Panic(int error, const char *format, ...) { _exit(90); }
int main(int argc, char **argv) {
    int count=atoi(argv[1]);
    assert(count==0 || count==1 || count==3);
    struct action action={0};
    char *words[]={"one", "two", "three", NULL};
    int lengths[]={3,3,5};
    words[count]=NULL;
    for(int iteration=0;iteration<8;iteration++) {
        SaveAction(&action,RC_STUFF,words,lengths);
        assert(action.nr==RC_STUFF);
        if(count==0) assert(action.args==noargs && action.argl==NULL);
        else for(int i=0;i<count;i++) {
            assert(action.argl[i]==lengths[i]);
            assert(memcmp(action.args[i],words[i],lengths[i])==0);
            assert(action.args[i][lengths[i]]=='\0');
        }
        ClearAction(&action);
        assert(action.nr==RC_ILLEGAL && action.args==noargs && action.argl==NULL);
        ClearAction(&action);
        assert(action.nr==RC_ILLEGAL && action.args==noargs && action.argl==NULL);
    }
    puts("action contract passed");
    return 0;
}
'''

def compile_source(original,actual,output):
    rows=[r for r in records if r.get('file')==str(original)]
    assert len(rows)==1
    r=rows[0];args=r['arguments'].copy()
    args[args.index(str(original))]=str(actual)
    args[args.index('-o')+1]=str(output)
    args+=['-ffunction-sections','-fdata-sections','-iquote',str(source)]
    log=output.with_suffix('.build.log')
    with log.open('w') as stream: subprocess.run(args,cwd=r['directory'],stdout=stream,stderr=subprocess.STDOUT,check=True)
    return {'argv':args,'log':str(log.relative_to(ROOT)),'sha256':fingerprint(log)}

builds=[compile_source(source/'misc.c',source/'misc.c',stage/'misc.o')]
for label,original in [('gnu',source/'process.c'),('adapted',ROOT/'build/screen-key-cleanup/process.c')]:
    harness=stage/(label+'.c')
    harness.write_text('#include "'+str(original)+'"\n'+body)
    inputs[harness]=fingerprint(harness)
    builds.append(compile_source(source/'process.c',harness,stage/(label+'.o')))
    binary=stage/label
    subprocess.run(['gcc',str(stage/(label+'.o')),str(stage/'misc.o'),'-Wl,--gc-sections','-o',str(binary)],check=True)
    for count in [0,1,3]:
        log=stage/(label+'-'+str(count)+'.log')
        done=subprocess.run(['/usr/bin/valgrind','--leak-check=full','--show-leak-kinds=all',
            '--track-fds=yes','--log-file='+str(log),str(binary),str(count)],capture_output=True,timeout=30)
        assert done.returncode==0 and done.stdout==b'action contract passed\n' and not done.stderr
        text=log.read_text();pids=set(re.findall(r'^==([0-9]+)==',text,re.M));assert len(pids)==1
        memory=runner.parse_memory_log(text,pids.pop(),exec_only=True)
        assert memory['complete_exec_log'] and memory['non_inherited_descriptors']==0
        if label=='adapted':
            assert memory['errors']==0 and not any(memory['heap_bytes'].get(k,0) for k in ['definitely lost','indirectly lost','possibly lost'])
        else:
            assert memory['heap_bytes'].get('definitely lost',0)==8*count*4
        results.append({'implementation':label,'argument_count':count,'iterations':8,'pass':True,
            'binary':str(binary.relative_to(ROOT)),'binary_sha256':fingerprint(binary),
            'log':str(log.relative_to(ROOT)),'log_sha256':fingerprint(log),'memory':memory})
assert all(fingerprint(p)==expected for p,expected in inputs.items())
report={'scope':'Six original/adapted contract outcomes cover empty, one-argument and three-argument bindings '
    'with explicit ordinary string lengths, eight replacements and repeated cleanup. '
    'Actual GNU SaveAction, ClearAction and SaveStrn functions are used. The native GNU length-array '
    'leaks are retained; all three adapted Valgrind processes must be clean.',
    'inputs':{str(p):v for p,v in inputs.items()},'builds':builds,'passed':6,'total':6,'results':results}
(ROOT/'evidence/screen-key-contract.json').write_text(json.dumps(report,indent=2)+'\n')
print('Screen action ownership: six contract outcomes, three clean adapted processes')
