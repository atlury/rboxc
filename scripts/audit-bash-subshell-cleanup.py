#!/usr/bin/env python3
"""Audit pipeline/subshell ownership and classify the preserved SIGPIPE profile."""
# SPDX-License-Identifier: GPL-3.0-or-later
import hashlib,importlib.util,json,re,subprocess,sys
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
sys.path.insert(0,str(ROOT/'tests/gnu'))
spec=importlib.util.spec_from_file_location('reviewed',ROOT/'tests/gnu/reviewed-original.py')
runner=importlib.util.module_from_spec(spec);spec.loader.exec_module(runner)
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
target=ROOT/'evidence/bash-subshell-cleanup-validation.json';assert not target.exists()
binary=ROOT/'target/bash-subshell-cleanup-candidate/release/rboxc';digest=sha(binary)
assert sha(ROOT/'target/bash-subshell-cleanup-repro/release/rboxc')==digest
assert len(subprocess.check_output([binary,'--list']).splitlines())==187
assert sha(ROOT/'target/release/rboxc')=='8979835878e7dbe575440b5d0c7c86a01c75782ff5ddc735707ccada50e88e86'
raw={};reports={}
def pin(path,expected=None):
    p=Path(path)
    if not p.is_absolute():p=ROOT/p
    actual=sha(p)
    if expected is not None:assert actual==expected,str(p)
    raw[str(p.relative_to(ROOT)) if p.is_relative_to(ROOT) else str(p)]=actual
    return actual
profile_path=ROOT/'evidence/bash-subshell-cleanup.json';profile=json.loads(profile_path.read_text())
pin('scripts/bash_subshell_cleanup.py',profile['driver_sha256']);pin('evidence/bash-native-cleanup.json',profile['baseline_profile_sha256'])
for r in profile['files']:
    for field in ('original','source','object','log'):pin(r[field],r[field+'_sha256'])
reports[str(profile_path.relative_to(ROOT))]=pin(profile_path)
a=json.loads((ROOT/'evidence/bash-before-subshell-cleanup-link.json').read_text())['helper_inputs']
b=json.loads((ROOT/'evidence/bash-link.json').read_text())['helper_inputs'];assert a.keys()==b.keys()
changed=[p for p in a if a[p]!=b[p]]
assert changed==['build/helpers/bash-006-execute_cmd.o','build/helpers/bash-012-jobs.o','build/helpers/bash-019-unwind_prot.o','build/helpers/bash-041-libbuiltins.a']
for p,h in b.items():pin(p,h)
selections={};clean=[];open_processes=[];signal_logs=[]
for i in range(4):
    path=ROOT/f'evidence/bash-subshell-cleanup-batch-{i}.json';r=json.loads(path.read_text())
    audit_path=ROOT/f'evidence/bash-subshell-cleanup-batch-{i}-validation.json';audit=json.loads(audit_path.read_text())
    assert r['complete'] and r['total']==len(r['results'])==r['planned_total']
    assert r['binary_sha256']==audit['binary_sha256']==digest
    assert audit['original_sha256']==sha(path)
    for result in r['results']:
        assert result['selection'] not in selections;selections[result['selection']]=result['pass']
        assert all(o['expected_output_matches'] and not o['timed_out'] for o in result['outcomes'].values())
        for m in result['outcomes']['rboxc-valgrind']['memory']:
            if m['clean']:continue
            # The only remaining finding must be the expected default signal:
            # no heap error/loss and only the two ordinary remapped standards.
            assert result['selection']=='lastpipe'
            text=(ROOT/m['log']).read_text()
            assert 'Process terminating with default action of signal 13 (SIGPIPE)' in text
            assert m['complete'] and m['errors']==m['non_inherited_descriptors']==2
            assert not any(m['heap_bytes'].get(k,0) for k in ('definitely lost','indirectly lost','possibly lost'))
            assert set(re.findall(r'Open file descriptor (\d+):',text))=={'0','1'}
            assert 'rboxc_bash_echo_builtin' in text and 'rboxc_bash_owned_dup2' in text
            signal_logs.append(m)
    for p,h in audit['raw'].items():pin(p,h)
    clean.extend(audit['processes']);open_processes.extend(audit['open_processes'])
    reports[str(path.relative_to(ROOT))]=pin(path);reports[str(audit_path.relative_to(ROOT))]=pin(audit_path)
assert len(selections)==41 and sum(selections.values())==40
assert {n for n,v in selections.items() if not v}=={'lastpipe'}
assert selections['set-e'] and selections['posixexp2'] and signal_logs
assert len({r['log'] for r in clean})==len(clean)
for name,total in [('behavior',44),('adapters',69),('smoke',428),('dispatch',11),('errexit-contract',32)]:
    path=ROOT/('evidence/bash-subshell-errexit-contract.json' if name=='errexit-contract' else f'evidence/bash-subshell-cleanup-{name}.json')
    r=json.loads(path.read_text());assert r.get('complete',True) and r['binary_sha256']==digest
    assert r['passed']==r['total']==len(r['results'])==total and all(x['pass'] for x in r['results'])
    for result in r['results']:
        for key,outcome in result.get('outcomes',{}).items():
            for m in outcome.get('memory',[]):
                p=ROOT/m['log'];pin(p,m['sha256']);text=p.read_text();pids=set(re.findall(r'^==([0-9]+)==',text,re.M));assert len(pids)==1
                parsed=runner.parse_memory_log(text,pids.pop());assert all(m[k]==v for k,v in parsed.items())
                if key=='rboxc-valgrind':assert m['complete'] and m['clean'] and parsed['errors']==0 and parsed['non_inherited_descriptors']==0 and not any(parsed['heap_bytes'].get(k,0) for k in ('definitely lost','indirectly lost','possibly lost'))
        if result.get('log'):pin(result['log'])
    reports[str(path.relative_to(ROOT))]=pin(path)
for p in ['scripts/entry_provider_helpers.py','tests/bash-errexit-contract.py']:pin(p)
target.write_text(json.dumps({'scope':'Four isolated Bash helpers repair ownership of lastpipe backups, fully discarded frozen job tables, and heap payloads dropped by an errexit unwind frame. All 41 reviewed original scripts match GNU assertions; 40 have strictly clean candidate process families. Lastpipe retains only the expected default SIGPIPE child and its two standard descriptors, which the kernel releases when the process terminates. The complete lastpipe family remains outside strict Valgrind counts. No signal handler or disposition was changed to obtain a clean log. Thirty-two focused errexit contracts and shared checks pass. Full Bash/GNU acceptance remains open.',
 'binary':str(binary),'binary_sha256':digest,'size_bytes':binary.stat().st_size,'byte_identical_rebuild':True,'changed_helpers':changed,'strict_original_scripts':40,'clean_candidate_processes':len(clean),'signal_profile_processes':len(open_processes),'signal_logs':signal_logs,'ownership_contracts':32,'reports':reports,'raw':raw,'pass':True},indent=2)+'\n')
print('PASS: subshell ownership;',len(clean),'clean original processes; expected SIGPIPE profile separate')
