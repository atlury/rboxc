#!/usr/bin/env python3
"""Freeze mutable assembled helpers before the next incremental Bash build."""
# SPDX-License-Identifier: GPL-3.0-or-later
import hashlib
import json
from pathlib import Path
import shutil
ROOT=Path(__file__).resolve().parents[1]
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
source=ROOT/'evidence/bash-failglob-validation.json'
target=ROOT/'evidence/bash-failglob-stable-validation.json';assert not target.exists()
checkpoint=ROOT/'build/bash-before-exec-trap-cleanup';assert not checkpoint.exists();checkpoint.mkdir()
link=json.loads((ROOT/'evidence/bash-link.json').read_text())
for path in [ROOT/'evidence/bash-link.json',ROOT/'src/generated/applet_bash.rs',*[ROOT/p for p in link['helper_inputs']]]:shutil.copy2(path,checkpoint/path.name)
report=json.loads(source.read_text());mapping={}
for path,digest in report['raw'].copy().items():
 if path=='evidence/bash-link.json' or path.startswith('build/helpers/bash-'):
  original=ROOT/path;assert sha(original)==digest
  archived=checkpoint/original.name;assert sha(archived)==digest
  replacement=str(archived.relative_to(ROOT));del report['raw'][path];report['raw'][replacement]=digest;mapping[path]=replacement
assert 'evidence/bash-link.json' in mapping and len(mapping)>1
report.update(predecessor=str(source.relative_to(ROOT)),predecessor_sha256=sha(source),archived_mutable_inputs=mapping)
report['raw'][str(source.relative_to(ROOT))]=sha(source)
target.write_text(json.dumps(report,indent=2)+'\n')
print('Archived',len(mapping),'assembled inputs without changing any evidence hashes')
