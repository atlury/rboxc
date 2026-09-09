#!/usr/bin/env python3
"""Audit ordinary env -S cases and its external instrumentation baselines."""
# SPDX-License-Identifier: GPL-3.0-or-later
from collections import Counter
import copy,hashlib,importlib.util,json,re,sys
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
sys.path.insert(0,str(ROOT/'tests/gnu'))
spec=importlib.util.spec_from_file_location('reviewed',ROOT/'tests/gnu/reviewed-original.py')
runner=importlib.util.module_from_spec(spec);spec.loader.exec_module(runner)
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
target=ROOT/'evidence/gnu-env-s-selection-validation.json';assert not target.exists()
coverage_target=ROOT/'evidence/gnu-env-s-selection-coverage.json';assert not coverage_target.exists()
source=Path('/opt/src/coreutils-9.11');script='tests/env/env-S.pl'
snapshot=ROOT/'evidence/raw/gnu-env-s-reviewed-inventory.json'
definition,=[r for r in json.loads(snapshot.read_text()) if r['script']==script]
assert sha(source/script)==definition['sha256']
names=re.findall(r"^\s*\['([^']+)'",(source/script).read_text(),re.M)
all_cases=names+[n+'-debug' for n in names if not n.startswith('err')]
assert len(set(all_cases))==len(all_cases)==199
assert definition['cases']==[n for n in all_cases if n not in ('t5','t5-debug')]
assert definition['expected_case_count']==197 and not definition['full_suite']
raw={str(snapshot.relative_to(ROOT)):sha(snapshot)};reports={};outcomes={};mismatch_cases={}
roles={};findings=[];commands={}
def pin(path,expected=None):
    p=Path(path)
    if not p.is_absolute():p=ROOT/p
    key=str(p.relative_to(ROOT)) if p.is_relative_to(ROOT) else str(p)
    h=raw.get(key) or sha(p)
    if expected is not None:assert h==expected,str(p)
    raw[key]=h
    return h
for instrument in (False,True):
    name='gnu-env-s-selected-'+('valgrind' if instrument else 'original')
    p=ROOT/f'evidence/raw/{name}.json';d=json.loads(p.read_text());reports[str(p.relative_to(ROOT))]=pin(p)
    checkpoint=ROOT/f'evidence/raw/{name}.progress.json';pin(checkpoint)
    saved=json.loads(checkpoint.read_text())['runs'][script];context=saved['context']
    assert context['definition']==definition and context['valgrind']==instrument
    assert d['total']==1 and d['passed']==(0 if instrument else 1)
    r,=d['results'];assert all(r[k]==v for k,v in definition.items())
    assert context['shell']==sha('/bin/sh')
    pin('/bin/sh',context['shell']);pin(ROOT/'build/gnu-coreutils/lib/config.h',context['config_header'])
    pin(ROOT/'build/gnu-coreutils/src/getlimits',context['getlimits'])
    for path,h in context['drivers'].items():pin(path,h)
    for path,h in context['gnu_harness'].items():pin(source/path,h)
    for path,h in (context.get('valgrind_runtime') or {}).items():pin(path,h)
    for key in ('gnu','rboxc'):
        o=r[key];assert saved['outcomes'][key]['result']==o
        assert o['status']==(1 if instrument else 0) and o['case_count']==197 and o['case_count_pass']
        assert o['binary_sha256']==context['binaries'][key]
        binary=ROOT/('build/gnu-coreutils/src/coreutils' if key=='gnu' else 'target/bash-parameter-descriptor-cleanup-candidate/release/rboxc')
        pin(binary,o['binary_sha256']);pin(o['watchdog']['path'],o['watchdog']['sha256'])
        for path,h in saved['outcomes'][key]['logs'].items():pin(path,h)
        log=(ROOT/o['log']).read_text()
        assert log.count('RBOXC_SELECTION 197 of 199\n')==1
        assert not re.search(r'^t5(?:-debug)?\.\.\.$',log,re.M)
        if not instrument:
            assert not re.search(r'^-e: test .* mismatch',log,re.M)
            continue
        failures=re.findall(r'^-e: test (.*?): stdout mismatch',log,re.M)
        assert len(set(failures))==len(failures)==88
        diagnostics=re.findall(r'^-e: test (.*?): stderr mismatch',log,re.M)
        assert diagnostics==['err_sp2','err_sp3']
        assert not re.search(r'^-e: test .* exit status mismatch',log,re.M)
        additions=Counter(re.findall(r'^\+ (.*)$',log,re.M))
        assert additions==Counter({
            'VALGRIND_LIB=/usr/libexec/valgrind':88,
            'LD_PRELOAD=/usr/libexec/valgrind/vgpreload_core-amd64-linux.so:/usr/libexec/valgrind/vgpreload_memcheck-amd64-linux.so:':88})
        assert not re.search(r'^- ',log,re.M)
        mismatch_cases[key]=failures+diagnostics;roles[key]=Counter();commands[key]=Counter()
        assert len(o['memory'])==198
        for m in o['memory']:
            path=ROOT/m['log'];text=path.read_text();pid=path.stem
            parsed=runner.parse_memory_log(text,pid)
            assert all(m[k]==v for k,v in parsed.items())
            assert set(re.findall(r'^==([0-9]+)==',text,re.M))=={pid}
            image,=re.findall(r'^==[0-9]+== Command: (.*)$',text,re.M)
            errors=list(re.finditer('ERROR SUMMARY:',text));fds=list(re.finditer('FILE DESCRIPTORS:',text))
            assert errors and len(errors)==len(fds)
            assert errors[-1].start()>text.index('Command:') and fds[-1].start()>text.index('Command:')
            command=re.sub(r'/tmp/rboxc-upstream-[a-z0-9_]{8}/src/../real/', '<private>/real/',image)
            if command=='<private>/real/env --version':
                version_image=image.removesuffix(' --version')
            commands[key][command]+=1
            clean=m['errors']==m['non_inherited_descriptors']==0 and not any(m['heap_bytes'].get(k,0) for k in ('definitely lost','indirectly lost','possibly lost'))
            if image.startswith('/usr/bin/perl -w -T -e '):
                assert not clean and m['errors']==231 and m['non_inherited_descriptors']==0
                assert m['heap_bytes']['definitely lost']==8191 and m['heap_bytes']['possibly lost']>0
                roles[key]['native-perl-finding']+=1
                findings.append({'implementation':key,**m})
            else:
                assert clean
                role='environment-baseline' if command=='<private>/real/env' else 'diagnostic-name-baseline' if command in (r'<private>/real/env -v\ -S\ cat\ -n',r'<private>/real/env -v_-S\ cat\ -n') else 'clean-other'
                roles[key][role]+=1
        assert re.findall(r'^! (.*)$',log,re.M)==["Try 'env --help' for more information.",f"Try '{version_image} --help' for more information."]*2
        assert roles[key]==Counter({'environment-baseline':88,'diagnostic-name-baseline':2,'native-perl-finding':2,'clean-other':106})
    outcomes[name]={key:r[key] for key in ('gnu','rboxc')}
