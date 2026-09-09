#!/usr/bin/env python3
"""Compare borrowed and temporary printf string ownership with pinned GNU Gawk."""
# SPDX-License-Identifier: GPL-3.0-or-later
import importlib.util
import json
from pathlib import Path
import re
import subprocess
import sys
import tempfile
from comparison_profile import ComparisonProfile, fingerprint

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT/'tests/gnu'))
spec = importlib.util.spec_from_file_location('reviewed', ROOT/'tests/gnu/reviewed-original.py')
runner = importlib.util.module_from_spec(spec)
spec.loader.exec_module(runner)
profile = ComparisonProfile('gawk-format-ownership', oracle=ROOT/'build/gnu-gawk/gawk')
programs = {
    'borrowed-string': 'BEGIN { s="alpha"; print sprintf("%1$s|%1$8.3s|%1$-8s",s); print s }',
    'borrowed-number': 'BEGIN { x=42; print sprintf("%1$s|%1$04d|%1$s",x); print x+1 }',
    'infinity-position': 'BEGIN { x=-log(0); print sprintf("%1$s|%1$10s|%1$-8.3s",x); print x>0 }',
    'nan-signs': 'BEGIN { x=sqrt(-1); print sprintf("%s|%s",x,-x); print typeof(x) }',
    'mixed-tail': 'BEGIN { x=-log(0); print sprintf("%s%c%s:%d",x,65,-x,7); print typeof(x) }',
    'repeated-conversion': 'BEGIN { x=-log(0); for(i=0;i<200;i++) { s=sprintf("%s|%s|%s",x,-x,"done"); n+=length(s) } print n,s,typeof(x) }',
}
inputs = {str(p): fingerprint(p) for p in (Path(__file__), ROOT/'tests/comparison_profile.py', ROOT/'tests/gnu/reviewed-original.py', Path('/usr/bin/valgrind'), Path('/usr/bin/valgrind.bin'))}
results = []
for alias in ('awk', 'gawk', 'nawk'):
    for name, program in programs.items():
        outcomes = {}
        for implementation in ('gnu', 'rboxc'):
            for instrument in (False, True):
                key = implementation+('-valgrind' if instrument else '')
                saved = profile.logs/(alias+'-'+name+'-'+key)
                saved.mkdir()
                with tempfile.TemporaryDirectory(prefix='rboxc-gawk-format-') as directory:
                    work = Path(directory)
                    (work/alias).symlink_to(profile.oracle if implementation == 'gnu' else profile.binary)
                    argv = [str(work/alias), program]
                    log = saved/'memory.log'
                    if instrument:
                        argv = ['/usr/bin/valgrind', '--leak-check=full', '--show-leak-kinds=all', '--track-fds=yes', '--log-file='+str(log), *argv]
                    done = subprocess.run(argv, cwd=work, input=b'', capture_output=True, timeout=30,
                                          env={'LC_ALL':'C', 'LANGUAGE':'C', 'HOME':directory, 'TZ':'UTC0'})
                (saved/'stdout').write_bytes(done.stdout)
                (saved/'stderr').write_bytes(done.stderr)
                outcome = {'status':done.returncode, 'stdout':done.stdout.hex(), 'stderr':done.stderr.hex()}
                if instrument:
                    text = log.read_text()
                    pids = set(re.findall(r'^==([0-9]+)==', text, re.M))
                    assert len(pids) == 1
                    memory = runner.parse_memory_log(text, pids.pop(), exec_only=True)
                    outcome.update(memory=memory, memory_log=str(log.relative_to(ROOT)), memory_clean=memory['complete_exec_log'] and memory['errors'] == memory['non_inherited_descriptors'] == 0 and not any(memory['heap_bytes'].get(k, 0) for k in ('definitely lost', 'indirectly lost', 'possibly lost')))
                outcome['raw'] = {str(p.relative_to(ROOT)):fingerprint(p) for p in saved.iterdir()}
                outcomes[key] = outcome
        reference = outcomes['gnu']
        equivalent = reference['status'] == 0 and all(all(o[k] == reference[k] for k in ('status','stdout','stderr')) for o in outcomes.values())
        passed = equivalent and outcomes['rboxc-valgrind']['memory_clean']
        results.append({'alias':alias, 'name':name, 'program':program, 'equivalent':equivalent, 'pass':passed, 'outcomes':outcomes})
        print('PASS' if passed else 'OPEN', alias, name, flush=True)
assert all(fingerprint(Path(p)) == h for p, h in inputs.items())
profile.report.write_text(json.dumps({**profile.metadata(), 'scope':'Six bounded ownership cases through all three Gawk aliases; original native findings retained. Compare status and exact output in native and instrumented runs. Candidate logs require zero errors, lost allocations and non-inherited descriptors.', 'inputs':inputs, 'complete':True, 'passed':sum(r['pass'] for r in results), 'total':len(results), 'results':results}, indent=2)+'\n')
raise SystemExit(not all(r['pass'] for r in results))
