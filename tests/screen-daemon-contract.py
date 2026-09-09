#!/usr/bin/env python3
"""Check Screen daemon cleanup ownership, repeat cleanup, and fork boundaries."""
# SPDX-License-Identifier: GPL-3.0-or-later
import importlib.util
import json
from pathlib import Path
import re
import subprocess
import sys
from comparison_profile import fingerprint

ROOT=Path(__file__).resolve().parents[1]
sys.path[:0]=[str(ROOT/'scripts'),str(ROOT/'tests/gnu')]
from screen_daemon_cleanup import RUST_OWNERSHIP
spec=importlib.util.spec_from_file_location('reviewed',ROOT/'tests/gnu/reviewed-original.py')
runner=importlib.util.module_from_spec(spec);spec.loader.exec_module(runner)
stage=ROOT/'build/screen-owned-contract';stage.mkdir()
rs=stage/'streams.rs'
rs.write_text(r'''
extern crate libc;
use libc::{FILE, freopen, getpid};
extern "C" { fn tgetent(buffer: *mut libc::c_char, name: *const libc::c_char)->libc::c_int; fn tgoto(cap: *const libc::c_char, col:libc::c_int,row:libc::c_int)->*mut libc::c_char; static mut stdin: *mut FILE; static mut stdout: *mut FILE; static mut stderr: *mut FILE; }
'''+RUST_OWNERSHIP+r'''
fn main() { unsafe {
    let mode=std::env::args().nth(1).unwrap();
    let selected:i32=std::env::args().nth(2).unwrap().parse().unwrap();
    if mode=="borrowed" {
        rboxc_screen_release_streams();
        for fd in 0..3 { assert!(libc::fcntl(fd,libc::F_GETFD)>=0); }
        return;
    }
    let stream=if selected==0 {stdin} else if selected==1 {stdout} else {stderr};
    let access=if selected==0 {b"r\0"} else {b"w\0"};
    let opened=rboxc_screen_owned_freopen(b"/dev/null\0".as_ptr().cast(),access.as_ptr().cast(),stream);
    assert!(!opened.is_null());
    if mode=="reopen" {
        assert!(!rboxc_screen_owned_freopen(b"/dev/null\0".as_ptr().cast(),access.as_ptr().cast(),stream).is_null());
    }
    if mode=="terminal" {
        for terminal in [b"xterm\0".as_slice(),b"screen\0",b"vt100\0",b"xterm\0"] {
            assert_eq!(tgetent(std::ptr::null_mut(),terminal.as_ptr().cast()),1);
            for cap in [b"\x1b[%i%p1%d;%p2%dH\0".as_slice(),b"\x1b[%i%p1%d;%p2%dr\0"] {
                let output=tgoto(cap.as_ptr().cast(),2,3); assert!(!output.is_null());
                assert!(std::ffi::CStr::from_ptr(output).to_bytes().starts_with(b"\x1b[4;3"));
            }
        }
    }
    if mode=="fail" {
        assert!(rboxc_screen_owned_freopen(b"/dev/null/rboxc-input\0".as_ptr().cast(),access.as_ptr().cast(),stream).is_null());
    }
    if mode=="fork" || mode=="fork-reopen" {
        let pid=libc::fork(); assert!(pid>=0);
        if pid==0 {
            let other=(selected+1)%3;
            if mode=="fork-reopen" {
                let borrowed=if other==0 {stdin} else if other==1 {stdout} else {stderr};
                assert!(!rboxc_screen_owned_freopen(b"/dev/null\0".as_ptr().cast(),b"r\0".as_ptr().cast(),borrowed).is_null());
            }
            rboxc_screen_release_streams();
            if mode=="fork-reopen" && libc::fcntl(other,libc::F_GETFD)>=0 {libc::_exit(92);}
            if libc::fcntl(selected,libc::F_GETFD)<0 {libc::_exit(91);}
            libc::close(selected);
            libc::_exit(0);
        }
        let mut status=0; assert_eq!(libc::waitpid(pid,&mut status,0),pid); assert_eq!(status,0);
    }
    *libc::__errno_location()=libc::E2BIG;
    rboxc_screen_release_streams();
    assert_eq!(*libc::__errno_location(),libc::E2BIG);
    assert!(libc::fcntl(selected,libc::F_GETFD)<0);
    for fd in 0..3 {if fd!=selected {assert!(libc::fcntl(fd,libc::F_GETFD)>=0);}}
    let reused=libc::open(b"/dev/null\0".as_ptr().cast(),libc::O_RDONLY); assert_eq!(reused,selected);
    rboxc_screen_release_streams();
    assert!(libc::fcntl(reused,libc::F_GETFD)>=0); assert_eq!(libc::close(reused),0);
}}
''')
libc_paths=list((ROOT/'target/screen-owned-candidate/release/deps').glob('liblibc-*.rlib'));assert len(libc_paths)==1
rust_binary=stage/'streams'
build=['rustup','run','nightly-2026-01-22','rustc','--edition=2021','-C','panic=abort',str(rs),'--extern','libc='+str(libc_paths[0]),
       '-L','dependency='+str(libc_paths[0].parent),'-l','tinfo','-o',str(rust_binary)]
