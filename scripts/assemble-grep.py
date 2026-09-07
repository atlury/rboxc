#!/usr/bin/env python3
"""Link GNU Grep's Rust entry and two internal aliases with private helpers."""
# SPDX-License-Identifier: GPL-3.0-or-later
import json
from pathlib import Path
import re
import subprocess
from grep_helpers import fingerprint, native_inputs, prepare_archives, symbol_map

ROOT = Path(__file__).resolve().parents[1]
mapping = symbol_map(ROOT)
entry = json.loads((ROOT/'evidence/grep-translation.json').read_text())
assert entry['translated'] and entry['entry'] == 'single_binary_main_grep'
assert entry['rust_sha256'] == fingerprint(ROOT/entry['rust_file'])
assert all(mapping[s] == target for s, target in {**entry['helper_imports'], **entry['rust_exports']}.items())
outputs, definitions = prepare_archives(ROOT, mapping)
makefile = ROOT/'build/grep-print-link.mk'
makefile.write_text(".PHONY: rboxc-grep-link\nrboxc-grep-link:\n\t@printf '%s\\n' '$(grep_LDADD) $(LIBS)'\n")
inputs = subprocess.check_output(['make', '--no-print-directory', '-s', '-f', 'Makefile',
    '-f', makefile, 'rboxc-grep-link'], cwd=ROOT/'build/gnu-grep/src', text=True).split()
assert inputs.count('../lib/libgreputils.a') == 2
flags = [p for p in inputs if p != '../lib/libgreputils.a']
assert all(flag.startswith('-') for flag in flags)
assert '-lpcre2-8' in flags
archive = next(p for p in outputs if p.suffix == '.a')
extra = [*[str(p) for p in outputs if p.suffix == '.o'],
         *[str(archive) if p == '../lib/libgreputils.a' else p for p in inputs]]
link_file = ROOT/'build/rust-link-inputs.txt'
link = link_file.read_text().splitlines()
if extra[0] in link:
    assert link[-len(extra):] == extra
    link = link[:-len(extra)]
link_file.write_text('\n'.join(link+extra)+'\n')
registry = ROOT/'src/registry.rs'
text = registry.read_text()
module = '#[path = "generated/applet_grep.rs"]\nmod applet_grep;\n'
if module not in text:
    text = module+text
for command in ('grep', 'egrep', 'fgrep'):
    item = f'    (b"{command}", applet_grep::single_binary_main_{command}),\n'
    if item not in text:
        assert text.endswith('];\n')
        text = text[:-3]+item+'];\n'
head, body = text.split('static APPLETS: &[(&[u8], Entry)] = &[\n')
assert body.endswith('];\n')
items = body[:-3].splitlines(keepends=True)
assert all(re.fullmatch(r'    \(b"[^"]+", [^\n]+\),\n', line) for line in items)
registry.write_text(head+'static APPLETS: &[(&[u8], Entry)] = &[\n'+''.join(sorted(items))+'];\n')
report = {'provider': 'grep', 'entry': entry['entry'], 'aliases': ['egrep', 'fgrep'],
    'native_command_entries': [], 'scope': 'C2Rust grep entry and two Rust adaptations of upstream shell aliases. All native helpers and Rust-owned matcher state share a private provider prefix.',
    'rust_source_sha256': entry['rust_sha256'],
    'original_inputs': {str(p.relative_to(ROOT)): fingerprint(p) for p in native_inputs(ROOT)},
    'helper_inputs': {str(p.relative_to(ROOT)): fingerprint(p) for p in outputs},
    'symbol_map_sha256': fingerprint(definitions), 'namespaced_symbols': len(mapping),
    'original_link_inputs': inputs, 'link_inputs': extra}
(ROOT/'evidence/grep-link.json').write_text(json.dumps(report, indent=2)+'\n')
print('Assembled GNU Grep and two aliases with', len(mapping), 'private symbols and no C main')
