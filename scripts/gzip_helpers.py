"""Namespace Gzip helper definitions and Rust-owned matcher state together."""
# SPDX-License-Identifier: GPL-3.0-or-later
import hashlib
import re
import subprocess

COMMANDS = ('gzip',)
HELPER_OBJECTS = ('bits', 'deflate', 'inflate', 'trees', 'unlzh', 'unlzw', 'unpack', 'unzip', 'util', 'zip')


def fingerprint(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def native_inputs(root):
    build = root/'build/gnu-gzip'
    return [build/'lib/libgzip.a', build/'libver.a',
            *[build/f'{name}.o' for name in HELPER_OBJECTS]]


def defined_symbols(paths):
    output = subprocess.check_output(['nm', '-g', '--defined-only', '--format=posix', *paths], text=True)
    return set(re.findall(r'^(\w+) [A-Z] ', output, re.M))


def symbol_map(root):
    symbols = defined_symbols(native_inputs(root))
    assert 'main' not in symbols
    for name in COMMANDS:
        symbols |= defined_symbols([root/f'build/gnu-gzip/{name}.o']) - {'main'}
    assert not any(s.startswith('single_binary_main_') for s in symbols)
    return {s: 'rboxc_gzip_'+s for s in sorted(symbols)}


def prepare_archives(root, mapping):
    stage = root/'build/translation/gzip'
    stage.mkdir(parents=True, exist_ok=True)
    definitions = stage/'helper-symbol-map'
    definitions.write_text(''.join(f'{old} {new}\n' for old, new in mapping.items()))
    outputs = []
    prepared = []
    for original in native_inputs(root):
        prepared.append(original)
        target = root/'build/helpers'/('gzip-'+original.name)
        temporary = target.with_name(target.name+'.tmp')
        subprocess.run(['objcopy', '--redefine-syms='+str(definitions), original, temporary], check=True)
        if original.suffix == '.a':
            subprocess.run(['ranlib', temporary], check=True)
        temporary.replace(target)
        outputs.append(target)
    assert defined_symbols(outputs) == {mapping[s] for s in defined_symbols(prepared)}
    return outputs, definitions
