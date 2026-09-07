#!/usr/bin/env python3
"""Translate the pinned GNU Grep entry and namespace its helper imports."""
# SPDX-License-Identifier: GPL-3.0-or-later
import json
import os
from pathlib import Path
import re
import subprocess
from grep_helpers import COMMANDS, defined_symbols, fingerprint, symbol_map

ROOT = Path(__file__).resolve().parents[1]
import argparse
parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('command', choices=COMMANDS)
name = parser.parse_args().command
pin = json.loads((ROOT/'inventory/sources.json').read_text())['grep']
source = Path(os.environ.get('GNU_DIFFUTILS_SOURCE', pin['source']))/f'src/{name}.c'
expected = pin['entry_sha256']
assert fingerprint(source) == expected
records = [json.loads(p.read_text()) for p in (ROOT/'build/grep-cc-records').glob('*.json')]
records = [r for r in records if r['output'] == name+'.o']
assert len(records) == 1
command = records[0]
assert Path(command['file']) == source
command['arguments'][0] = 'clang-21'
command['arguments'] += ['-std=gnu17', '-Dunreachable()=__builtin_unreachable()',
                         '-Dalignof=_Alignof', '-Dnullptr=((void*)0)', '-Dmain=single_binary_main_'+name]
stage = ROOT/'build/translation/grep'/name
stage.mkdir(parents=True, exist_ok=True)
database = stage/'compile_commands.json'
database.write_text(json.dumps([command], indent=2)+'\n')
log = ROOT/f'evidence/raw/transpile-grep-{name}.log'
tool = os.environ.get('C2RUST', str(ROOT/'.tools/c2rust-build/release/c2rust'))
with log.open('w') as output:
    subprocess.run([tool, 'transpile', '--emit-modules', '--emit-no-std', '--fail-on-error',
                    '--overwrite-existing', '--output-dir', stage/'out', database],
                   env={**os.environ, 'RUSTUP_TOOLCHAIN': '1.93.0'},
                   stdout=output, stderr=subprocess.STDOUT, check=True)
outputs = list((stage/'out').rglob('*.rs'))
assert len(outputs) == 1
text = outputs[0].read_text()
# Same pinned-nightly ABI/API spellings used by the Coreutils translator.
text = text.replace('::core::ffi::VaListImpl', '::core::ffi::VaList')
text = text.replace('.as_va_list()', '')
text = text.replace('::core::ptr::from_exposed_addr_mut', '::core::ptr::with_exposed_provenance_mut')
text = text.replace('.expose_addr()', '.expose_provenance()')

opaque = re.findall(r'^    pub type (\w+);$', text, re.M)
text = re.sub(r'^    pub type \w+;\n', '', text, flags=re.M)
# External is the default linkage for an exported Rust definition.
assert set(re.findall(r'#\[linkage = "([^"]+)"\]', text)) <= {'external'}
text = text.replace('#[linkage = "external"]\n', '')
declarations = ''.join(f'#[repr(C)]\npub struct {name} {{ _opaque: [u8; 0] }}\n' for name in opaque)
# Match the provider's glibc pathname diagnostics through the public
# callback used by Coreutils' Gnulib error implementation.
anchor = '#[no_mangle]\npub unsafe extern "C" fn single_binary_main_'+name+'('
helper = (ROOT/'src/bridges/grep-invocation.rs').read_text()+'\n'
assert text.count(anchor) == 1
text = text.replace(anchor, helper+anchor, 1)
anchor = '    let mut keys: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();'
assert text.count(anchor) == 1
text = text.replace(anchor, '    rboxc_grep_set_invocation(*argv);\n'+anchor, 1)
anchor = '    q = xstrdup(p);'
assert text.count(anchor) == 1
text = text.replace(anchor, anchor+'\n    RBOXC_GREP_COLORS = q;', 1)
anchor = 'unsafe extern "C" fn grepdesc(mut desc: ::core::ffi::c_int, mut command_line: bool) -> bool {'
assert text.count(anchor) == 1
text = text.replace(anchor, anchor+'\n    RBOXC_GREP_INPUT = desc;', 1)
assert text.count('close(desc)') == 2
text = text.replace('close(desc)', 'rboxc_grep_close_input(desc)')
anchor = '                loop {\n                    ent = rpl_fts_read(fts);'
assert text.count(anchor) == 1
text = text.replace(anchor, '                rboxc_grep_track_tree(fts);\n'+anchor, 1)
assert text.count('rpl_fts_close(fts)') == 1
text = text.replace('rpl_fts_close(fts)', 'rboxc_grep_close_tree(fts)', 1)
anchor = 'ximalloc(bufalloc)'
assert text.count(anchor) == 1
text = text.replace(anchor, 'rboxc_grep_allocate_buffer(bufalloc)', 1)
anchor = '            newbuf = xpalloc(nullptr, &raw mut bufalloc, incr_min, alloc_max, 1 as idx_t)\n                as *mut ::core::ffi::c_char;'
assert text.count(anchor) == 1
text = text.replace(anchor, anchor+'\n            libc::memset(newbuf.cast(), 0, bufalloc as usize);', 1)
text += '\n'+(ROOT/'src/bridges/grep-owned.rs').read_text()+'\n'
anchor = '    atexit(Some(clean_up_stdout as unsafe extern "C" fn() -> ()));'
assert text.count(anchor) == 1
text = text.replace(anchor, anchor+'\n    atexit(Some(rboxc_grep_release_owned));', 1)
anchor = '    compiled_pattern = matchers[matcher as usize]'
assert text.count(anchor) == 1
text = text.replace(anchor, '    let compiler_address = matchers[matcher as usize].compile.map(|function| function as usize);\n    RBOXC_GREP_MATCHER_FREE = if compiler_address == Some(Fcompile as *const () as usize) {\n        Some(Ffree)\n    } else if compiler_address == Some(GEAcompile as *const () as usize) {\n        Some(GEAfree)\n    } else { None };\n'+anchor, 1)
mapping = symbol_map(ROOT)
imports = []
def namespace(match):
    name = match['name']
    if name not in mapping:
        return match[0]
    imports.append(name)
    return '    #[link_name = "'+mapping[name]+'"]\n'+match[0]
