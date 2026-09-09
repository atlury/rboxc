#!/usr/bin/env python3
"""Audit executable-name dispatch and explicitly bounded GNU baselines."""
# SPDX-License-Identifier: GPL-3.0-or-later
import hashlib,importlib.util,json,re,subprocess,sys
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
sys.path.insert(0,str(ROOT/'tests/gnu'))
spec=importlib.util.spec_from_file_location('reviewed',ROOT/'tests/gnu/reviewed-original.py')
runner=importlib.util.module_from_spec(spec);spec.loader.exec_module(runner)
target=ROOT/'evidence/executable-name-validation.json';assert not target.exists()
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
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
    errors=list(re.finditer('ERROR SUMMARY:',text));fds=list(re.finditer('FILE DESCRIPTORS:',text))
    images=list(re.finditer(r'^==[0-9]+== Command:',text,re.M))
    complete=bool(errors) and len(errors)==len(fds) and (not images or errors[-1].start()>images[-1].start() and fds[-1].start()>images[-1].start())
    assert m['complete']==complete
    clean=complete and parsed['errors']==parsed['non_inherited_descriptors']==0 and not any(parsed['heap_bytes'].get(k,0) for k in ('definitely lost','indirectly lost','possibly lost'))
    assert m['clean']==clean
    if strict:assert clean
binary=ROOT/'target/executable-name-candidate/release/rboxc';digest=pin(binary)
assert pin('target/executable-name-repro/release/rboxc')==digest
assert len(subprocess.check_output([binary,'--list']).splitlines())==187
pin('target/release/rboxc','8979835878e7dbe575440b5d0c7c86a01c75782ff5ddc735707ccada50e88e86')
source=report('executable-name-source')
for p,h in source['inputs'].items():pin(p,h)
before=(ROOT/'evidence/raw/executable-name-before-main.rs').read_text()
after=(ROOT/'src/main.rs').read_text()
start=after.index('    #[cfg(target_os = "linux")]')
end=after.index('    let rbox_invocation',start)
assert after[:start]+after[end:]==before
link=report('bash-link')
for p,h in link['helper_inputs'].items():pin(p,h)
for name,total in [('smoke',428),('dispatch',11),('behavior',44),('adapters',69)]:
    r=report('executable-name-'+name)
    assert r.get('complete',True) and r['binary_sha256']==digest
    assert r['passed']==r['total']==len(r['results'])==total and all(x['pass'] for x in r['results'])
    for p,h in r.get('inputs',{}).items():pin(p,h)
    for result in r['results']:
        for key,o in result.get('outcomes',{}).items():
            for m in o.get('memory',[]):memory(m,key=='rboxc-valgrind')
        if result.get('log'):pin(result['log'])
helper=report('bash-printenv-helper');pin('scripts/prepare-bash-printenv-helper.py',helper['driver_sha256'])
for p,h in helper['inputs'].items():pin(p,h)
pin(helper['binary'],helper['binary_sha256']);pin(helper['log'],helper['log_sha256'])
focused={};fields=('status','stdout','stderr','tree')
for name,normal_count,strict_count in [('executable-name-before-contract',4,0),('executable-name-contract',28,8)]:
    r=report(name);assert r['complete'] and r['total']==r['planned_total']==28 and r['passed']==strict_count
    pin(r['binary'],r['binary_sha256'])
    for p,h in r['inputs'].items():pin(p,h)
    normal=sum(all(x['outcomes']['gnu'][k]==x['outcomes']['rboxc'][k] for k in fields) for x in r['results'])
    instrumented=sum(all(x['outcomes']['gnu-valgrind'][k]==x['outcomes']['rboxc-valgrind'][k] for k in fields) for x in r['results'])
    assert normal==normal_count and instrumented==28
    count=0
    for x in r['results']:
        alias,label=x['name'].split(':');o=x['outcomes']
        for key,v in o.items():
            for m in v['memory']:memory(m,name=='executable-name-contract' and key=='rboxc-valgrind')
        if name!='executable-name-contract':continue
        assert r['binary_sha256']==digest
        count+=len(o['rboxc-valgrind']['memory'])
        assert all(v['stderr']=='' and v['tree']=={} for v in o.values())
        if label.startswith('printenv-'):
            assert x['pass'] and all(v['status']==(1 if label=='printenv-clean-login' else 0) for v in o.values())
            assert bytes.fromhex(o['gnu']['stdout'])==(b'' if label=='printenv-clean-login' else b'fixture\n')
        else:
            names={'custom':'specialname','known-applet':'cat','dispatcher-name':'rboxc','login':'-specialname','posix-name':'sh'}
            assert not x['pass'] and all(v['status']==0 for v in o.values())
            assert bytes.fromhex(o['gnu']['stdout'])==(names[label]+'\nposix:'+('on' if label=='posix-name' else 'off')+'\n').encode()
            assert bytes.fromhex(o['gnu-valgrind']['stdout'])==('<fixture>/exec/'+alias+'\nposix:'+('on' if alias=='sh' else 'off')+'\n').encode()
    if name=='executable-name-contract':assert count==68
    focused[name]={'normal_matches':normal,'instrumented_matches':instrumented,'strict_matches':strict_count}
