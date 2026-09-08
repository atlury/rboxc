#!/usr/bin/env python3
"""Run the full reviewed GNU Ed check script with every original input."""
# SPDX-License-Identifier: GPL-3.0-or-later
import importlib.util
import json
from pathlib import Path
import re
import shutil
import subprocess
import sys
import tempfile
from comparison_profile import ComparisonProfile, fingerprint

ROOT=Path(__file__).resolve().parents[1]
sys.path.insert(0,str(ROOT/'tests/gnu'))
spec=importlib.util.spec_from_file_location('reviewed',ROOT/'tests/gnu/reviewed-original.py')
runner=importlib.util.module_from_spec(spec);spec.loader.exec_module(runner)
profile=ComparisonProfile('ed-original',oracle=ROOT/'build/gnu-ed/ed')
pin=json.loads((ROOT/'inventory/sources.json').read_text())['ed'];source=Path(pin['source'])
manifest_path=ROOT/'inventory/ed-tests.json';manifest=json.loads(manifest_path.read_text())
manifest_hash=fingerprint(manifest_path);driver_hash=fingerprint(Path(__file__))
assert manifest['registration']['reviewed'] and all(r['reviewed'] for r in manifest['inputs'])
script=source/manifest['registration']['path']
core=ROOT/'build/gnu-coreutils/src/coreutils'
core_hash=json.loads((ROOT/'evidence/smoke.json').read_text())['gnu_binary_sha256']
dependencies=('cat','cp','touch','grep','cmp','mv','rm','mkdir','sed','sort','date','wc')
helpers={n:core for n in dependencies if n not in ('sed','grep','cmp')}
helpers['sed']=ROOT/'build/gnu-sed/sed/sed'
helpers['grep']=ROOT/'build/gnu-grep/src/grep'
helpers['cmp']=ROOT/'build/gnu-diffutils/src/cmp'
hashes={n:fingerprint(p) for n,p in helpers.items()}
assert all(v==core_hash for n,v in hashes.items() if n not in ('sed','grep','cmp'))
shell=Path('/bin/sh');shell_hash=fingerprint(shell)
def validate_inputs():
    assert fingerprint(manifest_path)==manifest_hash
    assert fingerprint(script)==manifest['registration']['sha256']
    actual={str(p.relative_to(source)) for p in (source/'testsuite').iterdir() if p.suffix in ('.ed','.err')}
    assert actual=={r['path'] for r in manifest['inputs']}
    for row in manifest['inputs']:assert fingerprint(source/row['path'])==row['sha256']
    for name,digest in manifest['fixtures'].items():assert fingerprint(source/'testsuite'/name)==digest
    assert fingerprint(Path(__file__))==driver_hash and fingerprint(shell)==shell_hash
    assert all(fingerprint(p)==hashes[n] for n,p in helpers.items())
validate_inputs();outcomes={}
with tempfile.TemporaryDirectory(prefix='rboxc-ed-prerequisite-') as directory:
    for name,path in helpers.items():
        alias=Path(directory)/name;alias.symlink_to(path)
        probe=subprocess.run([alias,'--version'],capture_output=True,timeout=10)
        assert probe.returncode==0 and name.encode() in probe.stdout.splitlines()[0], (name,probe.stderr)
