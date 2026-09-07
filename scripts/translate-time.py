#!/usr/bin/env python3
"""Translate the pinned GNU Time entry and namespace its helper imports."""
# SPDX-License-Identifier: GPL-3.0-or-later
import json
import os
from pathlib import Path
import re
import subprocess
from time_helpers import fingerprint, symbol_map

ROOT = Path(__file__).resolve().parents[1]
pin = json.loads((ROOT/'inventory/sources.json').read_text())['time']
source = Path(os.environ.get('GNU_TIME_SOURCE', pin['source']))/'src/time.c'
assert fingerprint(source) == pin['entry_sha256']
records = [json.loads(p.read_text()) for p in (ROOT/'build/time-cc-records').glob('*.json')]
records = [r for r in records if r['output'] == 'src/time.o']
assert len(records) == 1
command = records[0]
assert Path(command['file']) == source
command['arguments'][0] = 'clang-21'
command['arguments'] += ['-std=gnu17', '-Dunreachable()=__builtin_unreachable()',
                         '-Dalignof=_Alignof', '-Dmain=single_binary_main_time']
stage = ROOT/'build/translation/time'
stage.mkdir(parents=True, exist_ok=True)
database = stage/'compile_commands.json'
database.write_text(json.dumps([command], indent=2)+'\n')
log = ROOT/'evidence/raw/transpile-time.log'
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
# Preserve the provider's glibc error pathname convention through the public
# callback used by Coreutils' Gnulib error implementation.
anchor = '#[no_mangle]\npub unsafe extern "C" fn single_binary_main_time('
helper = 'extern "C" {\n    static mut error_print_progname: Option<unsafe extern "C" fn()>;\n    fn fclose(stream: *mut FILE) -> ::core::ffi::c_int;\n    fn atexit(callback: unsafe extern "C" fn()) -> ::core::ffi::c_int;\n}\nunsafe extern "C" fn rboxc_time_error_prefix() {\n    fprintf(stderr, b"%s: \\0".as_ptr().cast(), program_name);\n}\nunsafe extern "C" fn rboxc_time_close_output() {\n    if !outfile.is_null() && !outfp.is_null() {\n        let saved_errno = *__errno_location();\n        let owned = outfp;\n        outfp = ::core::ptr::null_mut();\n        fclose(owned);\n        *__errno_location() = saved_errno;\n    }\n}\n'
assert text.count(anchor) == 1
text = text.replace(anchor, helper+anchor, 1)
anchor = '    set_program_name(*argv.offset(0isize));'
assert text.count(anchor) == 1
text = text.replace(anchor, anchor+'\n    let prior_error_prefix = error_print_progname;\n    if prior_error_prefix.is_none() {\n        error_print_progname = Some(rboxc_time_error_prefix);\n    }\n    atexit(rboxc_time_close_output);', 1)
# The original flushes an owned output file but leaves it open until process
# teardown. Close that stream after its final use, retaining the exit status.
anchor = '    fflush_unlocked(outfp);'
assert text.count(anchor) == 1
text = text.replace(anchor, anchor+'\n    rboxc_time_close_output();', 1)
# A failed exec exits with _exit(), which bypasses atexit handlers. Close
# only the child's private copy after the diagnostic, retaining saved errno.
anchor = '        _exit('
assert text.count(anchor) == 1
text = text.replace(anchor, '        rboxc_time_close_output();\n'+anchor, 1)
mapping = symbol_map(ROOT)
imports = []
def namespace(match):
    name = match['name']
    if name not in mapping:
        return match[0]
    imports.append(name)
    return '    #[link_name = "'+mapping[name]+'"]\n'+match[0]
text = re.sub(r'^    (?:fn|static mut) (?P<name>\w+)\b', namespace, text, flags=re.M)
assert imports and 'program_name' in imports
notice = re.match(r'\s*(/\*.*?\*/)', source.read_text(), re.S)[1]
target = ROOT/'src/generated/applet_time.rs'
target.write_text('// Generated from pinned GNU Time '+pin['version']+' by scripts/translate-time.py.\n'
                  '// Source SHA-256: '+pin['entry_sha256']+'\n'+notice+'\n'+declarations+text)
report = {'provider': 'time', 'version': pin['version'], 'command': 'time',
          'entry': 'single_binary_main_time', 'source': str(source),
          'source_sha256': pin['entry_sha256'], 'translated': True,
          'scope': 'C2Rust entry translation with namespaced native helper imports; compilation and suite validation are separate.',
          'rust_file': str(target.relative_to(ROOT)), 'rust_sha256': fingerprint(target),
          'raw_translation_sha256': fingerprint(outputs[0]), 'compile_database_sha256': fingerprint(database),
          'helper_imports': {name: mapping[name] for name in sorted(imports)},
          'adaptations': ['Preserve Time error prefixes through the public error callback.',
                          'Close the owned output stream on normal/fatal exit and failed child exec; preserve errno and child status.'],
          'opaque_pointer_types': opaque, 'log': str(log.relative_to(ROOT)), 'log_sha256': fingerprint(log)}
(ROOT/'evidence/time-translation.json').write_text(json.dumps(report, indent=2)+'\n')
print('Translated GNU Time entry with', len(imports), 'namespaced helper imports')
