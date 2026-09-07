#!/usr/bin/env python3
"""Check yes buffer ownership on bounded output-error and signal exits."""
# SPDX-License-Identifier: GPL-3.0-or-later
import hashlib
import importlib.util
import json
import os
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
profile = ComparisonProfile('yes-lifetime')
patterns = [('default', []), ('empty', ['']), ('small', ['x'])]
patterns += [(str(size), ['x' * size]) for size in (4095, 4096, 8191, 8192, 16384)]
patterns += [('two-small', ['alpha', 'beta']), ('two-large', ['x' * 8192, 'y' * 8192]),
             ('many', [str(i) for i in range(128)])]
cases = [(name, args, 'full') for name, args in patterns]
cases += [('small-signal', ['x'], 'closed-pipe'), ('large-signal', ['x' * 8192], 'closed-pipe')]
results = []
for index, (name, operands, sink) in enumerate(cases):
    observations = {}
    for implementation, binary in [('gnu', profile.oracle), ('rboxc', profile.binary)]:
        for instrument in (False, True):
            key = implementation + ('-valgrind' if instrument else '')
            command = [str(binary), '--coreutils-prog=yes' if implementation == 'gnu' else 'yes', *operands]
            log = profile.logs/f'{index:02}-{key}.log'
            if instrument:
                command = ['/usr/bin/valgrind', '--leak-check=full', '--show-leak-kinds=all',
                           '--track-fds=yes', '--log-file='+str(log), *command]
            with tempfile.TemporaryDirectory(prefix='rboxc-yes-lifetime-') as directory:
                if sink == 'full':
                    output = open('/dev/full', 'wb')
                else:
                    read_fd, write_fd = os.pipe()
                    os.close(read_fd)
                    output = os.fdopen(write_fd, 'wb')
                with output:
                    done = subprocess.run(command, stdin=subprocess.DEVNULL, stdout=output,
                        stderr=subprocess.PIPE, cwd=directory, timeout=30,
                        env={'PATH': '/usr/bin:/bin', 'LC_ALL': 'C', 'HOME': directory})
                row = {'status': done.returncode, 'stderr': done.stderr.decode(errors='replace')}
                if instrument:
                    row.update(memory=runner.parse_memory_log(log.read_text(), log.stem),
                               log=str(log.relative_to(ROOT)), log_sha256=fingerprint(log))
                observations[key] = row
    reference = observations['gnu']
    equivalent = all((r['status'], r['stderr']) == (reference['status'], reference['stderr'])
                     for r in observations.values())
    memory = observations['rboxc-valgrind']['memory']
    clean = (memory['errors'] == 0 and memory['non_inherited_descriptors'] == 0
             and not any(memory['heap_bytes'].get(kind, 0) for kind in
                         ('definitely lost', 'indirectly lost', 'possibly lost')))
    passed = equivalent and reference['status'] == (1 if sink == 'full' else -13)
    passed &= clean or sink == 'closed-pipe'
    results.append({'name': name, 'operand_bytes': [len(x) for x in operands], 'sink': sink,
        'pass': passed, 'equivalent': equivalent, 'memory_clean': clean,
        'assessment': 'ownership' if sink == 'full' else 'signal-equivalence',
        'outcomes': observations})
report = {'scope': 'Fatal write exits must release owned buffers; signal exits retain GNU semantics and separate memory findings',
    **profile.metadata(), 'sources': {p: fingerprint(ROOT/p) for p in
        ('scripts/cleanup.py', 'src/generated/applet_yes.rs', 'tests/yes-lifetime.py')},
    'passed': sum(r['pass'] for r in results), 'total': len(results), 'results': results}
profile.report.write_text(json.dumps(report, indent=2)+'\n')
print(json.dumps({k: report[k] for k in ('passed', 'total')}))
raise SystemExit(report['passed'] != report['total'])
