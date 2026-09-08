#!/usr/bin/env python3
"""Verify translated gate ABI and actual C boundary recovery from Rust callers."""
# SPDX-License-Identifier: GPL-3.0-or-later
import json,re,subprocess,sys
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1];sys.path.insert(0,str(ROOT/'scripts'))
from entry_provider_helpers import fingerprint
from split_entry import extent
stage=ROOT/'build/bash-gate-contract';stage.mkdir(exist_ok=True)
outlined=ROOT/'build/translation/bash/outlined.c';text=outlined.read_text()
module=ROOT/'src/generated/applet_bash.rs'
structures=re.findall(r'struct (rboxc_bash_gate|rboxc_gate_result_\d+) \{(.*?)\};',text,re.S)
assert len(structures)==88
headers='#include <signal.h>\n#include <setjmp.h>\n#include <errno.h>\n#include <stdio.h>\n#include <stddef.h>\n#include <assert.h>\ntypedef struct variable SHELL_VAR;\n'
c=stage/'layout.c';rust=stage/'layout.rs'
cs=headers+''.join('struct '+n+' {'+body+'};\n' for n,body in structures)+'int main(void) {\n'
rs='#![feature(c_variadic)]\n#![allow(warnings)]\n#[path="'+str(module)+'"] mod entry;\nuse entry::*;\nfn main() {\n'
fields={}
for n,body in structures:
 if n=='rboxc_bash_gate':fs=['allowed','top_mask','sub_mask','top','sub','top_save','sub_save','pending','target']
 else:fs=['jumped','target','code']+(['value'] if 'value' in body else [])
 fields[n]=fs
 cs+='printf("'+n+' %zu %zu'+' %zu'*len(fs)+'\\n",sizeof(struct '+n+'),_Alignof(struct '+n+')'+''.join(',offsetof(struct '+n+','+f+')' for f in fs)+');\n'
 rs+='println!("'+n+' {} {}'+' {}'*len(fs)+'",std::mem::size_of::<'+n+'>(),std::mem::align_of::<'+n+'>()'+''.join(',std::mem::offset_of!('+n+','+f+')' for f in fs)+');\n'
c.write_text(cs+'}\n');rust.write_text(rs+'}\n')
subprocess.run(['gcc','-std=gnu17',str(c),'-o',str(stage/'layout-c')],check=True)
deps=ROOT/'target/gnu-all-integrations-candidate/release/deps'
libs=[]
for name in ('libc','c2rust_bitfields'):
 lib=list(deps.glob('lib'+name+'-*.rlib'));assert len(lib)==1
 libs+=['--extern',name+'='+str(lib[0])]
def compile_rust(source,binary,extra=[]):
 with source.with_suffix('.log').open('w') as log:
  subprocess.run(['rustup','run','nightly-2026-01-22','rustc','--edition=2021','-C','opt-level=1','-C','panic=abort',*libs,'-L','dependency='+str(deps),str(source),'-o',str(binary),*extra],stdout=log,stderr=subprocess.STDOUT,check=True)
compile_rust(rust,stage/'layout-rust')
a=subprocess.check_output([stage/'layout-c']);b=subprocess.check_output([stage/'layout-rust']);assert a==b
# Extract the actual emitted helper bodies using their reviewed Clang ranges.
ast=json.loads((ROOT/'build/translation/bash/split-ast.json').read_text())
selected=['rboxc_gate_begin','rboxc_gate_block','rboxc_gate_caught','rboxc_gate_checkpoint','rboxc_gate_call_70']
bodies={}
for n in ast['inner']:
 if n.get('name') in selected and any(c.get('kind')=='CompoundStmt' for c in n.get('inner',[])):
  bodies[n['name']]=text[slice(*extent(n))]
