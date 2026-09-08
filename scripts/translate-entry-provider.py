#!/usr/bin/env python3
"""Translate the pinned GNU entry provider entry and namespace its helper imports."""
# SPDX-License-Identifier: GPL-3.0-or-later
import json
import os
from pathlib import Path
import re
import subprocess
from entry_provider_helpers import ENTRY_OBJECTS, PROVIDERS, defined_symbols, fingerprint, symbol_map

ROOT = Path(__file__).resolve().parents[1]
import argparse
parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('command', choices=sorted(set(ENTRY_OBJECTS)-{'bash'}))
name = parser.parse_args().command
provider = PROVIDERS[name]
pin = json.loads((ROOT/'inventory/sources.json').read_text())[provider]
source = Path(os.environ.get('GNU_'+name.upper()+'_SOURCE', pin['source']))/ENTRY_OBJECTS[name].replace('.o', '.c')
expected = pin['entry_source_sha256'][ENTRY_OBJECTS[name].replace('.o', '.c')]
assert fingerprint(source) == expected
records = [json.loads(p.read_text()) for p in (ROOT/f'build/{provider}-cc-records').glob('*.json')]
entry_object = ROOT/f'build/gnu-{provider}'/ENTRY_OBJECTS[name]
records = [r for r in records if r.get('file') and Path(r['file']) == source
           and (Path(r['directory'])/r['output']).resolve() == entry_object]
assert len(records) == 1
command = records[0]
command.pop('kind', None)
assert Path(command['file']) == source
command['arguments'][0] = 'clang-21'
command['arguments'] += ['-Wno-error', '-std=gnu17', '-include', 'stdbool.h', '-Dunreachable()=__builtin_unreachable()',
                         '-Dstatic_assert=_Static_assert', '-Dalignof=_Alignof', '-Dnullptr=((void*)0)', '-Dmain=single_binary_main_'+name]
stage = ROOT/'build/translation'/name
stage.mkdir(parents=True, exist_ok=True)
database = stage/'compile_commands.json'
database.write_text(json.dumps([command], indent=2)+'\n')
log = ROOT/f'evidence/raw/transpile-entry-{name}.log'
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

# C2Rust represents C enums as transparent integer newtypes. Its bitfield
# derive still expects FieldType; delegate to the exact underlying integer.
enum_fields = []
for enum, integer in re.findall(r'pub struct (\w+)\(pub (::core::ffi::c_\w+)\);', text):
    if re.search(r'ty = "'+re.escape(enum)+'"', text):
        enum_fields.append(enum)
        text += f'''
impl ::c2rust_bitfields::FieldType for {enum} {{
    const IS_SIGNED: bool = <{integer} as ::c2rust_bitfields::FieldType>::IS_SIGNED;
    fn get_bit(&self, bit: usize) -> bool {{
        <{integer} as ::c2rust_bitfields::FieldType>::get_bit(&self.0, bit)
    }}
    fn get_field(field: &[u8], range: (usize, usize)) -> Self {{
        Self(<{integer} as ::c2rust_bitfields::FieldType>::get_field(field, range))
    }}
}}
'''

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
assert imports or provider == 'glibc'
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
    if symbol.startswith('rboxc_'+name+'_'):
        return match[0]
    assert symbol in mapping
    exports.append(symbol)
    return '#[export_name = "'+mapping[symbol]+'"]'
