#!/usr/bin/env python3
"""Check GNU's unchanged MPFR capability guard without executing MPFR recipes."""
# SPDX-License-Identifier: GPL-3.0-or-later
import importlib.util
import json
import os
from pathlib import Path
import re
import shlex
import subprocess
import sys
import tempfile
import shutil
from comparison_profile import ComparisonProfile, fingerprint

ROOT=Path(__file__).resolve().parents[1]
sys.path.insert(0,str(ROOT/'tests/gnu'))
spec=importlib.util.spec_from_file_location('reviewed',ROOT/'tests/gnu/reviewed-original.py')
runner=importlib.util.module_from_spec(spec);spec.loader.exec_module(runner)
profile=ComparisonProfile('gawk-optional-profiles',oracle=ROOT/'build/gnu-gawk/gawk')
source=Path('/opt/src/gawk-5.4.1');build=ROOT/'build/gnu-gawk';makefile=build/'test/Makefile'
assert '/* #undef HAVE_MPFR */' in (build/'config.h').read_text()
make_text=makefile.read_text()
registration=re.search(r'^NEED_MPFR = .*?(?=\n\S|\Z)',make_text,re.M|re.S)[0]
names=registration.split('=',1)[1].replace('\\\n',' ').split()
assert len(names)==21 and len(set(names))==21
inputs={p:fingerprint(p) for p in [Path(__file__),ROOT/'tests/comparison_profile.py',ROOT/'tests/gnu/reviewed-original.py',
    source/'test/Makefile.am',source/'test/Makefile.in',build/'config.h',makefile,Path('/usr/bin/make'),Path('/bin/sh').resolve(),Path('/usr/bin/valgrind'),Path('/usr/bin/valgrind.bin')]}
originals={}
for name in names:
    p=source/'test'/(name+'.awk')
    recipe=re.search('^'+re.escape(name)+r':.*?(?=\n\S|\Z)',make_text,re.M|re.S)[0]
    originals[name]={'program':str(p) if p.exists() else None,'program_sha256':fingerprint(p) if p.exists() else None,
        'configured_recipe_sha256':__import__('hashlib').sha256(recipe.encode()).hexdigest(),'executed':False}
    if p.exists():inputs[p]=fingerprint(p)
outcomes={}
for implementation in ('gnu','rboxc'):
    for instrument in (False,True):
        key=implementation+('-valgrind' if instrument else '')
        saved=profile.logs/key;saved.mkdir()
        with tempfile.TemporaryDirectory(prefix='rboxc-gawk-optional-') as directory:
            work=Path(directory);(work/'exec').mkdir();(work/'memory').mkdir()
            (work/'exec/gawk').symlink_to(profile.oracle if implementation=='gnu' else profile.binary)
            args=['gawk']
            if instrument:args=['/usr/bin/valgrind','--leak-check=full','--show-leak-kinds=all','--track-fds=yes','--trace-children=yes','--log-file='+str(work/'memory/%p.log'),*args]
            wrapper=work/'awk-wrapper';wrapper.write_text('#!/bin/sh\nexec '+shlex.join(args)+' "$@"\n');wrapper.chmod(0o755)
            command=['/usr/bin/make','--no-print-directory','-f',str(makefile),'top_builddir='+str(build),'top_srcdir='+str(source),
                'srcdir='+str(source/'test'),'AWKPROG='+str(wrapper),'mpfr-tests']
            env={'PATH':str(work/'exec')+':/usr/bin:/bin','HOME':directory,'TMPDIR':directory,'LC_ALL':'C','LANGUAGE':'C','TZ':'UTC0'}
            done=subprocess.run(command,cwd=work,env=env,stdin=subprocess.DEVNULL,stdout=subprocess.PIPE,stderr=subprocess.STDOUT,timeout=30)
            (saved/'driver.log').write_bytes(done.stdout)
            shutil.copytree(work/'memory',saved/'memory')
            logs=[]
            for p in sorted((saved/'memory').glob('*.log')):
                text=p.read_text();commands=re.findall(r'^==[0-9]+== Command: (.*)$',text,re.M);assert len(commands)==1
                assert commands[0].startswith('gawk ') and '-f ' not in commands[0]
                logs.append({**runner.parse_memory_log(text,p.stem,exec_only=True),'command':commands[0],'log':str(p.relative_to(ROOT)),'sha256':fingerprint(p)})
            if instrument:assert len(logs)==2
            assertions=done.returncode==0 and done.stdout==b'MPFR tests not supported on this system\n'
            clean=bool(logs) and all(m['complete_exec_log'] and m['errors']==m['non_inherited_descriptors']==0 and not any(m['heap_bytes'].get(k,0) for k in ('definitely lost','indirectly lost','possibly lost')) for m in logs)
            outcomes[key]={'status':done.returncode,'stdout':done.stdout.decode(),'assertions_pass':assertions,'memory_clean':clean if instrument else None,
                'driver_log':str((saved/'driver.log').relative_to(ROOT)),'driver_log_sha256':fingerprint(saved/'driver.log'),'memory':logs}
assert all(fingerprint(p)==h for p,h in inputs.items())
passed=all(o['assertions_pass'] for o in outcomes.values()) and outcomes['rboxc-valgrind']['memory_clean']
profile.report.write_text(json.dumps({**profile.metadata(),'scope':'Execute only the unchanged GNU Make mpfr-tests capability guard. Both configured binaries lack MPFR and must report unsupported. No individual MPFR recipe is executed or counted as passing; 20 distributed AWK inputs and one inline recipe are accounted as unavailable in this build.',
    'inputs':{str(p):h for p,h in inputs.items()},'driver_sha256':fingerprint(Path(__file__)),
    'complete':True,'pass':passed,'original_guard':'mpfr-tests','unavailable_recipes':originals,'outcomes':outcomes},indent=2)+'\n')
print('PASS' if passed else 'OPEN','MPFR capability guard; individual MPFR recipes remain unavailable')
raise SystemExit(not passed)
