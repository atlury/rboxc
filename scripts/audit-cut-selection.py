#!/usr/bin/env python3
"""Audit the unchanged ordinary cut matrix and every instrumented image."""
# SPDX-License-Identifier: GPL-3.0-or-later
from collections import Counter
import copy,hashlib,importlib.util,json,re,sys
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
sys.path.insert(0,str(ROOT/'tests/gnu'))
spec=importlib.util.spec_from_file_location('reviewed',ROOT/'tests/gnu/reviewed-original.py')
runner=importlib.util.module_from_spec(spec);spec.loader.exec_module(runner)
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
target=ROOT/'evidence/gnu-cut-selection-validation.json';assert not target.exists()
coverage_target=ROOT/'evidence/gnu-cut-selection-coverage.json';assert not coverage_target.exists()
source=Path('/opt/src/coreutils-9.11');script='tests/cut/cut.pl'
snapshot=ROOT/'evidence/raw/gnu-cut-multicall-reviewed-inventory.json'
definition,=[r for r in json.loads(snapshot.read_text()) if r['script']==script]
review_path=ROOT/'evidence/gnu-cut-case-review.json';review=json.loads(review_path.read_text())
assert sha(source/script)==definition['sha256']==review['script_sha256']
assert sha(source/'tests/Coreutils.pm')==review['generator_sha256']
assert len(review['names'])==len(set(review['names']))==909
assert review['held']==[n for n in review['names'] if n.startswith(('dbl-free','big-unbounded-'))]
assert definition['cases']==review['selected']==[n for n in review['names'] if n not in review['held']]
assert definition['expected_case_count']==877 and definition['original_case_count']==909 and not definition['full_suite']
assert set(definition['excluded_cases'])==set(review['held']) and len(review['held'])==32
raw={};reports={};clean_counts={}
def pin(path,expected=None):
    p=Path(path)
    if not p.is_absolute():p=ROOT/p
    key=str(p.relative_to(ROOT)) if p.is_relative_to(ROOT) else str(p)
    h=raw.get(key) or sha(p)
    if expected is not None:assert h==expected,str(p)
    raw[key]=h
    return h
pin(snapshot);pin(review_path);pin(review['collector'],review['collector_sha256'])
assert (ROOT/'build/cut-case-inventory.stdout').read_text().splitlines()==review['names']
pin('build/cut-case-inventory.stdout');pin('build/cut-case-inventory.stderr')
locale_report=ROOT/'evidence/test-locales.json';locales=json.loads(locale_report.read_text());pin(locale_report)
for row in locales['locales']:
    for suffix,h in row['files'].items():
        pin(ROOT/locales['path']/row['name']/suffix,h)
        pin(Path(locales['runtime_path'])/row['name']/suffix,h)
for instrument in (False,True):
    name='gnu-cut-multicall-'+('valgrind' if instrument else 'original')
    p=ROOT/f'evidence/raw/{name}.json';d=json.loads(p.read_text());reports[str(p.relative_to(ROOT))]=pin(p)
    checkpoint=ROOT/f'evidence/raw/{name}.progress.json';pin(checkpoint)
    saved=json.loads(checkpoint.read_text())['runs'][script];context=saved['context']
    assert context['definition']==definition and context['valgrind']==instrument
    assert d['total']==d['passed']==1
    r,=d['results'];assert all(r[k]==v for k,v in definition.items())
    pin('/bin/sh',context['shell']);pin(ROOT/'build/gnu-coreutils/lib/config.h',context['config_header'])
    pin(ROOT/'build/gnu-coreutils/src/getlimits',context['getlimits']);pin(locale_report,context['locales'])
    for path,h in context['drivers'].items():pin(path,h)
    for path,h in context['gnu_harness'].items():pin(source/path,h)
    for path,h in (context.get('valgrind_runtime') or {}).items():pin(path,h)
    for key in ('gnu','rboxc'):
        o=r[key];assert saved['outcomes'][key]['result']==o
        assert o['status']==0 and o['case_count']==877 and o['case_count_pass']
        assert o['binary_sha256']==context['binaries'][key]
        binary=ROOT/('build/gnu-coreutils/src/coreutils' if key=='gnu' else 'target/bash-sigchld-cleanup-candidate/release/rboxc')
        pin(binary,o['binary_sha256']);pin(o['watchdog']['path'],o['watchdog']['sha256'])
        assert o['locale_evidence_sha256']==sha(locale_report)
        for path,h in saved['outcomes'][key]['logs'].items():pin(path,h)
        log=(ROOT/o['log']).read_text()
        assert log.count('RBOXC_SELECTION 877 of 909\n')==1
        assert not re.search(r'^-e: test .* mismatch',log,re.M)
        assert not any(re.search(r'^'+re.escape(n)+r'\.\.\.$',log,re.M) for n in review['held'])
        if not instrument:continue
        assert len(o['memory'])==878
        versions=0
        for m in o['memory']:
            path=ROOT/m['log'];text=path.read_text();pid=path.stem
            parsed=runner.parse_memory_log(text,pid)
            assert all(m[k]==v for k,v in parsed.items())
            assert set(re.findall(r'^==([0-9]+)==',text,re.M))=={pid}
            image,=re.findall(r'^==[0-9]+== Command: (.*)$',text,re.M)
            assert re.match(r'coreutils --coreutils-prog=cut(?: |$)',image)
            versions+=image=='coreutils --coreutils-prog=cut --version'
            errors=list(re.finditer('ERROR SUMMARY:',text));fds=list(re.finditer('FILE DESCRIPTORS:',text))
            assert errors and len(errors)==len(fds)
            assert errors[-1].start()>text.index('Command:') and fds[-1].start()>text.index('Command:')
            assert m['errors']==m['non_inherited_descriptors']==0
            assert not any(m['heap_bytes'].get(k,0) for k in ('definitely lost','indirectly lost','possibly lost'))
        assert versions==1
        clean_counts[key]=len(o['memory'])
