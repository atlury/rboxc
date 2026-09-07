#!/usr/bin/env python3
"""Link four Diffutils Rust entries with isolated provider helpers and state."""
# SPDX-License-Identifier: GPL-3.0-or-later
import json
from pathlib import Path
import re
import subprocess
from diffutils_helpers import COMMANDS, fingerprint, native_inputs, prepare_archives, symbol_map

ROOT = Path(__file__).resolve().parents[1]
mapping = symbol_map(ROOT)
translations = {}
for name in COMMANDS:
    entry = json.loads((ROOT/f'evidence/diffutils-{name}-translation.json').read_text())
    assert entry['translated'] and entry['entry'] == 'single_binary_main_'+name
    assert entry['rust_sha256'] == fingerprint(ROOT/entry['rust_file'])
    assert all(mapping[s] == target for s, target in {**entry['helper_imports'], **entry['rust_exports']}.items())
    translations[name] = entry
outputs, definitions = prepare_archives(ROOT, mapping)
makefile = ROOT/'build/diffutils-print-link.mk'
makefile.write_text('.PHONY: rboxc-diffutils-link\nrboxc-diffutils-link:\n'+''.join(
    f"\t@printf '{name} %s\\n' '$({name}_LDADD) $(LIBS)'\n" for name in COMMANDS))
lines = subprocess.check_output(['make', '--no-print-directory', '-s', '-f', 'Makefile',
    '-f', makefile, 'rboxc-diffutils-link'], cwd=ROOT/'build/gnu-diffutils/src', text=True).splitlines()
flags = []
recorded = {}
for line in lines:
    name, *inputs = line.split()
    assert name in COMMANDS and inputs[:2] == ['libver.a', '../lib/libdiffutils.a']
    assert all(flag.startswith('-') for flag in inputs[2:]), 'unreviewed link input'
    recorded[name] = inputs
    for flag in inputs[2:]:
        if flag not in flags:
            flags.append(flag)
assert set(recorded) == set(COMMANDS)
extra = [*[str(p) for p in outputs if p.suffix == '.o'],
         *[str(p) for p in outputs if p.suffix == '.a'], *flags]
link_file = ROOT/'build/rust-link-inputs.txt'
link = link_file.read_text().splitlines()
if extra[0] in link:
    assert link[-len(extra):] == extra
    link = link[:-len(extra)]
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
report = {'provider': 'diffutils', 'entries': [translations[n]['entry'] for n in COMMANDS],
          'native_command_entries': [], 'scope': 'Four Rust entries; all native helper symbols and Rust-owned diff state share a private provider prefix.',
          'rust_source_sha256': {n: translations[n]['rust_sha256'] for n in COMMANDS},
          'original_inputs': {str(p.relative_to(ROOT)): fingerprint(p) for p in native_inputs(ROOT)},
          'helper_inputs': {str(p.relative_to(ROOT)): fingerprint(p) for p in outputs},
          'symbol_map_sha256': fingerprint(definitions), 'namespaced_symbols': len(mapping),
          'original_link_inputs': recorded, 'link_inputs': extra}
(ROOT/'evidence/diffutils-link.json').write_text(json.dumps(report, indent=2)+'\n')
print('Assembled four GNU Diffutils entries with', len(mapping), 'private symbols and no C main')
