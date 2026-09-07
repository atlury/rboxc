"""Add destructors for completed GNU Grep matcher objects without changing matching."""
# SPDX-License-Identifier: GPL-3.0-or-later
import json
from pathlib import Path
import subprocess
from grep_helpers import fingerprint


def replace_once(text, before, after):
    assert text.count(before) == 1, before
    return text.replace(before, after, 1)


def prepare(root):
    pin = json.loads((root/'inventory/sources.json').read_text())['grep']
    stage = root/'build/grep-cleanup'
    stage.mkdir(exist_ok=True)
    records = [json.loads(p.read_text()) for p in (root/'build/grep-cc-records').glob('*.json')]
    outputs = {}
    evidence = {}
    for name in ('dfasearch', 'kwsearch', 'kwset', 'pcresearch'):
        original = Path(pin['source'])/f'src/{name}.c'
        assert fingerprint(original) == pin['helper_source_sha256'][name]
        text = original.read_text()
        if name == 'dfasearch':
            text = replace_once(text, '  struct re_pattern_buffer *patterns;',
                '  struct re_pattern_buffer *patterns;\n  struct re_pattern_buffer *rboxc_patterns_base;\n  struct dfa_comp *rboxc_next;\n  char *rboxc_buffer;\n  char *rboxc_motif;')
            text = replace_once(text, '\n  dc->patterns++;',
                '\n  dc->rboxc_patterns_base = dc->patterns;\n  dc->patterns++;')
            text = replace_once(text, '          dc->patterns++;',
                '          dc->rboxc_patterns_base = dc->patterns;\n          dc->patterns++;')
            text = replace_once(text, 'void\ndfaerror (char const *mesg)',
                'static struct dfa_comp *rboxc_dfa_head;\n\nvoid\ndfaerror (char const *mesg)')
            text = replace_once(text, '  struct dfa_comp *dc = xcalloc (1, sizeof (*dc));',
                '  struct dfa_comp *dc = xcalloc (1, sizeof (*dc));\n  dc->rboxc_next = rboxc_dfa_head;\n  rboxc_dfa_head = dc;')
            text = replace_once(text, '  struct re_pattern_buffer pat;',
                '  memset (&dc->patterns[pcount], 0, sizeof dc->patterns[pcount]);\n  struct re_pattern_buffer pat;')
            text = replace_once(text, '            buf = xpalloc (buf, &bufalloc, bufshortage, -1, 1);',
                '            buf = xpalloc (buf, &bufalloc, bufshortage, -1, 1);\n          dc->rboxc_buffer = buf;')
            text = replace_once(text, '      buf = xirealloc (buf, buflen + prevlen);',
                '      buf = xirealloc (buf, buflen + prevlen);\n      dc->rboxc_buffer = buf;')
            text = replace_once(text, '      char *n = ximalloc (size + bracket_bytes);',
                '      char *n = ximalloc (size + bracket_bytes);\n      dc->rboxc_motif = n;')
            text = replace_once(text, '        free (buf);',
                '        free (buf);\n      dc->rboxc_buffer = nullptr;')
            text = replace_once(text, '  free (motif);',
                '  free (motif);\n  dc->rboxc_motif = nullptr;')
            text += '''
/* The matcher keeps an interior pointer until execution completes. */
void kwsfree_owned (kwset_t);
void
GEAfree (void *compiled)
{
  struct dfa_comp *dc = compiled;
  if (!dc)
    return;
  struct dfa_comp **link = &rboxc_dfa_head;
  while (*link && *link != dc)
    link = &(*link)->rboxc_next;
  if (*link)
    *link = dc->rboxc_next;
  for (idx_t i = 0; i < dc->pcount; i++)
    if (dc->patterns[i].buffer)
      regfree (&dc->patterns[i]);
    else
      free (dc->patterns[i].fastmap);
  free (dc->rboxc_buffer);
  free (dc->rboxc_motif);
  free (dc->rboxc_patterns_base);
  free (dc->regs.start);
  free (dc->regs.end);
  if (dc->kwset)
    kwsfree_owned (dc->kwset);
  if (dc->dfa)
    {
      dfafree (dc->dfa);
      free (dc->dfa);
    }
  free (dc);
}
void
GEAfree_all (void)
{
  while (rboxc_dfa_head)
    GEAfree (rboxc_dfa_head);
}
'''
        elif name == 'kwsearch':
            text += '''
void kwsfree_owned (kwset_t);
void GEAfree (void *);
void
Ffree (void *compiled)
{
  struct kwsearch *kw = compiled;
  if (!kw)
    return;
  if (kw->re)
    GEAfree (kw->re);
  kwsfree_owned (kw->kwset);
  /* pattern borrows the entry's pattern storage. */
  free (kw);
}
'''
        elif name == 'kwset':
            text += '''
/* Only for kwsets created by grep's kwsinit, which owns trans. */
void
kwsfree_owned (kwset_t kwset)
{
  free ((void *) kwset->trans);
  kwsfree (kwset);
}
'''
        else:
            text = replace_once(text, '  pcre2_general_context *gcontext;',
                '  pcre2_general_context *gcontext;\n  struct pcre_comp *rboxc_next;\n  pcre2_compile_context *rboxc_ccontext;\n  uint8_t const *rboxc_tables;\n  void *rboxc_re_storage;')
            text = replace_once(text, '/* Memory allocation functions for PCRE.  */',
                'static struct pcre_comp *rboxc_pcre_head;\n\n/* Memory allocation functions for PCRE.  */')
            text = replace_once(text, '  struct pcre_comp *pc = ximalloc (sizeof *pc);',
                '  struct pcre_comp *pc = xcalloc (1, sizeof *pc);\n  pc->rboxc_next = rboxc_pcre_head;\n  rboxc_pcre_head = pc;')
            text = replace_once(text, '  pcre2_compile_context *ccontext = pcre2_compile_context_create (gcontext);',
                '  pcre2_compile_context *ccontext = pcre2_compile_context_create (gcontext);\n  pc->rboxc_ccontext = ccontext;')
            anchor = '      char *re = re_storage = ximalloc (re_size);'
            assert text.count(anchor) == 2
            text = text.replace(anchor, anchor+'\n      pc->rboxc_re_storage = re_storage;')
            text = replace_once(text, '    pcre2_set_character_tables (ccontext, pcre2_maketables (gcontext));',
                '    {\n      pc->rboxc_tables = pcre2_maketables (gcontext);\n      pcre2_set_character_tables (ccontext, pc->rboxc_tables);\n    }')
            text = replace_once(text, '  free (re_storage);',
                '  free (re_storage);\n  pc->rboxc_re_storage = nullptr;')
            text = replace_once(text, '  pcre2_compile_context_free (ccontext);',
                '  pcre2_compile_context_free (ccontext);\n  pc->rboxc_ccontext = nullptr;')
            text += '''
void
Pfree (void *compiled)
{
  struct pcre_comp *pc = compiled;
  if (!pc)
    return;
  struct pcre_comp **link = &rboxc_pcre_head;
  while (*link && *link != pc)
    link = &(*link)->rboxc_next;
  if (*link)
    *link = pc->rboxc_next;
  pcre2_match_data_free (pc->data);
  pcre2_code_free (pc->cre);
  pcre2_match_context_free (pc->mcontext);
  pcre2_jit_stack_free (pc->jit_stack);
  pcre2_compile_context_free (pc->rboxc_ccontext);
  if (pc->rboxc_tables)
    pcre2_maketables_free (pc->gcontext, pc->rboxc_tables);
  free (pc->rboxc_re_storage);
  pcre2_general_context_free (pc->gcontext);
  free (pc);
}
void
Pfree_all (void)
{
  while (rboxc_pcre_head)
    Pfree (rboxc_pcre_head);
}
'''
        translated = stage/(name+'.c'); translated.write_text(text)
        selected = [r for r in records if Path(r['file']) == original]
        assert len(selected) == 1
        record = selected[0]
        arguments = record['arguments'].copy()
        arguments[arguments.index('-o')+1] = str(stage/(name+'.o'))
        arguments[arguments.index(str(original))] = str(translated)
        arguments += ['-I'+str(original.parent)]
        log = root/f'evidence/raw/grep-{name}-cleanup-build.log'
        with log.open('w') as out:
            subprocess.run(arguments, cwd=record['directory'], stdout=out, stderr=subprocess.STDOUT, check=True)
        outputs[name] = stage/(name+'.o')
        evidence[name] = {'original_sha256': fingerprint(original), 'adapted_source_sha256': fingerprint(translated),
                          'object_sha256': fingerprint(outputs[name]), 'compiler_arguments': arguments,
                          'log': str(log.relative_to(root)), 'log_sha256': fingerprint(log)}
    (root/'evidence/grep-native-cleanup.json').write_text(json.dumps({
        'scope': 'Track basic/extended and PCRE matcher objects through compilation and execution, and release completed fixed matchers and their case-folding tables.',
        'driver_sha256': fingerprint(Path(__file__)), 'helpers': evidence}, indent=2)+'\n')
    return outputs
