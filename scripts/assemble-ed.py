#!/usr/bin/env python3
"""Add the translated GNU Ed entry and its own native helpers to rboxc."""
# SPDX-License-Identifier: GPL-3.0-or-later
import json
from pathlib import Path
import re
import subprocess
from ed_helpers import fingerprint, native_inputs, prepare_archive, symbol_map

ROOT = Path(__file__).resolve().parents[1]
translation = json.loads((ROOT/'evidence/ed-translation.json').read_text())
assert translation['translated'] and translation['entry'] == 'single_binary_main_ed'
assert fingerprint(ROOT/translation['rust_file']) == translation['rust_sha256']
mapping = symbol_map(ROOT)
assert all(mapping.get(name) == symbol for name, symbol in translation['helper_imports'].items())
archive, definitions = prepare_archive(ROOT, mapping)
# All seven non-entry objects are private; the native main is excluded.
makefile = ROOT/'build/ed-print-link.mk'
makefile.write_text(".PHONY: rboxc-ed-link\nrboxc-ed-link:\n\t@printf '%s\\n' $(LDFLAGS)\n")
inputs = subprocess.check_output(['make', '--no-print-directory', '-s', '-f', 'Makefile',
    '-f', makefile, 'rboxc-ed-link'], cwd=ROOT/'build/gnu-ed', text=True).split()
assert not inputs, 'unreviewed additional libraries'
extra = [str(archive)]
link_file = ROOT/'build/rust-link-inputs.txt'
link = link_file.read_text().splitlines()
if str(archive) in link:
    assert link[-len(extra):] == extra
    link = link[:-len(extra)]
link_file.write_text('\n'.join(link+extra)+'\n')
registry = ROOT/'src/registry.rs'
text = registry.read_text()
module = '#[path = "generated/applet_ed.rs"]\nmod applet_ed;\n'
if module not in text:
    text = module+text
entry = '    (b"ed", applet_ed::single_binary_main_ed),\n'
if entry not in text:
    assert text.endswith('];\n') and text.count('static APPLETS:') == 1
    text = text[:-3]+entry+'];\n'
head, body = text.split('static APPLETS: &[(&[u8], Entry)] = &[\n')
assert body.endswith('];\n')
entries = body[:-3].splitlines(keepends=True)
assert all(re.fullmatch(r'    \(b"[^"]+", [^\n]+\),\n', line) for line in entries)
text = head+'static APPLETS: &[(&[u8], Entry)] = &[\n'+''.join(sorted(entries))+'];\n'
registry.write_text(text)
report = {'provider': 'ed', 'entry': translation['entry'], 'native_command_entries': [],
          'scope': 'Native helper definitions are renamed; no native main is linked.',
          'rust_source_sha256': translation['rust_sha256'],
          'original_inputs': {str(p.relative_to(ROOT)): fingerprint(p) for p in native_inputs(ROOT)},
          'rust_exports': translation['rust_exports'],
          'helper_archive': str(archive.relative_to(ROOT)), 'helper_archive_sha256': fingerprint(archive),
          'symbol_map_sha256': fingerprint(definitions), 'renamed_helper_symbols': len(mapping),
          'link_inputs': extra}
(ROOT/'evidence/ed-link.json').write_text(json.dumps(report, indent=2)+'\n')
print('Assembled GNU Ed with', len(mapping), 'namespaced helper symbols and no C main')