with (stage/'rust-build.log').open('w') as log:subprocess.run(build,stdout=log,stderr=subprocess.STDOUT,check=True)
# Compile the actual private socket helper, keeping its static ownership routine
# in this bounded harness; unrelated GNU sections are discarded by the linker.
c=stage/'socket-contract.c'
c.write_text('#include "'+str(ROOT/'build/screen-daemon-cleanup/socket.c')+'"\n'+r'''
#include <assert.h>
#include <sys/wait.h>
int ServerSocket=-1;
int main(int argc,char **argv) {
 int pair[2]; assert(socketpair(AF_UNIX,SOCK_STREAM,0,pair)==0);
 int owned=pair[0], borrowed=pair[1];
 rboxc_owned_server=ServerSocket=owned; rboxc_server_owner=getpid();
 if(strcmp(argv[1],"fork")==0) {
  pid_t child=fork(); assert(child>=0);
  if(child==0) {rboxc_release_server(); if(fcntl(owned,F_GETFD)<0)_exit(91); close(owned);close(borrowed);_exit(0);}
  int status;assert(waitpid(child,&status,0)==child);assert(status==0);
 }
 errno=E2BIG;rboxc_release_server();assert(errno==E2BIG);
 assert(ServerSocket==-1 && rboxc_owned_server==-1);
 assert(fcntl(owned,F_GETFD)<0 && fcntl(borrowed,F_GETFD)>=0);
 int reused=open("/dev/null",O_RDONLY);assert(reused==owned);
 rboxc_release_server();assert(fcntl(reused,F_GETFD)>=0);
 close(reused);close(borrowed);return 0;
}
''')
records=[json.loads(p.read_text()) for p in (ROOT/'build/screen-cc-records').glob('*.json')]
record=next(r for r in records if r.get('file')=='/opt/src/screen-5.0.2/socket.c')
obj=stage/'socket-contract.o';argv=record['arguments'].copy();argv[argv.index(record['file'])]=str(c);argv[argv.index('-o')+1]=str(obj)
argv+=['-ffunction-sections','-fdata-sections','-iquote','/opt/src/screen-5.0.2']
with (stage/'socket-build.log').open('w') as log:
 subprocess.run(argv,cwd=record['directory'],stdout=log,stderr=subprocess.STDOUT,check=True)
 subprocess.run(['gcc',str(obj),'-Wl,--gc-sections','-o',str(stage/'socket')],stdout=log,stderr=subprocess.STDOUT,check=True)
results=[]
cases=[('streams',mode,str(fd)) for mode in ['borrowed','owned','reopen','fork','fail','fork-reopen'] for fd in ([0] if mode=='borrowed' else [0,1,2])]
cases += [('streams','terminal','0')]
cases += [('socket',mode) for mode in ['owned','fork']]
for index,case in enumerate(cases):
 logdir=stage/('case-'+str(index));logdir.mkdir()
 done=subprocess.run(['/usr/bin/valgrind','--leak-check=full','--show-leak-kinds=all','--track-fds=yes',
     '--log-file='+str(logdir/'%p.log'),str(stage/case[0]),*case[1:]],capture_output=True,timeout=30)
 (logdir/'stdout').write_bytes(done.stdout);(logdir/'stderr').write_bytes(done.stderr)
 assert done.returncode==0,(case,done.stderr)
 memory=[]
 for log in sorted(logdir.glob('*.log')):
  text=log.read_text();pids=set(re.findall(r'^==([0-9]+)==',text,re.M));assert pids=={log.stem}
  parsed=runner.parse_memory_log(text,log.stem,exec_only=True)
  assert parsed['complete_exec_log'] and parsed['errors']==parsed['non_inherited_descriptors']==0,(case,parsed)
  assert not any(parsed['heap_bytes'].get(k,0) for k in ['definitely lost','indirectly lost','possibly lost']),(case,parsed)
  memory.append({'path':str(log.relative_to(ROOT)),'sha256':fingerprint(log),**parsed})
 assert len(memory)==(2 if any(n.startswith('fork') for n in case) else 1)
 results.append({'case':case,'pass':True,'status':done.returncode,'memory':memory})
inputs=[Path(__file__),ROOT/'scripts/screen_daemon_cleanup.py',ROOT/'build/screen-daemon-cleanup/socket.c',rs,c,libc_paths[0]]
report={'scope':'Nineteen ownership contracts cover borrowed standard streams, all three successfully reopened '
 'standard streams, repeated reopen/cleanup, descriptor reuse and fork inheritance. Socket cases use '
 'a private socketpair and the exact adapted static cleanup function. After checking inherited-handle preservation, the child harness disposes its copies. All parent and child logs must be clean.',
 'passed':len(results),'total':19,'processes':sum(len(r['memory']) for r in results),
 'inputs':{str(p):fingerprint(p) for p in inputs+[Path('/usr/lib/x86_64-linux-gnu/libtinfo.so.6').resolve()]},'rust_build':build,'socket_compile':argv,
 'artifacts':{str(p.relative_to(ROOT)):fingerprint(p) for p in stage.rglob('*') if p.is_file()},'results':results}
(ROOT/'evidence/screen-owned-contract.json').write_text(json.dumps(report,indent=2)+'\n')
print('Daemon ownership contracts:',len(results),'passed;',report['processes'],'clean processes')
