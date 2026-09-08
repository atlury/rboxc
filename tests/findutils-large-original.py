#!/usr/bin/env python3
"""Run reviewed, unchanged Findutils DejaGNU tests with private executable paths."""
# SPDX-License-Identifier: GPL-3.0-or-later
import importlib.util
import json
import os
from pathlib import Path
import re
import shlex
import shutil
import signal
import subprocess
import sys
import tempfile
from comparison_profile import ComparisonProfile, fingerprint

ROOT=Path(__file__).resolve().parents[1]
sys.path.insert(0,str(ROOT/'tests/gnu'))
spec=importlib.util.spec_from_file_location('reviewed',ROOT/'tests/gnu/reviewed-original.py')
runner=importlib.util.module_from_spec(spec);spec.loader.exec_module(runner)
profile=ComparisonProfile('findutils-large-original',oracle=ROOT/'build/gnu-findutils/find/find',selections=True)
source=Path(json.loads((ROOT/'inventory/sources.json').read_text())['findutils']['source'])
selections={'find/execdir-multiple':(8,[]),
            'find/posix:exec-nogaps':(2,[])}
def selected_path(selection):
 command,name=selection.split('/')
 if ':' in name:category,name=name.split(':');category=command+'.'+category
 else:category=command+'.gnu'
 return command,category,name
selected=profile.options.commands or list(selections)
assert set(selected)<=set(selections)
oracles={n:ROOT/'build/gnu-findutils'/n/n for n in ('find','xargs','locate')}
helpers={n:ROOT/'build/gnu-coreutils/src/coreutils' for n in ('cat','rm','mkdir','chmod','touch','sort','echo','basename','cp','ln','true','false','sleep','ls','mv','mktemp','cut','id','date','printf','dd','rmdir')}
helpers.update({n:ROOT/'build/gnu-diffutils/src'/n for n in ('cmp','diff')})
helpers['sed']=ROOT/'build/gnu-sed/sed/sed'
frcode=ROOT/'build/gnu-findutils/locate/frcode'
updatedb=ROOT/'build/gnu-findutils/locate/updatedb'
inputs={p:fingerprint(p) for p in {*oracles.values(),*helpers.values(),frcode,updatedb,Path('/usr/bin/sort'),Path('/usr/bin/locale'),source/'locate/updatedb.sh',Path('/bin/sh'),Path('/usr/bin/sh'),Path('/bin/sh').resolve(),Path(__file__)}}
for selection in selected:
 command,category,name=selected_path(selection);suite=source/command/'testsuite'
 for p in [suite/'config/unix.exp',suite/category/(name+'.exp'),*[(suite/'inputs'/n).resolve() for n in selections[selection][1]]]:inputs[p]=fingerprint(p)
 for suffix in ('xo','xe'):
  p=suite/category/(name+'.'+suffix)
  if p.exists():inputs[p]=fingerprint(p)
results=[]
def memory_clean(logs):
 return bool(logs) and all(m['complete_exec_log'] and m['errors']==0 and m['non_inherited_descriptors']==0 and not any(m['heap_bytes'].get(k,0) for k in ('definitely lost','indirectly lost','possibly lost')) for m in logs)
