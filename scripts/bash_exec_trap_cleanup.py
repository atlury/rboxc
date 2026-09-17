"""Restore trap ownership flags before replacing strings after failed exec."""
# SPDX-License-Identifier: GPL-3.0-or-later
import json
from pathlib import Path
import subprocess
from entry_provider_helpers import fingerprint

def adapt(text):
 old='''      else if (trapstr != (char *)DEFAULT_SIG)
        /* set_signal duplicates the string argument before freeing it. */
\tset_signal (i, trapstr);'''
 new='''      else if (trapstr != (char *)DEFAULT_SIG)
        {
          /* reset_signal_handlers kept the string but cleared SIG_TRAPPED.
             Restore ownership before set_signal duplicates and replaces it.
             SIG_INPROGRESS still protects a command borrowed by a handler. */
          reinit_trap (i);
          set_signal (i, trapstr);
        }'''
 assert text.count(old)==1
 return text.replace(old,new)

def prepare(root,adapted):
 previous=json.loads((root/'evidence/bash-sigchld-cleanup.json').read_text())
 original=Path(previous['source']);assert fingerprint(original)==previous['source_sha256']
 stage=root/'build/bash-exec-trap-cleanup';stage.mkdir(exist_ok=True)
 source=stage/'trap.c';source.write_text(adapt(original.read_text()));obj=stage/'trap.o'
 command=previous['compile_command'].copy();command[command.index(str(original))]=str(source);command[command.index('-o')+1]=str(obj)
 log=stage/'build.log'
 with log.open('w') as out:subprocess.run(command,cwd=root/'build/gnu-bash',stdout=out,stderr=subprocess.STDOUT,check=True)
 assert adapted['trap.o']==Path(previous['object']);adapted['trap.o']=obj
 (root/'evidence/bash-exec-trap-cleanup.json').write_text(json.dumps({'scope':'Reinitialize ownership flags for retained ordinary-signal trap strings before restoring their handlers after failed exec. Keep GNU signal decisions and in-progress borrow protection.',
 'driver_sha256':fingerprint(Path(__file__)),'original':str(original),'original_sha256':fingerprint(original),'source':str(source),'source_sha256':fingerprint(source),'object':str(obj),'object_sha256':fingerprint(obj),'compile_command':command,'log':str(log),'log_sha256':fingerprint(log)},indent=2)+'\n')
