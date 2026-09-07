#!/usr/bin/env python3
"""Run individually reviewed, unchanged GNU Sed original shell assertions."""
# SPDX-License-Identifier: GPL-3.0-or-later
import importlib.util
import json
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
profile=ComparisonProfile('sed-original',oracle=ROOT/'build/gnu-sed/sed/sed',selections=True)
pin=json.loads((ROOT/'inventory/sources.json').read_text())['sed'];source=Path(pin['source'])
manifest=json.loads((ROOT/'inventory/sed-tests.json').read_text())
assert fingerprint(source/manifest['registration']['path'])==manifest['registration']['sha256']
driver_sha256=fingerprint(Path(__file__))
selected=set(profile.options.commands)
assert selected<={Path(r['script']).name for r in manifest['scripts'] if r['reviewed']}
results=[]
for index,row in enumerate(manifest['scripts']):
    script=source/row['script'];assert fingerprint(script)==row['source_sha256']
    if not row['reviewed'] or (selected and script.name not in selected):continue
    assert script.suffix=='.sh', 'Perl originals require an explicit case-count assessment'
    outcomes={}
    for implementation in ('gnu','rboxc'):
        for instrument in (False,True):
            key=implementation+('-valgrind' if instrument else '')
            with tempfile.TemporaryDirectory(prefix='rboxc-sed-original-') as directory:
                work=Path(directory)
                for sub in ('sed','real','memory'):(work/sub).mkdir()
                binary=profile.oracle if implementation=='gnu' else profile.binary
                (work/'real/sed').symlink_to(binary)
                if instrument:
                    wrapper=work/'sed/sed'
                    wrapper.write_text('#!/bin/sh\nPATH='+str(work/'real')+':$PATH\nexport PATH\nexec /usr/bin/valgrind --leak-check=full --show-leak-kinds=all --track-fds=yes --trace-children=yes --log-file='+str(work/'memory/%p.log')+' sed "$@"\n')
                    wrapper.chmod(0o755)
                else:(work/'sed/sed').symlink_to(work/'real/sed')
                done=subprocess.run(['/bin/bash','-c','exec 9>&2; exec /bin/bash "$1"','sed-test',str(script)],
                    cwd=work,stdin=subprocess.DEVNULL,capture_output=True,timeout=row.get('timeout_seconds',300),
                    env={'PATH':str(work/'sed')+':/usr/bin:/bin','HOME':directory,'TMPDIR':directory,
                         'LC_ALL':'C','LANGUAGE':'C','TZ':'UTC0','srcdir':str(source),
                         'top_srcdir':str(source),'abs_top_srcdir':str(source),'abs_srcdir':str(source),
                         'abs_top_builddir':directory,'VERSION':pin['version'],'PACKAGE_BUGREPORT':'bug-sed@gnu.org',
                         'built_programs':'sed','PERL':'/usr/bin/perl','SHELL':'/bin/bash'})
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
    report={'scope':'Individually reviewed unchanged original shell assertions on the pinned native GNU and Rust candidate, with strict final-exec Valgrind checks.',**profile.metadata(),'driver_sha256':driver_sha256,'passed':sum(r['pass'] for r in results),'native_passed':sum(r['native_pass'] for r in results),'total':len(results),'state_counts':{s:sum(r['state']==s for r in results) for s in sorted({r['state'] for r in results})},'selected_scripts':sorted(selected),'registered_original_scripts':len(manifest['scripts']),'remaining':[r for r in manifest['scripts'] if not r['reviewed']],'results':results}
    assert fingerprint(Path(__file__))==driver_sha256
    profile.report.write_text(json.dumps(report,indent=2)+'\n')
raise SystemExit(any(not r['pass'] for r in results))
