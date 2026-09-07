#!/usr/bin/env python3
"""Compare ordinary GNU Grep modes, output, aliases, and I/O errors."""
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
runner = importlib.util.module_from_spec(spec); spec.loader.exec_module(runner)
profile = ComparisonProfile('grep-behavior', oracle=ROOT/'build/gnu-grep/src/grep', selections=True)
commands = ('grep', 'egrep', 'fgrep')
oracles = {name: ROOT/f'build/gnu-grep/src/{name}' for name in commands}
hashes = {name: fingerprint(path) for name, path in oracles.items()}
fixture_text = b'alpha\nBeta 123\nalphabet\n\nomega 456\n'
cases = []
def case(name, args, command='grep', data=b'', locale='C', output='pipe'):
    cases.append((name, command, args, data, locale, output))
for name, args in [
    ('help', ['--help']), ('version', ['--version']), ('missing-operand', []),
    ('unknown-option', ['--unknown-option']), ('basic', ['alpha', 'input']),
    ('extended', ['-E', '^(alpha|Beta)', 'input']), ('fixed', ['-F', 'Beta 123', 'input']),
    ('perl', ['-P', r'\d+', 'input']), ('backreference', [r'\(a\)lph\1', 'input']),
    ('backreference-extended', ['-E', r'(a)lph\1', 'input']),
    ('whole-word', ['-w', 'alpha', 'input']), ('whole-line', ['-x', 'alpha', 'input']),
    ('only-match', ['-o', 'alpha', 'input']), ('count', ['-c', 'alpha', 'input']),
    ('quiet', ['-q', 'alpha', 'input']), ('quiet-missing', ['-q', 'alpha', 'missing', 'input']),
    ('list-matching', ['-l', 'alpha', 'input', 'other']), ('list-nonmatching', ['-L', 'alpha', 'input', 'other']),
    ('no-match', ['not-present', 'input']), ('invert', ['-v', 'alpha', 'input']),
    ('line-number', ['-n', '.', 'input']), ('byte-offset', ['-bo', 'alpha', 'input']),
    ('filename', ['-H', 'alpha', 'input']), ('no-filename', ['-h', 'alpha', 'input', 'other']),
    ('context', ['-C', '1', 'Beta', 'input']), ('max-count', ['-m', '1', '.', 'input']),
    ('ignore-case', ['-i', 'beta', 'input']), ('patterns-file', ['-f', 'patterns', 'input']),
    ('several-patterns', ['-e', 'alpha', '-e', 'omega', 'input']),
    ('empty-pattern', ['-e', '', 'input']), ('empty-pattern-file', ['-f', 'empty', 'input']),
    ('binary', ['alpha', 'binary']), ('binary-text', ['-a', 'alpha', 'binary']),
    ('null-lines', ['-z', 'alpha', 'binary']), ('color', ['--color=always', 'alpha', 'input']),
    ('recursive', ['-r', 'alpha', 'tree']), ('recursive-exclude', ['-r', '--exclude=skip', 'alpha', 'tree']),
    ('missing-file', ['alpha', 'missing']), ('invalid-regex', ['[', 'input']),
    ('conflicting-matchers', ['-EF', 'alpha', 'input']), ('full-output', ['.', 'input']),
]:
    case(name, args, output='full' if name == 'full-output' else 'pipe')
case('stdin', ['alpha'], data=fixture_text)
case('perl-unicode', ['-P', r'\p{L}+', 'unicode'], locale='C.UTF-8')
case('fixed-unicode-case', ['-Fi', 'ÉCOLE', 'unicode'], locale='C.UTF-8')
case('perl-only-match', ['-Po', r'\d+', 'input'])
case('perl-no-match', ['-P', r'\d{9}', 'input'])
for command in ('egrep', 'fgrep'):
    for suffix, args in [('help', ['--help']), ('version', ['--version']),
                         ('match', ['alpha', 'input']), ('missing', ['alpha', 'missing']),
                         ('option-error', ['--unknown-option'])]:
        case(command+'-'+suffix, args, command)
