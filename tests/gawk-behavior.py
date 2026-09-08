#!/usr/bin/env python3
"""Compare bounded, valid Gawk programs and ordinary I/O errors."""
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
from comparison_profile import ComparisonProfile, fingerprint

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT/'tests/gnu'))
spec = importlib.util.spec_from_file_location('reviewed', ROOT/'tests/gnu/reviewed-original.py')
runner = importlib.util.module_from_spec(spec); spec.loader.exec_module(runner)
profile = ComparisonProfile('gawk-behavior', oracle=ROOT/'build/gnu-gawk/gawk')
oracles = {n: ROOT/'build/gnu-gawk/gawk' for n in ('awk','gawk','nawk')}
hashes = {n: fingerprint(p) for n, p in oracles.items()}
driver_hash = fingerprint(Path(__file__))
cases = []
def case(name, command, args, data=b'', *, fixture=None, multicall=False):
    cases.append((name, command, args, data, fixture, multicall))
for command in oracles:
    for option in ('help', 'version'):
        case(command+'-'+option, command, ['--'+option])
        case(command+'-'+option+'-multicall', command, ['--'+option], multicall=True)
    case(command+'-unknown-option', command, ['--not-an-option'])
programs = [
    ('begin','BEGIN { print "hello", 6*7 }',b''),
    ('fields','{ print NR, NF, $1, $NF }',b'alpha beta\nred green blue\n'),
    ('sum','{ total += $2 } END { print total }',b'a 17\nb 25\n'),
    ('filter','$2 > 3 { print $1 }',b'a 2\nb 4\nc 8\n'),
    ('regex','$0 ~ /a[[:alpha:]]+/ { print }',b'alpha\n123\nbeta\n'),
    ('arrays','BEGIN { a["x"]=3; a["y"]=4; print length(a), a["x"]+a["y"]; delete a["x"]; print ("x" in a) }',b''),
    ('function','function square(x) { return x*x } BEGIN { print square(12) }',b''),
    ('strings','BEGIN { x="abracadabra"; n=gsub(/abra/,"X",x); print n,x,substr(x,2,3),toupper(x) }',b''),
    ('split','BEGIN { n=split("red,green,blue",a,","); print n,a[2] }',b''),
    ('sort','BEGIN { a[1]="z"; a[2]="a"; a[3]="m"; n=asort(a); for(i=1;i<=n;i++)print a[i] }',b''),
    ('nested-array','BEGIN { a["x"]["y"]=42; print a["x"]["y"] }',b''),
    ('printf','BEGIN { printf "%04d %.3f %s\\n", 17, 1.25, "text" }',b''),
    ('loop','BEGIN { s=0; for(i=0;i<100;i++)s+=i; print s }',b''),
    ('csv','{print NF,$1,$2}',b'one,two\n"three,four",five\n'),
]
for command in oracles:
    case(command+'-no-program',command,[])
    case(command+'-missing-program',command,['-f','missing'])
    case(command+'-missing-input',command,['{print}','missing'])
    case(command+'-variable',command,['-v','value=42','BEGIN {print value}'])
    case(command+'-separator',command,['-F',':','{print $2}'],b'a:b:c\n')
    case(command+'-program-file',command,['-f','input'],fixture=b'BEGIN {print 42}\n')
    case(command+'-posix',command,['--posix','BEGIN {print (3+4)*6}'])
    case(command+'-traditional',command,['--traditional','BEGIN {print 42}'])
    for name,program,data in programs:
        args=(['--csv'] if name=='csv' else [])+[program]
        case(command+'-'+name,command,args,data)
case('gawk-multicall-program','gawk',['BEGIN {print 42}'],multicall=True)
case('gawk-output-file','gawk',['BEGIN {print "file output" > "output"; close("output")}'])
case('gawk-output-missing-parent','gawk',['BEGIN {print "x" > "missing/output"}'])
case('gawk-getline','gawk',['BEGIN {while((getline line < "input")>0)print toupper(line);close("input")}'],fixture=b'alpha\nbeta\n')
results = []
for index, (name, command, args, data, fixture, multicall) in enumerate(cases):
    outcomes = {}
    for implementation in ('gnu', 'rboxc'):
        for instrument in (False, True):
            key = implementation+('-valgrind' if instrument else '')
            with tempfile.TemporaryDirectory(prefix='rboxc-gawk-') as directory:
                work = Path(directory); (work/'exec').mkdir(); (work/'files').mkdir()
                if fixture is not None:
                    (work/'files/input').write_bytes(fixture)
                    (work/'files/input').chmod(0o640)
                executable = oracles[command] if implementation == 'gnu' else profile.binary
                alias = work/'exec'/command; alias.symlink_to(executable)
                argv = [str(alias), *args]
                if multicall and implementation == 'rboxc':
                    box = work/'exec/rboxc'; box.symlink_to(executable)
                    argv = [str(box), command, *args]
                log = work/'memory.log'
                if instrument:
                    argv = ['/usr/bin/valgrind', '--leak-check=full', '--show-leak-kinds=all',
                            '--track-fds=yes', '--log-file='+str(log), *argv]
                done = subprocess.run(argv, input=data, cwd=work/'files', capture_output=True,
                                      env={'LC_ALL':'C', 'LANGUAGE':'C', 'HOME':directory, 'TZ':'UTC0'},
                                      umask=0o022, timeout=45)
                tree = {str(p.relative_to(work/'files')): {'sha256':fingerprint(p), 'bytes':p.stat().st_size,
                         'mode':p.stat().st_mode & 0o7777} for p in sorted((work/'files').rglob('*')) if p.is_file()}
                row = {'status':done.returncode, 'raw_stdout':done.stdout.hex(), 'raw_stderr':done.stderr.hex(),
                       'fixture':directory, 'stdout':done.stdout.replace(directory.encode(), b'<fixture>').hex(),
                       'stderr':done.stderr.replace(directory.encode(), b'<fixture>').hex(), 'tree':tree}
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
    results.append({'name':name, 'command':command, 'arguments':args, 'input':data.hex(),
                    'pass':equivalent and clean, 'equivalent':equivalent, 'memory_clean':clean, 'outcomes':outcomes})
    assert fingerprint(Path(__file__)) == driver_hash
    assert all(fingerprint(p) == hashes[n] for n, p in oracles.items())
    report = {'scope':'Bounded valid Gawk programs covering all three aliases, arithmetic, fields, regular expressions, arrays, sorting, functions, CSV, named input/output, both dispatch forms, and ordinary option/I/O errors. Private fixture paths alone are normalized. Native Valgrind findings remain baseline observations.',
              **profile.metadata(), 'oracles':{n:{'path':str(p),'sha256':hashes[n]} for n,p in oracles.items()},
              'driver_sha256':driver_hash, 'planned_total':len(cases), 'complete':len(results)==len(cases),
              'passed':sum(r['pass'] for r in results), 'total':len(results), 'results':results}
    profile.report.write_text(json.dumps(report, indent=2)+'\n')
    print('PASS' if results[-1]['pass'] else 'OPEN', name, flush=True)
raise SystemExit(report['passed'] != report['total'])
