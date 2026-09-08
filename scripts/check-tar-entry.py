#!/usr/bin/env python3
"""Type-check the translated Tar entry without changing the multicall registry."""
# SPDX-License-Identifier: GPL-3.0-or-later
import json
from pathlib import Path
import subprocess
import tomllib
from tar_helpers import fingerprint
ROOT=Path(__file__).resolve().parents[1]
entry=json.loads((ROOT/'evidence/tar-translation.json').read_text())
source=ROOT/entry['rust_file'];assert fingerprint(source)==entry['rust_sha256']
crate=ROOT/'build/tar-entry-check';crate.mkdir(exist_ok=True)
dependencies=tomllib.loads((ROOT/'Cargo.toml').read_text())['dependencies']
versions={p['name']:p['version'] for p in tomllib.loads((ROOT/'Cargo.lock').read_text())['package']}
(crate/'Cargo.toml').write_text('[package]\nname = "rboxc-tar-entry-check"\nversion = "0.0.0"\nedition = "2021"\npublish = false\n[lib]\npath = "lib.rs"\n[dependencies]\n'+''.join(name+' = '+json.dumps('='+versions[name])+'\n' for name in dependencies))
(crate/'lib.rs').write_text('#![feature(c_variadic)]\n#![allow(dead_code, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut, unused_variables)]\n#[path = '+json.dumps(str(source))+']\nmod applet_tar;\n')
log=ROOT/'evidence/raw/tar-entry-check.log'
with log.open('w') as output:
 result=subprocess.run(['cargo','+nightly-2026-01-22','check','--offline','--manifest-path',str(crate/'Cargo.toml'),'--target-dir',str(ROOT/'target/tar-entry-check')],stdout=output,stderr=subprocess.STDOUT)
assert fingerprint(source)==entry['rust_sha256']
report={'scope':'Isolated Rust type check of the translated GNU Tar entry. No linking or runtime equivalence is claimed, and the multicall registry remains unchanged.',
        'passed':int(result.returncode==0),'total':1,'rust_sha256':entry['rust_sha256'],'driver_sha256':fingerprint(Path(__file__)),
        'root_lock_sha256':fingerprint(ROOT/'Cargo.lock'),'manifest_sha256':fingerprint(crate/'Cargo.toml'),'log':str(log.relative_to(ROOT)),'log_sha256':fingerprint(log)}
(ROOT/'evidence/tar-entry-check.json').write_text(json.dumps(report,indent=2)+'\n')
print('PASS' if result.returncode==0 else 'OPEN','Tar entry type check')
raise SystemExit(result.returncode)
