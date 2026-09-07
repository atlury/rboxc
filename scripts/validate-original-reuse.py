#!/usr/bin/env python3
"""Record unchanged provider inputs while retaining original tests on their real binary hash."""
# SPDX-License-Identifier: GPL-3.0-or-later
import argparse
import hashlib
import json
from pathlib import Path
import re
import subprocess

ROOT=Path(__file__).resolve().parents[1]
parser=argparse.ArgumentParser(description=__doc__)
parser.add_argument('--provider',required=True,choices=['grep'])
parser.add_argument('--candidate',required=True,type=Path)
parser.add_argument('--focused-report',required=True,type=Path)
parser.add_argument('--baseline-revision',required=True)
parser.add_argument('--report-name',required=True)
options=parser.parse_args()
assert re.fullmatch(r'[a-z0-9][a-z0-9-]*',options.report_name)
def digest(path):return hashlib.sha256(path.read_bytes()).hexdigest()
provider=options.provider
binary=options.candidate.resolve(strict=True);binary_hash=digest(binary)
original_path=ROOT/f'evidence/{provider}-original.json'
original=json.loads(original_path.read_text())
proof_path=ROOT/f'evidence/{provider}-candidate-proof.json'
proof=json.loads(proof_path.read_text())
assert original['binary_sha256']==proof['candidate']['sha256']
assert binary_hash!=original['binary_sha256'], 'same-byte reports use ordinary activation reuse'
original_source=ROOT/proof['reports'][provider+'-original']['path']
assert digest(original_source)==proof['reports'][provider+'-original']['sha256']
assert original['activation']['source_report_sha256']==digest(original_source)
assert original['activation']['same_executable_bytes']
assert original['results']==json.loads(original_source.read_text())['results']
unchanged={}
for path,expected in proof['source_evidence'].items():
    assert digest(ROOT/path)==expected, 'provider source or original assessment changed'
    unchanged[path]=expected
entry=json.loads((ROOT/f'evidence/{provider}-translation.json').read_text())
assert digest(ROOT/entry['rust_file'])==entry['rust_sha256']
linked=json.loads((ROOT/f'evidence/{provider}-link.json').read_text())
assert linked['rust_source_sha256']==entry['rust_sha256'] and not linked['native_command_entries']
for path,expected in {**linked['original_inputs'],**linked['helper_inputs']}.items():
    assert digest(ROOT/path)==expected, 'provider native object changed'
    unchanged[path]=expected
unchanged[entry['rust_file']]=entry['rust_sha256']
revision=subprocess.check_output(['git','rev-parse',options.baseline_revision],cwd=ROOT,text=True).strip()
compiler_inputs={}
for name in ('Cargo.toml','Cargo.lock','rust-toolchain.toml','build.rs','src/main.rs'):
    data=subprocess.check_output(['git','show',revision+':'+name],cwd=ROOT)
    expected=hashlib.sha256(data).hexdigest()
    assert digest(ROOT/name)==expected, 'compiler or multicall boundary changed'
    compiler_inputs[name]=expected
absent_configs=['.cargo/config', '.cargo/config.toml']
for name in absent_configs:
    assert not subprocess.check_output(['git','ls-tree',revision,'--',name],cwd=ROOT), 'baseline has a local Cargo configuration'
    assert not (ROOT/name).exists(), 'new Cargo configuration requires fresh assessment'
focused_path=options.focused_report.resolve(strict=True)
focused=json.loads(focused_path.read_text())
assert focused['binary_sha256']==binary_hash
assert focused['passed']==focused['total']>0 and all(r['pass'] and r['memory_clean'] for r in focused['results'])
listed=subprocess.check_output([binary,'--list'],text=True).splitlines()
assert set(json.loads((ROOT/'inventory/sources.json').read_text())[provider]['commands'])<=set(listed)
report={'scope':'Source and helper identity plus fresh focused comparisons support retaining prior original-test evidence. The original assertions were executed on their recorded earlier binary, not rerun on this executable. No byte-identity claim or new original-test pass is made.',
        'provider':provider,'binary':str(binary),'binary_sha256':binary_hash,
        'passed':1,'total':1,'assessment':'unchanged-inputs-with-current-focused-validation',
        'original_observations':{'path':str(original_path.relative_to(ROOT)),'sha256':digest(original_path),
                                 'binary_sha256':original['binary_sha256'],'passed':original['passed'],'total':original['total']},
        'original_activation_proof':{'path':str(proof_path.relative_to(ROOT)),'sha256':digest(proof_path)},
        'focused_observations':{'path':str(focused_path.relative_to(ROOT)),'sha256':digest(focused_path),
                                'passed':focused['passed'],'total':focused['total']},
        'baseline_revision':revision,'unchanged_compiler_inputs':compiler_inputs,'absent_compiler_configs':absent_configs,
        'unchanged_provider_inputs':unchanged,'driver_sha256':digest(Path(__file__))}
(ROOT/'evidence'/(options.report_name+'.json')).write_text(json.dumps(report,indent=2)+'\n')
print('Verified retained',provider,'original evidence; current focused checks',focused['passed'],'/',focused['total'])
