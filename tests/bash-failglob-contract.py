#!/usr/bin/env python3
"""Exercise owned glob lists through normal, nested and nonlocal error exits."""
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
profile=ComparisonProfile('bash-failglob-contract',oracle=ROOT/'build/gnu-bash/bash')
driver=fingerprint(Path(__file__))
cases={
 'no-match':'printf "%s\\n" absent*\n',
 'literal-prefix':'printf "%s\\n" "two words" absent*\n',
 'remaining-words':'printf "%s\\n" absent* "tail words" file_a\n',
 'one-match':'printf "%s\\n" file_a* absent*\n',
 'multiple-matches':'printf "%s\\n" file_* absent* tail\n',
 'two-patterns':'printf "%s\\n" file_* dir/* absent* tail\n',
 'brace-patterns':'printf "%s\\n" {file_*,dir/*} absent* tail\n',
 'nullglob-precedence':'shopt -s nullglob\nprintf "%s\\n" file_* absent*\n',
 'function':'f() { printf "%s\\n" file_* absent* tail; }\nf\n',
 'subshell':'( printf "%s\\n" file_* absent* tail )\n',
 'substitution':'result=$(printf "%s\\n" file_* absent* tail)\nprintf "result:<%s>\\n" "$result"\n',
 'group':'{ printf "%s\\n" file_* absent* tail; }\n',
 'temporary-assignment':'VALUE=example printf "%s\\n" file_* absent* tail\n',
 'eval':"eval 'printf \"%s\\n\" file_* absent* tail'\n",
 'source':'. ./source.inc\n',
 'exit-trap':"trap 'printf \"exit:%s\\n\" \"$?\"' EXIT\nprintf '%s\\n' file_* absent* tail\n",
}
results=[]
with tempfile.TemporaryDirectory(prefix='rboxc-failglob-') as directory:
 work=Path(directory);(work/'file_a').write_bytes(b'a\n');(work/'file_b').write_bytes(b'b\n')
 (work/'dir').mkdir();(work/'dir/one').write_bytes(b'one\n');(work/'dir/two').write_bytes(b'two\n')
 (work/'source.inc').write_text('printf "%s\\n" file_* absent* tail\n')
 for name,body in cases.items():
  script='shopt -s failglob\n'+body+'printf "after\\n"\n';(work/'case.sh').write_text(script)
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
  profile.report.write_text(json.dumps({**profile.metadata(),'driver_sha256':driver,'scope':'Sixteen finite failglob contexts with original GNU diagnostics and status comparisons; full child memory tracing. File fixtures contain two ordinary matches in each of two directories.',
   'results':results,'passed':sum(r['pass'] for r in results),'total':len(results),'planned_total':len(cases),'complete':len(results)==len(cases)},indent=2)+'\n')
  print('PASS' if passed else 'OPEN',name,flush=True)
assert fingerprint(Path(__file__))==driver
raise SystemExit(not all(r['pass'] for r in results))
