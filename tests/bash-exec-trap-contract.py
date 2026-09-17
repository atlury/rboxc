#!/usr/bin/env python3
"""Check trap restoration through finite unsuccessful exec operations."""
# SPDX-License-Identifier: GPL-3.0-or-later
import importlib.util
import json
from pathlib import Path
import re
import subprocess
import sys
import tempfile
from comparison_profile import ComparisonProfile,fingerprint
ROOT=Path(__file__).resolve().parents[1]
sys.path[:0]=[str(ROOT/'tests/gnu')]
spec=importlib.util.spec_from_file_location('reviewed',ROOT/'tests/gnu/reviewed-original.py')
runner=importlib.util.module_from_spec(spec);spec.loader.exec_module(runner)
profile=ComparisonProfile('bash-exec-trap-contract',oracle=ROOT/'build/gnu-bash/bash')
driver=fingerprint(Path(__file__))
cases={
 'usr1':"trap 'echo usr1' USR1\nexec ./nonexec\nkill -USR1 $$\n",
 'usr2':"trap 'echo usr2' USR2\nexec ./nonexec\nkill -USR2 $$\n",
 'alarm':"trap 'echo alarm' ALRM\nexec ./nonexec\nkill -ALRM $$\n",
 'repeat':"trap 'echo restored' USR1\nfor n in 1 2 3; do exec ./nonexec; kill -USR1 $$; done\n",
 'two-signals':"trap 'echo one' USR1\ntrap 'echo two' USR2\nexec ./nonexec\nkill -USR1 $$\nkill -USR2 $$\n",
 'ignored':"trap '' TERM\ntrap 'echo one' USR1\nexec ./nonexec\nkill -TERM $$\nkill -USR1 $$\n",
 'replace':"trap 'echo old' USR1\nexec ./nonexec\ntrap 'echo new' USR1\nkill -USR1 $$\n",
 'reset':"trap 'echo old' USR1\nexec ./nonexec\ntrap - USR1\ntrap -p USR1\n",
 'function':"trap 'echo one' USR1\nf() { exec ./nonexec; }\nf\nkill -USR1 $$\n",
 'group':"trap 'echo one' USR1\n{ exec ./nonexec; }\nkill -USR1 $$\n",
 'exit-trap':"trap 'echo end' EXIT\ntrap 'echo one' USR1\nexec ./nonexec\nkill -USR1 $$\n",
 'in-handler':"trap 'exec ./nonexec; echo handler' USR1\nkill -USR1 $$\nkill -USR1 $$\n",
 'handler-replace':"f() { exec ./nonexec; trap 'echo replacement' USR1; }\ntrap f USR1\nkill -USR1 $$\nkill -USR1 $$\n",
 'enoent':"trap 'echo one' USR1\nexec ./not-present\nkill -USR1 $$\n",
 'clean-environment':"trap 'echo one' USR1\nexec -c ./nonexec\nkill -USR1 $$\n",
 'command-builtin':"trap 'echo one' USR1\ncommand exec ./nonexec\nkill -USR1 $$\n",
}

results=[]
with tempfile.TemporaryDirectory(prefix='rboxc-exec-trap-') as directory:
 work=Path(directory);(work/'nonexec').write_text('echo should-not-execute\n');(work/'nonexec').chmod(0o644)
 for name,body in cases.items():
  script='shopt -s execfail\n'+body+'printf "after\\n"\n';(work/'case.sh').write_text(script)
  outcomes={}
  for impl,binary in [('gnu',profile.oracle),('rboxc',profile.binary)]:
   alias=work/'bash';alias.unlink(missing_ok=True);alias.symlink_to(binary)
   for instrument in [False,True]:
    label=impl+('-valgrind' if instrument else '')
    argv=[str(alias),'--noprofile','--norc','./case.sh']
    prefix=profile.logs/(name+'-'+label)
    if instrument:argv=['/usr/bin/valgrind','--leak-check=full','--show-leak-kinds=all','--track-fds=yes','--trace-children=yes','--log-file='+str(prefix)+'-%p.log',*argv]
    done=subprocess.run(argv,cwd=work,env={'PATH':'/usr/bin:/bin','LC_ALL':'C','HOME':directory,'TMPDIR':directory},capture_output=True,timeout=30)
    raw={}
    for stream,data in [('stdout',done.stdout),('stderr',done.stderr)]:
     path=Path(str(prefix)+'-'+stream);path.write_bytes(data);raw[str(path.relative_to(ROOT))]=fingerprint(path)
    memory=[]
    for path in sorted(profile.logs.glob(prefix.name+'-*.log')):
     text=path.read_text();pid=path.stem.rsplit('-',1)[-1]
     memory.append({'log':str(path.relative_to(ROOT)),'sha256':fingerprint(path),**runner.parse_memory_log(text,pid,True)})
    outcome={'status':done.returncode,'stdout':done.stdout.hex(),'stderr':done.stderr.hex(),'raw':raw,'memory':memory}
    if instrument:outcome['clean']=bool(memory) and all(m['complete_exec_log'] and m['errors']==m['non_inherited_descriptors']==0 and not any(m['heap_bytes'].get(k,0) for k in ['definitely lost','indirectly lost','possibly lost']) for m in memory)
    outcomes[label]=outcome
  expected=outcomes['gnu'];equivalent=all(all(o[k]==expected[k] for k in ['status','stdout','stderr']) for o in outcomes.values())
  passed=equivalent and outcomes['rboxc-valgrind']['clean']
  results.append({'name':name,'script':script,'outcomes':outcomes,'behavior_pass':equivalent,'pass':passed})
  profile.report.write_text(json.dumps({**profile.metadata(),'driver_sha256':driver,'scope':'Sixteen finite trap-restoration contexts after failed execution of an owned nonexecutable file or absent pathname. Self-directed signals only; GNU output/status comparison and complete Valgrind tracing.',
   'results':results,'passed':sum(r['pass'] for r in results),'total':len(results),'planned_total':len(cases),'complete':len(results)==len(cases)},indent=2)+'\n')
  print('PASS' if passed else 'OPEN',name,flush=True)
assert fingerprint(Path(__file__))==driver
raise SystemExit(not all(r['pass'] for r in results))
