#!/usr/bin/env python3
"""GNU comparisons for standalone builtin adapters and embedded updatedb."""
# SPDX-License-Identifier: GPL-3.0-or-later
import importlib.util,json,os,re,subprocess,tempfile,sys
from pathlib import Path
from comparison_profile import ComparisonProfile,fingerprint
ROOT=Path(__file__).resolve().parents[1]
sys.path.insert(0,str(ROOT/'tests/gnu'))
spec=importlib.util.spec_from_file_location('reviewed',ROOT/'tests/gnu/reviewed-original.py')
runner=importlib.util.module_from_spec(spec);spec.loader.exec_module(runner)
profile=ComparisonProfile('shell-adapters-behavior',oracle=ROOT/'build/gnu-bash/bash')
script=(ROOT/'build/gnu-findutils/locate/updatedb').read_text()
cases=[]
builtins={'.':['./source','one'],':':['argument'],'alias':['sample=printf'],'break':[],
 'cd':['.'],'continue':[],'declare':['-p','BASH_VERSION'],'eval':['printf "evaluated\\n"'],
 'exec':['/bin/true'],'exit':['7'],'export':['ITEM=value'],'help':['cd'],'jobs':[],
 'local':['item=value'],'return':['3'],'set':['--','one','two'],'shift':['0'],
 'source':['./source','one'],'trap':['-l'],'ulimit':['-n'],'unalias':['missing'],
 'unset':['missing'],'wait':[]}
for name,args in builtins.items():
 for suffix,selected in [('help',['--help']),('behavior',args)]:
  cases.append((name+'-'+suffix,name,selected,'builtin '+name+' "$@"',False))
for label,args,expression in [
 ('empty',[], '""'),('nonempty',['x'],'"x"'),('empty-string',[''],'""'),
 ('pattern',['alpha','==','a*'],'"alpha" == a*'),('not-equal',['x','!=','y'],'"x" != "y"'),
 ('false',['x','==','y'],'"x" == "y"'),('regex',['ab12','=~','^[a-z]+([0-9]+)$'],'"ab12" =~ ^[a-z]+([0-9]+)$'),
 ('numeric',['17','-gt','4'],'17 -gt 4'),('unary',['-n','x'],'-n "x"'),
 ('unary-looking-lhs',['-n','==','-n'],'"-n" == "-n"'),
 ('and',['x','==','x','&&','-f','source'],'"x" == "x" && -f source'),
 ('or',['','||','x'],'"" || "x"'),('group',['!','(','x','==','y',')'],'! ( "x" == "y" )'),
 ('literal',['$(printf unexpected)','==','$(printf unexpected)'],'"$(printf %s \'$(printf unexpected)\')" == \'$(printf unexpected)\''),
 ('closing',['alpha','==','a*',']]'],'"alpha" == a*')]:
 cases.append(('conditional-'+label,'[[',args,'[[ '+expression+' ]]',False))
for label,args in [('help',['--help']),('version',['--version']),('option',['--unknown']),
 ('format',['--dbformat=obsolete']),('locate02',[]),('slocate',['--dbformat=slocate']),
 ('prune',['--prunepaths={work}/files/tree/pruned']),('empty',['--localpaths='])]:
 cases.append(('updatedb-'+label,'updatedb',args,script,True))
