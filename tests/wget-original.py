#!/usr/bin/env python3
"""Execute reviewed original GNU Wget scripts without changing assertions."""
# SPDX-License-Identifier: GPL-3.0-or-later
import importlib.util
import json
import os
from pathlib import Path
import re
import shlex
import signal
import shutil
import subprocess
import sys
import tempfile
from comparison_profile import ComparisonProfile,fingerprint

ROOT=Path(__file__).resolve().parents[1]
sys.path.insert(0,str(ROOT/'tests/gnu'))
spec=importlib.util.spec_from_file_location('reviewed',ROOT/'tests/gnu/reviewed-original.py')
runner=importlib.util.module_from_spec(spec);spec.loader.exec_module(runner)
profile=ComparisonProfile('wget-original',oracle=ROOT/'build/gnu-wget/src/wget')
source=Path(json.loads((ROOT/'inventory/sources.json').read_text())['wget']['source'])
manifest=json.loads((ROOT/'inventory/wget-tests.json').read_text())
selected=[r for r in manifest['inputs'] if r['reviewed']]
assert selected and len({r['target'] for r in selected})==len(selected)
helpers={}
inputs={p:fingerprint(p) for p in {source/'tests/Makefile.am',Path(__file__),Path('/usr/bin/perl'),Path('/bin/sh').resolve(),profile.oracle,*list((source/'tests').glob('*.pm'))}}
for row in selected:inputs[source/row['path']]=row['sha256']
if any(r.get('fixture_profile')=='private-tls-log' for r in selected):
    for p in (source/'tests/certs').rglob('*'):
        if p.is_file():inputs[p]=fingerprint(p)
