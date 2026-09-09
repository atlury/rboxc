#!/usr/bin/env python3
"""Verify Screen exit ownership, terminal lifecycle and recorded ncurses ABI."""
# SPDX-License-Identifier: GPL-3.0-or-later
import importlib.util,json,re,sys
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
sys.path[:0]=[str(ROOT/'tests'),str(ROOT/'tests/gnu'),str(ROOT/'scripts')]
from comparison_profile import fingerprint
from screen_daemon_cleanup import rust_adapter,socket_adapter
spec=importlib.util.spec_from_file_location('reviewed',ROOT/'tests/gnu/reviewed-original.py')
runner=importlib.util.module_from_spec(spec);spec.loader.exec_module(runner)
target=ROOT/'evidence/screen-daemon-validation.json';assert not target.exists()
binary=ROOT/'target/screen-tparm-candidate/release/rboxc';repro=ROOT/'target/screen-tparm-repro/release/rboxc'
assert binary.read_bytes()==repro.read_bytes()
prior=ROOT/'build/screen-before-daemon-cleanup'
old=json.loads((prior/'screen-link.json').read_text());new=json.loads((ROOT/'evidence/screen-link.json').read_text())
assert (ROOT/'src/generated/applet_screen.rs').read_text()==rust_adapter((prior/'applet_screen.rs').read_text())
translation=json.loads((ROOT/'evidence/screen-translation.json').read_text())
assert translation['raw_translation_sha256']==json.loads((prior/'screen-translation.json').read_text())['raw_translation_sha256']
assert translation['rust_sha256']==fingerprint(ROOT/'src/generated/applet_screen.rs')
source=Path('/opt/src/screen-5.0.2/socket.c')
assert (ROOT/'build/screen-daemon-cleanup/socket.c').read_text()==socket_adapter(source.read_text())
changed=[]
for path,h in old['helper_inputs'].items():
 assert fingerprint(prior/Path(path).name)==h
 assert fingerprint(ROOT/path)==new['helper_inputs'][path]
 if h!=new['helper_inputs'][path]:changed.append(path)
assert len(changed)==1 and changed[0].endswith('-socket.o')
abi=json.loads((ROOT/'evidence/screen-ncurses-cleanup-abi.json').read_text())
assert fingerprint(Path(abi['runtime_library']))==abi['runtime_sha256']
assert fingerprint(Path(abi['term_header']))==abi['term_header_sha256']
assert fingerprint(ROOT/abi['source_reference']['path'])==abi['source_reference']['sha256']
reports={n:json.loads((ROOT/'evidence'/(n+'.json')).read_text()) for n in [
 'screen-tparm-original-detach','screen-tparm-recovery-ready','screen-tparm-contract',
 'screen-tparm-focused','screen-tparm-smoke','screen-tparm-full-dispatch',
 'screen-daemon-original-detach','screen-daemon-contract']}
processes=[]
def memory(entry,path_key,hash_key):
 p=ROOT/entry[path_key];assert fingerprint(p)==entry[hash_key]
 parsed=runner.parse_memory_log(p.read_text(),p.stem,exec_only=True)
 assert all(entry[k]==v for k,v in parsed.items()) and parsed['complete_exec_log']
 return parsed

def clean(m):
 return m['errors']==m['non_inherited_descriptors']==0 and not any(m['heap_bytes'].get(k,0) for k in ['definitely lost','indirectly lost','possibly lost'])

for name,expected in [('screen-tparm-focused',5),('screen-tparm-smoke',428),('screen-tparm-full-dispatch',11)]:
 d=reports[name];assert d['passed']==d['total']==expected and d['binary_sha256']==fingerprint(binary)
for name in ['screen-tparm-original-detach','screen-tparm-recovery-ready']:
 d=reports[name];assert d['passed']==d['total']==1 and d['binary_sha256']==fingerprint(binary)
 driver=ROOT/'tests'/('screen-original-detach.py' if name.endswith('detach') else 'screen-socket-recovery.py')
 assert d['driver_sha256']==fingerprint(driver)
 for p,h in d.get('inputs',{}).items():assert fingerprint(Path(p))==h
 outcomes=d['outcomes'] if 'outcomes' in d else d['results']
 for label,o in outcomes.items():
  assert o['status']==0
  if 'sockets_remaining' in o:assert not o['sockets_remaining'] and not o['timed_out']
  if 'recovered' in o:assert o['recovered'] and o['control_status']==0
  for stream in o['streams'].values():assert fingerprint(ROOT/stream['path'])==stream['sha256']
  for m in o['memory']:
   parsed=memory(m,'log' if 'log' in m else 'path','sha256')
   if label=='rboxc-valgrind':assert clean(parsed)
   processes.append({'profile':name,'implementation':label,**m})
