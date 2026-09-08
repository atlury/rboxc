"""Finalize owned Bash resources across normal exits and interpreter restart."""
# SPDX-License-Identifier: GPL-3.0-or-later
import json,os,re,shutil,subprocess
from pathlib import Path
from entry_provider_helpers import fingerprint

def prepare(root):
 pin=json.loads((root/'inventory/sources.json').read_text())['bash'];source=Path(pin['source'])
 stage=root/'build/bash-cleanup';stage.mkdir(exist_ok=True)
 records=[json.loads(p.read_text()) for p in (root/'build/bash-cc-records').glob('*.json')]
 outputs={};reports=[]
 def replace(text,old,new):
  assert text.count(old)==1,(old,text.count(old));return text.replace(old,new)
 for name in ['make_cmd.c','dispose_cmd.c','unwind_prot.c','variables.c','execute_cmd.c','subst.c','trap.c','expr.c','redir.c','eval.c','bashline.c','general.c','y.tab.c','builtins/evalstring.c','native-helpers.c']:
  original=(root/'build/translation/bash'/name) if name=='native-helpers.c' else source/name
  if name!='native-helpers.c':assert fingerprint(original)==pin['source_and_header_sha256'][name]
  text=original.read_text()
  if name=='make_cmd.c':
   text=replace(text,'      free (init);\n      free (test);\n      free (step);','      dispose_words (init);\n      dispose_words (test);\n      dispose_words (step);\n      dispose_words (exprs);\n      dispose_command (action);')
   for cache,ty,size in [('wdcache','WORD_DESC','WDCACHESIZE'),('wlcache','WORD_LIST','WLCACHESIZE')]:
    anchor=f'  ocache_create ({cache}, {ty}, {size});'
    text=replace(text,anchor,f'  ocache_flush ({cache}, {ty});\n  ocache_destroy ({cache});\n'+anchor)
  elif name=='dispose_cmd.c':
   text=replace(text,'extern sh_obj_cache_t wdcache, wlcache;', 'extern sh_obj_cache_t wdcache, wlcache;\nextern COMMAND *currently_executing_command;')
   text=replace(text,'dispose_command (COMMAND *command)\n{','dispose_command (COMMAND *command)\n{\n  if (command == currently_executing_command) currently_executing_command = (COMMAND *)NULL;')
  elif name=='unwind_prot.c':
   text=replace(text,'    sh_uwfunc_t *cleanup;','    sh_uwfunc_t *cleanup;\n    sh_uwfunc_t *discard;')
   text=replace(text,'  elt->head.cleanup = cleanup;','  elt->head.cleanup = cleanup;\n  elt->head.discard = 0;')
   text=replace(text,'  elt->head.cleanup = restore_variable;','  elt->head.cleanup = restore_variable;\n  elt->head.discard = 0;')
   anchor='/* Remove the top unwind protect from the list. */'
   text=replace(text,anchor,'''/* Preserve the normal callback while specifying ownership on exec restart. */
void
add_unwind_protect_owned (sh_uwfunc_t *cleanup, sh_uwfunc_t *discard, void *arg)
{
  add_unwind_protect_internal (cleanup, arg);
  unwind_protect_list->head.discard = discard;
}

'''+anchor)
   anchor='  ocache_create (uwcache, UNWIND_ELT, UWCACHESIZE);'
   text=replace(text,anchor,'  ocache_flush (uwcache, UNWIND_ELT);\n  ocache_destroy (uwcache);\n'+anchor)
   text=replace(text,'#include "unwind_prot.h"','#include "unwind_prot.h"\n#include "dispose_cmd.h"\nextern void uw_dispose_fd_bitmap (void *);')
   anchor='clear_unwind_protects_internal (int flag)\n{'
   text=replace(text,anchor,anchor+'''
  if (flag == 2) {
    UNWIND_ELT *elt;
    while ((elt = unwind_protect_list)) {
      unwind_protect_list = elt->head.next;
      /* Only owned heap destructors: never restore abandoned stack state,
         signal masks, directory state or file-descriptor state. */
      if (elt->head.discard)
        elt->head.discard (elt->arg.v);
      else if (elt->head.cleanup == uw_dispose_command ||
          elt->head.cleanup == uw_dispose_words ||
          elt->head.cleanup == uw_dispose_fd_bitmap ||
          elt->head.cleanup == xfree)
        elt->head.cleanup (elt->arg.v);
      uwpfree (elt);
    }
    return;
  }
''')

  elif name=='variables.c':
   # These dynamic associative snapshots own their keys and copied values.
   # Keep att_nofree during shell operations; destroy only at variable disposal.
   anchor='  if (nofree_p (var) == 0)\n    dispose_variable_value (var);'
   text=replace(text,anchor,'''#if defined (ARRAY_VARS)
  if (nofree_p (var) && assoc_p (var) &&
      (var->dynamic_value == get_hashcmd
#if defined (ALIAS)
       || var->dynamic_value == get_aliasvar
#endif
      )) {
    assoc_dispose (assoc_cell (var));
    var_setvalue (var, (char *)NULL);
  }
#endif
'''+anchor)
  elif name=='execute_cmd.c':
   text=replace(text,'#include "shell.h"','#include "shell.h"\nextern void add_unwind_protect_owned (sh_uwfunc_t *, sh_uwfunc_t *, void *);\nstatic void rboxc_keep_function_payload (void *p) { (void)p; }')
   for callback,arg in [('uw_maybe_restore_getopt_state','gs'),('uw_restore_funcarray_state','fa')]:
    text=replace(text,f'add_unwind_protect ({callback}, {arg});',f'add_unwind_protect_owned ({callback}, xfree, {arg});')
   text=replace(text,'  gs = sh_getopt_save_istate ();','''  gs = sh_getopt_save_istate ();
  if (subshell) {
    begin_unwind_frame ("rboxc-function-heap");
    add_unwind_protect (xfree, gs);
    add_unwind_protect (uw_dispose_command, tc);
  }''')
   anchor='    add_unwind_protect_owned (uw_restore_funcarray_state, xfree, fa);'
   text=replace(text,anchor,anchor+'''
  else
    add_unwind_protect_owned (rboxc_keep_function_payload, xfree, fa);''')
   anchor='  function_misc_cleanup ();\n'
   start=text.index('execute_function (SHELL_VAR *var, WORD_LIST *words, int flags, struct fd_bitmap *fds_to_close, int async, int subshell)\n{')
   end=text.index('\nstatic ',start)
   segment=text[start:end]
   segment=replace(segment,anchor,'  if (subshell) run_unwind_frame ("rboxc-function-heap");\n'+anchor)
   text=text[:start]+segment+text[end:]
   # Retain close's result/errno for already-closed pipes without issuing a
   # second close syscall. Valid descriptors still follow GNU's close path.
   anchor='static int\nexecute_pipeline (COMMAND *command, int asynchronous, int pipe_in, int pipe_out, struct fd_bitmap *fds_to_close)'
   text=replace(text,anchor,'''static int
rboxc_close_pipeline_fd (int fd)
{
  int saved = errno;
  if (fcntl (fd, F_GETFD) < 0 && errno == EBADF) return -1;
  errno = saved;
  return close (fd);
}

'''+anchor)
   start=text.index(anchor);end=text.index('\nstatic ',start+len(anchor))
   segment=text[start:end];assert segment.count('close (prev);')==2 and segment.count('close (fildes[1]);')==1
   segment=segment.replace('close (prev);','rboxc_close_pipeline_fd (prev);').replace('close (fildes[1]);','rboxc_close_pipeline_fd (fildes[1]);')
   text=text[:start]+segment+text[end:]
   # Release owned heap objects without restoring abandoned native frames.
   start=text.index('static void\ninitialize_subshell (void)');end=text.index('#define HASH_BANG_BUFSIZ',start)
   segment=text[start:end];segment=replace(segment,'  clear_unwind_protect_list (0);','  clear_unwind_protect_list (2);');text=text[:start]+segment+text[end:]
   anchor='      args = strvec_from_word_list (words, 0, 0, (int *)NULL);'
   text=replace(text,anchor,anchor.replace('(words, 0,','(words, 1,')+'\n      FREE (command_line);\n      command_line = (char *)NULL;')
   text=replace(text,'  args[0] = shell_name;\n  args[1] = command;', '  args[0] = shell_name;\n  if (args[1] != command) free (args[1]);\n  args[1] = command;')
   # The new interpreter no longer inherits trap commands from the old one.
   # This GNU helper preserves dispositions explicitly set to ignore.
   text=replace(text,'  reset_parser ();\n  initialize_subshell ();','  reset_parser ();\n  free_trap_strings ();\n  initialize_subshell ();')
  elif name=='expr.c':
   assert text.count('vincdec = itos (v2);')==2 and text.count('free (vincdec);')==2
   text=text.replace('vincdec = itos (v2);','vincdec = rboxc_expr_hold (itos (v2));').replace('free (vincdec);','rboxc_expr_release (vincdec);')
   anchor='static procenv_t evalbuf;'
   text=replace(text,anchor,anchor+'''
/* Assignment strings outlive recursive arithmetic parsing, but not a failed
   evaluation. Each evalexp invocation records its own cleanup boundary. */
struct rboxc_expr_string { char *value; struct rboxc_expr_string *next; };
static struct rboxc_expr_string *rboxc_expr_strings;
static char *rboxc_expr_hold (char *value)
{
  struct rboxc_expr_string *item = xmalloc (sizeof *item);
  item->value = value; item->next = rboxc_expr_strings;
  rboxc_expr_strings = item;
  return value;
}
static void rboxc_expr_release (char *value)
{
  struct rboxc_expr_string **slot, *item;
  for (slot = &rboxc_expr_strings; (item = *slot); slot = &item->next)
    if (item->value == value) {
      *slot = item->next; free (value); free (item); return;
    }
  abort (); /* Every caller releases an explicitly registered owned string. */
}
static void rboxc_expr_release_to (struct rboxc_expr_string *mark)
{
  while (rboxc_expr_strings != mark)
    rboxc_expr_release (rboxc_expr_strings->value);
}
''')
   text=replace(text,'  procenv_t oevalbuf;','  procenv_t oevalbuf;\n  struct rboxc_expr_string *const owned_mark = rboxc_expr_strings;')
   text=replace(text,'  if (c)\n    {\n      FREE (tokstr);','  if (c)\n    {\n      rboxc_expr_release_to (owned_mark);\n      FREE (tokstr);')
   start=text.index('expassign (void)\n{');end=text.index('/* Conditional expression',start)
   segment=text[start:end]
   segment=replace(segment,'lhs = savestring (tokstr);','lhs = rboxc_expr_hold (savestring (tokstr));')
   segment=replace(segment,'rhs = itos (value);','rhs = rboxc_expr_hold (itos (value));')
   assert segment.count('free (lhs);')==2 and segment.count('free (rhs);')==1
   segment=segment.replace('free (lhs);','rboxc_expr_release (lhs);').replace('free (rhs);','rboxc_expr_release (rhs);')
   text=text[:start]+segment+text[end:]
  elif name=='eval.c':
   anchor='static void send_pwd_to_eterm (void);'
   text=replace(text,anchor,'''extern void add_unwind_protect_owned (sh_uwfunc_t *, sh_uwfunc_t *, void *);
static void rboxc_keep_reader_command (void *command) { (void)command; }
'''+anchor)
   anchor='\t      execute_command (current_command);'
   text=replace(text,anchor,'''              begin_unwind_frame ("rboxc-reader-command");
              add_unwind_protect_owned (rboxc_keep_reader_command, uw_dispose_command, current_command);
'''+anchor+'''
              discard_unwind_frame ("rboxc-reader-command");''')
  elif name=='bashline.c':
   text=replace(text,'reset_completer_word_break_chars (void)\n{','''reset_completer_word_break_chars (void)
{
  /* Same ownership contract as assign_comp_wordbreaks in variables.c. */
  if (rl_completer_word_break_characters &&
      rl_completer_word_break_characters != rl_basic_word_break_characters)
    free ((void *)rl_completer_word_break_characters);''')
  elif name=='general.c':
   # These four fixed tables have process lifetime. Preserve the original
   # element initialization and pointer bindings without heap allocations.
   for table,size in [('prefixes',3),('prefixes2',2),('suffixes',3),('suffixes2',2)]:
    var='bash_tilde_'+table
    text=replace(text,f'static char **{var};',f'static char *{var}[{size}];')
    text=replace(text,f'      {var} = strvec_create ({size});\n','')
  elif name=='y.tab.c':
   # The command-valued nonterminals from GNU parse.y own their trees
   # until a successful reduction transfers them. inputunit returns via
   # YYACCEPT without assigning a semantic value, so exclude that symbol.
   commands='command pipeline pipeline_command list0 list1 compound_list simple_list simple_list1 simple_command shell_command for_command select_command case_command group_command arith_command cond_command arith_for_command coproc comsub funsub function_def function_body if_command elif_clause subshell'.split()
   assert len(commands)==25 and all('YYSYMBOL_'+n+' =' in text for n in commands)
   cases=''.join('    case YYSYMBOL_'+n+':\n' for n in commands)
   anchor='''  YY_IGNORE_MAYBE_UNINITIALIZED_BEGIN
  YY_USE (yykind);
  YY_IGNORE_MAYBE_UNINITIALIZED_END'''
   start=text.index('yydestruct (const char *yymsg,');end=text.index('\n}',start)
   segment=replace(text[start:end],anchor,'''  YY_IGNORE_MAYBE_UNINITIALIZED_BEGIN
  switch (yykind) {
'''+cases+'''      dispose_command (yyvaluep->command);
      break;
    default: break;
  }
  YY_IGNORE_MAYBE_UNINITIALIZED_END''')
   text=text[:start]+segment+text[end:]
  elif name=='redir.c':
   text=replace(text,'static int add_undo_redirect (int, enum r_instruction, int);','static int add_undo_redirect (int, enum r_instruction, int);\nextern void rboxc_bash_track_backup (int);')
   text=replace(text,'  clexec_flag = fcntl (fd, F_GETFD, 0);','  rboxc_bash_track_backup (new_fd);\n  clexec_flag = fcntl (fd, F_GETFD, 0);')
  elif name=='trap.c':
   text=replace(text,'      sigmodes[EXIT_TRAP] &= ~SIG_TRAPPED;\t/* XXX - SIG_INPROGRESS? */\n','')
   anchor='\t  trap_list[EXIT_TRAP] = (char *)NULL;\n\t}\n    }'
   text=replace(text,anchor,'\t  trap_list[EXIT_TRAP] = (char *)NULL;\n\t}\n      sigmodes[EXIT_TRAP] &= ~SIG_TRAPPED;\n    }')
   anchor='free_trap_string (int sig)\n{'
   text=replace(text,anchor,anchor+'''
  /* reset_signal_handlers retains strings after clearing SIG_TRAPPED.
     Dispose these retained strings before change_signal replaces them. */
  if ((sigmodes[sig] & (SIG_TRAPPED | SIG_INPROGRESS)) == 0 &&
      trap_list[sig] && trap_list[sig] != (char *)IGNORE_SIG &&
      trap_list[sig] != (char *)DEFAULT_SIG &&
      trap_list[sig] != (char *)IMPOSSIBLE_TRAP_HANDLER) {
    free (trap_list[sig]);
    trap_list[sig] = (char *)NULL;
  }
''')
  elif name=='subst.c':
   text=replace(text,'#include "shell.h"','''#include "shell.h"
extern void add_unwind_protect_owned (sh_uwfunc_t *, sh_uwfunc_t *, void *);
/* Normal error unwinding may already have abandoned the expansion frame.
   Only exec restart, before its nonlocal transfer, owns these live slots. */
static void rboxc_keep_expansion_slot (void *slot) { (void)slot; }
static void rboxc_release_expansion_slot (void *slot)
{
  char **value = slot;
  free (*value);
  *value = 0;
}
struct rboxc_word_expansion { char *text; };
static void rboxc_release_word_expansion (void *arg)
{
  struct rboxc_word_expansion *owned = arg;
  free (owned->text);
  free (owned);
}
static void rboxc_dispose_expansion_word (void *arg) { dispose_word (arg); }
''')
   start=text.index('string_extract_double_quoted (const char *string, size_t *sindex, int flags)\n{');end=text.index('\n/*',text.index('\n}',start))
   segment=text[start:end]
   anchor='  temp = (char *)xmalloc (1 + slen - *sindex);'
   segment=replace(segment,anchor,anchor+'''
  begin_unwind_frame ("rboxc-extract-quoted");
  add_unwind_protect (xfree, temp);''')
   segment=replace(segment,'  return (temp);','  discard_unwind_frame ("rboxc-extract-quoted");\n  return (temp);')
   text=text[:start]+segment+text[end:]
   # The process-substitution child owns its private pathname copy; it can
   # abandon this C frame when executing an ordinary text script.
   start=text.index('process_substitute (char *string, int open_for_read_in_child)');end=text.index('#endif /* PROCESS_SUBSTITUTION */',start)
   segment=text[start:end]
   segment=replace(segment,'  remove_quoted_escapes (string);','  add_unwind_protect (xfree, pathname);\n  remove_quoted_escapes (string);')
   text=text[:start]+segment+text[end:]
   start=text.index('parameter_brace_expand_rhs (char *name, char *value,');end=text.index('\n/*',text.index('\n}',start))
   segment=text[start:end]
   anchor='  l = *temp ? expand_string_for_rhs (temp, quoted, op, pflags, &l_hasdollat, (int *)NULL)\n\t    : (WORD_LIST *)0;'
   segment=replace(segment,anchor,'''  begin_unwind_frame ("rboxc-brace-rhs");
  if (temp != value) add_unwind_protect (xfree, temp);
  add_unwind_protect_owned (rboxc_dispose_expansion_word, rboxc_dispose_expansion_word, w);
'''+anchor+'''
  discard_unwind_frame ("rboxc-brace-rhs");''')
   text=text[:start]+segment+text[end:]
   start=text.index('parameter_brace_expand (char *string, size_t *indexp, int quoted, int pflags, int *quoted_dollar_atp, int *contains_dollar_at)\n{');end=text.index('\n/*',text.index('\n}',start))
   segment=text[start:end]
   def protect_rhs(match):
    return '''begin_unwind_frame ("rboxc-brace-arguments");
          add_unwind_protect (xfree, name);
          add_unwind_protect (xfree, value);
          '''+match[0]+'''
          discard_unwind_frame ("rboxc-brace-arguments");'''
   segment,count=re.subn(r'ret = parameter_brace_expand_rhs \(.*?\);',protect_rhs,segment,flags=re.S)
   assert count==2,count
   text=text[:start]+segment+text[end:]
   anchor='static WORD_LIST *\nshell_expand_word_list (WORD_LIST *tlist, int eflags)'
   text=replace(text,anchor,'''struct rboxc_expanded_words { WORD_LIST *original, *expanded; };
static void rboxc_dispose_expanded_words (void *arg)
{
  struct rboxc_expanded_words *owned = arg;
  dispose_words (owned->original);
  dispose_words (owned->expanded);
  free (owned);
}

'''+anchor)
   start=text.index(anchor);end=text.index('/* Perform assignment statements optionally',start)
   segment=text[start:end]
   segment=replace(segment,'  int expanded_something, has_dollar_at;','''  int expanded_something, has_dollar_at;
  struct rboxc_expanded_words *owned = xmalloc (sizeof *owned);
  owned->original = tlist; owned->expanded = 0;
  begin_unwind_frame ("rboxc-shell-expand");
  add_unwind_protect_owned (rboxc_dispose_expanded_words, rboxc_dispose_expanded_words, owned);''')
   segment=replace(segment,'\t  dispose_words (orig_list);\n\t  /* Dispose the new list we\'re building. */\n\t  dispose_words (new_list);','\t  run_unwind_frame ("rboxc-shell-expand");')
   anchor='      new_list = (WORD_LIST *)list_append ((GENERIC_LIST *)expanded, (GENERIC_LIST *)new_list);'
   segment=replace(segment,anchor,anchor+'\n      owned->expanded = new_list;')
   segment=replace(segment,'  if (orig_list)  ','  discard_unwind_frame ("rboxc-shell-expand");\n  free (owned);\n  if (orig_list)  ')
   text=text[:start]+segment+text[end:]
   start=text.index('expand_string_assignment (const char *string, int quoted)');end=text.index('\n/* Expand one of the PS?',start)
   segment=text[start:end]
   anchor='  td.word = savestring (string);'
   segment=replace(segment,anchor,anchor+'''
  begin_unwind_frame ("rboxc-assignment-expansion");
  add_unwind_protect_owned (rboxc_keep_expansion_slot, rboxc_release_expansion_slot, &td.word);''')
   segment=replace(segment,'  FREE (td.word);','  discard_unwind_frame ("rboxc-assignment-expansion");\n  FREE (td.word);')
   text=text[:start]+segment+text[end:]
   start=text.index('expand_word_internal (WORD_DESC *word, int quoted, int isexp, int *contains_dollar_at, int *expanded_something)');end=text.index('\n/* **************************************************************** */',start)
   segment=text[start:end]
   anchor='\t      list = expand_word_internal (tword, Q_DOUBLE_QUOTES|(quoted&Q_ARITH), 0, &temp_has_dollar_at, (int *)NULL);'
   segment=replace(segment,anchor,'''              begin_unwind_frame ("rboxc-quoted-word");
              add_unwind_protect_owned (rboxc_dispose_expansion_word, rboxc_dispose_expansion_word, tword);
'''+anchor+'''
              discard_unwind_frame ("rboxc-quoted-word");''')
   segment=replace(segment,'  char *istring;','  struct rboxc_word_expansion *owned;')
   anchor='  istring = (char *)xmalloc (istring_size = DEFAULT_INITIAL_ARRAY_SIZE);'
   before,after=segment.split(anchor)
   # Four actual returns after allocation (the other occurrences are prose).
   after,count=re.subn(r'(?m)^(\s*)return ((?:\(|list;))',r'\1discard_unwind_frame ("rboxc-word-expansion");\n\1free (owned);\n\1return \2',after)
   assert count==4,count
   segment=before+'  owned = xmalloc (sizeof *owned);\n'+anchor+'''
  begin_unwind_frame ("rboxc-word-expansion");
  add_unwind_protect_owned (rboxc_release_word_expansion, rboxc_release_word_expansion, owned);
'''+after
   segment=re.sub(r'\bistring\b','owned->text',segment)
   segment=segment.replace('free (owned->text);','{ free (owned->text); owned->text = 0; }')
   text=text[:start]+segment+text[end:]
  elif name=='builtins/evalstring.c':
   text=replace(text,'#include "../shell.h"','#include "../shell.h"\nextern void add_unwind_protect_owned (sh_uwfunc_t *, sh_uwfunc_t *, void *);')
   text=replace(text,'add_unwind_protect (uw_restore_lastcom, lastcom);','add_unwind_protect_owned (uw_restore_lastcom, xfree, lastcom);')
  else:
   anchor='open_shell_script (char *script_name)\n{'
   pos=text.index(anchor);pos=text.rfind('\n',0,pos-1)
   helper='''
static int rboxc_script_cleanup_registered;
static struct stat rboxc_script_identity;
static void rboxc_script_cleanup (void)
{
  int saved = errno;
  struct stat current;
  if (default_buffered_input > 0 && fstat(default_buffered_input, &current) == 0 &&
      current.st_dev == rboxc_script_identity.st_dev && current.st_ino == rboxc_script_identity.st_ino &&
      current.st_rdev == rboxc_script_identity.st_rdev)
    unset_bash_input (0);
  errno = saved;
}
'''
   text=text[:pos]+helper+text[pos:]
   anchor='  reading_shell_script = 1;'
   text=replace(text,anchor,'''  if (default_buffered_input > 0 && fstat(default_buffered_input, &rboxc_script_identity) == 0 && !rboxc_script_cleanup_registered) {
    if (atexit(rboxc_script_cleanup) != 0) _exit(2);
    rboxc_script_cleanup_registered = 1;
  }
'''+anchor)
  adapted=stage/name;adapted.parent.mkdir(parents=True,exist_ok=True);adapted.write_text(text);output=adapted.with_suffix('.o')
  if name=='native-helpers.c':
   r=json.loads((root/'evidence/bash-split-entry.json').read_text());record={'arguments':r['gcc_arguments'],'directory':r['directory']}
  else:
   matches=[r for r in records if r.get('file')==str(original)]
   if not matches:
    # Recover exact configured object commands without relinking the oracle.
    log=stage/(name+'.record.log')
    directory=root/'build/gnu-bash'/Path(name).parent
    with log.open('w') as out:
     subprocess.run(['make','-C',str(directory),'-W',str(original),Path(name).with_suffix('.o').name,
       'CC=python3 '+str(root/'scripts/record-provider-cc.py')],
       env={**os.environ,'RBOXC_CC_RECORDS':str(root/'build/bash-cc-records')},stdout=out,stderr=subprocess.STDOUT,check=True)
    records=[json.loads(p.read_text()) for p in (root/'build/bash-cc-records').glob('*.json')]
    matches=[r for r in records if r.get('file')==str(original)]
   assert len(matches)==1;record=matches[0]
  args=record['arguments'].copy()
  if '-o' in args:args[args.index('-o')+1]=str(output)
  else:args+=['-o',str(output)]
  args[args.index(str(original))]=str(adapted);args+=['-iquote',str(original.parent),'-iquote',str(source)]
  log=stage/(name+'.log')
  with log.open('w') as out:subprocess.run(args,cwd=record['directory'],stdout=out,stderr=subprocess.STDOUT,check=True)
  outputs[output.name]=output;reports.append({'source':str(original),'source_sha256':fingerprint(original),'adapted_sha256':fingerprint(adapted),'object_sha256':fingerprint(output),'compile_arguments':args,'log_sha256':fingerprint(log)})
 archive=stage/'libbuiltins.a';shutil.copyfile(root/'build/gnu-bash/builtins/libbuiltins.a',archive)
 subprocess.run(['ar','r',str(archive),str(outputs.pop('evalstring.o'))],check=True)
 subprocess.run(['ranlib',str(archive)],check=True);outputs[archive.name]=archive
 (root/'evidence/bash-native-cleanup.json').write_text(json.dumps({'scope':'Release Bash restart caches, snapshots, argument vectors, unwind payloads, expansion buffers and discarded trap strings. Dispose rejected arithmetic-for trees, failed arithmetic assignment/increment strings, temporary word lists, reader commands abandoned during exec restart and replaced completion strings. Track owned standard remaps and redirection backups, forget normal backup closes, and finalize abandoned descriptors at normal exit.','driver_sha256':fingerprint(Path(__file__)),'files':reports,'adapted_builtin_archive_sha256':fingerprint(archive)},indent=2)+'\n')
 return outputs
