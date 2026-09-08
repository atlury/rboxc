#!/usr/bin/env python3
"""Translate the pinned GNU Findutils entry and namespace its helper imports."""
# SPDX-License-Identifier: GPL-3.0-or-later
import json
import os
from pathlib import Path
import re
import subprocess
from findutils_helpers import COMMANDS, ENTRY_OBJECTS, defined_symbols, fingerprint, symbol_map

ROOT = Path(__file__).resolve().parents[1]
import argparse
parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('command', choices=COMMANDS)
name = parser.parse_args().command
pin = json.loads((ROOT/'inventory/sources.json').read_text())['findutils']
source = Path(os.environ.get('GNU_FINDUTILS_SOURCE', pin['source']))/pin['entry_source'][name]
expected = pin['entry_sha256'][name]
assert fingerprint(source) == expected
records = [json.loads(p.read_text()) for p in (ROOT/'build/findutils-cc-records').glob('*.json')]
records = [r for r in records if Path(r['file']) == source]
assert len(records) == 1
command = records[0]
assert Path(command['file']) == source
command['arguments'][0] = 'clang-21'
command['arguments'] += ['-std=gnu17', '-Dunreachable()=__builtin_unreachable()',
                         '-Dalignof=_Alignof', '-Dnullptr=((void*)0)', '-Dmain=single_binary_main_'+name]
stage = ROOT/'build/translation/findutils'/name
stage.mkdir(parents=True, exist_ok=True)
database = stage/'compile_commands.json'
database.write_text(json.dumps([command], indent=2)+'\n')
log = ROOT/f'evidence/raw/transpile-findutils-{name}.log'
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
    if symbol.startswith('rboxc_findutils_'):
        return match[0]
    assert symbol in mapping
    exports.append(symbol)
    return '#[export_name = "'+mapping[symbol]+'"]'
text = re.sub(r'#\[export_name = "(?P<name>\w+)"\]', renamed_export, text)
assert set(exports) == defined_symbols([ROOT/'build/gnu-findutils'/ENTRY_OBJECTS[name]]) - {'main'}
# Native Findutils uses glibc's full argv[0] diagnostic prefix on this host.
# Coreutils supplies a basename-printing error implementation in the multicall
# link; its public callback restores the provider's original prefix.
anchor = '#[no_mangle]\npub unsafe extern "C" fn single_binary_main_'+name+'('
assert text.count(anchor) == 1
text = text.replace(anchor, 'unsafe extern "C" fn rboxc_findutils_main_inner(')
text += '''
extern "C" {
    static mut error_print_progname: Option<unsafe extern "C" fn()>;
}
static mut RBOXC_INVOCATION: *const ::core::ffi::c_char = ::core::ptr::null();
unsafe extern "C" fn rboxc_findutils_error_prefix() {
    libc::fprintf(stderr.cast(), b"%s: \\0".as_ptr().cast(), RBOXC_INVOCATION);
}
'''
cleanup_registration = ''
ownership_adaptations = []
if name == 'find':
    assert mapping['initial_wd'] == 'rboxc_findutils_find_initial_wd'
    assert mapping['free_cwd'] == 'rboxc_findutils_find_free_cwd'
    imports += ['initial_wd', 'free_cwd']
    text += '''
extern "C" {
    #[link_name = "rboxc_findutils_find_initial_wd"]
    static mut rboxc_initial_wd: *mut ::core::ffi::c_void;
    #[link_name = "rboxc_findutils_find_free_cwd"]
    fn rboxc_free_cwd(directory: *mut ::core::ffi::c_void);
}
extern "C" fn rboxc_release_initial_wd() {
    unsafe {
        let saved_errno = *libc::__errno_location();
        let directory = rboxc_initial_wd;
        if !directory.is_null() {
            rboxc_initial_wd = ::core::ptr::null_mut();
            rboxc_free_cwd(directory);
            libc::free(directory);
        }
        *libc::__errno_location() = saved_errno;
    }
}
'''
    cleanup_registration = '    if libc::atexit(rboxc_release_initial_wd) != 0 { return 1; }\n'
    anchor = '    end_of_leading_options = process_leading_options(argc, argv);'
    assert text.count(anchor) == 1
    text = text.replace(anchor, cleanup_registration + anchor)
    ownership_adaptations.append('Register idempotent saved-directory cleanup again after close_stdout so write-error exits release it first.')
