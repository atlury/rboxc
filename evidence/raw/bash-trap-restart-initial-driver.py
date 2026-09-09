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
    start = text.index('_run_trap_internal (int sig, char *tag)\n{')
    end = text.index('\nint\nrun_debug_trap', start)
    body = text[start:end]
    old = '      save_parser_state (&pstate);'
    assert body.count(old) == 1
    body = body.replace(old, old+'''
      begin_unwind_frame ("rboxc-trap-restart-state");
      add_unwind_protect_owned (rboxc_keep_trap_state,
                                rboxc_discard_trap_parser, &pstate);
#if defined (ARRAY_VARS)
      add_unwind_protect_owned (rboxc_keep_trap_state,
                                rboxc_discard_trap_status, ps);
#endif''')
    old = '      subst_assign_varlist = save_subst_varlist;'
    assert body.count(old) == 1
    body = body.replace(old, '      discard_unwind_frame ("rboxc-trap-restart-state");\n'+old)
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
        'scope':'Register discard-only owners for the saved trap parser state and the separate PIPESTATUS snapshot. GNU flush_parser_state frees its owned token buffer, saved token state and status copy. Normal trap returns discard these registrations and retain GNU restoration. Only interpreter restart invokes the owned destructors before abandoning live native frames.',
        'driver_sha256':fingerprint(Path(__file__)),
        'previous_profile_sha256':fingerprint(previous_path),
        'previous_source_sha256':fingerprint(original),
        'source':str(source),'source_sha256':fingerprint(source),
        'object':str(obj),'object_sha256':fingerprint(obj),
        'compile_command':command,'build_log':str(log),'build_log_sha256':fingerprint(log)
    },indent=2)+'\n')
