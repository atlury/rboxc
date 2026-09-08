#!/usr/bin/env python3
"""Compare iconv's translated interface layouts with the pinned GNU headers."""
# SPDX-License-Identifier: GPL-3.0-or-later
import hashlib
import json
from pathlib import Path
import re
import subprocess

ROOT=Path(__file__).resolve().parents[1]
digest=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
stage=ROOT/'build/iconv-abi';stage.mkdir(exist_ok=True)
module=ROOT/'src/generated/applet_iconv.rs'
text=module.read_text()
types=['__gconv_step','__gconv_step_data','__gconv_info','__gconv_loaded_object',
       'gconv_spec','gconv_alias','gconv_module','gconvcache_header',
       'argp','argp_option','argp_state','charmap_t','charseq']
c=['#include <stddef.h>','#include <stdio.h>','#include <argp.h>',
   '#include <charmap.h>','#include <gconv_int.h>','#include "iconvconfig.h"',
   '#include "gconv_charset.h"','int main(void) {']
r=['#![feature(c_variadic)]','#![allow(warnings)]',
   '#[path = '+json.dumps(str(module))+'] mod applet;',
   'use applet::*;','fn main() {']
fields={}
for name in types:
 body=re.search(r'pub struct '+name+r' \{(.*?)\n\}',text,re.S)[1]
 fields[name]=re.findall(r'^    pub (\w+):',body,re.M)
 assert fields[name]
 c.append('printf("'+name+' %zu %zu", sizeof(struct '+name+'), _Alignof(struct '+name+'));')
 r.append('print!("'+name+' {} {}",std::mem::size_of::<'+name+'>(),std::mem::align_of::<'+name+'>());')
 for field in fields[name]:
  c.append('printf(" %zu",offsetof(struct '+name+','+field+'));')
  r.append('print!(" {}",std::mem::offset_of!('+name+','+field+'));')
 c.append('putchar(10);');r.append('println!();')
c.append('}');r.append('}')
source=stage/'layout.c';source.write_text('\n'.join(c)+'\n')
rust=stage/'layout.rs';rust.write_text('\n'.join(r)+'\n')
record=json.loads((ROOT/'build/translation/iconv/compile_commands.json').read_text())[0]
args=['gcc'];skip=False
for word in record['arguments'][1:]:
 if skip:skip=False;continue
 if word in ('-o','-MF','-MT'):skip=True;continue
 if word in ('-c','-MD','-MP') or word.endswith('.c') or word.startswith('-Dmain='):continue
 args.append(word)
args += [str(source),'-o',str(stage/'native')]
with (stage/'native-build.log').open('w') as log:
 subprocess.run(args,cwd=record['directory'],stdout=log,stderr=subprocess.STDOUT,check=True)
expected=subprocess.check_output([stage/'native'])
deps=ROOT/'target/gnu-glibc-entries-candidate/release/deps'
libc=list(deps.glob('liblibc-*.rlib'));bits=list(deps.glob('libc2rust_bitfields-*.rlib'))
assert len(libc)==len(bits)==1
with (stage/'rust-build.log').open('w') as log:
 subprocess.run(['rustc','+nightly-2026-01-22','--edition=2021','-C','opt-level=1','-C','panic=abort',
  '--extern','libc='+str(libc[0]),'--extern','c2rust_bitfields='+str(bits[0]),
  '-L','dependency='+str(deps),str(rust),'-o',str(stage/'rust')],stdout=log,stderr=subprocess.STDOUT,check=True)
actual=subprocess.check_output([stage/'rust']);assert actual==expected,(actual,expected)
report={'scope':'Thirteen interface structures match pinned GNU 2.43 C sizeof, alignment, and every named field offset. This checks the translated header ABI on Linux x86-64; it does not certify other libc releases or encoding modules.',
 'driver_sha256':digest(Path(__file__)),'module_sha256':digest(module),'fields':fields,
 'c_arguments':args,'c_directory':record['directory'],'native_source_sha256':digest(source),
 'rust_source_sha256':digest(rust),'native_binary_sha256':digest(stage/'native'),
 'rust_binary_sha256':digest(stage/'rust'),'output':actual.decode(),'passed':len(types),'total':len(types)}
(ROOT/'evidence/iconv-abi.json').write_text(json.dumps(report,indent=2)+'\n')
print(len(types),'iconv interface structures match pinned GNU C layout')
