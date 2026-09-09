#!/usr/bin/env python3
"""Run individually reviewed GNU Tar Autotest selections unchanged."""
# SPDX-License-Identifier: GPL-3.0-or-later
import importlib.util
from concurrent.futures import ThreadPoolExecutor
import json
import os
from pathlib import Path
import re
import shlex
import shutil
import subprocess
import sys
import tempfile
from comparison_profile import ComparisonProfile, fingerprint

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT/'tests/gnu'))
spec = importlib.util.spec_from_file_location('reviewed', ROOT/'tests/gnu/reviewed-original.py')
runner = importlib.util.module_from_spec(spec)
spec.loader.exec_module(runner)
profile = ComparisonProfile('tar-original', oracle=ROOT/'build/gnu-tar/src/tar', selections=True)
source = Path(json.loads((ROOT/'inventory/sources.json').read_text())['tar']['source'])
selections = {'version': (1, 'version.at'), 'append': (49, 'append.at'), 'exclude': (59, 'exclude.at')}
selections.update({
    'options02': (4, 'options02.at'),
    'opcomp01': (6, 'opcomp01.at'), 'opcomp02': (7, 'opcomp02.at'),
    'opcomp03': (8, 'opcomp03.at'), 'opcomp05': (10, 'opcomp05.at'),
    'positional01': (23, 'positional01.at'), 'positional02': (24, 'positional02.at'),
    'positional03': (25, 'positional03.at'),
})
selections.update({
    'add-file': (26, 'add-file.at'), 'xform02': (57, 'xform02.at'), 'xform03': (58, 'xform03.at'),
    'exclude01': (60, 'exclude01.at'), 'exclude02': (61, 'exclude02.at'),
    'exclude03': (62, 'exclude03.at'), 'exclude04': (63, 'exclude04.at'),
    'update': (183, 'update.at'),
})
selections.update({
    'T-mult': (27, 'T-mult.at'), 'T-nest': (28, 'T-nest.at'), 'T-cd': (32, 'T-cd.at'),
    'T-zfile': (36, 'T-zfile.at'), 'T-nonl': (37, 'T-nonl.at'),
})
selections.update({
    'delete02': (79, 'delete02.at'), 'delete03': (80, 'delete03.at'),
    'extrac01': (84, 'extrac01.at'), 'extrac02': (85, 'extrac02.at'),
    'extrac04': (87, 'extrac04.at'), 'extrac17': (100, 'extrac17.at'),
    'extrac18': (101, 'extrac18.at'), 'extrac19': (102, 'extrac19.at'),
    'extrac20': (103, 'extrac20.at'), 'extrac24': (107, 'extrac24.at'),
})
selections.update({
    'T-null': (34, 'T-null.at'), 'T-null2': (35, 'T-null2.at'),
    'T-dir00': (38, 'T-dir00.at'), 'T-dir01': (39, 'T-dir01.at'),
    'recurse': (43, 'recurse.at'), 'recurs02': (44, 'recurs02.at'),
    'shortrec': (45, 'shortrec.at'), 'same-order01': (47, 'same-order01.at'),
    'same-order02': (48, 'same-order02.at'), 'append03': (52, 'append03.at'),
    'xform-h': (55, 'xform-h.at'), 'xform01': (56, 'xform01.at'),
    'exclude07': (66, 'exclude07.at'), 'label01': (111, 'label01.at'),
    'label03': (113, 'label03.at'),
})
permission_selections = {
    'extrac06': (89, 'extrac06.at'), 'extrac07': (90, 'extrac07.at'),
    'extrac08': (91, 'extrac08.at'), 'extrac10': (93, 'extrac10.at'),
    'extrac12': (95, 'extrac12.at'), 'extrac15': (98, 'extrac15.at'),
    'extrac16': (99, 'extrac16.at'), 'extrac21': (104, 'extrac21.at'),
    'extrac22': (105, 'extrac22.at'), 'extrac23': (106, 'extrac23.at'),
}
selections.update(permission_selections)
selections.update({
    'checkpoint-defaults': (12, 'checkpoint/defaults.at'),
    'checkpoint-dot-compat': (15, 'checkpoint/dot-compat.at'),
    'checkpoint-dot-int': (16, 'checkpoint/dot-int.at'),
    'checkpoint-dot': (14, 'checkpoint/dot.at'),
    'checkpoint-interval': (13, 'checkpoint/interval.at'),
    'exclude08': (67, 'exclude08.at'),
    'exclude10': (69, 'exclude10.at'),
    'exclude12': (71, 'exclude12.at'),
    'indexfile': (40, 'indexfile.at'),
    'pipe': (2, 'pipe.at'),
    'verbose': (41, 'verbose.at'),
})
selections.update({
    'exclude09': (68, 'exclude09.at'), 'exclude11': (70, 'exclude11.at'),
    'exclude13': (72, 'exclude13.at'), 'exclude14': (73, 'exclude14.at'),
    'exclude15': (74, 'exclude15.at'), 'exclude16': (75, 'exclude16.at'),
})
selections.update({'label02': (112, 'label02.at'), 'owner': (167, 'owner.at')})
selections.update({
    'label04': (114, 'label04.at'), 'label05': (115, 'label05.at'),
    'onetop01': (234, 'onetop01.at'), 'onetop02': (235, 'onetop02.at'),
    'onetop03': (236, 'onetop03.at'), 'onetop04': (237, 'onetop04.at'),
})
selections.update({
    'T-recurse': (30, 'T-recurse.at'), 'T-recurse2': (31, 'T-recurse.at'),
    'append01': (50, 'append01.at'), 'append02': (51, 'append02.at'),
    'append04': (53, 'append04.at'), 'append05': (54, 'append05.at'),
    'delete01': (78, 'delete01.at'), 'delete04': (81, 'delete04.at'),
    'delete05': (82, 'delete05.at'),
    'backup01': (109, 'backup01.at'), 'difflink': (110, 'difflink.at'),
    'old': (154, 'old.at'), 'verify': (188, 'verify.at'),
})
selections.update({
    'incremental': (116, 'incremental.at'), 'incr01': (117, 'incr01.at'),
    'incr02': (118, 'incr02.at'), 'listed01': (119, 'listed01.at'),
    'listed02': (120, 'listed02.at'), 'listed03': (121, 'listed03.at'),
    'listed04': (122, 'listed04.at'), 'incr03': (124, 'incr03.at'),
    'incr05': (126, 'incr05.at'), 'incr06': (127, 'incr06.at'),
    'incr07': (128, 'incr07.at'), 'incr08': (129, 'incr08.at'),
    'incr09': (130, 'incr09.at'), 'incr10': (131, 'incr10.at'),
    'incr11': (132, 'incr11.at'), 'rename01': (137, 'rename01.at'),
    'rename02': (138, 'rename02.at'), 'rename03': (139, 'rename03.at'),
    'rename04': (140, 'rename04.at'), 'rename05': (141, 'rename05.at'),
    'rename06': (142, 'rename06.at'),
})
permission_selections['listed03'] = selections['listed03']
selected = profile.options.commands or list(selections)
assert set(selected) <= set(selections)
helpers = {n: ROOT/'build/gnu-coreutils/src/coreutils' for n in ('cat','rm','mkdir','chmod','touch','sort','echo','basename','cp','ln','true','false','sleep','ls','mv','mktemp','cut','id','date','printf','dd','rmdir','expr','tr','wc','head','tail','uname','cksum')}
helpers.update({n: ROOT/'build/gnu-diffutils/src'/n for n in ('cmp', 'diff')})
helpers['sed'] = ROOT/'build/gnu-sed/sed/sed'
helpers['grep'] = ROOT/'build/gnu-grep/src/grep'
helpers['genfile'] = ROOT/'build/gnu-tar/tests/genfile'
helpers['ckmtime'] = ROOT/'build/gnu-tar/tests/ckmtime'
helpers['checkseekhole'] = ROOT/'build/gnu-tar/tests/checkseekhole'
helpers['find'] = ROOT/'build/gnu-findutils/find/find'
inputs = {p: fingerprint(p) for p in {*helpers.values(), profile.oracle, source/'tests/testsuite', source/'tests/testsuite.at',
    ROOT/'build/gnu-tar/tests/atconfig', ROOT/'build/gnu-tar/tests/atlocal', Path('/bin/bash'), Path('/bin/sh').resolve(), Path('/usr/bin/awk').resolve(), Path(__file__)}}
