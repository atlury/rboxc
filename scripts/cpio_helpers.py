"""Namespace each GNU Cpio command and its native helper state independently."""
# SPDX-License-Identifier: GPL-3.0-or-later
import hashlib
import re
import subprocess

COMMANDS = ('cpio', 'mt')
ENTRY_OBJECTS = {'cpio': 'src/main.o', 'mt': 'src/mt.o'}
CPIO_HELPERS = ('copyin', 'copyout', 'copypass', 'defer', 'dstring', 'global',
                'fatal', 'tar', 'util', 'filemode', 'idcache', 'makepath', 'userspec')

def fingerprint(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()

def native_inputs(root, command):
    assert command in COMMANDS
    build = root/'build/gnu-cpio'
    assert {p.name for p in (build/'src').glob('*.o')} == {n+'.o' for n in CPIO_HELPERS} | {'main.o', 'mt.o'}
    objects = [build/'src'/(n+'.o') for n in CPIO_HELPERS] if command == 'cpio' else []
    return [*objects, build/'lib/libpax.a', build/'gnu/libgnu.a']

def defined_symbols(paths):
    output = subprocess.check_output(['nm', '-g', '--defined-only', '--format=posix', *paths], text=True)
    return set(re.findall(r'^(\w+) [A-Z] ', output, re.M))

def symbol_map(root, command):
    symbols = defined_symbols(native_inputs(root, command))
    assert 'main' not in symbols
    symbols |= defined_symbols([root/'build/gnu-cpio'/ENTRY_OBJECTS[command]]) - {'main'}
    return {name: f'rboxc_cpio_{command}_{name}' for name in sorted(symbols)}

def prepare_archives(root, command, mapping):
    from cpio_cleanup import prepare
    adapted = prepare(root) if command == 'cpio' else None
    stage = root/'build/translation/cpio'/command
    stage.mkdir(parents=True, exist_ok=True)
    definitions = stage/'helper-symbol-map'
    definitions.write_text(''.join(f'{old} {new}\n' for old, new in mapping.items()))
    outputs = []
    for source in native_inputs(root, command):
        target = root/'build/helpers'/f'cpio-{command}-{source.name}'
        temporary = target.with_name(target.name+'.tmp')
        compiled = adapted if source.name == 'copypass.o' else source
        subprocess.run(['objcopy', '--redefine-syms='+str(definitions), compiled, temporary], check=True)
        if source.suffix == '.a':
            subprocess.run(['ranlib', temporary], check=True)
        temporary.replace(target)
        outputs.append(target)
    assert defined_symbols(outputs) == {mapping[s] for s in defined_symbols(native_inputs(root, command))}
    return outputs, definitions
