"""Retain ownership of a script descriptor before startup validation."""
# SPDX-License-Identifier: GPL-3.0-or-later
import json,subprocess
from pathlib import Path
from entry_provider_helpers import fingerprint

def prepare(root,adapted):
    profile=json.loads((root/'evidence/bash-native-cleanup.json').read_text())
    record,=[r for r in profile['files'] if Path(r['source']).name=='native-helpers.c']
    original=root/'build/bash-cleanup/native-helpers.c'
    assert fingerprint(original)==record['adapted_sha256']
    text=original.read_text();anchor='#include "shell.h"'
    assert text.count(anchor)==1
    text=text.replace(anchor,anchor+'\nextern void rboxc_bash_track_backup (int);')
    start=text.index('open_shell_script (char *script_name)\n{')
    end=text.index('\n}\n',start)+3
    segment=text[start:end];anchor='  free (dollar_vars[0]);'
    assert segment.count(anchor)==1
    segment=segment.replace(anchor,'''  /* Directory, binary-file and read diagnostics can exit before the
     normal buffered-input owner is installed. */
  rboxc_bash_track_backup (fd);

'''+anchor)
    text=text[:start]+segment+text[end:]
    stage=root/'build/bash-script-input-cleanup';stage.mkdir(exist_ok=True)
    source=stage/'native-helpers.c';source.write_text(text)
    obj=source.with_suffix('.o');log=source.with_suffix('.c.log')
    command=record['compile_arguments'].copy()
    command[command.index(str(original))]=str(source)
    command[command.index('-o')+1]=str(obj)
    directory=json.loads((root/'evidence/bash-split-entry.json').read_text())['directory']
    with log.open('w') as out:subprocess.run(command,cwd=directory,stdout=out,stderr=subprocess.STDOUT,check=True)
    adapted[obj.name]=obj
    (root/'evidence/bash-script-input-cleanup.json').write_text(json.dumps({
        'scope':'Successfully opened script descriptors retain exit ownership before directory, binary-file and read validation. Normal moves and closes use the existing ownership registry; original startup diagnostics and exit decisions remain unchanged.',
        'driver_sha256':fingerprint(Path(__file__)),
        'files':[{'original':str(original),'original_sha256':fingerprint(original),
            'source':str(source),'source_sha256':fingerprint(source),'object':str(obj),'object_sha256':fingerprint(obj),
            'compile_arguments':command,'directory':directory,'log':str(log),'log_sha256':fingerprint(log)}]},indent=2)+'\n')
