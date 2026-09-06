#!/usr/bin/env python3
"""Bounded local GNU comparisons, including Valgrind and fixture effects."""
# SPDX-License-Identifier: GPL-3.0-or-later
import hashlib
import json
import os
from pathlib import Path
import re
import stat
import subprocess
import sys
import tempfile

ROOT = Path(__file__).resolve().parents[1]
GNU = ROOT/'build/gnu-coreutils/src'
BINARY = ROOT/'target/release/rboxc'
CASES = [
    ('basename', ['dir/input.txt', '.txt']), ('dirname', ['dir/input.txt']),
    ('cat', ['-n', 'input']), ('comm', ['left', 'right']),
    ('cut', ['-d', ' ', '-f', '2', 'input']), ('echo', ['-e', 'one\\ttwo']),
    ('expand', ['-t', '4', 'input']), ('unexpand', ['-a', '-t', '4', 'input']),
    ('expr', ['17', '*', '23']), ('factor', ['1', '2', '360', '65537']),
    ('fmt', ['-w', '16', 'input']), ('fold', ['-s', '-w', '12', 'input']),
    ('head', ['-n', '2', 'input']), ('tail', ['-n', '2', 'input']),
    ('join', ['left', 'right']), ('nl', ['-ba', 'input']),
    ('paste', ['left', 'right']), ('pathchk', ['-p', 'dir/input']),
    ('pr', ['-t', '-2', 'input']), ('printf', ['%s %08x %.3f\\n', 'text', '42', '1.25']),
    ('seq', ['-w', '0', '0.25', '1']), ('numfmt', ['--to=iec', '1024', '1048576']),
    ('od', ['-An', '-tx1', 'input']), ('sort', ['-n', 'numbers']),
    ('sort', ['-g', 'numbers']), ('tac', ['input']),
    ('tee', ['output']), ('tr', ['-s', 'a-z', 'A-Z']),
    ('tsort', ['edges']), ('uniq', ['-c', 'input']), ('wc', ['-lwmc', 'input']),
    ('cksum', ['input']), ('sum', ['input']), ('base32', ['input']),
    ('base64', ['input']), ('basenc', ['--base16', 'input']),
    ('b2sum', ['input']), ('md5sum', ['input']), ('sha1sum', ['input']),
    ('sha224sum', ['input']), ('sha256sum', ['input']),
    ('sha384sum', ['input']), ('sha512sum', ['input']),
    ('date', ['-u', '-d', '@0', '+%Y-%m-%dT%H:%M:%S']),
    ('dd', ['if=input', 'of=output', 'bs=7', 'count=3', 'status=none']),
    ('env', ['-i', 'NAME=value', str(GNU/'printenv'), 'NAME']),
    ('printenv', ['RBOXC_FIXTURE']), ('false', []), ('true', []),
    ('test', ['-f', 'input']), ('[', ['-d', 'absent', ']']),
    ('chmod', ['u=rw,go=r', 'input']), ('mkdir', ['-p', 'new/nested']),
    ('rmdir', ['empty']), ('link', ['input', 'output']),
    ('ln', ['-s', 'input', 'output']), ('unlink', ['input']),
    ('rm', ['-r', 'dir']), ('mv', ['input', 'output']),
    ('install', ['-m', '640', 'input', 'output']),
    ('truncate', ['-s', '13', 'input']), ('touch', ['-d', '@946684800', 'output']),
    ('split', ['-l', '2', 'input', 'part']), ('csplit', ['-s', 'input', '2']),
    ('readlink', ['link']), ('realpath', ['link']),
    ('ls', ['-1', 'dir']), ('dir', ['-1', 'dir']),
    ('vdir', ['--time-style=+%s', 'dir']), ('stat', ['-c', '%n %s %a %Y', 'input']),
    ('cp', ['input', 'output']), ('cp', ['-av', 'dir', 'output']),
    ('cp', ['--parents', '-v', 'dir/file', 'empty']),
    ('cp', ['--backup=numbered', '-v', 'input', 'numbers']),
    ('cp', ['--reflink=never', '--sparse=always', 'input', 'output']),
    ('cp', ['-P', 'link', 'output']), ('cp', ['missing', 'output']),
    ('cp', ['--attributes-only', 'input', 'numbers']),
    ('cp', ['-l', 'input', 'output']), ('cp', ['-s', 'input', 'output']),
    ('expr', ['0']), ('expr', ['length', 'alpha']),
    ('expr', ['substr', 'alphabet', '2', '3']),
    ('expr', ['12345678901234567890', '*', '98765432109876543210']),
    ('expr', ['alpha', ':', 'a.*']), ('expr', ['alpha', ':', 'z.*']),
    ('date', ['-u', '-d', '2000-02-29 12:34:56', '+%s %F %T']),
    ('date', ['-u', '-r', 'input', '+%s']),
    ('date', ['-u', '-f', 'dates', '+%F']),
    ('tail', ['-c', '7', 'input']), ('tail', ['-n', '+2', 'input']),
    ('tail', ['-n', '0', 'input']), ('tail', ['-n', '2', 'input', 'left']),
    ('tail', ['missing']), ('tail', ['-n', '2']),
    ('tr', ['-d', 'a-z']), ('tr', ['-s', 'a']),
    ('tr', ['[:lower:]', '[:upper:]']), ('tr', ['-ds', 'a', 'b']),
    ('tr', ['-c', 'a-z', '?']), ('tr', ['-t', 'a-z', 'AB']),
    ('tr', ['z-a', 'x']), ('tr', ['a', 'z-a']),
    ('cat', ['input', 'left']), ('cat', ['-A', 'dir/file']),
    ('cat', ['-b', '-s', 'input']), ('cat', ['missing']),
    ('dd', ['if=input', 'of=output', 'bs=1', 'conv=ucase', 'status=none']),
    ('dd', ['if=input', 'of=output', 'bs=4096', 'status=none']),
    ('split', ['-b', '7', 'input', 'part']),
    ('split', ['-C', '16', 'input', 'part']),
    ('split', ['-d', '-l', '2', 'input', 'part']),
]


