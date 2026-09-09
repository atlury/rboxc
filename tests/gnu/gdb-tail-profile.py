#!/usr/bin/env python3
"""Map two owned-file GNU tail timing tests to explicit source breakpoints."""
# SPDX-License-Identifier: GPL-3.0-or-later
import hashlib
import json
import os
from pathlib import Path
import re
import sys

config = json.loads(Path(sys.argv[1]).read_text())
args = sys.argv[2:]
run = Path(config['run']).resolve(strict=True)
assert Path.cwd().is_relative_to(run)
assert config['script'] in ('tests/tail/inotify-race.sh', 'tests/tail/inotify-race2.sh')
gdb = Path('/usr/bin/gdb')
assert hashlib.sha256(gdb.read_bytes()).hexdigest() == config['gdb_sha256']
if args != ['--version']:
    assert args[:2] == ['-nx', '--batch-silent'] and args[-1] == 'tail'
    commands = [a.removeprefix('--eval-command=') for a in args[2:-1]]
    assert all(a.startswith('--eval-command=') for a in args[2:-1])
    assert commands[0] == 'break '+str(config['gnu_line'])
    assert commands[-1] == 'quit'
    match = re.fullmatch(r'run --pid=(\d+) (-f file|-F file 2>tail.err)( >> tail.out| >>tail.out)?', commands[1])
    assert match, commands[1]
    sleeper = int(match[1])
    assert Path(f'/proc/{sleeper}/cwd').resolve(strict=True) == Path.cwd()
    assert len(commands) in (3, 5)
    if len(commands) == 5:
        assert commands[3] == 'continue'
        assert commands[2] == ('shell echo never-seen-with-tail-7.5 >> file'
                               if config['script'].endswith('/inotify-race.sh')
                               else 'shell mv file.new file')
    source = Path(config['break_source'])
    assert hashlib.sha256(source.read_bytes()).hexdigest() == config['break_source_sha256']
    mapped = '--eval-command=break '+str(source)+':'+str(config['break_line'])
    args = [args[0], args[1], mapped, *args[3:]]
    # Keep host ASLR enabled; no auto-load scripts or external debuginfod queries.
    args = ['-iex=set disable-randomization off', '-iex=set auto-load off',
            '-iex=set debuginfod enabled off', '-iex=set confirm off', *args]
record = {'original_arguments': sys.argv[2:], 'effective_arguments': args,
          'fixture': str(Path.cwd()), 'implementation': config['implementation'],
          'break_source': config['break_source'], 'break_line': config['break_line']}
with (run/'gdb-tail-profile.jsonl').open('a') as log:
    log.write(json.dumps(record)+'\n')
os.execv(gdb, [str(gdb), *args])
