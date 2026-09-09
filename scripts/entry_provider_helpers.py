"""Use a provider's recorded GNU link command to isolate its native helpers."""
# SPDX-License-Identifier: GPL-3.0-or-later
import hashlib
import json
import re
import subprocess

ENTRY_OBJECTS = {'frcode':'locate/frcode.o','gawk':'main.o','bash':'shell.o','patch':'src/patch.o',
    'less':'main.o','screen':'screen.o','wget':'src/main.o',
    'getconf':'posix/getconf.o','iconv':'iconv/iconv_prog.o',
    'ar':'binutils/not-ranlib.o','readelf':'binutils/readelf.o','strings':'binutils/strings.o',
    'dnsdomainname':'src/dnsdomainname.o','logger':'src/logger.o','inetd':'src/inetd.o',
    'syslogd':'src/syslogd.o','tftpd':'src/tftpd.o','traceroute':'src/traceroute.o',
    'ping':'ping/ping.o','ping6':'ping/ping6.o','ifconfig':'ifconfig/ifconfig.o',
    'telnetd':'telnetd/telnetd.o','tftp':'src/tftp.o','ftpd':'ftpd/ftpd.o','telnet':'telnet/main.o'}
PROVIDERS = {n: ('findutils' if n=='frcode' else 'glibc' if n in ('getconf','iconv') else 'binutils' if n in ('ar','readelf','strings') else
                 n if n in ('gawk','bash','patch','less','screen','wget') else 'inetutils') for n in ENTRY_OBJECTS}

def binary_path(root,command):
    relative={'gawk':'gawk','bash':'bash','ar':'binutils/ar','less':'less','wget':'src/wget','telnet':'telnet/telnet'}.get(command,ENTRY_OBJECTS[command][:-2])
    return root/f'build/gnu-{PROVIDERS[command]}'/relative

def fingerprint(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()

def original_link(root,provider):
    from pathlib import Path
    records=[json.loads(p.read_text()) for p in (root/f'build/{PROVIDERS[provider]}-cc-records').glob('*.json')]
    records=[r for r in records if r.get('kind')=='link' and
             (Path(r['directory'])/r['output']).resolve()==binary_path(root,provider)]
    assert len(records)==1
    return records[0]

def local_libraries(root,command):
    from pathlib import Path
    record=original_link(root,command)
    directories=[(Path(record['directory'])/w[2:]).resolve() for w in record['arguments'] if w.startswith('-L')]
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
    if PROVIDERS[provider]=='glibc':
        # The Rust toolchain supplies startup objects and the process libc.
        # Retain only GNU command helper modules, never CRT or libc archives.
        return [p for p in objects if p!=entry and p.parent==entry.parent]
    result=[p for p in objects if p!=entry]
    if provider in ('tftpd','tftp','ftpd','telnet','bash'):
        extra=root/f'build/translation/{provider}/native-helpers.o'
        assert extra.exists(), 'prepare the isolated command helper object before assembly'
        result.append(extra)
    return result

def defined_symbols(paths):
    if not paths:return set()
    output=subprocess.check_output(['nm','-g','--defined-only','--format=posix',*paths],text=True)
    return set(re.findall(r'^(\w+) [A-Z] ',output,re.M))

def symbol_map(root,provider):
    symbols=defined_symbols(native_inputs(root,provider))
    if provider=='bash':symbols.update(('add_unwind_protect_owned','discard_unwind_frame_heap'))
    assert 'main' not in symbols
    symbols |= defined_symbols([root/f'build/gnu-{PROVIDERS[provider]}'/ENTRY_OBJECTS[provider]])-{'main'}
    return {s:f'rboxc_{provider}_'+s for s in sorted(symbols)}

def prepare_archives(root,provider,mapping):
    adapted={}
    if provider=='bash':
        from bash_cleanup import prepare
        adapted=prepare(root)
        from bash_return_trap_cleanup import prepare as prepare_return_trap
        prepare_return_trap(root, adapted)
        from bash_trap_restart_cleanup import prepare as prepare_trap_restart
        prepare_trap_restart(root, adapted)
        from bash_parser_cleanup import prepare as prepare_parser_cleanup
        prepare_parser_cleanup(root, adapted)
        from bash_subshell_cleanup import prepare as prepare_subshell_cleanup
        prepare_subshell_cleanup(root, adapted)
        from bash_globstar_cleanup import prepare as prepare_globstar_cleanup
        prepare_globstar_cleanup(root, adapted)
        from bash_assignment_cleanup import prepare as prepare_assignment_cleanup
        prepare_assignment_cleanup(root, adapted)
    if provider=='gawk':
        from gawk_cleanup import prepare
        adapted=prepare(root)
        from gawk_format_cleanup import prepare as prepare_format
        adapted.update(prepare_format(root))
    if provider=='patch':
        from patch_cleanup import prepare
        adapted=prepare(root)
    if provider=='ifconfig':
        from ifconfig_cleanup import prepare
        adapted=prepare(root)
    if provider in ('screen','wget'):
        from terminal_http_cleanup import prepare
        adapted=prepare(root,provider)
    if provider=='screen':
        from screen_key_cleanup import prepare
        adapted.update(prepare(root))
        from screen_daemon_cleanup import prepare
        adapted.update(prepare(root))
    if provider=='less':
        from less_cleanup import prepare
        adapted=prepare(root)
    stage=root/f'build/translation/{provider}';stage.mkdir(parents=True,exist_ok=True)
    definitions=stage/'helper-symbol-map'
    definitions.write_text(''.join(f'{old} {new}\n' for old,new in mapping.items()))
    outputs=[]
    for index,source in enumerate(native_inputs(root,provider)):
        target=root/'build/helpers'/f'{provider}-{index:03}-{source.name}'
        temporary=target.with_name(target.name+'.tmp')
        subprocess.run(['objcopy','--redefine-syms='+str(definitions),*(['--redefine-sym=dup2=rboxc_bash_owned_dup2','--redefine-sym=close=rboxc_bash_owned_close','--redefine-sym=pipe=rboxc_bash_owned_pipe'] if provider=='bash' else []),adapted.get(source.name,source),temporary],check=True)
        if source.suffix=='.a':subprocess.run(['ranlib',temporary],check=True)
        temporary.replace(target);outputs.append(target)
    expected=defined_symbols(native_inputs(root,provider))
    if provider=='bash':expected.update(('add_unwind_protect_owned','discard_unwind_frame_heap'))
    assert defined_symbols(outputs)=={mapping[s] for s in expected}
    return outputs,definitions
