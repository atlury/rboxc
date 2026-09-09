#!/usr/bin/env python3
"""Verify the original loopback daemon test and preserve signal-exit findings."""
# SPDX-License-Identifier: GPL-3.0-or-later
import hashlib
import importlib.util
import json
from pathlib import Path
import sys
ROOT=Path(__file__).resolve().parents[1]
sys.path[:0]=[str(ROOT/'tests/gnu')]
spec=importlib.util.spec_from_file_location('reviewed',ROOT/'tests/gnu/reviewed-original.py')
runner=importlib.util.module_from_spec(spec);spec.loader.exec_module(runner)
target=ROOT/'evidence/inetd-private-original-validation.json';assert not target.exists()
raw={}
def pin(path,expected=None):
 p=Path(path);p=p if p.is_absolute() else ROOT/p
 value=hashlib.sha256(p.read_bytes()).hexdigest()
 assert expected is None or value==expected,str(p)
 raw[str(p.relative_to(ROOT)) if p.is_relative_to(ROOT) else str(p)]=value
 return value
summary={}
for stem in ['inetd-private-original','inetd-private-ready-original']:
 path=ROOT/'evidence'/(stem+'.json');pin(path);report=json.loads(path.read_text());assert report['complete']
 pin(report['binary'],report['binary_sha256'])
 pin('build/gnu-inetutils/src/inetd',report['gnu_binary_sha256'])
 initial=stem=='inetd-private-original'
 for path,value in report['inputs'].items():
  actual=path
  if path.endswith('tests/inetutils-inetd-original.py'):
   actual='evidence/raw/inetd-private-'+('initial' if initial else 'ready')+'-driver.py'
  pin(actual,value)
 for row in report['results']:
  for impl,outcome in row['outcomes'].items():
   pin(outcome['log'],outcome['log_sha256'])
   if initial:
    assert outcome['status']==77 and not outcome['behavior_pass'] and outcome['service_connections']==0
   else:
    assert outcome['status']==0 and outcome['behavior_pass'] and outcome['service_connections']==10 and outcome['reload_signals']==5
    ns=outcome['private_profile'];assert ns['uid']==ns['gid']==65534 and ns['groups']==[]
    assert all(ns['namespaces'][k]!=ns['parent_namespaces'][k] for k in ['net','mnt','pid'])
    assert [i['ifname'] for i in ns['interfaces']]==['lo']
    assert len(outcome['children_reaped'])==2 and sorted(x['status'] for x in outcome['children_reaped'])==[-15,0]
   bad=[]
   for memory in outcome['memory']:
    pin(memory['log'],memory['sha256']);path=ROOT/memory['log']
    parsed=runner.parse_memory_log(path.read_text(),path.stem.removeprefix(impl+'-'),True)
    assert all(memory[k]==v for k,v in parsed.items()),(path,parsed,memory)
    assert parsed['complete_exec_log'] and not any(parsed['heap_bytes'].get(k,0) for k in ['definitely lost','indirectly lost','possibly lost'])
    if memory['errors'] or memory['non_inherited_descriptors']:bad.append(memory)
   if not initial and row['instrumented']:
    assert len(outcome['memory'])==14 and len(bad)==1 and bad[0]['errors']==6 and bad[0]['non_inherited_descriptors']==5
    assert not outcome['strict_memory_pass']
    summary[impl]={'process_images':14,'strict_clean_images':13,'daemon_findings':bad[0],'connections':10,'reloads':5}
target.write_text(json.dumps({'scope':'Unchanged GNU Inetutils 2.8 inetd.sh passes natively and under instrumentation in private loopback-only network and PID namespaces as nobody. Daemon startup redundant close and default SIGTERM descriptor findings remain open.',
 'results':summary,'raw':raw,'behavior_pass':True,'strict_memory_pass':False,'full_provider_pass':False},indent=2)+'\n')
print('PASS: original inetd assertions in four modes; daemon memory profile explicitly remains open')
