#!/usr/bin/env python3
"""Compare bounded, valid uuencode/uudecode operations and ordinary I/O errors."""
# SPDX-License-Identifier: GPL-3.0-or-later
import importlib.util
import json
from pathlib import Path
import re
import shutil
import subprocess
import sys
import tempfile
from comparison_profile import ComparisonProfile, fingerprint

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT/'tests/gnu'))
spec = importlib.util.spec_from_file_location('reviewed', ROOT/'tests/gnu/reviewed-original.py')
runner = importlib.util.module_from_spec(spec); spec.loader.exec_module(runner)
profile = ComparisonProfile('sharutils-behavior', oracle=ROOT/'build/gnu-sharutils/src/uuencode')
oracles = {n: ROOT/'build/gnu-sharutils/src'/n for n in ('uuencode', 'uudecode')}
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
case('encode-no-operands', 'uuencode', [])
case('encode-missing-input', 'uuencode', ['missing', 'result'])
case('decode-missing-input', 'uudecode', ['missing'])
case('decode-empty-input', 'uudecode', [])
for size in (0, 1, 2, 3, 44, 45, 46, 57, 58, 4096):
    data = bytes(i % 256 for i in range(size))
    for base64 in (False, True):
        mode = 'base64' if base64 else 'traditional'
        args = (['-m'] if base64 else [])+['result']
        case(f'encode-{mode}-{size}', 'uuencode', args, data)
        # The native encoder supplies a valid fixture; no malformed payloads.
        encoded = subprocess.check_output([oracles['uuencode'], *args], input=data,
                                         env={'LC_ALL':'C', 'LANGUAGE':'C'}, umask=0o022)
        case(f'decode-{mode}-{size}', 'uudecode', [], encoded)
        if size == 58:
            case(f'encode-file-{mode}', 'uuencode', (['-m'] if base64 else [])+['input', 'result'], fixture=data)
            case(f'decode-file-{mode}', 'uudecode', ['input'], fixture=encoded)
            case(f'decode-stdout-{mode}', 'uudecode', ['-o', '-'], encoded)
            case(f'decode-override-{mode}', 'uudecode', ['-o', 'override'], encoded)
            case(f'decode-output-error-{mode}', 'uudecode', ['-o', '/dev/full'], encoded)
            case(f'decode-missing-parent-{mode}', 'uudecode', ['-o', 'absent/output'], encoded)
            case(f'encode-multicall-{mode}', 'uuencode', args, data, multicall=True)
            case(f'decode-multicall-{mode}', 'uudecode', [], encoded, multicall=True)
case('encode-filename', 'uuencode', ['-em', 'hello'], b'hello\n')
encoded = subprocess.check_output([oracles['uuencode'], '-em', 'hello'], input=b'hello\n', umask=0o022)
case('decode-filename', 'uudecode', [], encoded)
results = []
for index, (name, command, args, data, fixture, multicall) in enumerate(cases):
    outcomes = {}
    for implementation in ('gnu', 'rboxc'):
        for instrument in (False, True):
            key = implementation+('-valgrind' if instrument else '')
            with tempfile.TemporaryDirectory(prefix='rboxc-sharutils-') as directory:
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
    report = {'scope':'Valid bounded traditional/base64 data, line boundaries, named input/output, encoded names, both dispatch forms, and ordinary option/I/O errors. Private fixture paths alone are normalized. Native Valgrind findings remain baseline observations.',
              **profile.metadata(), 'oracles':{n:{'path':str(p),'sha256':hashes[n]} for n,p in oracles.items()},
              'driver_sha256':driver_hash, 'planned_total':len(cases), 'complete':len(results)==len(cases),
              'passed':sum(r['pass'] for r in results), 'total':len(results), 'results':results}
    profile.report.write_text(json.dumps(report, indent=2)+'\n')
    print('PASS' if results[-1]['pass'] else 'OPEN', name, flush=True)
raise SystemExit(report['passed'] != report['total'])
