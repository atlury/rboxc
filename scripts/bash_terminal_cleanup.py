"""Retain exit ownership of the terminal allocated by interactive Bash."""
# SPDX-License-Identifier: GPL-3.0-or-later
import json, subprocess
from pathlib import Path
from entry_provider_helpers import fingerprint


def prepare(root, adapted):
    profile=json.loads((root/'evidence/bash-subshell-cleanup.json').read_text())
    record,=[r for r in profile['files'] if Path(r['source']).name=='jobs.c']
    original=Path(record['source'])
    assert fingerprint(original)==record['source_sha256']
    text=original.read_text()
    anchor='#include "shell.h"'
    assert text.count(anchor)==1
    text=text.replace(anchor,anchor+'\nextern void rboxc_bash_track_backup (int);')
    anchor='      if (shell_tty != -1)\n\tshell_tty = move_to_high_fd (shell_tty, 1, -1);'
    assert text.count(anchor)==1
    text=text.replace(anchor,anchor+'''
      /* Only this branch allocates a terminal; the noninteractive branch
         borrows stderr. Normal close forgets this exit owner. */
      if (shell_tty != -1)
        rboxc_bash_track_backup (shell_tty);''')
    stage=root/'build/bash-terminal-cleanup';stage.mkdir(exist_ok=True)
    source=stage/'jobs.c';source.write_text(text)
    obj=source.with_suffix('.o');log=source.with_suffix('.c.log')
    command=record['compile_arguments'].copy()
    command[command.index(str(original))]=str(source)
    command[command.index('-o')+1]=str(obj)
    with log.open('w') as out:
        subprocess.run(command,cwd=record['directory'],stdout=out,stderr=subprocess.STDOUT,check=True)
    adapted[obj.name]=obj
    (root/'evidence/bash-terminal-cleanup.json').write_text(json.dumps({
        'scope':'Interactive job initialization retains exit ownership of its allocated terminal descriptor. Borrowed stderr, terminal process groups, signal policy and ordinary close behavior are unchanged.',
        'driver_sha256':fingerprint(Path(__file__)),
        'files':[{'original':str(original),'original_sha256':fingerprint(original),
            'source':str(source),'source_sha256':fingerprint(source),
            'object':str(obj),'object_sha256':fingerprint(obj),'compile_arguments':command,
            'directory':record['directory'],'log':str(log),'log_sha256':fingerprint(log)}]},indent=2)+'\n')
