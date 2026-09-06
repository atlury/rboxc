#!/usr/bin/env python3
"""Compare GNU reopening with the descriptor-probe adapter in private fixtures."""
# SPDX-License-Identifier: GPL-3.0-or-later
import json
import os
from pathlib import Path
import re
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[1]
GNU = ROOT/'build/gnu-coreutils'
SOURCE = Path(os.environ.get('GNU_COREUTILS_SOURCE', '/opt/src/coreutils-9.11'))
STAGE = ROOT/'build/freopen-tests'
STAGE.mkdir(exist_ok=True)
DRIVER = r'''
#include <config.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <errno.h>
#include <fcntl.h>
#include <unistd.h>
extern FILE *freopen_safer(char const *, char const *, FILE *);
int main(int argc, char **argv) {
  if (argc != 4) return 10;
  int target = atoi(argv[1]), mask = atoi(argv[2]);
  FILE *streams[] = {stdin, stdout, stderr};
  /* The target stream stays valid; only other streams are closed. */
  for (int fd = 0; fd < 3; fd++) {
    if (fd != target && (mask & (1 << fd))) {
      if (fclose(streams[fd])) return 11;
    } else if (fcntl(fd, F_SETFD, FD_CLOEXEC) < 0) return 12;
  }
  FILE *result = freopen_safer(argv[3], "w+", streams[target]);
  int saved = errno;
  if (strchr(argv[3], '/')) {
    if (result || saved != ENOENT) return 13;
  } else {
    if (result != streams[target] || fileno(result) != target) return 14;
    if (fputs("fixture\n", result) < 0 || fflush(result) || fseek(result, 0, SEEK_SET)) return 15;
    char text[16];
    if (!fgets(text, sizeof text, result) || strcmp(text, "fixture\n")) return 16;
    if (fclose(result)) return 17;
  }
  for (int fd = 0; fd < 3; fd++) {
    if (fd == target) continue;
    int flags = fcntl(fd, F_GETFD);
    if (mask & (1 << fd)) {
      if (flags != -1 || errno != EBADF) return 18;
    } else {
      if (flags != FD_CLOEXEC) return 19;
      if (fclose(streams[fd])) return 20;
    }
  }
  return 0;
}
'''
(STAGE/'driver.c').write_text(DRIVER)
for name, helper in [('gnu', SOURCE/'lib/freopen-safer.c'),
                     ('rboxc', ROOT/'src/bridges/freopen-safer.c')]:
    subprocess.run(['gcc', '-O2', '-I'+str(GNU/'lib'), '-I'+str(SOURCE/'lib'),
                    STAGE/'driver.c', helper, GNU/'lib/libcoreutils.a', '-o', STAGE/name], check=True)
results = []
for target in range(3):
    for mask in range(8):
        if mask & (1 << target):
            continue
        for output in ('output', 'missing/output'):
            observations = {}
            for implementation in ('gnu', 'rboxc', 'valgrind'):
                with tempfile.TemporaryDirectory(prefix='rboxc-freopen-') as temporary:
                    cwd = Path(temporary)
                    log = ROOT/'evidence/raw'/f'freopen-{target}-{mask}-{output.replace("/", "-")}.log'
                    prefix = ['valgrind', '--error-exitcode=97', '--track-fds=yes', '--leak-check=full',
                              '--log-file='+str(log)] if implementation == 'valgrind' else []
                    binary = STAGE/('rboxc' if implementation == 'valgrind' else implementation)
                    run = subprocess.run([*prefix, binary, str(target), str(mask), output],
                                         cwd=cwd, input=b'', capture_output=True, timeout=20)
                    observation = {'status': run.returncode,
                                   'content': (cwd/'output').read_text() if (cwd/'output').exists() else None}
                    if prefix:
                        report = log.read_text()
                        errors = re.search(r'ERROR SUMMARY: (\d+) errors', report)
                        fds = re.search(r'FILE DESCRIPTORS: (\d+) open \((\d+) (?:inherited|std)\)', report)
                        observation.update(errors=int(errors[1]) if errors else None,
                                           non_inherited_descriptors=int(fds[1])-int(fds[2]) if fds else None)
                    observations[implementation] = observation
            vg = observations['valgrind']
            passed = all(r['status'] == 0 for r in observations.values())
            passed &= observations['gnu'] == observations['rboxc']
            passed &= vg['content'] == observations['gnu']['content']
            passed &= vg['errors'] == 0 and vg['non_inherited_descriptors'] == 0
            results.append({'target': target, 'closed_mask': mask, 'path': output,
                            'pass': passed, **observations})
report = {'passed': sum(row['pass'] for row in results), 'total': len(results), 'results': results}
(ROOT/'evidence/freopen-safer.json').write_text(json.dumps(report, indent=2)+'\n')
print(f"GNU descriptor preservation and Valgrind: {report['passed']}/{report['total']} pass")
raise SystemExit(report['passed'] != report['total'])
