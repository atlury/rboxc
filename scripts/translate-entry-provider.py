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
    if symbol.startswith('rboxc_'+name+'_'):
        return match[0]
    assert symbol in mapping
    exports.append(symbol)
    return '#[export_name = "'+mapping[symbol]+'"]'
text = re.sub(r'#\[export_name = "(?P<name>\w+)"\]', renamed_export, text)
assert set(exports) == defined_symbols([ROOT/f'build/gnu-{provider}'/ENTRY_OBJECTS[name]]) - {'main'}
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
(ROOT/f'evidence/{name}-translation.json').write_text(json.dumps(report,indent=2)+'\n')
print('Translated GNU',name,'with',len(imports),'helper imports and',len(exports),'Rust exports')