assert all(fingerprint(p)==h for p,h in inputs.items())
results=[]
for row in selected:
    name=row['target'];outcomes={}
    for implementation in ('gnu','rboxc'):
        for instrument in (False,True):
            key=implementation+('-valgrind' if instrument else '')
            saved=profile.logs/(name+'-'+key);saved.mkdir()
            with tempfile.TemporaryDirectory(prefix='rboxc-wget-original-') as directory:
                work=Path(directory)
                for n in ('exec','deps','memory','tests'): (work/n).mkdir()
                for n,p in helpers.items():(work/'deps'/n).symlink_to(p)
                (work/'exec/wget').symlink_to(profile.oracle if implementation=='gnu' else profile.binary)
                argv=['wget']
                if instrument:argv=['/usr/bin/valgrind','--leak-check=full','--show-leak-kinds=all',
                    '--track-fds=yes','--trace-children=yes','--log-file='+str(work/'memory/%p.log'),*argv]
                wrapper=work/'awk-wrapper'
                wrapper.write_text('#!/bin/sh\nexec '+shlex.join(argv)+' "$@"\n');wrapper.chmod(0o755)
                command=['/usr/bin/perl','-I'+str(source/'tests'),str(source/row['path'])]
                env={'PATH':str(work/'exec')+':'+str(work/'deps')+':/usr/bin:/bin','HOME':directory,
                     'TMPDIR':directory,'LC_ALL':'C','LANGUAGE':'C','TZ':'UTC0',
                     'srcdir':str(source/'tests'),'WGET_PATH':str(wrapper),'VALGRIND_TESTS':'0'}
                private_inputs={}
                if row.get('fixture_profile')=='private-tls-log':
                    copies=work/'source';copies.mkdir()
                    for p in (source/'tests').glob('*.pm'):shutil.copy2(p,copies/p.name)
                    shutil.copytree(source/'tests/certs',copies/'certs')
                    helper=copies/'SSLServer.pm';text=helper.read_text()
                    assert text.count('/tmp/wgetserver.log')==1
                    helper.write_text(text.replace('/tmp/wgetserver.log',str(work/'server.log')))
                    for p in copies.rglob('*'):
                        if p.is_file():private_inputs[str(p.relative_to(copies))]=fingerprint(p)
                    command[1]='-I'+str(copies);env['srcdir']=str(copies)
                    shutil.copy2(helper,saved/'SSLServer.pm')
                timed_out=False
                process=subprocess.Popen(command,cwd=work,env=env,stdin=subprocess.DEVNULL,
                    stdout=subprocess.PIPE,stderr=subprocess.STDOUT,start_new_session=True)
                try:
                    output,_=process.communicate(timeout=90)
                except subprocess.TimeoutExpired:
                    timed_out=True
                    os.killpg(process.pid,signal.SIGTERM)
                    try:output,_=process.communicate(timeout=5)
                    except subprocess.TimeoutExpired:
                        os.killpg(process.pid,signal.SIGKILL);output,_=process.communicate()
                done=subprocess.CompletedProcess(command,process.returncode,output)
                if (work/'server.log').exists():shutil.copy2(work/'server.log',saved/'server.log')
                for name_,expected in private_inputs.items():assert fingerprint(copies/name_)==expected

                (saved/'driver.log').write_bytes(done.stdout)
                successful=done.stdout.count(b'Test successful.')
                failures=[line.decode() for line in done.stdout.splitlines() if line.startswith(b'Test failed:')]
                counts=[(successful+len(failures),successful,len(failures))]
                expected_skip=row.get('expected_feature_skip')
                skip_matches=bool(expected_skip) and done.returncode==77 and successful==0 and ("Skipped test: Wget misses feature '"+expected_skip+"'").encode() in done.stdout and re.search(rb'^\s+'+expected_skip.encode()+rb'=0$',done.stdout,re.M) is not None
                upstream_skip=row.get('expected_upstream_skip')
                upstream_skip_matches=bool(upstream_skip) and done.returncode==77 and successful==0 and not failures and done.stdout==b'Setting --no-config (noconfig) to 1\n'
                ordinary_matches=done.returncode==0 and successful==row.get('success_markers',1) and failures==row.get('expected_phase_failures',[]) and all(s.encode() in done.stdout for s in row.get('required_output',[]))
                passed=not timed_out and (skip_matches if expected_skip else upstream_skip_matches if upstream_skip else ordinary_matches)
                shutil.copytree(work/'memory',saved/'memory')
                logs=[]
                for p in sorted((saved/'memory').glob('*.log')):
                    text=p.read_text();commands=re.findall(r'^==[0-9]+== Command: (.*)$',text,re.M)
                    assert len(commands)==1 and commands[0].split()[0]=='wget'
                    logs.append({**runner.parse_memory_log(text,p.stem,exec_only=True),
                        'log':str(p.relative_to(ROOT)),'sha256':fingerprint(p)})
                clean=bool(logs) and all(m['complete_exec_log'] and m['errors']==0
                    and m['non_inherited_descriptors']==0 and not any(m['heap_bytes'].get(k,0)
                    for k in ('definitely lost','indirectly lost','possibly lost')) for m in logs)
                outcomes[key]={'status':done.returncode,'timed_out':timed_out,'private_inputs':private_inputs,'fixture_raw':{str(p.relative_to(ROOT)):fingerprint(p) for p in (saved/'SSLServer.pm',saved/'server.log') if p.exists()},'assertions_pass':passed and not expected_skip and not upstream_skip,'upstream_skip_matches':upstream_skip_matches,'phase_failures':failures,'expectation_matches':passed,'feature_skip_matches':skip_matches,'assertion_counts':[[int(n) for n in row] for row in counts],
                    'driver_log':str((saved/'driver.log').relative_to(ROOT)),'driver_log_sha256':fingerprint(saved/'driver.log'),
                    'memory':logs,'memory_clean':clean if instrument else None}
    expected_processes=row.get('expected_processes')
    if expected_processes:
        assert all(len(outcomes[k]['memory'])==expected_processes for k in ('gnu-valgrind','rboxc-valgrind'))
    passed=all(o['expectation_matches'] for o in outcomes.values()) and outcomes['rboxc-valgrind']['memory_clean']
    results.append({'selection':name,'expected_feature_skip':row.get('expected_feature_skip'),'fixture_profile':row.get('fixture_profile'),'expected_upstream_skip':row.get('expected_upstream_skip'),'source':row['path'],'source_sha256':row['sha256'],
                    'pass':passed,'outcomes':outcomes})
    assert all(fingerprint(p)==h for p,h in inputs.items())
    report={**profile.metadata(),'scope':'Reviewed unchanged GNU Wget Perl scripts serve fixed HTTP responses or FTP file listings/content on their own localhost server and verify status, resumed content and downloaded filenames. Local input/output error cases preserve the original assertions. HTTPS fixtures copy the original modules and certificates, changing only the helper log path to a private location. GNU feature-gate skips are recorded separately and do not count as tested optional behavior. All Wget processes, including feature probes, are instrumented without upstream suppressions; server helpers are not instrumented.',
        'inputs':{str(p):h for p,h in inputs.items()},'driver_sha256':fingerprint(Path(__file__)),
        'ordinary_passed':sum(r['pass'] and not r['expected_feature_skip'] and not r['expected_upstream_skip'] for r in results),
        'feature_skips_matched':sum(r['pass'] and bool(r['expected_feature_skip']) for r in results),
        'upstream_skips_matched':sum(r['pass'] and bool(r['expected_upstream_skip']) for r in results),
        'planned_total':len(selected),'complete':len(results)==len(selected),
        'passed':sum(r['pass'] for r in results),'total':len(results),'results':results}
    profile.report.write_text(json.dumps(report,indent=2)+'\n')
    print(('FEATURE-SKIP' if row.get('expected_feature_skip') else 'UPSTREAM-SKIP' if row.get('expected_upstream_skip') else 'PASS') if passed else 'OPEN',name,flush=True)
raise SystemExit(report['passed']!=report['total'])
