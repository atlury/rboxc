#!/usr/bin/env python3
"""Run pinned, explicitly selected upstream compatibility tests."""
# SPDX-License-Identifier: GPL-3.0-or-later
import hashlib
import json
import os
from pathlib import Path
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[2]
SOURCE = Path(os.environ.get('GNU_COREUTILS_SOURCE', '/opt/src/coreutils-9.11'))
BUILD = ROOT/'build/gnu-coreutils'
PERL_SELECTION = r'''
no warnings 'redefine';
*main::run_tests = sub ($$$$$) {
    my %approved = map { $_ => 1 } split /,/, $ENV{RBOXC_APPROVED_CASES};
    my $original = $_[2];
    my @selected = grep { exists $approved{$_->[0]} } @$original;
    die "upstream case inventory changed" unless @selected == keys %approved;
    print "RBOXC_SELECTION ", scalar(@selected), " of ", scalar(@$original), "\n";
    $_[2] = \@selected;
    return &Coreutils::run_tests(@_);
};
do $ARGV[0];
die $@ if $@;
'''


def main():
    manifest = json.loads((ROOT/'inventory/gnu-reviewed-tests.json').read_text())
    results = []
    for row in manifest:
        script = SOURCE/row['script']
        assert hashlib.sha256(script.read_bytes()).hexdigest() == row['sha256'], row['script']
        outcomes = {}
        for implementation in ('gnu', 'rboxc'):
            with tempfile.TemporaryDirectory(prefix='rboxc-upstream-') as temporary:
                run = Path(temporary)
                (run/'src').mkdir()
                candidate = BUILD/'src/coreutils' if implementation == 'gnu' else ROOT/'target/release/rboxc'
                (run/'src'/row['command']).symlink_to(candidate)
                (run/'src/getlimits').symlink_to(BUILD/'src/getlimits')
                environment = {
                    **os.environ, 'PATH': f'{run}/src:/opt/gnu/coreutils-9.11/bin:/usr/bin:/bin',
                    'LC_ALL': 'C', 'LANGUAGE': 'C', 'TZ': 'UTC0', 'built_programs': row['command'],
                    'srcdir': str(SOURCE), 'top_srcdir': str(SOURCE), 'abs_srcdir': str(SOURCE),
                    'abs_top_srcdir': str(SOURCE), 'abs_top_builddir': str(run),
                    'CONFIG_HEADER': str(BUILD/'lib/config.h'), 'LOCALE_FR': '',
                    'LOCALE_FR_UTF8': 'none', 'PERL': 'perl', 'AWK': 'awk', 'SHELL': '/bin/sh',
                    'VERBOSE': 'yes', 'RBOXC_APPROVED_CASES': ','.join(row.get('cases', [])),
                }
                for key in ('POSIXLY_CORRECT', 'VERSION_CONTROL', 'SIMPLE_BACKUP_SUFFIX'):
                    environment.pop(key, None)
                if script.suffix == '.pl':
                    assert row['cases'], 'Perl suites require an explicit case selection'
                    command = ['perl', '-I'+str(SOURCE/'tests'), '-MCuSkip', '-MCoreutils',
                               '-e', PERL_SELECTION, str(script)]
                else:
                    command = ['/bin/sh', '-c', 'exec /bin/sh "$1" 9>&2', 'test', str(script)]
                completed = subprocess.run(['timeout', '--kill-after=5s', '60s', *command],
                                           cwd=run, env=environment, capture_output=True)
                log = ROOT/'evidence/raw'/('reviewed-'+row['script'].replace('/', '-')+'-'+implementation+'.log')
                log.write_bytes(completed.stdout+completed.stderr)
                outcomes[implementation] = {'status': completed.returncode, 'log': str(log.relative_to(ROOT))}
        passed = outcomes['gnu']['status'] == outcomes['rboxc']['status'] == 0
        results.append({**row, **outcomes, 'pass': passed})
        print('PASS' if passed else 'OPEN', row['script'], outcomes, flush=True)
    report = {'scope': 'selected original GNU compatibility cases; unselected tests remain open',
              'passed': sum(row['pass'] for row in results), 'total': len(results), 'results': results}
    (ROOT/'evidence/gnu-reviewed-original.json').write_text(json.dumps(report, indent=2)+'\n')
    return report['passed'] != report['total']


if __name__ == '__main__':
    raise SystemExit(main())
