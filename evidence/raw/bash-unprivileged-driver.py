#!/usr/bin/env python3
"""Run reviewed, unchanged Bash scripts against their original expected output."""
# SPDX-License-Identifier: GPL-3.0-or-later
import importlib.util,json,os,re,shutil,signal,subprocess,sys,tempfile
import fcntl,pty,select,termios,threading
from pathlib import Path
from comparison_profile import ComparisonProfile,fingerprint

ROOT=Path(__file__).resolve().parents[1]
sys.path.insert(0,str(ROOT/'tests/gnu'))
spec=importlib.util.spec_from_file_location('reviewed',ROOT/'tests/gnu/reviewed-original.py')
runner=importlib.util.module_from_spec(spec);spec.loader.exec_module(runner)
profile=ComparisonProfile('bash-original',oracle=ROOT/'build/gnu-bash/bash',selections=True)
source=Path(json.loads((ROOT/'inventory/sources.json').read_text())['bash']['source'])/'tests'
manifest=ROOT/'inventory/bash-tests.json'
inventory=json.loads(manifest.read_text())
selected=[r for r in inventory['inputs'] if r['reviewed']]
if profile.options.commands:
    assert set(profile.options.commands)<={r['target'] for r in selected}
    selected=[r for r in selected if r['target'] in profile.options.commands]
assert selected
selected_groups=sorted(r['target'] for r in selected)
selected=[{**r,**case,'selection':r['target']+':'+case['script'] if r.get('script_cases') else r['target']}
          for r in selected for case in r.get('script_cases',[{}])]
helpers={'sed':ROOT/'build/gnu-sed/sed/sed','grep':ROOT/'build/gnu-grep/src/grep',
         'cmp':ROOT/'build/gnu-diffutils/src/cmp',
         'diff':ROOT/'build/gnu-diffutils/src/diff','awk':ROOT/'build/gnu-gawk/gawk',
         **{n:ROOT/'build/gnu-coreutils/src/coreutils' for n in ('od','mktemp','touch','chmod','chgrp','rm','cat','tr','mkdir','printenv','sleep','date','wc','seq','tee','expr','ls','ln','cp','uname','env','sort','mkfifo','printf','rmdir')}}
fixed_helpers=inventory.get('fixed_test_helpers',{})
runtime_helpers=inventory.get('runtime_test_helpers',{})
inputs={p:fingerprint(p) for p in [Path(__file__),manifest,profile.oracle,*helpers.values()]}
for helper in fixed_helpers.values():
    inputs[source.parent/helper['source']]=helper['source_sha256']
    inputs[ROOT/helper['binary']]=helper['binary_sha256']
for helper in runtime_helpers.values():
    inputs[ROOT/helper['source']]=helper['source_sha256']
    inputs[ROOT/helper['binary']]=helper['binary_sha256']
for row in selected:
    if row.get('unprivileged'):
        inputs[ROOT/'tests/bash-unprivileged-profile.py']=fingerprint(ROOT/'tests/bash-unprivileged-profile.py')
    for name,data in row.get('oracle_helpers',{}).items():
        assert row['target']=='builtins' and name=='printenv'
        inputs[Path(data['binary'])]=data['binary_sha256']
    for name,data in row.get('build_data',{}).items():
        assert Path(name).name==name and name not in ('.','..')
        inputs[Path(data['path'])]=data['sha256']
    inputs.update({Path(p):h for p,h in row.get('host_inputs',{}).items()})
    if row.get('locale_archive'):
        archive=row['locale_archive'];inputs[Path(archive['path'])]=archive['sha256']
        inputs[Path('/usr/bin/locale')]=row['host_inputs']['/usr/bin/locale']
    if row.get('absolute_helpers') or row.get('empty_system_profile') or row.get('locale_archive') or row.get('private_tmp'):
        inputs.update({p:fingerprint(p) for p in (Path('/usr/bin/unshare'),Path('/usr/bin/mount'),Path('/bin/sh').resolve())})
    inputs[source/row['recipe']]=row['recipe_sha256']
    inputs.update({source/n:h for n,h in row['fixtures'].items()})
