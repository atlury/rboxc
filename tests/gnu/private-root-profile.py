#!/usr/bin/env python3
"""Run the original ownership root-guard test in a disposable unprivileged root."""
# SPDX-License-Identifier: GPL-3.0-or-later
import ctypes
from contextlib import ExitStack
import hashlib
import json
import os
from pathlib import Path
import re
import shutil
import stat
import subprocess
import sys
import tempfile


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    config = json.loads(Path(sys.argv[1]).read_text())
    command = sys.argv[2:]
    assert os.geteuid() == 0
    assert config['script'] == 'tests/chown/preserve-root.sh'
    assert command[-1] == str(Path(config['source'])/config['script'])
    mount_namespace = os.readlink('/proc/self/ns/mnt')
    pid_namespace = os.readlink('/proc/self/ns/pid')
    assert mount_namespace != config['parent_mount_namespace']
    assert pid_namespace != config['parent_pid_namespace']
    subprocess.run(['/usr/bin/mount', '--make-rprivate', '/'], check=True)
    run = Path(config['run'])
    host_root = os.stat('/')
    staged_files = {}
    with tempfile.TemporaryDirectory(prefix='rboxc-root-guard-', dir='/tmp') as temporary, ExitStack() as mounts:
        root = Path(temporary)
        assert not root.is_relative_to(run) and not run.is_relative_to(root)
        root.chmod(0o755)

        def destination(path):
            path = Path(path)
            assert path.is_absolute() and '..' not in path.parts
            return root/str(path).lstrip('/')

        def copy_file(source):
            source = Path(source)
            target = destination(source)
            if str(source) in staged_files:
                return
            target.parent.mkdir(parents=True, exist_ok=True)
            shutil.copy2(source, target)
            staged_files[str(source)] = digest(source)
            assert digest(target) == staged_files[str(source)]
            if source.read_bytes()[:4] == b'\x7fELF':
                result = subprocess.run(['/usr/bin/ldd', str(source)],
                    capture_output=True, text=True)
                assert result.returncode == 0 or any(message in result.stdout+result.stderr
                    for message in ('not a dynamic executable', 'statically linked'))
                dependencies = result.stdout
                for dependency in re.findall(r'(?:=>\s+|^\s*)(/[^\s]+)', dependencies, re.M):
                    copy_file(dependency)

        # Copies and internal symlinks only: no host directory is exposed in
        # the child root. GNU's helpers supply the untested framework commands.
        source = Path(config['source'])
        for name in ('init.cfg', 'tests/init.sh', config['script']):
            copy_file(source/name)
        copy_file('/bin/sh')
        for name in ('grep', 'sed', 'awk', 'diff', 'getconf'):
            copy_file('/usr/bin/'+name)
        copy_file(config['gnu'])
        for name in config['framework_commands']:
            alias = destination('/usr/bin/'+name)
            if not alias.exists():
                alias.symlink_to(config['gnu'])
        copy_file(config['candidate'])
        copy_file(config['getlimits'])
        copy_file(config['config_header'])
        staged_run = destination(run)
        staged_run.parent.mkdir(parents=True, exist_ok=True)
        shutil.copytree(run, staged_run, symlinks=True)
        os.chown(staged_run, 65534, 65534)
        if config['instrument']:
            for path in config['valgrind_runtime']:
                copy_file(path)
            staged_memory = destination(config['memory'])
            os.chown(staged_memory, 65534, 65534)
        for directory in ('etc', 'dev', 'proc', 'tmp'):
            (root/directory).mkdir(exist_ok=True)
        (root/'tmp').chmod(0o1777)
        (root/'etc/passwd').write_text('root:x:0:0:root:/:/bin/sh\nnobody:x:65534:65534:nobody:/tmp:/bin/sh\n')
        (root/'etc/group').write_text('root:x:0:\nnobody:x:65534:\n')
        (root/'etc/nsswitch.conf').write_text('passwd: files\ngroup: files\n')
        subprocess.run(['/usr/bin/mount', '-t', 'tmpfs', '-o', 'mode=755,nosuid',
                        'tmpfs', str(root/'dev')], check=True)
        mounts.callback(subprocess.run, ['/usr/bin/umount', str(root/'dev')], check=True)
        os.mknod(root/'dev/null', stat.S_IFCHR | 0o666, os.makedev(1, 3))
        os.mknod(root/'dev/zero', stat.S_IFCHR | 0o666, os.makedev(1, 5))
        (root/'dev/null').chmod(0o666)
        (root/'dev/zero').chmod(0o666)
        subprocess.run(['/usr/bin/mount', '-t', 'proc', '-o', 'ro,nosuid,nodev,noexec',
                        'proc', str(root/'proc')], check=True)
        mounts.callback(subprocess.run, ['/usr/bin/umount', str(root/'proc')], check=True)
        root_identity = [root.stat().st_dev, root.stat().st_ino]
        assert root_identity != [host_root.st_dev, host_root.st_ino]
        libc = ctypes.CDLL(None, use_errno=True)

        def enter_root():
            os.chroot(root)
            os.chdir(run)
            os.setgroups([])
            os.setgid(65534)
            os.setuid(65534)
            assert libc.prctl(38, 1, 0, 0, 0) == 0  # PR_SET_NO_NEW_PRIVS
            actual = os.stat('/')
            assert [actual.st_dev, actual.st_ino] == root_identity
            assert os.geteuid() == os.getegid() == 65534 and not os.getgroups()
            fd = os.open('/dev/null', os.O_RDWR)
            device = os.fstat(fd)
            os.close(fd)
            assert stat.S_ISCHR(device.st_mode) and device.st_rdev == os.makedev(1, 3)
            profile = {'root_identity': root_identity, 'host_root_identity':
                [host_root.st_dev, host_root.st_ino], 'uid': os.geteuid(), 'gid': os.getegid(),
                'groups': os.getgroups(), 'no_new_privileges': True, 'private_null_device': True,
                'private_pid': os.getpid(),
                'mount_namespace': mount_namespace, 'pid_namespace': pid_namespace,
                'parent_mount_namespace': config['parent_mount_namespace'],
                'parent_pid_namespace': config['parent_pid_namespace'],
                'staged_files': staged_files, 'candidate_sha256': digest(Path(config['candidate']))}
            os.write(2, ('RBOXC_PRIVATE_ROOT_PROFILE '+json.dumps(profile)+'\n').encode())

        environment = {**os.environ, 'PATH': str(run/'src')+':/usr/bin:/bin',
                       'TMPDIR': '/tmp', 'HOME': '/tmp', 'VALGRIND_LIB': '/usr/libexec/valgrind'}
        completed = subprocess.run(command, env=environment, preexec_fn=enter_root)
        if config['instrument']:
            for path in destination(config['memory']).glob('*.log'):
                shutil.copy2(path, Path(config['memory'])/path.name)
        after = os.stat('/')
        assert (after.st_dev, after.st_ino, after.st_uid, after.st_gid, after.st_mode) == (
            host_root.st_dev, host_root.st_ino, host_root.st_uid, host_root.st_gid, host_root.st_mode)
        print('RBOXC_PRIVATE_ROOT_PARENT_UNCHANGED', file=sys.stderr, flush=True)
        return completed.returncode


if __name__ == '__main__':
    raise SystemExit(main())
