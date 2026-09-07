#!/usr/bin/env python3
"""Add the translated GNU Hello entry and its own native helpers to rboxc."""
# SPDX-License-Identifier: GPL-3.0-or-later
import json
from pathlib import Path
import re
import subprocess
from hello_helpers import fingerprint, prepare_archive, symbol_map

ROOT = Path(__file__).resolve().parents[1]
translation = json.loads((ROOT/'evidence/hello-translation.json').read_text())
assert translation['translated'] and translation['entry'] == 'single_binary_main_hello'
assert fingerprint(ROOT/translation['rust_file']) == translation['rust_sha256']
mapping = symbol_map(ROOT)
assert all(mapping.get(name) == symbol for name, symbol in translation['helper_imports'].items())
archive, definitions = prepare_archive(ROOT, mapping)
makefile = ROOT/'build/hello-print-link.mk'
makefile.write_text(".PHONY: rboxc-hello-link\nrboxc-hello-link:\n\t@printf '%s\\n' $(hello_LDADD)\n")
inputs = subprocess.check_output(['make', '--no-print-directory', '-s', '-f', 'Makefile',
    '-f', makefile, 'rboxc-hello-link'], cwd=ROOT/'build/gnu-hello', text=True).splitlines()
assert inputs.count('lib/libhello.a') == 1
extra = [str(archive) if p == 'lib/libhello.a' else p for p in inputs]
assert all(p.startswith('-') or p == str(archive) for p in extra), 'unreviewed helper input'
link_file = ROOT/'build/rust-link-inputs.txt'
link = link_file.read_text().splitlines()
if str(archive) in link:
    assert link[-len(extra):] == extra
    link = link[:-len(extra)]
link_file.write_text('\n'.join(link+extra)+'\n')
registry = ROOT/'src/registry.rs'
text = registry.read_text()
module = '#[path = "generated/applet_hello.rs"]\nmod applet_hello;\n'
if module not in text:
    text = module+text
entry = '    (b"hello", applet_hello::single_binary_main_hello),\n'
if entry not in text:
    assert text.endswith('];\n') and text.count('static APPLETS:') == 1
    text = text[:-3]+entry+'];\n'
head, body = text.split('static APPLETS: &[(&[u8], Entry)] = &[\n')
assert body.endswith('];\n')
entries = body[:-3].splitlines(keepends=True)
assert all(re.fullmatch(r'    \(b"[^"]+", [^\n]+\),\n', line) for line in entries)
text = head+'static APPLETS: &[(&[u8], Entry)] = &[\n'+''.join(sorted(entries))+'];\n'
registry.write_text(text)
report = {'provider': 'hello', 'entry': translation['entry'], 'native_command_entries': [],
          'scope': 'Native helper definitions are renamed; no native main is linked.',
          'rust_source_sha256': translation['rust_sha256'],
          'original_archive_sha256': fingerprint(ROOT/'build/gnu-hello/lib/libhello.a'),
          'helper_archive': str(archive.relative_to(ROOT)), 'helper_archive_sha256': fingerprint(archive),
          'symbol_map_sha256': fingerprint(definitions), 'renamed_helper_symbols': len(mapping),
          'link_inputs': extra}
(ROOT/'evidence/hello-link.json').write_text(json.dumps(report, indent=2)+'\n')
print('Assembled GNU Hello with', len(mapping), 'namespaced helper symbols and no C main')
