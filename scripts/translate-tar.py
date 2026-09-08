#!/usr/bin/env python3
"""Translate the pinned GNU Tar entry and namespace its helper imports."""
# SPDX-License-Identifier: GPL-3.0-or-later
import json
import os
from pathlib import Path
import re
import subprocess
from tar_helpers import COMMANDS, ENTRY_OBJECTS, defined_symbols, fingerprint, symbol_map

ROOT = Path(__file__).resolve().parents[1]
import argparse
parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('command', choices=COMMANDS)
name = parser.parse_args().command
pin = json.loads((ROOT/'inventory/sources.json').read_text())['tar']
source = Path(os.environ.get('GNU_TAR_SOURCE', pin['source']))/pin['entry_source']
expected = pin['entry_sha256']
assert fingerprint(source) == expected
records = [json.loads(p.read_text()) for p in (ROOT/'build/tar-cc-records').glob('*.json')]
records = [r for r in records if Path(r['file']) == source]
assert len(records) == 1
command = records[0]
assert Path(command['file']) == source
command['arguments'][0] = 'clang-21'
command['arguments'] += ['-std=gnu17', '-include', 'stdbool.h', '-Dunreachable()=__builtin_unreachable()',
                         '-Dstatic_assert=_Static_assert', '-Dalignof=_Alignof', '-Dnullptr=((void*)0)', '-Dmain=single_binary_main_'+name]
stage = ROOT/'build/translation/tar'/name
stage.mkdir(parents=True, exist_ok=True)
database = stage/'compile_commands.json'
database.write_text(json.dumps([command], indent=2)+'\n')
log = ROOT/f'evidence/raw/transpile-tar-{name}.log'
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
    if symbol.startswith('rboxc_tar_'):
        return match[0]
    assert symbol in mapping
    exports.append(symbol)
    return '#[export_name = "'+mapping[symbol]+'"]'
