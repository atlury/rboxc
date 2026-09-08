#!/usr/bin/env python3
"""Run individually reviewed GNU Cpio Autotest selections unchanged."""
# SPDX-License-Identifier: GPL-3.0-or-later
import importlib.util
import json
import os
from pathlib import Path
import re
import shlex
import shutil
import subprocess
import sys
import tempfile
from comparison_profile import ComparisonProfile, fingerprint

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT/'tests/gnu'))
spec = importlib.util.spec_from_file_location('reviewed', ROOT/'tests/gnu/reviewed-original.py')
runner = importlib.util.module_from_spec(spec)
spec.loader.exec_module(runner)
profile = ComparisonProfile('cpio-original', oracle=ROOT/'build/gnu-cpio/src/cpio', selections=True)
source = Path(json.loads((ROOT/'inventory/sources.json').read_text())['cpio']['source'])
selections = {'version':(1,'version.at'),'inout':(2,'inout.at'),'interdir':(7,'interdir.at'),
              **{f'setstat{i:02}':(7+i,f'setstat{i:02}.at') for i in range(1,6)},
              'linktime':(16,'linktime.at'),'linktime01':(17,'linktime01.at')}
selected = profile.options.commands or list(selections)
assert len(set(selected)) == len(selected) and set(selected) <= set(selections)
helpers = {n: ROOT/'build/gnu-coreutils/src/coreutils' for n in ('cat','rm','mkdir','chmod','touch','sort','echo','basename','cp','ln','true','false','sleep','ls','mv','mktemp','cut','id','date','printf','dd','rmdir','expr','tr','wc','head','tail','uname','cksum')}
helpers.update({n: ROOT/'build/gnu-diffutils/src'/n for n in ('cmp', 'diff')})
helpers['sed'] = ROOT/'build/gnu-sed/sed/sed'
helpers['grep'] = ROOT/'build/gnu-grep/src/grep'
helpers['genfile'] = ROOT/'build/gnu-cpio/tests/genfile'
helpers['find'] = ROOT/'build/gnu-findutils/find/find'
inputs = {p: fingerprint(p) for p in {*helpers.values(), profile.oracle, source/'tests/testsuite', source/'tests/testsuite.at',
    ROOT/'build/gnu-cpio/tests/atconfig', ROOT/'build/gnu-cpio/tests/atlocal', Path('/bin/bash'), Path('/bin/sh').resolve(), Path('/usr/bin/awk').resolve(), Path(__file__)}}
for filename in ('genfile.c', 'argcv.c', 'argcv.h', 'Makefile.am'):
    path = source/'tests'/filename
    inputs[path] = fingerprint(path)
for name in selected:
    path = source/'tests'/selections[name][1]
    inputs[path] = fingerprint(path)
