#!/usr/bin/env python3
"""Compare ordinary Diffutils formatting, merge, and ownership paths."""
# SPDX-License-Identifier: GPL-3.0-or-later
import importlib.util
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
from comparison_profile import ComparisonProfile, fingerprint

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT/'tests/gnu'))
spec = importlib.util.spec_from_file_location('reviewed_original', ROOT/'tests/gnu/reviewed-original.py')
runner = importlib.util.module_from_spec(spec)
spec.loader.exec_module(runner)
profile = ComparisonProfile('diffutils-behavior', oracle=ROOT/'build/gnu-diffutils/src/diff', selections=True)
commands = ('cmp', 'diff', 'diff3', 'sdiff')
oracles = {name: ROOT/f'build/gnu-diffutils/src/{name}' for name in commands}
hashes = {name: fingerprint(path) for name, path in oracles.items()}
cases = []
def case(name, command, args, data=b'', output='pipe'):
    cases.append((name, command, args, data, output))
for command in commands:
    case(command+'-help', command, ['--help'])
    case(command+'-version', command, ['--version'])
    case(command+'-unknown-option', command, ['--unknown-option'])
    case(command+'-missing-operand', command, [])
for name, args in [
    ('equal', ['a', 'a']), ('different', ['a', 'b']), ('silent-size', ['-s', 'a', 'empty']),
    ('list', ['-bl', 'a', 'b']), ('limit', ['-n', '4', 'a', 'b']),
    ('skip', ['-i', '5:5', 'a', 'b']), ('stdin', ['-', 'a']),
    ('missing', ['a', 'absent']), ('large', ['large-a', 'large-b']),
]:
    case('cmp-'+name, 'cmp', args, b'same\nleft\nlast\n' if name == 'stdin' else b'')
for name, args in [
    ('normal', ['a', 'b']), ('unified', ['-u', 'a', 'b']), ('context', ['-c', 'a', 'b']),
    ('ed', ['-e', 'a', 'b']), ('ifdef', ['-D', 'CHANGE', 'a', 'b']),
    ('color', ['--color=always', '-u', 'a', 'b']), ('no-color', ['--color=never', 'a', 'b']),
    ('several-regex', ['-I', '^left', '-I', '^right', 'a', 'b']),
    ('several-functions', ['-F', '^same', '-F', '^last', '-u', 'a', 'b']),
    ('exclude', ['-r', '-x', 'skip', 'left', 'right']),
    ('ignore-case-directory', ['--ignore-file-name-case', 'left/Name', 'right']),
    ('no-follow-directory', ['--no-dereference', 'loop', 'right']),
    ('full-output', ['-u', 'a', 'b']),
]:
    case('diff-'+name, 'diff', args, output='full' if name == 'full-output' else 'pipe')
for name, args in [
    ('equal', ['a', 'a', 'a']), ('normal', ['a', 'b', 'c']),
    ('merge', ['-m', 'a', 'b', 'c']), ('merge-labels', ['-m', '-L', 'ours', '-L', 'base', '-L', 'theirs', 'a', 'b', 'c']),
    ('ed', ['-e', 'a', 'b', 'c']), ('show-overlap', ['-E', 'a', 'b', 'c']),
    ('overlap-only', ['-x', 'a', 'b', 'c']), ('easy-only', ['-3', 'a', 'b', 'c']),
    ('growth', ['-m', 'large-a', 'large-b', 'large-c']),
    ('missing-file', ['absent', 'b', 'c']),
    ('missing-program', ['--diff-program=absent-diff', 'a', 'b', 'c']),
    ('non-executable-program', ['--diff-program=./not-executable', 'a', 'b', 'c']),
    ('full-output', ['-m', 'a', 'b', 'c']),
]:
    case('diff3-'+name, 'diff3', args, output='full' if name == 'full-output' else 'pipe')
