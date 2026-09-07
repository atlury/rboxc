#!/usr/bin/env python3
"""Run pinned, explicitly selected upstream compatibility tests."""
# SPDX-License-Identifier: GPL-3.0-or-later
import hashlib
import json
import os
import re
import shlex
import argparse
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
    die "full-suite selection omitted upstream cases"
        if $ENV{RBOXC_FULL_SUITE} && @selected != @$original;
    $_[2] = \@selected;
    return &Coreutils::run_tests(@_);
};
do $ARGV[0];
die $@ if $@;
'''


def main():
    manifest = json.loads((ROOT/'inventory/gnu-reviewed-tests.json').read_text())
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--valgrind', action='store_true')
    parser.add_argument('commands', nargs='*')
    options = parser.parse_args()
    instrument = options.valgrind
    if instrument:
        manifest = [row for row in manifest if row.get('valgrind')]
    selected = set(options.commands)
    assert selected <= {row['command'] for row in manifest}, 'unknown command selection'
    output = 'evidence/gnu-reviewed-valgrind.json' if instrument else 'evidence/gnu-reviewed-original.json'
    previous = json.loads((ROOT/output).read_text())['results'] if selected and (ROOT/output).exists() else []
    results_by_script = {row['script']: row for row in previous}
    for row in manifest:
        if selected and row['command'] not in selected:
            continue
        script = SOURCE/row['script']
        assert hashlib.sha256(script.read_bytes()).hexdigest() == row['sha256'], row['script']
        outcomes = {}
        for implementation in ('gnu', 'rboxc'):
            with tempfile.TemporaryDirectory(prefix='rboxc-upstream-') as temporary:
                run = Path(temporary)
                (run/'src').mkdir()
                candidate = BUILD/'src/coreutils' if implementation == 'gnu' else ROOT/'target/release/rboxc'
                commands = row.get('commands', [row['command']])
                assert row['command'] in commands
                memory_dir = None
                if instrument:
                    (run/'real').mkdir()
                    memory_dir = ROOT/'evidence/raw'/('reviewed-vg-'+run.name+'-'+implementation)
                    memory_dir.mkdir()
                for command in commands:
                    if instrument:
                        (run/'real'/command).symlink_to(candidate)
                        wrapper = run/'src'/command
                        vg = ['valgrind', '--leak-check=full', '--show-leak-kinds=all',
                              '--track-fds=yes', '--log-file='+str(memory_dir/'%p.log'), command]
                        wrapper.write_text('#!/bin/sh\nPATH='+shlex.quote(str(run/'real'))+':"$PATH"\n'
                                           'export PATH\nexec '+shlex.join(vg)+' "$@"\n')
                        wrapper.chmod(0o755)
                    else:
                        (run/'src'/command).symlink_to(candidate)
                (run/'src/getlimits').symlink_to(BUILD/'src/getlimits')
                environment = {
                    **({'HOME': str(run), 'TMPDIR': str(run)} if row.get('clean_environment') else os.environ), 'PATH': f'{run}/src:/opt/gnu/coreutils-9.11/bin:/usr/bin:/bin',
                    'LC_ALL': 'C', 'LANGUAGE': 'C', 'TZ': 'UTC0', 'built_programs': ' '.join(commands),
                    'srcdir': str(SOURCE), 'top_srcdir': str(SOURCE), 'abs_srcdir': str(SOURCE),
                    'abs_top_srcdir': str(SOURCE), 'abs_top_builddir': str(run),
                    'CONFIG_HEADER': str(BUILD/'lib/config.h'), 'LOCALE_FR': '',
                    'LOCALE_FR_UTF8': 'none', 'PERL': 'perl', 'AWK': 'awk', 'SHELL': '/bin/sh',
                    'RBOXC_FULL_SUITE': '1' if row.get('full_suite') else '',
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
                log = ROOT/'evidence/raw'/('reviewed-'+('vg-' if instrument else '')+row['script'].replace('/', '-')+'-'+implementation+'.log')
                log.write_bytes(completed.stdout+completed.stderr)
                outcomes[implementation] = {'status': completed.returncode, 'log': str(log.relative_to(ROOT))}
                if instrument:
                    memory = []
                    for path in sorted(memory_dir.glob('*.log')):
                        report = path.read_text()
                        errors = re.search(r'ERROR SUMMARY: ([\d,]+) errors', report)
                        fds = re.search(r'FILE DESCRIPTORS: (\d+) open \((\d+) (?:inherited|std)\)', report)
                        memory.append({'log': str(path.relative_to(ROOT)),
                                       'errors': int(errors[1].replace(',', '')) if errors else None,
                                       'non_inherited_descriptors': int(fds[1])-int(fds[2]) if fds else None,
                                       'heap_bytes': {kind: int(found[1].replace(',', ''))
                                           for kind in ['definitely lost','indirectly lost','possibly lost','still reachable']
                                           if (found := re.search(re.escape(kind)+r': ([\d,]+) bytes', report))}})
                    outcomes[implementation]['memory'] = memory
        passed = outcomes['gnu']['status'] == outcomes['rboxc']['status'] == 0
        if instrument:
            memory = outcomes['rboxc']['memory']
            passed &= bool(memory) and all(m['errors'] == 0 and m['non_inherited_descriptors'] == 0 for m in memory)
        results_by_script[row['script']] = {**row, **outcomes, 'pass': passed}
        print('PASS' if passed else 'OPEN', row['script'], {key: value['status'] for key, value in outcomes.items()}, flush=True)
    results = [results_by_script[row['script']] for row in manifest if row['script'] in results_by_script]
    report = {'scope': 'reviewed original GNU scripts and selected compatibility cases; unreviewed tests remain open',
              'passed': sum(row['pass'] for row in results), 'total': len(results), 'results': results}
    output = 'evidence/gnu-reviewed-valgrind.json' if instrument else 'evidence/gnu-reviewed-original.json'
    (ROOT/output).write_text(json.dumps(report, indent=2)+'\n')
    return any(not row['pass'] for row in results if not selected or row['command'] in selected)


if __name__ == '__main__':
    raise SystemExit(main())
