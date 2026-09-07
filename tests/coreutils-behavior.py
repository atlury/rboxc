#!/usr/bin/env python3
"""Bounded local GNU comparisons, including Valgrind and fixture effects."""
# SPDX-License-Identifier: GPL-3.0-or-later
import hashlib
import ctypes
import json
import os
from pathlib import Path
import re
import selectors
import stat
import struct
import subprocess
import sys
import tempfile
import time

ROOT = Path(__file__).resolve().parents[1]
GNU = ROOT/'build/gnu-coreutils/src'
from comparison_profile import ComparisonProfile
PROFILE = ComparisonProfile('behavior', selections=True)
BINARY = PROFILE.binary
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
    ('ln', ['-s', '--target-directory=empty', '../input']),
    ('ln', ['-s', '--target-directory=dir', '../file']),
    ('ln', ['--backup=invalid', '--target-directory=empty', 'input']),
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
    ('pr', ['-t', '-m', 'left', 'right']), ('pr', ['-t', '-n', 'input']),
    ('pr', ['-t', 'missing']),
    ('tac', ['-s', ' ', 'input']), ('tac', ['-r', '-s', '\\n', 'input']),
    ('tac', ['record']), ('tac', ['missing']), ('tac', []),
    ('sort', ['input']), ('sort', ['-u', 'input']),
    ('sort', ['-k2,2', 'input']), ('sort', ['-n', '-r', 'numbers']),
    ('tac', ['-', '-']),
    ('printf', ['%a %A %e %E %g %G\\n', '0.1', '-0', '1e-30', '1e30', '1.23456789', '9.87654321']),
    ('printf', ['%*.*f|%*.0f|%.*g\\n', '12', '5', '1.23456789', '-8', '2.5', '18', '1.00000000000000001']),
    ('printf', ['%.20Lf\\n', '1.00000000000000001']),
    ('printf', ['%.3f %.3f\\n', "'A", '0x1.8p+2']),
    ('printf', ['%f\\n', '12oops']),
    ('printf', ['%f\\n', 'oops']),
    ('printf', ['%g %g %g\\n', 'inf', '-inf', 'nan']),
    ('sort', ['-g', 'floating']),
    ('sort', ['-gr', 'floating']),
    ('sort', ['-gu', 'floating']),
    ('sort', ['-g', '--debug', 'floating']),
    ('sort', ['-s', '-k2,2g', 'float-keys']),
    *[('od', ['-An', '-v', '-tf'+kind, 'float-'+kind]) for kind in 'B H F D L'.split()],
    *[('od', ['-An', '-v', '--endian=big', '-tf'+kind, 'float-'+kind+'-be']) for kind in 'B H F D L'.split()],
    ('od', ['-An', '-v', '-tfFz', '-N', '9', 'float-F']),
    ('numfmt', ['--from=iec-i', '--to=si', '1Ki', '1.5Mi', '2Gi']),
    ('numfmt', ['--from=auto', '--to=iec-i', '1K', '1Ki', '1M', '1Mi']),
    *[('numfmt', ['--round='+style, '--to-unit=10', '--', '15', '-15', '14', '-14'])
      for style in ['up', 'down', 'from-zero', 'towards-zero', 'nearest']],
    ('numfmt', ['--to=si', '--format=[%10.2f]', '--suffix=B', '1500B', '2500000B']),
    ('numfmt', ['--to=iec', '--padding=-12', '1024', '1536']),
    ('numfmt', ['--from-unit=512', '--to-unit=1024', '3', '5']),
    ('numfmt', ['--delimiter=:', '--field=2', '--to=iec', 'one:1024:tail', 'two:1536:end']),
    ('numfmt', ['--invalid=warn', '--from=auto', '1K', 'bad', '2Ki']),
    ('numfmt', ['--debug', '--to=si', '12345678901234567890']),
    ('numfmt', ['--invalid=ignore']),
    ('seq', ['5']), ('seq', ['3', '-1', '-2']),
    ('seq', ['0', '0.000001', '0.000003']),
    ('seq', ['0.8', '0.1', '0.9']),
    ('seq', ['-w', '9', '0.5', '10']),
    ('seq', ['-f', '[%06.2f]', '-s', ', ', '1', '0.5', '2']),
    ('seq', ['-f', '%a', '0x1p0', '0x1p-1', '0x1p1']),
    ('seq', ['1e1', '2e0', '1.4e1']),
    ('seq', ['-s', '::', '1', '3']),
    ('seq', ['99999999999999999998', '100000000000000000002']),
    ('seq', ['3', '1']), ('seq', ['1', '0', '2']),
    ('seq', ['nan']), ('seq', ['bad']),
    ('seq', ['-f', '%g%g', '2']),
    ('arch', []), ('chgrp', ['+65534', 'input']), ('chown', ['+65534:+65534', 'input']),
    ('chroot', ['missing', '/missing']), ('coreutils', ['--coreutils-prog=true']),
    ('df', ['--output=fstype', 'input']), ('dircolors', ['--sh', 'color-config']),
    ('du', ['-bs', 'dir']), ('groups', []), ('hostid', []), ('hostname', []),
    ('id', ['-u']), ('kill', ['-l', 'TERM']), ('logname', []),
    ('mkfifo', ['-m', '600', 'output']), ('mknod', ['-m', '600', 'output', 'p']),
    ('mktemp', ['xx']), ('nice', ['-n', '1', str(GNU/'true')]),
    ('nohup', [str(GNU/'true')]), ('nproc', ['--all']),
    ('pinky', ['rboxc-nonexistent-fixture-user']), ('ptx', ['input']),
    ('pwd', ['-P']), ('shred', ['-n', '0', '-z', 'input']),
    ('shuf', ['-i', '7-7', '-n', '1']), ('sleep', ['0']),
    ('stdbuf', ['-oL', str(GNU/'printf'), '%s\\n', 'fixture']), ('stty', ['-a']),
    ('sync', ['-d', 'input']), ('timeout', ['5', str(GNU/'true')]),
    ('tty', []), ('uname', ['-srm']), ('uptime', ['--invalid-fixture-option']),
    ('users', ['empty-utmp']), ('who', ['empty-utmp']), ('whoami', []),
    ('yes', ['fixture']),
    ('hostname', ['--invalid-fixture-option']),
    ('df', ['--output=fstype', 'missing']),
    ('shuf', ['-e', 'single']), ('shuf', ['-e']),
    ('shuf', ['-n', '1']), ('shuf', ['-n', '10']),
    ('shuf', ['-r', '-n', '3', '-e', 'single']),
    ('shuf', ['-n', '0']), ('shuf', ['empty-utmp']),
    ('shuf', ['--random-source=zeros', 'input']),
    ('shuf', ['--random-source=zeros', '-n', '2']),
    ('shuf', ['--random-source=zeros', '-r', '-n', '8', 'input']),
    ('shuf', ['--random-source=zeros', '-o', 'output', '-e', 'one', 'two', 'three']),
    ('stdbuf', ['-oL', str(GNU/'printenv'), '_STDBUF_O']),
    ('stdbuf', ['-o0', 'rboxc-nonexistent-fixture-command']),
    ('stdbuf', ['-i0', '-oL', '-e0', 'rboxc-nonexistent-fixture-command']),
    ('shuf', ['--random-source=zeros', '-o', 'output', 'input']),
    ('nl', ['-p', '-v9223372036854775807', 'input']),
    ('nl', ['-p', '-v9223372036854775807']),
    ('base32', ['--decode', 'input']),
    ('base64', ['--decode', 'input']),
    ('basenc', ['--base16', '--decode', 'input']),
    ('tee', ['/dev/full', 'output']),
    ('tee', ['--output-error=exit', 'output', 'missing/file']),
    ('tee', ['--output-error=exit', 'output', '/dev/full']),
    ('chmod', ['-2000', 'input']),
    ('chmod', ['-w', '-x', 'input']),
    ('chmod', ['-w', '--reference=input', 'numbers']),
    ('stat', ['/dev/null']),
    ('stat', ['-t', '/dev/null']),
    ('stat', ['-f', '-']),
    ('stat', ['-f', '-t', '-']),
    ('stat', ['missing']),
    ('chown', ['--reference=input', 'numbers']),
    ('chown', ['--reference=input', 'missing']),
    ('chgrp', ['--reference=input', 'numbers']),
    ('mktemp', ['--suffix=/bad', 'aXXXX']),
    ('mktemp', ['aXXXX/b']),
    ('mktemp', ['-d', '--suffix=X', 'aXX']),
    ('mktemp', ['--tmpdir=.', '/aXXXX']),
    ('install', ['input', 'left', '-t', 'empty']),
    ('install', ['-D', 'input', 'numbers', '-t', 'new/nested']),
    ('install', ['-m', 'invalid', '-t', 'empty', 'input']),
    ('install', ['input', 'missing', '-t', 'empty']),
    ('install', ['--strip-program=first', '--strip-program=second', '--invalid-option']),
    ('install', ['-C', '-s', '--strip-program=first', '--strip-program=second', 'input', 'output']),
    ('wc', ['--files0-from=/dev/null']),
    ('wc', ['--files0-from=empty-utmp']),
    ('wc', ['--files0-from=missing']),
    ('wc', ['--files0-from=dir']),
    ('wc', ['--files0-from=-']),
    ('split', ['-n', 'r/3', 'input', 'part']),
    ('split', ['-e', '-n', 'r/10', 'empty-utmp', 'part']),
    ('split', ['-n', 'r/1/3', 'input']),
    ('split', ['-n', 'r/3', 'input', 'missing/part']),
    ('split', ['-b', '1', '-a', '1', 'input', 'part']),
    ('split', ['-C', '6', 'input', 'missing/part']),
    ('du', ['--files0-from=/dev/null']),
    ('du', ['--files0-from=empty-utmp']),
    ('du', ['--files0-from=missing']),
    ('du', ['--files0-from=dir']),
    ('du', ['--files0-from=-']),
    ('cp', ['input', 'left', '-t', 'empty']),
    ('cp', ['input', 'input', 'empty']),
    ('cp', ['missing', 'input', 'empty']),
    ('cp', ['--backup=invalid', '-t', 'empty', 'input']),
    ('mv', ['input', 'left', '-t', 'empty']),
    ('mv', ['missing', 'input', 'empty']),
    ('mv', ['--backup=invalid', '-t', 'empty', 'input']),
    ('mv', ['-bn', '-t', 'empty', 'input']),
    ('date', ['--debug', '-d', 'TZ="America/Edmonton" 2006-04-02 02:30:00']),
    ('date', ['-u', '-d', 'not-a-date', '+%-N']),
    ('date', ['-u', '-f', 'missing', '+%-N']),
    ('tail', ['-f', '-n', '1']),
    ('sort', ['-m', 'left', 'right']),
    ('sort', ['-m', '--batch-size=0', 'left', 'right']),
    ('sort', ['-c', 'left']),
    ('sort', ['--files0-from=/dev/null']),
    ('sort', ['--files0-from=empty-utmp']),
    ('sort', ['--files0-from=dir']),
    ('sort', ['--files0-from=missing']),
    ('sort', ['--files0-from=-']),
    ('sort', ['-m', '--batch-size=2', '-Tmissing', 'left', 'right', 'left']),
    ('tail', ['-f', '--pid=2147483647', 'input']),
    ('tail', ['-f', '---disable-inotify', '--pid=2147483647', 'input']),
    ('tail', ['-f', '--pid=2147483647', 'missing']),
    ('tail', ['-f', '--pid=2147483647', '-']),
    ('tail', ['-f', '--pid=1', '--pid=invalid', 'input']),
    ('cat', ['dir']),
    ('cat', ['dir', 'input']),
    ('csplit', ['dir', '1']),
    ('date', ['-f', 'dir']),
    ('join', ['dir', 'dir']),
    ('join', ['left', 'missing']),
    ('shuf', ['-n1', 'dir']),
    ('shuf', ['-r', 'dir']),
    ('sort', ['dir']),
    ('tail', ['-c+1', 'dir']),
    ('tail', ['-n+1', 'dir']),
    ('uniq', ['dir']),
    ('uniq', ['-c', 'dir']),
]