results = []
for name in selected:
    number, filename = selections[name]
    outcomes = {}
    for implementation in ('gnu', 'rboxc'):
        for instrument in (False, True):
            key = implementation + ('-valgrind' if instrument else '')
            saved = profile.logs/(name+'-'+key)
            saved.mkdir()
            with tempfile.TemporaryDirectory(prefix='rboxc-cpio-original-') as directory:
                work = Path(directory)
                for sub in ('exec', 'real', 'deps', 'memory', 'copies'):
                    (work/sub).mkdir()
                for command, binary in helpers.items():
                    (work/'deps'/command).symlink_to(binary)
                (work/'real/cpio').symlink_to(profile.oracle if implementation == 'gnu' else profile.binary)
                invocation = ['cpio']
                if instrument:
                    invocation = ['/usr/bin/valgrind', '--leak-check=full', '--show-leak-kinds=all', '--track-fds=yes', '--trace-children=yes', '--log-file='+str(work/'memory/%p.log'), *invocation]
                path = str(work/'real')+':'+str(work/'deps')+':/usr/bin:/bin'
                wrapper = work/'exec/cpio'
                wrapper.write_text('#!/bin/sh\nPATH='+shlex.quote(path)+'\nexport PATH\nexec '+shlex.join(invocation)+' "$@"\n')
                wrapper.chmod(0o755)
                config = (ROOT/'build/gnu-cpio/tests/atconfig').read_text()
                config = config.replace(str(ROOT/'build/gnu-cpio/tests'), directory).replace(str(ROOT/'build/gnu-cpio'), directory)
                (work/'atconfig').write_text(config)
                shutil.copy2(ROOT/'build/gnu-cpio/tests/atlocal', work/'atlocal')
                environment = {'PATH': str(work/'deps')+':/usr/bin:/bin', 'HOME': directory, 'TMPDIR': directory,
                               'LC_ALL': 'C', 'LANGUAGE': 'C', 'TZ': 'UTC0', 'CONFIG_SHELL': '/bin/bash'}
                command = ['/bin/bash', str(source/'tests/testsuite'), '--debug', str(number),
                           'AUTOTEST_PATH='+str(work/'exec')+':'+str(work/'deps')]
                done = subprocess.run(command,cwd=work,env=environment,stdin=subprocess.DEVNULL,
                    stdout=subprocess.PIPE,stderr=subprocess.STDOUT,timeout=300)
                (saved/'driver.log').write_bytes(done.stdout)
                for file in ('atconfig', 'atlocal', 'testsuite.log'):
                    if (work/file).exists():
                        shutil.copy2(work/file, saved/file)
                if (work/'testsuite.dir').exists():
                    shutil.copytree(work/'testsuite.dir', saved/'suite', symlinks=True)
                shutil.copytree(work/'memory', saved/'memory')
                output = done.stdout.decode(errors='replace')
                assertions = re.findall(r'^\s*(\d+):\s+.*?\s+(ok|FAILED|skipped|expected failure)\s*$', output, re.M)
                passed = done.returncode == 0 and assertions == [(str(number), 'ok')]
                logs = [{**runner.parse_memory_log(p.read_text(), p.stem, exec_only=True), 'log': str(p.relative_to(ROOT)), 'sha256': fingerprint(p)} for p in sorted((saved/'memory').glob('*.log'))]
                clean = bool(logs) and all(m['complete_exec_log'] and m['errors'] == 0 and m['non_inherited_descriptors'] == 0 and not any(m['heap_bytes'].get(k, 0) for k in ('definitely lost','indirectly lost','possibly lost')) for m in logs)
                outcomes[key] = {'status': done.returncode, 'assertions': assertions, 'assertions_pass': passed,
                                 'memory': logs, 'memory_clean': clean if instrument else None,
                                 'driver_log': str((saved/'driver.log').relative_to(ROOT)), 'driver_log_sha256': fingerprint(saved/'driver.log'),
                                 'private_atconfig_sha256': fingerprint(saved/'atconfig')}
    row = {'selection': name, 'autotest_number': number, 'source': 'tests/'+filename, 'source_sha256': inputs[source/'tests'/filename],
           'outcomes': outcomes, 'assertions_pass': all(r['assertions_pass'] for r in outcomes.values())}
    row['pass'] = row['assertions_pass'] and outcomes['rboxc-valgrind']['memory_clean']
    results.append(row)
    assert all(fingerprint(p) == expected for p, expected in inputs.items())
    report = {'scope':'Unchanged reviewed GNU Cpio Autotest selections in private directories. Original atlocal is retained; atconfig build paths point into fixtures. The wrapper preserves argv[0]=cpio and instruments each command with child tracing. Native test helpers are dependencies, not ports. No tape-device or held-out original selections are executed.',
              **profile.metadata(), 'inputs': {str(p): value for p, value in inputs.items()},
              'selected': selected, 'planned_total': len(selected), 'complete': len(results) == len(selected),
              'passed': sum(r['pass'] for r in results), 'total': len(results), 'results': results}
    profile.report.write_text(json.dumps(report, indent=2)+'\n')
    print('PASS' if row['pass'] else 'OPEN', name, flush=True)
raise SystemExit(any(not r['pass'] for r in results))
