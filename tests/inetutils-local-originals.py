#!/usr/bin/env python3
"""Run unchanged GNU interface formats and local hostname originals."""
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
sys.path[:0]=[str(ROOT/'scripts'),str(ROOT/'tests/gnu')]
from entry_provider_helpers import binary_path
spec=importlib.util.spec_from_file_location('reviewed',ROOT/'tests/gnu/reviewed-original.py')
runner=importlib.util.module_from_spec(spec);spec.loader.exec_module(runner)
profile=ComparisonProfile('inetutils-local-originals',oracle=binary_path(ROOT,'ifconfig'))
sources=Path('/opt/src/inetutils-2.8/tests')
helper=ROOT/'tests/gnu/inetutils-local-profile.py'
tools=ROOT/'build/gnu-inetutils/tests/tools.sh'
oracles={n:binary_path(ROOT,n) for n in ['ifconfig','dnsdomainname']}
inputs={str(p):fingerprint(p) for p in [Path(__file__),helper,tools,*oracles.values(),*[sources/(n+'.sh') for n in oracles]]}
parents={k:os.readlink('/proc/self/ns/'+k) for k in ['mnt','net','uts']}
host_pins={str(p):fingerprint(p) for p in [Path('/etc/hosts'),Path('/etc/nsswitch.conf')]}
hostname=os.uname().nodename
results=[]
for command in oracles:
 for instrument in [False,True]:
  outcomes={}
  for impl in ['gnu','rboxc']:
   with tempfile.TemporaryDirectory(prefix='rboxc-inetutils-local-') as directory:
    work=Path(directory);work.chmod(0o755);os.chown(work,65534,65534)
    shutil.copy2(sources/(command+'.sh'),work/(command+'.sh'));shutil.copy2(tools,work/'tools.sh')
    (work/'hosts').write_text('127.0.0.1 fixture.example.test fixture localhost\n::1 localhost\n')
    (work/'nsswitch.conf').write_text('passwd: files\ngroup: files\nhosts: files\nnetworks: files\nprotocols: files\nservices: files\n')
    real=work/'real';real.mkdir();binary=oracles[command] if impl=='gnu' else profile.binary
    shutil.copy2(binary,real/command);(work/command).symlink_to('real/'+command)
    memory=work/'memory';memory.mkdir();os.chown(memory,65534,65534)
    if instrument:
     (work/command).unlink()
     (work/command).write_text('#!/bin/sh\nexec '+shlex.join(['/usr/bin/valgrind','--leak-check=full','--show-leak-kinds=all','--track-fds=yes','--trace-children=yes','--log-file='+str(memory/'%p.log'),str(real/command)])+' "$@"\n')
     (work/command).chmod(0o755)
    env={'PATH':'/usr/bin:/bin','LC_ALL':'C','LANGUAGE':'C','TZ':'UTC0','HOME':directory,'USER':'nobody',
     command.upper():str(work/command),'VERBOSE':'1','EXEEXT':'','TEST_IPV4':'yes','TEST_IPV6':'yes','TARGET':'127.0.0.1'}
    config=work/'profile.json';config.write_text(json.dumps({'work':directory,'script':command+'.sh','environment':env,'parent_namespaces':parents}))
    done=subprocess.run(['/usr/bin/timeout','--kill-after=5s','120s','/usr/bin/unshare','--mount','--net','--uts',sys.executable,str(helper),str(config)],capture_output=True,timeout=130)
    log=profile.logs/f'{command}-{impl}-{int(instrument)}.log';log.write_bytes(done.stdout+done.stderr)
    text=log.read_text(errors='backslashreplace');ns=re.findall(r'^RBOXC_INETUTILS_LOCAL_PROFILE (.+)$',text,re.M)
    isolated=json.loads(ns[0]) if len(ns)==1 else None
    images=[]
    for path in sorted(memory.glob('*.log')):
     saved=profile.logs/f'{command}-{impl}-{path.name}';shutil.copy2(path,saved)
     images.append({'log':str(saved.relative_to(ROOT)),'sha256':fingerprint(saved),**runner.parse_memory_log(saved.read_text(),path.stem,True)})
    outcome={'status':done.returncode,'log':str(log.relative_to(ROOT)),'log_sha256':fingerprint(log),'private_profile':isolated,
     'binary_sha256':fingerprint(binary),'original_sha256':fingerprint(work/(command+'.sh')),'memory':images,
     'invocations':len(re.findall(r'^\+ '+re.escape(str(work/command))+r'(?: |$)',text,re.M))}
    outcome['behavior_pass']=done.returncode==0 and bool(isolated)
    if instrument:outcome['strict_memory_pass']=bool(images) and all(m['complete_exec_log'] and m['errors']==m['non_inherited_descriptors']==0 and not any(m['heap_bytes'].get(k,0) for k in ['definitely lost','indirectly lost','possibly lost']) for m in images)
    outcomes[impl]=outcome
    assert os.uname().nodename==hostname and all(os.readlink('/proc/self/ns/'+k)==v for k,v in parents.items())
    assert all(fingerprint(Path(p))==h for p,h in host_pins.items())
  results.append({'command':command,'instrumented':instrument,'outcomes':outcomes,'behavior_pass':all(o['behavior_pass'] for o in outcomes.values()),'strict_memory_pass':outcomes['rboxc'].get('strict_memory_pass')})
  profile.report.write_text(json.dumps({**profile.metadata(),'inputs':inputs,'host_files':host_pins,'results':results,'complete':len(results)==4,
   'scope':'Two unchanged Inetutils 2.8 originals in private network, mount and UTS namespaces as nobody, with loopback only and files-only local name resolution. Interface formatting and invalid prefix argument rejection; hostname help/version and local lookup.'},indent=2)+'\n')
  print(command,int(instrument),[(i,o['status'],o['invocations'],o.get('strict_memory_pass')) for i,o in outcomes.items()],flush=True)
assert all(fingerprint(Path(p))==h for p,h in inputs.items())
raise SystemExit(not all(r['behavior_pass'] for r in results))
