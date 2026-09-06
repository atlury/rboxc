#!/usr/bin/env python3
"""Compare GNU backup/removal/rollback operations and finite syscall errors."""
# Copyright (C) 2026 Rbox contributors.
# SPDX-License-Identifier: GPL-3.0-or-later
import os
from pathlib import Path
import posixpath
import re
import shutil
import stat
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[2]
ORACLE = os.environ.get('GNU_CP', '/opt/gnu/coreutils-9.11/bin/cp')
RBOX = Path(os.environ.get('RBOX_PATH', ROOT/'target/release/rboxc')).resolve()
CALLS = ['rename', 'renameat', 'renameat2', 'unlink', 'unlinkat', 'stat', 'lstat', 'newfstatat', 'statx']


def run(binary, profile, target_directory, ordinary, injections=()):
    with tempfile.TemporaryDirectory(prefix='rbox-cp-mutations-') as work:
        root = Path(work)
        root.chmod(0o777)
        (root/'src').mkdir()
        (root/'src/file').write_bytes(b'new source\n')
        (root/'out').mkdir(mode=0o777)
        (root/'out').chmod(0o777)
        if target_directory == 'alias':
            (root/'alias').symlink_to('out')
        (root/'out/file').write_bytes(b'old destination\n')
        (root/'out/file').chmod(0o666)
        options = ['-v', '--reflink=never']
        if 'numbered' in profile or profile.startswith('existing'):
            versions = [9, 99] if profile.endswith('carry') else [1, 3]
            for version in versions:
                (root/f'out/file.~{version}~').write_bytes(f'saved {version}\n'.encode())
            options += ['--backup=existing' if profile.startswith('existing') else '--backup=numbered']
        elif 'simple' in profile:
            options += ['--backup=simple']
        elif profile == 'remove':
            options += ['--remove-destination']
        elif profile == 'force':
            options += ['-f']
            (root/'out/file').chmod(0o444)
        elif profile == 'physical':
            (root/'src/file').unlink()
            (root/'src/file').symlink_to('target')
            options += ['-P']
        else:
            raise AssertionError(profile)
        if profile.startswith('rollback'):
            (root/'src/file').chmod(0)
        trace = root/'trace'
        args = ['strace', '--argv0=cp', '-qq', '-yy', '-s', '4096',
                '-e', 'trace='+','.join(CALLS), '-o', str(trace)]
        for injection in injections:
            args += ['-e', 'inject='+injection]
        credentials = {'user': 65534, 'group': 65534, 'extra_groups': []} if ordinary else {}
        destination = 'alias' if target_directory == 'alias' else 'out' if target_directory else 'out/file'
        result = subprocess.run([*args, binary, *options, 'src/file', destination],
                                cwd=root, capture_output=True, timeout=20,
                                env={**os.environ, 'LC_ALL': 'C', 'LANGUAGE': 'C'}, **credentials)
        counts = dict.fromkeys(CALLS, 0)
        events, targets = [], []
        for line in trace.read_text().splitlines():
            call = line.split('(', 1)[0]
            if call not in counts:
                continue
            counts[call] += 1
            pairs = list(re.finditer(r'(AT_FDCWD|\d+)(?:<([^>]+)>)?, "([^"]*)"', line))
            if pairs:
                paths = [match[3] if match[1] == 'AT_FDCWD' else
                         posixpath.join(match[2] or '', match[3]) for match in pairs]
            else:
                paths = re.findall(r'"([^"]*)"', line)
            paths = [posixpath.normpath(path.removeprefix(work+'/')) for path in paths]
            mutation = call.startswith(('rename', 'unlink'))
            if not paths or not any(path.startswith('out/') for path in paths):
                continue
            if not mutation and not any('~' in path for path in paths):
                continue  # Only the backup fallback's metadata checks belong here.
            kind = 'rename' if call.startswith('rename') else 'unlink' if mutation else 'stat'
            flag = ('RENAME_NOREPLACE' in line if kind == 'rename' else
                    call == 'lstat' or 'AT_SYMLINK_NOFOLLOW' in line if kind == 'stat' else False)
            outcome = line.rsplit(' = ', 1)[1].replace(' (INJECTED)', '')
            events.append((kind, paths, flag, outcome))
            targets.append((call, counts[call]))
        if injections:
            assert '(INJECTED)' in trace.read_text(), (injections, trace.read_text())
        tree = []
        for path in sorted((root/'out').iterdir()):
            metadata = path.lstat()
            value = os.readlink(path) if path.is_symlink() else path.read_bytes()
            tree.append((path.name, stat.S_IFMT(metadata.st_mode), stat.S_IMODE(metadata.st_mode),
                         metadata.st_uid, metadata.st_gid, value))
        (root/'src/file').chmod(0o644) if not (root/'src/file').is_symlink() else None
        return (result.returncode, result.stdout, result.stderr, tree, events), targets


