#!/usr/bin/env python3
"""Link the translated GNU Tar entry with its private native helpers."""
# SPDX-License-Identifier: GPL-3.0-or-later
import json
from pathlib import Path
import re
import subprocess
from tar_helpers import fingerprint,native_inputs,prepare_archives,symbol_map
ROOT=Path(__file__).resolve().parents[1]
mapping=symbol_map(ROOT)
entry=json.loads((ROOT/'evidence/tar-translation.json').read_text())
assert entry['translated'] and entry['entry']=='single_binary_main_tar'
assert fingerprint(ROOT/entry['rust_file'])==entry['rust_sha256']
assert all(mapping[s]==v for s,v in {**entry['helper_imports'],**entry['rust_exports']}.items())
outputs,definitions=prepare_archives(ROOT,'tar',mapping)
makefile=ROOT/'build/tar-print-link.mk'
makefile.write_text(".PHONY: rboxc-tar-link\nrboxc-tar-link:\n\t@printf '%s\\n' '$(tar_LDADD) $(LIBS)'\n")
recorded=subprocess.check_output(['make','--no-print-directory','-s','-f','Makefile','-f',makefile,'rboxc-tar-link'],cwd=ROOT/'build/gnu-tar/src',text=True).split()
assert [word for word in recorded if word.endswith('.a')]==['../lib/libtar.a','../gnu/libgnu.a']
flags=[word for word in recorded if not word.endswith('.a')]
assert set(flags)<={'-lacl'},'unreviewed GNU Tar link flag'
extra=[*[str(p) for p in outputs if p.suffix=='.o'],*[str(p) for p in outputs if p.suffix=='.a'],*dict.fromkeys(flags)]
link_file=ROOT/'build/rust-link-inputs.txt';link=link_file.read_text().splitlines()
report_path=ROOT/'evidence/tar-link.json'
if report_path.exists():
 previous=json.loads(report_path.read_text())['link_inputs']
 if previous[0] in link:
  assert link[-len(previous):]==previous
  link=link[:-len(previous)]
assert extra[0] not in link
link_file.write_text('\n'.join(link+extra)+'\n')
registry=ROOT/'src/registry.rs';text=registry.read_text()
module='#[path = "generated/applet_tar.rs"]\nmod applet_tar;\n'
if module not in text:text=module+text
row='    (b"tar", applet_tar::single_binary_main_tar),\n'
if row not in text:
 assert text.endswith('];\n');text=text[:-3]+row+'];\n'
head,body=text.split('static APPLETS: &[(&[u8], Entry)] = &[\n');assert body.endswith('];\n')
rows=body[:-3].splitlines(keepends=True)
assert all(re.fullmatch(r'    \(b"[^"]+", [^\n]+\),\n',line) for line in rows)
registry.write_text(head+'static APPLETS: &[(&[u8], Entry)] = &[\n'+''.join(sorted(rows))+'];\n')
report={'provider':'tar','entries':[entry['entry']],'native_command_entries':[],
        'scope':'One Rust GNU Tar entry with private native helpers and command state. The original C tar.o entry is excluded.',
        'rust_source_sha256':entry['rust_sha256'],'original_inputs':{str(p.relative_to(ROOT)):fingerprint(p) for p in native_inputs(ROOT)},
        'helper_inputs':{str(p.relative_to(ROOT)):fingerprint(p) for p in outputs},'symbol_map_sha256':fingerprint(definitions),
        'namespaced_symbols':len(mapping),'original_link_inputs':recorded,'link_inputs':extra}
report_path.write_text(json.dumps(report,indent=2)+'\n')
print('Assembled GNU Tar with',len(mapping),'private symbols and no C main')
