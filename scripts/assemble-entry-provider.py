#!/usr/bin/env python3
"""Append translated GNU entries and their isolated native helper objects."""
# SPDX-License-Identifier: GPL-3.0-or-later
import argparse
import json
from pathlib import Path
import re
from entry_provider_helpers import ENTRY_OBJECTS,PROVIDERS,fingerprint,native_inputs,original_link,local_libraries,prepare_archives,symbol_map

ROOT=Path(__file__).resolve().parents[1]
parser=argparse.ArgumentParser(description=__doc__)
parser.add_argument('provider',choices=sorted(set(ENTRY_OBJECTS)-{'bash'}))
name=parser.parse_args().provider
provider=PROVIDERS[name]
pin=json.loads((ROOT/'inventory/sources.json').read_text())[provider]
commands=pin['commands'] if name==provider else [name]
entry=json.loads((ROOT/f'evidence/{name}-translation.json').read_text())
assert entry['translated'] and fingerprint(ROOT/entry['rust_file'])==entry['rust_sha256']
mapping=symbol_map(ROOT,name)
assert all(mapping[s]==target for s,target in {**entry['helper_imports'],**entry['rust_exports']}.items())
outputs,definitions=prepare_archives(ROOT,name,mapping)
record=original_link(ROOT,name)
local=local_libraries(ROOT,name)
flags=[word for word in record['arguments'] if word.startswith(('-l','-L','-Wl,')) and word not in local]
expected={'gawk':['-lreadline','-lm'],'patch':['-lattr'],'less':['-ltinfo'],
    'wget':['-lpcre2-8','-lssl','-lcrypto','-lz'],'screen':['-lcrypt','-lcurses'],
    'dnsdomainname':['-lutil'],'logger':['-lutil'],'inetd':['-lutil'],
    'syslogd':['-lutil'],'tftpd':['-lutil'],'traceroute':['-lutil'],
    'ping':[],'ping6':[],'ifconfig':[],'telnetd':['-ltermcap','-lutil','-lcrypt']}
if provider=='binutils':
    assert '-lz' in local, 'retain the configured bundled zlib'
    assert [f for f in flags if not f.startswith('-L')]==['-lzstd']
    assert all(Path(f[2:]).resolve().is_relative_to(ROOT/'build/gnu-binutils') for f in flags if f.startswith('-L'))
else:
    assert flags==expected[name], 'review changed GNU link flags'
extra=[str(p) for p in outputs]+flags
link_file=ROOT/'build/rust-link-inputs.txt';link=link_file.read_text().splitlines()
report_path=ROOT/f'evidence/{name}-link.json'
if report_path.exists():
    previous=json.loads(report_path.read_text())['link_inputs']
    if previous[0] in link:
        start=link.index(previous[0])
        assert link[start:start+len(previous)]==previous
        del link[start:start+len(previous)]
assert not any(p in link for p in map(str,outputs))
link_file.write_text('\n'.join(link+extra)+'\n')
registry=ROOT/'src/registry.rs';text=registry.read_text()
module=f'#[path = "generated/applet_{name}.rs"]\nmod applet_{name};\n'
if module not in text:text=module+text
for command in commands:
    row=f'    (b"{command}", applet_{name}::single_binary_main_{name}),\n'
    if row not in text:
        assert text.endswith('];\n');text=text[:-3]+row+'];\n'
head,body=text.split('static APPLETS: &[(&[u8], Entry)] = &[\n');assert body.endswith('];\n')
rows=body[:-3].splitlines(keepends=True)
assert all(re.fullmatch(r'    \(b"[^"]+", [^\n]+\),\n',row) for row in rows)
registry.write_text(head+'static APPLETS: &[(&[u8], Entry)] = &[\n'+''.join(sorted(rows))+'];\n')
report={'provider':provider,'entries':[entry['entry']],'commands':commands,'native_command_entries':[],
    'scope':'Translated Rust entry with original GNU aliases and isolated native helpers; original C entry object excluded. Runtime validation is separate.',
    'rust_source_sha256':entry['rust_sha256'],
    'original_inputs':{str(p.relative_to(ROOT)):fingerprint(p) for p in native_inputs(ROOT,name)},
    'helper_inputs':{str(p.relative_to(ROOT)):fingerprint(p) for p in outputs},
    'namespaced_symbols':len(mapping),'symbol_map_sha256':fingerprint(definitions),
    'original_link_inputs':record['arguments'],'link_inputs':extra}
report_path.write_text(json.dumps(report,indent=2)+'\n')
print('Assembled GNU',name,'for',len(commands),'names with',len(mapping),'private symbols')
