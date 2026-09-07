#!/usr/bin/env python3
"""Run a reviewed device test on a new image in a private mount namespace."""
# SPDX-License-Identifier: GPL-3.0-or-later
import json
import os
from pathlib import Path
import stat
import subprocess
import sys

run = Path.cwd()
image = run/'device-test.img'
mountpoint = run/'device-test-mount'
assert os.geteuid() == 0 and len(sys.argv) > 1
assert os.readlink('/proc/self/ns/mnt') != os.readlink('/proc/1/ns/mnt'), 'private mount namespace required'
mountpoint.mkdir()
with image.open('xb') as stream:
    stream.truncate(32 * 1024 * 1024)
subprocess.run(['/usr/sbin/mkfs.ext4', '-q', '-F', image], check=True)
mounted = subprocess.run(['/usr/bin/mount', '-t', 'ext4', '-o', 'loop,nosuid,nodev,noexec',
                          image, mountpoint], capture_output=True)
if mounted.returncode:
    sys.stderr.buffer.write(mounted.stderr)
    raise SystemExit(77)
try:
    source = subprocess.check_output(['/usr/bin/findmnt', '--noheadings', '--output', 'SOURCE',
                                      '--target', mountpoint], text=True).strip()
    device = Path(source).resolve(strict=True)
    device_stat = device.stat()
    assert stat.S_ISBLK(device_stat.st_mode)
    assert mountpoint.stat().st_dev == device_stat.st_rdev
    sysfs = Path('/sys/dev/block')/f'{os.major(device_stat.st_rdev)}:{os.minor(device_stat.st_rdev)}'
    backing = Path((sysfs/'loop/backing_file').read_text().strip())
    assert backing.resolve(strict=True) == image.resolve(strict=True), 'unexpected backing device'
    (mountpoint/'src').symlink_to(run/'src', target_is_directory=True)
    print('RBOXC_LOOPBACK_PROFILE '+json.dumps({'bytes': image.stat().st_size,
          'filesystem': 'ext4', 'verified_backing_image': True,
          'private_mount_namespace': True}), file=sys.stderr, flush=True)
    result = subprocess.run(sys.argv[1:], cwd=mountpoint,
                            env={**os.environ, 'TMPDIR': str(mountpoint), 'PWD': str(mountpoint)})
    status = result.returncode
finally:
    subprocess.run(['/usr/bin/umount', mountpoint], check=True)
raise SystemExit(status)
