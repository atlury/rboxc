"""Namespace Grep helper definitions and Rust-owned matcher state together."""
# SPDX-License-Identifier: GPL-3.0-or-later
import hashlib
import re
import subprocess

COMMANDS = ('grep',)
HELPER_OBJECTS = ('dfasearch', 'kwsearch', 'kwset', 'searchutils', 'pcresearch')


def fingerprint(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def native_inputs(root):
    build = root/'build/gnu-grep'
    return [build/'lib/libgreputils.a',
            *[build/f'src/{name}.o' for name in HELPER_OBJECTS]]


def defined_symbols(paths):
    output = subprocess.check_output(['nm', '-g', '--defined-only', '--format=posix', *paths], text=True)
    return set(re.findall(r'^(\w+) [A-Z] ', output, re.M))


def symbol_map(root):
    symbols = defined_symbols(native_inputs(root))
    assert 'main' not in symbols
    for name in COMMANDS:
        symbols |= defined_symbols([root/f'build/gnu-grep/src/{name}.o']) - {'main'}
    assert not any(s.startswith('single_binary_main_') for s in symbols)
    symbols |= {'GEAfree', 'Ffree', 'kwsfree_owned'}
    return {s: 'rboxc_grep_'+s for s in sorted(symbols)}


def prepare_archives(root, mapping):
    from grep_cleanup import prepare
    replacements = prepare(root)
    stage = root/'build/translation/grep'
    stage.mkdir(parents=True, exist_ok=True)
    definitions = stage/'helper-symbol-map'
    definitions.write_text(''.join(f'{old} {new}\n' for old, new in mapping.items()))
    outputs = []
    prepared = []
    for original in native_inputs(root):
        original = replacements.get(original.stem, original)
        prepared.append(original)
        target = root/'build/helpers'/('grep-'+original.name)
        temporary = target.with_name(target.name+'.tmp')
        subprocess.run(['objcopy', '--redefine-syms='+str(definitions), original, temporary], check=True)
        if original.suffix == '.a':
            subprocess.run(['ranlib', temporary], check=True)
        temporary.replace(target)
        outputs.append(target)
    assert defined_symbols(outputs) == {mapping[s] for s in defined_symbols(prepared)}
    return outputs, definitions
