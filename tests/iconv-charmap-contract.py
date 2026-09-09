#!/usr/bin/env python3
"""Compare charmap conversions and ordinary file/encoding diagnostics with GNU."""
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
profile = ComparisonProfile('iconv-charmap-contract',oracle=ROOT/'build/gnu-glibc/iconv/iconv_prog')
oracles = {n:binary_path(ROOT,n) for n in ('iconv',)}
hashes = {n:fingerprint(p) for n,p in oracles.items()}
driver_hash = fingerprint(Path(__file__))
cases=[]
def case(name,command,args,data=b'',*,files=None,multicall=False):
    cases.append((name,command,args,data,files or {},multicall))
maps=Path('/opt/src/glibc-2.43/localedata/charmaps')
latin=str(maps/'ISO-8859-1');utf8=str(maps/'UTF-8')
fixtures={str(p):fingerprint(p) for p in (maps/'ISO-8859-1',maps/'UTF-8')}
for name,frm,to,data,extra in [
    ('from-map',latin,'UTF-8',b'caf\xe9\n',[]),
    ('to-map','UTF-8',latin,'café\n'.encode(),[]),
    ('both-maps',latin,utf8,b'caf\xe9\n',[]),
    ('reverse-maps',utf8,latin,'café\n'.encode(),[]),
    ('output-file',latin,utf8,b'caf\xe9\n',['-o','output']),
    ('empty',latin,'UTF-8',b'',[]),
    ('unknown-target',latin,'RBOXC_UNKNOWN_ENCODING',b'cafe\n',[]),
    ('missing-input',latin,utf8,b'',['missing'])]:
    case(name,'iconv',['-f',frm,'-t',to,*extra],data)
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
    assert all(fingerprint(Path(p))==h for p,h in fixtures.items())
    assert fingerprint(Path(__file__)) == driver_hash
    assert all(fingerprint(p) == hashes[n] for n, p in oracles.items())
    report = {'scope':'Eight ordinary charmap conversions cover from-only, to-only, both/reverse maps, output files, empty input, unavailable target encoding and missing input. Canonical GNU 2.43 charmaps are pinned. Exact outputs, status and files compare GNU and Rboxc normally and under Valgrind; only private fixture paths are normalized.',
              **profile.metadata(), 'oracles':{n:{'path':str(p),'sha256':hashes[n]} for n,p in oracles.items()},
              'driver_sha256':driver_hash,'fixtures':fixtures, 'planned_total':len(cases), 'complete':len(results)==len(cases),
              'passed':sum(r['pass'] for r in results), 'total':len(results), 'results':results}
    profile.report.write_text(json.dumps(report, indent=2)+'\n')
    print('PASS' if results[-1]['pass'] else 'OPEN', name, flush=True)
raise SystemExit(report['passed'] != report['total'])
