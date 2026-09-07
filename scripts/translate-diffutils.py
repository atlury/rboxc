#!/usr/bin/env python3
"""Translate the pinned GNU Diffutils entry and namespace its helper imports."""
# SPDX-License-Identifier: GPL-3.0-or-later
import json
import os
from pathlib import Path
import re
import subprocess
from diffutils_helpers import COMMANDS, defined_symbols, fingerprint, symbol_map

ROOT = Path(__file__).resolve().parents[1]
import argparse
parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('command', choices=COMMANDS)
name = parser.parse_args().command
pin = json.loads((ROOT/'inventory/sources.json').read_text())['diffutils']
source = Path(os.environ.get('GNU_DIFFUTILS_SOURCE', pin['source']))/f'src/{name}.c'
expected = pin['entry_sha256'][name]
assert fingerprint(source) == expected
records = [json.loads(p.read_text()) for p in (ROOT/'build/diffutils-cc-records').glob('*.json')]
records = [r for r in records if r['output'] == name+'.o']
assert len(records) == 1
command = records[0]
assert Path(command['file']) == source
command['arguments'][0] = 'clang-21'
command['arguments'] += ['-std=gnu17', '-Dunreachable()=__builtin_unreachable()',
                         '-Dalignof=_Alignof', '-Dnullptr=((void*)0)', '-Dmain=single_binary_main_'+name]
stage = ROOT/'build/translation/diffutils'/name
stage.mkdir(parents=True, exist_ok=True)
database = stage/'compile_commands.json'
database.write_text(json.dumps([command], indent=2)+'\n')
log = ROOT/f'evidence/raw/transpile-diffutils-{name}.log'
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
# External is the default linkage for an exported Rust definition.
assert set(re.findall(r'#\[linkage = "([^"]+)"\]', text)) <= {'external'}
text = text.replace('#[linkage = "external"]\n', '')
declarations = ''.join(f'#[repr(C)]\npub struct {name} {{ _opaque: [u8; 0] }}\n' for name in opaque)
# Match the provider's glibc pathname diagnostics through the public
# callback used by Coreutils' Gnulib error implementation.
anchor = '#[no_mangle]\npub unsafe extern "C" fn single_binary_main_'+name+'('
helper = 'extern "C" {\n    static mut error_print_progname: Option<unsafe extern "C" fn()>;\n    #[link_name = "stderr"]\n    static mut rboxc_diffutils_stderr: *mut libc::FILE;\n}\nunsafe extern "C" fn rboxc_diffutils_error_prefix() {\n    libc::fprintf(rboxc_diffutils_stderr, b"%s: \\0".as_ptr().cast(), program_name);\n}\n'
assert text.count(anchor) == 1
text = text.replace(anchor, helper+anchor, 1)
anchor = '    set_program_name(*argv.offset(0isize));'
assert text.count(anchor) == 1
text = text.replace(anchor, anchor+'\n    let prior_error_prefix = error_print_progname;\n    if prior_error_prefix.is_none() {\n        error_print_progname = Some(rboxc_diffutils_error_prefix);\n    }', 1)
if name == 'cmp':
    # Same-file shortcuts and fatal diagnostics bypass GNU's final close loop.
    # Track unopened/detached descriptors and release the single buffer base.
    anchor = 'static mut file_desc: [::core::ffi::c_int; 2] = [0; 2];'
    assert text.count(anchor) == 1
    text = text.replace(anchor, anchor.replace('[0; 2]', '[-1; 2]'), 1)
    anchor = '#[no_mangle]\npub unsafe extern "C" fn single_binary_main_cmp('
    assert text.count(anchor) == 1
    text = text.replace(anchor, 'extern "C" {\n    fn atexit(callback: unsafe extern "C" fn()) -> ::core::ffi::c_int;\n}\nunsafe extern "C" fn rboxc_cmp_close_input(index: usize) -> ::core::ffi::c_int {\n    let descriptor = file_desc[index];\n    file_desc[index] = -1;\n    close(descriptor)\n}\nunsafe extern "C" fn rboxc_cmp_release_owned() {\n    let saved_errno = *__errno_location();\n    for index in 0..2 {\n        if file_desc[index] > 2 {\n            rboxc_cmp_close_input(index);\n        }\n    }\n    libc::free(buffer[0].cast());\n    buffer[0] = ::core::ptr::null_mut();\n    buffer[1] = ::core::ptr::null_mut();\n    *__errno_location() = saved_errno;\n}\n'+anchor, 1)
    anchor = '    set_program_name(*argv.offset(0isize));'
    text = text.replace(anchor, anchor+'\n    atexit(rboxc_cmp_release_owned);', 1)
    anchor = 'close(file_desc[f_2 as usize])'
    assert text.count(anchor) == 1
    text = text.replace(anchor, 'rboxc_cmp_close_input(f_2 as usize)', 1)
