#!/usr/bin/env python3
"""Retain original observations only after verifying unchanged provider inputs."""
# SPDX-License-Identifier: GPL-3.0-or-later
import argparse
import hashlib
import json
from pathlib import Path
import re
import subprocess

ROOT=Path(__file__).resolve().parents[1]
parser=argparse.ArgumentParser(description=__doc__)
parser.add_argument('--provider',required=True,choices=['hello','time','diffutils','gzip','sed','bc','ed'])
parser.add_argument('--candidate',type=Path,required=True)
parser.add_argument('--focused-report',type=Path,required=True)
parser.add_argument('--baseline-proof',type=Path,required=True)
parser.add_argument('--baseline-revision',required=True)
parser.add_argument('--report-name',required=True)
options=parser.parse_args();provider=options.provider
assert re.fullmatch(r'[a-z0-9][a-z0-9-]*',options.report_name)
def digest(path):return hashlib.sha256(path.read_bytes()).hexdigest()
binary=options.candidate.resolve(strict=True);binary_hash=digest(binary)
proof_path=options.baseline_proof.resolve(strict=True);proof=json.loads(proof_path.read_text())
old_binary=ROOT/proof['candidate']['path']
assert digest(old_binary)==proof['candidate']['sha256']
original_path=ROOT/f'evidence/{provider}-original.json';original=json.loads(original_path.read_text())
assert original['binary_sha256']==proof['candidate']['sha256']!=binary_hash
item=proof['reports'][provider+'-original'];source=ROOT/item['path']
assert digest(source)==item['sha256']==original['activation']['source_report_sha256']
assert original['activation']['same_executable_bytes']
source_data=json.loads(source.read_text())
omit={'activation','binary','runtime_helpers'}
assert {k:v for k,v in original.items() if k not in omit}=={k:v for k,v in source_data.items() if k not in omit}
linked_path=ROOT/f'evidence/{provider}-link.json'
linked=json.loads(linked_path.read_text());assert not linked['native_command_entries']
entries=([ROOT/f'evidence/{provider}-{name}-translation.json' for name in linked['rust_source_sha256']]
         if isinstance(linked['rust_source_sha256'],dict) else [ROOT/f'evidence/{provider}-translation.json'])
metadata=[linked_path,*entries,ROOT/f'inventory/{provider}-tests.json']
cleanup=ROOT/f'evidence/{provider}-native-cleanup.json'
if cleanup.exists():metadata.append(cleanup)
unchanged={}
for path in metadata:
    relative=str(path.relative_to(ROOT));expected=proof['source_evidence'][relative]
    assert digest(path)==expected,'provider build metadata or reviewed inventory changed'
    unchanged[relative]=expected
for path in entries:
    entry=json.loads(path.read_text());expected=linked['rust_source_sha256']
    if isinstance(expected,dict):expected=expected[entry['command']]
    assert entry['translated'] and expected==entry['rust_sha256']==digest(ROOT/entry['rust_file'])
    unchanged[entry['rust_file']]=expected
for path,expected in {**linked.get('original_inputs',{}),**linked.get('helper_inputs',{})}.items():
    assert digest(ROOT/path)==expected,'native provider object changed'
    unchanged[path]=expected
if 'helper_archive' in linked:
    assert digest(ROOT/linked['helper_archive'])==linked['helper_archive_sha256']
    unchanged[linked['helper_archive']]=linked['helper_archive_sha256']
assert linked.get('helper_inputs') or linked.get('helper_archive'), 'no native helper evidence'
revision=subprocess.check_output(['git','rev-parse',options.baseline_revision],cwd=ROOT,text=True).strip()
compiler={}
for name in ('Cargo.toml','Cargo.lock','rust-toolchain.toml','build.rs','src/main.rs'):
    expected=hashlib.sha256(subprocess.check_output(['git','show',revision+':'+name],cwd=ROOT)).hexdigest()
    assert digest(ROOT/name)==expected,'compiler or multicall boundary changed'
    compiler[name]=expected
absent=['.cargo/config','.cargo/config.toml']
for name in absent:
    assert not subprocess.check_output(['git','ls-tree',revision,'--',name],cwd=ROOT)
    assert not (ROOT/name).exists()
focused_path=options.focused_report.resolve(strict=True);focused=json.loads(focused_path.read_text())
assert focused['binary_sha256']==binary_hash and focused['passed']==focused['total']>0
assert all(r['pass'] and r['memory_clean'] for r in focused['results'])
baseline_focused=proof['reports'][provider+'-behavior']
baseline_focused_path=ROOT/baseline_focused['path']
assert digest(baseline_focused_path)==baseline_focused['sha256']
prior_focused=json.loads(baseline_focused_path.read_text())
assert focused['driver_sha256']==prior_focused['driver_sha256'], 'focused driver changed and needs separate review'
assert focused['total']==prior_focused['total']==len(focused['results']), 'focused batch is incomplete'
assert [(r.get('command'),r['name']) for r in focused['results']]==[(r.get('command'),r['name']) for r in prior_focused['results']], 'focused selection changed'
for path,expected in focused.get('runtime_helpers',{}).items():assert digest(Path(path))==expected
listed=subprocess.check_output([binary,'--list'],text=True).splitlines()
assert set(json.loads((ROOT/'inventory/sources.json').read_text())[provider]['commands'])<=set(listed)
report={'scope':'Prior original-suite observations retain their real executable hash and every open finding. Verified unchanged provider sources, native objects, compiler boundary, and fresh focused comparisons support retaining those observations. This is an input-identity assessment, not a rerun or executable-byte identity claim.',
    'provider':provider,'binary':str(binary),'binary_sha256':binary_hash,'passed':1,'total':1,
    'assessment':'unchanged-inputs-with-current-focused-validation',
    'original_observations':{'path':str(original_path.relative_to(ROOT)),'sha256':digest(original_path),'binary_sha256':original['binary_sha256'],'passed':original['passed'],'total':original['total']},
    'original_activation_proof':{'path':str(proof_path.relative_to(ROOT)),'sha256':digest(proof_path)},
    'focused_observations':{'path':str(focused_path.relative_to(ROOT)),'sha256':digest(focused_path),'passed':focused['passed'],'total':focused['total']},
    'baseline_focused_observations':baseline_focused,
    'baseline_revision':revision,'unchanged_compiler_inputs':compiler,'absent_compiler_configs':absent,
    'unchanged_provider_inputs':unchanged,'driver_sha256':digest(Path(__file__))}
(ROOT/'evidence'/(options.report_name+'.json')).write_text(json.dumps(report,indent=2)+'\n')
print('Verified retained',provider,'observations with',focused['passed'],'current focused passes')
