"""Keep Findutils helper definitions and translated state private to each command."""
# SPDX-License-Identifier: GPL-3.0-or-later
import hashlib
import re
import subprocess

COMMANDS=('find','xargs','locate')
ENTRY_OBJECTS={'find':'find/ftsfind.o','xargs':'xargs/xargs.o','locate':'locate/locate.o'}


def fingerprint(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def native_inputs(root, command):
    build=root/'build/gnu-findutils'
    extra={'find':[build/'find/libfindtools.a'],'xargs':[],'locate':[build/'locate/word_io.o']}[command]
    return [*extra,build/'lib/libfind.a',build/'gl/lib/libgnulib.a']


def defined_symbols(paths):
    output=subprocess.check_output(['nm','-g','--defined-only','--format=posix',*paths],text=True)
    return set(re.findall(r'^(\w+) [A-Z] ',output,re.M))


def symbol_map(root, command):
    symbols=defined_symbols(native_inputs(root, command))
    assert 'main' not in symbols
    symbols |= defined_symbols([root/'build/gnu-findutils'/ENTRY_OBJECTS[command]])-{'main'}
    assert not any(s.startswith('single_binary_main_') for s in symbols)
    return {s:'rboxc_findutils_'+command+'_'+s for s in sorted(symbols)}


def prepare_archives(root,command,mapping):
    from findutils_cleanup import prepare
    cleaned_find = prepare(root)
    stage=root/'build/translation/findutils'/command;stage.mkdir(parents=True,exist_ok=True)
    definitions=stage/'helper-symbol-map'
    definitions.write_text(''.join(f'{old} {new}\n' for old,new in mapping.items()))
    outputs=[]
    for original in native_inputs(root,command):
        target=root/'build/helpers'/('findutils-'+command+'-'+original.name)
        temporary=target.with_name(target.name+'.tmp')
        selected = cleaned_find.get(original.name, original)
        subprocess.run(['objcopy','--redefine-syms='+str(definitions),selected,temporary],check=True)
        if original.suffix=='.a':subprocess.run(['ranlib',temporary],check=True)
        temporary.replace(target);outputs.append(target)
    assert defined_symbols(outputs)=={mapping[s] for s in defined_symbols(native_inputs(root,command))}
    return outputs,definitions
