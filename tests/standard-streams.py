#!/usr/bin/env python3
"""Compare GNU standard-stream finalization, including closed descriptors."""
# SPDX-License-Identifier: GPL-3.0-or-later
import hashlib
import json
from pathlib import Path
import re
import subprocess

ROOT = Path(__file__).resolve().parents[1]
GNU = ROOT/'build/gnu-coreutils'
SOURCE = Path('/opt/src/coreutils-9.11')
STAGE = ROOT/'build/standard-stream-tests'
STAGE.mkdir(exist_ok=True)
DRIVER = r'''
#include <config.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <fcntl.h>
#include <errno.h>
#include "closeout.h"
#include "error.h"
#include "progname.h"
int main(int argc, char **argv) {
  if (argc != 2) return 125;
  set_program_name("stream-fixture");
  atexit(close_stdout);
  char const *mode = argv[1];
  if (!strcmp(mode, "full") || !strcmp(mode, "already-failed")) {
    if (!freopen("/dev/full", "w", stdout)) return 125;
    fputs("buffered output\n", stdout);
    if (!strcmp(mode, "already-failed") && fflush(stdout) == 0) return 125;
  } else if (!strcmp(mode, "flushed-closed")) {
    if (fputs("flushed output\n", stdout) < 0 || fflush(stdout) || close(1)) return 125;
  } else if (!strcmp(mode, "stdout-closed") || !strcmp(mode, "both-closed")
             || !strcmp(mode, "diagnostic-closed")) {
    if (close(1)) return 125;
    if (!strcmp(mode, "both-closed") && close(2)) return 125;
    if (!strcmp(mode, "diagnostic-closed")) error(0, ENOENT, "fixture");
  } else {
    if (!strcmp(mode, "stderr-closed") && close(2)) return 125;
    if (!strcmp(mode, "cloexec") && fcntl(1, F_SETFD, FD_CLOEXEC)) return 125;
    fputs("buffered output\n", stdout);
  }
  return 0;
}
'''
(STAGE/'driver.c').write_text(DRIVER)
for name in ('gnu', 'rboxc'):
    helpers = [ROOT/'src/bridges/error.c', ROOT/'src/bridges/closeout.c'] if name == 'rboxc' else []
    subprocess.run(['gcc', '-O2', '-I'+str(GNU/'lib'), '-I'+str(SOURCE/'lib'),
                    STAGE/'driver.c', *helpers, GNU/'lib/libcoreutils.a', '-o', STAGE/name], check=True)
results = []
for mode in ('normal', 'stdout-closed', 'stderr-closed', 'both-closed',
             'diagnostic-closed', 'flushed-closed', 'cloexec', 'full', 'already-failed'):
    observations = {}
    for implementation in ('gnu', 'rboxc', 'valgrind'):
        log = ROOT/'evidence/raw'/f'standard-streams-{mode}.log'
        prefix = ['valgrind', '--error-exitcode=97', '--track-fds=yes', '--leak-check=full',
                  '--show-leak-kinds=all', '--log-file='+str(log)] if implementation == 'valgrind' else []
        result = subprocess.run([*prefix, STAGE/('rboxc' if prefix else implementation), mode],
                                capture_output=True, timeout=30, env={'LC_ALL': 'C'})
        observation = {'status': result.returncode, 'stdout': result.stdout.decode(),
                       'stderr': result.stderr.decode()}
        if prefix:
            report = log.read_text()
            errors = re.search(r'ERROR SUMMARY: ([\d,]+) errors', report)
            fds = re.search(r'FILE DESCRIPTORS: (\d+) open \((\d+) (?:inherited|std)\)', report)
            observation.update(errors=int(errors[1].replace(',', '')) if errors else None,
                               non_inherited_descriptors=int(fds[1])-int(fds[2]) if fds else None,
                               log=str(log.relative_to(ROOT)))
        observations[implementation] = observation
    expected = observations['gnu']
    vg = observations['valgrind']
    passed = observations['rboxc'] == expected and all(vg[k] == v for k, v in expected.items())
    passed &= expected['status'] == (1 if mode in ('full', 'already-failed') else 0)
    passed &= vg['errors'] == 0 and vg['non_inherited_descriptors'] == 0
    results.append({'case': mode, 'pass': passed, **observations})
    print('PASS' if passed else 'OPEN', mode, flush=True)
report = {'passed': sum(r['pass'] for r in results), 'total': len(results),
          'scope': 'glibc standard FILE objects; GNU status/streams and candidate Valgrind',
          'adapter_hashes': {name: hashlib.sha256((ROOT/f'src/bridges/{name}.c').read_bytes()).hexdigest()
                             for name in ('error', 'closeout')}, 'results': results}
(ROOT/'evidence/standard-streams.json').write_text(json.dumps(report, indent=2)+'\n')
raise SystemExit(report['passed'] != report['total'])