def main():
    total = 0
    with tempfile.TemporaryDirectory(prefix='rbox-cp-mutations-bin-') as work:
        root = Path(work)
        root.chmod(0o755)
        binary = root/'rbox'
        shutil.copy2(RBOX, binary)
        for ordinary in ([False, True] if os.geteuid() == 0 else [False]):
            for target_directory in [False, True, 'alias']:
                for profile in ['simple', 'numbered', 'existing', 'numbered-carry', 'existing-carry', 'rollback-simple',
                                'rollback-numbered', 'remove', 'force', 'physical']:
                    want, expected = run(ORACLE, profile, target_directory, ordinary)
                    got, actual = run(str(binary), profile, target_directory, ordinary)
                    assert want == got, (profile, target_directory, ordinary, 'baseline', want, got)
                    assert len(expected) == len(actual)
                    total += 1
                    for expected_target, actual_target in zip(expected, actual):
                        for error in ['EIO', 'EACCES', 'ENOENT', 'EINTR', 'EEXIST', 'ENOSYS', 'EINVAL', 'EOPNOTSUPP']:
                            def inject(target):
                                return f'{target[0]}:error={error}:when={target[1]}'
                            want, _ = run(ORACLE, profile, target_directory, ordinary, [inject(expected_target)])
                            got, _ = run(str(binary), profile, target_directory, ordinary, [inject(actual_target)])
                            assert want == got, (profile, target_directory, ordinary, expected_target, error, want, got)
                            total += 1
                    # Enter the native-NOREPLACE fallback, then fail each
                    # metadata/rename operation introduced by that fallback.
                    for expected_target, actual_target in zip(expected, actual):
                        if expected_target[0] != 'renameat2':
                            continue
                        def unsupported(target):
                            return f'{target[0]}:error=ENOSYS:when={target[1]}'
                        want, fallback_expected = run(ORACLE, profile, target_directory, ordinary,
                                                      [unsupported(expected_target)])
                        got, fallback_actual = run(str(binary), profile, target_directory, ordinary,
                                                   [unsupported(actual_target)])
                        assert want == got and len(fallback_expected) == len(fallback_actual)
                        for left, right in zip(fallback_expected, fallback_actual):
                            if left[0] == expected_target[0] or right[0] == actual_target[0]:
                                continue
                            for error in ['EIO', 'EACCES', 'ENOENT', 'EINTR', 'EOVERFLOW']:
                                want, _ = run(ORACLE, profile, target_directory, ordinary,
                                              [unsupported(expected_target), f'{left[0]}:error={error}:when={left[1]}'])
                                got, _ = run(str(binary), profile, target_directory, ordinary,
                                             [unsupported(actual_target), f'{right[0]}:error={error}:when={right[1]}'])
                                assert want == got, (profile, target_directory, ordinary, 'fallback', left, error, want, got)
                                total += 1
    print(f'GNU cp mutations: {total} exact status/output/tree/operation-order comparisons pass')


if __name__ == '__main__':
    main()
