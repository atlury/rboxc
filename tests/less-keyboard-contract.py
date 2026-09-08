#!/usr/bin/env python3
"""Check owned keyboard handles, borrowed fallbacks and descriptor reuse."""
# SPDX-License-Identifier: GPL-3.0-or-later
import importlib.util
import json
from pathlib import Path
import subprocess
import sys
from comparison_profile import fingerprint

ROOT = Path(__file__).resolve().parents[1]
sys.path[:0] = [str(ROOT/'scripts'), str(ROOT/'tests/gnu')]
from less_cleanup import adapt
spec = importlib.util.spec_from_file_location('reviewed', ROOT/'tests/gnu/reviewed-original.py')
runner = importlib.util.module_from_spec(spec)
spec.loader.exec_module(runner)
stage = ROOT/'build/less-keyboard-contract'
assert not stage.exists()
stage.mkdir()
report_path = ROOT/'evidence/less-keyboard-contract.json'
assert not report_path.exists()
source = Path('/opt/src/less-704/ttyin.c')
adapted = stage/'ttyin.c'
adapted.write_text(adapt(source.read_text()))
harness = stage/'contract.c'
harness.write_text(r'''#include <unistd.h>
#include <fcntl.h>
#include <errno.h>
#include <stdlib.h>
#include <string.h>
extern int tty;
extern char *ttyin_name;
void open_getchr(void);
void close_getchr(void);
static void require(int condition) { if (!condition) _exit(90); }
static int live(int fd) { return fcntl(fd,F_GETFD) >= 0; }
int main(int argc,char **argv) {
    require(argc==3);
    int adapted=strcmp(argv[2],"adapted")==0;
    const char *mode=argv[1];
    require(live(0)&&live(1)&&live(2));
    errno=E2BIG; close_getchr(); require(errno==E2BIG);
    require(live(0)&&live(1)&&live(2));
    if (!strcmp(mode,"unopened")) return 0;
    int borrowed=-1;
    ttyin_name="/dev/null";
    if (!strcmp(mode,"fallback-stdin")) { ttyin_name="/missing-rboxc-keyboard-fixture"; borrowed=0; }
    if (!strcmp(mode,"fallback-stderr")) { ttyin_name=NULL; borrowed=2; }
    int desired=-1;
    if (!strncmp(mode,"owned",5) && mode[5]) { desired=mode[5]-'0'; require(desired>=0&&desired<=2); require(close(desired)==0); }
    int rounds=!strcmp(mode,"reopen")?3:1;
    for(int i=0;i<rounds;i++) {
        open_getchr();
        int fd=tty;
        require(live(fd));
        if(desired>=0) require(fd==desired);
        if(borrowed>=0) require(fd==borrowed);
        errno=E2BIG; close_getchr(); require(errno==E2BIG);
        if(borrowed>=0) require(live(fd));
        else {
            require(live(fd)==!adapted);
            if(!adapted) require(close(fd)==0);
        }
        if(!strcmp(mode,"reuse")) {
            int replacement=open("/dev/null",O_RDONLY);
            require(replacement==fd);
            close_getchr(); require(live(replacement));
            require(close(replacement)==0);
        }
    }
    for(int fd=0;fd<3;fd++) if(fd!=desired) require(live(fd));
    return 0;
}
''')
records = [json.loads(p.read_text()) for p in (ROOT/'build/less-cc-records').glob('*.json')]
record = next(r for r in records if r.get('file') == str(source) and r.get('kind') == 'compile')
results = []
for label, csource in [('gnu', source), ('adapted', adapted)]:
    obj = stage/(label+'.o')
    binary = stage/label
    argv = record['arguments'].copy()
    argv[argv.index(str(source))] = str(csource)
    argv += ['-iquote'+str(source.parent), '-DLESSTEST', '-ffunction-sections', '-fdata-sections', '-o', str(obj)]
    with (stage/(label+'-build.log')).open('wb') as stream:
        subprocess.run(argv, cwd=record['directory'], stdout=stream, stderr=subprocess.STDOUT, check=True)
        subprocess.run(['gcc', str(harness), str(obj), '-Wl,--gc-sections', '-o', str(binary)],
                       stdout=stream, stderr=subprocess.STDOUT, check=True)
    for mode in ['unopened', 'owned', 'owned0', 'owned1', 'owned2', 'reuse', 'reopen', 'fallback-stdin', 'fallback-stderr']:
        log = stage/(label+'-'+mode+'.log')
        done = subprocess.run(['/usr/bin/valgrind', '--leak-check=full', '--show-leak-kinds=all', '--track-fds=yes',
                               '--log-file='+str(log), str(binary), mode, label],
                              stdin=subprocess.DEVNULL, capture_output=True, start_new_session=True, timeout=30)
        assert done.returncode == 0 and not done.stdout and not done.stderr, (label, mode, done.returncode)
        text = log.read_text()
        import re
        pids = set(re.findall(r'^==([0-9]+)==', text, re.M))
        assert len(pids) == 1
        memory = runner.parse_memory_log(text, pids.pop(), exec_only=True)
        assert memory['complete_exec_log'] and memory['errors'] == memory['non_inherited_descriptors'] == 0
        assert not any(memory['heap_bytes'].get(k, 0) for k in ('definitely lost', 'indirectly lost', 'possibly lost'))
        results.append({'implementation': label, 'case': mode, 'pass': True, 'memory': memory,
                        'log': str(log.relative_to(ROOT)), 'log_sha256': fingerprint(log)})
report = {'scope': 'Nine contracts per implementation. Native GNU keeps opened keyboard handles, which the '
          'harness closes after observing that baseline. The adaptation closes owned handles including reused '
          'standard descriptor numbers, preserves borrowed stdin/stderr, preserves errno, supports reopen and '
          'does not close an unrelated descriptor after repeated cleanup. All 18 harness processes are clean.',
          'passed': len(results), 'total': len(results), 'results': results,
          'driver_sha256': fingerprint(Path(__file__)), 'adapter_sha256': fingerprint(ROOT/'scripts/less_cleanup.py'),
          'original_source_sha256': fingerprint(source),
          'artifacts': {str(p.relative_to(ROOT)): fingerprint(p) for p in stage.iterdir() if p.is_file()}}
report_path.write_text(json.dumps(report, indent=2)+'\n')
print('Less keyboard ownership: 18 contracts passed with clean Valgrind logs')
