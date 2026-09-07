"""Namespace the pinned GNU Time helper definitions, excluding its C entry."""
# SPDX-License-Identifier: GPL-3.0-or-later
import hashlib
import re
import subprocess


def fingerprint(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def native_inputs(root):
    return [root/'build/gnu-time/src/libver.a', root/'build/gnu-time/lib/libtime.a',
            root/'build/gnu-time/src/rusage-kb.o']


def defined_symbols(paths):
    output = subprocess.check_output(['nm', '-g', '--defined-only', '--format=posix', *paths], text=True)
    return set(re.findall(r'^(\w+) [A-Z] ', output, re.M))


def symbol_map(root):
    symbols = defined_symbols(native_inputs(root))
    assert symbols and 'main' not in symbols
    assert not any(s.startswith('single_binary_main_') for s in symbols)
    return {s: 'rboxc_time_'+s for s in sorted(symbols)}


def prepare_archives(root, mapping):
    stage = root/'build/translation/time'
    stage.mkdir(parents=True, exist_ok=True)
    definitions = stage/'helper-symbol-map'
    definitions.write_text(''.join(f'{old} {new}\n' for old, new in mapping.items()))
    outputs = []
    for original in native_inputs(root):
        target = root/'build/helpers'/('time-'+original.name)
        temporary = target.with_name(target.name+'.tmp')
        subprocess.run(['objcopy', '--redefine-syms='+str(definitions), original, temporary], check=True)
        if original.suffix == '.a':
            subprocess.run(['ranlib', temporary], check=True)
        temporary.replace(target)
        outputs.append(target)
    assert defined_symbols(outputs) == set(mapping.values())
    return outputs, definitions