# Preserve the initial native-launcher timeout and the deliberately stopped
# candidate profile. The announced selection count is not a completion count.
initial_snapshot=ROOT/'evidence/raw/gnu-cut-reviewed-inventory.json';pin(initial_snapshot)
initial_definition,=[r for r in json.loads(initial_snapshot.read_text()) if r['script']==script]
assert initial_definition['cases']==definition['cases']
for suffix in ('original','valgrind'):
    path=ROOT/f'evidence/raw/gnu-cut-selected-{suffix}.json';d=json.loads(path.read_text());reports[str(path.relative_to(ROOT))]=pin(path)
    progress=path.with_suffix('.progress.json');pin(progress)
    saved=json.loads(progress.read_text())['runs'][script]
    assert saved['context']['definition']==initial_definition
    for p,h in saved['context']['drivers'].items():
        if p=='tests/gnu/reviewed-original.py':p='evidence/raw/gnu-cut-initial-original-driver.py'
        pin(p,h)
    r,=d['results'];assert d['total']==1 and d['passed']==(1 if suffix=='original' else 0)
    for key in ('gnu','rboxc'):
        o=r[key];assert saved['outcomes'][key]['result']==o
        assert o['status']==(0 if suffix=='original' else 124)
        for p,h in saved['outcomes'][key]['logs'].items():pin(p,h)
        if suffix=='valgrind':
            assert len(o['memory'])==({'gnu':844,'rboxc':198}[key])
            assert o['elapsed_seconds']>=900 if key=='gnu' else o['elapsed_seconds']<900
            for m in o['memory']:
                log=ROOT/m['log'];parsed=runner.parse_memory_log(log.read_text(),log.stem)
                assert all(m[k]==v for k,v in parsed.items())
pin('evidence/gnu-cut-initial-watchdog-stop.json')
base_path=ROOT/'evidence/gnu-ptx-selection-coverage.json';base=json.loads(base_path.read_text());pin(base_path)
coverage=copy.deepcopy(base);row,=[r for r in coverage['results'] if r['script']==script]
assert row['state']=='pending' and row['valgrind_state']=='pending'
row.update(state='partial',execution_coverage='selected-cases',selected_case_count=877,
    original_case_count=909,held_cases=review['held'],valgrind_state='passed-selection',
    evidence='evidence/raw/gnu-cut-multicall-original.json',valgrind_evidence='evidence/raw/gnu-cut-multicall-valgrind.json',
    audit=str(target.relative_to(ROOT)),binary_sha256=context['binaries']['rboxc'])
assert all(a==b for a,b in zip(base['results'],coverage['results']) if a['script']!=script)
coverage['counts']=dict(sorted(Counter(r['state'] for r in coverage['results']).items()))
coverage['valgrind_counts']=dict(sorted(Counter(r['valgrind_state'] for r in coverage['results']).items()))
coverage.update(base_report=str(base_path.relative_to(ROOT)),base_report_sha256=sha(base_path),
    scope='Pinned 733-script Coreutils ledger with the previously audited env and ptx selections and this 877-case cut selection on their recorded candidates. Only the cut row changes from the preceding ledger. Thirty-two historical inputs remain unexecuted; this is selected coverage, not full-script acceptance.')
coverage_target.write_text(json.dumps(coverage,indent=2)+'\n')
target.write_text(json.dumps({'scope':'All 877 selected unchanged cut cases pass against original assertions, normally and under full-child Valgrind tracing in GNU and Rboxc. This covers file, stdin and pipe forms, including the declared multibyte locale. Every command image is checked for complete summaries, heap loss and descriptor ownership. The explicit GNU dispatcher preserves the original command name. Initial incomplete native-launcher profiles are retained separately and never counted as passes. Thirty-two historical inputs remain held; installed-release certification is unchanged.',
    'binary_sha256':context['binaries']['rboxc'],'selected_cases':877,'original_cases':909,
    'held_cases':review['held'],'clean_processes':clean_counts,'reports':reports,'raw':raw,
    'coverage':str(coverage_target.relative_to(ROOT)),'coverage_sha256':sha(coverage_target),
    'accounting_pass':True,'selected_cases_pass':True,'full_suite_pass':False},indent=2)+'\n')
print('PASS: 877 selected cut cases; 878 clean candidate images; 32 held inputs')
