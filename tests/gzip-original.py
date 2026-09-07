#!/usr/bin/env python3
"""Compare individually reviewed GNU Gzip originals on private fixtures."""
# SPDX-License-Identifier: GPL-3.0-or-later
import importlib.util
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
from comparison_profile import ComparisonProfile, fingerprint

ROOT=Path(__file__).resolve().parents[1]
sys.path.insert(0,str(ROOT/'tests/gnu'))
spec=importlib.util.spec_from_file_location('reviewed',ROOT/'tests/gnu/reviewed-original.py')
runner=importlib.util.module_from_spec(spec);spec.loader.exec_module(runner)
profile=ComparisonProfile('gzip-original',oracle=ROOT/'build/gnu-gzip/gzip',selections=True)
pin=json.loads((ROOT/'inventory/sources.json').read_text())['gzip'];source=Path(pin['source'])
manifest=json.loads((ROOT/'inventory/gzip-tests.json').read_text())
assert fingerprint(source/manifest['registration']['path'])==manifest['registration']['sha256']
oracles={name:ROOT/'build/gnu-gzip'/name for name in pin['commands']}
hashes={name:fingerprint(path) for name,path in oracles.items()}
interpreters={name:Path(path.read_text().splitlines()[0][2:]) for name,path in oracles.items() if name!='gzip'}
driver_sha256=fingerprint(Path(__file__))
selected=set(profile.options.commands)
assert selected<={Path(r['script']).name for r in manifest['scripts'] if r['reviewed']}
results=[]
for index,row in enumerate(manifest['scripts']):
    script=source/row['script'];assert fingerprint(script)==row['source_sha256']
    if not row['reviewed'] or (selected and script.name not in selected):continue
    outcomes={}
    for implementation in ('gnu','rboxc'):
        for instrument in (False,True):
            key=implementation+('-valgrind' if instrument else '')
            with tempfile.TemporaryDirectory(prefix='rboxc-gzip-original-') as directory:
                work=Path(directory)
                for sub in ('src','real','tests','memory'):(work/sub).mkdir()
                credentials=row.get('credentials')
                if credentials:
                    assert os.geteuid()==0
                    binary=oracles['gzip'] if implementation=='gnu' else profile.binary
                    copied=work/'real/provider-binary';shutil.copy2(binary,copied)
                    assert fingerprint(copied)==fingerprint(binary)
                for name in pin['commands']:
                    binary=oracles[name] if implementation=='gnu' else profile.binary
                    if credentials:
                        if implementation=='gnu' and name!='gzip':
                            shutil.copy2(binary,work/'real'/name)
                        else:(work/'real'/name).symlink_to('provider-binary')
                    else:(work/'real'/name).symlink_to(binary)
                    if instrument:
                        program=str(interpreters[name])+' '+str(work/'real'/name) if implementation=='gnu' and name!='gzip' else name
                        wrapper=work/'src'/name
                        wrapper.write_text('#!/bin/sh\nPATH='+str(work/'real')+':$PATH\nexport PATH\nexec /usr/bin/valgrind --leak-check=full --show-leak-kinds=all --track-fds=yes --trace-children=yes --log-file='+str(work/'memory/%p.log')+' '+program+' "$@"\n')
                        wrapper.chmod(0o755)
                    else:(work/'src'/name).symlink_to(work/'real'/name)
                command=['/bin/bash','-c','exec 9>&2; exec /bin/bash "$1"','gzip-test',str(script)]
                if credentials:
                    for path in [work,*work.rglob('*')]:os.chown(path,credentials['uid'],credentials['gid'],follow_symlinks=False)
                    command=['/usr/bin/setpriv','--reuid='+str(credentials['uid']),'--regid='+str(credentials['gid']),'--clear-groups',*command]
                done=subprocess.run(command,cwd=work/'tests',stdin=subprocess.DEVNULL,capture_output=True,timeout=row.get('timeout_seconds',300),
                    env={'PATH':str(work/'src')+':/usr/bin:/bin','HOME':directory,'TMPDIR':directory,
                         'LC_ALL':'C','LANGUAGE':'C','TZ':'UTC0','srcdir':str(source/'tests'),
                         'top_srcdir':str(source),'abs_top_srcdir':str(source),'abs_srcdir':str(source/'tests'),
                         'abs_top_builddir':directory,'VERSION':pin['version'],'PACKAGE_BUGREPORT':'bug-gzip@gnu.org',
                         'built_programs':' '.join(pin['commands']),'PERL':'/usr/bin/perl','SHELL':'/bin/bash'})
                log=profile.logs/f'{index:02}-{key}.log';log.write_bytes(done.stdout+done.stderr)
                outcome={'status':done.returncode,'stdout':done.stdout.hex(),'stderr':done.stderr.hex(),
                         'log':str(log.relative_to(ROOT)),'log_sha256':fingerprint(log)}
                if instrument:
                    saved=profile.logs/f'{index:02}-{key}-memory';shutil.copytree(work/'memory',saved)
                    outcome['memory']=[{**runner.parse_memory_log(p.read_text(),p.stem,exec_only=True),'log':str(p.relative_to(ROOT)),'sha256':fingerprint(p)} for p in sorted(saved.glob('*.log'))]
                outcomes[key]=outcome
    native_pass=outcomes['gnu']['status']==outcomes['rboxc']['status']==0
    assertions_pass=native_pass and all(r['status']==0 for r in outcomes.values())
    logs=outcomes['rboxc-valgrind']['memory']
    clean=bool(logs) and all(m['complete_exec_log'] and m['errors']==0 and m['non_inherited_descriptors']==0 and not any(m['heap_bytes'].get(k,0) for k in ('definitely lost','indirectly lost','possibly lost')) for m in logs)
    skipped=all(r['status']==77 for r in outcomes.values())
    passed=assertions_pass and clean
    state='passed' if passed else 'prerequisite_skip' if skipped else 'assertions_passed_memory_open' if assertions_pass else 'assertions_open'
    results.append({**row,'pass':passed,'native_pass':native_pass,'assertions_pass':assertions_pass,'memory_clean':clean,'skipped':skipped,'state':state,'outcomes':outcomes})
    print(state.upper(),row['script'],flush=True)
    report={'scope':'Individually reviewed original assertions, native and Valgrind, with private provider dispatch. The write-error test uses an unprivileged identity and verified executable copies. Unported auxiliary scripts and excluded originals are not executed.',**profile.metadata(),'gnu_binaries':{n:{'path':str(p),'sha256':hashes[n]} for n,p in oracles.items()},'driver_sha256':driver_sha256,'passed':sum(r['pass'] for r in results),'native_passed':sum(r['native_pass'] for r in results),'total':len(results),'state_counts':{s:sum(r['state']==s for r in results) for s in sorted({r['state'] for r in results})},'selected_scripts':sorted(selected),'registered_original_scripts':len(manifest['scripts']),'remaining':[r for r in manifest['scripts'] if not r['reviewed']],'results':results}
    assert fingerprint(Path(__file__))==driver_sha256
    assert all(fingerprint(p)==hashes[n] for n,p in oracles.items())
    profile.report.write_text(json.dumps(report,indent=2)+'\n')
raise SystemExit(any(not r['pass'] for r in results))
