"""Release the pathname-expansion work lists before a failglob nonlocal exit."""
# SPDX-License-Identifier: GPL-3.0-or-later
import json
from pathlib import Path
import subprocess
from entry_provider_helpers import fingerprint

def adapt(text):
 old='\t      report_error (_("no match: %s"), tlist->word->word);\n\t      exp_jump_to_top_level (DISCARD);'
 assert text.count(old)==1
 return text.replace(old,'''\t      report_error (_("no match: %s"), tlist->word->word);
              /* The remaining input and the two processed lists are disjoint.
                 This path leaves the expansion frame without its normal frees. */
              strvec_dispose (glob_array);
              dispose_words (tlist);
              dispose_words (output_list);
              dispose_words (disposables);
\t      exp_jump_to_top_level (DISCARD);''')

def prepare(root,adapted):
 profile=json.loads((root/'evidence/bash-multibyte-cleanup.json').read_text())
 record,=[r for r in profile['files'] if Path(r['source']).name=='subst.c']
 original=Path(record['source']);assert fingerprint(original)==record['source_sha256']
 stage=root/'build/bash-failglob-cleanup';stage.mkdir(exist_ok=True)
 source=stage/'subst.c';source.write_text(adapt(original.read_text()));obj=source.with_suffix('.o')
 command=record['compile_arguments'].copy()
 command[command.index(str(original))]=str(source);command[command.index('-o')+1]=str(obj)
 log=stage/'subst.c.log'
 with log.open('w') as out:subprocess.run(command,cwd=record['directory'],stdout=out,stderr=subprocess.STDOUT,check=True)
 adapted[obj.name]=obj
 (root/'evidence/bash-failglob-cleanup.json').write_text(json.dumps({'scope':'Keep GNU failglob diagnostics and DISCARD behavior. Release the empty glob vector, remaining input words, completed output words and discarded original words before leaving the expansion frame.',
  'driver_sha256':fingerprint(Path(__file__)),'original':str(original),'original_sha256':fingerprint(original),
  'source':str(source),'source_sha256':fingerprint(source),'object':str(obj),'object_sha256':fingerprint(obj),
  'compile_arguments':command,'directory':record['directory'],'log':str(log),'log_sha256':fingerprint(log)},indent=2)+'\n')
