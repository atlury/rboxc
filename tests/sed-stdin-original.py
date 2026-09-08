#!/usr/bin/env python3
"""Retain the upstream stdin fixture gap and test an explicit private alias."""
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
from sed_dependencies import prepare

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0,str(ROOT/'tests/gnu'))
spec = importlib.util.spec_from_file_location('reviewed',ROOT/'tests/gnu/reviewed-original.py')
runner = importlib.util.module_from_spec(spec);spec.loader.exec_module(runner)
profile = ComparisonProfile('sed-stdin-original',oracle=ROOT/'build/gnu-sed/sed/sed')
source = Path(json.loads((ROOT/'inventory/sources.json').read_text())['sed']['source'])
script = source/'testsuite/stdin.sh'
entry = next(r for r in json.loads((ROOT/'inventory/sed-tests.json').read_text())['scripts'] if r['script']=='testsuite/stdin.sh')
assert fingerprint(script)==entry['source_sha256']
original_init = (source/'testsuite/init.sh').read_text()
anchor = 'setup_ "$@"\n'
assert original_init.count(anchor)==1
addition = 'ln -s stdin-in stdin || framework_failure_ "cannot create private stdin fixture alias"\n'
adapted_init = original_init.replace(anchor,anchor+addition)
match = re.search(rb'cat << \\EOF > stdin-in[^\n]*\n(.*?)\nEOF\n',script.read_bytes(),re.S)
assert match
fixture_bytes = match[1]+b'\n'
dependencies = prepare()
helpers,prerequisite_environment,prerequisites = dependencies
helpers = {n:Path(v['path']) for n,v in helpers.items()}
helpers.update(ln=ROOT/'build/gnu-coreutils/src/coreutils',diff=ROOT/'build/gnu-diffutils/src/diff')
inputs = {p:fingerprint(p) for p in {script,source/'testsuite/init.sh',source/'init.cfg',Path(__file__),
          Path('/bin/bash').resolve(),Path('/bin/sh').resolve(),*helpers.values()}}
