#!/usr/bin/env python3
"""Run pinned, explicitly selected upstream compatibility tests."""
# SPDX-License-Identifier: GPL-3.0-or-later
import hashlib
import json
import os
import re
import shlex
import shutil
import argparse
from pathlib import Path
import subprocess
import tempfile
import sys
import time

ROOT = Path(__file__).resolve().parents[2]
SOURCE = Path(os.environ.get('GNU_COREUTILS_SOURCE', '/opt/src/coreutils-9.11'))
BUILD = ROOT/'build/gnu-coreutils'
sys.path.insert(0, str(ROOT/'scripts'))
from generated_tests import materialize
PERL_SELECTION = r'''
no warnings 'redefine';
*main::run_tests = sub ($$$$$) {
    my %approved = map { $_ => 1 } split /,/, $ENV{RBOXC_APPROVED_CASES};
    my $original = $_[2];
    my $all = $ENV{RBOXC_FULL_SUITE} && !keys %approved;
    my @selected = $all ? @$original : grep { exists $approved{$_->[0]} } @$original;
    die "upstream case inventory changed" unless $all || @selected == keys %approved;
    print "RBOXC_SELECTION ", scalar(@selected), " of ", scalar(@$original), "\n";
    die "full-suite selection omitted upstream cases"
        if $ENV{RBOXC_FULL_SUITE} && @selected != @$original;
    $_[2] = \@selected;
    return &Coreutils::run_tests(@_);
};
do $ARGV[0];
die $@ if $@;
'''

PERL_TTY_EOF = r'''
require Expect;
print STDERR "RBOXC_EXPECT_VERSION $Expect::VERSION\n";
my $original_spawn = \&Expect::spawn;
no warnings 'redefine';
*Expect::spawn = sub {
    print STDERR "RBOXC_TTY_COMMAND $_[1]\n";
    goto &$original_spawn;
};
do $ARGV[0];
die $@ if $@;
'''


def parse_memory_log(report, pid, exec_only=False):
    """Assess all summaries, including append-only logs spanning reviewed execs."""
    errors = list(re.finditer(r'ERROR SUMMARY: ([\d,]+) errors', report))
    fds = list(re.finditer(r'FILE DESCRIPTORS: (\d+) open \((\d+) (?:inherited|std)\)', report))
    entry = {
        'errors': sum(int(found[1].replace(',', '')) for found in errors) if errors else None,
        'non_inherited_descriptors': sum(int(found[1])-int(found[2]) for found in fds) if fds else None,
        'heap_bytes': {kind: sum(int(found.replace(',', '')) for found in matches)
            for kind in ['definitely lost', 'indirectly lost', 'possibly lost', 'still reachable']
            if (matches := re.findall(re.escape(kind)+r': ([\d,]+) bytes', report))}}
    if exec_only:
        pids = sorted(set(re.findall(r'^==([0-9]+)==', report, re.M)))
        images = list(re.finditer(r'^==[0-9]+== Command: ', report, re.M))
        # This profile only execs: forked writers or an unfinished
        # last image must never be accepted as a clean process.
        entry['exec_images'] = len(images)
        entry['log_pids'] = pids
        entry['complete_exec_log'] = (pids == [pid] and bool(images)
            and bool(errors) and bool(fds) and len(errors) == len(fds)
            and errors[-1].start() > images[-1].start()
            and fds[-1].start() > images[-1].start())
    return entry


