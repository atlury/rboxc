#!/usr/bin/env python3
"""Audit parameter owners, external redirections and duplicated targets."""
# SPDX-License-Identifier: GPL-3.0-or-later
import hashlib,importlib.util,json,re,subprocess,sys
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
sys.path.insert(0,str(ROOT/'tests/gnu'))
spec=importlib.util.spec_from_file_location('reviewed',ROOT/'tests/gnu/reviewed-original.py')
runner=importlib.util.module_from_spec(spec);spec.loader.exec_module(runner)
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
target=ROOT/'evidence/bash-parameter-descriptor-validation.json';assert not target.exists()
binary=ROOT/'target/bash-parameter-descriptor-cleanup-candidate/release/rboxc';digest=sha(binary)
assert sha(ROOT/'target/bash-parameter-descriptor-cleanup-repro/release/rboxc')==digest
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
    p=ROOT/f'evidence/{name}.json';reports[str(p.relative_to(ROOT))]=pin(p)
    return json.loads(p.read_text())
def memory(m,strict=False):
    pin(m['log'],m['sha256']);text=(ROOT/m['log']).read_text()
    pid,=set(re.findall(r'^==([0-9]+)==',text,re.M))
    parsed=runner.parse_memory_log(text,pid)
    assert all(m[k]==v for k,v in parsed.items())
    if strict:
        assert m['complete'] and m['clean'] and parsed['errors']==parsed['non_inherited_descriptors']==0
        assert not any(parsed['heap_bytes'].get(k,0) for k in ('definitely lost','indirectly lost','possibly lost'))
    return text
for name,driver in [('bash-parameter-cleanup','bash_parameter_cleanup'),
                    ('bash-redirection-command-cleanup','bash_redirection_command_cleanup')]:
    profile=report(name);pin(f'scripts/{driver}.py',profile['driver_sha256'])
    for r in profile['files']:
        for field in ('original','source','object','log'):pin(r[field],r[field+'_sha256'])
before=report('bash-before-parameter-cleanup-link');after=report('bash-link')
assert before['original_inputs']==after['original_inputs']
assert before['helper_inputs'].keys()==after['helper_inputs'].keys()
changed=[p for p in before['helper_inputs'] if before['helper_inputs'][p]!=after['helper_inputs'][p]]
assert changed==['build/helpers/bash-006-execute_cmd.o','build/helpers/bash-013-subst.o']
for p,h in after['helper_inputs'].items():pin(p,h)
for name,total in [('contract',40),('terminal',8),('exit-scope',12),('behavior',44),('adapters',69),('smoke',428),('dispatch',11)]:
    r=report('bash-parameter-descriptor-'+name)
    assert r.get('complete',True) and r['binary_sha256']==digest
    assert r['passed']==r['total']==len(r['results'])==total and all(x['pass'] for x in r['results'])
    for p,h in r.get('inputs',{}).items():pin(p,h)
    for result in r['results']:
        for key,outcome in result.get('outcomes',{}).items():
            for m in outcome.get('memory',[]):memory(m,key=='rboxc-valgrind')
        if result.get('log'):pin(result['log'])
for name,total in [('contract',7),('backup',7),('pipe',17)]:
    r=report('bash-duplicated-descriptor-'+name)
    assert r['passed']==r['total']==total and r['runtime_source_sha256']==sha(ROOT/'src/shell_arguments.c')
    for result in r['results']:
        assert result['pass']
        for o in result['outcomes']:
            assert o['pass']
            if o.get('log'):
                pin(o['log'],o['sha256']);text=(ROOT/o['log']).read_text()
                pid,=set(re.findall(r'^==([0-9]+)==',text,re.M))
                assert runner.parse_memory_log(text,pid,exec_only=True)==o['memory'] and o['memory_clean']
# Every batch is audited independently. Initial and intermediate findings are
# retained alongside the final profiles, never retroactively marked as passes.
batches=[('bash-parameter-descriptor-original','bash-parameter-descriptor-original-validation'),
         ('bash-duplicated-descriptor-read','bash-duplicated-descriptor-read-validation'),
         ('bash-duplicated-descriptor-jobs','bash-duplicated-descriptor-jobs-validation'),
         ('bash-parameter-cleanup-original','bash-parameter-initial-original-validation'),
         ('bash-parameter-word-cleanup-original','bash-parameter-word-original-validation'),
         ('bash-read-terminal-original','bash-read-terminal-validation'),
         ('bash-jobs-terminal-original','bash-jobs-terminal-validation')]
