"""Private mount profiles for unchanged locale and resolver recipes."""
# SPDX-License-Identifier: GPL-3.0-or-later
import json
from pathlib import Path
import re

from comparison_profile import ROOT, fingerprint

SCRIPT = '/usr/bin/mount --bind "$1" "$2" || exit 77; shift 2; exec "$@"'


def inputs_for(row):
    mode = row.get('private_environment')
    if mode is None:
        return {}
    assert (row['target'], mode) in (
        ('nonfatal1', 'files-only-hosts'), ('jarebug', 'japanese-locale-mount'))
    paths = [Path(__file__), Path('/usr/bin/unshare'), Path('/usr/bin/mount')]
    if mode == 'files-only-hosts':
        paths += [Path('/etc/nsswitch.conf'), Path('/etc/hosts')]
    else:
        paths += [Path('/usr/bin/locale')]
    return {p: fingerprint(p) for p in paths}


def prepare(row, work, saved, command, env):
    mode = row.get('private_environment')
    if mode is None:
        return command, None
    inputs_for(row)
    if mode == 'files-only-hosts':
        host = Path('/etc/nsswitch.conf')
        original = host.read_text()
        text, count = re.subn(r'^hosts:.*$', 'hosts: files', original, flags=re.M)
        assert count == 1 and not re.search(r'\b1\.2\.3\.4\.5\b', Path('/etc/hosts').read_text())
        source = work/'nsswitch.conf'
        source.write_text(text)
        retained = saved/'nsswitch.conf'
        retained.write_text(text)
        destination = str(host)
        extra = {'configuration': str(retained.relative_to(ROOT)),
                 'configuration_sha256': fingerprint(retained),
                 'host_sha256': fingerprint(host)}
    else:
        locale = json.loads((ROOT/row['locale_profile']).read_text())
        assert locale['name'] == 'ja_JP.EUC-JP'
        source = Path(locale['runtime_path'])
        destination = '/usr/lib/locale'
        env['LOCPATH'] = destination
        extra = {'locale_profile': row['locale_profile'], 'LOCPATH': destination}
    prefix = ['/usr/bin/unshare', '--mount', '--propagation', 'private',
              '/bin/sh', '-c', SCRIPT, 'gawk-private-environment', str(source), destination]
    return prefix + command, {'mode': mode, 'prefix': prefix, **extra}


def verify(row, outcome, inputs):
    mode = row.get('private_environment')
    record = outcome.get('private_environment')
    if mode is None:
        assert record is None
        return
    assert record['mode'] == mode
    for path, digest in inputs_for(row).items():
        assert inputs[str(path)] == digest
    if mode == 'files-only-hosts':
        host = Path('/etc/nsswitch.conf')
        assert fingerprint(host) == record['host_sha256']
        retained = ROOT/record['configuration']
        assert fingerprint(retained) == record['configuration_sha256']
        expected, count = re.subn(r'^hosts:.*$', 'hosts: files', host.read_text(), flags=re.M)
        assert count == 1 and retained.read_text() == expected
        source = outcome['private_work_directory'] + '/nsswitch.conf'
        destination = str(host)
    else:
        assert record['locale_profile'] == row['locale_profile']
        locale = json.loads((ROOT/row['locale_profile']).read_text())
        source = locale['runtime_path']
        destination = '/usr/lib/locale'
        assert record['LOCPATH'] == destination
    assert record['prefix'] == ['/usr/bin/unshare', '--mount', '--propagation', 'private',
        '/bin/sh', '-c', SCRIPT, 'gawk-private-environment', source, destination]
