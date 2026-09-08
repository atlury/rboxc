#!/usr/bin/env python3
"""Translate the pinned GNU Cpio entry and namespace its helper imports."""
# SPDX-License-Identifier: GPL-3.0-or-later
import json
import os
from pathlib import Path
import re
import subprocess
from cpio_helpers import COMMANDS, ENTRY_OBJECTS, defined_symbols, fingerprint, symbol_map

ROOT = Path(__file__).resolve().parents[1]
import argparse
parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('command', choices=COMMANDS)
name = parser.parse_args().command
pin = json.loads((ROOT/'inventory/sources.json').read_text())['cpio']
source = Path(os.environ.get('GNU_CPIO_SOURCE', pin['source']))/ENTRY_OBJECTS[name].replace('.o', '.c')
expected = pin['entry_source_sha256'][ENTRY_OBJECTS[name].replace('.o', '.c')]
assert fingerprint(source) == expected
records = [json.loads(p.read_text()) for p in (ROOT/'build/cpio-cc-records').glob('*.json')]
records = [r for r in records if Path(r['file']) == source]
assert len(records) == 1
command = records[0]
assert Path(command['file']) == source
command['arguments'][0] = 'clang-21'
command['arguments'] += ['-std=gnu17', '-include', 'stdbool.h', '-Dunreachable()=__builtin_unreachable()',
                         '-Dstatic_assert=_Static_assert', '-Dalignof=_Alignof', '-Dnullptr=((void*)0)', '-Dmain=single_binary_main_'+name]
stage = ROOT/'build/translation/cpio'/name
stage.mkdir(parents=True, exist_ok=True)
database = stage/'compile_commands.json'
database.write_text(json.dumps([command], indent=2)+'\n')
log = ROOT/f'evidence/raw/transpile-cpio-{name}.log'
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
mapping = symbol_map(ROOT, name)
imports = []
def namespace(match):
    name = match['name']
    if name not in mapping:
        return match[0]
    imports.append(name)
    return '    #[link_name = "'+mapping[name]+'"]\n'+match[0]
text = re.sub(r'^    (?:fn|static(?: mut)?) (?P<name>\w+)\b', namespace, text, flags=re.M)
# C2Rust renames C identifiers that collide with Rust type names and preserves
# the original linker spelling in an explicit attribute (archive_format).
def namespace_renamed(match):
    symbol = match['name']
    if symbol not in mapping:
        return match[0]
    imports.append(symbol)
    return '#[link_name = "'+mapping[symbol]+'"]'
text = re.sub(r'#\[link_name = "(?P<name>\w+)"\]', namespace_renamed, text)
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
    if symbol.startswith('rboxc_cpio_'):
        return match[0]
    assert symbol in mapping
    exports.append(symbol)
    return '#[export_name = "'+mapping[symbol]+'"]'
text = re.sub(r'#\[export_name = "(?P<name>\w+)"\]', renamed_export, text)
assert set(exports) == defined_symbols([ROOT/'build/gnu-cpio'/ENTRY_OBJECTS[name]]) - {'main'}
anchor = '#[no_mangle]\npub unsafe extern "C" fn single_binary_main_'+name+'('
assert text.count(anchor) == 1
text = text.replace(anchor, 'unsafe extern "C" fn rboxc_cpio_main_inner(')
adaptations = ['Preserve full argv[0] diagnostics through GNU error_print_progname.']
if name == 'mt':
    anchor = '    check_type(tapedev, tapedesc);'
    assert text.count(anchor) == 1
    text = text.replace(anchor, '''    if tapedesc >= 0 && tapedesc < __REM_BIAS {
        RBOXC_MT_OWNED_LOCAL = tapedesc;
    }
'''+anchor)
    anchor = '    if if tapedesc >= __REM_BIAS {'
    assert text.count(anchor) == 1
    text = text.replace(anchor, '    RBOXC_MT_OWNED_LOCAL = -1;\n'+anchor)
    text += '\ninclude!("../bridges/mt-owned.rs");\n'
    adaptations.append('Track and close the opened local descriptor on early exit; invalidate ownership before the original normal close. Remote operations retain GNU behavior and remain untested.')
text += '\ninclude!("../bridges/cpio-invocation.rs");\n'
text += '#[no_mangle]\npub unsafe extern "C" fn single_binary_main_'+name+'''(
    argc: ::core::ffi::c_int, argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    rboxc_cpio_setup(argv);
    rboxc_cpio_main_inner(argc, argv)
}
'''
if name == 'mt':
    anchor = '    rboxc_cpio_setup(argv);'
    assert text.count(anchor) == 1
    text = text.replace(anchor, anchor+'\n    if libc::atexit(rboxc_mt_release) != 0 { libc::_exit(2); }')
notice = re.match(r'\s*(/\*.*?\*/)', source.read_text(), re.S)[1]
target = ROOT/f'src/generated/applet_{name}.rs'
target.write_text('// Generated from pinned GNU Cpio '+pin['version']+' by scripts/translate-cpio.py.\n'
                  '// Source SHA-256: '+expected+'\n'+notice+'\n'+declarations+text)
bridge = ROOT/'src/bridges/cpio-invocation.rs'
report = {'provider':'cpio','version':pin['version'],'command':name,
          'entry':'single_binary_main_'+name,'source':str(source),'source_sha256':expected,
          'translated':True,'scope':'C2Rust entry with namespaced GNU native helpers. Compilation and behavior validation are separate.',
          'rust_file':str(target.relative_to(ROOT)),'rust_sha256':fingerprint(target),
          'raw_translation_sha256':fingerprint(outputs[0]),'compile_database_sha256':fingerprint(database),
          'adaptations':adaptations,
          'invocation_adapter_sha256':fingerprint(bridge),
          'helper_imports':{s:mapping[s] for s in sorted(imports)},
          'rust_exports':{s:mapping[s] for s in sorted(exports)},
          'opaque_pointer_types':opaque,'log':str(log.relative_to(ROOT)),'log_sha256':fingerprint(log)}
if name == 'mt':
    report['ownership_adapter_sha256'] = fingerprint(ROOT/'src/bridges/mt-owned.rs')
(ROOT/f'evidence/cpio-{name}-translation.json').write_text(json.dumps(report,indent=2)+'\n')
print('Translated GNU Cpio',name,'with',len(imports),'helper imports and',len(exports),'Rust exports')
