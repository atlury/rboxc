#!/usr/bin/env python3
"""Execute reviewed original GNU Patch scripts without changing assertions."""
# SPDX-License-Identifier: GPL-3.0-or-later
import importlib.util
import json
import os
from pathlib import Path
import re
import shlex
import shutil
import subprocess
import sys
import tempfile
from comparison_profile import ComparisonProfile,fingerprint

ROOT=Path(__file__).resolve().parents[1]
sys.path.insert(0,str(ROOT/'tests/gnu'))
spec=importlib.util.spec_from_file_location('reviewed',ROOT/'tests/gnu/reviewed-original.py')
runner=importlib.util.module_from_spec(spec);spec.loader.exec_module(runner)
profile=ComparisonProfile('patch-original',oracle=ROOT/'build/gnu-patch/src/patch')
source=Path(json.loads((ROOT/'inventory/sources.json').read_text())['patch']['source'])
manifest=json.loads((ROOT/'inventory/patch-tests.json').read_text())
selected=[r for r in manifest['inputs'] if r['reviewed']]
assert selected and len({r['target'] for r in selected})==len(selected)
helpers={n:ROOT/'build/gnu-coreutils/src/coreutils' for n in ('cat','rm','echo','chmod','ls','cut','mktemp','touch','ln','mkdir','expr','seq','mv','stat','mkfifo','cp')}
helpers['ed']=Path('/usr/bin/ed')
assert '#define EDITOR_PROGRAM "/usr/bin/ed"' in (ROOT/'build/gnu-patch/config.h').read_text()
helpers['grep']=ROOT/'build/gnu-grep/src/grep'
helpers.update(diff=ROOT/'build/gnu-diffutils/src/diff',sed=ROOT/'build/gnu-sed/sed/sed')
inputs={p:fingerprint(p) for p in {ROOT/'build/gnu-patch/config.h',source/'tests/Makefile.am',source/'tests/test-lib.sh',Path(__file__),Path('/bin/sh').resolve(),*helpers.values(),profile.oracle}}
for row in selected:inputs[source/row['path']]=row['sha256']
assert all(fingerprint(p)==h for p,h in inputs.items())
registration=(source/'tests/Makefile.am').read_text()
xfail_match=re.search(r'^XFAIL_TESTS = ((?:.*\\\n)*.*)',registration,re.M)
assert xfail_match
expected_failures=set(xfail_match[1].replace('\\\n',' ').split())
assert expected_failures=={'context-format','dash-o-append'}
results=[]
for row in selected:
    name=row['target'];outcomes={}
    for implementation in ('gnu','rboxc'):
        for instrument in (False,True):
            key=implementation+('-valgrind' if instrument else '')
            saved=profile.logs/(name+'-'+key);saved.mkdir()
            with tempfile.TemporaryDirectory(prefix='rboxc-patch-original-') as directory:
                work=Path(directory)
                for n in ('exec','deps','memory','tests'): (work/n).mkdir()
                binary=profile.oracle if implementation=='gnu' else profile.binary
                helper_paths=helpers.copy();copied_inputs={};identity={}
                if row.get('unprivileged'):
                    assert os.geteuid()==0, 'the private unprivileged profile requires a root launcher'
                    (work/'binaries').mkdir()
                    for n,p in enumerate(sorted({binary,*helpers.values(),*profile.runtime_helpers},key=str)):
                        copy=work/'binaries'/str(n);shutil.copy2(p,copy)
                        assert fingerprint(copy)==fingerprint(p)
                        copied_inputs[str(p)]={'path':str(copy),'sha256':fingerprint(copy)}
                    helper_paths={n:Path(copied_inputs[str(p)]['path']) for n,p in helpers.items()}
                    binary=Path(copied_inputs[str(binary)]['path'])
                    identity={'user':65534,'group':65534,'extra_groups':[]}
                for n,p in helper_paths.items():(work/'deps'/n).symlink_to(p)
                (work/'exec/patch').symlink_to(binary)
                argv=['patch']
                whole_driver=row.get('launch_profile')=='instrument-original-driver'
                if instrument and not whole_driver:argv=['/usr/bin/valgrind','--leak-check=full','--show-leak-kinds=all',
                    '--track-fds=yes','--trace-children=yes','--log-file='+str(work/'memory/%p.log'),*argv]
                wrapper=work/'awk-wrapper'
                wrapper.write_text('#!/bin/sh\nexec '+shlex.join(argv)+' "$@"\n');wrapper.chmod(0o755)
                command=['/bin/sh',str(source/row['path'])]
                env={'PATH':str(work/'exec')+':'+str(work/'deps')+':/usr/bin:/bin','HOME':directory,
                     'TMPDIR':directory,'LC_ALL':'C','LANGUAGE':'C','TZ':'UTC0',
                     'srcdir':str(source/'tests'),'abs_top_builddir':directory,'PATCH':str(wrapper)}
                if whole_driver:
                    env['PATCH']=str(work/'exec/patch')
                    if instrument:
                        command=['/usr/bin/valgrind','--leak-check=full','--show-leak-kinds=all',
                            '--track-fds=yes','--trace-children=yes','--log-file='+str(work/'memory/%p.log'),*command]
                if identity:
                    for p in [work,*(p for p in work.iterdir() if p.is_dir())]:os.chown(p,65534,65534)
                done=subprocess.run(command,cwd=work,env=env,stdin=subprocess.DEVNULL,
                    stdout=subprocess.PIPE,stderr=subprocess.STDOUT,timeout=300,**identity)
                (saved/'driver.log').write_bytes(done.stdout)
                counts=re.findall(rb'(\d+) tests \((\d+) passed, (\d+) failed\)',done.stdout)
                passed=(done.returncode==0 and len(counts)==1 and int(counts[0][0])>0
                        and counts[0][0]==counts[0][1] and int(counts[0][2])==0)
                shutil.copytree(work/'memory',saved/'memory')
                logs=[]
                for p in sorted((saved/'memory').glob('*.log')):
                    text=p.read_text();commands=re.findall(r'^==[0-9]+== Command: (.*)$',text,re.M)
                    image_names=[shlex.split(c)[0] for c in commands]
                    if whole_driver:
                        assert all(n in {str(work/'exec/patch'),'/bin/sh',*(str(work/'deps'/h) for h in helpers),*(['/usr/bin/ed'] if row.get('native_editor') else [])} for n in image_names)
                        assert len(image_names)<=1
                        role='patch' if image_names==[str(work/'exec/patch')] else 'native-test-helper'
                    else:
                        if row.get('native_editor') and image_names in [['/bin/sh'],['/usr/bin/ed']]:
                            role='native-test-helper'
                        else:
                            assert image_names==['patch'], image_names
                            role='patch'
                    logs.append({'role':role,**runner.parse_memory_log(text,p.stem,exec_only=True),
                        'log':str(p.relative_to(ROOT)),'sha256':fingerprint(p)})
                patch_logs=[m for m in logs if m['role']=='patch']
                clean=bool(patch_logs) and all(m['complete_exec_log'] and m['errors']==0
                    and m['non_inherited_descriptors']==0 and not any(m['heap_bytes'].get(k,0)
                    for k in ('definitely lost','indirectly lost','possibly lost')) for m in patch_logs)
                outcomes[key]={'status':done.returncode,'assertions_pass':passed,'assertion_counts':[[int(n) for n in row] for row in counts],
                    'identity':identity,'copied_inputs':copied_inputs,
                    'driver_log':str((saved/'driver.log').relative_to(ROOT)),'driver_log_sha256':fingerprint(saved/'driver.log'),
                    'memory':logs,'memory_clean':clean if instrument else None}
    expected_failure=name in expected_failures
    reference=outcomes['gnu']
    for o in outcomes.values():
        o['expectation_matches']=(o['assertions_pass'] if not expected_failure else
            o['status']==reference['status']==1 and o['assertion_counts']==reference['assertion_counts']
            and len(o['assertion_counts'])==1 and o['assertion_counts'][0][2]>0
            and (ROOT/o['driver_log']).read_bytes()==(ROOT/reference['driver_log']).read_bytes())
    passed=all(o['expectation_matches'] for o in outcomes.values()) and outcomes['rboxc-valgrind']['memory_clean']
    results.append({'selection':name,'expected_failure':expected_failure,'source':row['path'],'source_sha256':row['sha256'],'launch_profile':row.get('launch_profile','instrument-patch'),'native_editor':row.get('native_editor',False),'unprivileged':row.get('unprivileged',False),
                    'pass':passed,'outcomes':outcomes})
    assert all(fingerprint(p)==h for p,h in inputs.items())
    report={**profile.metadata(),'scope':'Reviewed unchanged GNU Patch scripts run in private fixtures with original assertions. Each Patch invocation is instrumented. Original Ed-format selections retain the configured /usr/bin/ed test dependency, hashed separately; read-only tests run as uid/gid 65534 against private byte-identical executable copies. The diagnostic-name selection instruments the original shell driver to retain the direct Patch path; native shell/helper logs are preserved separately and are not candidate process results.',
        'inputs':{str(p):h for p,h in inputs.items()},'driver_sha256':fingerprint(Path(__file__)),
        'expected_failure_selections':sorted(expected_failures & {r['target'] for r in selected}),
        'ordinary_passed':sum(r['pass'] and not r['expected_failure'] for r in results),
        'expected_failures_matched':sum(r['pass'] and r['expected_failure'] for r in results),
        'planned_total':len(selected),'complete':len(results)==len(selected),
        'passed':sum(r['pass'] for r in results),'total':len(results),'results':results}
    profile.report.write_text(json.dumps(report,indent=2)+'\n')
    print(('XFAIL-MATCH' if expected_failure else 'PASS') if passed else 'OPEN',name,flush=True)
raise SystemExit(report['passed']!=report['total'])
