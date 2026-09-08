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
import subprocess
import sys
import tempfile
from comparison_profile import ComparisonProfile, fingerprint

ROOT=Path(__file__).resolve().parents[1]
sys.path.insert(0,str(ROOT/'tests/gnu'))
spec=importlib.util.spec_from_file_location('reviewed',ROOT/'tests/gnu/reviewed-original.py')
runner=importlib.util.module_from_spec(spec);spec.loader.exec_module(runner)
profile=ComparisonProfile('findutils-original',oracle=ROOT/'build/gnu-findutils/find/find',selections=True)
source=Path(json.loads((ROOT/'inventory/sources.json').read_text())['findutils']['source'])
selections={
 'find/regex1':(4,[]), 'find/printf':(4,[]),
 'xargs/n1-0':(1,['stairs-0.xi']), 'xargs/n2-0':(1,['stairs-0.xi']),
 'xargs/idef-0':(1,['items-0.xi']), 'xargs/space-0':(1,['space.xi']),
 'xargs/empty-r':(1,[]),
 'locate/regex1':(8,[]), 'locate/space1st':(1,[]),
}
# Each added script and its input/expected-output fixtures was reviewed before registration.
for name in ('samefile-copy','samefile-link','samefile-missing','samefile-p-brokenlink','samefile-same','samefile-symlink','comma','delete','deletedir','deletefile','depth','empty','iname','ipath','iregex1','iwholename','lname','path','perm','perm000','true','false','quit'):
 selections['find/'+name]=(4,[])
for name,fixture in {
 '0n3':'files0','E_-0':'eof_-0','L2-0':'ldata-0','L2_2-0':'ldatab-0','L3-0':'ldata-0',
 'P3-n1-IARG':'Pdata','delim-o':'helloworld','empty_def-r':None,'idef-s26-0':'items-0',
 'l1-0':'ldata-0','l1_2-0':'ldatab-0','n2-s21-0':'stairs-0','n2-s21-x-0':'stairs-0',
 'n3-0':'stairs2-0','n3-s31-0':'stairs2-0','noeof-0':'noeof-0','nothing':None,'r':'blank',
 's118-0':'stairs-0','s14-0':'stairs-0','s14_2-0':'stairs2-0','s15-0':'stairs-0','s25-0':'stairs-0',
 'space-r':'space','space-t-0':'space',
}.items():selections['xargs/'+name]=(1,[fixture+'.xi'] if fixture else [])
for name in ('depth-d','exec-many-rtn-failure','exec-many-rtn-success','exec-one-rtn-fail','exec-one-rtn-success','execdir-hier','execdir-one','execdir-pwd','execdir-pwd1','execdir-root-only','follow-arg-parent-symlink','follow-basic','fprint0_stdout','fprintf-samefile','gnu-or','gnuand','gnunot','ilname','inum','name-opt','name-period','name-slash','no-fdleak-test','perm-slash','posix-dflt','posix-h','posix-l','posix-perminvalid','print0','print_stdout','printf-h','printf-nonlocal-symlink','printf-slash','printf-symlink','printfHdfl','prune-default-print','regex2','sv-bug-12230','sv-bug-17477','sv-bug-17782','sv-bug-18222','sv-bug-27563-execdir','used-invarg','used-missing','wholename','xtype-symlink','xtype'):
 selections['find/'+name]=(4,[])
selections.update({'find/mindepth-arg':(8,[]),'find/mindepth-badarg':(64,[]),'find/printf-reserved':(12,[]),'find/user-invalid':(20,[])})
for name in ('exists1','exists2','exists3','notexists1','notexists2','notexists3','ignore_case1','ignore_case2','ignore_case3','slocate'):
 selections['locate/'+name]=(1,[])
selections.update({'locate/bigendian':(1,['../locate.gnu/locateddb.old.powerpc.xi']),
                   'locate/littleendian':(1,['../locate.gnu/locateddb.old.x86.xi']),
                   'locate/bigprefix1':(2,[]),'locate/exceedshort':(1,[]),'locate/sv-bug-14535':(8,[])})
selected=profile.options.commands or list(selections)
assert set(selected)<=set(selections)
oracles={n:ROOT/'build/gnu-findutils'/n/n for n in ('find','xargs','locate')}
helpers={n:ROOT/'build/gnu-coreutils/src/coreutils' for n in ('cat','rm','mkdir','chmod','touch','sort','echo','basename','cp','ln','true','false','sleep','ls','mv','mktemp','cut','id','date')}
helpers.update({n:ROOT/'build/gnu-diffutils/src'/n for n in ('cmp','diff')})
helpers['sed']=ROOT/'build/gnu-sed/sed/sed'
frcode=ROOT/'build/gnu-findutils/locate/frcode'
updatedb=ROOT/'build/gnu-findutils/locate/updatedb'
inputs={p:fingerprint(p) for p in {*oracles.values(),*helpers.values(),frcode,updatedb,Path('/usr/bin/sort'),Path('/usr/bin/locale'),source/'locate/updatedb.sh',Path(__file__)}}
for selection in selected:
 command,name=selection.split('/');suite=source/command/'testsuite'
 for p in [suite/'config/unix.exp',suite/(command+'.gnu')/(name+'.exp'),*[(suite/'inputs'/n).resolve() for n in selections[selection][1]]]:inputs[p]=fingerprint(p)
 for suffix in ('xo','xe'):
  p=suite/(command+'.gnu')/(name+'.'+suffix)
  if p.exists():inputs[p]=fingerprint(p)
