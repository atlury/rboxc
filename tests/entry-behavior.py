#!/usr/bin/env python3
"""Compare selected GNU entries on ordinary valid local fixtures."""
# SPDX-License-Identifier: GPL-3.0-or-later
import importlib.util
import json
import os
from pathlib import Path
import re
import stat
import shutil
import subprocess
import sys
import tempfile
import time
from comparison_profile import ComparisonProfile, fingerprint

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT/'tests/gnu'))
spec = importlib.util.spec_from_file_location('reviewed', ROOT/'tests/gnu/reviewed-original.py')
runner = importlib.util.module_from_spec(spec); spec.loader.exec_module(runner)
profile = ComparisonProfile('entry-behavior', selections=True)
sys.path.insert(0,str(ROOT/'scripts'))
from entry_provider_helpers import binary_path
selected = profile.options.commands or ['patch','ar','readelf','strings','dnsdomainname','logger','inetd','syslogd','traceroute','ping','ping6','ifconfig','telnetd']
oracles = {n:binary_path(ROOT,n) for n in selected}
hashes = {n:fingerprint(p) for n,p in oracles.items()}
driver_hash = fingerprint(Path(__file__))
cases=[]
def case(name,command,args,data=b'',*,files=None,multicall=False):
    if command in oracles:cases.append((name,command,args,data,files or {},multicall))
for command in selected:
    for option in ('help','version'):
        case(command+'-'+option,command,['--'+option])
        case(command+'-'+option+'-multicall',command,['--'+option],multicall=True)
    case(command+'-unknown-option',command,['--not-an-option'])
patch=b'--- input\n+++ input\n@@ -1,3 +1,3 @@\n alpha\n-beta\n+BETA\n gamma\n'
initial=b'alpha\nbeta\ngamma\n'
changed=b'alpha\nBETA\ngamma\n'
for name,args in [('apply',[]),('dry-run',['--dry-run']),('backup',['-b']),('numbered-backup',['-b','-V','numbered']),('suffix',['-b','-z','.old']),('output',['-o','output']),('silent',['-s']),('verbose',['--verbose']),('posix',['--posix']),('input-file',['-i','change.diff']),('strip',['-p0'])]:
    files={'input':initial}
    if name=='input-file':files['change.diff']=patch
    case('patch-'+name,'patch',args,b'' if name=='input-file' else patch,files=files)
case('patch-reverse','patch',['-R'],patch,files={'input':changed})
case('patch-multicall-apply','patch',[],patch,files={'input':initial},multicall=True)
case('patch-missing-patch','patch',['-i','missing'])
case('patch-missing-output-parent','patch',['-o','missing/output'],patch,files={'input':initial})
case('patch-create','patch',['-p0'],b'--- /dev/null\n+++ input\n@@ -0,0 +1 @@\n+created\n')
case('patch-delete','patch',['-p0','-E'],b'--- input\n+++ /dev/null\n@@ -1 +0,0 @@\n-deleted\n',files={'input':b'deleted\n'})
case('patch-context','patch',[],b'*** input\n--- input\n***************\n*** 1,3 ****\n  alpha\n! beta\n  gamma\n--- 1,3 ----\n  alpha\n! BETA\n  gamma\n',files={'input':initial})
for args in [[],['-a'],['-n','3'],['-t','x'],['-t','d'],['-w'],['-s','|']]:
    case('strings-data-'+str(len(cases)),'strings',args+['input'],files={'input':b'\x00alpha\x00beta\nGamma delta\x00xyz\x00'})
case('strings-stdin','strings',[],b'alpha\x00beta\x00')
case('strings-missing','strings',['missing'])
# Ordinary compiler-produced ELF object; no crafted object or corruption cases.
fixture=ROOT/'build/entry-valid-fixtures';fixture.mkdir(exist_ok=True)
source=fixture/'sample.c';source.write_text('const char message[]="rboxc fixture"; int square(int n) { return n*n; }\n')
subprocess.run(['gcc','-g','-c','-o',str(fixture/'sample.o'),str(source)],check=True)
elf=(fixture/'sample.o').read_bytes()
for name,args in [('header',['-h']),('sections',['-S']),('symbols',['-s']),('relocations',['-r']),('all',['-a']),('wide',['-W','-a']),('notes',['-n'])]:
    case('readelf-'+name,'readelf',args+['input.o'],files={'input.o':elf})
