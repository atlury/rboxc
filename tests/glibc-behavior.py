#!/usr/bin/env python3
"""Compare translated glibc utilities with the pinned native command oracles."""
# SPDX-License-Identifier: GPL-3.0-or-later
import importlib.util
import json
import os
from pathlib import Path
import re
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
sys.path.insert(0,str(ROOT/'scripts'))
from entry_provider_helpers import binary_path
profile = ComparisonProfile('glibc-behavior')
oracles = {n:binary_path(ROOT,n) for n in ('getconf','iconv')}
hashes = {n:fingerprint(p) for n,p in oracles.items()}
driver_hash = fingerprint(Path(__file__))
cases=[]
def case(name,command,args,data=b'',*,files=None,multicall=False):
    cases.append((name,command,args,data,files or {},multicall))
for command in oracles:
    for option in ('help','version'):
        case(command+'-'+option,command,['--'+option])
        case(command+'-'+option+'-multicall',command,['--'+option],multicall=True)
    case(command+'-unknown-option',command,['--not-an-option'])
    case(command+'-unknown-option-multicall',command,['--not-an-option'],multicall=True)
for variable in ('ARG_MAX','CHAR_BIT','INT_MAX','LONG_BIT','OPEN_MAX','PATH','PAGESIZE','_POSIX_VERSION','GNU_LIBC_VERSION','GNU_LIBPTHREAD_VERSION','POSIX_V7_LP64_OFF64_CFLAGS'):
    case('getconf-'+variable,'getconf',[variable])
for variable in ('NAME_MAX','PATH_MAX','PIPE_BUF','LINK_MAX'):
    case('getconf-path-'+variable,'getconf',[variable,'.'])
case('getconf-no-argument','getconf',[])
case('getconf-unknown-name','getconf',['RBOXC_UNKNOWN_VARIABLE'])
case('getconf-path-missing','getconf',['NAME_MAX','missing'])
case('getconf-multicall-value','getconf',['LONG_BIT'],multicall=True)
for label,source,target,data in [
    ('ascii','ASCII','UTF-8',b'alpha beta\n'),
    ('latin1','ISO-8859-1','UTF-8',b'caf\xe9\n'),
    ('utf16le','UTF-8','UTF-16LE','Hello \u03a9 \U0001f642\n'.encode()),
    ('utf32be','UTF-8','UTF-32BE','Hello \u03a9\n'.encode()),
    ('decode-utf16','UTF-16LE','UTF-8','Hello \u03a9\n'.encode('utf-16le')),
    ('empty','UTF-8','UTF-16LE',b'')]:
    args=['-f',source,'-t',target]
    case('iconv-'+label+'-stdin','iconv',args,data)
    case('iconv-'+label+'-file','iconv',args+['input'],files={'input':data})
case('iconv-multicall-convert','iconv',['-f','ISO-8859-1','-t','UTF-8'],b'caf\xe9\n',multicall=True)
case('iconv-output-file','iconv',['-f','UTF-8','-t','UTF-16LE','-o','output'],b'alpha\n')
case('iconv-two-files','iconv',['-f','UTF-8','-t','UTF-16LE','one','two'],files={'one':b'alpha\n','two':b'beta\n'})
case('iconv-list','iconv',['--list'])
case('iconv-missing-input','iconv',['-f','UTF-8','-t','UTF-8','missing'])
case('iconv-unknown-encoding','iconv',['-f','RBOXC_UNKNOWN_ENCODING','-t','UTF-8'],b'alpha\n')
case('iconv-missing-output-parent','iconv',['-f','UTF-8','-t','UTF-16LE','-o','missing/output'],b'alpha\n')
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
    report = {'scope':'GNU glibc 2.43 getconf and iconv ordinary command fixtures using the existing host 2.43 libc and conversion modules. Exact outputs, status and files across GNU/Rust native and Valgrind runs; only private fixture paths normalized. Not full glibc library or command certification.',
              **profile.metadata(), 'oracles':{n:{'path':str(p),'sha256':hashes[n]} for n,p in oracles.items()},
              'driver_sha256':driver_hash, 'planned_total':len(cases), 'complete':len(results)==len(cases),
              'passed':sum(r['pass'] for r in results), 'total':len(results), 'results':results}
    profile.report.write_text(json.dumps(report, indent=2)+'\n')
    print('PASS' if results[-1]['pass'] else 'OPEN', name, flush=True)
raise SystemExit(report['passed'] != report['total'])
