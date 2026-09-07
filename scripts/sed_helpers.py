"""Namespace Sed helper definitions and Rust-owned matcher state together."""
# SPDX-License-Identifier: GPL-3.0-or-later
import hashlib
import re
import subprocess

COMMANDS = ('sed',)
HELPER_OBJECTS = ('compile', 'debug', 'execute', 'mbcs', 'regexp', 'utils')


def fingerprint(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def native_inputs(root):
    build = root/'build/gnu-sed'
    return [build/'lib/libsed.a', build/'sed/libver.a',
            *[build/f'sed/sed-{name}.o' for name in HELPER_OBJECTS]]


def defined_symbols(paths):
    output = subprocess.check_output(['nm', '-g', '--defined-only', '--format=posix', *paths], text=True)
    return set(re.findall(r'^(\w+) [A-Z] ', output, re.M))


def symbol_map(root):
    symbols = defined_symbols(native_inputs(root))
    assert 'main' not in symbols
    for name in COMMANDS:
        symbols |= defined_symbols([root/f'build/gnu-sed/sed/sed-{name}.o']) - {'main'}
    assert not any(s.startswith('single_binary_main_') for s in symbols)
    return {s: 'rboxc_sed_'+s for s in sorted(symbols)}


def prepare_archives(root, mapping):
    stage = root/'build/translation/sed'
    stage.mkdir(parents=True, exist_ok=True)
    definitions = stage/'helper-symbol-map'
    definitions.write_text(''.join(f'{old} {new}\n' for old, new in mapping.items()))
    outputs = []
    prepared = []
    for original in native_inputs(root):
        prepared.append(original)
        target = root/'build/helpers'/('sed-'+original.name)
        temporary = target.with_name(target.name+'.tmp')
        subprocess.run(['objcopy', '--redefine-syms='+str(definitions), original, temporary], check=True)
        if original.suffix == '.a':
            subprocess.run(['ranlib', temporary], check=True)
        temporary.replace(target)
        outputs.append(target)
    assert defined_symbols(outputs) == {mapping[s] for s in defined_symbols(prepared)}
    return outputs, definitions