for filename in ('genfile.c', 'argcv.c', 'argcv.h', 'ckmtime.c', 'checkseekhole.c', 'Makefile.am'):
    path = source/'tests'/filename
    inputs[path] = fingerprint(path)
manifest = json.loads((ROOT/'inventory/tar-tests.json').read_text())
registered = {row['path']: row for row in manifest['inputs']}
assert fingerprint(source/'tests/testsuite.at') == manifest['registration_sha256']
for name in selected:
    path = source/'tests'/selections[name][1]
    reviewed = registered[str(path.relative_to(source))]
    assert reviewed['reviewed'] and fingerprint(path) == reviewed['sha256']
    inputs[path] = fingerprint(path)
if 'owner' in selected:
    for filename in ('/etc/nsswitch.conf', '/etc/passwd', '/etc/group', '/usr/bin/unshare', '/usr/bin/mount'):
        path = Path(filename)
        inputs[path] = fingerprint(path)
def run_selection(name):
    number, filename = selections[name]
    unprivileged = name in permission_selections
    outcomes = {}
    for implementation in ('gnu', 'rboxc'):
        for instrument in (False, True):
            key = implementation + ('-valgrind' if instrument else '')
            saved = profile.logs/(name+'-'+key)
            saved.mkdir()
            with tempfile.TemporaryDirectory(prefix='rboxc-tar-original-') as directory:
                work = Path(directory)
                for sub in ('exec', 'real', 'deps', 'memory', 'copies'):
                    (work/sub).mkdir()
                copies = {}
                def executable(binary):
                    if not unprivileged:
                        return binary
                    if binary not in copies:
                        copied = work/'copies'/str(len(copies))
                        shutil.copy2(binary, copied)
                        expected = fingerprint(binary)
                        assert fingerprint(copied) == expected
                        copies[binary] = (copied, expected)
                    return copies[binary][0]
                for command, binary in helpers.items():
                    (work/'deps'/command).symlink_to(executable(binary))
                (work/'real/tar').symlink_to(executable(profile.oracle if implementation == 'gnu' else profile.binary))
                invocation = ['tar']
                if instrument:
                    invocation = ['/usr/bin/valgrind', '--leak-check=full', '--show-leak-kinds=all', '--track-fds=yes', '--trace-children=yes', '--log-file='+str(work/'memory/%p.log'), *invocation]
                path = str(work/'real')+':'+str(work/'deps')+':/usr/bin:/bin'
                wrapper = work/'exec/tar'
                wrapper.write_text('#!/bin/sh\nPATH='+shlex.quote(path)+'\nexport PATH\nexec '+shlex.join(invocation)+' "$@"\n')
                wrapper.chmod(0o755)
                config = (ROOT/'build/gnu-tar/tests/atconfig').read_text()
                config = config.replace(str(ROOT/'build/gnu-tar/tests'), directory).replace(str(ROOT/'build/gnu-tar'), directory)
                (work/'atconfig').write_text(config)
                shutil.copy2(ROOT/'build/gnu-tar/tests/atlocal', work/'atlocal')
                environment = {'PATH': str(work/'deps')+':/usr/bin:/bin', 'HOME': directory, 'TMPDIR': directory,
                               'LC_ALL': 'C', 'LANGUAGE': 'C', 'TZ': 'UTC0', 'CONFIG_SHELL': '/bin/bash'}
                command = ['/bin/bash', str(source/'tests/testsuite'), '--debug', str(number),
                           'AUTOTEST_PATH='+str(work/'exec')+':'+str(work/'deps')]
                nss = None
                if name == 'owner':
                    host_nss = Path('/etc/nsswitch.conf').read_text()
                    config = work/'nsswitch.conf'
                    config.write_text(re.sub(r'^(passwd|group|shadow|gshadow|initgroups):.*$',
                                             r'\1: files', host_nss, flags=re.M))
                    shutil.copy2(config, saved/'nsswitch.conf')
                    nss = {'profile': 'private-mount-local-files',
                           'host_sha256': inputs[Path('/etc/nsswitch.conf')],
                           'private_path': str((saved/'nsswitch.conf').relative_to(ROOT)),
                           'private_sha256': fingerprint(config)}
                    command = ['/usr/bin/unshare', '--mount', '--propagation', 'private',
                               '/bin/sh', '-c', '/usr/bin/mount --bind "$1" /etc/nsswitch.conf || exit 77; shift; exec "$@"',
                               'local-nss', str(config), *command]
                credentials = {}
                if unprivileged:
                    assert os.geteuid() == 0, 'permission profile needs private uid/gid setup'
                    for entry in [work, *work.rglob('*')]:
                        if not entry.is_symlink():
                            os.chown(entry, 65534, 65534)
                    credentials = {'user': 65534, 'group': 65534, 'extra_groups': []}
                done = subprocess.run(command, cwd=work, env=environment, stdin=subprocess.DEVNULL, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, timeout=300, **credentials)
                if nss:
                    assert Path('/etc/nsswitch.conf').read_text() == host_nss
                assert all(fingerprint(copied) == expected for copied, expected in copies.values()), 'private executable changed'
                (saved/'driver.log').write_bytes(done.stdout)
                for file in ('atconfig', 'atlocal', 'testsuite.log'):
                    if (work/file).exists():
                        shutil.copy2(work/file, saved/file)
                if (work/'testsuite.dir').exists():
                    shutil.copytree(work/'testsuite.dir', saved/'suite', symlinks=True)
                shutil.copytree(work/'memory', saved/'memory')
                output = done.stdout.decode(errors='replace')
                assertions = re.findall(r'^\s*(\d+):\s+.*?\s+(ok|FAILED|skipped|expected failure)(?: \([^\n]*\))?\s*$', output, re.M)
                passed = done.returncode == 0 and assertions == [(str(number), 'ok')]
                logs = [{**runner.parse_memory_log(p.read_text(), p.stem, exec_only=True), 'log': str(p.relative_to(ROOT)), 'sha256': fingerprint(p)} for p in sorted((saved/'memory').glob('*.log'))]
                clean = bool(logs) and all(m['complete_exec_log'] and m['errors'] == 0 and m['non_inherited_descriptors'] == 0 and not any(m['heap_bytes'].get(k, 0) for k in ('definitely lost','indirectly lost','possibly lost')) for m in logs)
                outcomes[key] = {'status': done.returncode, 'assertions': assertions, 'assertions_pass': passed,
                                 'nss': nss,
                                 'execution_uid': 65534 if unprivileged else os.geteuid(),
                                 'execution_gid': 65534 if unprivileged else os.getegid(),
                                 'copied_executables': [{'source': str(original), 'private_path': str(copied.relative_to(work)), 'sha256': expected} for original, (copied, expected) in copies.items()],
                                 'memory': logs, 'memory_clean': clean if instrument else None,
                                 'driver_log': str((saved/'driver.log').relative_to(ROOT)), 'driver_log_sha256': fingerprint(saved/'driver.log'),
                                 'private_atconfig_sha256': fingerprint(saved/'atconfig')}
    row = {'selection': name, 'autotest_number': number, 'source': 'tests/'+filename, 'source_sha256': inputs[source/'tests'/filename],
           'outcomes': outcomes, 'assertions_pass': all(r['assertions_pass'] for r in outcomes.values())}
    row['pass'] = row['assertions_pass'] and outcomes['rboxc-valgrind']['memory_clean']
    return row

results = []
with ThreadPoolExecutor(max_workers=4) as pool:
  for row in pool.map(run_selection, selected):
    results.append(row)
    assert all(fingerprint(p) == expected for p, expected in inputs.items())
    report = {'scope': 'Unchanged reviewed GNU Tar Autotest selections, including every archive format registered by each selected original. Native atlocal is copied unchanged; atconfig build paths point into private fixtures. AUTOTEST_PATH selects a wrapper preserving argv[0]=tar and instruments every Tar invocation with child tracing. Permission selections run as uid/gid 65534 with no supplementary groups using byte-verified private executable copies. Test helpers are native dependencies, not ports.',
              **profile.metadata(), 'inputs': {str(p): value for p, value in inputs.items()},
              'selected': selected, 'parallel_selections': 4, 'planned_total': len(selected), 'complete': len(results) == len(selected),
              'passed': sum(r['pass'] for r in results), 'total': len(results), 'results': results}
    profile.report.write_text(json.dumps(report, indent=2)+'\n')
    print('PASS' if row['pass'] else 'OPEN', row['selection'], flush=True)
raise SystemExit(any(not r['pass'] for r in results))