def fixture(root):
    (root/'dir').mkdir()
    (root/'empty').mkdir()
    for name, data in {'input': b'alpha one\nbeta\ttwo\n\ngamma three\ngamma three\n',
                       'numbers': b'10\n2\n-3\n1.5\n', 'left': b'a 1\nb 2\n',
                       'right': b'b 3\nc 4\n', 'edges': b'a b\nb c\n',
                       'dates': b'2000-02-29\n2001-03-01\n',
                       'record': b'a' * 40000 + b'\nend\n',
                       'floating': b'nan\n-inf\n-1e30\n-0\n0\n1e-30\n1.00000000000000001\n1.00000000000000002\ninf\nnan\n',
                       'float-keys': b'a 1e30\nb -0\nc 0\nd 1e-30\ne nan\n',
                       'color-config': b'TERM *\nDIR 01;34\nLINK 01;36\n',
                       'empty-utmp': b'',
                       'zeros': bytes(128),
                       'dir/file': b'fixture data\x00\xff\n'}.items():
        (root/name).write_bytes(data)
        (root/name).chmod(0o644)
    # Values exactly representable in every format; native long double bytes
    # include the platform padding and avoid a binary128 ABI assumption.
    values = (0.0, -0.0, 1.5, -2.25, 0.125)
    encodings = {kind: [struct.pack('<'+code, v) for v in values]
                 for kind, code in [('H', 'e'), ('F', 'f'), ('D', 'd')]}
    encodings['B'] = [struct.pack('<f', v)[2:] for v in values]
    assert ctypes.sizeof(ctypes.c_longdouble) == 16, 'pinned x86-64 profile'
    encodings['L'] = [bytes(ctypes.c_longdouble(v))[:10] + bytes(6) for v in values]
    for kind, words in encodings.items():
        (root/('float-'+kind)).write_bytes(b''.join(words))
        (root/('float-'+kind+'-be')).write_bytes(b''.join(word[::-1] for word in words))
    (root/'link').symlink_to('input')
    for path in root.rglob('*'):
        os.utime(path, ns=(946684800000000000,)*2, follow_symlinks=False)


