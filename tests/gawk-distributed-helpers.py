#!/usr/bin/env python3
"""Compare three distributed Gawk utilities, separately from original assertions."""
# SPDX-License-Identifier: GPL-3.0-or-later
import importlib.util
import json
from pathlib import Path
import re
import shlex
import shutil
import subprocess
import sys
import tempfile
from comparison_profile import ComparisonProfile, fingerprint

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT/'tests/gnu'))
spec = importlib.util.spec_from_file_location('reviewed', ROOT/'tests/gnu/reviewed-original.py')
runner = importlib.util.module_from_spec(spec)
spec.loader.exec_module(runner)
profile = ComparisonProfile('gawk-distributed-helpers', oracle=ROOT/'build/gnu-gawk/gawk')
source = Path('/opt/src/gawk-5.4.1/test')
makefile = ROOT/'build/gnu-gawk/test/Makefile'
baseline_path = ROOT/'evidence/gawk-format-lifetime-baseline-contract.json'
baseline = json.loads(baseline_path.read_text())
row = next(r for r in baseline['results'] if r['alias']=='gawk' and r['name']=='repeated-conversion')
log_input = ROOT/row['outcomes']['rboxc-valgrind']['memory_log']
assert fingerprint(log_input) == row['outcomes']['rboxc-valgrind']['raw'][str(log_input.relative_to(ROOT))]
inputs = {str(p):fingerprint(p) for p in (Path(__file__), ROOT/'tests/gnu/reviewed-original.py',
    ROOT/'tests/comparison_profile.py', makefile, source/'Makefile.am', source/'Makefile.in',
    source/'printfloat.awk', source/'printlang.awk', source/'valgrind.awk', baseline_path, log_input,
    Path('/usr/bin/make'), Path('/bin/sh').resolve(), Path('/usr/bin/valgrind'), Path('/usr/bin/valgrind.bin'))}
results = []
for name, target in [('printfloat',None), ('printlang','printlang'), ('valgrind','valgrind-scan')]:
    outcomes = {}
    for implementation in ('gnu','rboxc'):
        for instrument in (False,True):
            key = implementation+('-valgrind' if instrument else '')
            saved = profile.logs/(name+'-'+key)
            saved.mkdir()
            with tempfile.TemporaryDirectory(prefix='rboxc-gawk-helper-') as directory:
                work = Path(directory)
                (work/'exec').mkdir()
                (work/'exec/gawk').symlink_to(profile.oracle if implementation=='gnu' else profile.binary)
                argv = ['gawk']
                log = saved/'memory.log'
                if instrument:
                    argv = ['/usr/bin/valgrind','--leak-check=full','--show-leak-kinds=all','--track-fds=yes','--log-file='+str(log),*argv]
                wrapper = work/'awk-wrapper'
                wrapper.write_text('#!/bin/sh\nexec '+shlex.join(argv)+' "$@"\n')
                wrapper.chmod(0o755)
                if target:
                    command = ['/usr/bin/make','--no-print-directory','-f',str(makefile),'srcdir='+str(source),'AWKPROG='+str(wrapper),target]
                else:
                    command = [str(wrapper),'-f',str(source/'printfloat.awk')]
                if name=='valgrind':
                    shutil.copy2(log_input, work/'log.1')
                done = subprocess.run(command,cwd=work,input=b'',capture_output=True,timeout=90,
                    env={'PATH':str(work/'exec')+':/usr/bin:/bin','HOME':directory,'TMPDIR':directory,'LC_ALL':'C','LANG':'C','LANGUAGE':'','TZ':'UTC0'})
            (saved/'stdout').write_bytes(done.stdout)
            (saved/'stderr').write_bytes(done.stderr)
            outcome = {'status':done.returncode,'stdout_sha256':fingerprint(saved/'stdout'),'stderr_sha256':fingerprint(saved/'stderr'),
                       'stdout_bytes':len(done.stdout),'stdout_lines':done.stdout.count(b'\n')}
            if instrument:
                text = log.read_text()
                pids = set(re.findall(r'^==([0-9]+)==',text,re.M))
                assert len(pids)==1
                memory = runner.parse_memory_log(text,pids.pop(),exec_only=True)
                assert memory['complete_exec_log'] and memory['exec_images']==1
                outcome.update(memory=memory,memory_log=str(log.relative_to(ROOT)),memory_clean=memory['errors']==memory['non_inherited_descriptors']==0 and not any(memory['heap_bytes'].get(k,0) for k in ('definitely lost','indirectly lost','possibly lost')))
            outcome['raw'] = {str(p.relative_to(ROOT)):fingerprint(p) for p in saved.iterdir()}
            outcomes[key] = outcome
    reference = outcomes['gnu']
    equivalent = all(all(o[k]==reference[k] for k in ('status','stdout_sha256','stderr_sha256','stdout_lines')) for o in outcomes.values())
    passed = equivalent and reference['status']==0 and outcomes['rboxc-valgrind']['memory_clean']
    if name=='printfloat':
        passed = passed and reference['stdout_lines']==24000
    results.append({'name':name,'program_sha256':fingerprint(source/(name+'.awk')),'original_make_target':target,'equivalent':equivalent,'pass':passed,'outcomes':outcomes})
    print('PASS' if passed else 'OPEN',name,flush=True)
assert all(fingerprint(Path(p))==h for p,h in inputs.items())
profile.report.write_text(json.dumps({**profile.metadata(), 'scope':'Three distributed utilities: a finite 24000-line floating-format example, the unchanged GNU locale diagnostic target, and the unchanged Valgrind-log scanning target over a preserved local log. These are utility comparisons, not additional passing original assertion recipes.',
    'inputs':inputs,'complete':True,'passed':sum(r['pass'] for r in results),'total':len(results),'results':results},indent=2)+'\n')
raise SystemExit(not all(r['pass'] for r in results))
