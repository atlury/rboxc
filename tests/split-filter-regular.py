#!/usr/bin/env python3
"""Compare regular-file split filters that stop after one byte per chunk."""
# SPDX-License-Identifier: GPL-3.0-or-later
import importlib.util
import json
from pathlib import Path
import subprocess
import sys
import tempfile
from comparison_profile import ComparisonProfile, fingerprint

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT/'tests/gnu'))
spec = importlib.util.spec_from_file_location('reviewed_original', ROOT/'tests/gnu/reviewed-original.py')
runner = importlib.util.module_from_spec(spec)
spec.loader.exec_module(runner)
profile = ComparisonProfile('split-filter-regular')
shell = Path('/bin/sh').resolve()
head = ROOT/'build/gnu-coreutils/src/coreutils'
helper_hashes = {str(p): fingerprint(p) for p in (shell, head)}
results = []
for chunks in (1, 2, 3):
    observations = {}
    expected = {f'part{chr(97+i//26)}{chr(97+i%26)}.n': bytes([65+i]).hex()
                for i in range(chunks)}
    for implementation, binary in [('gnu', profile.oracle), ('rboxc', profile.binary)]:
        for instrument in (False, True):
            key = implementation + ('-valgrind' if instrument else '')
            with tempfile.TemporaryDirectory(prefix='rboxc-split-filter-') as directory:
                work = Path(directory)
                (work/'bin').mkdir()
                (work/'bin/head').symlink_to(head)
                # A bounded sparse input places a distinct byte at each boundary.
                chunk_bytes = 1024 * 1024 + 1
                with (work/'input').open('wb') as stream:
                    for i in range(chunks):
                        stream.seek(i * chunk_bytes)
                        stream.write(bytes([65+i]))
                    stream.truncate(chunks * chunk_bytes)
                command = [str(binary), '--coreutils-prog=split' if implementation == 'gnu' else 'split',
                           '-n', str(chunks), '--filter=head -c1 >"$FILE.n"', 'input', 'part']
                logs = profile.logs/f'{chunks}-{key}'
                if instrument:
                    logs.mkdir()
                    command = ['/usr/bin/valgrind', '--leak-check=full', '--show-leak-kinds=all',
                               '--track-fds=yes', '--trace-children=yes',
                               '--log-file='+str(logs/'%p.log'), *command]
                done = subprocess.run(command, cwd=work, stdin=subprocess.DEVNULL,
                    capture_output=True, timeout=30,
                    env={'PATH': str(work/'bin')+':/usr/bin:/bin', 'LC_ALL': 'C',
                         'HOME': directory, 'SHELL': str(shell)})
                row = {'status': done.returncode, 'stdout': done.stdout.hex(),
                       'stderr': done.stderr.decode(errors='replace'),
                       'outputs': {p.name: p.read_bytes().hex() for p in sorted(work.glob('part*'))}}
                if instrument:
                    row['memory'] = []
                    for log in sorted(logs.glob('*.log')):
                        text = log.read_text()
                        row['memory'].append({**runner.parse_memory_log(text, log.stem),
                            'log': str(log.relative_to(ROOT)), 'sha256': fingerprint(log),
                            'commands': [line.split('Command: ', 1)[1] for line in text.splitlines()
                                         if '== Command: ' in line]})
                observations[key] = row
    equivalent = all(r['status'] == 0 and r['stdout'] == '' and r['stderr'] == ''
                     and r['outputs'] == expected for r in observations.values())
    candidate_logs = observations['rboxc-valgrind']['memory']
    clean = bool(candidate_logs) and all(m['errors'] == 0 and m['non_inherited_descriptors'] == 0
        and not any(m['heap_bytes'].get(k, 0) for k in
                    ('definitely lost', 'indirectly lost', 'possibly lost')) for m in candidate_logs)
    results.append({'chunks': chunks, 'input_bytes': chunks * chunk_bytes,
                    'expected': expected, 'behavior_pass': equivalent,
                    'memory_clean': clean, 'outcomes': observations})
for path, expected_hash in helper_hashes.items():
    assert fingerprint(Path(path)) == expected_hash, 'helper changed during comparison'
report = {'scope': 'Supplemental bounded regular-file coverage; does not replace the unchanged original split/filter.sh result. Child memory findings remain separate from functional equivalence.',
          **profile.metadata(), 'driver_sha256': fingerprint(Path(__file__)),
          'helpers': helper_hashes, 'behavior_passed': sum(r['behavior_pass'] for r in results),
          'memory_clean': sum(r['memory_clean'] for r in results), 'total': len(results), 'results': results}
profile.report.write_text(json.dumps(report, indent=2)+'\n')
print(json.dumps({k: report[k] for k in ('behavior_passed', 'memory_clean', 'total')}))
raise SystemExit(report['behavior_passed'] != report['total'])