def main():
    manifest = json.loads((ROOT/'inventory/gnu-reviewed-tests.json').read_text())
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--valgrind', action='store_true')
    parser.add_argument('--candidate', type=Path, default=ROOT/'target/release/rboxc',
                        help='Candidate executable for a separately named comparison batch')
    parser.add_argument('--report-name', help='Separate evidence filename stem for independent test batches')
    parser.add_argument('commands', nargs='*')
    parser.add_argument('--script', action='append', default=[])
    options = parser.parse_args()
    watchdog = Path('/usr/bin/timeout').resolve(strict=True)
    watchdog_hash = hashlib.sha256(watchdog.read_bytes()).hexdigest()
    candidate_binary = options.candidate.resolve(strict=True)
    assert candidate_binary == ROOT/'target/release/rboxc' or options.report_name, 'alternate candidates require a separate report'
    instrument = options.valgrind
    if instrument:
        manifest = [row for row in manifest if row.get('valgrind')]
    selected = set(options.commands)
    assert selected <= {row['command'] for row in manifest}, 'unknown command selection'
    selected_scripts = set(options.script)
    assert selected_scripts <= {row['script'] for row in manifest}, 'unknown script selection'
    def included(row):
        return (not selected or row['command'] in selected) and (not selected_scripts or row['script'] in selected_scripts)
    output = 'evidence/gnu-reviewed-valgrind.json' if instrument else 'evidence/gnu-reviewed-original.json'
    if options.report_name:
        assert re.fullmatch(r'[a-z0-9][a-z0-9-]*', options.report_name), 'invalid report name'
        output = 'evidence/raw/'+options.report_name+'.json'
    previous = json.loads((ROOT/output).read_text())['results'] if (selected or selected_scripts) and (ROOT/output).exists() else []
    results_by_script = {row['script']: row for row in previous}
    def checkpoint():
        results = [results_by_script[row['script']] for row in manifest if row['script'] in results_by_script]
        report = {'scope': 'reviewed original GNU scripts and selected compatibility cases; unreviewed tests remain open',
                  'passed': sum(row['pass'] for row in results), 'total': len(results), 'results': results}
        destination = ROOT/output
        temporary = destination.with_suffix('.json.tmp')
        temporary.write_text(json.dumps(report, indent=2)+'\n')
        temporary.replace(destination)
        return results

    native_launchers = {}
    for direct, log_fd in sorted({(bool(row.get('direct_valgrind')), bool(row.get('valgrind_log_fd'))) for row in manifest
                          if instrument and row.get('native_launcher') and included(row)}):
        native_launcher = ROOT/('build/valgrind-launch'+('-direct' if direct else '')+('-log-fd' if log_fd else ''))
        extra = ['-DRBOXC_VALGRIND_EXECUTABLE="/usr/bin/valgrind.bin"'] if direct else []
        if log_fd:
            extra.append('-DRBOXC_VALGRIND_LOG_FD=1')
        if direct:
            assert Path('/usr/bin/valgrind.bin').read_bytes()[:4] == b'\x7fELF'
        subprocess.run(['cc', '-O2', '-Wall', '-Wextra', '-Werror',
                        *extra,
                        ROOT/'tests/gnu/valgrind-launch.c', '-o', native_launcher], check=True)
        native_launchers[direct, log_fd] = native_launcher
    tmpdir_adapter = None
    if instrument and any(row.get('valgrind_tmpdir_adapter') and included(row) for row in manifest):
        tmpdir_adapter = ROOT/'build/valgrind-tmpdir.so'
        subprocess.run(['cc', '-shared', '-fPIC', '-O2', '-Wall', '-Wextra', '-Werror',
                        ROOT/'tests/gnu/valgrind-tmpdir.c', '-o', tmpdir_adapter], check=True)
    binary_hashes = {name: hashlib.sha256(path.read_bytes()).hexdigest() for name, path in
                     [('gnu', BUILD/'src/coreutils'), ('rboxc', candidate_binary)]}
    for row in manifest:
        if not included(row):
            continue
        script = materialize(ROOT, SOURCE, row) if row.get('generator_inputs') else SOURCE/row['script']
        assert hashlib.sha256(script.read_bytes()).hexdigest() == row['sha256'], row['script']
        assert row.get('profile') in (None, 'ordinary-user', 'loopback-device', 'private-mount'), 'unknown execution profile'
        if row.get('valgrind_log_fd'):
            assert row.get('native_launcher') and row['script'] == 'tests/chroot/chroot-credentials.sh', 'log descriptor mode requires a reviewed exec-only profile'
            assert row.get('valgrind_vgdb') is False, 'credential changes require disabling vgdb files'
        assert row.get('perl_driver') in (None, 'tty-eof'), 'unknown Perl driver'
        if row.get('perl_driver') == 'tty-eof':
            assert row['script'] == 'tests/misc/tty-eof.pl' and row.get('full_suite')
            assert set(command.split()[0] for command in row['tty_commands']) == set(row['commands'])
        test_shell = row.get('test_shell', '/bin/sh')
        assert test_shell in ('/bin/sh', '/bin/bash'), 'unsupported test shell'
        outcomes = {}
        for implementation in ('gnu', 'rboxc'):
            with tempfile.TemporaryDirectory(prefix='rboxc-upstream-') as temporary:
                run = Path(temporary)
                (run/'src').mkdir()
                candidate = BUILD/'src/coreutils' if implementation == 'gnu' else candidate_binary
                credentials = {}
                helper = BUILD/'src/getlimits'
                config_header = BUILD/'lib/config.h'
                if row.get('profile') == 'ordinary-user' and os.geteuid() == 0:
                    credentials = {'user': 65534, 'group': 65534, 'extra_groups': []}
                if credentials or row.get('shared_runtime'):
                    run.chmod(0o755)
                    if credentials:
                        os.chown(run, 65534, 65534)
                    runtime = run/'runtime'
                    runtime.mkdir()
                    copied = runtime/candidate.name
                    shutil.copy2(candidate, copied)
                    candidate = copied
                    shutil.copy2(helper, runtime/'getlimits')
                    shutil.copy2(config_header, runtime/'config.h')
                    helper = runtime/'getlimits'
                    config_header = runtime/'config.h'
                commands = row.get('commands', [row['command']])
                assert row['command'] in commands
                stdbuf_library = None
                if 'stdbuf' in commands:
                    stdbuf_library = (BUILD/'src/libstdbuf.so' if implementation == 'gnu'
                                      else candidate_binary.parent/'libstdbuf.so')
                    shutil.copy2(stdbuf_library, run/'src/libstdbuf.so')
                memory_dir = None
                if instrument:
                    (run/'real').mkdir()
                    if stdbuf_library:
                        shutil.copy2(stdbuf_library, run/'real/libstdbuf.so')
                    memory_dir = ROOT/'evidence/raw'/('reviewed-vg-'+run.name+'-'+implementation)
                    memory_dir.mkdir()
                runtime_memory_dir = memory_dir
                if instrument and (credentials or row.get('native_launcher') or row.get('shared_runtime')):
                    runtime_memory_dir = run/'memory'
                    runtime_memory_dir.mkdir()
                    if credentials:
                        os.chown(runtime_memory_dir, 65534, 65534)
                    elif row.get('shared_runtime'):
                        runtime_memory_dir.chmod(0o1777)
                if instrument and row.get('native_launcher'):
                    shutil.copy2(native_launchers[bool(row.get('direct_valgrind')), bool(row.get('valgrind_log_fd'))], run/'src/.valgrind-launch')
                tmpdir_library = None
                if instrument and row.get('valgrind_tmpdir_adapter'):
                    assert not row.get('native_launcher'), 'TMPDIR adapter requires the shell launcher'
                    tmpdir_library = run/'src/.valgrind-tmpdir.so'
                    shutil.copy2(tmpdir_adapter, tmpdir_library)
                for command in commands:
                    if instrument:
                        (run/'real'/command).symlink_to(candidate)
                        wrapper = run/'src'/command
                        if row.get('native_launcher'):
                            wrapper.symlink_to('.valgrind-launch')
                            continue
                        vg = ['valgrind', '--leak-check=full', '--show-leak-kinds=all',
                              '--track-fds=yes', *(['--vgdb=no'] if row.get('valgrind_vgdb') is False else []), *(['--trace-children=yes'] if row.get('trace_children') else []),
                              '--log-file='+str(runtime_memory_dir/'%p.log'), command]
                        startup = ''
                        if tmpdir_library:
                            # Valgrind needs an existing directory at startup. The
                            # constructor restores the original TMPDIR in its client.
                            startup = ('RBOXC_VALGRIND_TMPDIR_PRESENT=0\n'
                                       'if [ "${TMPDIR+x}" = x ]; then RBOXC_VALGRIND_TMPDIR_PRESENT=1; fi\n'
                                       'RBOXC_VALGRIND_TMPDIR_VALUE=${TMPDIR-}\n'
                                       'export RBOXC_VALGRIND_TMPDIR_PRESENT RBOXC_VALGRIND_TMPDIR_VALUE\n'
                                       'TMPDIR='+shlex.quote(str(run))+'\n'
                                       'LD_PRELOAD='+shlex.quote(str(tmpdir_library))+':${LD_PRELOAD-}\n'
                                       'export TMPDIR LD_PRELOAD\n')
                        wrapper.write_text('#!/bin/sh\nPATH='+shlex.quote(str(run/'real'))+':"$PATH"\n'
                                           'export PATH\n'+startup+'exec '+shlex.join(vg)+' "$@"\n')
                        wrapper.chmod(0o755)
                    else:
                        (run/'src'/command).symlink_to(candidate)
                (run/'src/getlimits').symlink_to(helper)
                if row.get('python3_helper'):
                    (run/'src/python').symlink_to(sys.executable)
                environment = {
                    **({'HOME': str(run), 'TMPDIR': str(run)} if row.get('clean_environment') else os.environ), 'PATH': f'{run}/src:/opt/gnu/coreutils-9.11/bin:/usr/bin:/bin',
                    'LC_ALL': 'C', 'LANGUAGE': 'C', 'TZ': 'UTC0', 'built_programs': ' '.join(row.get('built_programs', commands)),
                    'srcdir': str(SOURCE), 'top_srcdir': str(SOURCE), 'abs_srcdir': str(SOURCE),
                    'abs_top_srcdir': str(SOURCE), 'abs_top_builddir': str(run),
                    'CONFIG_HEADER': str(config_header), 'LOCALE_FR': row.get('locale_fr', ''),
                    'LOCALE_FR_UTF8': row.get('locale_fr_utf8', 'none'), 'VERSION': '9.11', 'PACKAGE_VERSION': '9.11',
                    'EXEEXT': '', 'host_os': 'linux-gnu', 'CC': 'cc', 'EGREP': 'grep -E', 'MAKE': 'make', 'PERL': 'perl', 'AWK': 'awk', 'SHELL': test_shell,
                    'RBOXC_FULL_SUITE': '1' if row.get('full_suite') else '',
                    'VERBOSE': 'yes', 'RBOXC_APPROVED_CASES': ','.join(row.get('cases', [])),
                }
                for key in ('POSIXLY_CORRECT', 'VERSION_CONTROL', 'SIMPLE_BACKUP_SUFFIX'):
                    environment.pop(key, None)
                if row.get('shared_runtime'):
                    shared_tmp = run/'tmp'
                    shared_tmp.mkdir(mode=0o1777)
                    shared_tmp.chmod(0o1777)
                    environment['TMPDIR'] = str(shared_tmp)
                if row.get('very_expensive'):
                    environment['RUN_VERY_EXPENSIVE_TESTS'] = 'yes'
                if row.get('expensive'):
                    environment['RUN_EXPENSIVE_TESTS'] = 'yes'
                if row.get('locale_profile') == 'extended':
                    locale_path = Path(json.loads((ROOT/'evidence/test-locales.json').read_text())['runtime_path'])
                    assert locale_path.is_dir(), 'run scripts/prepare-test-locales.py first'
                    environment['LOCPATH'] = str(locale_path)
                if script.suffix == '.pl':
                    assert row.get('cases') or (row.get('full_suite') and row.get('expected_case_count')), 'Perl suites require a reviewed selection or full case count'
                    command = ['perl', '-I'+str(SOURCE/'tests'), '-MCuSkip', '-MCoreutils',
                               '-e', PERL_TTY_EOF if row.get('perl_driver') == 'tty-eof' else PERL_SELECTION, str(script)]
                else:
                    command = [test_shell, '-c', 'exec "$1" "$2" 9>&2', 'test', test_shell, str(script)]
                nss_profile = None
                if row.get('nss_profile') == 'local-files':
                    assert os.geteuid() == 0, 'local-files NSS profile requires private mount privileges'
                    if credentials:
                        groups = row.get('supplementary_groups', credentials['extra_groups'])
                        group_option = '--groups='+','.join(map(str, groups)) if groups else '--clear-groups'
                        command = ['/usr/bin/setpriv', '--reuid='+str(credentials['user']),
                                   '--regid='+str(credentials['group']), group_option,
                                   '--no-new-privs', *command]
                    original_nss = Path('/etc/nsswitch.conf').read_text()
                    private_nss = re.sub(r'^(passwd|group|shadow|gshadow|initgroups):.*$',
                                         r'\1: files', original_nss, flags=re.M)
                    private_config = run/'nsswitch.conf'
                    private_config.write_text(private_nss)
                    nss_profile = {'profile': 'local-files',
                                   'host_sha256': hashlib.sha256(original_nss.encode()).hexdigest(),
                                   'private_sha256': hashlib.sha256(private_nss.encode()).hexdigest()}
                    command = ['/usr/bin/unshare', '--mount', '--propagation', 'private',
                               '/bin/sh', '-c', '/usr/bin/mount --bind "$1" /etc/nsswitch.conf || exit 77; shift; exec "$@"',
                               'local-nss', str(private_config), *command]
                # Enter the private namespace before dropping credentials.
                # Ordinary tests without this profile still drop in Popen.
                launch_credentials = {} if nss_profile and credentials else dict(credentials)
                parent_mount_namespace = os.readlink('/proc/self/ns/mnt')
                if row.get('profile') == 'private-mount':
                    assert os.geteuid() == 0 and not credentials and not nss_profile
                    namespace_driver = ("import os,sys; "
                                        "print('RBOXC_MOUNT_NAMESPACE '+os.readlink('/proc/self/ns/mnt'), file=sys.stderr, flush=True); "
                                        "os.execv(sys.argv[1], sys.argv[1:])")
                    command = ['/usr/bin/unshare', '--mount', '--propagation', 'private',
                               sys.executable, '-c', namespace_driver, *command]
                if row.get('profile') == 'loopback-device':
                    assert not credentials and not nss_profile
                    command = ['/usr/bin/unshare', '--mount', '--propagation', 'private',
                               sys.executable, str(ROOT/'tests/gnu/loopback-profile.py'), *command]
                parent_groups = os.getgroups()
                if 'supplementary_groups' in row:
                    groups = row['supplementary_groups']
                    assert os.geteuid() == 0 and isinstance(groups, list)
                    assert all(type(group) is int and 0 <= group < 2**32-1 for group in groups)
                    launch_credentials['extra_groups'] = groups
                fixture = run
                if row.get('separate_fixture'):
                    assert script.suffix == '.pl', 'separate fixture is a Perl harness profile'
                    fixture = run/'fixture'
                    fixture.mkdir()
                    if credentials:
                        os.chown(fixture, 65534, 65534)
                if row.get('terminal'):
                    driver = run/'terminal-profile.py'
                    shutil.copy2(ROOT/'tests/gnu/terminal-profile.py', driver)
                    command = [sys.executable, str(driver), *command]
                started = time.monotonic()
                config_hash = hashlib.sha256(config_header.read_bytes()).hexdigest()
                completed = subprocess.run([str(watchdog), '--kill-after=5s', str(row.get('timeout_seconds', 60))+'s', *command],
                                           cwd=fixture, env=environment, capture_output=True, **launch_credentials)
                assert os.getgroups() == parent_groups, 'parent group membership changed'
                log = ROOT/'evidence/raw'/('reviewed-'+('vg-' if instrument else '')+row['script'].replace('/', '-')+'-'+implementation+'-'+run.name+'.log')
                log.write_bytes(completed.stdout+completed.stderr)
                outcomes[implementation] = {'status': completed.returncode, 'log': str(log.relative_to(ROOT)),
                                            'binary_sha256': binary_hashes[implementation],
                                            'config_header_sha256': config_hash,
                                            'watchdog': {'path': str(watchdog), 'sha256': watchdog_hash},
                                            'test_shell': test_shell,
                                            'elapsed_seconds': round(time.monotonic()-started, 3)}
                if row.get('profile') == 'private-mount':
                    namespaces = re.findall(rb'^RBOXC_MOUNT_NAMESPACE (.+)$', completed.stderr, re.M)
                    assert len(namespaces) == 1, 'private mount namespace did not start'
                    child_namespace = namespaces[0].decode()
                    assert child_namespace != parent_mount_namespace, 'mount namespace was not isolated'
                    assert os.readlink('/proc/self/ns/mnt') == parent_mount_namespace, 'parent namespace changed'
                    outcomes[implementation]['mount_namespace'] = {
                        'parent': parent_mount_namespace, 'child': child_namespace,
                        'private_propagation': True, 'parent_unchanged': True}
                if row.get('terminal'):
                    profiles = re.findall(rb'^RBOXC_TERMINAL_PROFILE (.+)$', completed.stderr, re.M)
                    outcomes[implementation]['terminal_profile'] = json.loads(profiles[0]) if profiles else None
                    outcomes[implementation]['terminal_runner_sha256'] = hashlib.sha256((ROOT/'tests/gnu/terminal-profile.py').read_bytes()).hexdigest()
                if row.get('profile') == 'loopback-device':
                    profiles = re.findall(rb'^RBOXC_LOOPBACK_PROFILE (.+)$', completed.stderr, re.M)
                    outcomes[implementation]['loopback_profile'] = json.loads(profiles[0]) if profiles else None
                    outcomes[implementation]['loopback_runner_sha256'] = hashlib.sha256((ROOT/'tests/gnu/loopback-profile.py').read_bytes()).hexdigest()
                if instrument and row.get('native_launcher'):
                    outcomes[implementation]['launcher_source_sha256'] = hashlib.sha256((ROOT/'tests/gnu/valgrind-launch.c').read_bytes()).hexdigest()
                if stdbuf_library:
                    outcomes[implementation]['stdbuf_library_sha256'] = hashlib.sha256(stdbuf_library.read_bytes()).hexdigest()
                if tmpdir_library:
                    outcomes[implementation]['tmpdir_adapter_source_sha256'] = hashlib.sha256((ROOT/'tests/gnu/valgrind-tmpdir.c').read_bytes()).hexdigest()
                if row.get('locale_profile') == 'extended':
                    outcomes[implementation]['locale_evidence_sha256'] = hashlib.sha256((ROOT/'evidence/test-locales.json').read_bytes()).hexdigest()
                if nss_profile:
                    outcomes[implementation]['nss'] = nss_profile
                    assert Path('/etc/nsswitch.conf').read_text() == original_nss, 'host NSS configuration changed'
                if row.get('perl_driver') == 'tty-eof':
                    commands_run = [line.decode() for line in re.findall(rb'^RBOXC_TTY_COMMAND (.+)$', completed.stderr, re.M)]
                    expected_commands = [command+' 2> tty-eof.err' for command in row['tty_commands']] * 2
                    versions = re.findall(rb'^RBOXC_EXPECT_VERSION (.+)$', completed.stderr, re.M)
                    outcomes[implementation]['expect_version'] = versions[0].decode() if len(versions) == 1 else None
                    outcomes[implementation]['tty_commands'] = commands_run
                    outcomes[implementation]['case_count'] = len(commands_run)
                    outcomes[implementation]['case_count_pass'] = (commands_run == expected_commands
                        and len(commands_run) == row['expected_case_count'] and len(versions) == 1)
                elif script.suffix == '.pl':
                    counts = re.findall(rb'^RBOXC_SELECTION (\d+) of \d+$', completed.stdout, re.M)
                    expected_count = row.get('expected_case_count', len(row.get('cases', [])))
                    outcomes[implementation]['case_count'] = sum(int(count) for count in counts)
                    outcomes[implementation]['case_count_pass'] = bool(counts) and outcomes[implementation]['case_count'] == expected_count
                if instrument and (credentials or row.get('native_launcher') or row.get('shared_runtime')):
                    for path in runtime_memory_dir.glob('*.log'):
                        shutil.copy2(path, memory_dir/path.name)
                if instrument:
                    memory = []
                    for path in sorted(memory_dir.glob('*.log')):
                        report = path.read_text(errors='backslashreplace')
                        memory.append({'log': str(path.relative_to(ROOT)),
                                       **parse_memory_log(report, path.stem, row.get('valgrind_log_fd', False))})
                    outcomes[implementation]['memory'] = memory
        passed = outcomes['gnu']['status'] == outcomes['rboxc']['status'] == 0
        passed &= all(result.get('case_count_pass', True) for result in outcomes.values())
        if instrument:
            memory = outcomes['rboxc']['memory']
            passed &= bool(memory) and all(m['errors'] == 0 and m['non_inherited_descriptors'] == 0
                                          and m.get('complete_exec_log', True) for m in memory)
        state = 'pass' if passed else 'skip' if outcomes['gnu']['status'] == outcomes['rboxc']['status'] == 77 else 'open'
        results_by_script[row['script']] = {**row, **outcomes, 'pass': passed, 'state': state}
        print(state.upper(), row['script'], {key: value['status'] for key, value in outcomes.items()}, flush=True)
        checkpoint()
    results = checkpoint()
    return any(not row['pass'] and not row['gnu']['status'] == row['rboxc']['status'] == 77 for row in results if included(row))


if __name__ == '__main__':
    raise SystemExit(main())
