"""Discard heap owners without restoring a scope that EXIT must still observe."""
# SPDX-License-Identifier: GPL-3.0-or-later
import json, subprocess
from pathlib import Path
from entry_provider_helpers import fingerprint


def prepare(root, adapted):
    profiles = {n:json.loads((root/f'evidence/{n}.json').read_text()) for n in
                ('bash-function-substitution-cleanup','bash-subshell-cleanup')}
    stage = root/'build/bash-exit-scope-cleanup'
    stage.mkdir(exist_ok=True)
    reports = []
    for name in ('subst.c','unwind_prot.c','evalstring.c'):
        profile = profiles['bash-function-substitution-cleanup' if name=='subst.c' else 'bash-subshell-cleanup']
        record, = [r for r in profile['files'] if Path(r['source']).name==name]
        original = Path(record['source'])
        assert fingerprint(original)==record['source_sha256']
        text = original.read_text()

        def replace(old,new):
            nonlocal text
            assert text.count(old)==1,(name,old)
            text=text.replace(old,new)

        if name=='subst.c':
            # Only these arguments are heap allocations. Trap restart also has
            # stack-based owners, so it must keep its separate discard policy.
            replace('extern void add_unwind_protect_owned (sh_uwfunc_t *, sh_uwfunc_t *, void *);',
                    'extern void add_unwind_protect_owned (sh_uwfunc_t *, sh_uwfunc_t *, void *);\nextern void add_unwind_protect_heap (sh_uwfunc_t *, sh_uwfunc_t *, void *);')
            for callback in ('rboxc_dispose_expansion_word','rboxc_release_word_expansion','rboxc_dispose_expanded_words'):
                assert 'add_unwind_protect_owned ('+callback in text
                text=text.replace('add_unwind_protect_owned ('+callback,'add_unwind_protect_heap ('+callback)
            replace('  add_unwind_protect (uw_maybe_restore_getopt_state, gs);',
                    '  add_unwind_protect_heap (uw_maybe_restore_getopt_state, xfree, gs);')
            anchor='static void\nuw_unbind_variable (void *name)'
            replace(anchor,'''/* Keep descriptor visibility through EXIT; the exit registry owns it. */
static void
rboxc_discard_anonfile (void *arg)
{
  STRING_INT_ALIST *af = arg;
  free (af->word);
  free (af);
}

'''+anchor)
            replace('      add_unwind_protect_owned (uw_anonclose, uw_anonclose, anonf);',
                    '      add_unwind_protect_heap (uw_anonclose, rboxc_discard_anonfile, anonf);')
        elif name=='unwind_prot.c':
            replace('    sh_uwfunc_t *discard;', '    sh_uwfunc_t *discard;\n    int heap_discard;')
            assert text.count('  elt->head.discard = 0;')==2
            text=text.replace('  elt->head.discard = 0;', '  elt->head.discard = 0;\n  elt->head.heap_discard = 0;')
            anchor='/* Remove the top unwind protect from the list. */'
            replace(anchor,'''/* The discard callback owns heap data and never references a C stack slot. */
void
add_unwind_protect_heap (sh_uwfunc_t *cleanup, sh_uwfunc_t *discard, void *arg)
{
  add_unwind_protect_owned (cleanup, discard, arg);
  unwind_protect_list->head.heap_discard = 1;
}

'''+anchor)
            replace('      if (elt->head.discard == xfree)\n        xfree (elt->arg.v);',
                    '      if (elt->head.heap_discard && elt->head.discard)\n        elt->head.discard (elt->arg.v);\n      else if (elt->head.discard == xfree)\n        xfree (elt->arg.v);')
        else:
            replace('\t\t      dispose_command (command);\n\t\t      discard_unwind_frame ("pe_dispose");',
                    '\t\t      discard_unwind_frame_heap ("pe_dispose");')

        source = stage/('builtins/'+name if name=='evalstring.c' else name)
        source.parent.mkdir(exist_ok=True)
        source.write_text(text)
        obj = source.with_suffix('.o')
        command = record['compile_arguments'].copy()
        command[command.index(str(original))]=str(source)
        command[command.index('-o')+1]=str(obj)
        log = source.with_suffix('.c.log')
        with log.open('w') as out:
            subprocess.run(command,cwd=record['directory'],stdout=out,stderr=subprocess.STDOUT,check=True)
        adapted[obj.name]=obj
        reports.append({'original':str(original),'original_sha256':fingerprint(original),
            'source':str(source),'source_sha256':fingerprint(source),
            'object':str(obj),'object_sha256':fingerprint(obj),'compile_arguments':command,
            'directory':record['directory'],'log':str(log),'log_sha256':fingerprint(log)})
    (root/'evidence/bash-exit-scope-cleanup.json').write_text(json.dumps({
        'scope':'Explicit heap-only discard registrations release expansion and getopt owners when EXIT keeps function scope visible. Stack-based trap restart registrations retain their separate policy. The existing pe_dispose heap cleanup releases command and descriptor bitmap without restoring function or descriptor state.',
        'driver_sha256':fingerprint(Path(__file__)),'files':reports},indent=2)+'\n')