text = re.sub(r'#\[export_name = "(?P<name>\w+)"\]', renamed_export, text)
assert set(exports) == defined_symbols([ROOT/'build/gnu-tar'/ENTRY_OBJECTS[name]]) - {'main'}
anchor = '#[no_mangle]\npub unsafe extern "C" fn single_binary_main_tar('
assert text.count(anchor) == 1
text = text.replace(anchor, 'unsafe extern "C" fn rboxc_tar_main_inner(')
text += '''
extern "C" {
    static mut error_print_progname: Option<unsafe extern "C" fn()>;
}
static mut RBOXC_INVOCATION: *const ::core::ffi::c_char = ::core::ptr::null();
unsafe extern "C" fn rboxc_tar_error_prefix() {
    libc::fprintf(stderr.cast(), b"%s: \\0".as_ptr().cast(), RBOXC_INVOCATION);
}
#[no_mangle]
pub unsafe extern "C" fn single_binary_main_tar(
    argc: ::core::ffi::c_int, argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    RBOXC_INVOCATION = if argv.is_null() || (*argv).is_null() {
        b"tar\\0".as_ptr().cast()
    } else { *argv };
    if error_print_progname.is_none() {
        error_print_progname = Some(rboxc_tar_error_prefix);
    }
    rboxc_tar_main_inner(argc, argv)
}
'''
# The default-settings text is copied into the help obstack before S is
# overwritten with the completed help text; release that intermediate copy.
start = text.index('unsafe extern "C" fn tar_help_filter(')
end = text.index('unsafe extern "C" fn expand_pax_option(', start)
part = text[start:end]
anchor = '            (*__o_2).next_free = (*__o_2).next_free.offset(__len_2 as isize);'
assert part.count(anchor) == 1 and part.count('s = format_default_settings();') == 1
part = part.replace(anchor, anchor+'\n            free(s.cast());')
text = text[:start]+part+text[end:]
# Environment option strings remain borrowed by GNU option state until exit.
# Retain the whole initialized wordsplit workspace for normal and fatal exits.
start = text.index('unsafe extern "C" fn parse_default_options(')
end = text.index('unsafe extern "C" fn decode_options(', start)
part = text[start:end]
initializer = re.search(r'    let mut ws: wordsplit = wordsplit \{.*?\n    \};', part, re.S)
assert initializer
global_ws = initializer[0].replace('    let mut ws:', 'static mut RBOXC_DEFAULT_WORDS:', 1)
part = part.replace(initializer[0], '')
assert part.count('        ws.ws_wordc = 0 as size_t;') == 1
part = part.replace('        ws.ws_wordc = 0 as size_t;', '')
assert part.count('    wordsplit_free(&raw mut ws);') == 1
part = part.replace('    wordsplit_free(&raw mut ws);', '')
part = re.sub(r'\bws\b', 'RBOXC_DEFAULT_WORDS', part)
text = text[:start]+part+text[end:]
text += '\n'+global_ws+'\n'
assert mapping['wordsplit_clearerr'] == 'rboxc_tar_wordsplit_clearerr'
imports.append('wordsplit_clearerr')
text += '''
extern "C" {
    #[link_name = "rboxc_tar_wordsplit_clearerr"]
    fn rboxc_wordsplit_clearerr(ws: *mut wordsplit);
}
static mut RBOXC_OWNED_ARGS: Vec<*mut ::core::ffi::c_void> = Vec::new();
static mut RBOXC_STDOPEN_OWNED: [bool; 3] = [false; 3];
unsafe fn rboxc_tar_stdopen_owned() -> ::core::ffi::c_int {
    let saved_errno = *libc::__errno_location();
    let mut missing = [false; 3];
    for fd in 0..3 {
        missing[fd] = libc::fcntl(fd as i32, libc::F_GETFD) < 0
            && *libc::__errno_location() == libc::EBADF;
    }
    *libc::__errno_location() = saved_errno;
    let result = stdopen();
    let result_errno = *libc::__errno_location();
    for fd in 0..3 {
        RBOXC_STDOPEN_OWNED[fd] = missing[fd]
            && libc::fcntl(fd as i32, libc::F_GETFD) >= 0;
    }
    *libc::__errno_location() = result_errno;
    result
}
unsafe fn rboxc_tar_own_argument(pointer: *mut ::core::ffi::c_void) {
    RBOXC_OWNED_ARGS.push(pointer);
}
extern "C" fn rboxc_tar_release_arguments() {
    unsafe {
        let saved_errno = *libc::__errno_location();
        wordsplit_free(&raw mut RBOXC_DEFAULT_WORDS);
        rboxc_wordsplit_clearerr(&raw mut RBOXC_DEFAULT_WORDS);
        for pointer in ::core::mem::take(&mut *(&raw mut RBOXC_OWNED_ARGS)) {
            free(pointer);
        }
        for fd in 0..3 {
            if RBOXC_STDOPEN_OWNED[fd] && libc::fcntl(fd as i32, libc::F_GETFD) >= 0 {
                // Detach a live standard FILE before libc's final flush. GNU
                // may have buffered output even on its read-only replacement.
                let stream = [stdin, stdout, stderr][fd];
                if libc::fileno(stream.cast()) == fd as i32 {
                    libc::fclose(stream.cast());
                } else {
                    libc::close(fd as i32);
                }
            }
            RBOXC_STDOPEN_OWNED[fd] = false;
        }
        *libc::__errno_location() = saved_errno;
    }
}
'''
# Preserve GNU's standard-descriptor replacement policy. At exit, release only
# replacements opened for descriptors that were closed immediately before it.
anchor = '    if stdopen() != 0 {'
assert text.count(anchor) == 1
text = text.replace(anchor, '    if rboxc_tar_stdopen_owned() != 0 {')
start = text.index('unsafe extern "C" fn decode_options(')
end = text.index('unsafe extern "C" fn ', start+1)
part = text[start:end]
anchor = '        r#in = argv;'
assert part.count(anchor) == 1
part = part.replace(anchor, '        rboxc_tar_own_argument(new_argv.cast());\n'+anchor)
anchor = '            *c2rust_fresh10 = xstrdup(&raw mut buffer as *mut ::core::ffi::c_char);'
assert part.count(anchor) == 1
part = part.replace(anchor, anchor+'\n            rboxc_tar_own_argument((*c2rust_fresh10).cast());')
text = text[:start]+part+text[end:]
anchor = '    rboxc_tar_main_inner(argc, argv)'
assert text.count(anchor) == 1
text = text.replace(anchor, '''    if libc::atexit(rboxc_tar_release_arguments) != 0 {
        libc::_exit(2);
    }
'''+anchor)
notice = re.match(r'\s*(/\*.*?\*/)', source.read_text(), re.S)[1]
target = ROOT/f'src/generated/applet_{name}.rs'
target.write_text('// Generated from pinned GNU Tar '+pin['version']+' by scripts/translate-tar.py.\n'
                  '// Source SHA-256: '+expected+'\n'+notice+'\n'+declarations+text)
report = {'provider': 'tar', 'version': pin['version'], 'command': name,
          'entry': 'single_binary_main_'+name, 'source': str(source),
          'source_sha256': expected, 'translated': True,
          'scope': 'C2Rust entry with namespaced GNU native helpers. Compilation and behavior validation are separate.',
          'rust_file': str(target.relative_to(ROOT)), 'rust_sha256': fingerprint(target),
          'raw_translation_sha256': fingerprint(outputs[0]), 'compile_database_sha256': fingerprint(database),
          'adaptations': ['Preserve full argv[0] diagnostics through GNU error_print_progname.',
                          'Free the default-settings help string after copying it into the obstack.',
                          'Retain environment option words and owned old-style arguments until exit, then release them.',
                          'Close only standard-descriptor replacements opened by GNU stdopen at exit, preserving errno.'],
          'helper_imports': {s: mapping[s] for s in sorted(imports)},
          'rust_exports': {s: mapping[s] for s in sorted(exports)},
          'opaque_pointer_types': opaque, 'log': str(log.relative_to(ROOT)), 'log_sha256': fingerprint(log)}
(ROOT/'evidence/tar-translation.json').write_text(json.dumps(report, indent=2)+'\n')
print('Translated GNU Tar with', len(imports), 'helper imports and', len(exports), 'Rust exports')