if name == 'diff':
    # Replace calls only, leaving the original libc declaration in scope.
    assert text.count('        re_compile_pattern(') == 2
    text = text.replace('        re_compile_pattern(', '        rboxc_diff_compile_regex(')
    before = 'if !cmp.file[f_3 as usize].dirstream.is_null() {\n                (closedir(cmp.file[f_3 as usize].dirstream) < 0 as ::core::ffi::c_int)\n                    as ::core::ffi::c_int\n            } else {\n                (0 as ::core::ffi::c_int <= cmp.file[f_3 as usize].desc\n                    && close(cmp.file[f_3 as usize].desc) < 0 as ::core::ffi::c_int)\n                    as ::core::ffi::c_int\n            } != 0'
    assert text.count(before) == 1
    text = text.replace(before, 'rboxc_diff_close_input(&raw mut cmp.file[f_3 as usize]) < 0', 1)
    anchor = '    let mut free0: *mut ::core::ffi::c_char'
    assert text.count(anchor) == 1
    text = text.replace(anchor, '    let mut rboxc_directory_descriptor: ::core::ffi::c_int = -1;\n'+anchor, 1)
    anchor = '            cmp.file[dir_arg as usize].desc = C2Rust_Unnamed_1::UNOPENED.0;'
    assert text.count(anchor) == 1
    text = text.replace(anchor, '            if cmp.file[dir_arg as usize].dirstream.is_null() {\n                rboxc_directory_descriptor = dirfd;\n            }\n'+anchor, 1)
    anchor = '        f_3 += 1;\n    }\n    if status == EXIT_SUCCESS {'
    assert text.count(anchor) == 1
    text = text.replace(anchor, '        f_3 += 1;\n    }\n    if rboxc_directory_descriptor >= 0 && close(rboxc_directory_descriptor) < 0 {\n        perror_with_name(free0);\n        status = C2Rust_Unnamed_3::EXIT_TROUBLE.0 as ::core::ffi::c_int;\n    }\n    if status == EXIT_SUCCESS {', 1)
    anchor = '#[no_mangle]\npub unsafe extern "C" fn single_binary_main_diff('
    assert text.count(anchor) == 1
    text = text.replace(anchor, (ROOT/'src/bridges/diff-owned.rs').read_text()+'\n'+anchor, 1)
    anchor = '    set_program_name(*argv.offset(0isize));'
    text = text.replace(anchor, anchor+'\n    atexit(rboxc_diff_release_owned);', 1)
