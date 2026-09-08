#!/usr/bin/env python3
"""Run Sed's original intrinsic-Valgrind ownership check without nesting tools."""
# SPDX-License-Identifier: GPL-3.0-or-later
import json
from pathlib import Path
import shutil
import subprocess
import tempfile
from comparison_profile import ComparisonProfile, fingerprint
from sed_dependencies import prepare
from valgrind_xml import parse_memory_xml

ROOT = Path(__file__).resolve().parents[1]
profile = ComparisonProfile('sed-intrinsic-original', oracle=ROOT/'build/gnu-sed/sed/sed')
source = Path(json.loads((ROOT/'inventory/sources.json').read_text())['sed']['source'])
script = source/'testsuite/bug80573.sh'
row = next(r for r in json.loads((ROOT/'inventory/sed-tests.json').read_text())['scripts'] if r['script'] == 'testsuite/bug80573.sh')
assert row['source_sha256'] == fingerprint(script)
dependencies = prepare()
helpers, prerequisite_environment, prerequisites = dependencies
helpers = {name: Path(item['path']) for name, item in helpers.items()}
helpers.update(true=ROOT/'build/gnu-coreutils/src/coreutils',
               grep=ROOT/'build/gnu-grep/src/grep', diff=ROOT/'build/gnu-diffutils/src/diff')
inputs = {p: fingerprint(p) for p in {script, source/'testsuite/init.sh', source/'init.cfg',
    Path(__file__), ROOT/'tests/valgrind_xml.py', ROOT/'tests/comparison_profile.py',
    Path('/usr/bin/valgrind'), Path('/bin/bash').resolve(), Path('/bin/sh').resolve(),
    *helpers.values()}}
outcomes = {}
for implementation, binary in (('gnu', profile.oracle), ('rboxc', profile.binary)):
    saved = profile.logs/implementation
    saved.mkdir()
    with tempfile.TemporaryDirectory(prefix='rboxc-sed-intrinsic-') as directory:
        work = Path(directory)
        for name in ('sed', 'deps', 'memory'):
            (work/name).mkdir()
        (work/'sed/sed').symlink_to(binary)
        for name, path in helpers.items():
            (work/'deps'/name).symlink_to(path)
        options = ['--xml=yes', '--xml-file='+str(work/'memory/%p.xml'),
                   '--log-file='+str(work/'memory/%p.log'), '--trace-children=yes',
                   '--child-silent-after-fork=yes',
                   '--track-fds=yes', '--leak-check=full', '--show-leak-kinds=all',
                   '--default-suppressions=no']
        env = {**prerequisite_environment, 'PATH':str(work/'sed')+':'+str(work/'deps')+':/usr/bin:/bin',
               'HOME':directory, 'TMPDIR':directory, 'LC_ALL':'C', 'LANGUAGE':'C', 'TZ':'UTC0',
               'srcdir':str(source), 'top_srcdir':str(source), 'abs_top_srcdir':str(source),
               'abs_top_builddir':directory, 'SHELL':'/bin/bash', 'VALGRIND_OPTS':' '.join(options)}
        done = subprocess.run(['/bin/bash', '-c', 'exec 9>&2; exec /bin/bash "$1"',
                               'sed-intrinsic-test', str(script)], cwd=work, env=env,
                              stdin=subprocess.DEVNULL, capture_output=True, timeout=90)
        (saved/'stdout').write_bytes(done.stdout)
        (saved/'stderr').write_bytes(done.stderr)
        shutil.copytree(work/'memory', saved/'memory')
        processes = []
        for path in sorted((saved/'memory').glob('*.xml')):
            parsed = parse_memory_xml(path.read_bytes())
            assert parsed['pid'] == path.stem
            executable = parsed['argv'][0]
            if executable == 'sed':
                role = 'sed-original'
                assert parsed['argv'][1:] == ['-e', 'x', '-e', 's/.*/echo hi/e', 'in']
            elif executable == 'true':
                role = 'valgrind-prerequisite'
                assert parsed['argv'] == ['true']
            elif executable == '/bin/sh':
                role = 'shell-child'
                assert parsed['argv'][1:] == ['-c', '--', 'echo hi']
            else:
                raise AssertionError('unclassified process: '+str(parsed['argv']))
            processes.append({'role':role, 'xml':str(path.relative_to(ROOT)),
                              'xml_sha256':fingerprint(path), **parsed})
        assert sorted(p['role'] for p in processes) == ['sed-original','shell-child','valgrind-prerequisite']
        outcomes[implementation] = {'status':done.returncode, 'stdout':done.stdout.hex(),
            'stderr':done.stderr.hex(), 'streams':{n:{'path':str((saved/n).relative_to(ROOT)),
            'sha256':fingerprint(saved/n)} for n in ('stdout','stderr')},
            'text_logs':{str(p.relative_to(ROOT)):fingerprint(p) for p in (saved/'memory').glob('*.log')},
            'memory':processes, 'memory_clean':all(p['memory_clean'] for p in processes),
            'valgrind_options':options}
    assert prepare() == dependencies
    assert all(fingerprint(p) == h for p, h in inputs.items())
equivalent = all(outcomes['gnu'][k] == outcomes['rboxc'][k] for k in ('status','stdout','stderr'))
passed = equivalent and outcomes['gnu']['status'] == 0 and outcomes['rboxc']['memory_clean']
report = {**profile.metadata(), 'scope':'Unchanged original bug80573.sh invokes its own Valgrind, including its quiet option. XML logging records complete final-exec process findings without nested instrumentation. Fork-before-exec output is silenced while executed children remain traced. This additional profile disables default suppressions. The original ASAN probe invokes Sed without Valgrind and is not included in the memory coverage count.',
          'script':'testsuite/bug80573.sh', 'source_sha256':row['source_sha256'],
          'inputs':{str(p):h for p,h in inputs.items()}, 'prerequisites':prerequisites,
          'driver_sha256':fingerprint(Path(__file__)), 'pass':passed, 'equivalent':equivalent,
          'passed':int(passed), 'total':1, 'outcomes':outcomes}
profile.report.write_text(json.dumps(report, indent=2)+'\n')
print('PASS' if passed else 'OPEN', report['script'])
raise SystemExit(not passed)
