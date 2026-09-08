// SPDX-License-Identifier: GPL-3.0-or-later
// External adapters run one builtin in a fresh GNU shell context. Shell state
// changes therefore belong to this process. [[ accepts literal argv operands;
// only the recognized grammar tokens are inserted into the shell program.
use core::ffi::{c_char,c_int,CStr};
use std::ffi::CString;
unsafe extern "C" {
    fn rboxc_shell_arguments(argc:c_int,argv:*mut *mut c_char,script:*const c_char)->*mut *mut c_char;
}
const BUILTINS: &[&[u8]] = &[b".",b":",b"alias",b"break",b"cd",b"continue",b"declare",b"eval",b"exec",b"exit",b"export",b"help",b"jobs",b"local",b"return",b"set",b"shift",b"source",b"trap",b"ulimit",b"unalias",b"unset",b"wait"];
fn binary(x:&[u8])->bool { [b"=".as_slice(),b"==",b"!=",b"=~",b"<",b">",b"-eq",b"-ne",b"-lt",b"-le",b"-gt",b"-ge",b"-nt",b"-ot",b"-ef"].contains(&x) }
fn unary(x:&[u8])->bool { [b"-a".as_slice(),b"-b",b"-c",b"-d",b"-e",b"-f",b"-g",b"-h",b"-k",b"-p",b"-r",b"-s",b"-t",b"-u",b"-w",b"-x",b"-G",b"-L",b"-N",b"-O",b"-S",b"-o",b"-v",b"-R",b"-z",b"-n"].contains(&x) }
struct Conditional<'a> { args:Vec<&'a [u8]>, at:usize }
impl Conditional<'_> {
    fn take(&mut self,x:&[u8])->bool { if self.args.get(self.at)==Some(&x) {self.at+=1;true} else {false} }
    fn operand(&mut self,pattern:bool)->Result<String,()> {
        if self.at>=self.args.len() {return Err(())}
        self.at+=1;
        let p=format!("${{{}}}",self.at);
        Ok(if pattern {p} else {format!("\"{p}\"")})
    }
    fn primary(&mut self)->Result<String,()> {
        if self.take(b"!") {return Ok(format!("! {}",self.primary()?))}
        if self.take(b"(") {let e=self.or()?;if !self.take(b")") {return Err(())} return Ok(format!("( {e} )"))}
        let token=*self.args.get(self.at).ok_or(())?;
        // GNU parses the binary operator before interpreting a unary-looking lhs.
        if self.args.get(self.at+1).is_some_and(|x|binary(x)) {
            let left=self.operand(false)?;
            let op=self.args[self.at];self.at+=1;
            let right=self.operand([b"=".as_slice(),b"==",b"!=",b"=~"].contains(&op))?;
            return Ok(format!("{left} {} {right}",std::str::from_utf8(op).unwrap()))
        }
        if unary(token) && self.at+1<self.args.len() {
            self.at+=1;let arg=self.operand(false)?;
            return Ok(format!("{} {arg}",std::str::from_utf8(token).unwrap()))
        }
        if [b")".as_slice(),b"&&",b"||"].contains(&token) {return Err(())}
        self.operand(false)
    }
    fn and(&mut self)->Result<String,()> {let mut e=self.primary()?;while self.take(b"&&") {e=format!("{e} && {}",self.primary()?)} Ok(e)}
    fn or(&mut self)->Result<String,()> {let mut e=self.and()?;while self.take(b"||") {e=format!("{e} || {}",self.and()?)} Ok(e)}
}
pub unsafe fn run_script(argc:c_int,argv:*mut *mut c_char,script:String)->c_int {
    let script=match CString::new(script) {Ok(x)=>x,Err(_)=>return 2};
    let shell_argv=rboxc_shell_arguments(argc,argv,script.as_ptr());
    drop(script);
    if shell_argv.is_null() {super::write_all(2,b"rboxc: cannot allocate shell arguments\n");return 1}
    super::applet_bash::single_binary_main_bash(argc+5,shell_argv)
}
pub unsafe extern "C" fn main(argc:c_int,argv:*mut *mut c_char)->c_int {
    let name=CStr::from_ptr(*argv).to_bytes().rsplit(|b|*b==b'/').next().unwrap();
    let script=if name==b"[[" {
        let mut p=Conditional {args:(1..argc).map(|i|CStr::from_ptr(*argv.add(i as usize)).to_bytes()).collect(),at:0};
        if p.args.last()==Some(&b"]]".as_slice()) {p.args.pop();}
        let e=if p.args.is_empty() {Ok(String::from("\"\""))} else {p.or()};
        match e {Ok(e) if p.at==p.args.len()=>format!("[[ {e} ]]"),_=>{super::write_all(2,b"rboxc [[: invalid conditional arguments\n");return 2}}
    } else if BUILTINS.contains(&name) {
        format!("builtin {} \"$@\"",std::str::from_utf8(name).unwrap())
    } else {return 127};
    run_script(argc,argv,script)
}