for implementation in ('gnu','rboxc'):
    for instrument in (False,True):
        key=implementation+('-valgrind' if instrument else '')
        with tempfile.TemporaryDirectory(prefix='rboxc-ed-original-') as directory:
            work=Path(directory)
            for name in ('real','deps','memory'):(work/name).mkdir()
            (work/'real/ed').symlink_to(profile.oracle if implementation=='gnu' else profile.binary)
            for name,path in helpers.items():(work/'deps'/name).symlink_to(path)
            if instrument:
                wrapper=work/'ed'
                wrapper.write_text('#!/bin/sh\nexec /usr/bin/valgrind --leak-check=full --show-leak-kinds=all --track-fds=yes --trace-children=yes --log-file='+str(work/'memory/%p.log')+' '+str(work/'real/ed')+' "$@"\n')
                wrapper.chmod(0o755)
            else:(work/'ed').symlink_to(work/'real/ed')
            done=subprocess.run([str(shell),str(script),str(source/'testsuite'),pin['version']],cwd=work,
                stdin=subprocess.DEVNULL,capture_output=True,timeout=2400,
                env={'PATH':str(work/'deps')+':/usr/bin:/bin','HOME':directory,'TMPDIR':directory,'LC_ALL':'C','TZ':'UTC0'})
            log=profile.logs/(key+'.log');log.write_bytes(done.stdout+done.stderr)
            outcome={'status':done.returncode,'stdout':done.stdout.hex(),'stderr':done.stderr.hex(),
                'assertions_pass':done.returncode==0 and b'tests completed successfully.' in done.stdout,
                'log':str(log.relative_to(ROOT)),'log_sha256':fingerprint(log)}
            if instrument:
                saved=profile.logs/(key+'-memory');shutil.copytree(work/'memory',saved)
                memory=[]
                for path in sorted(saved.glob('*.log')):
                    text=path.read_text();parsed=runner.parse_memory_log(text,path.stem,exec_only=True)
                    commands=re.findall(r'^==\d+== Command: (.*)$',text,re.M)
                    command=commands[-1] if commands else ''
                    is_ed=command.split(' ',1)[0]==str(work/'real/ed')
                    clean=parsed['complete_exec_log'] and parsed['errors']==0 and parsed['non_inherited_descriptors']==0 and not any(parsed['heap_bytes'].get(k,0) for k in ('definitely lost','indirectly lost','possibly lost'))
                    memory.append({'command':command,'ed_process':is_ed,'pass':clean,**parsed,'log':str(path.relative_to(ROOT)),'sha256':fingerprint(path)})
                outcome['memory']=memory
                outcome['all_processes_clean']=bool(memory) and all(m['pass'] for m in memory)
                editor=[m for m in memory if m['ed_process']]
                expected_runs=sum(3 if r['path'].endswith('.err') else 1 for r in manifest['inputs'])
                outcome['editor_processes']=len(editor)
                outcome['editor_memory_clean']=len(editor)>=expected_runs and all(m['pass'] for m in editor)
                outcome['external_child_findings']=[m for m in memory if not m['ed_process'] and not m['pass']]
            if (work/'tmp').exists():
                saved=profile.logs/(key+'-remaining-fixture');shutil.copytree(work/'tmp',saved)
                outcome['remaining_fixture']=str(saved.relative_to(ROOT))
            outcomes[key]=outcome
        validate_inputs()
        print(key,'assertions',outcome['assertions_pass'],'editor memory',outcome.get('editor_memory_clean'),flush=True)
        candidate=outcomes.get('rboxc-valgrind',{})
        assertions=len(outcomes)==4 and all(o['assertions_pass'] for o in outcomes.values())
        passed=assertions and candidate.get('all_processes_clean',False)
        report={'scope':'Full unchanged reviewed GNU Ed check script, all 90 original input files, all three original diagnostic input profiles and direct shell assertions. Editor memory is distinguished from traced native dependency findings; no excluded input selection.',
            **profile.metadata(),'driver_sha256':driver_hash,'manifest_sha256':manifest_hash,
            'prerequisites':{'shell':{'path':str(shell),'sha256':shell_hash},'commands':{n:{'path':str(p),'sha256':hashes[n]} for n,p in helpers.items()}},
            'passed':int(passed),'total':1,'assertions_passed':assertions,
            'editor_assertions_and_memory_passed':assertions and candidate.get('editor_memory_clean',False),
            'editing_inputs':sum(r['path'].endswith('.ed') for r in manifest['inputs']),
            'diagnostic_inputs':sum(r['path'].endswith('.err') for r in manifest['inputs']),
            'diagnostic_profiles':['regular-file','regular-file-loose-exit-status','pipe'],
            'registered_original_scripts':1,'outcomes':outcomes}
        profile.report.write_text(json.dumps(report,indent=2)+'\n')
raise SystemExit(not report['passed'])