case('readelf-missing','readelf',['missing'])
for name,args in [('create',['rc','output.a','one','two']),('create-object',['rcs','output.a','input.o']),('verbose',['rcv','output.a','one','two']),('quick',['qc','output.a','one']),('deterministic',['rcD','output.a','one','two']),('deterministic-object',['rcsD','output.a','input.o'])]:
    case('ar-'+name,'ar',args,files={'one':b'first\n','two':b'second\n','input.o':elf})
case('ar-missing','ar',['t','missing.a'])
case('ifconfig-list','ifconfig',['--list'])
case('less-stdin','less',[],b'alpha\nbeta\n')
case('less-file','less',['input'],files={'input':b'alpha\nbeta\n'})
case('less-two-files','less',['one','two'],files={'one':b'alpha\n','two':b'beta\n'})
case('less-raw-color','less',['-R','input'],files={'input':b'\x1b[31mred\x1b[0m\n'})
case('less-missing','less',['missing'])
case('wget-no-url','wget',['--no-config'])
case('wget-file-scheme','wget',['--no-config','file:///missing'])
case('wget-missing-input','wget',['--no-config','-i','missing'])
case('wget-invalid-timeout','wget',['--no-config','--timeout=abc'])
# A private loopback HTTP fixture supplies only fixed, valid responses.
if 'wget' in oracles:
    import http.server
    import threading
    class FixtureHandler(http.server.BaseHTTPRequestHandler):
        def log_message(self,*args):pass
        def do_GET(self):
            if self.path=='/redirect':
                self.send_response(302);self.send_header('Location','/data');self.end_headers();return
            status=404 if self.path=='/missing' else 200
            body=b'not found\n' if status==404 else b'rboxc GNU wget fixture\n'
            self.send_response(status);self.send_header('Content-Length',str(len(body)));self.end_headers()
            self.wfile.write(body)
    server=http.server.HTTPServer(('127.0.0.1',0),FixtureHandler)
    threading.Thread(target=server.serve_forever,daemon=True).start()
    url='http://127.0.0.1:'+str(server.server_port)
    for label,path,args in [('stdout','/data',['-O','-']),('file','/data',['-O','output']),('redirect','/redirect',['-O','output']),('not-found','/missing',['-O','output'])]:
        case('wget-http-'+label,'wget',['--no-config','--no-proxy','-q',*args,url+path])
