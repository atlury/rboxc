#!/usr/bin/env python3
"""Translate the pinned GNU Gzip entry and namespace its helper imports."""
# SPDX-License-Identifier: GPL-3.0-or-later
import json
import os
from pathlib import Path
import re
import subprocess
from gzip_helpers import COMMANDS, defined_symbols, fingerprint, symbol_map

ROOT = Path(__file__).resolve().parents[1]
import argparse
parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('command', choices=COMMANDS)
name = parser.parse_args().command
pin = json.loads((ROOT/'inventory/sources.json').read_text())['gzip']
source = Path(os.environ.get('GNU_GZIP_SOURCE', pin['source']))/f'{name}.c'
expected = pin['entry_sha256']
assert fingerprint(source) == expected
records = [json.loads(p.read_text()) for p in (ROOT/'build/gzip-cc-records').glob('*.json')]
records = [r for r in records if r['output'] == name+'.o']
assert len(records) == 1
command = records[0]
assert Path(command['file']) == source
command['arguments'][0] = 'clang-21'
command['arguments'] += ['-std=gnu17', '-Dunreachable()=__builtin_unreachable()',
                         '-Dalignof=_Alignof', '-Dalignas=_Alignas', '-Dnullptr=((void*)0)', '-Dmain=single_binary_main_'+name]
stage = ROOT/'build/translation/gzip'/name
stage.mkdir(parents=True, exist_ok=True)
database = stage/'compile_commands.json'
database.write_text(json.dumps([command], indent=2)+'\n')
log = ROOT/f'evidence/raw/transpile-gzip-{name}.log'
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
# C2Rust drops the three C buffer alignments; retain their ABI addresses
# with repr(C) wrappers whose byte-array field starts at offset zero.
for buffer_name, size in [('inbuf', 262208), ('outbuf', 264192), ('window', 65536)]:
    anchor = f'pub static mut {buffer_name}: [uch; {size}] = [0; {size}];'
    assert text.count(anchor) == 1
    text = text.replace(anchor, f'pub static mut {buffer_name}: RboxcGzipAligned<{size}> = RboxcGzipAligned([0; {size}]);', 1)
text, indexed_buffers = re.subn(r'\b(inbuf|outbuf|window)\[', r'\1.0[', text)
assert indexed_buffers == 20
declarations += '#[repr(C, align(4096))]\npub struct RboxcGzipAligned<const N: usize>([uch; N]);\n'
# Keep Gnulib allocation diagnostics tied to the dispatched applet name.
anchor = '#[no_mangle]\npub unsafe extern "C" fn single_binary_main_'+name+'('
helper = (ROOT/'src/bridges/gzip-invocation.rs').read_text()+'\n'
assert text.count(anchor) == 1
text = text.replace(anchor, helper+anchor, 1)
anchor = '    program_name = gzip_base_name(*argv.offset(0isize));'
assert text.count(anchor) == 1
text = text.replace(anchor, '    rboxc_gzip_set_invocation(*argv);\n'+anchor, 1)
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
    if symbol.startswith('rboxc_gzip_'):
        return match[0]
    assert symbol in mapping
    exports.append(symbol)
    return '#[export_name = "'+mapping[symbol]+'"]'
text = re.sub(r'#\[export_name = "(?P<name>\w+)"\]', renamed_export, text)
expected_exports = defined_symbols([ROOT/f'build/gnu-gzip/{name}.o']) - {'main'}
assert set(exports) == expected_exports, (set(exports)-expected_exports, expected_exports-set(exports))
notice = re.match(r'\s*(/\*.*?\*/)', source.read_text(), re.S)[1]
for alias in ('gunzip', 'zcat'):
    script = Path(pin['source'])/(alias+'.in')
    assert fingerprint(script) == pin['alias_source_sha256'][alias]
    for variable in ('version', 'usage'):
        value = re.search(r'^'+variable+r'="(.*?)"$', script.read_text(), re.S | re.M)[1]
        value = value.replace('@VERSION@', pin['version'])
        if variable == 'usage':
            assert value.startswith('Usage: $0')
            value = value[len('Usage: $0'):]
        data = value.encode()+b'\0'
        text += '\nconst RBOXC_'+alias.upper()+'_'+variable.upper()+': &[u8] = &['+','.join(map(str,data))+'];\n'
text += '\n'+(ROOT/'src/bridges/gzip-aliases.rs').read_text()+'\n'
target = ROOT/f'src/generated/applet_{name}.rs'
target.write_text('// Generated from pinned GNU Gzip '+pin['version']+' by scripts/translate-gzip.py.\n'
                  '// Source SHA-256: '+expected+'\n'+notice+'\n'+declarations+text)
report = {'provider': 'gzip', 'version': pin['version'], 'command': name,
          'entry': 'single_binary_main_'+name, 'source': str(source),
          'source_sha256': expected, 'translated': True,
          'scope': 'C2Rust entry translation with namespaced native helper imports; compilation and suite validation are separate.',
          'rust_file': str(target.relative_to(ROOT)), 'rust_sha256': fingerprint(target),
          'alias_adapter_sha256': fingerprint(ROOT/'src/bridges/gzip-aliases.rs'),
          'buffer_alignment': 4096,
          'invocation_adapter_sha256': fingerprint(ROOT/'src/bridges/gzip-invocation.rs'),
          'raw_translation_sha256': fingerprint(outputs[0]), 'compile_database_sha256': fingerprint(database),
          'helper_imports': {name: mapping[name] for name in sorted(imports)},
          'rust_exports': {symbol: mapping[symbol] for symbol in sorted(exports)},
          'adaptations': ['GNU17 parser adaptation maps C23 nullptr to a null pointer constant.',
                          'Use pinned-nightly VaList and exposed-provenance API spellings.',
                          'Preserve glibc invocation names for native allocation diagnostics.',
                          'Namespace native helpers and Rust-owned state together.'],
          'opaque_pointer_types': opaque, 'log': str(log.relative_to(ROOT)), 'log_sha256': fingerprint(log)}
(ROOT/'evidence/gzip-translation.json').write_text(json.dumps(report, indent=2)+'\n')
print('Translated GNU Gzip', name, 'with', len(imports), 'helper imports and', len(exports), 'Rust exports')