case('sdiff-display', 'sdiff', ['-w', '60', 'a', 'b'])
case('sdiff-suppress-common', 'sdiff', ['-s', 'a', 'b'])
case('sdiff-merge-left', 'sdiff', ['-o', 'merged', 'a', 'b'], b'l\n')
case('sdiff-merge-right', 'sdiff', ['-o', 'merged', 'a', 'b'], b'r\n')
case('sdiff-merge-quit', 'sdiff', ['-o', 'merged', 'a', 'b'], b'q\n')
selected = set(profile.options.commands)
assert selected <= {row[0] for row in cases}
results = []
for index, (name, command, args, data, output) in enumerate(cases):
    if selected and name not in selected:
        continue
    outcomes = {}
    for implementation in ('gnu', 'rboxc'):
        for instrument in (False, True):
            key = implementation + ('-valgrind' if instrument else '')
            with tempfile.TemporaryDirectory(prefix='rboxc-diffutils-behavior-') as directory:
                work = Path(directory)
                (work/'exec').mkdir(); (work/'memory').mkdir()
                for entry in commands:
                    (work/'exec'/entry).symlink_to(oracles[entry] if implementation == 'gnu' else profile.binary)
                for label, middle in [('a', 'left'), ('b', 'right'), ('c', 'third')]:
                    (work/label).write_text('same\n'+middle+'\nlast\n')
                    (work/('large-'+label)).write_text(''.join(f'{i:05d} {middle}\n' for i in range(2000)))
                (work/'empty').write_bytes(b'')
                (work/'not-executable').write_text('ordinary non-executable fixture\n')
                for side in ('left', 'right'):
                    (work/side).mkdir()
                    (work/side/'skip').write_text(side+'\n')
                (work/'left/Name').write_text('same\n')
                (work/'right/name').write_text('same\n')
                (work/'loop').symlink_to('loop')
                (work/'right/loop').symlink_to('loop')
                for item in work.rglob('*'):
                    if item.is_file() and not item.is_symlink():
                        os.utime(item, (946684800, 946684800))
                env = {'PATH': str(work/'exec')+':/usr/bin:/bin', 'LC_ALL': 'C', 'LANGUAGE': 'C',
                       'TZ': 'UTC0', 'HOME': directory, 'TMPDIR': directory}
                invocation = [command, *args]
                if instrument:
                    invocation = ['/usr/bin/valgrind', '--leak-check=full', '--show-leak-kinds=all',
                        '--track-fds=yes', '--trace-children=yes', '--log-file='+str(work/'memory/%p.log'), *invocation]
                with open('/dev/full', 'wb') if output == 'full' else open(os.devnull, 'wb') as sink:
                    done = subprocess.run(invocation, cwd=work, env=env, input=data,
                        stdout=sink if output == 'full' else subprocess.PIPE, stderr=subprocess.PIPE, timeout=40)
                stdout = done.stdout or b''
                row = {'status': done.returncode, 'stdout': stdout.replace(directory.encode(), b'<fixture>').hex(),
                    'stderr': done.stderr.replace(directory.encode(), b'<fixture>').hex(),
                    'raw_stdout': stdout.hex(), 'raw_stderr': done.stderr.hex(), 'fixture': directory,
                    'merged': (work/'merged').read_bytes().hex() if (work/'merged').exists() else None}
                if instrument:
                    saved = profile.logs/f'{index:02}-{key}-memory'
                    shutil.copytree(work/'memory', saved)
                    row['memory'] = [{**runner.parse_memory_log(p.read_text(), p.stem),
                        'log': str(p.relative_to(ROOT)), 'sha256': fingerprint(p)} for p in sorted(saved.glob('*.log'))]
                outcomes[key] = row
    def signature(row):
        return tuple(row[field] for field in ('status', 'stdout', 'stderr', 'merged'))
    equivalent = (signature(outcomes['gnu']) == signature(outcomes['rboxc'])
                  and signature(outcomes['gnu-valgrind']) == signature(outcomes['rboxc-valgrind']))
    instrumentation_consistent = all(signature(row) == signature(outcomes['gnu']) for row in outcomes.values())
    logs = outcomes['rboxc-valgrind']['memory']
    clean = bool(logs) and all(m['errors'] == 0 and m['non_inherited_descriptors'] == 0
        and not any(m['heap_bytes'].get(k, 0) for k in ('definitely lost', 'indirectly lost', 'possibly lost')) for m in logs)
    results.append({'name': name, 'command': command, 'arguments': args, 'stdin': data.hex(),
                    'stdout_target': output, 'pass': equivalent and clean, 'equivalent': equivalent,
                    'memory_clean': clean, 'instrumentation_consistent': instrumentation_consistent, 'outcomes': outcomes})
    print('PASS' if equivalent and clean else 'OPEN', name, flush=True)
    report = {'scope': 'Ordinary bounded formatting, comparisons, merge choices, allocation growth, and I/O failures; no historical vulnerability reproduction.',
        **profile.metadata(), 'gnu_binaries': {n: {'path': str(p), 'sha256': hashes[n]} for n, p in oracles.items()},
        'driver_sha256': fingerprint(Path(__file__)), 'comparison': 'GNU and rboxc must match separately in native and Valgrind modes. Cross-mode equality is recorded independently because tracing child exec changes argv[0].', 'normalization': 'Only replace the temporary fixture directory.',
        'passed': sum(r['pass'] for r in results), 'total': len(results), 'results': results}
    assert all(fingerprint(p) == hashes[n] for n, p in oracles.items())
    temp = profile.report.with_suffix('.tmp.json'); temp.write_text(json.dumps(report, indent=2)+'\n'); temp.replace(profile.report)
raise SystemExit(any(not r['pass'] for r in results))