if name == 'diff3':
    for function in ('xmalloc', 'ximalloc', 'xinmalloc', 'xicalloc', 'xpalloc', 'close', 'xfreopen'):
        # Restrict substitution to calls in the translated body, never declarations.
        body_start = text.index('pub type ')
        head, body = text[:body_start], text[body_start:]
        body, count = re.subn(r'\b'+function+r'\(', 'rboxc_diff3_'+function+'(', body)
        assert count > 0, function
        text = head + body
    anchor = '    let mut pid: pid_t = fork();'
    assert text.count(anchor) == 1
    text = text.replace(anchor, '    RBOXC_DIFF3_PIPE = fds;\n'+anchor, 1)
    anchor = '        _exit(if *__errno_location() == ENOENT {'
    assert text.count(anchor) == 1
    text = text.replace(anchor, '        rboxc_diff3_exec_failed();\n'+anchor, 1)
    anchor = '#[no_mangle]\npub unsafe extern "C" fn single_binary_main_diff3('
    assert text.count(anchor) == 1
    text = text.replace(anchor, (ROOT/'src/bridges/diff3-owned.rs').read_text()+'\n'+anchor, 1)
    anchor = '    set_program_name(*argv.offset(0isize));'
    text = text.replace(anchor, anchor+'\n    atexit(rboxc_diff3_release_owned);', 1)
mapping = symbol_map(ROOT)
imports = []
def namespace(match):
    name = match['name']
    if name not in mapping:
        return match[0]
    imports.append(name)
    return '    #[link_name = "'+mapping[name]+'"]\n'+match[0]
text = re.sub(r'^    (?:fn|static(?: mut)?) (?P<name>\w+)\b', namespace, text, flags=re.M)
assert imports and 'program_name' in imports
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
    if symbol.startswith('rboxc_diffutils_'):
        return match[0]
    assert symbol in mapping
    exports.append(symbol)
    return '#[export_name = "'+mapping[symbol]+'"]'
text = re.sub(r'#\[export_name = "(?P<name>\w+)"\]', renamed_export, text)
assert set(exports) == defined_symbols([ROOT/f'build/gnu-diffutils/src/{name}.o']) - {'main'}
notice = re.match(r'\s*(/\*.*?\*/)', source.read_text(), re.S)[1]
target = ROOT/f'src/generated/applet_{name}.rs'
target.write_text('// Generated from pinned GNU Diffutils '+pin['version']+' by scripts/translate-diffutils.py.\n'
                  '// Source SHA-256: '+expected+'\n'+notice+'\n'+declarations+text)
report = {'provider': 'diffutils', 'version': pin['version'], 'command': name,
          'entry': 'single_binary_main_'+name, 'source': str(source),
          'source_sha256': expected, 'translated': True,
          'scope': 'C2Rust entry translation with namespaced native helper imports; compilation and suite validation are separate.',
          'rust_file': str(target.relative_to(ROOT)), 'rust_sha256': fingerprint(target),
          'raw_translation_sha256': fingerprint(outputs[0]), 'compile_database_sha256': fingerprint(database),
          'helper_imports': {name: mapping[name] for name in sorted(imports)},
          'rust_exports': {symbol: mapping[symbol] for symbol in sorted(exports)},
          'adaptations': ['GNU17 parser adaptation maps C23 nullptr to the equivalent null pointer constant.',
                          'Preserve provider error prefixes through the public error callback.',
                          'Namespace native helpers and Rust-owned state together.'] +
                         (['Release cmp input descriptors and its single buffer allocation on normal and fatal exit.'] if name == 'cmp' else []) +
                         (['Release replaced regex programs, option storage, and both directory/file descriptors.'] if name == 'diff' else []) +
                         (['Track diff3 entry allocations independently of rewired block links; release allocations and pipe descriptors on exit.'] if name == 'diff3' else []),
          'ownership_adapter_sha256': fingerprint(ROOT/f'src/bridges/{name}-owned.rs') if name in ('diff', 'diff3') else None,
          'opaque_pointer_types': opaque, 'log': str(log.relative_to(ROOT)), 'log_sha256': fingerprint(log)}
(ROOT/f'evidence/diffutils-{name}-translation.json').write_text(json.dumps(report, indent=2)+'\n')
print('Translated GNU Diffutils', name, 'with', len(imports), 'helper imports and', len(exports), 'Rust exports')
