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
    for name in ('dfasearch', 'kwsearch', 'kwset'):
        original = Path(pin['source'])/f'src/{name}.c'
        assert fingerprint(original) == pin['helper_source_sha256'][name]
        text = original.read_text()
        if name == 'dfasearch':
            text = replace_once(text, '  struct re_pattern_buffer *patterns;',
                '  struct re_pattern_buffer *patterns;\n  struct re_pattern_buffer *rboxc_patterns_base;')
            text = replace_once(text, '\n  dc->patterns++;',
                '\n  dc->rboxc_patterns_base = dc->patterns;\n  dc->patterns++;')
            text = replace_once(text, '          dc->patterns++;',
                '          dc->rboxc_patterns_base = dc->patterns;\n          dc->patterns++;')
            text += '''
/* The matcher keeps an interior pointer until execution completes. */
void kwsfree_owned (kwset_t);
void
GEAfree (void *compiled)
{
  struct dfa_comp *dc = compiled;
  if (!dc)
    return;
  for (idx_t i = 0; i < dc->pcount; i++)
    regfree (&dc->patterns[i]);
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
        else:
            text += '''
/* Only for kwsets created by grep's kwsinit, which owns trans. */
void
kwsfree_owned (kwset_t kwset)
{
  free ((void *) kwset->trans);
  kwsfree (kwset);
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
        'scope': 'Destructors for completed basic/extended/fixed matcher objects; partial compilation and PCRE ownership are separate.',
        'driver_sha256': fingerprint(Path(__file__)), 'helpers': evidence}, indent=2)+'\n')
    return outputs