assert set(bodies)==set(selected)
mock=headers+'sigjmp_buf top_level,subshell_top_level;\n'+''.join('struct '+n+' {'+body+'};\n' for n,body in structures if n in ('rboxc_bash_gate','rboxc_gate_result_70'))
mock+='''
int run_one_command(char *mode) {
 sigset_t current; sigprocmask(SIG_SETMASK,0,&current);
 assert(sigismember(&current,SIGUSR1)==1);
 assert(sigismember(&current,SIGUSR2)==0);
 if (*mode=='t') siglongjmp(top_level,9);
 if (*mode=='s') siglongjmp(subshell_top_level,11);
 if (*mode=='m') {sigaddset(&current,SIGUSR2);sigprocmask(SIG_SETMASK,&current,0);siglongjmp(top_level,13);}
 errno=EAGAIN;
 return 42;
}
'''
mock+='\n'.join(bodies[n] for n in selected)
# Bind the real generated Rust imports to these isolated boundary definitions.
for n in selected:mock= re.sub(r'\b'+n+r'\b','rboxc_bash_'+n,mock)
mc=stage/'mock.c';mc.write_text(mock)
subprocess.run(['gcc','-O2','-std=gnu17','-Wall','-Wextra','-Werror','-c',str(mc),'-o',str(stage/'mock.o')],check=True)
probe=stage/'recovery.rs';probe.write_text('#![feature(c_variadic)]\n#![allow(warnings)]\n'+module.read_text()+r'''
fn main() {unsafe {
 let mut initial:libc::sigset_t=std::mem::zeroed();libc::sigemptyset(&mut initial);libc::sigaddset(&mut initial,libc::SIGUSR1);libc::sigprocmask(libc::SIG_SETMASK,&initial,std::ptr::null_mut());
 let mut g:rboxc_bash_gate=std::mem::zeroed();rboxc_gate_begin(&mut g);
 assert_eq!(rboxc_gate_checkpoint(&mut g,1,0,0),0);
 assert_eq!(rboxc_gate_checkpoint(&mut g,2,1,1),0);
 let r=rboxc_gate_call_70(&mut g,b"normal\0".as_ptr().cast_mut().cast());assert_eq!((r.jumped,r.value),(0,42));assert_eq!(*libc::__errno_location(),libc::EAGAIN);
 let mut mask:libc::sigset_t=std::mem::zeroed();libc::sigprocmask(libc::SIG_SETMASK,std::ptr::null(),&mut mask);assert_eq!(libc::sigismember(&mask,libc::SIGUSR2),1);
 for (mode,target,code) in [(b"top\0".as_slice(),1,9),(b"sub\0".as_slice(),2,11)] {
  let r=rboxc_gate_call_70(&mut g,mode.as_ptr().cast_mut().cast());assert_eq!((r.jumped,r.target,r.code),(1,target,code));g.target=r.target;g.pending=r.code;assert_eq!(rboxc_gate_checkpoint(&mut g,target,(target==2) as i32,(target==2) as i32),code);
 }
 assert_eq!(rboxc_gate_checkpoint(&mut g,4,0,1),0);
 let r=rboxc_gate_call_70(&mut g,b"mask\0".as_ptr().cast_mut().cast());assert_eq!((r.jumped,r.target,r.code),(1,4,13));
 // The saved checkpoint mask, not the callee's changed mask, reaches the next call.
 let r=rboxc_gate_call_70(&mut g,b"normal\0".as_ptr().cast_mut().cast());assert_eq!((r.jumped,r.value),(0,42));
 libc::sigprocmask(libc::SIG_SETMASK,&initial,std::ptr::null_mut());
 println!("normal return, errno, Rust mask, top recovery, subshell recovery, saved-mask recovery: pass");
}}
''')
compile_rust(probe,stage/'recovery',['-C','link-arg='+str(stage/'mock.o')])
outputs=[]
for instrument in (False,True):
 argv=[str(stage/'recovery')]
 log=stage/'valgrind.log'
 if instrument:argv=['valgrind','--error-exitcode=99','--leak-check=full','--track-fds=yes','--log-file='+str(log),*argv]
 p=subprocess.run(argv,capture_output=True,check=True);outputs.append(p.stdout.decode())
assert outputs[0]==outputs[1]
report={'scope':'All 88 generated gate/result structures match actual C declarations in size, alignment and every field offset. Rust callers exercise the emitted C boundary with bounded mock GNU call outcomes, including both jump buffers, errno and saved signal masks. This is a boundary contract, not full Bash signal certification.',
 'driver_sha256':fingerprint(Path(__file__)),'outlined_sha256':fingerprint(outlined),'rust_module_sha256':fingerprint(module),'layout_output':a.decode(),
 'abi_structures':len(structures),'contract_output':outputs[0],'files':{str(p.relative_to(ROOT)):fingerprint(p) for p in [c,rust,mc,probe,stage/'layout-c',stage/'layout-rust',stage/'recovery',stage/'valgrind.log']},'pass':True}
(ROOT/'evidence/bash-gate-contract.json').write_text(json.dumps(report,indent=2)+'\n');print('88 ABI layouts and native/Valgrind recovery contract pass')
