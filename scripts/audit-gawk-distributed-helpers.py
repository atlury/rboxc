#!/usr/bin/env python3
"""Audit distributed utility comparisons without inflating original test counts."""
# SPDX-License-Identifier: GPL-3.0-or-later
import importlib.util
import json
from pathlib import Path
import re
import sys

ROOT = Path(__file__).resolve().parents[1]
sys.path[:0] = [str(ROOT/'tests'),str(ROOT/'tests/gnu')]
from comparison_profile import fingerprint
spec = importlib.util.spec_from_file_location('reviewed',ROOT/'tests/gnu/reviewed-original.py')
runner = importlib.util.module_from_spec(spec)
spec.loader.exec_module(runner)
path = ROOT/'evidence/gawk-distributed-helper-comparison.json'
report = json.loads(path.read_text())
target = ROOT/'evidence/gawk-distributed-helper-validation.json'
assert not target.exists()
assert report['complete'] and report['passed']==report['total']==3
assert fingerprint(Path(report['binary']))==report['binary_sha256']
assert fingerprint(ROOT/'build/gnu-gawk/gawk')==report['gnu_binary_sha256']
for p,h in {**report['inputs'],**report['runtime_helpers']}.items():
    assert fingerprint(Path(p))==h
programs = {}
processes = []
for row in report['results']:
    assert row['pass'] and row['equivalent']
    name = row['name']
    assert name in ('printfloat','printlang','valgrind')
    program = Path('/opt/src/gawk-5.4.1/test')/(name+'.awk')
    assert fingerprint(program)==row['program_sha256']
    reference = row['outcomes']['gnu']
    assert reference['status']==0
    for key,outcome in row['outcomes'].items():
        assert all(outcome[k]==reference[k] for k in ('status','stdout_sha256','stderr_sha256','stdout_lines'))
        for p,h in outcome['raw'].items():
            f = ROOT/p
            assert fingerprint(f)==h
            if f.name=='stdout':
                assert h==outcome['stdout_sha256'] and len(f.read_bytes())==outcome['stdout_bytes']
                assert f.read_bytes().count(b'\n')==outcome['stdout_lines']
                if name=='printlang':
                    assert f.read_bytes()==b'\nLocale environment:\n\tLC_ALL="C" LANG="C"\n\n'
                if name=='valgrind':
                    assert b'definitely lost:' in f.read_bytes() and b'log.1:' in f.read_bytes()
        if 'memory' not in outcome:
            continue
        log = ROOT/outcome['memory_log']
        text = log.read_text()
        commands = re.findall(r'^==[0-9]+== Command: (.*)$',text,re.M)
        assert len(commands)==1 and commands[0].startswith('gawk -f '+str(program))
        pids = set(re.findall(r'^==([0-9]+)==',text,re.M))
        assert len(pids)==1
        parsed = runner.parse_memory_log(text,pids.pop(),exec_only=True)
        assert parsed==outcome['memory'] and parsed['complete_exec_log']
        if key=='rboxc-valgrind':
            assert parsed['errors']==parsed['non_inherited_descriptors']==0
            assert not any(parsed['heap_bytes'].get(k,0) for k in ('definitely lost','indirectly lost','possibly lost'))
            processes.append({'log':outcome['memory_log'],'sha256':fingerprint(log),**parsed})
    if name=='printfloat':
        assert reference['stdout_lines']==24000 and row['original_make_target'] is None
    programs[name]={'source_sha256':row['program_sha256'],'make_target':row['original_make_target']}
assert len(programs)==len(processes)==3
target.write_text(json.dumps({'scope':'Three utility comparisons and three clean candidate processes, separate from original assertion and focused-case totals. The floating-format example is bounded at 24000 lines; unchanged GNU targets display the controlled locale and scan a preserved local Valgrind log.',
    'binary':report['binary'],'binary_sha256':report['binary_sha256'],'comparison_sha256':fingerprint(path),
    'programs':programs,'candidate_processes':processes,'original_assertion_passes_added':0,
    'driver_sha256':fingerprint(Path(__file__))},indent=2)+'\n')
print('Audited three distributed utilities and three clean processes; no original assertion count added')
