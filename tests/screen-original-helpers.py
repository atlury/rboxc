#!/usr/bin/env python3
"""Run reviewed Screen original units against original and linked helper objects."""
# SPDX-License-Identifier: GPL-3.0-or-later
import importlib.util
import json
import os
from pathlib import Path
import re
import subprocess
import sys
from comparison_profile import ComparisonProfile, fingerprint

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT/'tests/gnu'))
spec = importlib.util.spec_from_file_location('reviewed', ROOT/'tests/gnu/reviewed-original.py')
runner = importlib.util.module_from_spec(spec)
spec.loader.exec_module(runner)
profile = ComparisonProfile('screen-original-helpers', oracle=ROOT/'build/gnu-screen/screen')
source = Path('/opt/src/screen-5.0.2')
link = json.loads((ROOT/'evidence/screen-link.json').read_text())
inputs = {p: fingerprint(p) for p in [Path(__file__), ROOT/'evidence/screen-link.json',
    source/'Makefile.in', ROOT/'build/gnu-screen/config.h',
    *sorted((source/'tests').glob('*.[ch]'))]}
mock_profile = os.environ.get("RBOXC_SCREEN_MOCK_PROFILE", "original")
assert mock_profile in ("original", "private-symbols", "private-symbols-and-fixtures")
results = []

def run_build(argv, work, log):
    done = subprocess.run(argv, cwd=work, capture_output=True, timeout=60)
    log.write_bytes(done.stdout+done.stderr)
    assert done.returncode == 0, str(log)
    return {'argv': argv, 'cwd': str(work), 'log': str(log.relative_to(ROOT)), 'sha256': fingerprint(log)}

