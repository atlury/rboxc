#!/usr/bin/env python3
"""Compare generated Binutils enum bitfield layout against the pinned C headers."""
# SPDX-License-Identifier: GPL-3.0-or-later
import hashlib,json,re,subprocess
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
digest=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
stage=ROOT/'build/binutils-enum-abi';stage.mkdir(exist_ok=True)
native=stage/'layout.c'
native.write_text(r'''#include "sysdep.h"
#include "bfd.h"
#include <stddef.h>
#include <stdio.h>
#include <string.h>
int main(void) {
 printf("%zu %zu %zu %zu %zu\n",sizeof(bfd),_Alignof(bfd),offsetof(bfd,flags),offsetof(bfd,plugin_dummy_bfd),sizeof(reloc_howto_type));
 for(unsigned i=0;i<8;i++) {
  bfd b; struct reloc_howto_struct h; memset(&b,0,sizeof b); memset(&h,0,sizeof h);
  b.format=i; b.direction=i&3; b.last_io=i&3; b.plugin_format=i&3; b.lto_type=i;
  h.complain_on_overflow=i&3;
  printf("%u %u %u %u %u %u ",b.format,b.direction,b.last_io,b.plugin_format,b.lto_type,h.complain_on_overflow);
  unsigned char *p=(unsigned char *)&b + offsetof(bfd,flags)+sizeof(b.flags);
  for(int k=0;k<4;k++)printf("%02x",p[k]);
  putchar(' ');
  p=(unsigned char *)&h+sizeof(h.type);
  for(int k=0;k<4;k++)printf("%02x",p[k]);
  putchar('\n');
 }
}
''')
source=ROOT/'build/gnu-binutils/binutils'
args=['gcc','-std=gnu17','-DHAVE_CONFIG_H','-I'+str(source),'-I/opt/src/binutils-2.47/binutils','-I'+str(ROOT/'build/gnu-binutils/bfd'),'-I/opt/src/binutils-2.47/include',str(native),'-o',str(stage/'native')]
subprocess.run(args,check=True)
expected=subprocess.check_output([stage/'native'])
results=[]
for command in ('ar','strings','readelf'):
 module=ROOT/f'src/generated/applet_{command}.rs';text=module.read_text()
 array=re.search(r'pub (format_direction_\w+):\s*\[u8; 4\]',text)[1]
 howto=re.search(r'pub (size_bitsize_\w+):\s*\[u8; 4\]',text)[1]
 rust=stage/(command+'.rs')
 rust.write_text('''#![feature(c_variadic)]
#![allow(warnings)]
#[path = "'''+str(module)+'''"] mod applet;
use applet::*;
fn main() { unsafe {
 println!("{} {} {} {} {}",std::mem::size_of::<bfd>(),std::mem::align_of::<bfd>(),std::mem::offset_of!(bfd,flags),std::mem::offset_of!(bfd,plugin_dummy_bfd),std::mem::size_of::<reloc_howto_struct>());
 for i in 0..8 {
  let mut b:bfd=std::mem::zeroed(); let mut h:reloc_howto_struct=std::mem::zeroed();
  b.set_format(bfd_format(i)); b.set_direction(bfd_direction(i&3)); b.set_last_io(bfd_last_io(i&3)); b.set_plugin_format(bfd_plugin_format(i&3)); b.set_lto_type(bfd_lto_object_type(i));
  h.set_complain_on_overflow(complain_overflow(i&3));
  print!("{} {} {} {} {} {} ",b.format().0,b.direction().0,b.last_io().0,b.plugin_format().0,b.lto_type().0,h.complain_on_overflow().0);
  for v in b.'''+array+''' {print!("{:02x}",v);}
  print!(" ");
  for v in h.'''+howto+''' {print!("{:02x}",v);}
  println!();
 }
}}
''')
 deps=ROOT/'target/gnu-batch-cleanup-candidate/release/deps'
 lib=list(deps.glob('libc2rust_bitfields-*.rlib'));assert len(lib)==1
 libc=list(deps.glob('liblibc-*.rlib'));assert len(libc)==1
 binary=stage/command
 with (stage/(command+'-build.log')).open('w') as log:
  subprocess.run(['rustc','--edition=2021','-C','opt-level=1','-C','panic=abort','--extern','c2rust_bitfields='+str(lib[0]),'--extern','libc='+str(libc[0]),'-L','dependency='+str(deps),str(rust),'-o',str(binary)],stdout=log,stderr=subprocess.STDOUT,check=True)
 actual=subprocess.check_output([binary]);assert actual==expected,(command,actual,expected)
 results.append({'command':command,'module_sha256':digest(module),'test_source_sha256':digest(rust),'test_binary_sha256':digest(binary),'output':actual.decode(),'pass':True})
report={'scope':'Three generated Binutils modules match GCC layout, integer getters and exact packed bytes for all eight tested bit patterns across six enum bitfields. This is ABI adapter evidence, not command certification.',
 'driver_sha256':digest(Path(__file__)),'c_source_sha256':digest(native),'bfd_header_sha256':digest(ROOT/'build/gnu-binutils/bfd/bfd.h'),'gcc_arguments':args,'native_binary_sha256':digest(stage/'native'),'passed':len(results),'total':len(results),'results':results}
(ROOT/'evidence/binutils-enum-abi.json').write_text(json.dumps(report,indent=2)+'\n')
print('Three Binutils enum bitfield layouts match pinned GNU C headers')