inputs={str(p):fingerprint(p) for p in [profile.binary,profile.oracle,ROOT/'build/gnu-findutils/locate/updatedb',Path(__file__).resolve()]}
results=[]
for index,(name,command,args,oracle_script,database) in enumerate(cases):
 outcomes={}
 for candidate in (False,True):
  for instrument in (False,True):
   key=('rboxc' if candidate else 'gnu')+('-valgrind' if instrument else '')
   with tempfile.TemporaryDirectory(prefix='rboxc-shell-adapter-') as directory:
    work=Path(directory);(work/'exec').mkdir();(work/'files').mkdir();cwd=work/'files'
    (cwd/'source').write_text('printf "sourced:%s\\n" "$1"\nreturn 4\n')
    if database:
     (cwd/'tree/pruned').mkdir(parents=True);(cwd/'tree/space name').write_text('one');(cwd/'tree/pruned/skip').write_text('two');(cwd/'tree/line\nbreak').write_text('three')
    alias=work/'exec'/command
    if command!='.':alias.symlink_to(profile.binary if candidate else profile.oracle)
    env={'LC_ALL':'C','LANGUAGE':'C','PATH':str(work/'exec')+':/usr/bin:/bin','HOME':directory,'TZ':'UTC0'}
    selected=[s.replace('{work}',directory) for s in args]
    if database:
     selected=['--localpaths='+str(cwd/'tree'),'--netpaths=','--prunepaths=','--prunefs=','--output='+str(cwd/'db'),*selected]
     if not candidate:
      env['BINDIR']=str(ROOT/'build/gnu-findutils/find');env['LIBEXECDIR']=str(ROOT/'build/gnu-findutils/locate')
    invocation='.' if command=='.' else str(alias)
    argv=([str(profile.binary),'.',*selected] if command=='.' else [invocation,*selected]) if candidate else [str(profile.oracle),'--noprofile','--norc','-c',oracle_script,invocation,*selected]
    logs=profile.logs/(str(index)+'-'+key);logs.mkdir()
    if instrument:argv=['/usr/bin/valgrind','--leak-check=full','--show-leak-kinds=all','--track-fds=yes','--trace-children=yes','--log-file='+str(logs/'process-%p.log'),*argv]
    done=subprocess.run(argv,input=b'',cwd=cwd,capture_output=True,timeout=90,env=env)
    normalize=lambda b:b.replace(directory.encode(),b'<fixture>')
    tree={str(p.relative_to(cwd)):{'data':normalize(p.read_bytes()).hex(),'mode':p.stat().st_mode&0o7777} for p in sorted(cwd.rglob('*')) if p.is_file()}
    row={'status':done.returncode,'stdout':normalize(done.stdout).hex(),'stderr':normalize(done.stderr).hex(),'tree':tree,'memory':[]}
    for log in sorted(logs.glob('process-*.log')):
     contents=log.read_text();pids=set(re.findall(r'^==([0-9]+)==',contents,re.M));assert len(pids)==1
     pid=pids.pop();m=runner.parse_memory_log(contents,pid)
     errors=list(re.finditer('ERROR SUMMARY:',contents));fds=list(re.finditer('FILE DESCRIPTORS:',contents));images=list(re.finditer(r'^==[0-9]+== Command:',contents,re.M))
     complete=bool(errors) and len(errors)==len(fds) and (not images or errors[-1].start()>images[-1].start() and fds[-1].start()>images[-1].start())
     clean=complete and m['errors']==0 and m['non_inherited_descriptors']==0 and not any(m['heap_bytes'].get(k,0) for k in ('definitely lost','indirectly lost','possibly lost'))
     row['memory'].append({'log':str(log.relative_to(ROOT)),'sha256':fingerprint(log),'complete':complete,'clean':clean,**m})
    assert not instrument or row['memory']
    if database and (cwd/'db').exists():
     reader=profile.binary if candidate else ROOT/'build/gnu-findutils/locate/locate'
     cmd=[str(reader),*(['locate'] if candidate else []),'-d',str(cwd/'db'),'tree']
     read=subprocess.run(cmd,capture_output=True,env=env,timeout=20)
     row['database_read']={'status':read.returncode,'stdout':normalize(read.stdout).hex(),'stderr':normalize(read.stderr).hex()}
    outcomes[key]=row
 reference=outcomes['gnu'];fields=['status','stdout','stderr','tree','database_read']
 equivalent=all(all(v.get(k)==reference.get(k) for k in fields) for v in outcomes.values())
 clean=all(m['clean'] for m in outcomes['rboxc-valgrind']['memory'])
 results.append({'name':name,'command':command,'args':args,'equivalent':equivalent,'memory_clean':clean,'pass':equivalent and clean,'outcomes':outcomes})
 for p,h in inputs.items():assert fingerprint(Path(p))==h
 report={**profile.metadata(),'inputs':inputs,'complete':len(results)==len(cases),'total':len(results),'planned_total':len(cases),'equivalent':sum(r['equivalent'] for r in results),'passed':sum(r['pass'] for r in results),'results':results}
 profile.report.write_text(json.dumps(report,indent=2)+'\n');print('PASS' if results[-1]['pass'] else 'OPEN',name,flush=True)
raise SystemExit(report['passed']!=report['total'])