results = []
for fixture in ('unchanged','private-alias'):
    outcomes = {}
    for implementation,binary in (('gnu',profile.oracle),('rboxc',profile.binary)):
        for instrument in (False,True):
            key = implementation+('-valgrind' if instrument else '')
            saved = profile.logs/(fixture+'-'+key);saved.mkdir()
            with tempfile.TemporaryDirectory(prefix='rboxc-sed-stdin-') as directory:
                work = Path(directory)
                for n in ('sed','real','deps','memory','source/testsuite'):(work/n).mkdir(parents=True)
                init = work/'source/testsuite/init.sh'
                init.write_text(original_init if fixture=='unchanged' else adapted_init)
                (work/'source/init.cfg').symlink_to(source/'init.cfg')
                shutil.copy2(init,saved/'init.sh')
                for n,p in helpers.items():(work/'deps'/n).symlink_to(p)
                (work/'real/sed').symlink_to(binary)
                if instrument:
                    wrapper = work/'sed/sed'
                    wrapper.write_text('#!/bin/sh\nPATH='+str(work/'real')+':$PATH\nexport PATH\nexec /usr/bin/valgrind --leak-check=full --show-leak-kinds=all --track-fds=yes --trace-children=yes --log-file='+str(work/'memory/%p.log')+' sed "$@"\n')
                    wrapper.chmod(0o755)
                else:(work/'sed/sed').symlink_to(binary)
                done = subprocess.run(['/bin/bash','-c','exec 9>&2; exec /bin/bash "$1"',
                    'sed-stdin-test',str(script)],cwd=work,stdin=subprocess.DEVNULL,capture_output=True,timeout=90,
                    env={**prerequisite_environment,'PATH':str(work/'sed')+':'+str(work/'deps')+':/usr/bin:/bin',
                         'HOME':directory,'TMPDIR':directory,'LC_ALL':'C','LANGUAGE':'C','TZ':'UTC0',
                         'srcdir':str(work/'source'),'SHELL':'/bin/bash','KEEP':'yes'})
                for name,data in (('stdout',done.stdout),('stderr',done.stderr)):(saved/name).write_bytes(data)
                directories = [p for p in work.glob('gt-stdin.sh.*') if p.is_dir()]
                assert len(directories)==1
                testdir = directories[0]
                assert done.stdout==('Not removing temporary directory '+str(testdir)+'\n').encode()
                assert (testdir/'stdin-in').read_bytes()==fixture_bytes
                alias = testdir/'stdin'
                if fixture=='private-alias':
                    assert alias.is_symlink() and str(alias.readlink())=='stdin-in'
                    assert alias.read_bytes()==fixture_bytes
                else:assert not alias.exists() and not alias.is_symlink()
                snapshot = {}
                for n in ('stdin-in','stdin-out1','stdin-out2'):
                    shutil.copy2(testdir/n,saved/n)
                    snapshot[n]={'path':str((saved/n).relative_to(ROOT)),'sha256':fingerprint(saved/n),
                                 'contents':(saved/n).read_bytes().hex()}
                shutil.copytree(work/'memory',saved/'memory')
                memory = []
                for p in sorted((saved/'memory').glob('*.log')):
                    text = p.read_text()
                    commands = re.findall(r'^==[0-9]+== Command: (.*)$',text,re.M)
                    assert len(commands)==1 and commands[0].split()[0]=='sed'
                    memory.append({**runner.parse_memory_log(text,p.stem,exec_only=True),
                                   'log':str(p.relative_to(ROOT)),'sha256':fingerprint(p)})
                clean = bool(memory) and all(m['complete_exec_log'] and m['errors']==0 and
                    m['non_inherited_descriptors']==0 and not any(m['heap_bytes'].get(k,0)
                    for k in ('definitely lost','indirectly lost','possibly lost')) for m in memory)
                expected_error = b'cat: stdin: No such file or directory\n' if fixture=='unchanged' else b''
                assertions = done.returncode==0 and not (testdir/'stdin-out1').read_bytes() and not (testdir/'stdin-out2').read_bytes()
                outcomes[key]={'status':done.returncode,'assertions_pass':assertions,'fixture_alias':str(alias.readlink()) if alias.is_symlink() else None,
                    'diagnostic_matches':done.stderr==expected_error,'stdout':done.stdout.hex(),'stderr':done.stderr.hex(),
                    'streams':{n:{'path':str((saved/n).relative_to(ROOT)),'sha256':fingerprint(saved/n)} for n in ('stdout','stderr')},
                    'init':{'path':str((saved/'init.sh').relative_to(ROOT)),'sha256':fingerprint(saved/'init.sh')},
                    'snapshot':snapshot,'memory':memory,'memory_clean':clean if instrument else None}
            assert prepare()==dependencies and all(fingerprint(p)==h for p,h in inputs.items())
    matched = all(o['assertions_pass'] and o['diagnostic_matches'] for o in outcomes.values()) and outcomes['rboxc-valgrind']['memory_clean']
    results.append({'fixture_profile':fixture,'fixture_valid':fixture=='private-alias',
                    'matched':matched,'pass':matched and fixture=='private-alias','outcomes':outcomes})
report = {**profile.metadata(),'scope':'The unchanged original exits successfully despite cat reading a missing stdin fixture; that outcome is retained and is not a valid-input pass. The separate private framework overlay creates stdin -> stdin-in after setup. Original script, inline input and assertions remain unchanged. KEEP=yes preserves private test files for byte checks; only Sed invocations are instrumented.',
          'script':entry['script'],'source_sha256':entry['source_sha256'],
          'inputs':{str(p):h for p,h in inputs.items()},'prerequisites':prerequisites,
          'fixture_bytes':fixture_bytes.hex(),'framework_anchor':anchor,'framework_addition':addition,
          'driver_sha256':fingerprint(Path(__file__)),'passed':sum(r['pass'] for r in results),
          'matched':sum(r['matched'] for r in results),'total':2,'results':results}
profile.report.write_text(json.dumps(report,indent=2)+'\n')
print('Matched',report['matched'],'profiles; private fixture passes:',report['passed'])
raise SystemExit(report['matched']!=2 or report['passed']!=1)