# Network-service commands are limited here to option parsing: no listener,
# packet transmission, external endpoint, host log write, or configuration change.
results = []
for index, (name, command, args, data, files, multicall) in enumerate(cases):
    outcomes = {}
    for implementation in ('gnu', 'rboxc'):
        for instrument in (False, True):
            key = implementation+('-valgrind' if instrument else '')
            with tempfile.TemporaryDirectory(prefix='rboxc-entry-') as directory:
                work = Path(directory); (work/'exec').mkdir(); (work/'files').mkdir()
                for filename,contents in files.items():
                    path=work/'files'/filename;path.parent.mkdir(parents=True,exist_ok=True)
                    path.write_bytes(contents);path.chmod(0o640)
                    os.utime(path,(1700000000,1700000000))
                executable = oracles[command] if implementation == 'gnu' else profile.binary
                alias = work/'exec'/command; alias.symlink_to(executable)
                argv = [command if multicall and implementation == 'gnu' else str(alias), *args]
                if multicall and implementation == 'rboxc':
                    box = work/'exec/rboxc'; box.symlink_to(executable)
                    argv = [str(box), command, *args]
                log = work/'memory.log'
                if instrument:
                    argv = ['/usr/bin/valgrind', '--leak-check=full', '--show-leak-kinds=all',
                            '--track-fds=yes', '--log-file='+str(log), *argv]
                started = int(time.time())
                done = subprocess.run(argv, input=data, cwd=work/'files', capture_output=True,
                                      env={'LC_ALL':'C', 'LANGUAGE':'C', 'HOME':directory, 'TZ':'UTC0', 'PATH':str(work/'exec')+':/usr/bin:/bin'},
                                      umask=0o022, timeout=45)
                ended = int(time.time())
                tree = {str(p.relative_to(work/'files')): {'sha256':fingerprint(p), 'bytes':p.stat().st_size,
                         'mode':p.stat().st_mode & 0o7777} for p in sorted((work/'files').rglob('*')) if p.is_file()}
                archive_observations = []
                if command == 'ar' and (work/'files/output.a').exists():
                    original = (work/'files/output.a').read_bytes()
                    assert original.startswith(b'!<arch>\n')
                    canonical = bytearray(original)
                    offset = 8
                    while offset < len(original):
                        header = original[offset:offset+60]
                        assert len(header) == 60 and header[58:] == b'`\n'
                        size = int(header[48:58])
                        if header[:16].strip() in (b'/', b'/SYM64/'):
                            timestamp = int(header[16:28])
                            assert timestamp == 0 if 'D' in args[0] else started <= timestamp <= ended
                            canonical[offset+16:offset+28] = b'<generated> '
                            archive_observations.append({'member':header[:16].decode().strip(),
                                'timestamp':timestamp,'invocation_window':[started,ended]})
                        offset += 60 + size + size % 2
                    assert offset == len(original)
                    tree['output.a']['sha256'] = __import__('hashlib').sha256(canonical).hexdigest()
                    archive_observations.append({'raw_archive':original.hex(),'raw_sha256':__import__('hashlib').sha256(original).hexdigest()})
                row = {'status':done.returncode, 'raw_stdout':done.stdout.hex(), 'raw_stderr':done.stderr.hex(),
                       'fixture':directory, 'stdout':done.stdout.replace(directory.encode(), b'<fixture>').hex(),
                       'stderr':done.stderr.replace(directory.encode(), b'<fixture>').hex(), 'tree':tree,
                       'archive_observations':archive_observations}
                if instrument:
                    saved = profile.logs/f'{index:03}-{key}.log'; shutil.copy2(log, saved)
                    contents = saved.read_text()
                    pids = set(re.findall(r'^==([0-9]+)==', contents, re.M))
                    assert len(pids) == 1
                    row.update(memory=runner.parse_memory_log(contents, pids.pop(), exec_only=True),
                               log=str(saved.relative_to(ROOT)), log_sha256=fingerprint(saved))
                outcomes[key] = row
    reference = outcomes['gnu']
    equivalent = all(all(row[k] == reference[k] for k in ('status','stdout','stderr','tree')) for row in outcomes.values())
    memory = outcomes['rboxc-valgrind']['memory']
    clean = memory['complete_exec_log'] and memory['errors'] == 0 and memory['non_inherited_descriptors'] == 0 and not any(
        memory['heap_bytes'].get(k, 0) for k in ('definitely lost','indirectly lost','possibly lost'))
    results.append({'name':name, 'command':command, 'arguments':args, 'input':data.hex(), 'fixture_files':{n:__import__('hashlib').sha256(v).hexdigest() for n,v in files.items()},
                    'pass':equivalent and clean, 'equivalent':equivalent, 'memory_clean':clean, 'outcomes':outcomes})
    assert fingerprint(Path(__file__)) == driver_hash
    assert all(fingerprint(p) == hashes[n] for n, p in oracles.items())
    report = {'scope':'Selected GNU entry option parsing plus bounded valid Patch edits and compiler-produced Binutils inputs in private directories. Fixture paths and generated archive-index timestamps alone are normalized; original archive bytes are retained and index timestamps are checked against the invocation window or deterministic zero. Input file mtimes are fixed. Service and network commands have option coverage only; this is not full-provider certification.',
              **profile.metadata(), 'oracles':{n:{'path':str(p),'sha256':hashes[n]} for n,p in oracles.items()},
              'driver_sha256':driver_hash, 'planned_total':len(cases), 'complete':len(results)==len(cases),
              'passed':sum(r['pass'] for r in results), 'total':len(results), 'results':results}
    profile.report.write_text(json.dumps(report, indent=2)+'\n')
    print('PASS' if results[-1]['pass'] else 'OPEN', name, flush=True)
raise SystemExit(report['passed'] != report['total'])
