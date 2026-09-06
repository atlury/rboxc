#!/usr/bin/env python3
"""Run reviewed GNU cp compatibility tests in separate temporary fixtures."""
# SPDX-License-Identifier: GPL-3.0-or-later
import json
import hashlib
import os
from pathlib import Path
import shutil
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[2]
SOURCE = Path(os.environ.get('GNU_COREUTILS_SOURCE', '/opt/src/coreutils-9.11'))
BUILD = ROOT/'build/gnu-coreutils'
EXCLUDED = {
    'abuse': 'resource stress profile deferred',
    'cp-a-selinux': 'SELinux excluded by project scope',
    'no-ctx': 'SELinux regression profile excluded',
    'cp-mv-enotsup-xattr': 'mount-dependent profile deferred',
    'cross-dev-symlink': 'mount-dependent profile deferred',
    'existing-perm-race': 'race reproduction excluded from compatibility runner',
    'file-perm-race': 'race reproduction excluded from compatibility runner',
    'nfs-removal-race': 'race reproduction excluded from compatibility runner',
    'parent-perm-race': 'race reproduction excluded from compatibility runner',
    'copy-FMR': 'memory vulnerability reproduction excluded from compatibility runner',
    'link-heap': 'resource stress profile deferred',
    'proc-short-read': 'procfs profile deferred',
    'proc-zero-len': 'procfs profile deferred',
    'sparse-perf': 'performance profile deferred',
    'sparse-to-pipe': 'pipe profile deferred',
}


def main():
    approved = json.loads((ROOT/'inventory/gnu-cp-tests.json').read_text())
    discovered = {path.name: path for path in (SOURCE/'tests/cp').glob('*.sh')}
    assert set(discovered) == {row['name'] for row in approved}, 'GNU cp test inventory changed; review required'
    for row in approved:
        assert hashlib.sha256(discovered[row['name']].read_bytes()).hexdigest() == row['sha256'], row['name']
        assert row['enabled'] == (Path(row['name']).stem not in EXCLUDED), row['name']
    results = []
    profiles = [('current-user', {})]
    if os.geteuid() == 0:
        profiles.append(('ordinary-user', {'user': 65534, 'group': 65534, 'extra_groups': []}))
    with tempfile.TemporaryDirectory(prefix='rboxc-gnu-cp-') as temporary:
        stage = Path(temporary)
        stage.chmod(0o755)
        # Staging avoids granting ordinary users access to /root.
        for name, source in [('rboxc', ROOT/'target/release/rboxc'),
                             ('coreutils', BUILD/'src/coreutils'),
                             ('getlimits', BUILD/'src/getlimits'),
                             ('config.h', BUILD/'lib/config.h')]:
            shutil.copy2(source, stage/name)
        for profile, credentials in profiles:
            for test in sorted((SOURCE/'tests/cp').glob('*.sh')):
                if test.stem in EXCLUDED:
                    results.append({'test': test.name, 'profile': profile,
                                    'state': 'excluded', 'reason': EXCLUDED[test.stem]})
                    continue
                outcomes = {}
                for implementation in ('coreutils', 'rboxc'):
                    run = stage/f'{profile}-{test.stem}-{implementation}'
                    (run/'src').mkdir(parents=True)
                    if credentials:
                        os.chown(run, 65534, 65534)
                    (run/'src/cp').symlink_to(stage/implementation)
                    (run/'src/getlimits').symlink_to(stage/'getlimits')
                    environment = {
                        **os.environ, 'PATH': f'{run}/src:/opt/gnu/coreutils-9.11/bin:/usr/bin:/bin',
                        'LC_ALL': 'C', 'LANGUAGE': 'C', 'built_programs': 'cp mv',
                        'srcdir': str(SOURCE), 'top_srcdir': str(SOURCE),
                        'abs_srcdir': str(SOURCE), 'abs_top_srcdir': str(SOURCE),
                        'abs_top_builddir': str(run), 'CONFIG_HEADER': str(stage/'config.h'),
                        'NON_ROOT_USERNAME': '+65534:+65534', 'NON_ROOT_GID': '65534',
                        'LOCALE_FR_UTF8': 'none', 'PERL': 'perl', 'AWK': 'awk', 'SHELL': '/bin/sh',
                        'RUN_EXPENSIVE_TESTS': 'no', 'RUN_VERY_EXPENSIVE_TESTS': 'no',
                    }
                    for key in ('POSIXLY_CORRECT', 'VERSION_CONTROL', 'SIMPLE_BACKUP_SUFFIX'):
                        environment.pop(key, None)
                    completed = subprocess.run(['timeout', '--kill-after=5s', '60s', '/bin/sh',
                                                '-c', 'exec /bin/sh "$1" 9>&2', 'test', str(test)],
                                               cwd=run, env=environment, capture_output=True,
                                               **credentials)
                    log = ROOT/'evidence/raw'/f'gnu-cp-{profile}-{test.stem}-{implementation}.log'
                    log.write_bytes(completed.stdout + completed.stderr)
                    outcomes[implementation] = {'status': completed.returncode,
                                                'log': str(log.relative_to(ROOT))}
                candidate = outcomes['rboxc']['status']
                reference = outcomes['coreutils']['status']
                state = ('pass' if candidate == reference == 0 else
                         'skip' if candidate == reference == 77 else
                         'baseline-failure' if candidate == reference else 'mismatch')
                results.append({'test': test.name, 'profile': profile, 'state': state, **outcomes})
                print(profile, test.name, state, candidate, reference, flush=True)
    counts = {state: sum(row['state'] == state for row in results)
              for state in ('pass', 'skip', 'excluded', 'baseline-failure', 'mismatch')}
    report = {'provider': 'GNU Coreutils 9.11', 'counts': counts, 'results': results}
    (ROOT/'evidence/gnu-cp-original.json').write_text(json.dumps(report, indent=2)+'\n')
    print(counts)
    return bool(counts['baseline-failure'] or counts['mismatch'])


if __name__ == '__main__':
    raise SystemExit(main())