for unit in ['winmsgcond', 'winmsgbuf']:
    saved = profile.logs/unit
    saved.mkdir()
    paths = [p for p in link['helper_inputs'] if p.endswith('-'+unit+'.o')]
    assert len(paths) == 1
    helper = ROOT/paths[0]
    original = ROOT/'build/gnu-screen'/(unit+'.o')
    assert fingerprint(helper) == link['helper_inputs'][paths[0]]
    assert fingerprint(original) == link['original_inputs'][str(original.relative_to(ROOT))]
    for p in [helper, original, source/(unit+'.c'), source/(unit+'.h')]:
        inputs[p] = fingerprint(p)
    builds = []
    for name in ['test-'+unit, 'mallocmock']:
        inp = source/'tests'/(name+'.c')
        if name == 'test-winmsgbuf' and mock_profile == 'private-symbols-and-fixtures':
            text = inp.read_text()
            for variable in ['sznew', 'szmax']:
                needle = ('\t\tchar cfail = wmb_contents(wmb)[sznew - 1] + 1;' if variable == 'sznew'
                          else '\t\tchar last = wmb_contents(wmb)[szmax - 1];')
                assert text.count(needle) == 1
                text = text.replace(needle, '\t\t((char *)wmb_contents(wmb))['+variable+" - 1] = 'Q';\n"+needle)
            fixture_dir = saved/'fixture'
            (fixture_dir/'tests').mkdir(parents=True)
            (fixture_dir/'winmsgbuf.h').symlink_to(source/'winmsgbuf.h')
            for header in ['signature.h', 'macros.h']:
                (fixture_dir/'tests'/header).symlink_to(source/'tests'/header)
            inp = fixture_dir/'tests'/(name+'.c')
            inp.write_text(text)
            inputs[inp] = fingerprint(inp)
        obj = saved/(name+'.o')
        deps = saved/(name+'.d')
        argv = ['gcc', '-g', '-O2', '-std=gnu17', '-Wall', '-Wextra',
                '-iquote', str(ROOT/'build/gnu-screen'), '-iquote', str(source),
                '-MMD', '-MF', str(deps), '-c', str(inp), '-o', str(obj)]
        builds.append(run_build(argv, ROOT/'build/gnu-screen', saved/(name+'-compile.log')))
        for path in deps.read_text().replace('\\\n', '').split(':', 1)[1].split():
            p = Path(path)
            if not p.is_absolute(): p = ROOT/'build/gnu-screen'/p
            inputs[p.resolve()] = fingerprint(p)
    symbols = subprocess.check_output(['nm', '-g', '--defined-only', str(helper)], text=True)
    definitions = [line.split()[-1] for line in symbols.splitlines() if line.strip()]
    assert definitions and all(n.startswith('rboxc_screen_') for n in definitions)
    names = saved/'helper-symbols.map'
    names.write_text(''.join(n.removeprefix('rboxc_screen_')+' '+n+'\n' for n in definitions))
    candidate_obj = saved/'test-namespaced.o'
    builds.append(run_build(['objcopy', '--redefine-syms='+str(names),
        str(saved/('test-'+unit+'.o')), str(candidate_obj)], saved, saved/'namespace.log'))
    outcomes = {}
    for label, test_obj, linked in [('gnu', saved/('test-'+unit+'.o'), original),
                                     ('namespaced-helper', candidate_obj, helper)]:
        binary = saved/label
        mock = saved/'mallocmock.o'
        if mock_profile.startswith('private-symbols'):
            private = saved/(label+'-private')
            private.mkdir()
            rewritten = []
            for index, obj in enumerate([test_obj, linked, mock]):
                output = private/(str(index)+'.o')
                builds.append(run_build(['objcopy', '--redefine-sym=malloc=rboxc_test_malloc',
                    '--redefine-sym=realloc=rboxc_test_realloc', str(obj), str(output)],
                    saved, private/(str(index)+'-rename.log')))
                rewritten.append(output)
            test_obj, linked, mock = rewritten
        builds.append(run_build(['gcc', str(test_obj), str(linked), str(mock),
            '-o', str(binary)], saved, saved/(label+'-link.log')))
        for instrument in [False, True]:
            key = label+('-valgrind' if instrument else '')
            log = saved/(key+'-memory.log')
            argv = [str(binary)]
            if instrument:
                argv = ['/usr/bin/valgrind', '--leak-check=full', '--show-leak-kinds=all',
                    '--track-fds=yes', '--trace-children=yes', '--log-file='+str(log), *argv]
            done = subprocess.run(argv, capture_output=True, timeout=60, cwd=saved,
                env={'PATH':'/usr/bin:/bin', 'HOME':str(saved), 'LC_ALL':'C'})
            output = saved/(key+'-output.log')
            output.write_bytes(done.stdout+done.stderr)
            memory = None
            clean = None
            if instrument:
                text = log.read_text()
                pids = set(re.findall(r'^==([0-9]+)==', text, re.M))
                assert len(pids) == 1
                memory = runner.parse_memory_log(text, pids.pop(), exec_only=True)
                clean = memory['complete_exec_log'] and memory['errors'] == memory['non_inherited_descriptors'] == 0 and not any(
                    memory['heap_bytes'].get(k, 0) for k in ('definitely lost','indirectly lost','possibly lost'))
            outcomes[key] = {'status':done.returncode, 'stdout':done.stdout.hex(), 'stderr':done.stderr.hex(),
                'binary':str(binary.relative_to(ROOT)), 'binary_sha256':fingerprint(binary),
                'output':str(output.relative_to(ROOT)), 'output_sha256':fingerprint(output),
                'memory':memory, 'memory_clean':clean,
                'memory_log':str(log.relative_to(ROOT)) if instrument else None,
                'memory_log_sha256':fingerprint(log) if instrument else None}
    passed = all(o['status'] == 0 and not o['stdout'] and not o['stderr'] for o in outcomes.values()) and outcomes['namespaced-helper-valgrind']['memory_clean']
    results.append({'unit':unit, 'pass':passed, 'builds':builds, 'outcomes':outcomes})
    print(unit, 'PASS' if passed else 'OPEN', flush=True)
    assert all(fingerprint(p) == expected for p, expected in inputs.items())
    report = {**profile.metadata(), 'scope':'Two unchanged original Screen C unit tests linked separately against '
        'the original native helper and the exact namespaced helper object recorded in the candidate link inputs. '
        'These validate native helpers, not the translated Rust command entry or terminal integration. '
        'Original assertions and allocation-mock bodies are retained. The private-symbols profile '
        'renames only malloc/realloc definitions and references in temporary object copies so '
        'Valgrind observes the mocks underlying libc calls without replacing the mock itself. '
        'The private-symbols-and-fixtures profile additionally initializes two unused tail bytes '
        'before the test snapshots them to assert preservation on allocation failure. '
        'Original assertions and production objects are unchanged; original-profile findings remain separate.',
        'allocation_mock_profile':mock_profile,
        'inputs':{str(p):v for p,v in inputs.items()}, 'driver_sha256':fingerprint(Path(__file__)),
        'passed':sum(r['pass'] for r in results), 'total':len(results), 'complete':len(results)==2, 'results':results}
    profile.report.write_text(json.dumps(report,indent=2)+'\n')
raise SystemExit(not all(r['pass'] for r in results))
