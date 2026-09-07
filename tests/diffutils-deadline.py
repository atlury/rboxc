#!/usr/bin/env python3
"""Measure cmp's original sparse-size deadline without changing its assertion."""
# SPDX-License-Identifier: GPL-3.0-or-later
import json
from pathlib import Path
import subprocess
import tempfile
import time
from comparison_profile import ComparisonProfile, fingerprint

ROOT = Path(__file__).resolve().parents[1]
profile = ComparisonProfile('diffutils-deadline', oracle=ROOT/'build/gnu-diffutils/src/cmp')
results = []
with tempfile.TemporaryDirectory(prefix='rboxc-cmp-deadline-') as directory:
    work = Path(directory)
    for name, size in [('a', 14 << 40), ('b', 15 << 40)]:
        with (work/name).open('wb') as file:
            file.truncate(size)
    assert all((work/n).stat().st_blocks == 0 for n in ('a', 'b'))
    for implementation, binary in [('gnu', profile.oracle), ('rboxc', profile.binary)]:
        (work/'cmp').symlink_to(binary)
        for instrument, deadline in [(False, '0.4'), (True, '0.4'), (True, '5')]:
            command = [str(work/'cmp'), 'a', 'b']
            log = profile.logs/(implementation+'-'+str(instrument)+'-'+deadline+'.log')
            if instrument:
                command = ['/usr/bin/valgrind', '--leak-check=full', '--show-leak-kinds=all',
                           '--track-fds=yes', '--log-file='+str(log), *command]
            start = time.monotonic()
            with open('/dev/null', 'wb') as sink:
                done = subprocess.run(['/usr/bin/timeout', deadline, *command], cwd=work,
                    env={'PATH': '/usr/bin:/bin', 'LC_ALL': 'C'}, stdout=sink, stderr=subprocess.PIPE, timeout=10)
            results.append({'implementation': implementation, 'valgrind': instrument,
                'deadline_seconds': float(deadline), 'elapsed_seconds': time.monotonic()-start,
                'status': done.returncode, 'stderr': done.stderr.hex(),
                'log': str(log.relative_to(ROOT)) if log.exists() else None,
                'log_sha256': fingerprint(log) if log.exists() else None,
                'original_assertion_pass': done.returncode == 1,
                'memory_assessment': 'Retained raw log; a timed-out process is not clean-exit evidence.'})
        (work/'cmp').unlink()
report = {'scope': 'The original cmp script retains its 0.4-second sparse-size assertion unchanged. This separate measurement compares that deadline with a five-second instrumentation allowance.',
    **profile.metadata(), 'driver_sha256': fingerprint(Path(__file__)),
    'fixture': {'a_bytes': 14 << 40, 'b_bytes': 15 << 40, 'allocated_blocks': 0}, 'results': results}
profile.report.write_text(json.dumps(report, indent=2)+'\n')
for row in results:
    print(row['implementation'], 'valgrind' if row['valgrind'] else 'native', row['deadline_seconds'], row['status'])
