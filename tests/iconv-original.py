#!/usr/bin/env python3
"""Run the unchanged GNU iconv buffering recipes in private directories."""
# SPDX-License-Identifier: GPL-3.0-or-later
import importlib.util
import json
from pathlib import Path
import re
import shlex
import subprocess
import sys
import tempfile
from comparison_profile import ComparisonProfile, fingerprint

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT/'tests/gnu'))
spec = importlib.util.spec_from_file_location('reviewed', ROOT/'tests/gnu/reviewed-original.py')
runner = importlib.util.module_from_spec(spec)
spec.loader.exec_module(runner)
profile = ComparisonProfile('iconv-original', selections=True,
                            oracle=ROOT/'build/gnu-glibc/iconv/iconv_prog')
source = Path(json.loads((ROOT/'inventory/sources.json').read_text())['glibc']['source'])
script = source/'iconv/tst-iconv_prog-buffer.sh'
manifest = ROOT/'inventory/glibc-utility-tests.json'
row = next(r for r in json.loads(manifest.read_text())['tests'] if r['path'] == str(script.relative_to(source)))
assert row['state'] == 'reviewed-ready'
assert fingerprint(script) == row['source_sha256']
inputs = {p: fingerprint(p) for p in [Path(__file__), manifest, script,
          source/'iconv/Makefile', Path('/bin/bash'), ROOT/'tests/gnu/reviewed-original.py']}
variants = {'default': ['', '0'], 'tiny': ['--buffer-size=1', '0'], 'large': ['', '22']}
selected = profile.options.commands or list(variants)
assert selected and len(set(selected)) == len(selected) and set(selected) <= variants.keys()
results = []
for variant in selected:
    outcomes = {}
    for implementation, binary in [('gnu', profile.oracle), ('rboxc', profile.binary)]:
        for instrument in (False, True):
            key = implementation+('-valgrind' if instrument else '')
            logs = profile.logs/(variant+'-'+key)
            logs.mkdir()
            with tempfile.TemporaryDirectory(prefix='rboxc-iconv-original-') as directory:
                work = Path(directory)
                (work/'exec').mkdir()
                alias = work/'exec/iconv'
                alias.symlink_to(binary)
                command = [str(alias)]
                if instrument:
                    command = ['/usr/bin/valgrind', '--leak-check=full', '--show-leak-kinds=all',
                               '--track-fds=yes', '--log-file='+str(logs/'process-%p.log'), *command]
                wrapper = work/'run-iconv'
                # GNU's prefix receives the build-tree executable as argv[1].
                # Substitute only that executable with the corresponding alias.
                wrapper.write_text('#!/bin/sh\nshift\nexec '+shlex.join(command)+' "$@"\n')
                wrapper.chmod(0o755)
                argv = ['/bin/bash', str(script), directory, str(wrapper), *variants[variant]]
                done = subprocess.run(argv, cwd=work, stdin=subprocess.DEVNULL, capture_output=True,
                    env={'PATH': '/usr/bin:/bin', 'LC_ALL': 'C', 'LANGUAGE': 'C',
                         'HOME': directory, 'TMPDIR': directory}, timeout=1800)
                (logs/'stdout').write_bytes(done.stdout)
                (logs/'stderr').write_bytes(done.stderr)
                calls = len(re.findall(rb'^.*tst-iconv_prog-buffer\.sh:[0-9]+: iconv ', done.stdout, re.M))
                memory = []
                for path in sorted(logs.glob('process-*.log')):
                    text = path.read_text()
                    pids = set(re.findall(r'^==([0-9]+)==', text, re.M))
                    assert len(pids) == 1
                    parsed = runner.parse_memory_log(text, pids.pop(), exec_only=True)
                    clean = (parsed['complete_exec_log'] and parsed['errors'] == 0
                             and parsed['non_inherited_descriptors'] == 0
                             and not any(parsed['heap_bytes'].get(k, 0) for k in
                                         ('definitely lost', 'indirectly lost', 'possibly lost')))
                    memory.append({'log': str(path.relative_to(ROOT)), 'sha256': fingerprint(path),
                                   **parsed, 'clean': clean})
                assert not instrument or len(memory) == calls
                passed = done.returncode == 0 and calls > 0
                if implementation == 'rboxc' and instrument:
                    passed = passed and all(r['clean'] for r in memory)
                if outcomes:
                    passed = passed and calls == outcomes['gnu']['calls']
                outcomes[key] = {'status': done.returncode, 'calls': calls, 'memory': memory,
                    'raw': {str(p.relative_to(ROOT)): fingerprint(p) for p in (logs/'stdout', logs/'stderr')},
                    'pass': passed}
                assert all(fingerprint(p) == h for p, h in inputs.items())
                print(variant, key, calls, 'calls', 'PASS' if passed else 'OPEN', flush=True)
    results.append({'variant': variant, 'arguments': variants[variant],
                    'pass': all(r['pass'] for r in outcomes.values()), 'outcomes': outcomes})
    report = {**profile.metadata(), 'inputs': {str(p): h for p, h in inputs.items()},
        'scope': 'Unchanged GNU glibc 2.43 iconv buffer test with original default, tiny and large recipe arguments. Each selected recipe retains its own output, permission and exit-status checks. Native GNU and candidate run normally and under Valgrind; every candidate invocation requires a complete clean heap/descriptor log. The script runner and fixture utilities are host tools. Conversion-module and other glibc tests remain separate.',
        'complete': len(results) == len(selected), 'planned_total': len(selected),
        'passed': sum(r['pass'] for r in results), 'total': len(results), 'results': results}
    profile.report.write_text(json.dumps(report, indent=2)+'\n')
raise SystemExit(report['passed'] != report['total'])