text = re.sub(r'^    (?:fn|static(?: mut)?) (?P<name>\w+)\b', namespace, text, flags=re.M)
assert imports and 'getprogname' in imports
exports = []
def export(match):
    symbol = match['name']
    if symbol == 'single_binary_main_'+name:
        return match[0]
    assert symbol in mapping, symbol
    exports.append(symbol)
    return match[0].replace('#[no_mangle]', '#[export_name = "'+mapping[symbol]+'"]')
text = re.sub(r'#\[no_mangle\]\n(?:#\[[^\n]+\]\n)*pub (?:unsafe extern "C" fn|static(?: mut)?) (?P<name>\w+)\b', export, text)
def renamed_export(match):
    symbol = match['name']
    if symbol.startswith('rboxc_grep_'):
        return match[0]
    assert symbol in mapping
    exports.append(symbol)
    return '#[export_name = "'+mapping[symbol]+'"]'
text = re.sub(r'#\[export_name = "(?P<name>\w+)"\]', renamed_export, text)
assert set(exports) == defined_symbols([ROOT/f'build/gnu-grep/src/{name}.o']) - {'main'}
notice = re.match(r'\s*(/\*.*?\*/)', source.read_text(), re.S)[1]
text += '\n'+(ROOT/'src/bridges/grep-aliases.rs').read_text()+'\n'
target = ROOT/f'src/generated/applet_{name}.rs'
target.write_text('// Generated from pinned GNU Grep '+pin['version']+' by scripts/translate-grep.py.\n'
                  '// Source SHA-256: '+expected+'\n'+notice+'\n'+declarations+text)
report = {'provider': 'grep', 'version': pin['version'], 'command': name,
          'entry': 'single_binary_main_'+name, 'source': str(source),
          'source_sha256': expected, 'translated': True,
          'scope': 'C2Rust entry translation with namespaced native helper imports; compilation and suite validation are separate.',
          'rust_file': str(target.relative_to(ROOT)), 'rust_sha256': fingerprint(target),
          'ownership_adapter_sha256': fingerprint(ROOT/'src/bridges/grep-owned.rs'),
          'alias_adapter_sha256': fingerprint(ROOT/'src/bridges/grep-aliases.rs'),
          'invocation_adapter_sha256': fingerprint(ROOT/'src/bridges/grep-invocation.rs'),
          'alias_script_sha256': pin['alias_script_sha256'],
          'raw_translation_sha256': fingerprint(outputs[0]), 'compile_database_sha256': fingerprint(database),
          'helper_imports': {name: mapping[name] for name in sorted(imports)},
          'rust_exports': {symbol: mapping[symbol] for symbol in sorted(exports)},
          'adaptations': ['GNU17 parser adaptation maps C23 nullptr to a null pointer constant.',
                          'Use pinned-nightly VaList and exposed-provenance API spellings.',
                          'Preserve provider error prefixes through the public error callback.',
                          'Namespace native helpers and Rust-owned state together.'],
          'opaque_pointer_types': opaque, 'log': str(log.relative_to(ROOT)), 'log_sha256': fingerprint(log)}
(ROOT/'evidence/grep-translation.json').write_text(json.dumps(report, indent=2)+'\n')
print('Translated GNU Grep', name, 'with', len(imports), 'helper imports and', len(exports), 'Rust exports')
