#!/usr/bin/env python3
"""Run the two unchanged GNU originals for uuencode/uudecode."""
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
runner = importlib.util.module_from_spec(spec); spec.loader.exec_module(runner)
profile = ComparisonProfile('sharutils-original', oracle=ROOT/'build/gnu-sharutils/src/uuencode')
source = Path(json.loads((ROOT/'inventory/sources.json').read_text())['sharutils']['source'])
manifest = json.loads((ROOT/'inventory/sharutils-tests.json').read_text())
assert fingerprint(source/'tests/Makefile.am') == manifest['registration_sha256']
assert fingerprint(source/'tests/testdata') == manifest['fixture_sha256']
selections = [row for row in manifest['inputs'] if row['reviewed']]
assert {row['path'] for row in selections} == {'tests/uutest-1', 'tests/uudecode-2'}
oracles = {n: ROOT/'build/gnu-sharutils/src'/n for n in ('uuencode','uudecode')}
helpers = {n: ROOT/'build/gnu-coreutils/src/coreutils' for n in ('rm','basename','ls','mktemp','mkdir','cat')}
helpers.update({n: ROOT/'build/gnu-diffutils/src'/n for n in ('cmp','diff')})
inputs = {p:fingerprint(p) for p in {*oracles.values(), *helpers.values(), Path('/bin/bash'),
          Path('/bin/sh').resolve(), source/'tests/testdata', source/'tests/Makefile.am', Path(__file__)}}
for row in selections:
    p = source/row['path']; assert fingerprint(p) == row['sha256']; inputs[p] = row['sha256']
results = []
for row in selections:
    name = Path(row['path']).name
    outcomes = {}
    for implementation in ('gnu','rboxc'):
        for instrument in (False,True):
            key = implementation+('-valgrind' if instrument else '')
            saved = profile.logs/(name+'-'+key); saved.mkdir()
            memory = saved/'memory'; memory.mkdir()
            with tempfile.TemporaryDirectory(prefix='rboxc-sharutils-original-') as directory:
                work = Path(directory)
                for sub in ('exec','real','deps','files'):
                    (work/sub).mkdir()
                for command, binary in helpers.items():
                    (work/'deps'/command).symlink_to(binary)
                for command, binary in oracles.items():
                    (work/'real'/command).symlink_to(binary if implementation=='gnu' else profile.binary)
                    invocation = [command]
                    if instrument:
                        invocation = ['/usr/bin/valgrind','--leak-check=full','--show-leak-kinds=all',
                                      '--track-fds=yes','--trace-children=yes','--log-file='+str(memory/'%p.log'),*invocation]
                    wrapper = work/'exec'/command
                    wrapper.write_text('#!/bin/sh\nPATH='+shlex.quote(str(work/'real')+':'+str(work/'deps'))+
                                       '\nexport PATH\nexec '+shlex.join(invocation)+' "$@"\n')
                    wrapper.chmod(0o755)
                env = {'PATH':str(work/'exec')+':'+str(work/'deps'), 'HOME':directory, 'TMPDIR':directory,
                       'LC_ALL':'C','LANGUAGE':'C','TZ':'UTC0','top_srcdir':str(source),
                       'top_builddir':str(ROOT/'build/gnu-sharutils'),'UUENCODE':'uuencode','UUDECODE':'uudecode','DIFF':'diff'}
                done = subprocess.run(['/bin/bash',str(source/row['path'])],cwd=work/'files',env=env,
                                      stdin=subprocess.DEVNULL,capture_output=True,timeout=180,umask=0o022)
                (saved/'stdout').write_bytes(done.stdout); (saved/'stderr').write_bytes(done.stderr)
                shutil.copytree(work/'files',saved/'files')
                logs=[]
                for p in sorted(memory.glob('*.log')):
                    parsed=runner.parse_memory_log(p.read_text(),p.stem,exec_only=True)
                    commands=re.findall(r'^==\d+== Command: (.*)$',p.read_text(),re.M)
                    assert commands and all(c.split(' ',1)[0] in oracles for c in commands)
                    logs.append({**parsed,'command':commands[-1],'log':str(p.relative_to(ROOT)),'sha256':fingerprint(p)})
                outcomes[key]={'status':done.returncode,'stdout':done.stdout.hex(),'stderr':done.stderr.hex(),
                               'memory':logs,'stdout_log':str((saved/'stdout').relative_to(ROOT)),
                               'stdout_sha256':fingerprint(saved/'stdout'),'stderr_log':str((saved/'stderr').relative_to(ROOT)),
                               'stderr_sha256':fingerprint(saved/'stderr')}
    reference=outcomes['gnu']
    equivalent=all(all(o[k]==reference[k] for k in ('status','stdout','stderr')) for o in outcomes.values())
    logs=outcomes['rboxc-valgrind']['memory']
    clean=bool(logs) and all(m['complete_exec_log'] and m['errors']==0 and m['non_inherited_descriptors']==0 and
        not any(m['heap_bytes'].get(k,0) for k in ('definitely lost','indirectly lost','possibly lost')) for m in logs)
    results.append({**row,'pass':equivalent and reference['status']==0 and clean,'equivalent':equivalent,
                    'memory_clean':clean,'outcomes':outcomes})
    assert all(fingerprint(p)==expected for p,expected in inputs.items())
    report={'scope':'Both unchanged original scripts assigned to uuencode/uudecode, in four native/Valgrind profiles. Bash supplies the original ERR trap; all file operations occur in private fixtures. Each command log is classified and retained. shar/unshar originals are outside the assigned command scope.',
            **profile.metadata(),'inputs':{str(p):v for p,v in inputs.items()},'driver_sha256':fingerprint(Path(__file__)),
            'planned_total':2,'complete':len(results)==2,'passed':sum(r['pass'] for r in results),'total':len(results),
            'candidate_processes':sum(len(r['outcomes']['rboxc-valgrind']['memory']) for r in results),'results':results}
    profile.report.write_text(json.dumps(report,indent=2)+'\n')
    print('PASS' if results[-1]['pass'] else 'OPEN',name,flush=True)
raise SystemExit(report['passed']!=report['total'])