def fixture(root):
    (root/'dir').mkdir()
    (root/'empty').mkdir()
    for name, data in {'input': b'alpha one\nbeta\ttwo\n\ngamma three\ngamma three\n',
                       'numbers': b'10\n2\n-3\n1.5\n', 'left': b'a 1\nb 2\n',
                       'right': b'b 3\nc 4\n', 'edges': b'a b\nb c\n',
                       'dates': b'2000-02-29\n2001-03-01\n',
                       'dir/file': b'fixture data\x00\xff\n'}.items():
        (root/name).write_bytes(data)
        (root/name).chmod(0o644)
    (root/'link').symlink_to('input')
    for path in root.rglob('*'):
        os.utime(path, ns=(946684800000000000,)*2, follow_symlinks=False)


def tree(root):
    result = {}
    links = {}
    for path in sorted(root.rglob('*')):
        info = path.lstat()
        name = str(path.relative_to(root))
        entry = {'mode': stat.S_IMODE(info.st_mode), 'kind': stat.S_IFMT(info.st_mode)}
        if path.is_symlink():
            entry['target'] = os.readlink(path)
        elif path.is_file():
            entry['sha256'] = hashlib.sha256(path.read_bytes()).hexdigest()
            identity = (info.st_dev, info.st_ino)
            entry['link_group'] = links.setdefault(identity, name)
        result[name] = entry
    return result


