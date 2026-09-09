#!/usr/bin/env python3
"""Check the unchanged timeout recipe with only Gawk instrumented."""
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
profile = ComparisonProfile('gawk-timeout-native-children', oracle=ROOT/'build/gnu-gawk/gawk')
source = Path('/opt/src/gawk-5.4.1/test')
makefile = ROOT/'build/gnu-gawk/test/Makefile'
inputs = {str(p):fingerprint(p) for p in (Path(__file__), ROOT/'tests/gnu/reviewed-original.py',
    ROOT/'tests/comparison_profile.py', makefile, source/'Makefile.am', source/'Makefile.in',
    source/'timeout.awk', source/'timeout.ok', ROOT/'build/gnu-coreutils/src/coreutils',
    Path('/usr/bin/make'), Path('/bin/sh').resolve(), Path('/usr/bin/valgrind'), Path('/usr/bin/valgrind.bin'))}
results = []
for name, target in [('timeout','timeout')]:
    outcomes = {}
    for implementation in ('gnu','rboxc'):
        for instrument in (False,True):
            key = implementation+('-valgrind' if instrument else '')
            saved = profile.logs/(name+'-'+key)
            saved.mkdir()
            with tempfile.TemporaryDirectory(prefix='rboxc-gawk-helper-') as directory:
                work = Path(directory)
                (work/'exec').mkdir()
                (work/'exec/sleep').symlink_to(ROOT/'build/gnu-coreutils/src/coreutils')
                (work/'exec/gawk').symlink_to(profile.oracle if implementation=='gnu' else profile.binary)
                argv = ['gawk']
                log = saved/'memory.log'
                if instrument:
                    argv = ['/usr/bin/valgrind','--leak-check=full','--show-leak-kinds=all','--track-fds=yes','--trace-children=no','--child-silent-after-fork=yes','--log-file='+str(log),*argv]
                wrapper = work/'awk-wrapper'
                wrapper.write_text('#!/bin/sh\nexec '+shlex.join(argv)+' "$@"\n')
                wrapper.chmod(0o755)
                command = ['/usr/bin/make','--no-print-directory','-f',str(makefile),'srcdir='+str(source),'AWKPROG='+str(wrapper),target]
                done = subprocess.run(command,cwd=work,input=b'',capture_output=True,timeout=90,
                    env={'PATH':str(work/'exec')+':/usr/bin:/bin','HOME':directory,'TMPDIR':directory,'LC_ALL':'C','LANG':'C','LANGUAGE':'','TZ':'UTC0'})
                residual = work/'_timeout'
                original_passed = done.returncode==0 and not residual.exists() and done.stdout==b'timeout\n' and not done.stderr
                if residual.exists(): shutil.copy2(residual,saved/'actual-output')
            (saved/'stdout').write_bytes(done.stdout)
            (saved/'stderr').write_bytes(done.stderr)
            outcome = {'original_assertions_pass':original_passed,'status':done.returncode,'stdout_sha256':fingerprint(saved/'stdout'),'stderr_sha256':fingerprint(saved/'stderr'),
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
    passed = equivalent and all(o['original_assertions_pass'] for o in outcomes.values()) and outcomes['rboxc-valgrind']['memory_clean']
    results.append({'name':name,'program_sha256':fingerprint(source/(name+'.awk')),'original_make_target':target,'equivalent':equivalent,'pass':passed,'outcomes':outcomes})
    print('PASS' if passed else 'OPEN',name,flush=True)
assert all(fingerprint(Path(p))==h for p,h in inputs.items())
profile.report.write_text(json.dumps({**profile.metadata(), 'scope':'Unchanged GNU timeout recipe with only Gawk instrumented. Shell and sleep helpers run natively to avoid Valgrind startup exceeding the original 400 ms deadline. Their memory is not checked here. This bounded supplemental profile does not replace the preserved full-child-tracing limitation or add to strict original counts.',
    'inputs':inputs,'complete':True,'passed':sum(r['pass'] for r in results),'total':len(results),'results':results},indent=2)+'\n')
raise SystemExit(not all(r['pass'] for r in results))