elif name == 'xargs':
    anchor = '        let mut arglen: *mut size_t =\n'
    assert text.count(anchor) == 1
    text = text.replace(anchor, '        RBOXC_ARGLEN =\n')
    # All uses belong to the one replacement-mode allocation.
    text = re.sub(r'\*arglen\.offset', '*RBOXC_ARGLEN.offset', text)
    anchor = '    bcstatus = bc_init_controlinfo('
    assert text.count(anchor) == 1
    text = text.replace(anchor, '    if libc::atexit(rboxc_release_xargs_input) != 0 { return 1; }\n' + anchor)
    anchor = '            _exit(if saved_errno == ENOENT {'
    assert text.count(anchor) == 1
    text = text.replace(anchor, '''            // Only the failed-exec child owns this replacement for stdin.
            if keep_stdin == 0 || open_tty { libc::close(0); }
''' + anchor)
    text += '''
static mut RBOXC_ARGLEN: *mut size_t = ::core::ptr::null_mut();
extern "C" fn rboxc_release_xargs_input() {
    unsafe {
        let saved_errno = *libc::__errno_location();
        libc::free(RBOXC_ARGLEN.cast());
        RBOXC_ARGLEN = ::core::ptr::null_mut();
        if !input_stream.is_null() && input_stream != stdin {
            let owned = input_stream;
            input_stream = ::core::ptr::null_mut();
            libc::fclose(owned.cast());
        }
        *libc::__errno_location() = saved_errno;
    }
}
'''
    ownership_adaptations += ['Release replacement argument lengths and owned argument FILE before wait/close callbacks can exit.',
                              'Close redirected stdin in the child after exec has failed and its diagnostic has been printed.']
elif name == 'locate':
    declaration = re.search(r'    let mut procdata: process_data = process_data \{.*?\n    \};', text, re.S)
    assert declaration
    static = declaration[0].replace('    let mut procdata:', 'static mut RBOXC_PROCDATA:', 1)
    text = text[:declaration.start()] + '    rboxc_release_locate_path();' + text[declaration.end():]
    start = text.index('unsafe extern "C" fn search_one_database(')
    end = text.index('unsafe extern "C" fn usage(', start)
    text = text[:start] + re.sub(r'\bprocdata\b', 'RBOXC_PROCDATA', text[start:end]) + text[end:]
    text += '\n' + static + '''
extern "C" fn rboxc_release_locate_path() {
    unsafe {
        let saved_errno = *libc::__errno_location();
        libc::free(RBOXC_PROCDATA.original_filename.cast());
        RBOXC_PROCDATA.original_filename = ::core::ptr::null_mut();
        *libc::__errno_location() = saved_errno;
    }
}
'''
    # Each search starts by releasing the preceding buffer; final/error exits
    # have static storage available to the callback as well.
    start = text.index('unsafe extern "C" fn dolocate(')
    end = text.index('unsafe extern "C" fn open_secure_db(', start)
    body = text[start:end]
    declaration = '    let mut path_element: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();\n'
    assert body.count(declaration) == 1
    body = re.sub(r'\bpath_element\b', 'RBOXC_DBPATH', body.replace(declaration, ''))
    registration = '    if libc::atexit(rboxc_release_locate_resources) != 0 { return 1; }\n'
    anchor = '    limits.limit = 0 as uintmax_t;'
    assert body.count(anchor) == 1
    body = body.replace(anchor, registration + anchor)
    text = text[:start] + body + text[end:]
    text += '''
static mut RBOXC_DBPATH: *mut ::core::ffi::c_char = ::core::ptr::null_mut();
extern "C" fn rboxc_release_locate_resources() {
    rboxc_release_locate_path();
    unsafe {
        let saved_errno = *libc::__errno_location();
        libc::free(RBOXC_DBPATH.cast());
        RBOXC_DBPATH = ::core::ptr::null_mut();
        *libc::__errno_location() = saved_errno;
    }
}
'''
    cleanup_registration = registration
    ownership_adaptations.append('Give the database path buffer stable ownership, releasing it between searches and at exit.')
    ownership_adaptations.append('Release database-name storage on early returns; register before close_stdout can terminate a write-error exit.')
text += '''
#[no_mangle]
pub unsafe extern "C" fn single_binary_main_'''+name+'''(
    argc: ::core::ffi::c_int, argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    RBOXC_INVOCATION = if argv.is_null() || (*argv).is_null() {
        b"'''+name+'''\\0".as_ptr().cast()
    } else { *argv };
    if error_print_progname.is_none() {
        error_print_progname = Some(rboxc_findutils_error_prefix);
    }
'''+cleanup_registration+'''    rboxc_findutils_main_inner(argc, argv)
}
'''
notice = re.match(r'\s*(/\*.*?\*/)', source.read_text(), re.S)[1]
target = ROOT/f'src/generated/applet_{name}.rs'
target.write_text('// Generated from pinned GNU Findutils '+pin['version']+' by scripts/translate-findutils.py.\n'
                  '// Source SHA-256: '+expected+'\n'+notice+'\n'+declarations+text)
report = {'provider': 'findutils', 'version': pin['version'], 'command': name,
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
                          'Preserve full argv[0] diagnostics through GNU error_print_progname.'] +
                         (['Release any remaining saved initial working directory on exit; normal GNU cleanup already clears it.'] if name == 'find' else []) + ownership_adaptations,
          'opaque_pointer_types': opaque, 'log': str(log.relative_to(ROOT)), 'log_sha256': fingerprint(log)}
(ROOT/f'evidence/findutils-{name}-translation.json').write_text(json.dumps(report, indent=2)+'\n')
print('Translated GNU Findutils', name, 'with', len(imports), 'helper imports and', len(exports), 'Rust exports')