for index,selection in enumerate(selected):
 command,category,name=selected_path(selection);expected,_=selections[selection];outcomes={}
 for implementation in ('gnu','rboxc'):
  for instrument in (False,True):
   key=implementation+('-valgrind' if instrument else '')
   saved=profile.logs/(f'{index:02}-'+command+'-'+name+'-'+key);saved.mkdir()
   with tempfile.TemporaryDirectory(prefix='rboxc-findutils-original-') as directory:
    work=Path(directory);deps=work/'deps';deps.mkdir();executables=work/'exec';executables.mkdir();memory=work/'memory';memory.mkdir()
    native=work/'native';native.mkdir();copies={}
    for helper,binary in helpers.items():
     copied=native/binary.name
     if not copied.exists():shutil.copy2(binary,copied)
     assert fingerprint(copied)==inputs[binary]
     copies[copied]=inputs[binary]
     (deps/helper).symlink_to(copied)
    for applet in oracles:
     parent=work/applet;parent.mkdir();(parent/'testsuite').mkdir()
     original_binary=oracles[applet] if implementation=='gnu' else profile.binary
     copied=executables/applet;shutil.copy2(original_binary,copied)
     copies[copied]=fingerprint(original_binary)
     assert fingerprint(copied)==copies[copied]
     argv=[str(executables/applet)]
     if instrument and applet==command:
      child_options=['--trace-children=no','--child-silent-after-fork=yes'] if name=='exec-nogaps' else ['--trace-children=yes']
      argv=['/usr/bin/valgrind','--leak-check=full','--show-leak-kinds=all','--track-fds=yes',*child_options,'--log-file='+str(memory/'%p.log'),*argv]
     wrapper=parent/applet;wrapper.write_text('#!/bin/sh\nexec '+shlex.join(argv)+' "$@"\n');wrapper.chmod(0o755)
    (work/'find/ftsfind.o').write_bytes(b'') # Unchanged harness checks existence only.
    (work/'xargs/xargs.o').symlink_to(ROOT/'build/gnu-findutils/xargs/xargs.o')
    (work/'locate/frcode').symlink_to(frcode)
    # Native updatedb and frcode are fixture helpers, not translated commands.
    # Database construction uses only each original script's private paths.
    (work/'locate/updatedb').symlink_to(updatedb)
    cwd=work/command/'testsuite';suite=source/command/'testsuite'
    (cwd/'site.exp').write_text('set srcdir "'+str(suite)+'"\nset objdir "'+str(cwd)+'"\nset build_triplet x86_64-pc-linux-gnu\nset host_triplet x86_64-pc-linux-gnu\n')
    env={'PATH':str(deps)+':/usr/bin:/bin','HOME':directory,'LC_ALL':'C','LANGUAGE':'C','TZ':'UTC0','TMPDIR':directory,'DEJAGNU':'/dev/null','TERM':'dumb'}
    invocation=['/usr/bin/runtest','--tool',command,'--srcdir',str(suite),category+'/'+name+'.exp']
    nss=None
    if command=='find' and (name.startswith('user-') or name.startswith('group-')):
     host_nss=Path('/etc/nsswitch.conf').read_text()
     local_nss=re.sub(r'^(passwd|group|shadow|gshadow|initgroups):.*$',r'\1: files',host_nss,flags=re.M)
     config=work/'nsswitch.conf';config.write_text(local_nss)
     nss={'profile':'private-mount-local-files','host_sha256':fingerprint(Path('/etc/nsswitch.conf')),'private_sha256':fingerprint(config)}
     invocation=['/usr/bin/unshare','--mount','--propagation','private','/bin/sh','-c','/usr/bin/mount --bind "$1" /etc/nsswitch.conf || exit 77; shift; exec "$@"','local-nss',str(config),*invocation]
    assert os.geteuid()==0 and nss is None
    for entry in [work,*work.rglob('*')]:
     if not entry.is_symlink():os.chown(entry,65534,65534)
    timeout=1800 if name=='exec-nogaps' else 900
    process=subprocess.Popen(invocation,cwd=cwd,env=env,user=65534,group=65534,extra_groups=[],stdout=subprocess.PIPE,stderr=subprocess.STDOUT,start_new_session=True)
    timed_out=False
    try:
     output,_=process.communicate(timeout=timeout)
    except subprocess.TimeoutExpired:
     timed_out=True
     os.killpg(process.pid,signal.SIGKILL)
     output,_=process.communicate()
    done=subprocess.CompletedProcess(invocation,process.returncode,output)
    (saved/'execution.json').write_text(json.dumps({'timeout_seconds':timeout,'timed_out':timed_out,'status':done.returncode},indent=2)+'\n')
    assert all(fingerprint(p)==h for p,h in copies.items()),'copied executable changed'

    if nss:assert Path('/etc/nsswitch.conf').read_text()==host_nss
    (saved/'driver.log').write_bytes(done.stdout)
    for suffix in ('sum','log'):
     p=cwd/(command+'.'+suffix)
     if p.exists():shutil.copy2(p,saved/p.name)
    shutil.copytree(memory,saved/'memory')
    summary=(saved/(command+'.sum')).read_text() if (saved/(command+'.sum')).exists() else ''
    assertions=re.findall(r'^(PASS|FAIL|XFAIL|XPASS|UNRESOLVED|UNSUPPORTED|UNTESTED|ERROR): (.*)$',summary,re.M)
    logs=[{**runner.parse_memory_log(p.read_text(),p.stem,exec_only=True),'log':str(p.relative_to(ROOT)),'sha256':fingerprint(p)} for p in sorted((saved/'memory').glob('*.log'))]
    for log in logs:
     commands=re.findall(r'^==\d+== Command: (.*)$',(ROOT/log['log']).read_text(),re.M)
     assert commands
     log['command']=commands[-1]
     executable=commands[-1].split(' ',1)[0]
     log['applet_process']=bool(re.fullmatch(r'/tmp/rboxc-findutils-original-[^/]+/exec/'+command,executable))
     log['pass']=memory_clean([log])
    own=[m for m in logs if m['applet_process']]
    outcomes[key]={'timed_out':timed_out,'timeout_seconds':timeout,'status':done.returncode,'assertions':assertions,'assertions_pass':not timed_out and done.returncode==0 and len(assertions)==expected and all(r[0]=='PASS' for r in assertions),
                   'memory':logs,'memory_clean':memory_clean(logs) if instrument else None,
                   'applet_memory_clean':memory_clean(own) if instrument else None,'nss_profile':nss,
                   'driver_log':str((saved/'driver.log').relative_to(ROOT)),'driver_log_sha256':fingerprint(saved/'driver.log')}
 row={'selection':selection,'expected_assertions':expected,'uid':65534,'gid':65534,'child_instrumentation':name!='exec-nogaps','outcomes':outcomes}
 row['assertions_pass']=all(v['assertions_pass'] for v in outcomes.values()) and all(v['assertions']==outcomes['gnu']['assertions'] for v in outcomes.values())
 row['pass']=row['assertions_pass'] and outcomes['rboxc-valgrind']['memory_clean']
 row['applet_assertions_and_memory_passed']=row['assertions_pass'] and outcomes['rboxc-valgrind']['applet_memory_clean']
 results.append(row)
 assert all(fingerprint(p)==h for p,h in inputs.items()),'test input changed'
 report={'scope':'Unchanged original Findutils permission and large-exec tests run as uid/gid 65534 in owned private trees. Executables are copied by verified byte identity so /root permissions do not interfere. Every applet invocation is instrumented in Valgrind profiles. The 7200-file exec-nogaps case uses trace-children=no and child-silent-after-fork=yes: its native helper executables are pinned but are not instrumented; other cases retain full child tracing.',
         **profile.metadata(),'inputs':{str(p):h for p,h in inputs.items()},'passed':sum(r['pass'] for r in results),'total':len(results),
         'applet_assertions_and_memory_passed':sum(r['applet_assertions_and_memory_passed'] for r in results),
         'assertions_passed':sum(r['expected_assertions'] for r in results if r['assertions_pass']),'results':results}
 profile.report.write_text(json.dumps(report,indent=2)+'\n')
 print('PASS' if row['pass'] else 'OPEN',selection,flush=True)
raise SystemExit(any(not r['pass'] for r in results))
