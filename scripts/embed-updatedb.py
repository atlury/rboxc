#!/usr/bin/env python3
"""Embed GNU's pinned shell program with explicit multicall path defaults."""
# SPDX-License-Identifier: GPL-3.0-or-later
import hashlib,json
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
pin=json.loads((ROOT/'inventory/sources.json').read_text())['findutils']
source=Path(pin['source'])/'locate/updatedb.sh'
assert sha(source)==pin['shell_program_sha256']['updatedb']
# This is exactly the GNU configure substitution, with relocatable commands.
s=source.read_text()
substitutions={'@COPYRIGHT@':'Copyright (C) 1994-2026 Free Software Foundation, Inc.',
 '@PACKAGE_URL@':'https://www.gnu.org/software/findutils/',
 '@PACKAGE_BUGREPORT_URL@':'https://savannah.gnu.org/bugs/?group=findutils',
 '@PACKAGE_BUGREPORT@':'bug-findutils@gnu.org','@find@':'find','@frcode@':'frcode',
 '@PACKAGE@':'findutils','@VERSION@':pin['version'],'@PACKAGE_NAME@':'GNU findutils',
 '@PACKAGE_VERSION@':pin['version'],'@PACKAGE_STRING@':'GNU findutils '+pin['version'],
 '@SORT_SUPPORTS_Z@':'true','@SORT@':'sort','@LOCATE_DB@':'/var/lib/rboxc/locatedb',
 '@bindir@':'/usr/bin','@libexecdir@':'/usr/libexec'}
for k,v in substitutions.items():s=s.replace(k,v)
import re
assert not re.findall(r'@[A-Za-z_]+@',s),re.findall(r'@[A-Za-z_]+@',s)
target=ROOT/'src/generated/updatedb.sh';target.write_text(s)
p=ROOT/'src/registry.rs';text=p.read_text()
if 'mod updatedb;' not in text:text='mod updatedb;\n'+text
row='    (b"updatedb", updatedb::main),\n'
head,body=text.split('static APPLETS: &[(&[u8], Entry)] = &[\n')
rows=body[:-3].splitlines(keepends=True)
if row not in rows:rows.append(row)
p.write_text(head+'static APPLETS: &[(&[u8], Entry)] = &[\n'+''.join(sorted(rows))+'];\n')
(ROOT/'evidence/updatedb-integration.json').write_text(json.dumps({
 'provider':'findutils','version':pin['version'],'source':str(source),'source_sha256':sha(source),
 'script':str(target.relative_to(ROOT)),'script_sha256':sha(target),'substitutions':substitutions,
 'driver_sha256':sha(Path(__file__)), 'commands':['updatedb'],'private_commands':['frcode'],
 'strategy':'Pinned GNU shell program interpreted by the linked C2Rust Bash entry; frcode is separately C2Rust translated. Private command symlinks invoke this executable. Explicit user command-directory and shell overrides retain GNU semantics.',
 'limitations':['Default database parent /var/lib/rboxc must already exist; no privileged installation is implicit.','Native user-switch helper su remains external; user-switch and network filesystem profiles are not certified.','GNU shell script logic is retained, not claimed as C2Rust-generated Rust.']},indent=2)+'\n')
