#!/usr/bin/env python3
"""Compare ordinary GNU Which lookup and shell-definition display behavior."""
# SPDX-License-Identifier: GPL-3.0-or-later
import importlib.util
import json
import os
from pathlib import Path
import pwd
import shutil
import subprocess
import sys
import tempfile
from comparison_profile import ComparisonProfile, fingerprint

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT/'tests/gnu'))
spec = importlib.util.spec_from_file_location('reviewed_original', ROOT/'tests/gnu/reviewed-original.py')
runner = importlib.util.module_from_spec(spec)
spec.loader.exec_module(runner)
profile = ComparisonProfile('which-behavior', oracle=ROOT/'build/gnu-which/which')
function = 'chosen ()\n{\n    printf hello;\n}\n'
cases = [
    ('help', ['--help'], '', 'default', False),
    ('version', ['--version'], '', 'default', False),
    ('short-version', ['-V'], '', 'default', False),
    ('no-command', [], '', 'default', False),
    ('unknown-option', ['--unknown-option', 'tool'], '', 'default', False),
    ('first-match', ['tool'], '', 'default', False),
    ('all-matches', ['--all', 'tool'], '', 'default', False),
    ('several-commands', ['tool', 'only-second'], '', 'default', False),
    ('missing', ['missing'], '', 'default', False),
    ('mixed-missing', ['missing', 'tool', 'also-missing'], '', 'default', False),
    ('non-executable', ['plain'], '', 'default', False),
    ('directory', ['directory'], '', 'default', False),
    ('symlink', ['link'], '', 'default', False),
    ('dangling-symlink', ['dangling'], '', 'default', False),
    ('relative-command', ['./local/tool'], '', 'default', False),
    ('relative-subdirectory', ['local/tool'], '', 'default', False),
    ('absolute-command', ['{root}/first/tool'], '', 'default', False),
    ('missing-absolute', ['{root}/first/missing'], '', 'default', False),
    ('relative-path', ['tool'], '', 'relative', False),
    ('skip-dot', ['--skip-dot', 'tool'], '', 'relative', False),
    ('show-dot', ['--show-dot', 'tool'], '', 'relative', False),
    ('empty-path-element', ['tool'], '', 'empty-element', False),
    ('empty-path', ['tool'], '', 'empty', False),
    ('unset-path', ['tool'], '', 'unset', False),
    ('tilde-path', ['tool'], '', 'tilde', False),
    ('skip-tilde', ['--skip-tilde', 'tool'], '', 'tilde', False),
    ('show-tilde-root', ['--show-tilde', 'tool'], '', 'home', False),
    ('show-tilde-user', ['--show-tilde', 'tool'], '', 'home', True),
    ('tty-only-pipe', ['--tty-only', '--show-dot', 'tool'], '', 'relative', False),
    ('alias', ['--read-alias', 'chosen'], "alias chosen='tool --flag'\n", 'default', False),
    ('alias-all', ['-ai', 'chosen'], "alias chosen='tool --flag'\n", 'default', False),
    ('skip-alias', ['--read-alias', '--skip-alias', 'tool'], "alias tool='only-second'\n", 'default', False),
    ('function', ['--read-functions', 'chosen'], function, 'default', False),
    ('function-all', ['-a', '--read-functions', 'tool'], function.replace('chosen', 'tool'), 'default', False),
    ('skip-functions', ['--read-functions', '--skip-functions', 'tool'], function.replace('chosen', 'tool'), 'default', False),
    ('zsh-function', ['--read-functions', 'chosen'], 'chosen () {\n    printf hello\n}\n', 'default', False),
    ('old-bash-function', ['--read-functions', 'chosen'], 'declare -fx '+function, 'default', False),
    ('alias-function', ['--read-alias', '--read-functions', 'aliasname'], "alias aliasname='chosen'\n"+function, 'default', False),
    ('alias-storage-growth', ['--read-alias', 'alias39'], ''.join(f"alias alias{i}='tool'\n" for i in range(40)), 'default', False),
    ('function-storage-growth', ['--read-functions', 'fn19'], ''.join(function.replace('chosen', f'fn{i}') for i in range(20)), 'default', False),
    ('function-lines-growth', ['--read-functions', 'chosen'], 'chosen ()\n{\n'+'    printf hello;\n'*40+'}\n', 'default', False),
    ('option-terminator', ['--', '-tool'], '', 'default', False),
    ('multicall-help', ['--help'], '', 'default', False),
    ('multicall-version', ['--version'], '', 'default', False),
    ('multicall-lookup', ['-a', 'tool'], '', 'default', False),
    ('multicall-alias', ['--read-alias', 'chosen'], "alias chosen='tool --flag'\n", 'default', False),
]
results = []
for index, (name, operands, input_text, path_kind, ordinary_user) in enumerate(cases):
    outcomes = {}
    for implementation, binary in [('gnu', profile.oracle), ('rboxc', profile.binary)]:
        for instrument in (False, True):
            key = implementation + ('-valgrind' if instrument else '')
            with tempfile.TemporaryDirectory(prefix='rboxc-which-behavior-') as directory:
                work = Path(directory)
                work.chmod(0o755)
                for sub in ('first', 'second', 'cwd/local', 'home/bin', 'exec'):
                    (work/sub).mkdir(parents=True, exist_ok=True)
                for file in ('first/tool', 'second/tool', 'second/only-second', 'cwd/tool',
                             'cwd/local/tool', 'home/bin/tool', 'first/-tool'):
                    p = work/file
                    p.write_text('fixture\n')
                    p.chmod(0o755)
                (work/'first/plain').write_text('not executable\n')
                (work/'first/directory').mkdir()
                (work/'first/link').symlink_to('tool')
                (work/'first/dangling').symlink_to('missing')
                # Copies allow the ordinary-user profile to traverse its own
                # fixture without opening access to the root-owned workspace.
                executable = work/'exec'/('which' if implementation == 'gnu' else 'rboxc')
                shutil.copy2(binary, executable)
                expected_hash = profile.oracle_sha256 if implementation == 'gnu' else profile.binary_sha256
                assert fingerprint(executable) == expected_hash
                executable.chmod(0o755)
                command = ['which', *[x.replace('{root}', directory) for x in operands]]
                alias = work/'exec/which'
                if implementation == 'rboxc':
                    alias.symlink_to(executable.name)
                env = {'LC_ALL': 'C', 'LANGUAGE': 'C', 'HOME': str(work/'home'), 'PWD': str(work/'cwd')}
                paths = {'default': f'{work}/first:{work}/second', 'relative': f'./local:{work}/first',
                         'empty-element': f':{work}/first', 'empty': '', 'tilde': f'~/bin:{work}/first',
                         'home': f'{work}/home/bin:{work}/first'}
                if path_kind != 'unset':
                    env['PATH'] = paths[path_kind]
                if name.startswith('multicall-'):
                    env['PATH'] = str(work/'exec')+':'+env['PATH']
                    command = (['which', *operands] if implementation == 'gnu' else
                               [str(executable), 'which', *operands])
                else:
                    command[0] = str(alias)
                log = work/'memory.log'
                if instrument:
                    command = ['/usr/bin/valgrind', '--leak-check=full', '--show-leak-kinds=all',
                               '--track-fds=yes', '--log-file='+str(log), *command]
                identity = None
                if ordinary_user:
                    account = pwd.getpwnam('nobody')
                    os.chown(work, account.pw_uid, account.pw_gid)
                    def identity():
                        os.setgroups([])
                        os.setgid(account.pw_gid)
                        os.setuid(account.pw_uid)
                done = subprocess.run(command, cwd=work/'cwd', input=input_text.encode(),
                                      capture_output=True, timeout=30, env=env, preexec_fn=identity)
                row = {'status': done.returncode, 'raw_stdout': done.stdout.hex(), 'raw_stderr': done.stderr.hex(),
                       'fixture': directory, 'normalization': 'Replace only the private fixture directory with <fixture>.',
                       'stdout': done.stdout.replace(directory.encode(), b'<fixture>').hex(),
                       'stderr': done.stderr.replace(directory.encode(), b'<fixture>').hex()}
                if instrument:
                    saved = profile.logs/f'{index:02}-{key}.log'
                    shutil.copy2(log, saved)
                    row.update(memory=runner.parse_memory_log(saved.read_text(), saved.stem),
                               log=str(saved.relative_to(ROOT)), log_sha256=fingerprint(saved))
                outcomes[key] = row
    reference = outcomes['gnu']
    equivalent = all((r['status'], r['stdout'], r['stderr']) ==
                     (reference['status'], reference['stdout'], reference['stderr']) for r in outcomes.values())
    memory = outcomes['rboxc-valgrind']['memory']
    clean = memory['errors'] == 0 and memory['non_inherited_descriptors'] == 0 and not any(
        memory['heap_bytes'].get(k, 0) for k in ('definitely lost', 'indirectly lost', 'possibly lost'))
    results.append({'name': name, 'arguments': operands, 'input': input_text, 'path_profile': path_kind,
                    'ordinary_user': ordinary_user, 'pass': equivalent and clean,
                    'equivalent': equivalent, 'memory_clean': clean, 'outcomes': outcomes})
    print('PASS' if equivalent and clean else 'OPEN', name, flush=True)
report = {'scope': 'Bounded native/Valgrind comparisons of executable lookup and display of ordinary alias/function definitions. Source distribution registers no runtime test suite.',
          **profile.metadata(), 'gnu_binary': str(profile.oracle), 'driver_sha256': fingerprint(Path(__file__)),
          'passed': sum(r['pass'] for r in results), 'total': len(results), 'results': results}
profile.report.write_text(json.dumps(report, indent=2)+'\n')
raise SystemExit(report['passed'] != report['total'])
