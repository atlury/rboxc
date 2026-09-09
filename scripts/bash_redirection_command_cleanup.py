"""Keep child command strings owned while expanding external redirections."""
# SPDX-License-Identifier: GPL-3.0-or-later
import json,subprocess
from pathlib import Path
from entry_provider_helpers import fingerprint

def prepare(root,adapted):
    profile=json.loads((root/'evidence/bash-arithmetic-cleanup.json').read_text())
    record,=[r for r in profile['files'] if Path(r['source']).name=='execute_cmd.c']
    original=Path(record['source']);assert fingerprint(original)==record['source_sha256']
    text=original.read_text()
    anchor='      if (redirects && (do_redirections (redirects, RX_ACTIVE) != 0))'
    assert text.count(anchor)==1
    text=text.replace(anchor,'''      begin_unwind_frame ("rboxc-external-redirection-strings");
      add_unwind_protect (xfree, command_line);
      add_unwind_protect (xfree, command);
'''+anchor)
    anchor='''#if defined (PROCESS_SUBSTITUTION) && !defined (HAVE_DEV_FD)
      /* This should only contain FIFOs created as part of redirection'''
    assert text.count(anchor)==1
    text=text.replace(anchor,'''      discard_unwind_frame ("rboxc-external-redirection-strings");

'''+anchor)
    stage=root/'build/bash-redirection-command-cleanup';stage.mkdir(exist_ok=True)
    source=stage/'execute_cmd.c';source.write_text(text)
    obj=source.with_suffix('.o');log=source.with_suffix('.c.log')
    command=record['compile_arguments'].copy()
    command[command.index(str(original))]=str(source)
    command[command.index('-o')+1]=str(obj)
    with log.open('w') as out:subprocess.run(command,cwd=record['directory'],stdout=out,stderr=subprocess.STDOUT,check=True)
    adapted[obj.name]=obj
    (root/'evidence/bash-redirection-command-cleanup.json').write_text(json.dumps({
        'scope':'In the external-command child, the allocated command display string and resolved path retain heap unwind owners through redirection expansion. Normal expansion discards the registrations and retains GNU exec/free behavior. Diagnostic exits release the abandoned strings.',
        'driver_sha256':fingerprint(Path(__file__)),
        'files':[{'original':str(original),'original_sha256':fingerprint(original),
            'source':str(source),'source_sha256':fingerprint(source),'object':str(obj),'object_sha256':fingerprint(obj),
            'compile_arguments':command,'directory':record['directory'],'log':str(log),'log_sha256':fingerprint(log)}]},indent=2)+'\n')
