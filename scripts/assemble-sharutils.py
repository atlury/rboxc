#!/usr/bin/env python3
"""Link the two translated Sharutils entries with independent native helpers."""
# SPDX-License-Identifier: GPL-3.0-or-later
import json
from pathlib import Path
import re
import subprocess
from sharutils_helpers import COMMANDS, fingerprint, native_inputs, prepare_archives, symbol_map

ROOT = Path(__file__).resolve().parents[1]
translations = {}; outputs = []; definitions = {}; mappings = {}
for command in COMMANDS:
    mapping = symbol_map(ROOT, command)
    mappings[command] = mapping
    entry = json.loads((ROOT/f'evidence/sharutils-{command}-translation.json').read_text())
    assert entry['translated'] and entry['entry'] == 'single_binary_main_'+command
    assert entry['rust_sha256'] == fingerprint(ROOT/entry['rust_file'])
    assert all(mapping[n] == target for n, target in {**entry['helper_imports'], **entry['rust_exports']}.items())
    translations[command] = entry
    prepared, definitions[command] = prepare_archives(ROOT, command, mapping)
    outputs.extend(prepared)
makefile = ROOT/'build/sharutils-print-link.mk'
makefile.write_text(".PHONY: rboxc-sharutils-link\nrboxc-sharutils-link:\n\t@printf '%s\\n' '$(LDADD) $(LIBS)'\n")
recorded = subprocess.check_output(['make', '--no-print-directory', '-s', '-f', 'Makefile',
    '-f', makefile, 'rboxc-sharutils-link'], cwd=ROOT/'build/gnu-sharutils/src', text=True).split()
assert recorded == ['../libopts/libopts.a', '../lib/libgnu.a', '../lib/libgnu.a'], 'unreviewed native link inputs'
extra = [str(p) for p in outputs if p.suffix == '.o'] + [str(p) for p in outputs if p.suffix == '.a']
link_file = ROOT/'build/rust-link-inputs.txt'
link = link_file.read_text().splitlines()
report_path = ROOT/'evidence/sharutils-link.json'
if report_path.exists():
    previous = json.loads(report_path.read_text())['link_inputs']
    if previous[0] in link:
        assert link[-len(previous):] == previous
        link = link[:-len(previous)]
assert not any(p in link for p in extra)
link_file.write_text('\n'.join(link+extra)+'\n')
registry = ROOT/'src/registry.rs'
text = registry.read_text()
for name in COMMANDS:
    module = f'#[path = "generated/applet_{name}.rs"]\nmod applet_{name};\n'
    if module not in text:
        text = module+text
    row = f'    (b"{name}", applet_{name}::single_binary_main_{name}),\n'
    if row not in text:
        assert text.endswith('];\n')
        text = text[:-3]+row+'];\n'
head, body = text.split('static APPLETS: &[(&[u8], Entry)] = &[\n')
assert body.endswith('];\n')
rows = body[:-3].splitlines(keepends=True)
assert all(re.fullmatch(r'    \(b"[^"]+", [^\n]+\),\n', row) for row in rows)
registry.write_text(head+'static APPLETS: &[(&[u8], Entry)] = &[\n'+''.join(sorted(rows))+'];\n')
report = {'provider': 'sharutils', 'entries': [translations[n]['entry'] for n in COMMANDS],
          'native_command_entries': [], 'scope': 'Two Rust entries with command-private AutoOpts/Gnulib state. Both original C main objects are excluded.',
          'rust_source_sha256': {n: translations[n]['rust_sha256'] for n in COMMANDS},
          'original_inputs': {str(p.relative_to(ROOT)): fingerprint(p) for n in COMMANDS for p in native_inputs(ROOT, n)},
          'helper_inputs': {str(p.relative_to(ROOT)): fingerprint(p) for p in outputs},
          'symbol_map_sha256': {n: fingerprint(p) for n, p in definitions.items()},
          'namespaced_symbols': {n: len(mappings[n]) for n in COMMANDS},
          'original_link_inputs': recorded, 'link_inputs': extra}
report_path.write_text(json.dumps(report, indent=2)+'\n')
print('Assembled two GNU Sharutils Rust entries without C main')
