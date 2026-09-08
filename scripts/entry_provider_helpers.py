"""Use a provider's recorded GNU link command to isolate its native helpers."""
# SPDX-License-Identifier: GPL-3.0-or-later
import hashlib
import json
import re
import subprocess

ENTRY_OBJECTS = {'gawk':'main.o','bash':'shell.o','patch':'src/patch.o',
    'less':'main.o','screen':'screen.o','wget':'src/main.o',
    'ar':'binutils/not-ranlib.o','readelf':'binutils/readelf.o','strings':'binutils/strings.o',
    'dnsdomainname':'src/dnsdomainname.o','logger':'src/logger.o','inetd':'src/inetd.o',
    'syslogd':'src/syslogd.o','tftpd':'src/tftpd.o','traceroute':'src/traceroute.o',
    'ping':'ping/ping.o','ping6':'ping/ping6.o','ifconfig':'ifconfig/ifconfig.o',
    'telnetd':'telnetd/telnetd.o'}
PROVIDERS = {n: ('binutils' if n in ('ar','readelf','strings') else
                 n if n in ('gawk','bash','patch','less','screen','wget') else 'inetutils') for n in ENTRY_OBJECTS}

def binary_path(root,command):
    relative={'gawk':'gawk','bash':'bash','ar':'binutils/ar','less':'less','wget':'src/wget'}.get(command,ENTRY_OBJECTS[command][:-2])
    return root/f'build/gnu-{PROVIDERS[command]}'/relative

def fingerprint(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()

def original_link(root,provider):
    from pathlib import Path
    records=[json.loads(p.read_text()) for p in (root/f'build/{PROVIDERS[provider]}-cc-records').glob('*.json')]
    records=[r for r in records if r['kind']=='link' and
             (Path(r['directory'])/r['output']).resolve()==binary_path(root,provider)]
    assert len(records)==1
    return records[0]

def local_libraries(root,command):
    from pathlib import Path
    record=original_link(root,command)
    directories=[Path(w[2:]).resolve() for w in record['arguments'] if w.startswith('-L')]
    result={}
    for word in record['arguments']:
        if word.startswith('-l'):
            for directory in directories:
                path=directory/('lib'+word[2:]+'.a')
                if path.exists():
                    assert path.is_relative_to(root/'build')
                    result[word]=path;break
    return result

def native_inputs(root,provider):
    from pathlib import Path
    record=original_link(root,provider)
    libraries=local_libraries(root,provider)
    objects=[libraries[word] if word in libraries else (Path(record['directory'])/word).resolve()
             for word in record['arguments'][1:] if word.endswith(('.o','.a')) or word in libraries]
    entry=root/f'build/gnu-{PROVIDERS[provider]}'/ENTRY_OBJECTS[provider]
    assert objects.count(entry)==1
    return [p for p in objects if p!=entry]

def defined_symbols(paths):
    output=subprocess.check_output(['nm','-g','--defined-only','--format=posix',*paths],text=True)
    return set(re.findall(r'^(\w+) [A-Z] ',output,re.M))

def symbol_map(root,provider):
    symbols=defined_symbols(native_inputs(root,provider))
    assert 'main' not in symbols
    symbols |= defined_symbols([root/f'build/gnu-{PROVIDERS[provider]}'/ENTRY_OBJECTS[provider]])-{'main'}
    return {s:f'rboxc_{provider}_'+s for s in sorted(symbols)}

def prepare_archives(root,provider,mapping):
    adapted={}
    if provider=='gawk':
        from gawk_cleanup import prepare
        adapted=prepare(root)
    if provider=='patch':
        from patch_cleanup import prepare
        adapted=prepare(root)
    if provider=='ifconfig':
        from ifconfig_cleanup import prepare
        adapted=prepare(root)
    stage=root/f'build/translation/{provider}';stage.mkdir(parents=True,exist_ok=True)
    definitions=stage/'helper-symbol-map'
    definitions.write_text(''.join(f'{old} {new}\n' for old,new in mapping.items()))
    outputs=[]
    for index,source in enumerate(native_inputs(root,provider)):
        target=root/'build/helpers'/f'{provider}-{index:03}-{source.name}'
        temporary=target.with_name(target.name+'.tmp')
        subprocess.run(['objcopy','--redefine-syms='+str(definitions),adapted.get(source.name,source),temporary],check=True)
        if source.suffix=='.a':subprocess.run(['ranlib',temporary],check=True)
        temporary.replace(target);outputs.append(target)
    assert defined_symbols(outputs)=={mapping[s] for s in defined_symbols(native_inputs(root,provider))}
    return outputs,definitions
