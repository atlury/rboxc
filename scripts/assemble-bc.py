#!/usr/bin/env python3
"""Link two BC Rust entries with isolated provider helpers and state."""
# SPDX-License-Identifier: GPL-3.0-or-later
import json
from pathlib import Path
import re
import subprocess
from bc_helpers import COMMANDS, fingerprint, native_inputs, prepare_archives, symbol_map

ROOT = Path(__file__).resolve().parents[1]
mappings = {}; outputs = []; definitions = {}
translations = {}
for name in COMMANDS:
    mapping = symbol_map(ROOT, name)
    mappings[name] = mapping
    entry = json.loads((ROOT/f'evidence/bc-{name}-translation.json').read_text())
    assert entry['translated'] and entry['entry'] == 'single_binary_main_'+name
    assert entry['rust_sha256'] == fingerprint(ROOT/entry['rust_file'])
    assert all(mapping[s] == target for s, target in {**entry['helper_imports'], **entry['rust_exports']}.items())
    translations[name] = entry
    prepared, definitions[name] = prepare_archives(ROOT, name, mapping)
    outputs.extend(prepared)
makefile = ROOT/'build/bc-print-link.mk'
makefile.write_text(".PHONY: rboxc-bc-link\nrboxc-bc-link:\n\t@printf '%s\\n' '$(LDADD) $(LIBS)'\n")
flags = []
recorded = {}
for name in COMMANDS:
    inputs = subprocess.check_output(['make', '--no-print-directory', '-s', '-f', 'Makefile',
        '-f', makefile, 'rboxc-bc-link'], cwd=ROOT/'build/gnu-bc'/name, text=True).split()
    assert inputs[0] == '../lib/libbc.a'
    assert all(flag.startswith('-') for flag in inputs[1:]), 'unreviewed link input'
    recorded[name] = inputs
    for flag in inputs[1:]:
        if flag not in flags:
            flags.append(flag)
assert set(recorded) == set(COMMANDS)
extra = [*[str(p) for p in outputs if p.suffix == '.o'],
         *[str(p) for p in outputs if p.suffix == '.a'], *flags]
link_file = ROOT/'build/rust-link-inputs.txt'
link = link_file.read_text().splitlines()
previous_path = ROOT/'evidence/bc-link.json'
if previous_path.exists():
    previous = json.loads(previous_path.read_text())['link_inputs']
    if previous[0] in link:
        assert link[-len(previous):] == previous
        link = link[:-len(previous)]
assert extra[0] not in link
link_file.write_text('\n'.join(link+extra)+'\n')
registry = ROOT/'src/registry.rs'
text = registry.read_text()
for name in COMMANDS:
    module = f'#[path = "generated/applet_{name}.rs"]\nmod applet_{name};\n'
    if module not in text:
        text = module+text
    entry = f'    (b"{name}", applet_{name}::single_binary_main_{name}),\n'
    if entry not in text:
        assert text.endswith('];\n')
        text = text[:-3]+entry+'];\n'
head, body = text.split('static APPLETS: &[(&[u8], Entry)] = &[\n')
assert body.endswith('];\n')
entries = body[:-3].splitlines(keepends=True)
assert all(re.fullmatch(r'    \(b"[^"]+", [^\n]+\),\n', line) for line in entries)
registry.write_text(head+'static APPLETS: &[(&[u8], Entry)] = &[\n'+''.join(sorted(entries))+'];\n')
report = {'provider': 'bc', 'entries': [translations[n]['entry'] for n in COMMANDS],
          'native_command_entries': [], 'scope': 'Two Rust entries; native helper symbols and Rust-owned calculator state have separate command prefixes, including independent copies of libbc.',
          'rust_source_sha256': {n: translations[n]['rust_sha256'] for n in COMMANDS},
          'original_inputs': {str(p.relative_to(ROOT)): fingerprint(p) for n in COMMANDS for p in native_inputs(ROOT,n)},
          'helper_inputs': {str(p.relative_to(ROOT)): fingerprint(p) for p in outputs},
          'symbol_map_sha256': {n:fingerprint(p) for n,p in definitions.items()}, 'namespaced_symbols': sum(len(m) for m in mappings.values()),
          'original_link_inputs': recorded, 'link_inputs': extra}
(ROOT/'evidence/bc-link.json').write_text(json.dumps(report, indent=2)+'\n')
print('Assembled two GNU BC entries with', sum(len(m) for m in mappings.values()), 'private symbols and no C main')
