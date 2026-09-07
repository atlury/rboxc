#!/usr/bin/env python3
"""Compare individually reviewed GNU Diffutils original scripts."""
# SPDX-License-Identifier: GPL-3.0-or-later
import importlib.util
import json
from pathlib import Path
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
profile = ComparisonProfile('diffutils-original', oracle=ROOT/'build/gnu-diffutils/src/diff')
pin = json.loads((ROOT/'inventory/sources.json').read_text())['diffutils']
source = Path(pin['source'])
manifest = json.loads((ROOT/'inventory/diffutils-tests.json').read_text())
assert fingerprint(source/manifest['registration']['path']) == manifest['registration']['sha256']
oracles = {name: ROOT/f'build/gnu-diffutils/src/{name}' for name in pin['commands']}
oracle_hashes = {name: fingerprint(path) for name, path in oracles.items()}
launcher_source = ROOT/'tests/gnu/valgrind-launch.c'
launcher = profile.logs/'diffutils-valgrind-launch'
subprocess.run(['gcc', '-O2', '-Wall', '-Wextra', '-Werror', launcher_source, '-o', launcher], check=True)
results = []
for index, row in enumerate(manifest['scripts']):
    script = source/row['script']
    assert fingerprint(script) == row['source_sha256']
    if not row['reviewed']:
        continue
    outcomes = {}
    for implementation in ('gnu', 'rboxc'):
        for instrument in (False, True):
            key = implementation + ('-valgrind' if instrument else '')
            with tempfile.TemporaryDirectory(prefix='rboxc-diffutils-original-') as directory:
                work = Path(directory)
                for sub in ('src', 'real', 'tests', 'memory'):
                    (work/sub).mkdir()
                if instrument:
                    shutil.copy2(launcher, work/'src/.valgrind-launch')
                for name in pin['commands']:
                    binary = oracles[name] if implementation == 'gnu' else profile.binary
                    (work/'real'/name).symlink_to(binary)
                    (work/'src'/name).symlink_to('.valgrind-launch' if instrument else binary)
                done = subprocess.run(['/bin/bash', '-c', 'exec 9>&2; exec /bin/bash "$1"',
                                       'diffutils-test', str(script)], cwd=work/'tests',
                    stdin=subprocess.DEVNULL, capture_output=True, timeout=180,
                    env={'PATH': str(work/'src')+':/usr/bin:/bin', 'HOME': directory,
                         'TMPDIR': directory, 'LC_ALL': 'C', 'LANGUAGE': 'C', 'TZ': 'UTC0',
                         'srcdir': str(source/'tests'), 'top_srcdir': str(source),
                         'abs_top_srcdir': str(source), 'abs_srcdir': str(source/'tests'),
                         'abs_top_builddir': directory, 'VERSION': pin['version'],
                         'built_programs': ' '.join(pin['commands']), 'PERL': '/usr/bin/perl'})
                log = profile.logs/f'{index:02}-{key}.log'
                log.write_bytes(done.stdout+done.stderr)
                outcome = {'status': done.returncode, 'stdout': done.stdout.hex(),
                           'stderr': done.stderr.hex(), 'log': str(log.relative_to(ROOT)),
                           'log_sha256': fingerprint(log)}
                if instrument:
                    saved = profile.logs/f'{index:02}-{key}-memory'
                    shutil.copytree(work/'memory', saved)
                    outcome['memory'] = [{**runner.parse_memory_log(p.read_text(), p.stem),
                        'log': str(p.relative_to(ROOT)), 'sha256': fingerprint(p)} for p in sorted(saved.glob('*.log'))]
                outcomes[key] = outcome
    native_pass = outcomes['gnu']['status'] == outcomes['rboxc']['status'] == 0
    assertions_pass = native_pass and all(r['status'] == 0 for r in outcomes.values())
    logs = outcomes['rboxc-valgrind']['memory']
    clean = bool(logs) and all(m['errors'] == 0 and m['non_inherited_descriptors'] == 0
        and not any(m['heap_bytes'].get(k, 0) for k in ('definitely lost', 'indirectly lost', 'possibly lost')) for m in logs)
    passed = assertions_pass and clean
    results.append({**row, 'pass': passed, 'native_pass': native_pass,
                    'assertions_pass': assertions_pass, 'memory_clean': clean, 'outcomes': outcomes})
    print('PASS' if passed else 'OPEN', row['script'], flush=True)
    # Preserve each completed selection before starting the next one.
    report = {'scope': 'Individually reviewed original assertions, native and Valgrind; provider children use matching commands from the private PATH. Pending and excluded originals are not executed.',
              **profile.metadata(), 'gnu_binaries': {n: {'path': str(p), 'sha256': oracle_hashes[n]} for n, p in oracles.items()},
              'driver_sha256': fingerprint(Path(__file__)), 'launcher_source_sha256': fingerprint(launcher_source),
              'passed': sum(r['pass'] for r in results), 'native_passed': sum(r['native_pass'] for r in results),
              'assertions_passed': sum(r['assertions_pass'] for r in results), 'total': len(results),
              'reviewed_scripts': sum(r['reviewed'] for r in manifest['scripts']),
              'registered_original_scripts': len(manifest['scripts']),
              'remaining': [r for r in manifest['scripts'] if not r['reviewed']], 'results': results}
    assert all(fingerprint(p) == oracle_hashes[n] for n, p in oracles.items())
    temporary = profile.report.with_suffix('.tmp.json')
    temporary.write_text(json.dumps(report, indent=2)+'\n')
    temporary.replace(profile.report)
raise SystemExit(any(not r['pass'] for r in results))