results=[]
def memory_clean(logs):
 return bool(logs) and all(m['complete_exec_log'] and m['errors']==0 and m['non_inherited_descriptors']==0 and not any(m['heap_bytes'].get(k,0) for k in ('definitely lost','indirectly lost','possibly lost')) for m in logs)
for index,selection in enumerate(selected):
 command,name=selection.split('/');expected,_=selections[selection];outcomes={}
 for implementation in ('gnu','rboxc'):
  for instrument in (False,True):
   key=implementation+('-valgrind' if instrument else '')
   saved=profile.logs/(f'{index:02}-'+command+'-'+name+'-'+key);saved.mkdir()
   with tempfile.TemporaryDirectory(prefix='rboxc-findutils-original-') as directory:
    work=Path(directory);deps=work/'deps';deps.mkdir();executables=work/'exec';executables.mkdir();memory=work/'memory';memory.mkdir()
    for helper,binary in helpers.items():(deps/helper).symlink_to(binary)
    for applet in oracles:
     parent=work/applet;parent.mkdir();(parent/'testsuite').mkdir()
     (executables/applet).symlink_to(oracles[applet] if implementation=='gnu' else profile.binary)
     argv=[str(executables/applet)]
     if instrument and applet==command:
      argv=['/usr/bin/valgrind','--leak-check=full','--show-leak-kinds=all','--track-fds=yes','--trace-children=yes','--log-file='+str(memory/'%p.log'),*argv]
     wrapper=parent/applet;wrapper.write_text('#!/bin/sh\nexec '+shlex.join(argv)+' "$@"\n');wrapper.chmod(0o755)
    (work/'find/ftsfind.o').symlink_to(ROOT/'build/gnu-findutils/find/ftsfind.o')
    (work/'xargs/xargs.o').symlink_to(ROOT/'build/gnu-findutils/xargs/xargs.o')
    (work/'locate/frcode').symlink_to(frcode)
    # Native updatedb and frcode are fixture helpers, not translated commands.
    # Database construction uses only each original script's private paths.
    (work/'locate/updatedb').symlink_to(updatedb)
    cwd=work/command/'testsuite';suite=source/command/'testsuite'
    (cwd/'site.exp').write_text('set srcdir "'+str(suite)+'"\nset objdir "'+str(cwd)+'"\nset build_triplet x86_64-pc-linux-gnu\nset host_triplet x86_64-pc-linux-gnu\n')
    env={'PATH':str(deps)+':/usr/bin:/bin','HOME':directory,'LC_ALL':'C','LANGUAGE':'C','TZ':'UTC0','TMPDIR':directory,'DEJAGNU':'/dev/null','TERM':'dumb'}
    invocation=['/usr/bin/runtest','--tool',command,'--srcdir',str(suite),command+'.gnu/'+name+'.exp']
    done=subprocess.run(invocation,cwd=cwd,env=env,stdout=subprocess.PIPE,stderr=subprocess.STDOUT,timeout=180)
    (saved/'driver.log').write_bytes(done.stdout)
    for suffix in ('sum','log'):
     p=cwd/(command+'.'+suffix)
     if p.exists():shutil.copy2(p,saved/p.name)
    shutil.copytree(memory,saved/'memory')
    summary=(saved/(command+'.sum')).read_text() if (saved/(command+'.sum')).exists() else ''
    assertions=re.findall(r'^(PASS|FAIL|XFAIL|XPASS|UNRESOLVED|UNSUPPORTED|UNTESTED|ERROR): (.*)$',summary,re.M)
    logs=[{**runner.parse_memory_log(p.read_text(),p.stem,exec_only=True),'log':str(p.relative_to(ROOT)),'sha256':fingerprint(p)} for p in sorted((saved/'memory').glob('*.log'))]
    outcomes[key]={'status':done.returncode,'assertions':assertions,'assertions_pass':done.returncode==0 and len(assertions)==expected and all(r[0]=='PASS' for r in assertions),
                   'memory':logs,'memory_clean':memory_clean(logs) if instrument else None,
                   'driver_log':str((saved/'driver.log').relative_to(ROOT)),'driver_log_sha256':fingerprint(saved/'driver.log')}
 row={'selection':selection,'expected_assertions':expected,'outcomes':outcomes}
 row['assertions_pass']=all(v['assertions_pass'] for v in outcomes.values()) and all(v['assertions']==outcomes['gnu']['assertions'] for v in outcomes.values())
 row['pass']=row['assertions_pass'] and outcomes['rboxc-valgrind']['memory_clean']
 results.append(row)
 assert all(fingerprint(p)==h for p,h in inputs.items()),'test input changed'
 report={'scope':'Reviewed original DejaGNU scripts and expected fixtures, unchanged. Four GNU/Rust/native/Valgrind observations per selection. Private shell wrappers only select executables and instrumentation. Original find tests retain optimization levels 0,1,2,3. Native frcode and updatedb are fixture helpers and are not counted as ports. Their source/configured scripts and the configured absolute sort helper are pinned. Locale case-folding uses the available native UTF-8 locale selected by the unchanged original script.',
         **profile.metadata(),'inputs':{str(p):h for p,h in inputs.items()},'passed':sum(r['pass'] for r in results),'total':len(results),
         'assertions_passed':sum(r['expected_assertions'] for r in results if r['assertions_pass']),'results':results}
 profile.report.write_text(json.dumps(report,indent=2)+'\n')
 print('PASS' if row['pass'] else 'OPEN',selection,flush=True)
raise SystemExit(any(not r['pass'] for r in results))
