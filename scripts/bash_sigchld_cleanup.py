"""Release copied SIGCHLD text when installing the temporary sentinel."""
# SPDX-License-Identifier: GPL-3.0-or-later
import json,subprocess
from pathlib import Path
from entry_provider_helpers import fingerprint

def prepare(root,adapted):
    previous=json.loads((root/'evidence/bash-trap-restart-cleanup.json').read_text())
    original=Path(previous['source'])
    assert fingerprint(original)==previous['source_sha256']
    text=original.read_text()
    old='''set_impossible_sigchld_trap (void)
{
  restore_default_signal (SIGCHLD);
  change_signal (SIGCHLD, (char *)IMPOSSIBLE_TRAP_HANDLER);
  sigmodes[SIGCHLD] &= ~SIG_TRAPPED;\t/* uw_maybe_set_sigchld_trap checks this */
}'''
    new='''set_impossible_sigchld_trap (void)
{
  char *rboxc_copied_command = 0;
  /* run_sigchld_trap already copied the command for execution and restore.
     The pending-signal dispatcher sets SIG_INPROGRESS, which otherwise
     prevents change_signal from releasing this now-unborrowed slot. */
  if ((sigmodes[SIGCHLD] & (SIG_INPROGRESS|SIG_TRAPPED)) ==
      (SIG_INPROGRESS|SIG_TRAPPED) && trap_list[SIGCHLD] &&
      trap_list[SIGCHLD] != (char *)IGNORE_SIG &&
      trap_list[SIGCHLD] != (char *)DEFAULT_SIG &&
      trap_list[SIGCHLD] != (char *)IMPOSSIBLE_TRAP_HANDLER)
    rboxc_copied_command = trap_list[SIGCHLD];
  restore_default_signal (SIGCHLD);
  change_signal (SIGCHLD, (char *)IMPOSSIBLE_TRAP_HANDLER);
  sigmodes[SIGCHLD] &= ~SIG_TRAPPED;\t/* uw_maybe_set_sigchld_trap checks this */
  if (rboxc_copied_command)
    rboxc_release_trap_string (rboxc_copied_command);
}'''
    assert text.count(old)==1
    text=text.replace(old,new)
    stage=root/'build/bash-sigchld-cleanup';stage.mkdir(exist_ok=True)
    source=stage/'trap.c';source.write_text(text)
    obj=stage/'trap.o';log=stage/'build.log'
    command=previous['compile_command'].copy()
    command[command.index(str(original))]=str(source)
    command[command.index('-o')+1]=str(obj)
    with log.open('w') as out:
        subprocess.run(command,cwd=root/'build/gnu-bash',stdout=out,stderr=subprocess.STDOUT,check=True)
    assert adapted['trap.o']==Path(previous['object'])
    adapted['trap.o']=obj
    (root/'evidence/bash-sigchld-cleanup.json').write_text(json.dumps({
        'scope':'The SIGCHLD dispatcher owns an independent execution/restore copy before installing its sentinel. Release the replaced slot when SIG_INPROGRESS would otherwise abandon it. Original signal flags, sentinel decisions, trap execution and unwind restoration remain in order.',
        'driver_sha256':fingerprint(Path(__file__)),'previous_source_sha256':fingerprint(original),
        'source':str(source),'source_sha256':fingerprint(source),'object':str(obj),'object_sha256':fingerprint(obj),
        'compile_command':command,'build_log':str(log),'build_log_sha256':fingerprint(log)},indent=2)+'\n')
