"""Retain arithmetic lookahead and compound assignment ownership on errors."""
# SPDX-License-Identifier: GPL-3.0-or-later
import json,os,subprocess
from pathlib import Path
from entry_provider_helpers import fingerprint


def prepare(root,adapted):
    baseline=json.loads((root/'evidence/bash-native-cleanup.json').read_text())
    parser=json.loads((root/'evidence/bash-parser-cleanup.json').read_text())
    pin=json.loads((root/'inventory/sources.json').read_text())['bash'];gnu=Path(pin['source'])
    stage=root/'build/bash-assignment-cleanup';stage.mkdir(exist_ok=True);reports=[]
    for name in ('expr.c','subst.c','arrayfunc.c'):
        if name=='arrayfunc.c':
            original=gnu/name;assert fingerprint(original)==pin['source_and_header_sha256'][name]
            records=[json.loads(p.read_text()) for p in (root/'build/bash-cc-records').glob('*.json')];matches=[r for r in records if r.get('file')==str(original)]
            if not matches:
                with (stage/'arrayfunc.record.log').open('w') as log:
                    subprocess.run(['make','-C',str(root/'build/gnu-bash'),'-W',str(original),'arrayfunc.o','CC=python3 '+str(root/'scripts/record-provider-cc.py')],env={**os.environ,'RBOXC_CC_RECORDS':str(root/'build/bash-cc-records')},stdout=log,stderr=subprocess.STDOUT,check=True)
                matches=[json.loads(p.read_text()) for p in (root/'build/bash-cc-records').glob('*.json') if json.loads(p.read_text()).get('file')==str(original)]
            record,=matches;command=record['arguments'].copy();directory=record['directory'];command+=['-iquote',str(gnu)]
        elif name=='subst.c':
            record,=[r for r in parser['files'] if Path(r['source']).name==name];original=Path(record['source']);assert fingerprint(original)==record['source_sha256'];command=record['compile_command'].copy();directory=str(root/'build/gnu-bash')
        else:
            record,=[r for r in baseline['files'] if Path(r['source']).name==name];original=root/'build/bash-cleanup'/name;assert fingerprint(original)==record['adapted_sha256'];command=record['compile_arguments'].copy();directory=str(root/'build/gnu-bash')
        text=original.read_text()
        def replace(old,new):
            nonlocal text
            assert text.count(old)==1,(name,old);text=text.replace(old,new)
        if name=='expr.c':
            anchor='static void rboxc_expr_release_to (struct rboxc_expr_string *mark)'
            replace(anchor,'''/* Transfer a lookahead token back to the normal expression owner. */
static void rboxc_expr_forget (char *value)
{
  struct rboxc_expr_string **slot, *item;
  for (slot = &rboxc_expr_strings; (item = *slot); slot = &item->next)
    if (item->value == value) {
      *slot = item->next; free (item); return;
    }
  abort ();
}
'''+anchor)
            start=text.index('readtok (void)\n{');body=text[start:]
            old='      SAVETOK (&ec);\n      tokstr = (char *)NULL;';assert body.count(old)==1
            body=body.replace(old,'      SAVETOK (&ec);\n      rboxc_expr_hold (ec.tokstr);\n      tokstr = (char *)NULL;')
            old='      RESTORETOK (&ec);\n      cp = savecp;';assert body.count(old)==1
            body=body.replace(old,'      rboxc_expr_forget (ec.tokstr);\n'+old);text=text[:start]+body
        elif name=='subst.c':
            start=text.index('do_assignment_internal (const WORD_DESC *word, int expand)\n{');end=text.index('\nint\n',start);body=text[start:end]
            old='  if (echo_command_at_execute)';assert body.count(old)==1
            body=body.replace(old,'''  begin_unwind_frame ("rboxc-assignment-value");
  add_unwind_protect (xfree, value);

'''+old)
            old='  FREE (value); \\\n';assert body.count(old)==1
            body=body.replace(old,'  discard_unwind_frame ("rboxc-assignment-value"); \\\n'+old);text=text[:start]+body+text[end:]
            start=text.index('do_compound_assignment (');start=text.index('\n{',start);end=text.index('\nstatic int\ndo_assignment_internal',start);body=text[start:end]
            old='      list = expand_compound_array_assignment (v, value, flags);';assert body.count(old)==2
            body=body.replace(old,old+'\n      begin_unwind_frame ("rboxc-compound-list");\n      add_unwind_protect (uw_dispose_words, list);')
            old='      if (list)\n\tdispose_words (list);';assert body.count(old)==2
            body=body.replace(old,'      discard_unwind_frame ("rboxc-compound-list");\n'+old);text=text[:start]+body+text[end:]
        else:
            replace('#include "shell.h"','#include "shell.h"\n#include "unwind_prot.h"')
            start=text.index('assign_array_var_from_string (SHELL_VAR *var, char *value, int flags)\n{');end=text.index('\n/*',start);body=text[start:end]
            old='  nlist = expand_compound_array_assignment (var, value, flags);';assert body.count(old)==1
            body=body.replace(old,old+'\n  begin_unwind_frame ("rboxc-array-string-list");\n  add_unwind_protect (uw_dispose_words, nlist);')
            old='  if (nlist)\n    dispose_words (nlist);';assert body.count(old)==1
            body=body.replace(old,'  discard_unwind_frame ("rboxc-array-string-list");\n'+old);text=text[:start]+body+text[end:]
        source=stage/name;source.write_text(text);obj=source.with_suffix('.o');command[command.index(str(original))]=str(source)
        if '-o' in command:command[command.index('-o')+1]=str(obj)
        else:command+=['-o',str(obj)]
        log=stage/(name+'.log')
        with log.open('w') as out:subprocess.run(command,cwd=directory,stdout=out,stderr=subprocess.STDOUT,check=True)
        adapted[obj.name]=obj;reports.append({'original':str(original),'original_sha256':fingerprint(original),'source':str(source),'source_sha256':fingerprint(source),'object':str(obj),'object_sha256':fingerprint(obj),'compile_arguments':command,'directory':directory,'log':str(log),'log_sha256':fingerprint(log)})
    (root/'evidence/bash-assignment-cleanup.json').write_text(json.dumps({'scope':'A saved arithmetic lookahead token stays owned during recursive lexing and transfers back on success. Assignment values and expanded compound-array lists register their existing destructors until normal disposal, so ordinary index errors can unwind them. Assignment evaluation, diagnostics and resulting variable state remain GNU behavior.',
        'driver_sha256':fingerprint(Path(__file__)),'files':reports},indent=2)+'\n')
