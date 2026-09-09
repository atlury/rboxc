#!/usr/bin/env python3
"""Audit a bounded original Bash batch without changing installed-release evidence."""
# SPDX-License-Identifier: GPL-3.0-or-later
import argparse
import importlib.util
import json
from pathlib import Path
import re
import sys

ROOT = Path(__file__).resolve().parents[1]
sys.path[:0] = [str(ROOT/'tests'), str(ROOT/'tests/gnu')]
from comparison_profile import fingerprint
spec = importlib.util.spec_from_file_location('reviewed', ROOT/'tests/gnu/reviewed-original.py')
runner = importlib.util.module_from_spec(spec)
spec.loader.exec_module(runner)
parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('--original', type=Path, required=True)
parser.add_argument('--inventory-snapshot', type=Path, required=True)
parser.add_argument('--driver-snapshot', type=Path)
parser.add_argument('--memory-open',nargs='*',default=[],help='Explicit failing memory profiles to audit and exclude from strict counts')
parser.add_argument('--assertion-baseline',nargs='*',default=[],help='GNU and candidate share a profile-specific assertion difference; exclude the entire script')
parser.add_argument('--report-name', required=True)
args = parser.parse_args()
assert re.fullmatch(r'[a-z0-9-]+', args.report_name)
target = ROOT/'evidence'/(args.report_name+'.json')
assert not target.exists()
report = json.loads(args.original.read_text())
assert set(args.memory_open)<={r['selection'] for r in report['results'] if not r['pass']}
assert set(args.assertion_baseline)<={r['selection'] for r in report['results'] if not r['pass']}
assert report['complete'] and report['total'] == report['planned_total'] == len(report['results'])
assert report['passed'] == sum(r['pass'] for r in report['results'])
assert fingerprint(Path(report['binary'])) == report['binary_sha256']
assert fingerprint(ROOT/'build/gnu-bash/bash') == report['gnu_binary_sha256']
assert fingerprint(args.inventory_snapshot) == report['inputs'][str(ROOT/'inventory/bash-tests.json')]
inventory = json.loads(args.inventory_snapshot.read_text())
rows = {(r['target']+':'+case['script'] if r.get('script_cases') else r['target']):{**r,**case}
        for r in inventory['inputs'] if r['reviewed'] for case in r.get('script_cases',[{}])}
for path, digest in {**report['inputs'], **report['runtime_helpers']}.items():
    actual = Path(path)
    if actual == ROOT/'inventory/bash-tests.json': actual = args.inventory_snapshot
    if actual == ROOT/'tests/bash-original.py' and args.driver_snapshot: actual = args.driver_snapshot
    assert fingerprint(actual) == digest, path
