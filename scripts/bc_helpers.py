"""Keep BC and DC helper definitions and translated state private to each command."""
# SPDX-License-Identifier: GPL-3.0-or-later
import hashlib
import re
import subprocess

COMMANDS=('bc','dc')
HELPER_OBJECTS={'bc':('bc','scan','execute','load','storage','util','global','warranty'),
                'dc':('input','misc','eval','stack','array','numeric','string')}


def fingerprint(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def native_inputs(root, command):
    build=root/'build/gnu-bc'
    return [build/'lib/libbc.a',*[build/command/(name+'.o') for name in HELPER_OBJECTS[command]]]


def defined_symbols(paths):
    output=subprocess.check_output(['nm','-g','--defined-only','--format=posix',*paths],text=True)
    return set(re.findall(r'^(\w+) [A-Z] ',output,re.M))


def symbol_map(root, command):
    symbols=defined_symbols(native_inputs(root, command))
    assert 'main' not in symbols
    symbols |= defined_symbols([root/'build/gnu-bc'/command/('main.o' if command=='bc' else 'dc.o')])-{'main'}
    assert not any(s.startswith('single_binary_main_') for s in symbols)
    return {s:'rboxc_bc_'+command+'_'+s for s in sorted(symbols)}


def prepare_archives(root,command,mapping):
    stage=root/'build/translation/bc'/command;stage.mkdir(parents=True,exist_ok=True)
    definitions=stage/'helper-symbol-map'
    definitions.write_text(''.join(f'{old} {new}\n' for old,new in mapping.items()))
    outputs=[]
    for original in native_inputs(root,command):
        target=root/'build/helpers'/('bc-'+command+'-'+original.name)
        temporary=target.with_name(target.name+'.tmp')
        subprocess.run(['objcopy','--redefine-syms='+str(definitions),original,temporary],check=True)
        if original.suffix=='.a':subprocess.run(['ranlib',temporary],check=True)
        temporary.replace(target);outputs.append(target)
    assert defined_symbols(outputs)=={mapping[s] for s in defined_symbols(native_inputs(root,command))}
    return outputs,definitions
