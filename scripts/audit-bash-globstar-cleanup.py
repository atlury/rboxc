#!/usr/bin/env python3
"""Audit glob ownership, new original profiles and shared candidate checks."""
# SPDX-License-Identifier: GPL-3.0-or-later
import hashlib,importlib.util,json,re,subprocess,sys
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
sys.path.insert(0,str(ROOT/'tests/gnu'))
spec=importlib.util.spec_from_file_location('reviewed',ROOT/'tests/gnu/reviewed-original.py')
runner=importlib.util.module_from_spec(spec);spec.loader.exec_module(runner)
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
target=ROOT/'evidence/bash-globstar-cleanup-validation.json';assert not target.exists()
binary=ROOT/'target/bash-globstar-cleanup-candidate/release/rboxc';digest=sha(binary)
assert sha(ROOT/'target/bash-globstar-cleanup-repro/release/rboxc')==digest
assert len(subprocess.check_output([binary,'--list']).splitlines())==187
assert sha(ROOT/'target/release/rboxc')=='8979835878e7dbe575440b5d0c7c86a01c75782ff5ddc735707ccada50e88e86'
raw={};reports={}
def pin(path,expected=None):
 p=Path(path)
 if not p.is_absolute():p=ROOT/p
 h=sha(p)
 if expected is not None:assert h==expected,str(p)
 raw[str(p.relative_to(ROOT)) if p.is_relative_to(ROOT) else str(p)]=h
 return h
p=ROOT/'evidence/bash-globstar-cleanup.json';profile=json.loads(p.read_text());pin('scripts/bash_globstar_cleanup.py',profile['driver_sha256'])
for field in ('original','source','object','native_archive','archive','log'):pin(profile[field],profile[field+'_sha256'])
reports[str(p.relative_to(ROOT))]=pin(p)
a=json.loads((ROOT/'evidence/bash-before-globstar-cleanup-link.json').read_text());b=json.loads((ROOT/'evidence/bash-link.json').read_text())
assert a['original_inputs']==b['original_inputs']
assert a['helper_inputs'].keys()==b['helper_inputs'].keys()
changed=[p for p in a['helper_inputs'] if a['helper_inputs'][p]!=b['helper_inputs'][p]];assert changed==['build/helpers/bash-042-libglob.a']
for p,h in b['helper_inputs'].items():pin(p,h)
original_processes=[]
for report_name,audit_name,count in [('bash-globstar-cleanup-original','bash-globstar-original-validation',1),('bash-globstar-pattern-original','bash-globstar-pattern-validation',4)]:
 p=ROOT/f'evidence/{report_name}.json';r=json.loads(p.read_text());ap=ROOT/f'evidence/{audit_name}.json';audit=json.loads(ap.read_text())
 assert r['complete'] and r['passed']==r['total']==count and all(x['pass'] for x in r['results'])
 assert r['binary_sha256']==audit['binary_sha256']==digest and sha(p)==audit['original_sha256']
 assert audit['strict_original_passes']==count and not audit['open_processes'] and not audit['open_findings']
 original_processes.extend(audit['processes'])
 for path,h in audit['raw'].items():pin(path,h)
 reports[str(p.relative_to(ROOT))]=pin(p);reports[str(ap.relative_to(ROOT))]=pin(ap)
for name,total in [('behavior',44),('adapters',69),('smoke',428),('dispatch',11)]:
 p=ROOT/f'evidence/bash-globstar-cleanup-{name}.json';r=json.loads(p.read_text());assert r.get('complete',True) and r['binary_sha256']==digest
 assert r['passed']==r['total']==len(r['results'])==total and all(x['pass'] for x in r['results'])
 for result in r['results']:
  for key,outcome in result.get('outcomes',{}).items():
   for m in outcome.get('memory',[]):
    path=ROOT/m['log'];pin(path,m['sha256']);text=path.read_text();pids=set(re.findall(r'^==([0-9]+)==',text,re.M));assert len(pids)==1
    parsed=runner.parse_memory_log(text,pids.pop());assert all(m[k]==v for k,v in parsed.items())
    if key=='rboxc-valgrind':assert m['complete'] and m['clean'] and parsed['errors']==0 and parsed['non_inherited_descriptors']==0 and not any(parsed['heap_bytes'].get(k,0) for k in ('definitely lost','indirectly lost','possibly lost'))
  if result.get('log'):pin(result['log'])
 reports[str(p.relative_to(ROOT))]=pin(p)
# New recipe findings are preserved against the preceding immutable candidate.
for name,open_selection in [('structure','globstar'),('expansion','exp-tests')]:
 p=ROOT/f'evidence/bash-next-six-{name}.json';r=json.loads(p.read_text());ap=ROOT/f'evidence/bash-next-six-{name}-validation.json';audit=json.loads(ap.read_text())
 assert r['complete'] and r['passed']==2 and r['total']==3 and sha(p)==audit['original_sha256']
 assert {x['selection'] for x in r['results'] if not x['pass']}=={open_selection}
 for path,h in audit['raw'].items():pin(path,h)
 reports[str(p.relative_to(ROOT))]=pin(p);reports[str(ap.relative_to(ROOT))]=pin(ap)
for p in ['scripts/entry_provider_helpers.py','tests/bash-original.py','evidence/raw/bash-before-next-six-original-driver.py','evidence/raw/bash-next-six-original-inventory.json']:pin(p)
target.write_text(json.dumps({'scope':'Only the isolated Bash glob archive changes. Recursive names and empty placeholders are released before their vectors are discarded/compacted, and the current-directory malloc node joins the existing cleanup chain. The unchanged globstar original and four pattern originals pass strictly on the new candidate, along with shared checks. Six newly reviewed original recipes all match GNU assertions on their recorded candidates; the exp-tests invalid-array-index heap finding remains open. Earlier Bash results retain their actual tested hashes. Full Bash/GNU acceptance is incomplete.',
 'binary':str(binary),'binary_sha256':digest,'size_bytes':binary.stat().st_size,'byte_identical_rebuild':True,'changed_helpers':changed,'strict_pattern_scripts':5,'clean_pattern_processes':len(original_processes),'reports':reports,'raw':raw,'pass':True},indent=2)+'\n')
print('PASS: globstar ownership; five strict original pattern scripts;',len(original_processes),'clean processes')
