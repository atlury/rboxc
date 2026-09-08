"""Isolate GNU Ed helpers and their references to Rust-owned definitions."""
# SPDX-License-Identifier: GPL-3.0-or-later
import hashlib
import re
import subprocess


def fingerprint(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def native_inputs(root):
    return [root/'build/gnu-ed'/name for name in ('buffer.o','carg_parser.o','global.o','io.o','main_loop.o','regex.o','signal.o')]


def defined_symbols(paths):
    output = subprocess.check_output(['nm', '-g', '--defined-only', '--format=posix', *paths], text=True)
    return set(re.findall(r'^(\w+) [A-Z] ', output, re.M))


def symbol_map(root):
    symbols = defined_symbols([*native_inputs(root), root/'build/gnu-ed/main.o'])
    assert 'main' in symbols
    symbols.remove('main')
    assert not any(s.startswith('single_binary_main_') for s in symbols)
    return {s: 'rboxc_ed_'+s for s in sorted(symbols)}


def prepare_archive(root, mapping):
    stage = root/'build/translation/ed'
    stage.mkdir(parents=True, exist_ok=True)
    definitions = stage/'helper-symbol-map'
    definitions.write_text(''.join(f'{old} {new}\n' for old, new in mapping.items()))
    objects = []
    for original in native_inputs(root):
        target = stage/original.name
        subprocess.run(['objcopy', '--redefine-syms='+str(definitions), original, target], check=True)
        objects.append(target)
    archive = root/'build/helpers/libed-rboxc.a'
    temporary = archive.with_suffix('.tmp.a')
    if temporary.exists():
        temporary.unlink()
    subprocess.run(['ar', 'crs', temporary, *objects], check=True)
    assert defined_symbols([temporary]) == {mapping[s] for s in defined_symbols(native_inputs(root))}
    temporary.replace(archive)
    return archive, definitions