case('quiet-recursive', ['-rq', 'alpha', 'tree'])
case('egrep-full-output', ['.', 'input'], 'egrep', output='full')
case('fgrep-full-output', ['alpha', 'input'], 'fgrep', output='full')
case('invalid-regex-after-backreference', ['-e', r'\(a\)\1', '-e', '[', 'input'])
case('perl-invalid-regex', ['-P', '[', 'input'])
case('perl-several-patterns', ['-P', '-e', 'alpha', '-e', 'omega', 'input'])
case('perl-word', ['-Pw', 'alpha', 'input'])
case('perl-line', ['-Px', 'alpha', 'input'])
case('perl-full-output', ['-P', '.', 'input'], output='full')
case('perl-long-line', ['-Poc', r'\balpha\b', 'long'])
case('patterns-directory', ['-f', 'tree', 'input'])
case('several-pattern-files', ['-f', 'patterns', '-f', 'empty', 'input'])
selected = set(profile.options.commands)
assert selected <= {r[0] for r in cases}
results = []
for index, (name, command, args, data, locale, output) in enumerate(cases):
    if selected and name not in selected:
        continue
    outcomes = {}
    for implementation in ('gnu', 'rboxc'):
        for instrument in (False, True):
            key = implementation+('-valgrind' if instrument else '')
            with tempfile.TemporaryDirectory(prefix='rboxc-grep-behavior-') as directory:
                work = Path(directory); (work/'exec').mkdir(); (work/'memory').mkdir(); (work/'tree').mkdir()
                for entry in commands:
                    (work/'exec'/entry).symlink_to(oracles[entry] if implementation == 'gnu' else profile.binary)
                for label, contents in [('input',fixture_text), ('other',b'no selected line\n'),
                    ('patterns',b'alpha\nomega\n'), ('empty',b''), ('binary',b'alpha\0omega\0'),
                    ('unicode','école\nÉCOLE\n123\n'.encode()), ('tree/keep',fixture_text), ('tree/skip',fixture_text)]:
                    (work/label).write_bytes(contents)
                if name == 'perl-long-line':
                    (work/'long').write_bytes(b'x'*(256*1024)+b' alpha\n')
                env = {'PATH': str(work/'exec')+':/usr/bin:/bin', 'HOME': directory, 'TMPDIR': directory,
                       'LC_ALL': locale, 'LANGUAGE': 'C', 'TZ': 'UTC0'}
                invocation = [str(work/'exec'/command), *args]
                if instrument:
                    # GNU supplies these two aliases as shell scripts. Trace their
                    # actual interpreter and child grep; rboxc uses its Rust entries.
                    if implementation == 'gnu' and command != 'grep':
                        invocation = ['/bin/sh', *invocation]
                    invocation = ['/usr/bin/valgrind','--leak-check=full','--show-leak-kinds=all',
                        '--track-fds=yes','--trace-children=yes','--log-file='+str(work/'memory/%p.log'),*invocation]
                with open('/dev/full' if output == 'full' else os.devnull,'wb') as sink:
                    done = subprocess.run(invocation,cwd=work,env=env,input=data,
                        stdout=sink if output == 'full' else subprocess.PIPE,stderr=subprocess.PIPE,timeout=30)
                stdout = done.stdout or b''
                stderr = done.stderr.replace(directory.encode(),b'<fixture>')
                trace_prefix = instrument and implementation == 'gnu' and command != 'grep'
                if trace_prefix:
                    stderr = stderr.replace(b'<fixture>/exec/grep: ',b'grep: ')
                row = {'status':done.returncode,'stdout':stdout.replace(directory.encode(),b'<fixture>').hex(),
                       'stderr':stderr.hex(),'raw_stdout':stdout.hex(),'raw_stderr':done.stderr.hex(),'fixture':directory,
                       'traced_alias_prefix_normalization':trace_prefix}
                if instrument:
                    saved = profile.logs/f'{index:02}-{key}-memory';shutil.copytree(work/'memory',saved)
                    row['memory'] = [{**runner.parse_memory_log(p.read_text(),p.stem,exec_only=True),
                        'log':str(p.relative_to(ROOT)),'sha256':fingerprint(p)} for p in sorted(saved.glob('*.log'))]
                outcomes[key] = row
    def signature(row):return tuple(row[k] for k in ('status','stdout','stderr'))
    equivalent = all(signature(r)==signature(outcomes['gnu']) for r in outcomes.values())
    logs = outcomes['rboxc-valgrind']['memory']
    clean = bool(logs) and all(m['complete_exec_log'] and m['errors']==0 and m['non_inherited_descriptors']==0
        and not any(m['heap_bytes'].get(k,0) for k in ('definitely lost','indirectly lost','possibly lost')) for m in logs)
    results.append({'name':name,'command':command,'arguments':args,'stdin':data.hex(),'locale':locale,
        'stdout_target':output,'pass':equivalent and clean,'equivalent':equivalent,'memory_clean':clean,'outcomes':outcomes})
    print('PASS' if equivalent and clean else 'OPEN',name,flush=True)
    report = {'scope':'Ordinary bounded matching, formatting, aliases, and I/O errors; no historical vulnerability reproductions.',
        **profile.metadata(),'gnu_binaries':{n:{'path':str(p),'sha256':hashes[n]} for n,p in oracles.items()},
        'driver_sha256':fingerprint(Path(__file__)),
        'normalization':'Replace fixture directory; for traced GNU shell aliases only, record and remove the child grep executable directory from its diagnostic prefix. Raw streams are retained.',
        'passed':sum(r['pass'] for r in results),'total':len(results),'results':results}
    assert all(fingerprint(p)==hashes[n] for n,p in oracles.items())
    temp=profile.report.with_suffix('.tmp.json');temp.write_text(json.dumps(report,indent=2)+'\n');temp.replace(profile.report)
raise SystemExit(any(not r['pass'] for r in results))
