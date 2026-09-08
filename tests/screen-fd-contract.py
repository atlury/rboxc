#!/usr/bin/env python3
"""Compare Screen's descriptor cleanup contract with its GNU implementation."""
# SPDX-License-Identifier: GPL-3.0-or-later
import hashlib,importlib.util,json,re,subprocess,sys
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
digest=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
sys.path.insert(0,str(ROOT/'tests/gnu'))
spec=importlib.util.spec_from_file_location('reviewed',ROOT/'tests/gnu/reviewed-original.py')
runner=importlib.util.module_from_spec(spec);spec.loader.exec_module(runner)
stage=ROOT/'build/screen-fd-contract';stage.mkdir(exist_ok=True)
harness=stage/'contract.c'
harness.write_text(r'''#include <sys/resource.h>
#include <fcntl.h>
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
void closeallfiles(int);
void Panic(int error, const char *format, ...) { _exit(90); }
int main(int argc,char **argv) {
 struct rlimit limit; if(getrlimit(RLIMIT_NOFILE,&limit)) return 91;
 limit.rlim_cur=64; if(setrlimit(RLIMIT_NOFILE,&limit)) return 91;
 int except=atoi(argv[1]);
 for(int i=3;i<=9;i++) { int fd=open("/dev/null",O_RDONLY); if(fd!=i)return 92; }
 if(dup2(3,42)!=42)return 93;
 if(close(4))return 94;
 closeallfiles(except);
 for(int i=0;i<=42;i++) {
  int live=fcntl(i,F_GETFD)>=0;
  int expected=i<3 || (i==except && i!=4);
  if(live!=expected)return 95;
 }
 if(except>=3 && except!=4 && close(except))return 96;
 puts("descriptor contract passed"); return 0;
}
''')
records=[json.loads(p.read_text()) for p in (ROOT/'build/screen-cc-records').glob('*.json')]
source=Path('/opt/src/screen-5.0.2/misc.c');matching=[r for r in records if r.get('file')==str(source)];assert len(matching)==1
record=matching[0];results=[]
for label,csource in [('gnu',source),('adapted',ROOT/'build/screen-cleanup/misc.c')]:
 obj=stage/(label+'.o');args=record['arguments'].copy()
 args[args.index(str(source))]=str(csource);args[args.index('-o')+1]=str(obj)
 args+=['-ffunction-sections','-fdata-sections']
 binary=stage/label
 with (stage/(label+'-build.log')).open('w') as log:
  subprocess.run(args,cwd=record['directory'],stdout=log,stderr=subprocess.STDOUT,check=True)
  subprocess.run(['gcc',str(harness),str(obj),'-Wl,--gc-sections','-o',str(binary)],stdout=log,stderr=subprocess.STDOUT,check=True)
 for keep in [0,3,4,5,42]:
  log=ROOT/f'evidence/raw/screen-fd-contract-{label}-{keep}.log'
  argv=['/usr/bin/valgrind','--leak-check=full','--show-leak-kinds=all','--track-fds=yes','--log-file='+str(log),str(binary),str(keep)]
  done=subprocess.run(argv,capture_output=True,timeout=30);assert done.returncode==0 and done.stdout==b'descriptor contract passed\n'
  text=log.read_text();pids=set(re.findall(r'^==([0-9]+)==',text,re.M));assert len(pids)==1
  memory=runner.parse_memory_log(text,pids.pop(),exec_only=True)
  if label=='adapted':
   assert memory['complete_exec_log'] and memory['errors']==0 and memory['non_inherited_descriptors']==0
   assert not any(memory['heap_bytes'].get(k,0) for k in ('definitely lost','indirectly lost','possibly lost'))
  results.append({'implementation':label,'exception_fd':keep,'pass':True,'memory':memory,'log':str(log.relative_to(ROOT)),'log_sha256':digest(log),'binary_sha256':digest(binary)})
report={'scope':'GNU and adapted closeallfiles preserve standard descriptors and the requested open exception, close other owned descriptors including a higher-numbered descriptor, and handle an already-closed exception. GNU probe findings are retained; all five adapted processes must be clean.',
 'driver_sha256':digest(Path(__file__)),'harness_sha256':digest(harness),'original_source_sha256':digest(source),'adapted_source_sha256':digest(ROOT/'build/screen-cleanup/misc.c'),'passed':10,'total':10,'results':results}
(ROOT/'evidence/screen-fd-contract.json').write_text(json.dumps(report,indent=2)+'\n')
print('Screen descriptor contract: ten matching outcomes, five clean adapted processes')
