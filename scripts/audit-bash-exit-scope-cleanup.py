#!/usr/bin/env python3
"""Audit substitution owners, EXIT scope and every preserved diagnostic draft."""
# SPDX-License-Identifier: GPL-3.0-or-later
import hashlib, importlib.util, json, re, subprocess, sys
from pathlib import Path

ROOT=Path(__file__).resolve().parents[1]
sys.path.insert(0,str(ROOT/'tests/gnu'))
spec=importlib.util.spec_from_file_location('reviewed',ROOT/'tests/gnu/reviewed-original.py')
runner=importlib.util.module_from_spec(spec);spec.loader.exec_module(runner)
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
target=ROOT/'evidence/bash-exit-scope-status-validation.json';assert not target.exists()
binary=ROOT/'target/bash-exit-scope-status-candidate/release/rboxc';digest=sha(binary)
assert sha(ROOT/'target/bash-exit-scope-status-repro/release/rboxc')==digest
assert len(subprocess.check_output([binary,'--list']).splitlines())==187
assert sha(ROOT/'target/release/rboxc')=='8979835878e7dbe575440b5d0c7c86a01c75782ff5ddc735707ccada50e88e86'
raw={};reports={}

def pin(path,expected=None):
    p=Path(path)
    if not p.is_absolute():p=ROOT/p
    key=str(p.relative_to(ROOT)) if p.is_relative_to(ROOT) else str(p)
    h=raw.get(key) or sha(p)
    if expected is not None:assert h==expected,str(p)
    raw[key]=h
    return h

def report(name):
    p=ROOT/f'evidence/{name}.json';r=json.loads(p.read_text())
    reports[str(p.relative_to(ROOT))]=pin(p)
    return r

for name,driver in [('bash-function-substitution-cleanup','bash_function_substitution_cleanup'),
                    ('bash-exit-scope-cleanup','bash_exit_scope_cleanup')]:
    profile=report(name);pin(f'scripts/{driver}.py',profile['driver_sha256'])
    for record in profile['files']:
        for field in ('original','source','object','log'):pin(record[field],record[field+'_sha256'])
archive=ROOT/'build/bash-exit-scope-cleanup/libbuiltins.a';pin(archive)
assert hashlib.sha256(subprocess.check_output(['ar','p',archive,'evalstring.o'])).hexdigest()==sha(ROOT/'build/bash-exit-scope-cleanup/builtins/evalstring.o')
before=report('bash-before-function-substitution-cleanup-link');after=report('bash-link')
assert before['original_inputs']==after['original_inputs']
assert before['helper_inputs'].keys()==after['helper_inputs'].keys()
changed=[p for p in before['helper_inputs'] if before['helper_inputs'][p]!=after['helper_inputs'][p]]
assert changed==['build/helpers/bash-013-subst.o','build/helpers/bash-019-unwind_prot.o','build/helpers/bash-041-libbuiltins.a']
for p,h in after['helper_inputs'].items():pin(p,h)
assert b'rboxc_bash_add_unwind_protect_heap' in subprocess.check_output(['nm','-g','--defined-only',ROOT/'build/helpers/bash-019-unwind_prot.o'])

def memory(m,strict):
    pin(m['log'],m['sha256']);text=(ROOT/m['log']).read_text()
    pids=set(re.findall(r'^==([0-9]+)==',text,re.M));assert len(pids)==1
    parsed=runner.parse_memory_log(text,pids.pop())
    assert all(m[k]==v for k,v in parsed.items())
    if strict:
        assert m['complete'] and m['clean'] and parsed['errors']==0
        assert parsed['non_inherited_descriptors']==0
        assert not any(parsed['heap_bytes'].get(k,0) for k in ('definitely lost','indirectly lost','possibly lost'))

for name,total in [('contract',32),('context',12),('trap-state',32),('return-trap',32),('errexit',32),
                   ('behavior',44),('adapters',69),('smoke',428),('dispatch',11)]:
    r=report('bash-exit-scope-status-'+name)
    assert r.get('complete',True) and r['binary_sha256']==digest
    assert r['passed']==r['total']==len(r['results'])==total and all(x['pass'] for x in r['results'])
    for p,h in r.get('inputs',{}).items():pin(p,h)
    for result in r['results']:
        for key,outcome in result.get('outcomes',{}).items():
            for m in outcome.get('memory',[]):memory(m,key=='rboxc-valgrind')
        if result.get('log'):pin(result['log'])
        if name=='context':
            assert result['equivalent']
            assert bytes.fromhex(result['outcomes']['gnu']['stderr'])