assert all(fingerprint(p)==h for p,h in inputs.items())
results=[]
for row in selected:
    outcomes={};name=row['selection']
    timeout_seconds=row.get('timeout_seconds',180)
    assert type(timeout_seconds) is int and 1<=timeout_seconds<=900
    assert type(row.get('controlling_terminal',False)) is bool
    assert type(row.get('stdin_terminal',False)) is bool
    assert type(row.get('empty_system_profile',False)) is bool
    assert type(row.get('private_tmp',False)) is bool
    assert type(row.get('unprivileged',False)) is bool
    if row.get('stdin_terminal'):assert row.get('controlling_terminal') and not row.get('stdin_script')
    for implementation,binary in [('gnu',profile.oracle),('rboxc',profile.binary)]:
        for instrument in (False,True):
            key=implementation+('-valgrind' if instrument else '')
            saved=profile.logs/(name+'-'+key);saved.mkdir()
            with tempfile.TemporaryDirectory(prefix='rboxc-bash-original-',dir='/var/tmp' if row.get('unprivileged') else ROOT/'build' if row.get('private_tmp') else None) as directory:
                work=Path(directory);(work/'exec').mkdir()
                alias=work/'exec/bash';alias.symlink_to(binary)
                for n,p in helpers.items():
                    native=Path(row['oracle_helpers'][n]['binary']) if n in row.get('oracle_helpers',{}) else p
                    (work/'exec'/n).symlink_to(native if implementation=='gnu' else profile.binary)
                if row.get('locale_archive'):(work/'exec/locale').symlink_to('/usr/bin/locale')
                for n,h in fixed_helpers.items():(work/'exec'/n).symlink_to(ROOT/h['binary'])
                for n in row['fixtures']:shutil.copy2(source/n,work/n)
                staged_executables={}
                unprivileged_profile=None
                memory_directory=saved
                if row.get('unprivileged'):
                    # Private copies keep the parent workspace inaccessible to nobody.
                    # Hard-linked command names preserve multicall applet dispatch.
                    work.chmod(0o755);os.chown(work,65534,65534)
                    storage=work/'programs';storage.mkdir()
                    for executable in (work/'exec').iterdir():
                        original=executable.resolve(strict=True);digest=fingerprint(original)
                        copied=storage/digest
                        if not copied.exists():shutil.copy2(original,copied);copied.chmod(0o555)
                        executable.unlink();os.link(copied,executable)
                        staged_executables[executable.name]={'source':str(original),'sha256':digest,'copy_sha256':fingerprint(executable)}
                    for fixture in row['fixtures']:os.chown(work/fixture,65534,65534)
                    memory_directory=work/'memory';memory_directory.mkdir();os.chown(memory_directory,65534,65534)
                argv=[str(alias),'--noprofile','--norc']
                if not row.get('stdin_script'):argv+=['./'+row['script']]
                if instrument:argv=['/usr/bin/valgrind','--leak-check=full','--show-leak-kinds=all',
                    '--track-fds=yes','--trace-children=yes','--log-file='+str(memory_directory/'process-%p.log'),*argv]
                env={'PATH':str(work/'exec')+':/usr/bin:/bin','THIS_SH':str(alias),'HOME':directory,
                     'TMPDIR':directory,'LC_ALL':'C','LANGUAGE':'C','TZ':'UTC0'}
                if row.get('build_data'):
                    (work/'build-data').mkdir()
                    for filename,data in row['build_data'].items():
                        shutil.copy2(data['path'],work/'build-data'/filename)
                    env['BUILD_DIR']=str(work/'build-data')
                if row.get('runtime_helpers'):
                    env['LD_PRELOAD']=':'.join(str(ROOT/runtime_helpers[n]['binary']) for n in row['runtime_helpers'])
                private_mounts=[]
                private_profile=None
                private_locale=None
                private_tmp=None
                if row.get('unprivileged'):
                    launch=work/'unprivileged.json'
                    launch.write_text(json.dumps({'work':str(work),'journal':str(memory_directory/'unprivileged.json')}))
                    argv=[sys.executable,str(ROOT/'tests/bash-unprivileged-profile.py'),str(launch),*argv]
                if row.get('absolute_helpers') or row.get('empty_system_profile') or row.get('locale_archive') or row.get('private_tmp'):
                    mount_args=[]
                    for absolute in row.get('absolute_helpers',[]):
                        assert absolute in ('/bin/echo','/bin/sh','/bin/sed','/bin/ls','/bin/true','/bin/false','/bin/cat','/bin/mkdir','/bin/touch','/bin/chmod','/bin/rm','/usr/bin/true','/usr/bin/false','/usr/bin/printf')
                        destination=Path(absolute).resolve()
                        assert str(destination) in row['host_inputs']
                        native=profile.oracle if absolute=='/bin/sh' else helpers['sed'] if absolute=='/bin/sed' else ROOT/'build/gnu-coreutils/src/coreutils'
                        replacement=native if implementation=='gnu' else profile.binary
                        private_mounts.append({'original':absolute,'destination':str(destination),'source':str(replacement),'source_sha256':fingerprint(replacement)})
                        mount_args += [str(replacement),str(destination)]
                    if row.get('empty_system_profile'):
                        destination=Path('/etc/profile').resolve()
                        assert str(destination) in row['host_inputs']
                        empty=saved/'empty-system-profile';empty.write_bytes(b'')
                        private_profile={'destination':str(destination),'source':str(empty),'source_sha256':fingerprint(empty)}
                        mount_args += [str(empty),str(destination)]
                    if row.get('locale_archive'):
                        destination=Path('/usr/lib/locale/locale-archive').resolve()
                        assert str(destination) in row['host_inputs']
                        archive=Path(row['locale_archive']['path'])
                        assert fingerprint(archive)==row['locale_archive']['sha256']
                        private_locale={'destination':str(destination),'source':str(archive),'source_sha256':fingerprint(archive),
                                        'native_locale_helper':'/usr/bin/locale'}
                        mount_args += [str(archive),str(destination)]
                    mount_script='while [ "$1" != -- ]; do /usr/bin/mount --bind "$1" "$2" || exit 77; shift 2; done; shift; '
                    fixture_args=[]
                    if row.get('private_tmp'):
                        scratch=work/'private-tmp';scratch.mkdir();scratch.chmod(0o1777)
                        def host_tmp_identity():
                            st=Path('/tmp').stat()
                            return {k:getattr(st,'st_'+k) for k in ('dev','ino','mode','uid','gid')}
                        private_tmp={'source':str(scratch),'destination':'/tmp','mode':0o1777,
                                     'host_identity':host_tmp_identity()}
                        mount_args += [str(scratch),'/tmp']
                        mount_script += 'while IFS= read -r line; do printf "%s\\n" "$line"; done < /proc/self/mountinfo > "$1"; shift; '
                        fixture_args=[str(saved/'mountinfo')]
                    argv=['/usr/bin/unshare','--mount','--propagation','private','/bin/sh','-c',
                        mount_script+'exec "$@"',
                        'bash-private-helpers',*mount_args,'--',*fixture_args,*argv]
                script_input=(work/row['script']).open('rb') if row.get('stdin_script') else None
                terminal_output=bytearray()
                terminal_options={'start_new_session':True}
                if row.get('controlling_terminal'):
                    master,slave=pty.openpty()
                    def terminal_child():
                        os.setsid()
                        fcntl.ioctl(slave,termios.TIOCSCTTY,0)
                        os.close(slave)
                    terminal_options={'preexec_fn':terminal_child,'pass_fds':(slave,)}
                process=subprocess.Popen(argv,cwd=work,env=env,stdin=slave if row.get('stdin_terminal') else script_input if script_input is not None else subprocess.DEVNULL,stdout=subprocess.PIPE,
                    stderr=subprocess.PIPE if row['output_mode'] in ('stdout','drop-expect-stdout') else subprocess.STDOUT,
                    **terminal_options)
                if row.get('controlling_terminal'):
                    stop_drain=threading.Event()
                    def drain_terminal():
                        while True:
                            if select.select([master],[],[],0.05)[0]:
                                chunk=os.read(master,65536)
                                if not chunk:break
                                terminal_output.extend(chunk)
                            elif stop_drain.is_set():break
                    drain=threading.Thread(target=drain_terminal)
                    drain.start()
                if script_input is not None:script_input.close()
                timed_out=False
                try:
                    stdout,stderr=process.communicate(timeout=timeout_seconds)
                except subprocess.TimeoutExpired:
                    timed_out=True
                    try:os.killpg(process.pid,signal.SIGTERM)
                    except ProcessLookupError:pass
                    try:stdout,stderr=process.communicate(timeout=5)
                    except subprocess.TimeoutExpired:
                        try:os.killpg(process.pid,signal.SIGKILL)
                        except ProcessLookupError:pass
                        stdout,stderr=process.communicate()
                if row.get('controlling_terminal'):
                    stop_drain.set();drain.join()
                    os.close(slave);os.close(master)
                    (saved/'terminal-output').write_bytes(terminal_output)
                if private_tmp:assert host_tmp_identity()==private_tmp['host_identity']
                if row.get('unprivileged'):
                    for log in memory_directory.glob('process-*.log'):shutil.copy2(log,saved/log.name)
                    journal=memory_directory/'unprivileged.json'
                    if journal.exists():
                        shutil.copy2(journal,saved/'unprivileged.json')
                        unprivileged_profile=json.loads(journal.read_text())
                    assert unprivileged_profile and unprivileged_profile['uid']==unprivileged_profile['gid']==65534
                    assert unprivileged_profile['groups']==[] and unprivileged_profile['no_new_privileges']
                    assert unprivileged_profile['cap_effective']=='0000000000000000'
                done=subprocess.CompletedProcess(argv,process.returncode,stdout,stderr)
                actual=done.stdout
                # Match run-invert's original `grep -v '^expect'` filter.
                if row['output_mode'] in ('drop-expect','drop-expect-stdout'):
                    actual=b''.join(line for line in actual.splitlines(keepends=True) if not line.startswith(b'expect'))
                (saved/'stdout').write_bytes(done.stdout);(saved/'stderr').write_bytes(done.stderr or b'')
                (saved/'actual').write_bytes(actual)
                logs=[]
                for log in sorted(saved.glob('process-*.log')):
                    text=log.read_text();pids=set(re.findall(r'^==([0-9]+)==',text,re.M));assert len(pids)==1
                    pid=pids.pop();parsed=runner.parse_memory_log(text,pid)
                    errors=list(re.finditer(r'ERROR SUMMARY:',text));fds=list(re.finditer(r'FILE DESCRIPTORS:',text))
                    images=list(re.finditer(r'^==[0-9]+== Command:',text,re.M))
                    complete=bool(errors) and len(errors)==len(fds) and (not images or errors[-1].start()>images[-1].start() and fds[-1].start()>images[-1].start())
                    clean=complete and parsed['errors']==0 and parsed['non_inherited_descriptors']==0 and not any(parsed['heap_bytes'].get(k,0) for k in ('definitely lost','indirectly lost','possibly lost'))
                    logs.append({'log':str(log.relative_to(ROOT)),'sha256':fingerprint(log),'pid':pid,'complete':complete,'clean':clean,**parsed})
                assert not instrument or logs
                outcomes[key]={'unprivileged_profile':unprivileged_profile,'staged_executables':staged_executables,'private_mounts':private_mounts,'private_profile':private_profile,'private_locale':private_locale,'private_tmp':private_tmp,'controlling_terminal':bool(row.get('controlling_terminal')),'stdin_terminal':bool(row.get('stdin_terminal')),'status':done.returncode,'timeout_seconds':timeout_seconds,'stdin_script':bool(row.get('stdin_script')),'timed_out':timed_out,'expected_output_matches':actual==(source/row['expected']).read_bytes(),
                    'raw':{str(p.relative_to(ROOT)):fingerprint(p) for p in [saved/'stdout',saved/'stderr',saved/'actual',*([saved/'terminal-output'] if row.get('controlling_terminal') else []),*([saved/'empty-system-profile'] if row.get('empty_system_profile') else []),*([saved/'mountinfo'] if row.get('private_tmp') else []),*([saved/'unprivileged.json'] if row.get('unprivileged') else [])]},
                    'memory':logs,'memory_clean':all(m['clean'] for m in logs) if instrument else None}
    passed=all(not o['timed_out'] and o['expected_output_matches'] and o['status']==outcomes['gnu']['status'] for o in outcomes.values()) and outcomes['rboxc-valgrind']['memory_clean']
    results.append({'selection':name,'pass':passed,'outcomes':outcomes})
    assert all(fingerprint(p)==h for p,h in inputs.items())
    report={**profile.metadata(),'inputs':{str(p):h for p,h in inputs.items()},
        'environment_profile':{r['target']:r['runtime_helpers'] for r in selected if r.get('runtime_helpers')},
        'scope':'Reviewed complete Bash scripts and their nested dependencies execute unchanged in private directories. Compare original expected files using the output mode/filter from the original run recipe, and compare exit status with GNU. Trace shell children and command helpers: pinned native GNU tools for the oracle, integrated tools for the candidate. Original fixed test helpers are shared and never counted as ports. Preserve native GNU memory findings. Other original scripts remain open.',
        'selected_groups':selected_groups,'original_groups':len(selected_groups),'complete':len(results)==len(selected),'planned_total':len(selected),'passed':sum(r['pass'] for r in results),'total':len(results),'results':results}
    profile.report.write_text(json.dumps(report,indent=2)+'\n')
    print('PASS' if passed else 'OPEN',name,flush=True)
raise SystemExit(report['passed']!=report['total'])