def tree(root):
    result = {}
    links = {}
    for path in sorted(root.rglob('*')):
        info = path.lstat()
        name = str(path.relative_to(root))
        entry = {'mode': stat.S_IMODE(info.st_mode), 'kind': stat.S_IFMT(info.st_mode),
                 'uid': info.st_uid, 'gid': info.st_gid}
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
        log = PROFILE.logs/f'behavior-{index:03d}-{name}-{implementation}.log'
        # Resolve the GNU symlink through PATH so both entries receive the
        # same argv[0]; GNU's try-help message preserves that argument.
        command = [name] if implementation == 'gnu' else [BINARY, name]
        env = {**os.environ, 'LC_ALL': 'C', 'LANGUAGE': 'C', 'TZ': 'UTC0',
               'RBOXC_FIXTURE': 'local-test', 'TERM': 'dumb',
               'PATH': str(ROOT/'build/behavior-oracle')+os.pathsep+os.environ['PATH']}
        for key in ('POSIXLY_CORRECT', 'VERSION_CONTROL', 'SIMPLE_BACKUP_SUFFIX'):
            env.pop(key, None)
        prefix = ['valgrind', '--error-exitcode=97', '--leak-check=full',
                                    '--show-leak-kinds=all', '--track-fds=yes',
                                    '--trace-children=yes',
                                    '--errors-for-leak-kinds=definite,indirect,possible',
                                    '--log-file='+str(log)] if instrument else []
        if name == 'yes':
            # Read a bounded prefix, then close the consumer. Both programs
            # should terminate from SIGPIPE; never accumulate unbounded output.
            with subprocess.Popen([*prefix, *command, *args], stdin=subprocess.DEVNULL,
                                  stdout=subprocess.PIPE, stderr=subprocess.PIPE,
                                  cwd=root, env=env) as process:
                deadline = time.monotonic() + 45
                output = b''
                try:
                    with selectors.DefaultSelector() as selector:
                        selector.register(process.stdout, selectors.EVENT_READ)
                        while len(output) < 64:
                            if not selector.select(max(0, deadline - time.monotonic())):
                                raise subprocess.TimeoutExpired(process.args, 45)
                            chunk = os.read(process.stdout.fileno(), 64-len(output))
                            if not chunk:
                                break
                            output += chunk
                    process.stdout.close()
                    process.stdout = None
                    _, stderr = process.communicate(timeout=max(0, deadline-time.monotonic()))
                except BaseException:
                    process.kill()
                    process.wait()
                    raise
                completed = subprocess.CompletedProcess(process.args, process.returncode, output, stderr)
        else:
            completed = subprocess.run([*prefix, *command, *args],
                                       input=(b'a\nb\nc\nd\n' if '--random-source=zeros' in args else b'one\n'*5)
                                             if name == 'shuf' else b'alpha alpha\nbeta\n',
                                       capture_output=True, cwd=root,
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
    selected = set(PROFILE.options.commands)
    assert selected <= {name for name, _ in CASES}, 'unknown command selection'
    previous_path = PROFILE.report
    previous = json.loads(previous_path.read_text())['results'] if selected and previous_path.exists() else []
    if previous and PROFILE.options.report_name:
        saved = json.loads(previous_path.read_text())
        current = PROFILE.metadata()
        assert all(saved.get(key) == current[key] for key in
                   ('binary_sha256', 'gnu_binary_sha256', 'runtime_helpers')), 'comparison inputs changed since saved selections'
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
    report = {**PROFILE.metadata(), 'scope': 'bounded local fixtures; status, streams, contents, modes, link topology; Valgrind both implementations',
              'passed': sum(row['pass'] for row in results),
              'behavior_passed': sum(row['behavior_pass'] for row in results),
              'valgrind_passed': sum(row['valgrind_pass'] for row in results),
              'total': len(results), 'results': results}
    PROFILE.report.write_text(json.dumps(report, indent=2)+'\n')
    print(report['passed'], '/', report['total'])
    return any(not row['pass'] for row in results if not selected or row['name'] in selected)


if __name__ == '__main__':
    raise SystemExit(main())