contract=reports['screen-tparm-contract'];assert contract['passed']==contract['total']==13 and contract['processes']==17
for p,h in contract['inputs'].items():assert fingerprint(Path(p))==h
for p,h in contract['artifacts'].items():assert fingerprint(ROOT/p)==h
for r in contract['results']:
 assert r['pass'] and r['status']==0
 for m in r['memory']:assert clean(memory(m,'path','sha256'))
# Verify retained intermediate evidence without applying new helper hashes to it.
baseline=reports['screen-daemon-original-detach']
assert baseline['complete'] and all(o['status']==0 for o in baseline['outcomes'].values())
for p,h in baseline['inputs'].items():assert fingerprint(Path(p))==h
for o in baseline['outcomes'].values():
 for m in o['memory']:memory(m,'log','sha256')
old_contract=reports['screen-daemon-contract']
old_dir=ROOT/'build/screen-before-tparm-cleanup'
for p,h in old_contract['inputs'].items():
 actual=old_dir/Path(p).name if p in [str(ROOT/'scripts/screen_daemon_cleanup.py'),str(ROOT/'tests/screen-daemon-contract.py')] else Path(p)
 assert fingerprint(actual)==h,p
for r in old_contract['results']:
 for m in r['memory']:assert clean(memory(m,'path','sha256'))
prior_finding=[m for m in baseline['outcomes']['rboxc-valgrind']['memory'] if not clean(m)]
assert len(prior_finding)==1 and prior_finding[0]['heap_bytes']['possibly lost']==418 and prior_finding[0]['non_inherited_descriptors']==0
candidate=[p for p in processes if p['implementation']=='rboxc-valgrind']
assert len(candidate)==12
partial_dirs=['build/screen-daemon-contract','build/screen-daemon-contract-abort','evidence/raw/screen-tparm-recovery']
preserved={str(p.relative_to(ROOT)):fingerprint(p) for directory in partial_dirs for p in (ROOT/directory).rglob('*') if p.is_file() and not p.is_symlink()}
report={'scope':'Screen daemon exit closes only its successfully reopened standard streams and owned server '
 'socket after GNU terminal restoration. Forked children preserve inherited handles; socket recovery '
 'invalidates the old ownership before replacement. The recorded ncurses internal ABI releases its shared '
 'parameter cache before public terminal deletion. All three registered original targets are covered across '
 'the existing declared helper and terminal profiles; this is not complete GNU-wide acceptance or a claim '
 'about every interactive Screen feature or other termcap implementations.',
 'binary':str(binary),'binary_sha256':fingerprint(binary),'bytes':binary.stat().st_size,
 'rebuild':str(repro),'rebuild_sha256':fingerprint(repro),'byte_identical':True,
 'changed_helpers':changed,'unchanged_raw_translation':True,'registered_original_targets':3,
 'original_terminal_passed':1,'socket_recovery_passed':1,'clean_terminal_processes':12,
 'ownership_contracts':13,'clean_contract_processes':17,'focused_passed':5,
 'coreutils_smoke':428,'dispatcher_passed':11,'prior_findings_resolved':{'owned_descriptors':4,'possibly_lost_bytes':418},
 'retained_helper_validation':{'path':'evidence/screen-original-helpers-validation.json','sha256':fingerprint(ROOT/'evidence/screen-original-helpers-validation.json')},
 'ncurses_abi':{'path':'evidence/screen-ncurses-cleanup-abi.json','sha256':fingerprint(ROOT/'evidence/screen-ncurses-cleanup-abi.json')},
 'reports':{n:fingerprint(ROOT/'evidence'/(n+'.json')) for n in reports},
 'driver_sha256':fingerprint(Path(__file__)),'adapter_sha256':fingerprint(ROOT/'scripts/screen_daemon_cleanup.py'),
 'translation_report_sha256':fingerprint(ROOT/'evidence/screen-translation.json'),
 'link_report_sha256':fingerprint(ROOT/'evidence/screen-link.json'),'preserved_initial_attempts':preserved,'processes':processes}
target.write_text(json.dumps(report,indent=2)+'\n')
print('Screen daemon audit passed: 12 terminal process logs and 17 contract process logs clean')
