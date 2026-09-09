#!/usr/bin/env python3
"""Reconcile the iconv inventory with previously audited original recipe results."""
# SPDX-License-Identifier: GPL-3.0-or-later
import hashlib,json
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
target=ROOT/'evidence/glibc-utility-ledger-validation.json';assert not target.exists()
audit_path=ROOT/'evidence/iconv-spool-validation.json';audit=json.loads(audit_path.read_text())
old_path=ROOT/'evidence/raw/glibc-before-iconv-ledger.json';old=json.loads(old_path.read_text())
manifest=ROOT/'inventory/glibc-utility-tests.json';current=json.loads(manifest.read_text())
assert sha(old_path)==audit['source_files']['inventory/glibc-utility-tests.json']
assert sha(audit['candidate'])==audit['candidate_sha256']==sha(audit['rebuild'])
for p,h in {**audit['reports'],**audit['raw_files']}.items():assert sha(ROOT/p)==h,p
assert len(audit['raw_files'])==audit['verified_raw_files']==487
original=json.loads((ROOT/'evidence/iconv-spool-original.json').read_text())
assert original['complete'] and original['passed']==original['total']==original['planned_total']==len(original['results'])==3
assert all(r['pass'] for r in original['results'])
assert original['binary_sha256']==audit['candidate_sha256']
assert len(old['tests'])==len(current['tests'])==4
for before,after in zip(old['tests'],current['tests']):
    if before['path']=='iconv/tst-iconv_prog-buffer.sh':
        expected={**before,'state':'reviewed-passing','evidence':'evidence/iconv-spool-original.json','audit':'evidence/iconv-spool-validation.json'}
        assert after==expected
    else:assert before==after
source_status={p:sha(ROOT/p)==h for p,h in audit['source_files'].items() if p!='inventory/glibc-utility-tests.json'}
target.write_text(json.dumps({'scope':'Inventory reconciliation only. The previously completed default/tiny/large iconv buffer recipes were still marked ready. Their original report, immutable candidate/rebuild, and all 487 archived raw files are verified. The historical inventory is preserved so the old audit input hash remains resolvable. This does not execute new tests or certify other glibc coverage.',
 'inventory_sha256':sha(manifest),'previous_inventory':str(old_path.relative_to(ROOT)),'previous_inventory_sha256':sha(old_path),'original_audit_sha256':sha(audit_path),'candidate_sha256':audit['candidate_sha256'],'original_recipes':3,'verified_raw_files':487,'unchanged_historical_sources':source_status,'pass':True},indent=2)+'\n')
print('PASS: iconv three-recipe ledger reconciled; 487 existing raw files verified')
