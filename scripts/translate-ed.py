#!/usr/bin/env python3
"""Translate the pinned GNU Ed entry and namespace its helper imports."""
# SPDX-License-Identifier: GPL-3.0-or-later
import json
import os
from pathlib import Path
import re
import subprocess
from ed_helpers import defined_symbols, fingerprint, symbol_map

ROOT = Path(__file__).resolve().parents[1]
name = 'ed'
pin = json.loads((ROOT/'inventory/sources.json').read_text())['ed']
source = Path(os.environ.get('GNU_ED_SOURCE', pin['source']))/'main.c'
expected = pin['entry_sha256']
assert fingerprint(source) == expected
records = [json.loads(p.read_text()) for p in (ROOT/'build/ed-cc-records').glob('*.json')]
records = [r for r in records if Path(r['file']) == source]
assert len(records) == 1
command = records[0]
assert Path(command['file']) == source
command['arguments'][0] = 'clang-21'
command['arguments'] += ['-std=gnu17', '-Dunreachable()=__builtin_unreachable()',
                         '-Dalignof=_Alignof', '-Dnullptr=((void*)0)', '-Dmain=single_binary_main_'+name]
stage = ROOT/'build/translation/ed'
stage.mkdir(parents=True, exist_ok=True)
database = stage/'compile_commands.json'
database.write_text(json.dumps([command], indent=2)+'\n')
log = ROOT/'evidence/raw/transpile-ed.log'
tool = os.environ.get('C2RUST', str(ROOT/'.tools/c2rust-build/release/c2rust'))
with log.open('w') as output:
    subprocess.run([tool, 'transpile', '--emit-modules', '--emit-no-std', '--fail-on-error',
                    '--overwrite-existing', '--output-dir', stage/'out', database],
                   env={**os.environ, 'RUSTUP_TOOLCHAIN': '1.93.0'},
                   stdout=output, stderr=subprocess.STDOUT, check=True)
assert not re.search(r'(^|\n).*?:[0-9]+:[0-9]+: (?:fatal )?error:', log.read_text()), 'Clang reported translation diagnostics'
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
mapping = symbol_map(ROOT)
imports = []
def namespace(match):
    name = match['name']
    if name not in mapping:
        return match[0]
    imports.append(name)
    return '    #[link_name = "'+mapping[name]+'"]\n'+match[0]
text = re.sub(r'^    (?:fn|static(?: mut)?) (?P<name>\w+)\b', namespace, text, flags=re.M)
assert imports
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
    if symbol.startswith('rboxc_ed_'):
        return match[0]
    assert symbol in mapping
    exports.append(symbol)
    return '#[export_name = "'+mapping[symbol]+'"]'
text = re.sub(r'#\[export_name = "(?P<name>\w+)"\]', renamed_export, text)
assert set(exports) == defined_symbols([ROOT/'build/gnu-ed/main.o']) - {'main'}
assert mapping['close_sbuf'] == 'rboxc_ed_close_sbuf'
imports.append('close_sbuf')
# Keep parser storage alive until libc exit, including parse_addr's direct
# exit path. GNU's normal ap_free is idempotent and clears the owned pointers.
parser_init = re.search(r'    let mut parser: Arg_parser = (Arg_parser \{.*?\n    \});', text, re.S)
assert parser_init
initializer = parser_init[1]
text = text[:parser_init.start()] + text[parser_init.end():]
text = re.sub(r'\bparser\b', 'RBOXC_ED_PARSER', text)
anchor = '    if !init_buffers() {\n        return 1 as ::core::ffi::c_int;\n    }'
assert text.count(anchor) == 1
text = text.replace(anchor, anchor+'\n    RBOXC_ED_BUFFERS_READY = true;')
text += '\nstatic mut RBOXC_ED_PARSER: Arg_parser = '+initializer+';\n'
text += '''
static mut RBOXC_ED_BUFFERS_READY: bool = false;
extern "C" {
    #[link_name = "rboxc_ed_close_sbuf"]
    fn rboxc_ed_close_owned_scratch() -> bool;
}
extern "C" fn rboxc_ed_release_owned() {
    unsafe {
        let saved_errno = *libc::__errno_location();
        ap_free(&raw mut RBOXC_ED_PARSER);
        // The scratch destructor traverses initialized yank/undo lists.
        if RBOXC_ED_BUFFERS_READY {
            RBOXC_ED_BUFFERS_READY = false;
            rboxc_ed_close_owned_scratch();
        }
        *libc::__errno_location() = saved_errno;
    }
}
'''
anchor = '#[no_mangle]\npub unsafe extern "C" fn single_binary_main_ed('
assert text.count(anchor) == 1
text = text.replace(anchor, 'unsafe extern "C" fn rboxc_ed_main_const(')
text += '''
// The multicall ABI supplies mutable argv; GNU Ed only reads it.
#[no_mangle]
pub unsafe extern "C" fn single_binary_main_ed(
    argc: ::core::ffi::c_int, argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if libc::atexit(rboxc_ed_release_owned) != 0 { return 1; }
    rboxc_ed_main_const(argc, argv as *const *const ::core::ffi::c_char)
}
'''
notice = re.match(r'\s*(/\*.*?\*/)', source.read_text(), re.S)[1]
target = ROOT/f'src/generated/applet_{name}.rs'
target.write_text('// Generated from pinned GNU Ed '+pin['version']+' by scripts/translate-ed.py.\n'
                  '// Source SHA-256: '+expected+'\n'+notice+'\n'+declarations+text)
report = {'provider': 'ed', 'version': pin['version'], 'command': name,
          'entry': 'single_binary_main_'+name, 'source': str(source),
          'source_sha256': expected, 'translated': True,
          'scope': 'C2Rust entry translation with namespaced native helper imports; compilation and suite validation are separate.',
          'rust_file': str(target.relative_to(ROOT)), 'rust_sha256': fingerprint(target),
          'raw_translation_sha256': fingerprint(outputs[0]), 'compile_database_sha256': fingerprint(database),
          'helper_imports': {name: mapping[name] for name in sorted(imports)},
          'rust_exports': {symbol: mapping[symbol] for symbol in sorted(exports)},
          'adaptations': ['GNU17 parser adaptation maps C23 nullptr to a null pointer constant.',
                          'Use pinned-nightly VaList and exposed-provenance API spellings.',
                          'Namespace native helpers and Rust-owned state together.',
                          'Adapt mutable dispatcher argv to the unchanged const GNU Ed entry signature.',
                          'Keep parser storage valid through exit and call GNU parser/scratch destructors, including early exits; preserve errno.'],
          'opaque_pointer_types': opaque, 'log': str(log.relative_to(ROOT)), 'log_sha256': fingerprint(log)}
(ROOT/'evidence/ed-translation.json').write_text(json.dumps(report, indent=2)+'\n')
print('Translated GNU Ed', name, 'with', len(imports), 'helper imports and', len(exports), 'Rust exports')
