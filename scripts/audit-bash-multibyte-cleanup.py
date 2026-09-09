#!/usr/bin/env python3
"""Audit multibyte cleanup, private locales and native helper findings."""
# SPDX-License-Identifier: GPL-3.0-or-later
import hashlib,importlib.util,json,re,subprocess,sys
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
sys.path.insert(0,str(ROOT/'tests/gnu'))
spec=importlib.util.spec_from_file_location('reviewed',ROOT/'tests/gnu/reviewed-original.py')
runner=importlib.util.module_from_spec(spec);spec.loader.exec_module(runner)
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
target=ROOT/'evidence/bash-multibyte-validation.json';assert not target.exists()
binary=ROOT/'target/bash-multibyte-cleanup-candidate/release/rboxc';digest=sha(binary)
assert sha(ROOT/'target/bash-multibyte-cleanup-repro/release/rboxc')==digest
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
profile=report('bash-multibyte-cleanup');pin('scripts/bash_multibyte_cleanup.py',profile['driver_sha256'])
for r in profile['files']:
    for field in ('original','source','object','log'):pin(r[field],r[field+'_sha256'])
before=report('bash-before-multibyte-cleanup-link');after=report('bash-link')
assert before['original_inputs']==after['original_inputs']
assert before['helper_inputs'].keys()==after['helper_inputs'].keys()
changed=[p for p in before['helper_inputs'] if before['helper_inputs'][p]!=after['helper_inputs'][p]]
assert changed==['build/helpers/bash-006-execute_cmd.o','build/helpers/bash-013-subst.o']
for p,h in after['helper_inputs'].items():pin(p,h)
for name,total in [('contract',24),('behavior',44),('adapters',69),('smoke',428),('dispatch',11)]:
    r=report('bash-multibyte-'+name)
    assert r.get('complete',True) and r['binary_sha256']==digest
    assert r['passed']==r['total']==len(r['results'])==total and all(x['pass'] for x in r['results'])
    for p,h in r.get('inputs',{}).items():pin(p,h)
    for result in r['results']:
        for key,outcome in result.get('outcomes',{}).items():
            for m in outcome.get('memory',[]):memory(m,key=='rboxc-valgrind')
        if result.get('log'):pin(result['log'])
locale=report('bash-intl-locale-fixture')
repro_path=ROOT/'build/bash-intl-locales-repro/build.json';pin(repro_path)
repro=json.loads(repro_path.read_text());pin('scripts/prepare-bash-intl-locales.py',repro['driver_sha256'])
assert locale['archive_sha256']==repro['archive_sha256'] and locale['inputs']==repro['inputs']
for fixture in (locale,repro):
    pin(fixture['archive'],fixture['archive_sha256'])
    for p,h in fixture['inputs'].items():pin(p,h)
    assert len(fixture['builds'])==6 and all(r['status']==0 for r in fixture['builds'])
    for row in fixture['builds']:pin(row['log'],row['log_sha256'])
for name,audit_name,expected_findings in [('bash-intl-original','bash-intl-validation',8),
    ('bash-multibyte-intl-original','bash-multibyte-intl-validation',4),
    ('bash-multibyte-regression-original','bash-multibyte-regression-validation',0)]:
    r=report(name);a=report(audit_name)
    assert r['complete'] and a['original_sha256']==reports[f'evidence/{name}.json']
    pin(r['binary'],r['binary_sha256']);assert len(a['open_findings'])==expected_findings
    for p,h in a['raw'].items():pin(p,h)
    if name=='bash-multibyte-regression-original':
        assert r['passed']==r['total']==3 and r['binary_sha256']==digest
        assert {x['selection'] for x in r['results']}=={'case','quote','ifs-posix'}
        continue
    assert r['total']==1 and r['passed']==0 and not a['assertion_baselines']
    assert a['explicit_memory_open']==['intl']
    result,=r['results'];native_counts={}
    for key,o in result['outcomes'].items():
        assert o['status']==0 and o['expected_output_matches'] and not o['timed_out']
        output=next(ROOT/p for p in o['raw'] if p.endswith('/actual')).read_bytes()
        assert output.count(b'Passed all 1770 Unicode tests\n')==1
        assert o['private_locale']['source_sha256']==locale['archive_sha256']
        native=[]
        for m in o['memory']:
            text=memory(m)
            if re.search(r'^==[0-9]+== Command: /tmp/rboxc-bash-original-[a-z0-9_]{8}/exec/locale -a$',text,re.M):
                assert m['complete'] and m['errors']==3 and m['non_inherited_descriptors']==0
                assert m['heap_bytes']=={'definitely lost':306,'indirectly lost':271,'possibly lost':0,'still reachable':0}
                native.append(m)
            elif key=='rboxc-valgrind' and name=='bash-multibyte-intl-original':
                memory(m,True)
        if key.endswith('-valgrind'):
            assert len(o['memory'])==87 and len(native)==4
            native_counts[key]=len(native)
    assert native_counts=={'gnu-valgrind':4,'rboxc-valgrind':4}
    if name=='bash-multibyte-intl-original':
        assert r['binary_sha256']==digest
        assert sum(m['clean'] for m in result['outcomes']['rboxc-valgrind']['memory'])==83
coverage=report('bash-multibyte-coverage')
assert coverage['strict_original_scripts']==54 and coverage['original_scripts']==66
for p in ('scripts/entry_provider_helpers.py','tests/bash-original.py','scripts/audit-bash-reviewed-batch.py',
          'evidence/raw/bash-intl-original-inventory.json','evidence/raw/bash-intl-original-driver.py'):pin(p)
target.write_text(json.dumps({'scope':'All internationalization assertions, including 1770 Unicode checks, match the original normally and under Valgrind. After cleanup, all 83 candidate applet logs are clean; four shared native locale helper logs retain matching GNU findings, so the whole original remains outside strict coverage. Twenty-four focused comparisons, three related original recipes and shared checks pass. Both candidate and private locale archive rebuild byte-identically; system locale/profile files remain unchanged.',
    'binary':str(binary),'binary_sha256':digest,'size_bytes':binary.stat().st_size,
    'byte_identical_rebuild':True,'byte_identical_locale_rebuild':True,'changed_helpers':changed,
    'focused_contracts':24,'clean_intl_applet_logs_outside_strict_counts':83,
    'native_locale_helper_findings':4,'reports':reports,'raw':raw,'pass':True},indent=2)+'\n')
print('PASS: internationalization cleanup; 24 contracts; 83 clean applet logs and 4 native helper findings')
