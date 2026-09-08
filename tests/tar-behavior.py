#!/usr/bin/env python3
"""Compare GNU Tar operations on ordinary archives created in private fixtures."""
# SPDX-License-Identifier: GPL-3.0-or-later
import importlib.util
import json
import os
from pathlib import Path
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
profile = ComparisonProfile('tar-behavior', oracle=ROOT/'build/gnu-tar/src/tar', selections=True)
cases = [
    ('help', ['--help']), ('version', ['--version']),
    ('unknown-option', ['--not-an-option']), ('missing-archive', ['-tf', 'absent.tar']),
    ('no-operation', []), ('missing-operand', ['-cf']),
    ('list', ['-tf', 'archive.tar']), ('verbose-list', ['-tvf', 'archive.tar']),
    ('list-selected', ['-tf', 'archive.tar', './nested/beta']),
    ('list-wildcard', ['-tf', 'archive.tar', '--wildcards', '*.txt']),
    ('list-missing-member', ['-tf', 'archive.tar', 'absent']),
    ('extract', ['-xf', 'archive.tar', '-C', 'out']),
    ('extract-stdout', ['-xOf', 'archive.tar', './alpha.txt']),
    ('extract-selected', ['-xf', 'archive.tar', '-C', 'out', './nested/beta']),
    ('extract-strip', ['-xf', 'archive.tar', '-C', 'out', '--strip-components=2', './nested/beta']),
    ('extract-transform', ['-xf', 'archive.tar', '-C', 'out', '--transform=s/alpha/renamed/']),
    ('extract-exclude', ['-xf', 'archive.tar', '-C', 'out', '--exclude=alpha.txt']),
    ('compare-equal', ['-df', 'archive.tar', '-C', 'tree']),
    ('compare-changed', ['-df', 'archive.tar', '-C', 'tree']),
    ('create-ustar', ['--format=ustar', '-cf', 'result.tar', '-C', 'tree', '.']),
    ('create-gnu', ['--format=gnu', '-cf', 'result.tar', '-C', 'tree', '.']),
    ('create-pax', ['--format=pax', '--pax-option=delete=atime,delete=ctime,exthdr.name=%d/PaxHeaders/%f', '-cf', 'result.tar', '-C', 'tree', '.']),
    ('create-exclude', ['--format=ustar', '-cf', 'result.tar', '--exclude=alpha.txt', '-C', 'tree', '.']),
    ('append', ['-rf', 'archive.tar', '-C', 'tree', 'new.txt']),
    ('update', ['-uf', 'archive.tar', '-C', 'tree', './alpha.txt']),
    ('delete', ['--delete', '-f', 'archive.tar', './alpha.txt']),
    ('concatenate', ['-Af', 'archive.tar', 'second.tar']),
    ('help-full', ['--help']), ('version-full', ['--version']),
    ('list-full', ['-tf', 'archive.tar']),
]
cases += [
    ('old-create', ['cf', 'result.tar', '-C', 'tree', '.']),
    ('old-list', ['tvf', 'archive.tar']),
    ('old-missing-operand', ['f']),
    ('env-format', ['-cf', 'result.tar', '-C', 'tree', '.']),
    ('env-empty', ['-tf', 'archive.tar']),
    ('env-unclosed-quote', ['-tf', 'archive.tar']),
    ('env-old-create', ['cf', 'result.tar', '-C', 'tree', '.']),
    ('env-cli-override', ['--format=gnu', '-cf', 'result.tar', '-C', 'tree', '.']),
    ('same-order', ['--same-order', '-tf', 'archive.tar', './alpha.txt', './nested/beta']),
    ('same-order-missing', ['--same-order', '-tf', 'archive.tar', './alpha.txt', 'absent']),
    ('starting-file', ['--starting-file=./alpha.txt', '-tf', 'archive.tar']),
    ('starting-file-repeated', ['--starting-file=./empty', '--starting-file=./alpha.txt', '-tf', 'archive.tar']),
    ('directory-dot', ['--format=ustar', '-cf', 'result.tar', '-C', './tree', '-C', './', '.']),
    ('directory-nested', ['--format=ustar', '-cf', 'result.tar', '-C', 'tree', '-C', './nested', 'beta']),
]
case_environment = {
    'env-format': {'TAR_OPTIONS': '-H ustar'},
    'env-empty': {'TAR_OPTIONS': ''},
    'env-unclosed-quote': {'TAR_OPTIONS': "--exclude='alpha"},
    'env-old-create': {'TAR_OPTIONS': '-H ustar'},
    'env-cli-override': {'TAR_OPTIONS': '-H ustar'},
}
selected = set(profile.options.commands)
assert selected <= {name for name, _ in cases}
driver_sha256 = fingerprint(Path(__file__))
results = []
for index, (name, arguments) in enumerate(cases):
    if selected and name not in selected:
        continue
    outcomes = {}
    for implementation in ('gnu', 'rboxc'):
        for instrument in (False, True):
            key = implementation + ('-valgrind' if instrument else '')
            with tempfile.TemporaryDirectory(prefix='rboxc-tar-behavior-') as directory:
                work = Path(directory)
                for sub in ('exec', 'memory', 'tree', 'tree/nested', 'out'):
                    (work/sub).mkdir()
                (work/'exec/tar').symlink_to(profile.oracle if implementation == 'gnu' else profile.binary)
                (work/'tree/alpha.txt').write_bytes(b'alpha one\nalpha two\n')
                (work/'tree/nested/beta').write_bytes(b'beta\x00data\n')
                (work/'tree/empty').write_bytes(b'')
                (work/'tree/link').symlink_to('alpha.txt')
                os.link(work/'tree/alpha.txt', work/'tree/hardlink')
                for path in [work/'tree', *(work/'tree').rglob('*')]:
                    if not path.is_symlink():
                        path.chmod(0o750 if path.is_dir() else 0o640)
                    os.utime(path, (946684800, 946684800), follow_symlinks=False)
                environment = {'PATH': '/usr/bin:/bin', 'HOME': directory, 'LC_ALL': 'C', 'LANGUAGE': 'C', 'TZ': 'UTC0'}
                subprocess.run([str(profile.oracle), '--format=ustar', '-cf', 'archive.tar', '-C', 'tree', '.'], cwd=work, env=environment, capture_output=True, check=True, timeout=30)
                if name in ('append', 'concatenate'):
                    (work/'tree/new.txt').write_bytes(b'new member\n')
                    os.utime(work/'tree/new.txt', (946684801, 946684801))
                if name == 'concatenate':
                    subprocess.run([str(profile.oracle), '--format=ustar', '-cf', 'second.tar', '-C', 'tree', 'new.txt'], cwd=work, env=environment, capture_output=True, check=True, timeout=30)
                if name in ('update', 'compare-changed'):
                    (work/'tree/alpha.txt').write_bytes(b'changed alpha\n')
                    os.utime(work/'tree/alpha.txt', (946684802, 946684802))
                environment.update(case_environment.get(name, {}))
                invocation = [str(work/'exec/tar'), *arguments]
                if instrument:
                    invocation = ['/usr/bin/valgrind', '--leak-check=full', '--show-leak-kinds=all', '--track-fds=yes', '--trace-children=yes', '--log-file='+str(work/'memory/%p.log'), *invocation]
                with open('/dev/full' if name.endswith('-full') else os.devnull, 'wb') as sink:
                    done = subprocess.run(invocation, cwd=work, env=environment, stdin=subprocess.DEVNULL, stdout=sink if name.endswith('-full') else subprocess.PIPE, stderr=subprocess.PIPE, timeout=60)
                tree = {}
                links = {}
                for path in sorted(work.rglob('*')):
                    relative = str(path.relative_to(work))
                    if relative.split('/')[0] in ('exec', 'memory'):
                        continue
                    st = path.lstat()
                    if path.is_symlink():
                        tree[relative] = {'symlink': str(path.readlink())}
                    elif path.is_file():
                        inode = (st.st_dev, st.st_ino)
                        links.setdefault(inode, relative)
                        tree[relative] = {'sha256': fingerprint(path), 'bytes': st.st_size, 'mode': st.st_mode & 0o777, 'hardlink_group': links[inode]}
                    elif path.is_dir():
                        tree[relative] = {'directory_mode': st.st_mode & 0o777}
                normalize = lambda value: value.replace(directory.encode(), b'<fixture>').hex()
                row = {'status': done.returncode, 'stdout': normalize(done.stdout or b''), 'stderr': normalize(done.stderr), 'tree': tree,
                       'raw_stdout': (done.stdout or b'').hex(), 'raw_stderr': done.stderr.hex()}
                if instrument:
                    saved = profile.logs/f'{index:02}-{key}-memory'
                    shutil.copytree(work/'memory', saved)
                    row['memory'] = [{**runner.parse_memory_log(p.read_text(), p.stem, exec_only=True), 'log': str(p.relative_to(ROOT)), 'sha256': fingerprint(p)} for p in sorted(saved.glob('*.log'))]
                outcomes[key] = row
    equivalent = all(all(row[field] == outcomes['gnu'][field] for field in ('status', 'stdout', 'stderr', 'tree')) for row in outcomes.values())
    logs = outcomes['rboxc-valgrind']['memory']
    clean = bool(logs) and all(m['complete_exec_log'] and m['errors'] == 0 and m['non_inherited_descriptors'] == 0 and not any(m['heap_bytes'].get(k, 0) for k in ('definitely lost', 'indirectly lost', 'possibly lost')) for m in logs)
    results.append({'name': name, 'command': 'tar', 'arguments': arguments, 'environment': case_environment.get(name, {}), 'pass': equivalent and clean, 'equivalent': equivalent, 'memory_clean': clean, 'outcomes': outcomes})
    assert fingerprint(Path(__file__)) == driver_sha256
    report = {'scope': 'Ordinary private archives generated by the pinned native GNU Tar oracle; create, list, extract, compare, append, update, delete, and option/output errors. No untrusted archive inputs.',
              **profile.metadata(), 'candidate_is_native_oracle': profile.binary == profile.oracle.resolve(), 'driver_sha256': driver_sha256,
              'normalization': 'Replace the private fixture path in diagnostic/output bytes. Compare archive/file bytes, modes, symlinks, and hardlink groups. Creation source mtimes are fixed; no comparison of newly created filesystem mtimes.',
              'passed': sum(r['pass'] for r in results), 'total': len(results), 'results': results}
    profile.report.write_text(json.dumps(report, indent=2)+'\n')
    print('PASS' if results[-1]['pass'] else 'OPEN', name, flush=True)
raise SystemExit(any(not r['pass'] for r in results))
