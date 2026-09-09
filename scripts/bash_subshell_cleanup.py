"""Finalize abandoned pipeline backups, job entries, and errexit heap owners."""
# SPDX-License-Identifier: GPL-3.0-or-later
import json,os,shutil,subprocess
from pathlib import Path
from entry_provider_helpers import fingerprint


def prepare(root,adapted):
    baseline_path=root/'evidence/bash-native-cleanup.json';baseline=json.loads(baseline_path.read_text())
    pin=json.loads((root/'inventory/sources.json').read_text())['bash'];gnu=Path(pin['source'])
    stage=root/'build/bash-subshell-cleanup';stage.mkdir(exist_ok=True)
    reports=[]
    for name in ('execute_cmd.c','unwind_prot.c','builtins/evalstring.c','jobs.c'):
        if name=='jobs.c':
            original=gnu/name;assert fingerprint(original)==pin['source_and_header_sha256'][name]
            records=[json.loads(p.read_text()) for p in (root/'build/bash-cc-records').glob('*.json')]
            matches=[r for r in records if r.get('file')==str(original)]
            if not matches:
                with (stage/'jobs.record.log').open('w') as log:
                    subprocess.run(['make','-C',str(root/'build/gnu-bash'),'-W',str(original),'jobs.o','CC=python3 '+str(root/'scripts/record-provider-cc.py')],env={**os.environ,'RBOXC_CC_RECORDS':str(root/'build/bash-cc-records')},stdout=log,stderr=subprocess.STDOUT,check=True)
                matches=[json.loads(p.read_text()) for p in (root/'build/bash-cc-records').glob('*.json') if json.loads(p.read_text()).get('file')==str(original)]
            record,=matches;command=record['arguments'].copy();directory=record['directory']
            command+=['-iquote',str(original.parent),'-iquote',str(gnu)]
        else:
            original=root/'build/bash-cleanup'/name
            record,=[r for r in baseline['files'] if r['source']==str(gnu/name)]
            assert fingerprint(original)==record['adapted_sha256']
            command=record['compile_arguments'].copy();directory=str(root/'build/gnu-bash'/Path(name).parent)
        text=original.read_text()
        def replace(old,new):
            nonlocal text
            assert text.count(old)==1,(name,old)
            text=text.replace(old,new)
        if name=='execute_cmd.c':
            replace('#include "shell.h"','#include "shell.h"\nextern void rboxc_bash_track_backup (int);')
            old='      lstdin = (prev > 0 && stdin_valid) ? move_to_high_fd (0, 1, -1) : -1;'
            replace(old,old+'\n      if (lstdin > 0) rboxc_bash_track_backup (lstdin);')
        elif name=='jobs.c':
            start=text.index('delete_all_jobs (int running_only)\n{');end=text.index('\n/*',start)
            body=text[start:end]
            old='  register int i;';assert body.count(old)==1
            body=body.replace(old,old+'\n  int old_frozen = jobs_list_frozen;')
            old='  BLOCK_CHILD (set, oset);';assert body.count(old)==1
            body=body.replace(old,old+'''
  /* A full discard frees the table itself. Its entries must be released
     even when a lastpipe execution froze ordinary job-list updates. */
  if (running_only == 0) jobs_list_frozen = 0;''')
            old='  UNBLOCK_CHILD (oset);';assert body.count(old)==1
            body=body.replace(old,'  jobs_list_frozen = old_frozen;\n'+old)
            text=text[:start]+body+text[end:]
        elif name=='unwind_prot.c':
            anchor='/* Restore the value of a variable, based on the contents of SV.'
            replace(anchor,'''/* Discard an abandoned error frame while releasing known heap payloads.
   Its C stack has already been unwound. Never invoke saved stack restorers
   or callbacks whose arguments are live stack slots from exec restart. */
void
discard_unwind_frame_heap (char *tag)
{
  UNWIND_ELT *elt;
  int found = 0;
  while ((elt = unwind_protect_list))
    {
      unwind_protect_list = elt->head.next;
      if (elt->head.cleanup == 0 && STREQ (elt->arg.v, tag))
        {
          uwpfree (elt);
          found = 1;
          break;
        }
      if (elt->head.discard == xfree)
        xfree (elt->arg.v);
      else if (elt->head.cleanup == uw_dispose_command ||
               elt->head.cleanup == uw_dispose_words ||
               elt->head.cleanup == uw_dispose_fd_bitmap ||
               elt->head.cleanup == xfree)
        elt->head.cleanup (elt->arg.v);
      uwpfree (elt);
    }
  if (found == 0)
    internal_warning ("unwind_frame_discard: %s: %s", tag, _("frame not found"));
}

'''+anchor)
        else:
            replace('#include "../shell.h"','#include "../shell.h"\nextern void discard_unwind_frame_heap (char *);')
            old='\t          discard_unwind_frame ("pe_dispose");\n\t          reset_local_contexts ();'
            replace(old,old.replace('discard_unwind_frame (','discard_unwind_frame_heap ('))
        source=stage/name;source.parent.mkdir(parents=True,exist_ok=True);source.write_text(text);obj=source.with_suffix('.o')
        command[command.index(str(original))]=str(source)
        if '-o' in command:command[command.index('-o')+1]=str(obj)
        else:command+=['-o',str(obj)]
        log=source.with_suffix('.c.log')
        with log.open('w') as stream:subprocess.run(command,cwd=directory,stdout=stream,stderr=subprocess.STDOUT,check=True)
        if name=='builtins/evalstring.c':
            archive=stage/'libbuiltins.a';shutil.copyfile(adapted[archive.name],archive)
            subprocess.run(['ar','r',str(archive),str(obj)],check=True);subprocess.run(['ranlib',str(archive)],check=True);adapted[archive.name]=archive
        else:adapted[obj.name]=obj
        reports.append({'original':str(original),'original_sha256':fingerprint(original),'source':str(source),'source_sha256':fingerprint(source),'object':str(obj),'object_sha256':fingerprint(obj),'compile_arguments':command,'directory':directory,'log':str(log),'log_sha256':fingerprint(log)})
    (root/'evidence/bash-subshell-cleanup.json').write_text(json.dumps({'scope':'Track lastpipe stdin backups through child exit; release entries before a full job-table discard even when ordinary job updates are frozen; dispose known heap-only unwind payloads in the existing errexit frame discard without restoring abandoned stack state. Signal dispositions and GNU assertion behavior remain separate validation requirements.',
        'driver_sha256':fingerprint(Path(__file__)),'baseline_profile_sha256':fingerprint(baseline_path),'files':reports},indent=2)+'\n')
