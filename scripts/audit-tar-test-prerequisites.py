#!/usr/bin/env python3
"""Verify and exercise GNU Tar's unchanged timestamp and sparse-file probes."""
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
target = ROOT/'evidence/tar-test-prerequisites.json'
assert not target.exists()
build = ROOT/'build/gnu-tar/tests'
inputs = {p: fingerprint(p) for p in [build/'Makefile', build.parent/'gnu/libgnu.a',
    ROOT/'build/tar-test-prerequisites-build.log', Path('/usr/bin/gcc').resolve(),
    Path('/usr/bin/make'), Path('/usr/bin/valgrind'), Path(__file__)]}
results = []
logs = ROOT/'evidence/raw/tar-test-prerequisites'
logs.mkdir()
for name in ('ckmtime', 'checkseekhole'):
    dependency = build/'.deps'/(name+'.Po')
    words = dependency.read_text().replace('\\\n', ' ').splitlines()[0].split()[1:]
    for filename in words:
        p = (build/filename).resolve(strict=True)
        inputs[p] = fingerprint(p)
    for p in (dependency, build/name, build/(name+'.o')):
        inputs[p] = fingerprint(p)
    outcomes = {}
    for instrument in (False, True):
        key = 'valgrind' if instrument else 'native'
        with tempfile.TemporaryDirectory(prefix='tar-prerequisite-') as directory:
            command = [str(build/name)]
            log = logs/(name+'.log')
            if instrument:
                command = ['/usr/bin/valgrind', '--leak-check=full', '--show-leak-kinds=all',
                    '--track-fds=yes', '--log-file='+str(log), *command]
            done = subprocess.run(command, cwd=directory, capture_output=True, timeout=30,
                env={'PATH':'/usr/bin:/bin', 'LC_ALL':'C'})
            assert done.returncode == 0 and not done.stdout and not done.stderr
            assert not list(Path(directory).iterdir()), 'probe must remove its own fixture'
            outcome = {'status':done.returncode, 'stdout':'', 'stderr':'', 'fixture_removed':True}
            if instrument:
                text = log.read_text()
                pids = set(re.findall(r'^==([0-9]+)==', text, re.M))
                assert len(pids) == 1
                memory = runner.parse_memory_log(text, pids.pop(), exec_only=True)
                assert memory['complete_exec_log'] and memory['errors'] == memory['non_inherited_descriptors'] == 0
                assert not any(memory['heap_bytes'].get(k,0) for k in ('definitely lost','indirectly lost','possibly lost'))
                outcome.update(memory=memory, log=str(log.relative_to(ROOT)), sha256=fingerprint(log))
            outcomes[key] = outcome
    results.append({'helper':name, 'outcomes':outcomes})
assert all(fingerprint(p)==h for p,h in inputs.items())
target.write_text(json.dumps({'scope':'Unchanged GNU Tar test-only prerequisites, built by their original Make targets. Both native and Valgrind probes succeed and remove their private fixtures. These helpers are not applet ports.',
    'build_command':['make','-C','build/gnu-tar/tests','ckmtime','checkseekhole'],
    'inputs':{str(p):h for p,h in inputs.items()}, 'results':results,
    'passed':2, 'clean_helper_processes':2, 'driver_sha256':fingerprint(Path(__file__))},indent=2)+'\n')
print('Verified two original prerequisite helpers and two clean process logs')