def run(index, name, args, implementation, instrument=True):
    with tempfile.TemporaryDirectory(prefix='rboxc-behavior-') as temporary:
        root = Path(temporary)
        fixture(root)
        log = ROOT/'evidence/raw'/f'behavior-{index:03d}-{name}-{implementation}.log'
        command = [ROOT/'build/behavior-oracle'/name] if implementation == 'gnu' else [BINARY, name]
        env = {**os.environ, 'LC_ALL': 'C', 'LANGUAGE': 'C', 'TZ': 'UTC0',
               'RBOXC_FIXTURE': 'local-test', 'TERM': 'dumb'}
        for key in ('POSIXLY_CORRECT', 'VERSION_CONTROL', 'SIMPLE_BACKUP_SUFFIX'):
            env.pop(key, None)
        prefix = ['valgrind', '--error-exitcode=97', '--leak-check=full',
                                    '--show-leak-kinds=all', '--track-fds=yes',
                                    '--trace-children=yes',
                                    '--errors-for-leak-kinds=definite,indirect,possible',
                                    '--log-file='+str(log)] if instrument else []
        completed = subprocess.run([*prefix, *command, *args],
                                   input=b'alpha alpha\nbeta\n', capture_output=True, cwd=root,
                                   env=env, timeout=45)
        report = log.read_text() if instrument and log.exists() else ''
        errors = re.search(r'ERROR SUMMARY: ([\d,]+) errors', report)
        lost = {kind: int(match[1].replace(',', ''))
                for kind in ('definitely lost', 'indirectly lost', 'possibly lost', 'still reachable')
                if (match := re.search(re.escape(kind)+r': ([\d,]+) bytes', report))}
        if 'All heap blocks were freed' in report:
            lost = dict.fromkeys(('definitely lost', 'indirectly lost', 'possibly lost', 'still reachable'), 0)
        normalized = lambda value: value.replace(os.fsencode(root), b'<FIXTURE>').decode(errors='backslashreplace')
        descriptors = re.search(r'FILE DESCRIPTORS: (\d+) open \((\d+) (?:inherited|std)\)', report)
        return {'status': completed.returncode, 'stdout': normalized(completed.stdout),
                'stderr': normalized(completed.stderr), 'tree': tree(root),
                'errors': int(errors[1].replace(',', '')) if errors else None,
                'heap_bytes': lost,
                'non_inherited_descriptors': int(descriptors[1])-int(descriptors[2]) if descriptors else None,
                'log': str(log.relative_to(ROOT)) if instrument else None}


def main():
    selected = set(sys.argv[1:])
    assert selected <= {name for name, _ in CASES}, 'unknown command selection'
    previous_path = ROOT/'evidence/behavior.json'
    previous = json.loads(previous_path.read_text())['results'] if selected and previous_path.exists() else []
    results_by_case = {(row['name'], tuple(row['arguments'])): row for row in previous}
    oracle_links = ROOT/'build/behavior-oracle'
    oracle_links.mkdir(exist_ok=True)
    for name, _ in CASES:
        link = oracle_links/name
        if not link.exists():
            link.symlink_to(GNU/'coreutils')
    for index, (name, arguments) in enumerate(CASES):
        if selected and name not in selected:
            continue
        expected = run(index, name, arguments, 'gnu', instrument=False)
        actual = run(index, name, arguments, 'rboxc', instrument=False)
        expected_memory = run(index, name, arguments, 'gnu')
        actual_memory = run(index, name, arguments, 'rboxc')
        differences = [key for key in ('status', 'stdout', 'stderr', 'tree') if actual[key] != expected[key]]
        memory_pass = actual_memory['errors'] == 0 and actual_memory['non_inherited_descriptors'] == 0
        ok = not differences and memory_pass
        results_by_case[name, tuple(arguments)] = {'name': name, 'arguments': arguments, 'pass': ok,
                        'behavior_pass': not differences, 'valgrind_pass': memory_pass,
                        'differences': differences, 'gnu': expected, 'rboxc': actual,
                        'gnu_valgrind': expected_memory, 'rboxc_valgrind': actual_memory}
        print('PASS' if ok else 'OPEN', name, arguments, differences, actual_memory['errors'], flush=True)
    results = [results_by_case[name, tuple(args)] for name, args in CASES
               if (name, tuple(args)) in results_by_case]
    report = {'scope': 'bounded local fixtures; status, streams, contents, modes, link topology; Valgrind both implementations',
              'passed': sum(row['pass'] for row in results),
              'behavior_passed': sum(row['behavior_pass'] for row in results),
              'valgrind_passed': sum(row['valgrind_pass'] for row in results),
              'total': len(results), 'results': results}
    (ROOT/'evidence/behavior.json').write_text(json.dumps(report, indent=2)+'\n')
    print(report['passed'], '/', report['total'])
    return any(not row['pass'] for row in results if not selected or row['name'] in selected)


if __name__ == '__main__':
    raise SystemExit(main())