source = Path('/opt/src/bash-5.3/tests')
clean = []
open_processes = []
open_findings = []
raw = {}
for result in report['results']:
    row = rows[result['selection']]
    intrinsic_open = row.get('state')=='reviewed-original-intrinsic-descriptor-open'
    baseline = result['selection'] in args.assertion_baseline
    allow_open = intrinsic_open or result['selection'] in args.memory_open or baseline
    assert result['pass'] == (not allow_open)
    if intrinsic_open: assert result['selection']=='set-x'
    expected = source/row['expected']
    assert fingerprint(expected) == row['fixtures'][row['expected']]
    if baseline:
        assert any(not result['outcomes'][k]['expected_output_matches'] for k in ('gnu','gnu-valgrind'))
    for key, outcome in result['outcomes'].items():
        assert not outcome.get('timed_out',False)
        assert outcome.get('timeout_seconds',180)==row.get('timeout_seconds',180)
        assert outcome.get('stdin_script',False)==bool(row.get('stdin_script'))
        reference = result['outcomes']['gnu-valgrind' if key.endswith('-valgrind') and baseline else 'gnu']
        if not baseline: assert outcome['expected_output_matches']
        assert outcome['status'] == reference['status']
        for path, digest in outcome['raw'].items():
            assert fingerprint(ROOT/path) == digest
            raw[path] = digest
        output = next(ROOT/p for p in outcome['raw'] if p.endswith('/stdout')).read_bytes()
        if row['output_mode'] in ('drop-expect','drop-expect-stdout'):
            output = b''.join(line for line in output.splitlines(keepends=True) if not line.startswith(b'expect'))
        assert outcome['expected_output_matches'] == (output == expected.read_bytes())
        if baseline:
            reference_output = next(ROOT/p for p in reference['raw'] if p.endswith('/actual')).read_bytes()
            assert output == reference_output
        assert next(ROOT/p for p in outcome['raw'] if p.endswith('/actual')).read_bytes() == output
        mounts = outcome.get('private_mounts', [])
        assert [m['original'] for m in mounts] == row.get('absolute_helpers', [])
        for mount in mounts:
            assert mount['original'] in ('/bin/echo','/bin/cat','/bin/mkdir','/bin/touch','/bin/chmod','/bin/rm','/usr/bin/true','/usr/bin/false')
            assert str(Path(mount['original']).resolve()) == mount['destination']
            assert fingerprint(Path(mount['destination'])) == row['host_inputs'][mount['destination']]
            replacement = ROOT/'build/gnu-coreutils/src/coreutils' if key.startswith('gnu') else Path(report['binary'])
            assert mount['source'] == str(replacement) and fingerprint(replacement) == mount['source_sha256']
        assert bool(outcome['memory']) == key.endswith('-valgrind')
        for memory in outcome['memory']:
            log = ROOT/memory['log']
            assert fingerprint(log) == memory['sha256']
            text = log.read_text()
            pids = set(re.findall(r'^==([0-9]+)==', text, re.M))
            assert len(pids) == 1
            parsed = runner.parse_memory_log(text, pids.pop())
            assert all(memory[k] == v for k, v in parsed.items())
            errors = list(re.finditer(r'ERROR SUMMARY:', text))
            fds = list(re.finditer(r'FILE DESCRIPTORS:', text))
            images = list(re.finditer(r'^==[0-9]+== Command:', text, re.M))
            complete = bool(errors) and len(errors) == len(fds) and (not images or
                errors[-1].start() > images[-1].start() and fds[-1].start() > images[-1].start())
            assert memory['complete'] == complete
            lost = any(parsed['heap_bytes'].get(k,0) for k in ('definitely lost','indirectly lost','possibly lost'))
            actual_clean = complete and parsed['errors']==0 and parsed['non_inherited_descriptors']==0 and not lost
            assert memory['clean'] == actual_clean
            if key == 'rboxc-valgrind':
                assert complete
                if not allow_open: assert actual_clean
                if not memory['clean']:
                    assert allow_open
                    if intrinsic_open:
                        assert parsed['errors']==1 and parsed['non_inherited_descriptors']==0 and not lost
                        assert re.search(r'Command: [^\n]+/exec/bash \./set-x1\.sub\n', text)
                        assert re.search(r'File descriptor 4: [^\n]+/bash-trace-[0-9]+ is already closed', text)
                        assert 'rboxc_bash_owned_close' in text and 'rboxc_bash_xtrace_reset' in text
                        assert 'Previously closed' in text and 'fclose' in text
                    open_findings.append({'selection':result['selection'],'log':memory['log'],'sha256':memory['sha256'],'memory':parsed})
                else:
                    assert parsed['errors']==0
                collection = open_processes if allow_open else clean
                collection.append({'selection': result['selection'], 'log': memory['log'], 'sha256': memory['sha256']})
            raw[memory['log']] = memory['sha256']
assert len({p['log'] for p in clean+open_processes}) == len(clean)+len(open_processes)
assert {r['selection'] for r in open_findings}|set(args.assertion_baseline)=={r['selection'] for r in report['results'] if not r['pass']}
target.write_text(json.dumps({'scope':'Strict original scripts match unchanged GNU expected output and exit status. Explicit assertion baselines require candidate output and status to equal GNU separately with and without instrumentation; their entire families remain outside strict counts. Every raw process log is reparsed, and no timeout is accepted. Explicit memory-open profiles retain every finding. Private helper mounts leave host files unchanged. Native findings remain separate. This is batch evidence, not GNU-wide completion.',
    'binary':report['binary'], 'binary_sha256':report['binary_sha256'],
    'original':str(args.original), 'original_sha256':fingerprint(args.original),
    'inventory_snapshot':str(args.inventory_snapshot), 'inventory_sha256':fingerprint(args.inventory_snapshot),
    'original_groups':report.get('original_groups',report['total']),'original_scripts':report['total'], 'strict_original_passes':report['passed'],'clean_candidate_processes':len(clean),
    'open_processes':open_processes,'open_findings':open_findings,'explicit_memory_open':args.memory_open,'assertion_baselines':args.assertion_baseline,
    'driver_sha256':fingerprint(Path(__file__)), 'processes':clean, 'raw':raw}, indent=2)+'\n')
print('Audited', report['total'], 'original Bash scripts and', len(clean), 'clean candidate processes')
