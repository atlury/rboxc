"""Keep each Sharutils entry and its AutoOpts/Gnulib helpers private."""
# SPDX-License-Identifier: GPL-3.0-or-later
import hashlib
import re
import subprocess

COMMANDS = ('uuencode', 'uudecode')
def fingerprint(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()

def native_inputs(root, command):
    assert command in COMMANDS
    build = root/'build/gnu-sharutils'
    return [build/f'src/{command}-opts.o', build/'libopts/libopts.a', build/'lib/libgnu.a']

def defined_symbols(paths):
    output = subprocess.check_output(['nm', '-g', '--defined-only', '--format=posix', *paths], text=True)
    return set(re.findall(r'^(\w+) [A-Z] ', output, re.M))

def symbol_map(root, command):
    symbols = defined_symbols(native_inputs(root, command))
    assert 'main' not in symbols
    symbols |= defined_symbols([root/f'build/gnu-sharutils/src/{command}.o']) - {'main'}
    return {name: f'rboxc_sharutils_{command}_{name}' for name in sorted(symbols)}

def prepare_archives(root, command, mapping):
    stage = root/'build/translation/sharutils'/command
    stage.mkdir(parents=True, exist_ok=True)
    definitions = stage/'helper-symbol-map'
    definitions.write_text(''.join(f'{old} {new}\n' for old, new in mapping.items()))
    outputs = []
    for source in native_inputs(root, command):
        target = root/'build/helpers'/f'sharutils-{command}-{source.name}'
        temporary = target.with_name(target.name+'.tmp')
        subprocess.run(['objcopy', '--redefine-syms='+str(definitions), source, temporary], check=True)
        if source.suffix == '.a':
            subprocess.run(['ranlib', temporary], check=True)
        temporary.replace(target)
        outputs.append(target)
    assert defined_symbols(outputs) == {mapping[s] for s in defined_symbols(native_inputs(root, command))}
    return outputs, definitions
