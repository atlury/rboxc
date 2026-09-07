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
profile = ComparisonProfile('diffutils-original', oracle=ROOT/'build/gnu-diffutils/src/diff', selections=True)
pin = json.loads((ROOT/'inventory/sources.json').read_text())['diffutils']
source = Path(pin['source'])
manifest = json.loads((ROOT/'inventory/diffutils-tests.json').read_text())
assert fingerprint(source/manifest['registration']['path']) == manifest['registration']['sha256']
oracles = {name: ROOT/f'build/gnu-diffutils/src/{name}' for name in pin['commands']}
oracle_hashes = {name: fingerprint(path) for name, path in oracles.items()}
selected = set(profile.options.commands)
assert selected <= {Path(r['script']).name for r in manifest['scripts'] if r['reviewed']}
results = []
for index, row in enumerate(manifest['scripts']):
    script = source/row['script']
    assert fingerprint(script) == row['source_sha256']
    if not row['reviewed']:
        continue
    if selected and script.name not in selected:
        continue
    outcomes = {}
    for implementation in ('gnu', 'rboxc'):
        for instrument in (False, True):
            key = implementation + ('-valgrind' if instrument else '')
            with tempfile.TemporaryDirectory(prefix='rboxc-diffutils-original-') as directory:
                work = Path(directory)
                for sub in ('src', 'real', 'tests', 'memory'):
                    (work/sub).mkdir()
                for name in pin['commands']:
                    binary = oracles[name] if implementation == 'gnu' else profile.binary
                    (work/'real'/name).symlink_to(binary)
                    if instrument:
                        wrapper = work/'src'/name
                        wrapper.write_text('#!/bin/sh\nPATH='+str(work/'real')+':$PATH\nexport PATH\n'
                            'exec /usr/bin/valgrind --leak-check=full --show-leak-kinds=all '
                            '--track-fds=yes --trace-children=yes --log-file='+str(work/'memory/%p.log')+
                            ' '+name+' "$@"\n')
                        wrapper.chmod(0o755)
                    else:
                        (work/'src'/name).symlink_to(binary)
                done = subprocess.run(['/bin/bash', '-c', 'exec 9>&2; exec /bin/bash "$1"',
                                       'diffutils-test', str(script)], cwd=work/'tests',
                    stdin=subprocess.DEVNULL, capture_output=True, timeout=180,
                    env={'PATH': str(work/'src')+':/usr/bin:/bin', 'HOME': directory,
                         'TMPDIR': directory, 'LC_ALL': 'C', 'LANGUAGE': 'C', 'TZ': 'UTC0',
                         'srcdir': str(source/'tests'), 'top_srcdir': str(source),
                         'abs_top_srcdir': str(source), 'abs_srcdir': str(source/'tests'),
                         'abs_top_builddir': directory, 'VERSION': pin['version'],
                         'PACKAGE_BUGREPORT': 'bug-diffutils@gnu.org',
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
    expected_status = row.get('expected_status', 0)
    native_pass = outcomes['gnu']['status'] == outcomes['rboxc']['status'] == expected_status
    assertions_pass = native_pass and all(r['status'] == expected_status for r in outcomes.values())
    logs = outcomes['rboxc-valgrind']['memory']
    clean = bool(logs) and all(m['errors'] == 0 and m['non_inherited_descriptors'] == 0
        and not any(m['heap_bytes'].get(k, 0) for k in ('definitely lost', 'indirectly lost', 'possibly lost')) for m in logs)
    skipped = all(value['status'] == 77 for value in outcomes.values())
    expected_failure = bool(row.get('upstream_xfail')) and assertions_pass
    matched = assertions_pass and clean
    passed = matched and not expected_failure
    state = ('prerequisite_skip' if skipped else 'expected_failure_matches' if expected_failure and clean
             else 'passed' if passed else 'assertions_passed_memory_open' if assertions_pass
             else 'assertions_open')
    results.append({**row, 'pass': passed, 'native_pass': native_pass,
                    'assertions_pass': assertions_pass, 'memory_clean': clean, 'state': state,
                    'expected_failure': expected_failure, 'skipped': skipped, 'matched': matched, 'outcomes': outcomes})
    print(state.upper(), row['script'], flush=True)
    # Preserve each completed selection before starting the next one.
    report = {'scope': 'Individually reviewed original assertions, native and Valgrind; provider children use matching commands from the private PATH. Pending and excluded originals are not executed.',
              **profile.metadata(), 'gnu_binaries': {n: {'path': str(p), 'sha256': oracle_hashes[n]} for n, p in oracles.items()},
              'driver_sha256': fingerprint(Path(__file__)),
              'launch_profile': 'Valgrind locates the named command through a private PATH, preserving its initial argv[0]. Subsequent child execs remain traced.',
              'passed': sum(r['pass'] for r in results), 'native_passed': sum(r['native_pass'] and not r.get('upstream_xfail') for r in results),
              'assertions_passed': sum(r['assertions_pass'] and not r.get('upstream_xfail') for r in results), 'total': len(results),
              'reviewed_scripts': sum(r['reviewed'] for r in manifest['scripts']),
              'selected_scripts': sorted(selected),
              'expected_failures': sum(r['expected_failure'] for r in results),
              'prerequisite_skips': sum(r['skipped'] for r in results),
              'state_counts': {s: sum(r['state'] == s for r in results) for s in sorted({r['state'] for r in results})},
              'registered_original_scripts': len(manifest['scripts']),
              'remaining': [r for r in manifest['scripts'] if not r['reviewed']], 'results': results}
    assert all(fingerprint(p) == oracle_hashes[n] for n, p in oracles.items())
    temporary = profile.report.with_suffix('.tmp.json')
    temporary.write_text(json.dumps(report, indent=2)+'\n')
    temporary.replace(profile.report)
raise SystemExit(any(not r['pass'] for r in results))