text = re.sub(r'#\[export_name = "(?P<name>\w+)"\]', renamed_export, text)
assert set(exports) == defined_symbols([ROOT/f'build/gnu-{provider}'/ENTRY_OBJECTS[name]]) - {'main'}
if name == 'inetd':
    anchor = '#[no_mangle]\npub unsafe extern "C" fn single_binary_main_inetd('
    assert text.count(anchor) == 1
    text = text.replace(anchor, 'unsafe extern "C" fn rboxc_inetd_main_inner(')
    text += '''
extern "C" { #[link_name = "environ"] static mut RBOXC_INETD_ENVIRON: *mut *mut ::core::ffi::c_char; }
#[no_mangle]
pub unsafe extern "C" fn single_binary_main_inetd(argc: ::core::ffi::c_int, argv: *mut *mut ::core::ffi::c_char) -> ::core::ffi::c_int {
    rboxc_inetd_main_inner(argc, argv, RBOXC_INETD_ENVIRON)
}
'''
if name == 'less':
    anchor = '#[no_mangle]\npub unsafe extern "C" fn single_binary_main_less('
    assert text.count(anchor) == 1
    text = text.replace(anchor, 'unsafe extern "C" fn rboxc_less_main_inner(')
    text += '''
#[no_mangle]
pub unsafe extern "C" fn single_binary_main_less(argc: ::core::ffi::c_int, argv: *mut *mut ::core::ffi::c_char) -> ::core::ffi::c_int {
    rboxc_less_main_inner(argc, argv.cast())
}
'''
if provider == 'glibc':
    anchor = '#[no_mangle]\npub unsafe extern "C" fn single_binary_main_'+name+'('
    assert text.count(anchor) == 1
    text = text.replace(anchor, 'unsafe extern "C" fn rboxc_'+name+'_main_inner(')
    # GNU's standalone startup initializes these libc globals from argv[0].
    # Multicall dispatch has already shifted argv, so reproduce that binding
    # using the existing OS argument storage, without allocating a replacement.
    hook_declaration = '''
    #[link_name = "argp_program_version_hook"]
    static mut RBOXC_LIBC_VERSION_HOOK: Option<unsafe extern "C" fn(*mut FILE, *mut argp_state)>;
''' if name == 'iconv' else ''
    hook_assignment = '    RBOXC_LIBC_VERSION_HOOK = argp_program_version_hook;\n' if name == 'iconv' else ''
    text += '''
extern "C" {
    #[link_name = "program_invocation_name"]
    static mut RBOXC_LIBC_INVOCATION: *mut ::core::ffi::c_char;
    #[link_name = "program_invocation_short_name"]
    static mut RBOXC_LIBC_SHORT_INVOCATION: *mut ::core::ffi::c_char;
    #[link_name = "error_print_progname"]
    static mut RBOXC_ERROR_PRINT_PROGNAME: Option<unsafe extern "C" fn()>;
    #[link_name = "stderr"]
    static mut RBOXC_ERROR_STDERR: *mut libc::FILE;
'''+hook_declaration+'''}
unsafe extern "C" fn rboxc_glibc_error_prefix() {
    libc::fprintf(RBOXC_ERROR_STDERR, b"%s: \\0".as_ptr().cast(), RBOXC_LIBC_INVOCATION);
}
#[no_mangle]
pub unsafe extern "C" fn single_binary_main_'''+name+'''(argc: ::core::ffi::c_int, argv: *mut *mut ::core::ffi::c_char) -> ::core::ffi::c_int {
    RBOXC_LIBC_INVOCATION = *argv;
    let bytes = ::core::ffi::CStr::from_ptr(*argv).to_bytes();
    let offset = bytes.iter().rposition(|b| *b == b'/').map_or(0, |i| i+1);
    RBOXC_LIBC_SHORT_INVOCATION = (*argv).add(offset);
    RBOXC_ERROR_PRINT_PROGNAME = Some(rboxc_glibc_error_prefix);
'''+hook_assignment+'    rboxc_'+name+'''_main_inner(argc, argv)
}
'''
if name == 'iconv':
    from glibc_entry_adapters import iconv_cleanup
    text = iconv_cleanup(text)
if name == 'patch':
    anchor = '        if replace_file {\n            output_file('
    assert text.count(anchor) == 1
    text = text.replace(anchor, '''        if !outfile.is_null() && outfd >= 0 && close(outfd) < 0 {
            write_fatal();
        }
'''+anchor)
    anchor = '        if exiting == 0 {\n            free((*f).from.alloc'
    assert text.count(anchor) == 1
    text = text.replace(anchor, anchor.replace('exiting == 0', 'exiting >= 0'))
notice = re.match(r'\s*(/\*.*?\*/)', source.read_text(), re.S)[1]
target = ROOT/f'src/generated/applet_{name}.rs'
target.write_text('// Generated from pinned GNU '+name+' '+pin['version']+' by scripts/translate-entry-provider.py.\n'
                  '// Source SHA-256: '+expected+'\n'+notice+'\n'+declarations+text)
report = {'provider':provider,'version':pin['version'],'command':name,
          'entry':'single_binary_main_'+name,'source':str(source),'source_sha256':expected,
          'translated':True,'scope':'C2Rust entry with namespaced GNU native helpers. Compilation and behavior validation are separate.',
          'rust_file':str(target.relative_to(ROOT)),'rust_sha256':fingerprint(target),
          'raw_translation_sha256':fingerprint(outputs[0]),'compile_database_sha256':fingerprint(database),
          'adaptations':['Pinned Rust ABI/API spelling and opaque pointer type adaptations; helper imports and exports are provider-private.'],
          'helper_imports':{s:mapping[s] for s in sorted(imports)},
          'rust_exports':{s:mapping[s] for s in sorted(exports)},
          'opaque_pointer_types':opaque,'log':str(log.relative_to(ROOT)),'log_sha256':fingerprint(log)}
report['enum_bitfield_integer_delegation'] = enum_fields
if name == 'inetd': report['adaptations'].append('Pass the process environ as GNU main\'s third argument through a two-argument dispatcher adapter.')
if name == 'less': report['adaptations'].append('Preserve Less const-qualified argv pointees through a dispatcher pointer-qualification adapter.')
if provider == 'glibc': report['adaptations'].append('Bind libc invocation-name globals to the dispatched OS argv storage, reproducing standalone GNU startup.')
if provider == 'glibc': report['adaptations'].append('Use GNU error_print_progname to preserve the full invocation path in libc utility diagnostics while retaining the shared GNU error formatter.')
if name == 'iconv': report['adaptations'].append('Register the namespaced GNU version callback with the process libc argp parser before entering the translated command.')
if name == 'iconv': report['adaptations'].append('Close successful encoding probes immediately; close the owned conversion handle, release its output buffer and destroy borrowed-key print-list nodes at process exit, preserving errno.')
if name == 'patch': report['adaptations'].append('Close the unused per-file temporary descriptor when -o sends output to a separately owned stream, after the original final use.')
(ROOT/f'evidence/{name}-translation.json').write_text(json.dumps(report,indent=2)+'\n')
print('Translated GNU',name,'with',len(imports),'helper imports and',len(exports),'Rust exports')
