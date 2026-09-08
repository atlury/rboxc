#!/usr/bin/env python3
"""Verify the unchanged intrinsic-Valgrind original and its complete XML logs."""
# SPDX-License-Identifier: GPL-3.0-or-later
import json
from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0,str(ROOT/'tests'))
from comparison_profile import fingerprint
from sed_dependencies import prepare
from valgrind_xml import parse_memory_xml

path = ROOT/'evidence/sed-intrinsic-quiet-original.json'
target = ROOT/'evidence/sed-intrinsic-memory-audit.json'
assert not target.exists(), 'preserve prior audits'
report = json.loads(path.read_text())
assert report['pass'] and report['equivalent'] and report['passed'] == report['total'] == 1
assert fingerprint(Path(report['binary'])) == report['binary_sha256']
assert fingerprint(ROOT/'build/gnu-sed/sed/sed') == report['gnu_binary_sha256']
assert fingerprint(ROOT/'tests/sed-intrinsic-original.py') == report['driver_sha256']
assert prepare()[2] == report['prerequisites']
for filename,h in {**report['inputs'],**report['runtime_helpers']}.items():
    assert fingerprint(Path(filename)) == h
processes = []
for name,outcome in report['outcomes'].items():
    assert outcome['status'] == 0 and bytes.fromhex(outcome['stdout']) == b'hi\n'
    assert outcome['stderr'] == ''
    for stream,entry in outcome['streams'].items():
        p = ROOT/entry['path']
        assert fingerprint(p) == entry['sha256'] and p.read_bytes() == bytes.fromhex(outcome[stream])
    for filename,h in outcome['text_logs'].items():
        assert fingerprint(ROOT/filename) == h
    assert sorted(p['role'] for p in outcome['memory']) == ['sed-original','shell-child','valgrind-prerequisite']
    for recorded in outcome['memory']:
        p = ROOT/recorded['xml']
        assert fingerprint(p) == recorded['xml_sha256']
        parsed = parse_memory_xml(p.read_bytes())
        assert parsed['pid'] == p.stem and all(recorded[k] == v for k,v in parsed.items())
        assert set(outcome['valgrind_options']) <= set(parsed['valgrind_arguments'])
        assert '--trace-children=yes' in parsed['valgrind_arguments']
        assert '--child-silent-after-fork=yes' in parsed['valgrind_arguments']
        assert '--track-fds=yes' in parsed['valgrind_arguments']
        assert '--default-suppressions=no' in parsed['valgrind_arguments']
        if recorded['role'] == 'sed-original':
            assert parsed['argv'] == ['sed','-e','x','-e','s/.*/echo hi/e','in']
            assert {'--quiet','--error-exitcode=1','--errors-for-leak-kinds=definite'} <= set(parsed['valgrind_arguments'])
        elif recorded['role'] == 'shell-child':
            assert parsed['argv'] == ['/bin/sh','-c','--','echo hi']
        else:
            assert parsed['argv'] == ['true']
        if name == 'rboxc':
            assert parsed['memory_clean'] and parsed['finding_records'] == 0
        processes.append({'implementation':name,**recorded})
    assert outcome['memory_clean'] == all(p['memory_clean'] for p in outcome['memory'])
initial = json.loads((ROOT/'evidence/sed-intrinsic-xml-initial.json').read_text())
for filename,h in initial['raw_files'].items():assert fingerprint(ROOT/filename) == h
tests = ROOT/'evidence/raw/valgrind-xml-parser-tests.log'
assert 'Ran 6 tests' in tests.read_text() and tests.read_text().rstrip().endswith('OK')
result = {'scope':'One unchanged original assertion passes in both implementations. Three complete candidate XML logs are clean, including the prerequisite and executed shell child. Quiet output and GNU error-exit options remain unchanged. Native GNU Sed possible-loss findings and the initial fork/XML format failure are retained. The uninstrumented ASAN probe is outside memory coverage.',
          'binary':report['binary'],'binary_sha256':report['binary_sha256'],
          'original_report':{'path':str(path.relative_to(ROOT)),'sha256':fingerprint(path)},
          'inputs':report['inputs'], 'passed':3,'total':3,'original_passed':1,
          'parser_tests':{'passed':6,'total':6,'source_sha256':fingerprint(ROOT/'tests/valgrind-xml-test.py'),
                          'log':str(tests.relative_to(ROOT)),'log_sha256':fingerprint(tests)},
          'initial_report_sha256':fingerprint(ROOT/'evidence/sed-intrinsic-xml-initial.json'),
          'driver_sha256':fingerprint(Path(__file__)),'results':processes}
target.write_text(json.dumps(result,indent=2)+'\n')
print('Audited one unchanged original, three clean candidate processes and six XML parser checks')
