#!/usr/bin/env python3
"""Audit the unchanged ordinary ptx formatting selection."""
# SPDX-License-Identifier: GPL-3.0-or-later
from collections import Counter
import copy,hashlib,importlib.util,json,re,sys
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
sys.path.insert(0,str(ROOT/'tests/gnu'))
spec=importlib.util.spec_from_file_location('reviewed',ROOT/'tests/gnu/reviewed-original.py')
runner=importlib.util.module_from_spec(spec);spec.loader.exec_module(runner)
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
target=ROOT/'evidence/gnu-ptx-selection-validation.json';assert not target.exists()
coverage_target=ROOT/'evidence/gnu-ptx-selection-coverage.json';assert not coverage_target.exists()
source=Path('/opt/src/coreutils-9.11');script='tests/ptx/ptx.pl'
snapshot=ROOT/'evidence/raw/gnu-ptx-reviewed-inventory.json'
definition,=[r for r in json.loads(snapshot.read_text()) if r['script']==script]
review_path=ROOT/'evidence/gnu-ptx-case-review.json';review=json.loads(review_path.read_text())
assert sha(source/script)==definition['sha256']==review['script_sha256']
assert sha(source/'tests/Coreutils.pm')==review['generator_sha256']
assert len(review['names'])==len(set(review['names']))==31
bases=['1tok','2tok','width-1','width-3','width-2','format-r','format-t']
assert review['selected']==definition['cases']==[n+s for n in bases for s in ('','.r','.p')]
assert set(definition['excluded_cases'])==set(review['held'])==set(review['names'])-set(review['selected'])
assert definition['expected_case_count']==21 and definition['original_case_count']==31 and not definition['full_suite']
raw={};reports={}
def pin(path,expected=None):
    p=Path(path)
    if not p.is_absolute():p=ROOT/p
    key=str(p.relative_to(ROOT)) if p.is_relative_to(ROOT) else str(p)
    h=raw.get(key) or sha(p)
    if expected is not None:assert h==expected,str(p)
    raw[key]=h
    return h
pin(snapshot);pin(review_path);pin(review['collector'],review['collector_sha256'])
assert (ROOT/'build/ptx-case-inventory.stdout').read_text().splitlines()==review['names']
pin('build/ptx-case-inventory.stdout');pin('build/ptx-case-inventory.stderr')
for instrument in (False,True):
    name='gnu-ptx-selected-'+('valgrind' if instrument else 'original')
    p=ROOT/f'evidence/raw/{name}.json';d=json.loads(p.read_text());reports[str(p.relative_to(ROOT))]=pin(p)
    checkpoint=ROOT/f'evidence/raw/{name}.progress.json';pin(checkpoint)
    saved=json.loads(checkpoint.read_text())['runs'][script];context=saved['context']
    assert context['definition']==definition and context['valgrind']==instrument
    assert d['total']==d['passed']==1
    r,=d['results'];assert all(r[k]==v for k,v in definition.items())
    pin('/bin/sh',context['shell']);pin(ROOT/'build/gnu-coreutils/lib/config.h',context['config_header'])
    pin(ROOT/'build/gnu-coreutils/src/getlimits',context['getlimits'])
    for path,h in context['drivers'].items():pin(path,h)
    for path,h in context['gnu_harness'].items():pin(source/path,h)
    for path,h in (context.get('valgrind_runtime') or {}).items():pin(path,h)
    for key in ('gnu','rboxc'):
        o=r[key];assert saved['outcomes'][key]['result']==o
        assert o['status']==0 and o['case_count']==21 and o['case_count_pass']
        assert o['binary_sha256']==context['binaries'][key]
        binary=ROOT/('build/gnu-coreutils/src/coreutils' if key=='gnu' else 'target/bash-multibyte-cleanup-candidate/release/rboxc')
        pin(binary,o['binary_sha256']);pin(o['watchdog']['path'],o['watchdog']['sha256'])
        for path,h in saved['outcomes'][key]['logs'].items():pin(path,h)
        log=(ROOT/o['log']).read_text();assert log.count('RBOXC_SELECTION 21 of 31\n')==1
        assert not re.search(r'^ptx: test .* mismatch',log,re.M)
        if not instrument:continue
        assert len(o['memory'])==22
        versions=0
        for m in o['memory']:
            path=ROOT/m['log'];text=path.read_text();pid=path.stem
            parsed=runner.parse_memory_log(text,pid)
            assert all(m[k]==v for k,v in parsed.items())
            assert set(re.findall(r'^==([0-9]+)==',text,re.M))=={pid}
            image,=re.findall(r'^==[0-9]+== Command: (.*)$',text,re.M)
            assert image=='ptx' or image.startswith('ptx ')
            versions+=image=='ptx --version'
            errors=list(re.finditer('ERROR SUMMARY:',text));fds=list(re.finditer('FILE DESCRIPTORS:',text))
            assert errors and len(errors)==len(fds)
            assert errors[-1].start()>text.index('Command:') and fds[-1].start()>text.index('Command:')
            assert m['errors']==m['non_inherited_descriptors']==0
            assert not any(m['heap_bytes'].get(k,0) for k in ('definitely lost','indirectly lost','possibly lost'))
        assert versions==1
base_path=ROOT/'evidence/gnu-env-s-selection-coverage.json';base=json.loads(base_path.read_text());pin(base_path)
coverage=copy.deepcopy(base);row,=[r for r in coverage['results'] if r['script']==script]
assert row['state']=='pending' and row['valgrind_state']=='pending'
row.update(state='partial',execution_coverage='selected-cases',selected_case_count=21,
    original_case_count=31,held_cases=review['held'],valgrind_state='passed-selection',
    evidence='evidence/raw/gnu-ptx-selected-original.json',valgrind_evidence='evidence/raw/gnu-ptx-selected-valgrind.json',
    audit=str(target.relative_to(ROOT)),binary_sha256=context['binaries']['rboxc'])
assert all(a==b for a,b in zip(base['results'],coverage['results']) if a['script']!=script)
coverage['counts']=dict(sorted(Counter(r['state'] for r in coverage['results']).items()))
coverage['valgrind_counts']=dict(sorted(Counter(r['valgrind_state'] for r in coverage['results']).items()))
coverage.update(base_report=str(base_path.relative_to(ROOT)),base_report_sha256=sha(base_path),
    scope='Pinned 733-script Coreutils ledger with the previously audited env selection and this 21-case ptx selection on their recorded candidates. Only the ptx row changes. Ten historical inputs remain unexecuted; this is selected coverage, not full-script acceptance.')
coverage_target.write_text(json.dumps(coverage,indent=2)+'\n')
target.write_text(json.dumps({'scope':'All 21 unchanged ordinary ptx cases pass against original assertions, normally and under full-child Valgrind tracing. Token/default-width/roff/TeX cases include file, stdin and pipe forms. Each implementation has 22 clean command images, including its version check. Ten historical cases remain held; installed-release certification is unchanged.',
    'binary_sha256':context['binaries']['rboxc'],'selected_cases':21,'original_cases':31,
    'held_cases':review['held'],'clean_candidate_processes':22,'reports':reports,'raw':raw,
    'coverage':str(coverage_target.relative_to(ROOT)),'coverage_sha256':sha(coverage_target),
    'accounting_pass':True,'selected_cases_pass':True,'full_suite_pass':False},indent=2)+'\n')
print('PASS: 21 selected ptx cases; 22 clean candidate images; 10 held inputs')
