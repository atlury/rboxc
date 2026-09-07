#!/usr/bin/env python3
"""Translate the pinned GNU Which entry and namespace its helper imports."""
# SPDX-License-Identifier: GPL-3.0-or-later
import json
import os
from pathlib import Path
import re
import subprocess
from which_helpers import fingerprint, symbol_map

ROOT = Path(__file__).resolve().parents[1]
pin = json.loads((ROOT/'inventory/sources.json').read_text())['which']
source = Path(os.environ.get('GNU_WHICH_SOURCE', pin['source']))/'which.c'
assert fingerprint(source) == pin['entry_sha256']
records = [json.loads(p.read_text()) for p in (ROOT/'build/which-cc-records').glob('*.json')]
records = [r for r in records if r['output'] == 'which.o']
assert len(records) == 1
command = records[0]
assert Path(command['file']) == source
command['arguments'][0] = 'clang-21'
command['arguments'] += ['-std=gnu17', '-Dunreachable()=__builtin_unreachable()',
                         '-Dalignof=_Alignof', '-Dmain=single_binary_main_which']
stage = ROOT/'build/translation/which'
stage.mkdir(parents=True, exist_ok=True)
database = stage/'compile_commands.json'
database.write_text(json.dumps([command], indent=2)+'\n')
log = ROOT/'evidence/raw/transpile-which.log'
tool = os.environ.get('C2RUST', str(ROOT/'.tools/c2rust-build/release/c2rust'))
with log.open('w') as output:
    subprocess.run([tool, 'transpile', '--emit-modules', '--emit-no-std', '--fail-on-error',
                    '--overwrite-existing', '--output-dir', stage/'out', database],
                   env={**os.environ, 'RUSTUP_TOOLCHAIN': '1.93.0'},
                   stdout=output, stderr=subprocess.STDOUT, check=True)
outputs = list((stage/'out').rglob('*.rs'))
assert len(outputs) == 1
text = outputs[0].read_text()
opaque = re.findall(r'^    pub type (\w+);$', text, re.M)
text = re.sub(r'^    pub type \w+;\n', '', text, flags=re.M)
declarations = ''.join(f'#[repr(C)]\npub struct {name} {{ _opaque: [u8; 0] }}\n' for name in opaque)
# Alias/function records and the final explicit-path buffer are owned until
# the final command lookup. Release them only after their last use.
anchor = '#[no_mangle]\npub unsafe extern "C" fn single_binary_main_which('
assert text.count(anchor) == 1
text = text.replace(anchor, 'unsafe extern "C" fn rboxc_which_release_owned() {\n    for i in 0..alias_count {\n        free((*aliases.offset(i as isize)).cast());\n    }\n    free(aliases.cast());\n    aliases = ::core::ptr::null_mut();\n    alias_count = 0;\n    for i in 0..func_count {\n        let function = functions.offset(i as isize);\n        free((*function).name.cast());\n        for j in 0..(*function).line_count {\n            free((*(*function).lines.offset(j as isize)).cast());\n        }\n        free((*function).lines.cast());\n    }\n    free(functions.cast());\n    functions = ::core::ptr::null_mut();\n    func_count = 0;\n    free(abs_path.cast());\n    abs_path = ::core::ptr::null_mut();\n}\n'+anchor, 1)
anchor = '    return fail_count;'
assert text.count(anchor) == 1
text = text.replace(anchor, '    rboxc_which_release_owned();\n'+anchor, 1)
mapping = symbol_map(ROOT)
imports = []
def namespace(match):
    name = match['name']
    if name not in mapping:
        return match[0]
    imports.append(name)
    return '    #[link_name = "'+mapping[name]+'"]\n'+match[0]
text = re.sub(r'^    (?:fn|static mut) (?P<name>\w+)\b', namespace, text, flags=re.M)
assert imports and 'tilde_expand' in imports and 'file_status' in imports
exports = []
def export(match):
    name = match['name']
    if name == 'single_binary_main_which':
        return match[0]
    assert name in mapping
    exports.append(name)
    return match[0].replace('#[no_mangle]', '#[export_name = "'+mapping[name]+'"]')
text = re.sub(r'#\[no_mangle\]\npub unsafe extern "C" fn (?P<name>\w+)\b', export, text)
assert set(exports) == {'func_search', 'path_search', 'process_alias', 'xmalloc', 'xrealloc'}
notice = re.match(r'\s*(/\*.*?\*/)', source.read_text(), re.S)[1]
target = ROOT/'src/generated/applet_which.rs'
target.write_text('// Generated from pinned GNU Which '+pin['version']+' by scripts/translate-which.py.\n'
                  '// Source SHA-256: '+pin['entry_sha256']+'\n'+notice+'\n'+declarations+text)
report = {'provider': 'which', 'version': pin['version'], 'command': 'which',
          'entry': 'single_binary_main_which', 'source': str(source),
          'source_sha256': pin['entry_sha256'], 'translated': True,
          'scope': 'C2Rust entry translation with namespaced native helper imports; compilation and suite validation are separate.',
          'rust_file': str(target.relative_to(ROOT)), 'rust_sha256': fingerprint(target),
          'raw_translation_sha256': fingerprint(outputs[0]), 'compile_database_sha256': fingerprint(database),
          'helper_imports': {name: mapping[name] for name in sorted(imports)},
          'rust_exports': {name: mapping[name] for name in sorted(exports)},
          'adaptations': ['Namespace helper imports and Rust-owned definitions together.',
                          'Release alias/function records and the final explicit-path buffer after all lookups.'],
          'opaque_pointer_types': opaque, 'log': str(log.relative_to(ROOT)), 'log_sha256': fingerprint(log)}
(ROOT/'evidence/which-translation.json').write_text(json.dumps(report, indent=2)+'\n')
print('Translated GNU Which entry with', len(imports), 'namespaced helper imports')
