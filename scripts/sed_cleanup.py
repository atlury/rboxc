"""Track Sed regex and FILE ownership through normal and diagnostic exits."""
# SPDX-License-Identifier: GPL-3.0-or-later
import json
from pathlib import Path
import subprocess
from sed_helpers import fingerprint


def replace_once(text, before, after):
    assert text.count(before) == 1, before
    return text.replace(before, after, 1)


def prepare(root):
    pin = json.loads((root/'inventory/sources.json').read_text())['sed']
    stage = root/'build/sed-cleanup'
    stage.mkdir(exist_ok=True)
    records = [json.loads(p.read_text()) for p in (root/'build/sed-cc-records').glob('*.json')]
    outputs = {}
    evidence = {}
    for name in ('regexp', 'utils', 'compile'):
        original = Path(pin['source'])/f'sed/{name}.c'
        assert fingerprint(original) == pin['helper_source_sha256'][name]
        text = original.read_text()
        if name == 'regexp':
            text = replace_once(text, 'struct regex *\ncompile_regex (', '''struct owned_regex
{
  struct regex *value;
  struct owned_regex *next;
};
static struct owned_regex *owned_regexes;

struct regex *
compile_regex (''')
            text = replace_once(text, '  new_regex = xzalloc (sizeof (struct regex) + re_len - 1);', '''  struct owned_regex *owner = xzalloc (sizeof *owner);
  owner->next = owned_regexes;
  owned_regexes = owner;
  new_regex = xzalloc (sizeof (struct regex) + re_len - 1);
  owner->value = new_regex;''')
            # Expose GNU's existing destructor, without enabling unrelated lint code.
            text = replace_once(text, '#ifdef lint\nvoid\nrelease_regex', 'void\nrelease_regex')
            text = replace_once(text, '#endif /* lint */', '''
void
release_owned_regexes (void)
{
  while (owned_regexes)
    {
      struct owned_regex *owner = owned_regexes;
      owned_regexes = owner->next;
      if (owner->value)
        release_regex (owner->value);
      free (owner);
    }
}
''')
        elif name == 'utils':
            text = replace_once(text, '  p = xmalloc (sizeof *p);', '  p = xzalloc (sizeof *p);')
            text = replace_once(text, '  p->name = xstrdup (name);\n  p->fp = fp;',
                                '  p->fp = fp;\n  p->name = xstrdup (name);')
            text += '''
/* Diagnostic exits keep their original status and message.  Only still-owned
   streams are closed here; ck_fclose already checks normal output errors. */
void
release_owned_streams (void)
{
  int saved_errno = errno;
  while (open_files)
    {
      struct open_file *file = open_files;
      open_files = file->link;
      if (file->fp)
        fclose (file->fp);
      free (file->name);
      free (file);
    }
  errno = saved_errno;
}
'''
        else:
            text = replace_once(text, 'static void\nsetup_replacement (', '''struct owned_replacement
{
  char *value;
  struct owned_replacement *next;
};
static struct owned_replacement *owned_replacements;
static idx_t *owned_transliteration_lengths;

static void
setup_replacement (''')
            text = replace_once(text, '  base = MEMDUP (text, length, char);', '''  struct owned_replacement *owner = xzalloc (sizeof *owner);
  owner->next = owned_replacements;
  owned_replacements = owner;
  base = MEMDUP (text, length, char);
  owner->value = base;''')
            anchor = '                idx_t *src_lens = XNMALLOC (len, idx_t);'
            text = replace_once(text, anchor, anchor+'\n                owned_transliteration_lengths = src_lens;')
            text = replace_once(text, '                IF_LINT (free (src_lens));',
                                '                free (src_lens);\n                owned_transliteration_lengths = NULL;')
            text += '''
/* Replacement nodes borrow slices.  Retain each allocation base, including
   malloc(0) results for empty replacements, until command execution ends. */
void
release_owned_replacements (void)
{
  free (owned_transliteration_lengths);
  owned_transliteration_lengths = NULL;
  while (owned_replacements)
    {
      struct owned_replacement *owner = owned_replacements;
      owned_replacements = owner->next;
      free (owner->value);
      free (owner);
    }
}
'''
        adapted = stage/(name+'.c')
        adapted.write_text(text)
        selected = [r for r in records if Path(r['file']) == original]
        assert len(selected) == 1
        record = selected[0]
        arguments = record['arguments'].copy()
        output = stage/('sed-'+name+'.o')
        arguments[arguments.index('-o')+1] = str(output)
        arguments[arguments.index(str(original))] = str(adapted)
        arguments += ['-I'+str(original.parent)]
        log = root/f'evidence/raw/sed-{name}-cleanup-build.log'
        with log.open('w') as out:
            subprocess.run(arguments, cwd=record['directory'], stdout=out, stderr=subprocess.STDOUT, check=True)
        outputs['sed-'+name] = output
        evidence[name] = {'original_sha256': fingerprint(original), 'adapted_source_sha256': fingerprint(adapted),
                          'object_sha256': fingerprint(output), 'compiler_arguments': arguments,
                          'log': str(log.relative_to(root)), 'log_sha256': fingerprint(log)}
    (root/'evidence/sed-native-cleanup.json').write_text(json.dumps({
        'scope': 'Track regex and replacement allocation bases, release temporary multibyte transliteration lengths on completion or diagnostics, invoke GNU regex destructors, and close remaining owned streams at exit.',
        'driver_sha256': fingerprint(Path(__file__)), 'helpers': evidence}, indent=2)+'\n')
    return outputs
