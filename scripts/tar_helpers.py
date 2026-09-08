"""Namespace GNU Tar helper objects and Rust-owned command state together."""
# SPDX-License-Identifier: GPL-3.0-or-later
import hashlib
import json
import re
import subprocess

COMMANDS=('tar',)
ENTRY_OBJECTS={'tar':'src/tar.o'}
def fingerprint(path):return hashlib.sha256(path.read_bytes()).hexdigest()
def native_inputs(root,command='tar'):
 build=root/'build/gnu-tar'
 pin=json.loads((root/'inventory/sources.json').read_text())['tar']
 names=sorted(path.rsplit('/',1)[-1][:-2]+'.o' for path in pin['helper_source_sha256'])
 assert {p.name for p in (build/'src').glob('*.o')}==set(names)|{'tar.o'}
 return [*[build/'src'/name for name in names],build/'lib/libtar.a',build/'gnu/libgnu.a']
def defined_symbols(paths):
 output=subprocess.check_output(['nm','-g','--defined-only','--format=posix',*paths],text=True)
 return set(re.findall(r'^(\w+) [A-Z] ',output,re.M))
def symbol_map(root,name='tar'):
 symbols=defined_symbols(native_inputs(root,name));assert 'main' not in symbols
 symbols|=defined_symbols([root/'build/gnu-tar/src/tar.o'])-{'main'}
 return {symbol:'rboxc_tar_'+symbol for symbol in sorted(symbols)}
def prepare_archives(root,command,mapping):
 stage=root/'build/translation/tar';stage.mkdir(parents=True,exist_ok=True)
 definitions=stage/'helper-symbol-map';definitions.write_text(''.join(f'{old} {new}\n' for old,new in mapping.items()))
 outputs=[]
 for original in native_inputs(root):
  target=root/'build/helpers'/('tar-'+original.name);temporary=target.with_name(target.name+'.tmp')
  subprocess.run(['objcopy','--redefine-syms='+str(definitions),original,temporary],check=True)
  if original.suffix=='.a':subprocess.run(['ranlib',temporary],check=True)
  temporary.replace(target);outputs.append(target)
 assert defined_symbols(outputs)=={mapping[s] for s in defined_symbols(native_inputs(root))}
 return outputs,definitions