batch_reports={}
for name,audit_name in batches:
    r=report(name);a=report(audit_name);batch_reports[name]=(r,a)
    assert r['complete'] and a['original_sha256']==reports[f'evidence/{name}.json']
    pin(r['binary'],r['binary_sha256'])
    for p,h in a['raw'].items():pin(p,h)
original,audit=batch_reports['bash-parameter-descriptor-original']
assert original['binary_sha256']==digest and original['passed']==original['total']==5
read,read_audit=batch_reports['bash-duplicated-descriptor-read']
assert read['binary_sha256']==digest and read_audit['assertion_baselines']==['read']
assert not read_audit['open_findings']
assert all(m['clean'] for m in read['results'][0]['outcomes']['rboxc-valgrind']['memory'])
jobs,jobs_audit=batch_reports['bash-duplicated-descriptor-jobs']
assert jobs['binary_sha256']==digest and jobs_audit['killed_child_open']==['jobs']
for finding in jobs_audit['open_findings']:
    text=(ROOT/finding['log']).read_text();m=finding['memory']
    if 'default action of signal 1 (SIGHUP)' in text:
        assert not any(m['heap_bytes'].get(k,0) for k in ('definitely lost','indirectly lost','possibly lost'))
    else:
        assert m['errors'] is None and m['non_inherited_descriptors'] is None
initial_read,initial_read_audit=batch_reports['bash-read-terminal-original']
assert len(initial_read_audit['open_findings'])==4003
for f in initial_read_audit['open_findings']:
    m=f['memory'];text=(ROOT/f['log']).read_text()
    assert m['errors']==m['non_inherited_descriptors']==1
    assert re.findall(r'Open file descriptor ([0-9]+):',text)==['9']
    assert 'rboxc_bash_owned_dup2' in text and '/a.pipe' in text
    assert not any(m['heap_bytes'].get(k,0) for k in ('definitely lost','indirectly lost','possibly lost'))
for name,passed,total,driver in [('bash-parameter-cleanup-contract',28,32,'evidence/raw/bash-parameter-initial-contract-driver.py'),
                                ('bash-parameter-word-cleanup-contract',40,40,'tests/bash-parameter-contract.py')]:
    r=report(name);assert r['complete'] and r['passed']==passed and r['total']==total
    pin(r['binary'],r['binary_sha256']);pin(driver,r['inputs'][str(ROOT/'tests/bash-parameter-contract.py')])
    for result in r['results']:
        assert result['equivalent']
        for o in result['outcomes'].values():
            for m in o['memory']:memory(m)
initial=report('bash-parameter-initial-cleanup');pin(initial['driver_snapshot'],initial['driver_sha256'])
for r in initial['files']:
    for field in ('original','source','object','log'):pin(r[field],r[field+'_sha256'])
for p in ('scripts/entry_provider_helpers.py','src/shell_arguments.c','tests/bash-original.py',
          'evidence/raw/bash-before-duplicated-descriptor-shell-arguments.c',
          'evidence/raw/bash-read-jobs-original-driver.py','evidence/raw/bash-read-jobs-original-inventory.json'):pin(p)
coverage=report('bash-parameter-descriptor-coverage')
assert coverage['strict_original_scripts']==54 and coverage['original_scripts']==62
target.write_text(json.dumps({'scope':'Brace length, pattern and RHS callers retain heap ownership across diagnostics and EXIT; external redirection failures release command strings. Duplicated nonstandard targets retain exit ownership while normal close and descriptor reuse forget it. Sixty shell contracts, 31 direct descriptor checks and all shared checks pass on a byte-identical candidate. Five related original scripts pass strictly. Read has clean process logs but retains its GNU timer baseline; jobs retains intentional killed-child and SIGHUP profiles. All prior findings remain recorded. Full Bash/GNU coverage remains unfinished.',
    'binary':str(binary),'binary_sha256':digest,'size_bytes':binary.stat().st_size,
    'byte_identical_rebuild':True,'changed_helpers':changed,'focused_contracts':60,'direct_contracts':31,
    'strict_original_scripts':5,'clean_original_processes':len(audit['processes']),
    'read_clean_processes':len(read['results'][0]['outcomes']['rboxc-valgrind']['memory']),
    'reports':reports,'raw':raw,'pass':True},indent=2)+'\n')
print('PASS: parameter and descriptor owners; 60 shell and 31 direct contracts; five strict originals')
