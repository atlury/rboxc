#!/usr/bin/env python3
"""Account for every script registered by the pinned GNU Coreutils suite."""
# SPDX-License-Identifier: GPL-3.0-or-later
from collections import Counter
import hashlib
import json
import os
from pathlib import Path
import re

ROOT = Path(__file__).resolve().parents[1]
SOURCE = Path(os.environ.get('GNU_COREUTILS_SOURCE', '/opt/src/coreutils-9.11'))

def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()

def main():
    registration = SOURCE/'tests/local.mk'
    text = registration.read_text().replace('\\\n', ' ')
    def variable(name):
        return re.search(r'^'+name+r'\s*=\s*(.*)$', text, re.M)[1].strip()
    roots = variable('all_root_tests').split()
    ordinary = variable('all_tests').replace('$(all_root_tests)', ' '.join(roots)).split()
    generated = variable('factor_tests').replace('$(tf)', variable('tf')).split()
    scripts = sorted(ordinary + generated)
    assert len(scripts) == len(set(scripts)) == 733, 'pinned suite registration changed'
    read = lambda name: json.loads((ROOT/name).read_text())
    reviewed = {r['script']: r for r in read('evidence/gnu-reviewed-original.json')['results']}
    instrumented = {r['script']: r for r in read('evidence/gnu-reviewed-valgrind.json')['results']}
    cp_manifest = {r['name']: r for r in read('inventory/gnu-cp-tests.json')}
    cp_results = read('evidence/gnu-cp-original.json')['results']
    assert set(reviewed) <= set(scripts)
    rows = []
    for script in scripts:
        source = SOURCE/script
        row = {'script': script, 'registered_root_test': script in roots,
               'generated': script in generated, 'state': 'pending',
               'execution_coverage': 'none', 'valgrind_state': 'pending'}
        if source.exists():
            row['sha256'] = digest(source)
        else:
            assert script in generated, script
            row['generator_inputs'] = {p: digest(SOURCE/p) for p in
                ('tests/factor/create-test.sh', 'tests/factor/run.sh')}
        if any(part in script for part in ('selinux', '/chcon/', '/runcon/', 'systemd')):
            row.update(state='excluded', reason='SELinux and systemd excluded by project scope')
        if script.startswith('tests/cp/') and source.name in cp_manifest:
            pin = cp_manifest[source.name]
            assert row['sha256'] == pin['sha256'], script
            profiles = [r for r in cp_results if r['test'] == source.name]
            row['profiles'] = profiles
            states = {r['state'] for r in profiles}
            if not pin['enabled']:
                row.update(state='excluded', reason=pin['reason'])
            elif states & {'mismatch', 'baseline-failure'}:
                row['state'] = 'failed'
            elif states == {'pass'}:
                row['state'] = 'passed'
            elif states == {'pass', 'skip'}:
                row['state'] = 'passed-with-profile-skips'
            elif states == {'skip'}:
                row['state'] = 'skipped'
            if profiles and pin['enabled']:
                row['execution_coverage'] = 'full-script-with-recorded-profiles'
        if script in reviewed:
            result = reviewed[script]
            assert row['sha256'] == result['sha256'], script
            partial = bool(result.get('cases'))
            row['execution_coverage'] = 'selected-cases' if partial else 'full-script'
            row['selected_case_count'] = len(result.get('cases', []))
            if result['pass']:
                row['state'] = 'partial' if partial else 'passed'
            elif result['gnu']['status'] == result['rboxc']['status'] == 77:
                row['state'] = 'skipped'
            else:
                row['state'] = 'failed'
            row['evidence'] = 'evidence/gnu-reviewed-original.json'
            row['outcomes'] = {key: result[key] for key in ('gnu', 'rboxc')}
        if script in instrumented:
            result = instrumented[script]
            assert row['sha256'] == result['sha256'], script
            row['valgrind_state'] = 'passed-selection' if result['pass'] else 'open'
            row['valgrind_evidence'] = 'evidence/gnu-reviewed-valgrind.json'
        rows.append(row)
    report = {'provider': 'GNU Coreutils 9.11',
              'registration': {'path': 'tests/local.mk', 'sha256': digest(registration)},
              'scope': 'All registered scripts accounted for; partial and pending are not full-suite passes. Script passes may include platform-conditional branches.',
              'total': len(rows), 'registered_root_tests': len(roots),
              'generated_factor_tests': len(generated),
              'counts': dict(sorted(Counter(r['state'] for r in rows).items())),
              'results': rows}
    (ROOT/'evidence/gnu-suite-coverage.json').write_text(json.dumps(report, indent=2)+'\n')
    print(json.dumps({k: v for k, v in report.items() if k != 'results'}, indent=2))

if __name__ == '__main__':
    main()
