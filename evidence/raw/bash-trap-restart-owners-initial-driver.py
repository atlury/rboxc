"""Release saved trap parser/status state when interpreter restart abandons it."""
# SPDX-License-Identifier: GPL-3.0-or-later
import json
from pathlib import Path
import subprocess

from entry_provider_helpers import fingerprint


def prepare(root, adapted):
    previous_path = root/'evidence/bash-return-trap-cleanup.json'
    previous = json.loads(previous_path.read_text())
    original = Path(previous['source'])
    assert fingerprint(original) == previous['source_sha256']
    text = original.read_text()

    def replace(old, new):
        nonlocal text
        assert text.count(old) == 1, old
        text = text.replace(old, new)

    replace('#include "shell.h"', '''#include "shell.h"
#include "unwind_prot.h"
extern void add_unwind_protect_owned (sh_uwfunc_t *, sh_uwfunc_t *, void *);
/* Normal trap return still uses GNU's original state restoration below.
   The owned discard callbacks run only before interpreter restart jumps
   away from these still-live native frames. */
static void rboxc_keep_trap_state (void *state) { (void)state; }
static void rboxc_discard_trap_parser (void *state)
{
  flush_parser_state ((sh_parser_state_t *)state);
}
#if defined (ARRAY_VARS)
static void rboxc_discard_trap_status (void *state)
{
  if (state) array_dispose ((ARRAY *)state);
}
#endif''')
    # Every actual trap-string release invalidates live borrows. This avoids
    # mistaking a cleared/reinstalled slot or a nested borrow for ownership.
    assert text.count('free (trap_list[sig]);') == 2
    text = text.replace('free (trap_list[sig]);',
                        'rboxc_release_trap_string (trap_list[sig]);')
    replace('free (old_trap);', 'rboxc_release_trap_string (old_trap);')
    replace('char *trap_list[BASH_NSIG];', '''char *trap_list[BASH_NSIG];

struct rboxc_trap_borrow {
  char *string;
  int sig;
  struct rboxc_trap_borrow *previous;
};
static struct rboxc_trap_borrow *rboxc_trap_borrows;

static void
rboxc_release_trap_string (char *string)
{
  struct rboxc_trap_borrow *borrow;
  for (borrow = rboxc_trap_borrows; borrow; borrow = borrow->previous)
    if (borrow->string == string) borrow->string = 0;
  if (rboxc_active_return_trap == string) rboxc_active_return_trap = 0;
  free (string);
}

static void
rboxc_unlink_trap_borrow (void *state)
{
  struct rboxc_trap_borrow *borrow = state, **link;
  for (link = &rboxc_trap_borrows; *link; link = &(*link)->previous)
    if (*link == borrow) { *link = borrow->previous; break; }
}

static void
rboxc_discard_trap_borrow (void *state)
{
  struct rboxc_trap_borrow *borrow = state;
  /* The current slot still owns an attached string. Only a detached,
     unreleased borrow transfers ownership when its frame is abandoned. */
  if (borrow->string && trap_list[borrow->sig] != borrow->string)
    rboxc_release_trap_string (borrow->string);
  if (borrow->sig == RETURN_TRAP) rboxc_active_return_trap = 0;
  rboxc_unlink_trap_borrow (state);
}''')
    start = text.index('_run_trap_internal (int sig, char *tag)\n{')
    end = text.index('\nint\nrun_debug_trap', start)
    body = text[start:end]
    body = body.replace('  char *rboxc_previous_return_trap;',
                        '  struct rboxc_trap_borrow rboxc_borrow;\n'
                        '  char *rboxc_previous_return_trap;')
    old = '      save_parser_state (&pstate);'
    assert body.count(old) == 1
    body = body.replace(old, old+'''
      begin_unwind_frame ("rboxc-trap-restart-state");
      rboxc_borrow.string = old_trap;
      rboxc_borrow.sig = sig;
      rboxc_borrow.previous = rboxc_trap_borrows;
      rboxc_trap_borrows = &rboxc_borrow;
      add_unwind_protect_owned (rboxc_unlink_trap_borrow,
                                rboxc_discard_trap_borrow, &rboxc_borrow);
      add_unwind_protect_owned (rboxc_keep_trap_state, xfree, old_trapsig);
      add_unwind_protect_owned (rboxc_keep_trap_state,
                                rboxc_discard_trap_parser, &pstate);
#if defined (ARRAY_VARS)
      add_unwind_protect_owned (rboxc_keep_trap_state,
                                rboxc_discard_trap_status, ps);
#endif''')
    old = '      subst_assign_varlist = save_subst_varlist;'
    assert body.count(old) == 1
    body = body.replace(old, '      rboxc_unlink_trap_borrow (&rboxc_borrow);\n'
                            '      discard_unwind_frame ("rboxc-trap-restart-state");\n'+old)
    text = text[:start]+body+text[end:]
    # The pending-signal dispatcher owns a second status/signal snapshot
    # outside _run_trap_internal, plus parser state for ordinary signals.
    start = text.index('run_pending_traps (void)\n{')
    end = text.index('\n/* Set the private state variables', start)
    body = text[start:end]
    old = '  old_trapsig = save_bash_trapsig ();'
    assert body.count(old) == 1
    body = body.replace(old, old+'''
  begin_unwind_frame ("rboxc-pending-trap-state");
  add_unwind_protect_owned (rboxc_keep_trap_state, xfree, old_trapsig);
#if defined (ARRAY_VARS)
  add_unwind_protect_owned (rboxc_keep_trap_state,
                            rboxc_discard_trap_status, ps);
#endif''')
    old = '\t      save_parser_state (&pstate);'
    assert body.count(old) == 1
    body = body.replace(old, old+'''
              begin_unwind_frame ("rboxc-pending-trap-parser");
              add_unwind_protect_owned (rboxc_keep_trap_state,
                                        rboxc_discard_trap_parser, &pstate);''')
    old = '\t      restore_parser_state (&pstate);'
    assert body.count(old) == 1
    body = body.replace(old, '              discard_unwind_frame ("rboxc-pending-trap-parser");\n'+old)
    old = '\t\t      restore_bash_trapsig (old_trapsig);'
    assert body.count(old) == 1
    body = body.replace(old, '''                      discard_unwind_frame ("rboxc-pending-trap-state");
#if defined (ARRAY_VARS)
                      array_dispose (ps);
#endif
'''+old)
    old = '#if defined (ARRAY_VARS)\n  restore_pipestatus_array (ps);'
    assert body.count(old) == 1
    body = body.replace(old, '  discard_unwind_frame ("rboxc-pending-trap-state");\n'+old)
    text = text[:start]+body+text[end:]
    stage = root/'build/bash-trap-restart-cleanup'
    stage.mkdir(exist_ok=True)
    source = stage/'trap.c'
    source.write_text(text)
    obj = stage/'trap.o'
    command = list(previous['compile_command'])
    command[command.index(str(original))] = str(source)
    command[command.index('-o')+1] = str(obj)
    log = stage/'build.log'
    with log.open('w') as output:
        subprocess.run(command, cwd=root/'build/gnu-bash', stdout=output,
                       stderr=subprocess.STDOUT, check=True)
    assert adapted['trap.o'] == Path(previous['object'])
    adapted['trap.o'] = obj
    (root/'evidence/bash-trap-restart-cleanup.json').write_text(json.dumps({
        'scope':'Register discard-only owners for trap parser state, PIPESTATUS and BASH_TRAPSIG snapshots, including the outer pending-signal dispatcher. Track active handler borrows and invalidate them at every actual string release; restart releases only detached live borrows. Normal returns retain GNU restoration and unlink ownership records. A pending trap returning out of a function disposes its otherwise abandoned status copy without restoring it.',
        'driver_sha256':fingerprint(Path(__file__)),
        'previous_profile_sha256':fingerprint(previous_path),
        'previous_source_sha256':fingerprint(original),
        'source':str(source),'source_sha256':fingerprint(source),
        'object':str(obj),'object_sha256':fingerprint(obj),
        'compile_command':command,'build_log':str(log),'build_log_sha256':fingerprint(log)
    },indent=2)+'\n')
