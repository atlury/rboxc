#!/usr/bin/env python3
"""Translate the pinned GNU Hello entry and namespace its helper imports."""
# SPDX-License-Identifier: GPL-3.0-or-later
import json
import os
from pathlib import Path
import re
import subprocess
from hello_helpers import fingerprint, symbol_map

ROOT = Path(__file__).resolve().parents[1]
pin = json.loads((ROOT/'inventory/sources.json').read_text())['hello']
source = Path(os.environ.get('GNU_HELLO_SOURCE', pin['source']))/'src/hello.c'
assert fingerprint(source) == pin['entry_sha256']
records = [json.loads(p.read_text()) for p in (ROOT/'build/hello-cc-records').glob('*.json')]
records = [r for r in records if r['output'] == 'src/hello.o']
assert len(records) == 1
command = records[0]
assert Path(command['file']) == source
command['arguments'][0] = 'clang-21'
command['arguments'] += ['-std=gnu17', '-Dunreachable()=__builtin_unreachable()',
                         '-Dalignof=_Alignof', '-Dmain=single_binary_main_hello']
stage = ROOT/'build/translation/hello'
stage.mkdir(parents=True, exist_ok=True)
database = stage/'compile_commands.json'
database.write_text(json.dumps([command], indent=2)+'\n')
log = ROOT/'evidence/raw/transpile-hello.log'
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
# Coreutils supplies a Gnulib error implementation that prints a basename.
# GNU Hello on this glibc profile retains argv[0], including its directory.
# Use the documented error callback so its diagnostic prefix follows Hello.
anchor = '#[no_mangle]\npub unsafe extern "C" fn single_binary_main_hello('
helper = '''extern "C" {
    static mut error_print_progname: Option<unsafe extern "C" fn()>;
}
unsafe extern "C" fn rboxc_hello_error_prefix() {
    fprintf(stderr, b"%s: \\0".as_ptr().cast(), program_name);
}
'''
assert text.count(anchor) == 1
text = text.replace(anchor, helper+anchor, 1)
anchor = '    set_program_name(*argv.offset(0isize));'
assert text.count(anchor) == 1
text = text.replace(anchor, anchor+'''
    if error_print_progname.is_none() {
        error_print_progname = Some(rboxc_hello_error_prefix);
    }''', 1)
# Conversion failure exits through error(), bypassing the normal free below.
anchor = '    if len == -1 as ::core::ffi::c_int as size_t {'
assert text.count(anchor) == 1
text = text.replace(anchor, anchor+'''
        let saved_errno = *__errno_location();
        free(mb_greeting.cast());
        mb_greeting = ::core::ptr::null_mut();
        *__errno_location() = saved_errno;''', 1)
mapping = symbol_map(ROOT)
imports = []
def namespace(match):
    name = match['name']
    if name not in mapping:
        return match[0]
    imports.append(name)
    return '    #[link_name = "'+mapping[name]+'"]\n'+match[0]
text = re.sub(r'^    (?:fn|static mut) (?P<name>\w+)\b', namespace, text, flags=re.M)
assert imports and 'program_name' in imports and 'close_stdout' in imports
notice = re.match(r'\s*(/\*.*?\*/)', source.read_text(), re.S)[1]
target = ROOT/'src/generated/applet_hello.rs'
target.write_text('// Generated from pinned GNU Hello '+pin['version']+' by scripts/translate-hello.py.\n'
                  '// Source SHA-256: '+pin['entry_sha256']+'\n'+notice+'\n'+declarations+text)
report = {'provider': 'hello', 'version': pin['version'], 'command': 'hello',
          'entry': 'single_binary_main_hello', 'source': str(source),
          'source_sha256': pin['entry_sha256'], 'translated': True,
          'scope': 'C2Rust entry translation with namespaced native helper imports; compilation and suite validation are separate.',
          'rust_file': str(target.relative_to(ROOT)), 'rust_sha256': fingerprint(target),
          'raw_translation_sha256': fingerprint(outputs[0]), 'compile_database_sha256': fingerprint(database),
          'helper_imports': {name: mapping[name] for name in sorted(imports)},
          'adaptations': ['Preserve Hello error prefixes through the public error callback.',
                          'Release owned wide-character greeting before fatal conversion diagnostics.'],
          'opaque_pointer_types': opaque, 'log': str(log.relative_to(ROOT)), 'log_sha256': fingerprint(log)}
(ROOT/'evidence/hello-translation.json').write_text(json.dumps(report, indent=2)+'\n')
print('Translated GNU Hello entry with', len(imports), 'namespaced helper imports')
