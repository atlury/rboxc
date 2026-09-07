#!/usr/bin/env python3
"""Merge completed original-test batches, retaining superseded findings."""
# SPDX-License-Identifier: GPL-3.0-or-later
import argparse
import hashlib
import json
from pathlib import Path
import re

ROOT = Path(__file__).resolve().parents[1]

def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--native', action='store_true', help='Merge completed native batches instead of Valgrind batches')
    parser.add_argument('--archive-only', action='store_true', help='Move completed legacy batches to raw evidence without merging')
    parser.add_argument('reports', nargs='+', help='Evidence filename stems from completed batches')
    args = parser.parse_args()
    paths = []
    for name in args.reports:
        assert re.fullmatch(r'[a-z0-9][a-z0-9-]*', name), 'invalid report name'
        legacy = ROOT/'evidence'/f'{name}.json'
        raw = ROOT/'evidence/raw'/f'{name}.json'
        paths.append(legacy if legacy.exists() else raw)
    if args.archive_only:
        for source in paths:
            destination = ROOT/'evidence/raw'/source.name
            if source != destination:
                assert not destination.exists(), 'raw batch already exists'
                source.rename(destination)
        print(f'Archived {len(paths)} completed batch reports')
        return
    manifest = json.loads((ROOT/'inventory/gnu-reviewed-tests.json').read_text())
    definitions = {row['script']: row for row in manifest}
    kind = 'original' if args.native else 'valgrind'
    destination = ROOT/f'evidence/gnu-reviewed-{kind}.json'
    current = json.loads(destination.read_text())
    results = {row['script']: row for row in current['results']}
    history_path = ROOT/f'evidence/gnu-reviewed-{kind}-observations.json'
    history = json.loads(history_path.read_text()) if history_path.exists() else {
        'scope': 'Superseded unresolved original-test results; these are not clean passes.', 'results': []}
    known_history = {json.dumps(row, sort_keys=True) for row in history['results']}
    batch_scripts = set()
    candidate_hash = hashlib.sha256((ROOT/'target/release/rboxc').read_bytes()).hexdigest()
    for source in paths:
        report = json.loads(source.read_text())
        assert report['total'] == len(report['results'])
        assert report['passed'] == sum(row['pass'] for row in report['results'])
        for row in report['results']:
            script = row['script']
            assert script not in batch_scripts, 'duplicate script across input batches'
            batch_scripts.add(script)
            expected = definitions[script]
            assert all(row.get(key) == value for key, value in expected.items()), script
            assert row['rboxc']['binary_sha256'] == candidate_hash, 'batch uses an older candidate'
            assert all(('memory' in row[key]) != args.native for key in ('gnu', 'rboxc')), 'batch instrumentation mode differs'
            previous = results.get(script)
            if previous and previous != row and not previous['pass']:
                key = json.dumps(previous, sort_keys=True)
                if key not in known_history:
                    history['results'].append(previous)
                    known_history.add(key)
            results[script] = row
    merged = [results[row['script']] for row in manifest if row['script'] in results]
    current.update(results=merged, total=len(merged), passed=sum(row['pass'] for row in merged))
    # Write history first so an interruption cannot lose replaced findings.
    for path, report in [(history_path, history), (destination, current)]:
        temporary = path.with_suffix('.json.tmp')
        temporary.write_text(json.dumps(report, indent=2)+'\n')
        temporary.replace(path)
    print(json.dumps({'passed': current['passed'], 'total': current['total'],
                      'retained_observations': len(history['results'])}))

if __name__ == '__main__':
    main()
