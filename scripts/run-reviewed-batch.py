#!/usr/bin/env python3
"""Run an explicit reviewed script list with independent resumable reports."""
# SPDX-License-Identifier: GPL-3.0-or-later
import argparse
from concurrent.futures import ThreadPoolExecutor, as_completed
import fcntl
import hashlib
import json
from pathlib import Path
import re
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[1]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--script', action='append', required=True)
    parser.add_argument('--report-prefix', required=True)
    parser.add_argument('--jobs', type=int, choices=range(1, 9), default=4)
    parser.add_argument('--valgrind', action='store_true')
    parser.add_argument('--rerun', action='store_true', help='Rerun completed sides instead of resuming them')
    args = parser.parse_args()
    assert re.fullmatch(r'[a-z0-9][a-z0-9-]*', args.report_prefix), 'invalid report prefix'
    definitions = {row['script']: row for row in json.loads((ROOT/'inventory/gnu-reviewed-tests.json').read_text())}
    assert len(args.script) == len(set(args.script)), 'duplicate script selection'
    for script in args.script:
        assert script in definitions, f'unreviewed script: {script}'
        assert not args.valgrind or definitions[script].get('valgrind'), f'Valgrind profile not enabled: {script}'
    names = {script: args.report_prefix+'-'+hashlib.sha256(script.encode()).hexdigest()[:12]
             for script in args.script}
    assert len(set(names.values())) == len(names), 'report name collision'
    lock = (ROOT/'evidence/raw'/(args.report_prefix+'-batch.lock')).open('a')
    try:
        fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
    except BlockingIOError:
        parser.error('a controller with this report prefix is already running')
    results = {}

    def run(script):
        name = names[script]
        command = [sys.executable, str(ROOT/'tests/gnu/reviewed-original.py'),
                   '--script', script, '--report-name', name]
        if args.valgrind:
            command.append('--valgrind')
        if not args.rerun:
            command.append('--resume')
        log = ROOT/'evidence/raw'/(name+'.runner.log')
        with log.open('a') as output:
            outcome = subprocess.run(command, cwd=ROOT, stdout=output, stderr=subprocess.STDOUT)
        return {'script': script, 'report': name, 'runner_status': outcome.returncode,
                'runner_log': str(log.relative_to(ROOT))}

    with ThreadPoolExecutor(max_workers=args.jobs) as pool:
        for future in as_completed([pool.submit(run, script) for script in args.script]):
            row = future.result()
            results[row['script']] = row
            report = {'scope': 'Runner completion; consult each report for assertions and memory findings.',
                      'selected': args.script, 'completed': [results[s] for s in args.script if s in results]}
            destination = ROOT/'evidence/raw'/(args.report_prefix+'-index.json')
            temporary = destination.with_suffix('.json.tmp')
            temporary.write_text(json.dumps(report, indent=2)+'\n')
            temporary.replace(destination)
            print('COMPLETED', row['script'], 'runner status', row['runner_status'], flush=True)
    return any(row['runner_status'] != 0 for row in results.values())


if __name__ == '__main__':
    raise SystemExit(main())
