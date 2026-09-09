#!/usr/bin/env python3
"""Compare the original private-loopback inetd daemon and SIGHUP service test."""
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
from comparison_profile import ComparisonProfile, fingerprint
ROOT=Path(__file__).resolve().parents[1]
sys.path[:0]=[str(ROOT/'scripts'),str(ROOT/'tests/gnu')]
from entry_provider_helpers import binary_path
spec=importlib.util.spec_from_file_location('reviewed',ROOT/'tests/gnu/reviewed-original.py')
runner=importlib.util.module_from_spec(spec);spec.loader.exec_module(runner)
profile=ComparisonProfile('inetutils-inetd-original',oracle=binary_path(ROOT,'inetd'))
source=Path('/opt/src/inetutils-2.8/tests/inetd.sh')
build=ROOT/'build/gnu-inetutils/tests'
inputs=[source,build/'tools.sh',build/'addrpeek',build/'tcpget',ROOT/'tests/gnu/inetd-net-profile.py',Path(__file__)]
pins={str(p):fingerprint(p) for p in inputs}
parents={k:os.readlink('/proc/self/ns/'+k) for k in ['mnt','net','pid']}
results=[]
for instrument in [False,True]:
 outcomes={}
 for impl in ['gnu','rboxc']:
  with tempfile.TemporaryDirectory(prefix='rboxc-inetd-') as directory:
   work=Path(directory);work.chmod(0o755);os.chown(work,65534,65534)
   for name,p in [('inetd.sh',source),('tools.sh',build/'tools.sh'),('addrpeek',build/'addrpeek'),('tcpget',build/'tcpget')]:
    shutil.copy2(p,work/name)
   binary=profile.oracle if impl=='gnu' else profile.binary
   real=work/'real';real.mkdir()
   shutil.copy2(binary,real/'inetd')
   (work/'inetd').symlink_to('real/inetd')
   fixture=work/'fixture';fixture.mkdir();os.chown(fixture,65534,65534)
   memory=work/'memory';memory.mkdir();os.chown(memory,65534,65534)
   if instrument:
    (work/'inetd').unlink()
    (work/'inetd').write_text('#!/bin/sh\nexec '+shlex.join(['/usr/bin/valgrind','--leak-check=full','--show-leak-kinds=all','--track-fds=yes','--trace-children=yes','--log-file='+str(memory/'%p.log'),str(real/'inetd')])+' "$@"\n')
    (work/'inetd').chmod(0o755)
   env={'PATH':'/usr/bin:/bin','LC_ALL':'C','LANGUAGE':'C','TZ':'UTC0','HOME':directory,'USER':'nobody',
        'INETD':str(work/'inetd'),'ADDRPEEK':str(work/'addrpeek'),'TCPGET':str(work/'tcpget'),
        'TARGET':'127.0.0.1','TARGET6':'::1','TEST_IPV4':'yes','TEST_IPV6':'yes',
        'IU_TESTDIR':str(work/'fixture'),'NOCLEAN':'1','VERBOSE':'1','EXEEXT':''}
   config=work/'profile.json';config.write_text(json.dumps({'work':directory,'parent_namespaces':parents,'environment':env}))
   args=['/usr/bin/timeout','--kill-after=5s','90s','/usr/bin/unshare','--mount','--net','--pid','--fork',
         sys.executable,str(ROOT/'tests/gnu/inetd-net-profile.py'),str(config)]
   done=subprocess.run(args,capture_output=True,timeout=100)
   log=profile.logs/f'{impl}-{"valgrind" if instrument else "native"}.log';log.write_bytes(done.stdout+done.stderr)
   text=log.read_text(errors='backslashreplace')
   rows=re.findall(r'^RBOXC_INETD_PRIVATE_PROFILE (.+)$',text,re.M)
   isolated=json.loads(rows[0]) if len(rows)==1 else None
   children=re.findall(r'^RBOXC_INETD_CHILDREN_REAPED (.+)$',text,re.M)
   outcome={'status':done.returncode,'log':str(log.relative_to(ROOT)),'log_sha256':fingerprint(log),
            'private_profile':isolated,'children_reaped':json.loads(children[0]) if len(children)==1 else None,
            'original_sha256':fingerprint(work/'inetd.sh'),'binary_sha256':fingerprint(binary),
            'service_connections':len(re.findall(r'^\+ '+re.escape(str(work/'tcpget'))+r' ',text,re.M)),
            'reload_signals':len(re.findall(r'^\+ kill -HUP ',text,re.M)), 'memory':[]}
   for path in sorted(memory.glob('*.log')):
    saved=profile.logs/f'{impl}-{path.name}';shutil.copy2(path,saved)
    parsed=runner.parse_memory_log(saved.read_text(),path.stem,True)
    outcome['memory'].append({'log':str(saved.relative_to(ROOT)),'sha256':fingerprint(saved),**parsed})
   outcome['behavior_pass']=done.returncode==0 and bool(isolated) and outcome['children_reaped'] is not None and outcome['service_connections']==10 and outcome['reload_signals']==5
   if instrument:
    outcome['strict_memory_pass']=bool(outcome['memory']) and all(m['complete_exec_log'] and m['errors']==m['non_inherited_descriptors']==0 and not any(m['heap_bytes'].get(k,0) for k in ['definitely lost','indirectly lost','possibly lost']) for m in outcome['memory'])
   outcomes[impl]=outcome
   assert all(os.readlink('/proc/self/ns/'+k)==v for k,v in parents.items())
 results.append({'instrumented':instrument,'outcomes':outcomes,'behavior_pass':all(r['behavior_pass'] for r in outcomes.values()),'strict_memory_pass':outcomes['rboxc'].get('strict_memory_pass')})
 report={**profile.metadata(),'scope':'Whole original inetd.sh: local IPv4/IPv6 addrpeek services and five daemon reloads in private network/PID/mount namespaces as nobody. Only loopback exists. No authentication or external connection. Strict Valgrind findings remain separate.',
         'inputs':pins,'results':results,'complete':len(results)==2}
 profile.report.write_text(json.dumps(report,indent=2)+'\n')
 print('inetd', 'valgrind' if instrument else 'native', [(i,r['status'],r['service_connections'],r['reload_signals'],r.get('strict_memory_pass')) for i,r in outcomes.items()],flush=True)
assert all(fingerprint(Path(p))==v for p,v in pins.items())
raise SystemExit(not all(r['behavior_pass'] for r in results))
