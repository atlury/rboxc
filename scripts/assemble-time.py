#!/usr/bin/env python3
"""Add the translated GNU Time entry and its own native helpers to rboxc."""
# SPDX-License-Identifier: GPL-3.0-or-later
import json
from pathlib import Path
import re
import subprocess
from time_helpers import fingerprint, native_inputs, prepare_archives, symbol_map

ROOT = Path(__file__).resolve().parents[1]
translation = json.loads((ROOT/'evidence/time-translation.json').read_text())
assert translation['translated'] and translation['entry'] == 'single_binary_main_time'
assert fingerprint(ROOT/translation['rust_file']) == translation['rust_sha256']
mapping = symbol_map(ROOT)
assert all(mapping.get(name) == symbol for name, symbol in translation['helper_imports'].items())
archives, definitions = prepare_archives(ROOT, mapping)
makefile = ROOT/'build/time-print-link.mk'
makefile.write_text(".PHONY: rboxc-time-link\nrboxc-time-link:\n\t@printf '%s\\n' $(src_time_LDADD)\n")
inputs = subprocess.check_output(['make', '--no-print-directory', '-s', '-f', 'Makefile',
    '-f', makefile, 'rboxc-time-link'], cwd=ROOT/'build/gnu-time', text=True).splitlines()
assert inputs[:2] == ['src/libver.a', 'lib/libtime.a']
assert all(p.startswith('-') for p in inputs[2:]), 'unreviewed helper input'
extra = [*[str(p) for p in archives], *inputs[2:]]
link_file = ROOT/'build/rust-link-inputs.txt'
link = link_file.read_text().splitlines()
if str(archives[0]) in link:
    assert link[-len(extra):] == extra
    link = link[:-len(extra)]
link_file.write_text('\n'.join(link+extra)+'\n')
registry = ROOT/'src/registry.rs'
text = registry.read_text()
module = '#[path = "generated/applet_time.rs"]\nmod applet_time;\n'
if module not in text:
    text = module+text
entry = '    (b"time", applet_time::single_binary_main_time),\n'
if entry not in text:
    assert text.endswith('];\n') and text.count('static APPLETS:') == 1
    text = text[:-3]+entry+'];\n'
head, body = text.split('static APPLETS: &[(&[u8], Entry)] = &[\n')
assert body.endswith('];\n')
entries = body[:-3].splitlines(keepends=True)
assert all(re.fullmatch(r'    \(b"[^"]+", [^\n]+\),\n', line) for line in entries)
registry.write_text(head+'static APPLETS: &[(&[u8], Entry)] = &[\n'+''.join(sorted(entries))+'];\n')
report = {'provider': 'time', 'entry': translation['entry'], 'native_command_entries': [],
          'scope': 'Native helper definitions are renamed; no native main is linked.',
          'rust_source_sha256': translation['rust_sha256'],
          'original_inputs': {str(p.relative_to(ROOT)): fingerprint(p) for p in native_inputs(ROOT)},
          'helper_inputs': {str(p.relative_to(ROOT)): fingerprint(p) for p in archives},
          'symbol_map_sha256': fingerprint(definitions), 'renamed_helper_symbols': len(mapping),
          'link_inputs': extra}
(ROOT/'evidence/time-link.json').write_text(json.dumps(report, indent=2)+'\n')
print('Assembled GNU Time with', len(mapping), 'namespaced helper symbols and no C main')
