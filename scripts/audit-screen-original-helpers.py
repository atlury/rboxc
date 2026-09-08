#!/usr/bin/env python3
"""Audit original Screen helper assertions, fixture changes and memory evidence."""
# SPDX-License-Identifier: GPL-3.0-or-later
import importlib.util
import json
from pathlib import Path
import re
import subprocess
import sys
import tempfile

ROOT = Path(__file__).resolve().parents[1]
sys.path[:0] = [str(ROOT/'tests'), str(ROOT/'tests/gnu')]
from comparison_profile import fingerprint
spec = importlib.util.spec_from_file_location('reviewed', ROOT/'tests/gnu/reviewed-original.py')
runner = importlib.util.module_from_spec(spec)
spec.loader.exec_module(runner)
target = ROOT/'evidence/screen-original-helpers-validation.json'
assert not target.exists()
paths = {name:ROOT/'evidence'/('screen-original-helpers-'+name+'.json') for name in
         ['initial','private','isolated','defined-fixtures']}
reports = {name:json.loads(p.read_text()) for name,p in paths.items()}
drivers = {'initial':ROOT/'evidence/raw/screen-original-helpers-initial-driver.py',
           'private':ROOT/'evidence/raw/screen-original-helpers-initial-driver.py',
           'isolated':ROOT/'evidence/raw/screen-original-helpers-isolated-driver.py',
           'defined-fixtures':ROOT/'tests/screen-original-helpers.py'}
processes = []
artifacts = {}
for name,data in reports.items():
    assert data['complete'] and data['total'] == 2
    assert fingerprint(drivers[name]) == data['driver_sha256']
    assert fingerprint(Path(data['binary'])) == data['binary_sha256']
    for path,expected in data['inputs'].items():
        p = drivers[name] if path == str(ROOT/'tests/screen-original-helpers.py') else Path(path)
        assert fingerprint(p) == expected, path
    for result in data['results']:
        for key,o in result['outcomes'].items():
            assert fingerprint(ROOT/o['binary']) == o['binary_sha256']
            assert fingerprint(ROOT/o['output']) == o['output_sha256']
            assert (ROOT/o['output']).read_bytes() == bytes.fromhex(o['stdout']+o['stderr'])
            if key.endswith('-valgrind'):
                log = ROOT/o['memory_log']
                assert fingerprint(log) == o['memory_log_sha256']
                pids = set(re.findall(r'^==([0-9]+)==',log.read_text(),re.M))
                assert len(pids)==1
                parsed = runner.parse_memory_log(log.read_text(),pids.pop(),exec_only=True)
                assert parsed == o['memory'] and parsed['complete_exec_log']
                if name == 'defined-fixtures':
                    assert o['status']==0 and parsed['errors']==parsed['non_inherited_descriptors']==0
                    assert not any(parsed['heap_bytes'].get(k,0) for k in
                                   ['definitely lost','indirectly lost','possibly lost'])
                processes.append({'profile':name,'unit':result['unit'],'implementation':key,
                    'status':o['status'],'log':o['memory_log'],'sha256':o['memory_log_sha256'],**parsed})
            elif name in ['initial','defined-fixtures']:
                assert o['status']==0 and not o['stdout'] and not o['stderr']
        for build in result['builds']:
            assert fingerprint(ROOT/build['log'])==build['sha256']
            argv = build['argv']
            if argv[0] != 'objcopy': continue
            original, actual = map(Path,argv[-2:])
            with tempfile.TemporaryDirectory(prefix='rboxc-screen-helper-audit-') as directory:
                output = Path(directory)/'copy.o'
                subprocess.run([*argv[:-1],str(output)],check=True,capture_output=True)
                assert output.read_bytes()==actual.read_bytes()
            artifacts[str(original)] = fingerprint(original)
            artifacts[str(actual)] = fingerprint(actual)
            if '--redefine-sym=malloc=rboxc_test_malloc' in argv:
                # Symbol relocation only: production instructions are unchanged.
                with tempfile.TemporaryDirectory(prefix='rboxc-screen-text-audit-') as directory:
                    texts=[]
                    for i,obj in enumerate([original,actual]):
                        out=Path(directory)/str(i)
                        subprocess.run(['objcopy','-O','binary','--only-section=.text',str(obj),str(out)],check=True)
                        texts.append(out.read_bytes())
                    assert texts[0]==texts[1]
final=reports['defined-fixtures']
assert final['passed']==2 and final['allocation_mock_profile']=='private-symbols-and-fixtures'
original=Path('/opt/src/screen-5.0.2/tests/test-winmsgbuf.c').read_text()
expected=original
for variable,needle in [('sznew','\t\tchar cfail = wmb_contents(wmb)[sznew - 1] + 1;'),
                        ('szmax','\t\tchar last = wmb_contents(wmb)[szmax - 1];')]:
    assert expected.count(needle)==1
    expected=expected.replace(needle,'\t\t((char *)wmb_contents(wmb))['+variable+" - 1] = 'Q';\n"+needle)
fixture_paths=[Path(p) for p in final['inputs'] if '/fixture/tests/test-winmsgbuf.c' in p]
assert len(fixture_paths)==1 and fixture_paths[0].read_text()==expected
report={'scope':'Both Screen original helper units pass natively against original GNU and exact namespaced '
    'production helper objects. In the final instrumented profile, private object copies rename malloc/realloc '
    'references so GNU allocation mocks remain active; two private buffer fixture bytes are initialized before '
    'unchanged preservation assertions read them. Original test and production helper sources are preserved. '
    'All four final helper-unit Valgrind processes are clean. These are native helper checks, not Rust entry '
    'integration. Earlier mock-interception failures and uninitialized fixture findings remain recorded. '
    'The private report repeats the initial profile and adds no distinct unit coverage.',
    'binary':final['binary'],'binary_sha256':final['binary_sha256'],
    'native_original_units':2,'instrumented_units':2,'clean_namespaced_helper_processes':2,
    'clean_gnu_helper_processes':2,'fixture_initializations':2,
    'reports':{name:{'path':str(p.relative_to(ROOT)),'sha256':fingerprint(p)} for name,p in paths.items()},
    'drivers':{name:{'path':str(p.relative_to(ROOT)),'sha256':fingerprint(p)} for name,p in drivers.items()},
    'artifacts':artifacts,'driver_sha256':fingerprint(Path(__file__)),'processes':processes}
target.write_text(json.dumps(report,indent=2)+'\n')
print('Audited two original helper units; four clean final Valgrind processes; prior findings retained')
