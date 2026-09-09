#!/usr/bin/env python3
"""Exercise ownership boundaries using Tar's exact adapted input-pipe helpers."""
# SPDX-License-Identifier: GPL-3.0-or-later
import importlib.util
import json
from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT/'tests/gnu'))
from comparison_profile import fingerprint
spec = importlib.util.spec_from_file_location('reviewed', ROOT/'tests/gnu/reviewed-original.py')
runner = importlib.util.module_from_spec(spec)
spec.loader.exec_module(runner)
target = ROOT/'evidence/tar-child-input-contract-final.json'
assert not target.exists()
stage = ROOT/'build/tar-child-input-contract-final'
logs = ROOT/'evidence/raw/tar-child-input-contract-final'
stage.mkdir()
logs.mkdir()
source = ROOT/'build/tar-cleanup/system.c'
cleanup = json.loads((ROOT/'evidence/tar-native-cleanup.json').read_text())
row = next(r for r in cleanup['adaptations'] if r['source'].endswith('/src/system.c'))
assert fingerprint(source) == row['adapted_source_sha256']
text = source.read_text()
ownership = text[text.index('static pid_t rboxc_input_owner;'):text.index('static _Noreturn void\nxexec')]
duplicate = text[text.index('static void\nxdup2 ('):text.index('/* Propagate any failure')]
probe = stage/'contract.c'
probe.write_text('''#include <stdbool.h>
#include <stdlib.h>
#include <errno.h>
#include <unistd.h>
#include <fcntl.h>
#include <sys/stat.h>
#include <sys/wait.h>
#include <string.h>
static void xalloc_die(void) { _exit(99); }
static void xclose(int fd) { if (close(fd)) _exit(98); }
#define _(text) text
#define FATAL_ERROR(args) do { (void)e; _exit(97); } while (0)
''' + ownership + duplicate + '''
static void own_pipe(void) {
  int descriptors[2];
  if (pipe(descriptors)) _exit(90);
  xdup2(descriptors[0], 0);
  xclose(descriptors[1]);
}
static void require_open(void) { if (fcntl(0, F_GETFD) < 0) _exit(91); }
static void require_closed(void) {
  errno = 0;
  if (fcntl(0, F_GETFD) != -1 || errno != EBADF) _exit(92);
}
int main(int argc, char **argv) {
  if (argc != 2) return 89;
  if (!strcmp(argv[1], "failed-stat")) {
    xclose(0);
    rboxc_record_child_input();
    if (open("/dev/null", O_RDONLY) != 0) return 88;
    rboxc_release_child_input();
    require_open();
    xclose(0);
    return 0;
  }
  own_pipe();
  if (!strcmp(argv[1], "inherited")) {
    pid_t child = fork();
    if (child < 0) return 87;
    if (!child) {
      rboxc_release_child_input();
      require_open();
      xclose(0);
      return 0;
    }
    int status;
    if (waitpid(child, &status, 0) != child || status != 0) return 86;
    require_open();
  } else if (!strcmp(argv[1], "reused")) {
    xclose(0);
    if (open("/dev/null", O_RDONLY) != 0) return 85;
    rboxc_release_child_input();
    require_open();
    xclose(0);
    return 0;
  } else if (!strcmp(argv[1], "replaced")) {
    own_pipe();
  } else if (strcmp(argv[1], "owned")) return 84;
  rboxc_release_child_input();
  require_closed();
  return 0;
}
''')
binary = stage/'contract'
command = ['cc', '-O2', '-Wall', '-Wextra', '-Werror', '-o', str(binary), str(probe)]
with (logs/'build.log').open('w') as stream:
    subprocess.run(command, stdout=stream, stderr=subprocess.STDOUT, check=True)
results = []
processes = []
for name in ['owned', 'inherited', 'reused', 'replaced']:
    outcomes = {}
    for instrument in [False, True]:
        directory = logs/(name+('-valgrind' if instrument else '-native'))
        directory.mkdir()
        args = [str(binary), name]
        if instrument:
            args = ['valgrind', '--leak-check=full', '--show-leak-kinds=all',
                    '--track-fds=yes', '--trace-children=yes',
                    '--log-file='+str(directory/'%p.log'), *args]
        done = subprocess.run(args, stdin=subprocess.DEVNULL, capture_output=True, timeout=30)
        assert done.returncode == 0 and not done.stdout and not done.stderr
        memory = []
        for path in directory.glob('*.log'):
            parsed = runner.parse_memory_log(path.read_text(), path.stem, exec_only=True)
            assert parsed['complete_exec_log'] and parsed['errors'] == parsed['non_inherited_descriptors'] == 0
            assert not any(parsed['heap_bytes'].get(k, 0) for k in
                           ('definitely lost', 'indirectly lost', 'possibly lost'))
            item = {'log': str(path.relative_to(ROOT)), 'sha256': fingerprint(path), **parsed}
            memory.append(item)
            processes.append({'case': name, **item})
        assert not instrument or len(memory) == (2 if name == 'inherited' else 1)
        outcomes['valgrind' if instrument else 'native'] = {'status': 0, 'memory': memory}
    results.append({'case': name, 'pass': True, 'outcomes': outcomes})
target.write_text(json.dumps({
    'scope': 'Four native helper lifetime contracts using the exact adapted ownership and xdup2 source. Inherited child input, descriptor reuse, and replacement are checked independently of archive assertions. GNU fatal-diagnostic plumbing is stubbed in this isolated helper probe; production integration is separate.',
    'passed': 4, 'total': 4, 'clean_processes': len(processes),
    'inputs': {str(p): fingerprint(p) for p in [source, probe, binary, Path(__file__)]},
    'compiler_command': command, 'results': results, 'processes': processes,
}, indent=2)+'\n')
print('Four input ownership contracts pass; five process logs are clean')
