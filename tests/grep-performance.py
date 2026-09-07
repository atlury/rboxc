#!/usr/bin/env python3
"""Observe bounded cached-file matcher throughput before and after buffer initialization."""
# SPDX-License-Identifier: GPL-3.0-or-later
import hashlib
import json
import os
from pathlib import Path
import statistics
import subprocess
import tempfile
import time

ROOT = Path(__file__).resolve().parents[1]
def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()
binaries = {'gnu': ROOT/'build/gnu-grep/src/grep',
            'before': ROOT/'target/grep-pcre-cleanup/release/rboxc',
            'after': ROOT/'target/grep-buffer-cleanup/release/rboxc'}
hashes = {name: digest(path) for name, path in binaries.items()}
line = b'alpha 123 beta 456 gamma delta epsilon zeta eta theta 789\n'
line_count = 150000
cases = {'basic': ['-c', 'alpha'], 'extended': ['-Ec', 'alpha|omega'],
         'fixed': ['-Fc', 'alpha'], 'perl': ['-Pc', r'alpha\s+\d+']}
rows = []
with tempfile.TemporaryDirectory(prefix='rboxc-grep-performance-') as directory:
    work = Path(directory)
    data = work/'input'; data.write_bytes(line*line_count)
    for name, binary in binaries.items():
        (work/name).mkdir(); (work/name/'grep').symlink_to(binary)
    for case, arguments in cases.items():
        values = {name: [] for name in binaries}
        for trial in range(10):
            # Rotate order and discard one warmup for each executable.
            names = list(binaries)
            names = names[trial % 3:]+names[:trial % 3]
            for name in names:
                start = time.perf_counter_ns()
                done = subprocess.run([str(work/name/'grep'), *arguments, str(data)],
                                      capture_output=True, timeout=30,
                                      env={'PATH':'/usr/bin:/bin', 'LC_ALL':'C'})
                elapsed = time.perf_counter_ns()-start
                assert (done.returncode, done.stdout, done.stderr) == (0, f'{line_count}\n'.encode(), b'')
                if trial:
                    values[name].append(elapsed)
        medians = {name: statistics.median(samples) for name, samples in values.items()}
        rows.append({'case':case, 'arguments':arguments, 'nanoseconds':values,
                     'median_nanoseconds':medians,
                     'after_to_before_ratio':medians['after']/medians['before']})
        print(case, {name:round(value/1e6,3) for name,value in medians.items()}, flush=True)
    report = {'scope':'Bounded cached-file observation with identical counts, exit statuses, and empty diagnostics. Rotating executable order, one warmup and nine samples. Concurrent suite activity and process startup limit precision; this is not a performance certification.',
              'binaries':{name:{'path':str(path),'sha256':hashes[name]} for name,path in binaries.items()},
              'driver_sha256':digest(Path(__file__)), 'input_sha256':digest(data),
              'input_bytes':data.stat().st_size, 'input_lines':line_count,
              'load_average':os.getloadavg(), 'results':rows}
    assert all(digest(path)==hashes[name] for name,path in binaries.items())
    (ROOT/'evidence/grep-buffer-performance.json').write_text(json.dumps(report,indent=2)+'\n')
