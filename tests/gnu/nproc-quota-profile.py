#!/usr/bin/env python3
"""Stage Valgrind for the original nproc test's disposable quota chroot."""
# SPDX-License-Identifier: GPL-3.0-or-later
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def mount(*args):
    subprocess.run(['/usr/bin/mount', *map(str, args)], check=True)


def main():
    config_path = Path(sys.argv[1]).resolve(strict=True)
    config = json.loads(config_path.read_text())
    inside = sys.argv[2] == '--inside'
    args = sys.argv[3:] if inside else sys.argv[2:]
    root = Path(args[0]).resolve(strict=True)
    assert root.name == 'cgroup' and root.is_relative_to(Path(config['run'])), 'unexpected quota fixture'
    if inside:
        token, command = args[1], args[2:]
        assert os.getpid() == 1, 'private PID namespace did not start'
        assert os.readlink('/proc/self/ns/mnt') != config['mount_namespace']
        assert os.readlink('/proc/self/ns/pid') != config['pid_namespace']
        mount('--make-rprivate', '/')
        saved = root/'.rboxc-valgrind'/token
        # A genuine procfs supplies Valgrind's own maps. Bind only this
        # namespace's PID 1 quota inputs to the original test's fake files.
        # exec keeps PID 1 through chroot, Valgrind, and the nproc command.
        mount('-t', 'proc', 'proc', root/'proc')
        mount('--bind', saved/'sched', root/'proc/1/sched')
        mount('--bind', saved/'cgroup', root/'proc/1/cgroup')
        mount('--bind', '/dev/null', root/'dev/null')
        mount('--bind', config['memory'], root/'rboxc-memory')
        profile = {'private_mount_namespace': os.readlink('/proc/self/ns/mnt'),
                   'private_pid_namespace': os.readlink('/proc/self/ns/pid'),
                   'parent_mount_namespace': config['mount_namespace'],
                   'parent_pid_namespace': config['pid_namespace'],
                   'private_pid': os.getpid(), 'command': command,
                   'nproc_sha256': digest(root/'nproc'), 'preload_sha256': digest(root/'k.so'),
                   'quota': (root/'sys/fs/cgroup/cpu.max').read_text() if (root/'sys/fs/cgroup/cpu.max').exists() else None,
                   'scheduler': (saved/'sched').read_text(),
                   'thread_overrides': {name: os.environ.get(name) for name in ('OMP_NUM_THREADS', 'OMP_THREAD_LIMIT')},
                   'valgrind_runtime': config['runtime'], 'log': token+'.log'}
        (Path(config['memory'])/(token+'.profile.json')).write_text(json.dumps(profile, indent=2)+'\n')
        env = {**os.environ, 'VALGRIND_LIB': '/usr/libexec/valgrind', 'TMPDIR': '/tmp'}
        os.execve(config['gnu'], ['chroot', str(root), '/usr/bin/valgrind.bin',
                  '--leak-check=full', '--show-leak-kinds=all', '--track-fds=yes', '--vgdb=no',
                  '--log-file=/rboxc-memory/'+token+'.log', *command], env)
    assert args[1:] in (['/nproc'], ['/nproc', '--version']), 'unreviewed chroot invocation'
    token = 'quota-'+str(os.getpid())
    for source, expected in config['runtime'].items():
        source = Path(source)
        assert digest(source) == expected, 'Valgrind runtime changed'
        target = root/str(source).lstrip('/')
        target.parent.mkdir(parents=True, exist_ok=True)
        if target.exists():
            assert digest(target) == expected, 'unexpected staged runtime'
        else:
            shutil.copy2(source, target)
    saved = root/'.rboxc-valgrind'/token
    saved.mkdir(parents=True)
    for name in ('sched', 'cgroup'):
        shutil.copy2(root/'proc/self'/name, saved/name)
    for name in ('dev', 'tmp', 'rboxc-memory'):
        (root/name).mkdir(exist_ok=True)
    (root/'dev/null').touch(exist_ok=True)
    result = subprocess.run(['/usr/bin/unshare', '--mount', '--pid', '--fork',
                             sys.executable, __file__, str(config_path), '--inside',
                             str(root), token, *args[1:]])
    assert os.readlink('/proc/self/ns/mnt') == config['mount_namespace'], 'parent mount namespace changed'
    assert os.readlink('/proc/self/ns/pid') == config['pid_namespace'], 'parent PID namespace changed'
    return result.returncode


if __name__ == '__main__':
    raise SystemExit(main())