assert mismatch_cases['gnu']==mismatch_cases['rboxc'] and commands['gnu']==commands['rboxc']
pin('/usr/bin/perl') # Post-run host helper fingerprint, outside port certification.
base_path=ROOT/'evidence/gnu-suite-coverage.json';base=json.loads(base_path.read_text());pin(base_path)
coverage=copy.deepcopy(base);row,=[r for r in coverage['results'] if r['script']==script]
assert row['state']=='pending' and row['valgrind_state']=='pending'
row.update(state='partial',execution_coverage='selected-cases',selected_case_count=197,
           original_case_count=199,held_cases=['t5','t5-debug'],valgrind_state='open',
           evidence='evidence/raw/gnu-env-s-selected-original.json',
           valgrind_evidence='evidence/raw/gnu-env-s-selected-valgrind.json',
           audit=str(target.relative_to(ROOT)),binary_sha256=context['binaries']['rboxc'])
assert all(a==b for a,b in zip(base['results'],coverage['results']) if a['script']!=script)
coverage['counts']=dict(sorted(Counter(r['state'] for r in coverage['results']).items()))
coverage['valgrind_counts']=dict(sorted(Counter(r['valgrind_state'] for r in coverage['results']).items()))
coverage.update(base_report=str(base_path.relative_to(ROOT)),base_report_sha256=sha(base_path),
    scope='Pinned 733-script Coreutils ledger plus the audited 197-case env -S selection on its recorded candidate. Only that row changes. The selection is partial; instrumentation environment differences, native Perl findings and two held cases are not full-suite passes.')
coverage_target.write_text(json.dumps(coverage,indent=2)+'\n')
target.write_text(json.dumps({'scope':'All 197 selected unchanged env -S cases pass normally in GNU and Rboxc. Both instrumented runs retain exactly 88 environment-output differences consisting only of the two Valgrind-injected variables, two absolute diagnostic-name differences and two native Perl finding profiles. Command-image inventories match; 196 candidate process logs are clean, including 90 instrumentation-baseline images. The entire Valgrind selection remains open. Two historical cases are unexecuted, and installed-release certification is unchanged.',
    'binary_sha256':context['binaries']['rboxc'],'selected_cases':197,'original_cases':199,
    'held_cases':['t5','t5-debug'],'assertion_baseline_cases':mismatch_cases['rboxc'],
    'roles':roles,'native_helper_findings':findings,'clean_candidate_processes':196,
    'nonbaseline_clean_processes_including_version':106,'reports':reports,'raw':raw,
    'coverage':str(coverage_target.relative_to(ROOT)),'coverage_sha256':sha(coverage_target),
    'accounting_pass':True,'full_suite_pass':False},indent=2)+'\n')
print('Audited 197 ordinary cases; 90 instrumentation baselines; two native Perl finding profiles')
