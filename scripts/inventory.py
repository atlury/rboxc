#!/usr/bin/env python3
"""Preserve actual command names and distinguish GNU candidates from confirmed pins."""
import hashlib
import json
from pathlib import Path
import re
import subprocess
from collections import Counter

ROOT = Path(__file__).resolve().parents[1]
ORIGINAL = Path('/root/rbox')
listing = subprocess.check_output([ORIGINAL/'target/release/rbox', '--list'], text=True)
names = sorted(line for line in listing.splitlines() if line and not line.startswith('Currently defined'))
assert len(names) == len(set(names))
declared = int(re.search(r'APPLET_COUNT: usize = (\d+)', (ORIGINAL/'src/applets/mod.rs').read_text())[1])
source = Path('/opt/src/coreutils-9.11')
core = set(re.findall(r'\+= src/(\S+)', (source/'src/cu-progs.mk').read_text())) - {'libstdbuf.so'}
core.discard('ginstall')
core.add('install')
providers = {name: 'coreutils' for name in core}
candidates = {
    'grep': ['grep', 'egrep', 'fgrep'],
    'diffutils': ['diff', 'cmp', 'diff3', 'sdiff'],
    'findutils': ['find', 'xargs', 'locate', 'updatedb'],
    'gawk': ['awk', 'gawk', 'nawk'], 'sed': ['sed'], 'tar': ['tar'],
    'gzip': ['gzip', 'gunzip', 'zcat', 'uncompress'], 'bc': ['bc', 'dc'],
    'ed': ['ed'], 'cpio': ['cpio', 'mt'], 'patch': ['patch'],
    'binutils': ['ar', 'readelf', 'strings'], 'glibc': ['getconf', 'iconv'],
    'inetutils': ['dnsdomainname', 'ftpd', 'inetd', 'syslogd', 'telnetd',
                 'ifconfig', 'logger', 'ping', 'ping6', 'telnet', 'tftp', 'tftpd', 'traceroute'],
    'sharutils': ['uudecode', 'uuencode'], 'wget': ['wget'], 'time': ['time'],
    'which': ['which'], 'screen': ['screen'], 'less': ['less'], 'hello': ['hello'],
    'bash': ['bash', 'sh', '-bash', '-sh', '.', ':', '[', '[[', 'alias', 'break',
             'continue', 'declare', 'eval', 'exec', 'exit', 'export', 'jobs', 'local',
             'return', 'set', 'shift', 'source', 'trap', 'unalias', 'unset', 'wait',
             'cd', 'help', 'ulimit'],
}
for provider, commands in candidates.items():
    for name in commands:
        providers.setdefault(name, provider)
selinux = {'getenforce', 'setenforce', 'selinuxenabled', 'sestatus', 'getsebool',
           'setsebool', 'chcon', 'matchpathcon', 'restorecon', 'setfiles', 'runcon', 'load_policy'}
rows = []
for name in names:
    provider = providers.get(name)
    state = 'queued' if provider else 'deferred-provider-review'
    if name in selinux:
        state = 'excluded-selinux'
    rows.append({'name': name, 'gnu_provider': provider,
                 'provider_confirmed': provider == 'coreutils', 'state': state,
                 'translated': False, 'compiles': False, 'gnu_tests_pass': False,
                 'valgrind_pass': False, 'complete': False})
(ROOT/'inventory/applets.json').write_text(json.dumps(rows, indent=2) + '\n')
pins = {
    'inventory_source': {
        'path': str(ORIGINAL/'target/release/rbox'),
        'git_head': subprocess.check_output(['git', '-C', str(ORIGINAL), 'rev-parse', 'HEAD'], text=True).strip(),
        'binary_sha256': hashlib.sha256((ORIGINAL/'target/release/rbox').read_bytes()).hexdigest(),
        'declared_count': declared, 'observed_count': len(names),
        'unresolved_count_discrepancy': declared - len(names),
    },
    'coreutils': {'version': '9.11', 'source': str(source),
                  'archive_sha256': hashlib.sha256(Path('/root/upstream-work/coreutils-9.11.tar.xz').read_bytes()).hexdigest()},
    'c2rust': {'repository': 'https://github.com/immunant/c2rust',
              'revision': 'e1e5bf257863107c54e9f42b345c2aeccd925458'},
}
(ROOT/'inventory/sources.json').write_text(json.dumps(pins, indent=2) + '\n')
print(json.dumps({'declared': declared, 'observed': len(rows),
                  'providers': dict(sorted(Counter(row['gnu_provider'] or 'unassigned' for row in rows).items()))}, indent=2))
