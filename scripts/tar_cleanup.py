"""Release GNU Tar's owned directory handles and consumed name records."""
# SPDX-License-Identifier: GPL-3.0-or-later
import json
from pathlib import Path
import shutil
import subprocess
from tar_helpers import fingerprint


def prepare(root):
    pin = json.loads((root/'inventory/sources.json').read_text())['tar']
    stage = root/'build/tar-cleanup'
    stage.mkdir(exist_ok=True)
    records = [json.loads(p.read_text()) for p in (root/'build/tar-cc-records').glob('*.json')]
    outputs = {}
    evidence = []
    for name in ('misc', 'names', 'compare', 'wordsplit'):
        relative = ('lib/' if name == 'wordsplit' else 'src/')+name+'.c'
        original = Path(pin['source'])/relative
        expected = pin['native_cleanup_source_sha256'][relative] if name == 'wordsplit' else pin['helper_source_sha256'][relative]
        assert fingerprint(original) == expected
        text = original.read_text()

        def replace(before, after, count=1):
            nonlocal text
            assert text.count(before) == count, before
            text = text.replace(before, after)

        if name == 'misc':
            replace('static size_t wdcache_count;', '''static size_t wdcache_count;

static bool rboxc_wd_cleanup_registered;
static void
rboxc_release_working_directories (void)
{
  int saved_errno = errno;
  for (size_t i = 0; i < wd_count; i++)
    {
      if (wd[i].fd > 0)
        close (wd[i].fd);
      free (wd[i].abspath);
      free ((void *) wd[i].owned_name);
    }
  free (wd);
  wd = NULL;
  wd_count = wd_alloc = wdcache_count = 0;
  errno = saved_errno;
}''')
            replace('  char *abspath;\n', '  char *abspath;\n  const char *owned_name;\n')
            replace('chdir_arg (char const *dir)\n{', '''chdir_arg (char const *dir)
{
  /* Every pinned caller transfers an xstrdup allocation.  Keep its base
     separately because GNU normalizes DIR by advancing that pointer. */
  const char *owned_dir = dir;
  if (!rboxc_wd_cleanup_registered)
    {
      if (atexit (rboxc_release_working_directories))
        xalloc_die ();
      rboxc_wd_cleanup_registered = true;
    }''')
            replace('  wd[wd_count].name = ".";', '  wd[wd_count].owned_name = NULL;\n\t  wd[wd_count].name = ".";')
            replace("      if (! dir[dir[0] == '.'])\n\treturn wd_count - 1;", """      if (! dir[dir[0] == '.'])
        {
          free ((void *) owned_dir);
          return wd_count - 1;
        }""")
            replace('  wd[wd_count].name = dir;', '  wd[wd_count].owned_name = owned_dir;\n  wd[wd_count].name = dir;')
            names_source = Path(pin['source'])/'src/names.c'
            assert fingerprint(names_source) == pin['helper_source_sha256']['src/names.c']
            assert names_source.read_text().count('chdir_arg (xstrdup (ep->v.name))') == 3
        elif name == 'wordsplit':
            # The completed word has already been copied into WS_WORDV.
            replace('      wsnode_remove (wsp, wsp->ws_head);', '''      struct wordsplit_node *completed = wsp->ws_head;
      wsnode_remove (wsp, completed);
      wsnode_free (completed);''')
        elif name == 'compare':
            replace('static char *diff_buffer;', '''static char *diff_buffer;
static void *rboxc_diff_allocation;
static bool rboxc_diff_cleanup_registered;
static void
rboxc_release_diff_buffer (void)
{
  int saved_errno = errno;
  free (rboxc_diff_allocation);
  rboxc_diff_allocation = NULL;
  diff_buffer = NULL;
  errno = saved_errno;
}''')
            replace('''  void *ptr;
  diff_buffer = page_aligned_alloc (&ptr, record_size);''', '''  if (!rboxc_diff_cleanup_registered)
    {
      if (atexit (rboxc_release_diff_buffer))
        xalloc_die ();
      rboxc_diff_cleanup_registered = true;
    }
  rboxc_release_diff_buffer ();
  diff_buffer = page_aligned_alloc (&rboxc_diff_allocation, record_size);''')
        else:
            replace('static struct name *\nmake_name (const char *file_name)', '''/* Keep ownership independent of GNU's selection-list cursors. */
struct rboxc_owned_name
{
  struct name value;
  struct rboxc_owned_name *next;
  struct rboxc_owned_name **previous_link;
};
static struct rboxc_owned_name *rboxc_owned_names;
static bool rboxc_name_cleanup_registered;
static void free_name (struct name *p);
static void
rboxc_release_names (void)
{
  int saved_errno = errno;
  while (rboxc_owned_names)
    free_name (&rboxc_owned_names->value);
  errno = saved_errno;
}

static struct name *
make_name (const char *file_name)''')
            replace('  struct name *p = xzalloc (sizeof (*p));', '''  if (!rboxc_name_cleanup_registered)
    {
      if (atexit (rboxc_release_names))
        xalloc_die ();
      rboxc_name_cleanup_registered = true;
    }
  struct rboxc_owned_name *owned = xzalloc (sizeof (*owned));
  owned->next = rboxc_owned_names;
  owned->previous_link = &rboxc_owned_names;
  if (owned->next)
    owned->next->previous_link = &owned->next;
  rboxc_owned_names = owned;
  struct name *p = &owned->value;''')
            replace('''  if (p)
    {
      free (p->name);''', '''  if (p)
    {
      struct rboxc_owned_name *owned = (struct rboxc_owned_name *) p;
      *owned->previous_link = owned->next;
      if (owned->next)
        owned->next->previous_link = owned->previous_link;
      free (p->name);''')
            replace('''      if (subcommand_option == CREATE_SUBCOMMAND
	  || subcommand_option == UPDATE_SUBCOMMAND)
	unconsumed_option_push (elt);''', '''      if (subcommand_option == CREATE_SUBCOMMAND
	  || subcommand_option == UPDATE_SUBCOMMAND)
	unconsumed_option_push (elt);
      else
        free (elt);''')
        adapted = stage/(name+'.c')
        adapted.write_text(text)
        matching = [r for r in records if Path(r['file']) == original]
        assert len(matching) == 1
        record = matching[0]
        arguments = record['arguments'].copy()
        output = stage/(name+'.o')
        arguments[arguments.index('-o')+1] = str(output)
        arguments[arguments.index(str(original))] = str(adapted)
        arguments += ['-I'+str(original.parent)]
        log = root/'evidence/raw'/('tar-'+name+'-cleanup-build.log')
        with log.open('w') as stream:
            subprocess.run(arguments, cwd=record['directory'], stdout=stream, stderr=subprocess.STDOUT, check=True)
        outputs[name+'.o'] = output
        evidence.append({'source': str(original), 'original_sha256': fingerprint(original),
                         'adapted_source': str(adapted.relative_to(root)), 'adapted_source_sha256': fingerprint(adapted),
                         'object': str(output.relative_to(root)), 'object_sha256': fingerprint(output),
                         'compiler_arguments': arguments, 'log': str(log.relative_to(root)), 'log_sha256': fingerprint(log)})
    archive = stage/'libtar.a'
    shutil.copy2(root/'build/gnu-tar/lib/libtar.a', archive)
    assert subprocess.check_output(['ar', 't', archive], text=True).splitlines().count('wordsplit.o') == 1
    subprocess.run(['ar', 'r', archive, outputs.pop('wordsplit.o')], check=True)
    subprocess.run(['ranlib', archive], check=True)
    outputs['libtar.a'] = archive
    report = {'scope': 'Keep GNU selection and directory behavior, while tracking live name allocations independently of discarded selection cursors, releasing consumed directory/option records, closing/freeing owned working-directory state, retaining/freeing the aligned comparison allocation, and freeing consumed wordsplit nodes after copying their output. Native oracle objects remain unchanged.',
              'driver_sha256': fingerprint(Path(__file__)), 'adaptations': evidence}
    (root/'evidence/tar-native-cleanup.json').write_text(json.dumps(report, indent=2)+'\n')
    return outputs
