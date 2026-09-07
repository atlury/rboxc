#!/usr/bin/env python3
"""Link GNU Sed's Rust entry with private helpers."""
# SPDX-License-Identifier: GPL-3.0-or-later
import json
from pathlib import Path
import re
import subprocess
from sed_helpers import fingerprint, native_inputs, prepare_archives, symbol_map

ROOT = Path(__file__).resolve().parents[1]
mapping = symbol_map(ROOT)
entry = json.loads((ROOT/'evidence/sed-translation.json').read_text())
assert entry['translated'] and entry['entry'] == 'single_binary_main_sed'
assert entry['rust_sha256'] == fingerprint(ROOT/entry['rust_file'])
assert all(mapping[s] == target for s, target in {**entry['helper_imports'], **entry['rust_exports']}.items())
outputs, definitions = prepare_archives(ROOT, mapping)
makefile = ROOT/'build/sed-print-link.mk'
makefile.write_text(".PHONY: rboxc-sed-link\nrboxc-sed-link:\n\t@printf '%s\\n' '$(sed_sed_LDADD) $(LIBS)'\n")
inputs = subprocess.check_output(['make', '--no-print-directory', '-s', '-f', 'Makefile',
    '-f', makefile, 'rboxc-sed-link'], cwd=ROOT/'build/gnu-sed', text=True).split()
assert inputs.count('sed/libver.a') == inputs.count('lib/libsed.a') == 1
flags = [p for p in inputs if p not in ('sed/libver.a', 'lib/libsed.a')]
assert all(flag.startswith('-') for flag in flags)
archives = {p.name.removeprefix('sed-'): p for p in outputs if p.suffix == '.a'}
extra = [*[str(p) for p in outputs if p.suffix == '.o'],
         *[str(archives[Path(p).name]) if p in ('sed/libver.a', 'lib/libsed.a') else p for p in inputs]]
link_file = ROOT/'build/rust-link-inputs.txt'
link = link_file.read_text().splitlines()
if extra[0] in link:
    assert link[-len(extra):] == extra
    link = link[:-len(extra)]
link_file.write_text('\n'.join(link+extra)+'\n')
registry = ROOT/'src/registry.rs'
text = registry.read_text()
module = '#[path = "generated/applet_sed.rs"]\nmod applet_sed;\n'
if module not in text:
    text = module+text
for command in ('sed',):
    item = f'    (b"{command}", applet_sed::single_binary_main_{command}),\n'
    if item not in text:
        assert text.endswith('];\n')
        text = text[:-3]+item+'];\n'
head, body = text.split('static APPLETS: &[(&[u8], Entry)] = &[\n')
assert body.endswith('];\n')
items = body[:-3].splitlines(keepends=True)
assert all(re.fullmatch(r'    \(b"[^"]+", [^\n]+\),\n', line) for line in items)
registry.write_text(head+'static APPLETS: &[(&[u8], Entry)] = &[\n'+''.join(sorted(items))+'];\n')
report = {'provider': 'sed', 'entry': entry['entry'], 'aliases': [],
    'native_command_entries': [], 'scope': 'C2Rust sed entry. All native helpers and Rust-owned entry state share a private provider prefix.',
    'rust_source_sha256': entry['rust_sha256'],
    'original_inputs': {str(p.relative_to(ROOT)): fingerprint(p) for p in native_inputs(ROOT)},
    'helper_inputs': {str(p.relative_to(ROOT)): fingerprint(p) for p in outputs},
    'symbol_map_sha256': fingerprint(definitions), 'namespaced_symbols': len(mapping),
    'original_link_inputs': inputs, 'link_inputs': extra}
(ROOT/'evidence/sed-link.json').write_text(json.dumps(report, indent=2)+'\n')
print('Assembled GNU Sed entry with', len(mapping), 'private symbols and no C main')
