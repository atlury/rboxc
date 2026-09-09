#!/usr/bin/env python3
"""Check duplicated target ownership through closes, exit and descriptor reuse."""
# SPDX-License-Identifier: GPL-3.0-or-later
import argparse,errno,hashlib,importlib.util,json,re,subprocess,sys,tempfile
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
sys.path.insert(0,str(ROOT/'tests/gnu'))
spec=importlib.util.spec_from_file_location('reviewed',ROOT/'tests/gnu/reviewed-original.py')
runner=importlib.util.module_from_spec(spec);spec.loader.exec_module(runner)
parser=argparse.ArgumentParser(description=__doc__)
parser.add_argument('--report-name',default='bash-duplication-contract')
options=parser.parse_args()
assert re.fullmatch(r'[a-z0-9][a-z0-9-]*',options.report_name)
target=ROOT/'evidence'/(options.report_name+'.json')
assert not target.exists(), 'preserve existing evidence; choose a new report name'
stage=Path(tempfile.mkdtemp(prefix='bash-backup-contract-',dir=ROOT/'build'))
harness=stage/'contract.c'
harness.write_text(r'''#include <errno.h>
#include <fcntl.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
extern int rboxc_bash_owned_dup2(int,int);
extern int rboxc_bash_owned_close(int);
extern int __real_close(int);
static int injected, owned_exit;
static FILE *reused;
static const char *path;
int __wrap_close(int fd) {
  /* Already-closed descriptors now reach the real syscall if production
     fails to check them; Valgrind must catch any duplicate close. */
  int r=__real_close(fd);
  if (r==0 && fd==10 && injected) { errno=injected; injected=0; return -1; }
  return r;
}
static void verify(void) {
  if (owned_exit) {
    if (fcntl(10,F_GETFD)>=0 || errno!=EBADF) _Exit(91);
  } else {
    if (fcntl(10,F_GETFD)<0 || fflush(reused) || fclose(reused)) _Exit(92);
    reused=NULL;
    char buffer[32]={0}; int fd=open(path,O_RDONLY);
    if (fd<0 || read(fd,buffer,sizeof buffer)!=9 || memcmp(buffer,"retained\n",9)) _Exit(93);
    if (__real_close(fd)) _Exit(94);
  }
  if (unlink(path)) _Exit(95);
  puts("backup ownership passed");
}
int main(int argc,char **argv) {
  if (argc!=3) return 80;
  path=argv[2]; int mode=atoi(argv[1]);
  if (atexit(verify)) return 82;
  int fd=open(path,O_CREAT|O_EXCL|O_RDWR,0600);
  if (fd<0 || rboxc_bash_owned_dup2(fd,10)!=10 || __real_close(fd)) return 81;

  if (mode==-2) { owned_exit=1; return 0; }
  if (mode==-1 && __real_close(10)) return 83;
  injected=mode;
  errno=0;
  int result=rboxc_bash_owned_close(10), saved=errno;
  if (mode==0 ? result!=0 : (result!=-1 || saved!=(mode==-1?EBADF:mode))) return 84;
  fd=open(path,O_WRONLY|O_TRUNC);
  if (fd<0 || dup2(fd,10)!=10 || __real_close(fd)) return 85;
  reused=fdopen(10,"w");
  if (!reused || fputs("retained\n",reused)<0) return 86;
  return 0;
}
''')
source=ROOT/'src/shell_arguments.c';binary=stage/'contract'
args=['gcc','-O2','-fPIC','-Wall','-Wextra','-Werror',str(harness),str(source),'-Wl,--wrap=close','-o',str(binary)]
build=stage/'build.log'
with build.open('w') as log:subprocess.run(args,stdout=log,stderr=subprocess.STDOUT,check=True)
logs=Path(tempfile.mkdtemp(prefix='bash-backup-contract-',dir=ROOT/'evidence/raw'))
results=[]
for name,mode in [('success',0),('late-eio',errno.EIO),('interrupted',errno.EINTR),('late-enospc',errno.ENOSPC),('late-edquot',errno.EDQUOT),('already-closed',-1),('owned-exit',-2)]:
    outcomes=[]
    for instrument in (False,True):
        with tempfile.TemporaryDirectory(prefix='rboxc-backup-') as directory:
            log=logs/(name+'.log');argv=[str(binary),str(mode),str(Path(directory)/'file')]
            if instrument:argv=['/usr/bin/valgrind','--leak-check=full','--show-leak-kinds=all','--track-fds=yes','--log-file='+str(log),*argv]
            done=subprocess.run(argv,capture_output=True,timeout=30)
            passed=done.returncode==0 and done.stdout==b'backup ownership passed\n' and not done.stderr
            outcome={'instrumented':instrument,'status':done.returncode,'stdout':done.stdout.hex(),'stderr':done.stderr.hex(),'pass':passed}
            if instrument:
                text=log.read_text();pids=set(re.findall(r'^==([0-9]+)==',text,re.M));assert len(pids)==1
                memory=runner.parse_memory_log(text,pids.pop(),exec_only=True)
                clean=memory['complete_exec_log'] and memory['errors']==0 and memory['non_inherited_descriptors']==0 and not any(memory['heap_bytes'].get(k,0) for k in ('definitely lost','indirectly lost','possibly lost'))
                outcome.update(memory=memory,log=str(log.relative_to(ROOT)),sha256=sha(log),memory_clean=clean)
                outcome['pass']=passed and clean
            outcomes.append(outcome)
    results.append({'case':name,'pass':all(o['pass'] for o in outcomes),'outcomes':outcomes})
report={'scope':'Native and Valgrind checks of owned duplicated-target cleanup and same-inode descriptor reuse after successful close, simulated Linux late close errors, and an already-closed descriptor. The wrapper closes the real descriptor before returning a simulated late error. An already-closed descriptor reaches real close if the production guard fails, so duplicate-close findings cannot be hidden by the test wrapper. No filesystem fault or malformed input is required.',
        'driver_sha256':sha(__file__),'runtime_source_sha256':sha(source),'harness_sha256':sha(harness),'binary_sha256':sha(binary),'compile_arguments':args,'build_log_sha256':sha(build),'passed':sum(r['pass'] for r in results),'total':len(results),'results':results}
target.write_text(json.dumps(report,indent=2)+'\n')
print('Bash duplicated target ownership:',report['passed'],'/',report['total'])
raise SystemExit(report['passed']!=report['total'])
