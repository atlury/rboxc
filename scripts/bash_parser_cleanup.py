"""Release parser tokens and string expansion owners on ordinary syntax errors."""
# SPDX-License-Identifier: GPL-3.0-or-later
import json
from pathlib import Path
import subprocess
from entry_provider_helpers import fingerprint


def prepare(root, adapted):
    baseline_path=root/'evidence/bash-native-cleanup.json'
    baseline=json.loads(baseline_path.read_text())
    stage=root/'build/bash-parser-cleanup';stage.mkdir(exist_ok=True)
    reports=[]
    for name in ('y.tab.c','subst.c'):
        record,=[r for r in baseline['files'] if Path(r['source']).name==name]
        original=root/'build/bash-cleanup'/name
        assert fingerprint(original)==record['adapted_sha256']
        text=original.read_text()
        def replace(old,new):
            nonlocal text
            assert text.count(old)==1,(name,old)
            text=text.replace(old,new)
        if name=='y.tab.c':
            # Bison destroys discarded symbols, not values transferred by a
            # successful reduction. These tokens own the lexer WORD_DESC.
            replace('  switch (yykind) {', '''  switch (yykind) {
    case YYSYMBOL_WORD:
    case YYSYMBOL_ASSIGNMENT_WORD:
    case YYSYMBOL_REDIR_WORD:
      dispose_word (yyvaluep->word);
      break;''')
            start=text.index('parse_compound_assignment (size_t *retlenp)\n{')
            end=text.index('\n/************************************************',start)
            body=text[start:end]
            old='  if (wl == &parse_string_error)\n    {'
            assert body.count(old)==1
            body=body.replace(old,old+'''
      /* The error jump abandons this saved state; do not restore globals. */
      flush_parser_state (&ps);''')
            text=text[:start]+body+text[end:]
        else:
            start=text.index('expand_string_internal (const char *string, int quoted)\n{')
            end=text.index('\n/*',start)
            body=text[start:end]
            assert body.count('  WORD_DESC td;')==1
            body=body.replace('  WORD_DESC td;','  WORD_DESC *td;')
            old='  td.flags = 0;\n  td.word = savestring (string);'
            assert body.count(old)==1
            body=body.replace(old,'''  /* A heap descriptor survives a nonlocal expansion error. GNU's
     call_expand_word_internal clears its word when that word was freed. */
  td = alloc_word_desc ();
  td->word = savestring (string);
  begin_unwind_frame ("rboxc-string-expansion");
  add_unwind_protect_owned (rboxc_dispose_expansion_word,
                            rboxc_dispose_expansion_word, td);''')
            assert body.count('call_expand_word_internal (&td,')==1
            body=body.replace('call_expand_word_internal (&td,','call_expand_word_internal (td,')
            old='  FREE (td.word);'
            assert body.count(old)==1
            body=body.replace(old,'''  discard_unwind_frame ("rboxc-string-expansion");
  dispose_word (td);''')
            text=text[:start]+body+text[end:]
        source=stage/name;source.write_text(text)
        object_name=name[:-2]+'.o';obj=stage/object_name
        command=list(record['compile_arguments'])
        command[command.index(str(original))]=str(source)
        command[command.index('-o')+1]=str(obj)
        log=stage/(name+'.log')
        with log.open('w') as output:
            subprocess.run(command,cwd=root/'build/gnu-bash',stdout=output,stderr=subprocess.STDOUT,check=True)
        assert fingerprint(adapted[object_name])==record['object_sha256']
        adapted[object_name]=obj
        reports.append({'original':str(original),'original_sha256':record['adapted_sha256'],
            'source':str(source),'source_sha256':fingerprint(source),
            'object':str(obj),'object_sha256':fingerprint(obj),
            'compile_command':command,'build_log':str(log),'build_log_sha256':fingerprint(log)})
    (root/'evidence/bash-parser-cleanup.json').write_text(json.dumps({
        'scope':'Discard lexer-owned word tokens through Bison destructors; release a saved compound-assignment parser state before its existing error jump; retain a heap-owned WORD_DESC across string expansion so nonlocal errors can dispose its current word. Normal reductions and successful expansion results retain GNU behavior.',
        'driver_sha256':fingerprint(Path(__file__)),
        'baseline_profile_sha256':fingerprint(baseline_path),'files':reports,
    },indent=2)+'\n')
