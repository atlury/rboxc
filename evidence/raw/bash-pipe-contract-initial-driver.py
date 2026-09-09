#!/usr/bin/env python3
"""Check Bash pipe ownership when any combination of standard slots is closed."""
# SPDX-License-Identifier: GPL-3.0-or-later
import argparse,hashlib,importlib.util,json,re,subprocess,sys,tempfile
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
sys.path.insert(0,str(ROOT/'tests/gnu'))
spec=importlib.util.spec_from_file_location('reviewed',ROOT/'tests/gnu/reviewed-original.py')
runner=importlib.util.module_from_spec(spec);spec.loader.exec_module(runner)
p=argparse.ArgumentParser(description=__doc__);p.add_argument('--report-name',required=True);o=p.parse_args()
assert re.fullmatch(r'[a-z0-9][a-z0-9-]*',o.report_name)
target=ROOT/'evidence'/(o.report_name+'.json');assert not target.exists()
stage=Path(tempfile.mkdtemp(prefix='bash-pipe-contract-',dir=ROOT/'build'))
harness=stage/'contract.c'
harness.write_text(r'''#include <errno.h>
#include <fcntl.h>
#include <stdlib.h>
#include <unistd.h>
#include <sys/resource.h>
extern int rboxc_bash_owned_pipe(int[2]);
extern int rboxc_bash_owned_close(int);
static int mask;
static void verify(void) {
  for (int fd=0;fd<3;++fd) {
    errno=0;
    int result=fcntl(fd,F_GETFD);
    if ((mask&(1<<fd)) ? (result!=-1 || errno!=EBADF) : result<0) _Exit(91);
  }
}
int main(int argc,char **argv) {
  if (argc!=2) return 80;
  int mode=atoi(argv[1]); mask=mode&7;
  for (int fd=0;fd<3;++fd) if (mask&(1<<fd)) {
    if (close(fd)) return 81;
  }
  if (atexit(verify)) return 82;
  int fds[2]={-9,-9};
  if (mode==16) {
    struct rlimit limit={3,3};
    if (setrlimit(RLIMIT_NOFILE,&limit)) return 83;
    if (rboxc_bash_owned_pipe(fds)!=-1 || errno!=EMFILE || fds[0]!=-9 || fds[1]!=-9) return 84;
    return 0;
  }
  errno=E2BIG;
  if (rboxc_bash_owned_pipe(fds) || errno!=E2BIG) return 85;
  for (int i=0;i<2;++i) {
    /* Half the profiles close their pipe normally; the others exercise exit
       ownership. Inherited standards must stay open in either case. */
    if (fds[i]>=3 || (mode&8)) {
      if (rboxc_bash_owned_close(fds[i])) return 86;
    }
  }
  return 0;
}
''')
source=ROOT/'src/shell_arguments.c';binary=stage/'contract';log=stage/'build.log'
args=['gcc','-O2','-Wall','-Wextra','-Werror',str(harness),str(source),'-o',str(binary)]
with log.open('w') as stream:subprocess.run(args,stdout=stream,stderr=subprocess.STDOUT,check=True)
logs=Path(tempfile.mkdtemp(prefix='bash-pipe-contract-',dir=ROOT/'evidence/raw'));results=[]
for mode in range(17):
    outcomes=[]
    for instrument in (False,True):
        path=logs/(str(mode)+'.log');argv=[str(binary),str(mode)]
        if instrument:argv=['/usr/bin/valgrind','--leak-check=full','--show-leak-kinds=all','--track-fds=yes','--log-file='+str(path),*argv]
        done=subprocess.run(argv,stdin=subprocess.DEVNULL,capture_output=True,timeout=30)
        outcome={'instrumented':instrument,'status':done.returncode,'stdout':done.stdout.hex(),'stderr':done.stderr.hex(),'pass':done.returncode==0 and not done.stdout and not done.stderr}
        if instrument:
            text=path.read_text();pids=set(re.findall(r'^==([0-9]+)==',text,re.M));assert len(pids)==1
            m=runner.parse_memory_log(text,pids.pop(),exec_only=True)
            clean=m['complete_exec_log'] and m['errors']==0 and m['non_inherited_descriptors']==0 and not any(m['heap_bytes'].get(k,0) for k in ('definitely lost','indirectly lost','possibly lost'))
            outcome.update(memory=m,memory_clean=clean,log=str(path.relative_to(ROOT)),sha256=sha(path));outcome['pass'] &= clean
        outcomes.append(outcome)
    results.append({'mode':mode,'pass':all(o['pass'] for o in outcomes),'outcomes':outcomes})
report={'scope':'All eight combinations of closed standard descriptors, both normal pipe close and exit ownership, plus pipe failure under a private descriptor limit. Verify inherited standards survive, owned pipe standards close, errno is retained, and no stale ownership causes descriptor operations after close.',
 'driver_sha256':sha(__file__),'runtime_source_sha256':sha(source),'harness':str(harness),'harness_sha256':sha(harness),'binary_sha256':sha(binary),'compile_arguments':args,'build_log_sha256':sha(log),'passed':sum(r['pass'] for r in results),'total':len(results),'results':results}
target.write_text(json.dumps(report,indent=2)+'\n');print('Pipe ownership:',report['passed'],'/',report['total']);raise SystemExit(report['passed']!=report['total'])