initial=report('bash-builtins-original')
assert initial['complete'] and initial['passed']==0 and initial['total']==1
old,=initial['results']
for o in old['outcomes'].values():
    assert o['status']==0 and not o['timed_out']
    for p,h in o['raw'].items():pin(p,h)
    for m in o['memory']:memory(m)
def output(o):return next(ROOT/p for p in o['raw'] if p.endswith('/actual')).read_bytes()
assert output(old['outcomes']['rboxc'])==output(old['outcomes']['gnu']).replace(b'\nspecialname\n-specialname\n',b"\ncoreutils: unknown program 'specialname'\ncoreutils: unknown program '-specialname'\n")
original=report('bash-executable-name-original');audit=report('bash-executable-name-original-validation')
assert original['complete'] and original['binary_sha256']==digest and original['passed']==0 and original['total']==1
assert audit['original_sha256']==reports['evidence/bash-executable-name-original.json']
assert audit['assertion_baselines']==['builtins'] and not audit['open_findings']
for p,h in audit['raw'].items():pin(p,h)
row,=original['results'];normal=output(row['outcomes']['gnu'])
expected=Path('/opt/src/bash-5.3/tests/builtins.right').read_bytes()
baseline=expected.replace(b'source [-p path] filename [argument>',b'source [-p path] filename [arguments>',1)
baseline=baseline.replace(b'./builtins11.sub: line 39: ulimit: max user processes: cannot modify limit: Operation not permitted\n',b'')
assert normal==baseline and normal!=expected and output(row['outcomes']['rboxc'])==normal
for key,o in row['outcomes'].items():
    assert o['status']==0 and not o['timed_out'] and not o['expected_output_matches']
    for m in o['memory']:memory(m,key=='rboxc-valgrind')
    if key.endswith('-valgrind'):
        directory=audit['private_directories']['builtins:'+key]
        invocation=(directory+'/exec/bash\n').encode()
        assert output(o)==normal.replace(b'specialname\n-specialname\n',invocation*2)
assert len(row['outcomes']['rboxc-valgrind']['memory'])==111
coverage=report('bash-executable-name-coverage')
assert coverage['original_scripts']==67 and coverage['strict_original_scripts']==54
assert coverage['recipe_states']['pending-review']==17 and coverage['clean_candidate_processes']==12765
for p in ('tests/bash-original.py','scripts/audit-bash-reviewed-batch.py','evidence/bash-builtins-review.json',
    'evidence/raw/bash-builtins-original-inventory.json','evidence/raw/bash-builtins-original-driver.py',
    'evidence/raw/bash-builtins-standalone-inventory.json','evidence/raw/bash-builtins-standalone-driver.py'):
    pin(p)
target.write_text(json.dumps({'scope':'Linux named-applet execution retains GNU custom argv[0]. All 28 normal and 28 instrumented focused comparisons match their respective GNU mode; 20 retain the explicitly checked Valgrind argv[0] baseline, while eight are strict across modes. All 68 candidate process logs are clean. The unchanged builtins recipe has 111 clean candidate logs but remains outside strict original counts because its two native expected-output differences and Valgrind invocation-name baseline are retained exactly. Candidate rebuild is byte-identical, prior findings are preserved and installed release is unchanged.',
    'binary':str(binary),'binary_sha256':digest,'size_bytes':binary.stat().st_size,
    'byte_identical_rebuild':True,'focused':focused,'clean_focused_processes':68,
    'strict_focused_comparisons':8,'focused_instrumentation_baselines':20,
    'clean_builtin_processes_outside_strict_counts':111,'reports':reports,'raw':raw,
    'accounting_pass':True,'dispatch_fix_pass':True,'full_bash_complete':False},indent=2)+'\n')
print('PASS: custom executable names; 28 mode comparisons; 68 clean focused and 111 clean original process logs')