original=report('bash-exit-scope-status-original');audit=report('bash-exit-scope-status-original-validation')
assert original['complete'] and original['total']==5 and original['binary_sha256']==audit['binary_sha256']==digest
assert audit['original_sha256']==reports['evidence/bash-exit-scope-status-original.json']
assert {x['selection'] for x in original['results'] if not x['pass']}<= {'set-e'}
assert original['passed'] in (4,5)
for path,h in audit['raw'].items():pin(path,h)
for finding in audit['open_findings']:
    m=finding['memory'];text=(ROOT/finding['log']).read_text()
    assert finding['selection']=='set-e' and m['errors']==m['non_inherited_descriptors']==2
    assert not any(m['heap_bytes'].get(k,0) for k in ('definitely lost','indirectly lost','possibly lost'))
    assert 'default action of signal 13 (SIGPIPE)' in text and 'rboxc_bash_echo_builtin' in text
    assert set(re.findall(r'Open file descriptor ([0-9]+):',text))=={'0','1'}

# Direct helper contracts cover normal/exit ownership and late close errors.
for name,total in [('bash-exit-scope-cleanup-pipe',17),('bash-exit-scope-final-backup',7)]:
    r=report(name);assert r['passed']==r['total']==total
    assert r['runtime_source_sha256']==sha(ROOT/'src/shell_arguments.c')
    for result in r['results']:
        assert result['pass']
        for outcome in result['outcomes']:
            assert outcome['pass']
            if outcome.get('log'):
                pin(outcome['log'],outcome['sha256'])
                text=(ROOT/outcome['log']).read_text();pid,=set(re.findall(r'^==([0-9]+)==',text,re.M))
                parsed=runner.parse_memory_log(text,pid,exec_only=True)
                assert parsed==outcome['memory'] and outcome['memory_clean']

# Intermediate candidates are immutable evidence, never final acceptance.
for name,passed,total in [('bash-function-substitution-cleanup-contract',20,32),
                         ('bash-function-substitution-owner-contract',24,32),
                         ('bash-exit-scope-cleanup-contract',28,32),
                         ('bash-exit-scope-final-contract',28,32),
                         ('bash-exit-scope-final-context',0,12)]:
    r=report(name);assert r['complete'] and r['passed']==passed and r['total']==total
    pin(r['binary'],r['binary_sha256'])
    for result in r['results']:
        assert result['equivalent']
        for key,outcome in result['outcomes'].items():
            for m in outcome['memory']:memory(m,False)
for name in ('bash-function-substitution-initial-cleanup','bash-exit-scope-initial-cleanup','bash-exit-scope-pre-status-cleanup'):
    r=report(name);pin(r['driver_snapshot'],r['driver_sha256'])
    for record in r['files']:
        for field in ('original','source','object','log'):pin(record[field],record[field+'_sha256'])
for name,audit_name in [
        ('bash-function-substitution-cleanup-original','bash-function-substitution-original-validation'),
        ('bash-function-substitution-regression-original','bash-function-substitution-regression-validation'),
        ('bash-exit-scope-cleanup-original','bash-exit-scope-cleanup-original-validation'),
        ('bash-exit-scope-final-original','bash-exit-scope-final-original-validation')]:
    r=report(name);a=report(audit_name)
    assert r['complete'] and a['original_sha256']==reports[f'evidence/{name}.json']
    pin(r['binary'],r['binary_sha256'])
    for path,h in a['raw'].items():pin(path,h)
for p in ('scripts/entry_provider_helpers.py','src/shell_arguments.c','tests/bash-original.py',
          'evidence/raw/bash-substitution-original-inventory.json',
          'evidence/raw/bash-exit-scope-context-initial-driver.py',
          'evidence/raw/bash-before-function-substitution-shell-arguments.c'):pin(p)
target.write_text(json.dumps({
    'scope':'No-fork substitution files and saved stdout retain exit owners; anonymous-file payloads and assignment descriptors live on the heap. Explicit heap-only callbacks discard expansion/getopt/PIPESTATUS copies while EXIT retains GNU function scope. The builtins archive contains the actual adapted evaluator. Invalid dup2 sources preserve EBADF without a duplicate-descriptor syscall. All 140 focused contracts and shared checks pass on the byte-identical candidate. Five original scripts retain unchanged GNU assertions; any expected set-e SIGPIPE process family remains outside strict Valgrind counts. Intermediate diagnostic profiles remain preserved. Full Bash/GNU coverage is unfinished.',
    'binary':str(binary),'binary_sha256':digest,'size_bytes':binary.stat().st_size,
    'byte_identical_rebuild':True,'changed_helpers':changed,'focused_contracts':140,
    'strict_original_scripts':original['passed'],'clean_original_processes':len(audit['processes']),
    'open_original_processes':len(audit['open_processes']),'reports':reports,'raw':raw,'pass':True,
},indent=2)+'\n')
print('PASS: substitution/EXIT ownership; 140 focused contracts;',original['passed'],'strict original scripts')
