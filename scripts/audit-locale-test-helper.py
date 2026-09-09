#!/usr/bin/env python3
"""Verify source ownership cleanup and native-helper enumeration comparisons."""
# SPDX-License-Identifier: GPL-3.0-or-later
import hashlib
import importlib.util
import json
from pathlib import Path
import sys
ROOT=Path(__file__).resolve().parents[1]
sys.path[:0]=[str(ROOT/'tests/gnu')]
spec=importlib.util.spec_from_file_location('reviewed',ROOT/'tests/gnu/reviewed-original.py');runner=importlib.util.module_from_spec(spec);spec.loader.exec_module(runner)
spec=importlib.util.spec_from_file_location('prepare',ROOT/'scripts/prepare-locale-test-helper.py');prepare=importlib.util.module_from_spec(spec);spec.loader.exec_module(prepare)
target=ROOT/'evidence/locale-test-helper-validation.json';assert not target.exists()
raw={}
def pin(path,expected=None):
 p=Path(path);p=p if p.is_absolute() else ROOT/p
 value=hashlib.sha256(p.read_bytes()).hexdigest();assert expected is None or value==expected,str(p)
 raw[str(p.relative_to(ROOT)) if p.is_relative_to(ROOT) else str(p)]=value
 return value
build_path='evidence/locale-test-helper-build.json';pin(build_path);build=json.loads((ROOT/build_path).read_text())
pin('scripts/prepare-locale-test-helper.py',build['driver_sha256']);pin(build['original_source'],build['original_sha256']);pin(build['record'],build['record_sha256'])
for p,v in build['helper_objects'].items():pin(p,v)
for name,row in build['builds'].items():
 for k in ['source','object','binary','log']:pin(row[k],row[k+'_sha256'])
 text=Path(row['source']).read_text();source=Path(build['original_source']).read_text()
 assert text==(source if name=='original' else prepare.adapt(source))
 assert all('-D'+k+'="'+v+'"' in row['compile'] for k,v in build['configuration'].items())
assert build['builds']['original']['link'][4:]==build['builds']['cleanup']['link'][4:]
counts={}
for stem in ['locale-test-helper-contract','locale-test-helper-archive-contract']:
 path='evidence/'+stem+'.json';pin(path);report=json.loads((ROOT/path).read_text())
 assert report['complete'] and report['passed']==report['total']==report['planned_total']==8 and report['build_sha256']==pin(build_path)
 for p,v in report['inputs'].items():
  if stem=='locale-test-helper-contract' and p.endswith('/tests/locale-test-helper.py'):p='evidence/raw/locale-test-helper-initial-driver.py'
  pin(p,v)
 clean=0
 for row in report['results']:
  assert row['pass'] and row['behavior_pass'];reference=row['outcomes']['host']
  if stem.endswith('archive-contract') and row['name']=='archive-directory-aliases-names':assert b'en_US.utf8\n' in bytes.fromhex(reference['stdout'])
  for impl,outcome in row['outcomes'].items():
   assert all(outcome[k]==reference[k] for k in ['status','stdout','stderr'])
   for p,v in outcome['raw'].items():pin(p,v)
   for m in outcome['memory']:
    pin(m['log'],m['sha256']);pid=Path(m['log']).stem.rsplit('-',1)[-1]
    parsed=runner.parse_memory_log((ROOT/m['log']).read_text(),pid,True);assert all(m[k]==v for k,v in parsed.items())
    assert m['complete_exec_log']
    if impl=='cleanup-valgrind':
     assert m['errors']==m['non_inherited_descriptors']==0
     assert not any(m['heap_bytes'].get(k,0) for k in ['definitely lost','indirectly lost','possibly lost']);clean+=1
 counts[stem]=clean;assert clean==8
 # The unmodified sources and system helper retain their enumeration leaks.
 names,=[r for r in report['results'] if r['name']=='system-names']
 assert not names['outcomes']['original-valgrind']['clean'] and not names['outcomes']['host-valgrind']['clean']
target.write_text(json.dumps({'scope':'Adapted native test helper only. Eight verified comparisons cover real archive entries, locale directories, duplicate aliases, verbose output and empty locale storage. All cleanup images are clean and outputs match both unmodified GNU sources and the host helper. Initial archive fixture mounted on the wrong alias path; retained but not used as archive coverage.',
 'raw':raw,'build_sha256':pin(build_path),'comparisons':8,'clean_images':8,'initial_clean_images':counts['locale-test-helper-contract'],'applet_added':False,'validation_pass':True},indent=2)+'\n')
print('PASS: 8 native-helper comparisons, unchanged output, clean enumeration cleanup')
