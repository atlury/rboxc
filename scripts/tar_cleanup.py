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
    for name in ('misc', 'names', 'compare', 'wordsplit', 'incremen', 'buffer', 'map', 'system'):
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
        elif name == 'system':
            # Child stdin is a replacement pipe, not an inherited standard
            # stream. Only its owning process finalizes it on a non-exec exit.
            replace('      xclose (from);\n    }\n}\n\n/* Propagate',
                    '      xclose (from);\n      if (into == STDIN_FILENO)\n        rboxc_record_child_input ();\n    }\n}\n\n/* Propagate')
            anchor = 'static _Noreturn void\nxexec (const char *cmd)'
            replace(anchor, '''static pid_t rboxc_input_owner;
static struct stat rboxc_input_identity;
static bool rboxc_input_active;
static bool rboxc_input_cleanup_registered;

static void
rboxc_release_child_input (void)
{
  int saved_errno = errno;
  struct stat current;
  if (rboxc_input_active && rboxc_input_owner == getpid ()
      && fstat (STDIN_FILENO, &current) == 0
      && current.st_dev == rboxc_input_identity.st_dev
      && current.st_ino == rboxc_input_identity.st_ino
      && current.st_mode == rboxc_input_identity.st_mode
      && current.st_rdev == rboxc_input_identity.st_rdev)
    close (STDIN_FILENO);
  rboxc_input_active = false;
  errno = saved_errno;
}

static void
rboxc_record_child_input (void)
{
  int saved_errno = errno;
  rboxc_input_active = fstat (STDIN_FILENO, &rboxc_input_identity) == 0;
  rboxc_input_owner = getpid ();
  if (!rboxc_input_cleanup_registered)
    {
      if (atexit (rboxc_release_child_input))
        xalloc_die ();
      rboxc_input_cleanup_registered = true;
    }
  errno = saved_errno;
}

'''+anchor)
        elif name == 'map':
            # getline owns this buffer even for an empty map; parsed names
            # were duplicated into the table before its lifetime ends.
            replace('  fclose (fp);', '  free (buf);\n  fclose (fp);')
        elif name == 'buffer':
            # A newly created update archive can contain no bytes.  Initialize
            # the probe block before reading; short reads still retain every
            # actual byte for GNU's compression signature checks.
            replace('  record_end = record_start; /* set up for 1st record = # 0 */\n  sfr = read_full_records;',
                    '  record_end = record_start; /* set up for 1st record = # 0 */\n  memset (record_start, 0, BLOCKSIZE);\n  sfr = read_full_records;')
            # GNU's archive handle is assigned from standard streams, owned
            # local opens/pipes, or remote handles. Invalidate it at every
            # explicit close so an exit callback cannot close a reused fd.
            replace('rmtclose (archive)', 'rboxc_close_archive_handle ()', count=4)
            anchor = 'static struct tar_stat_info dummy;'
            replace(anchor, anchor+'''
static bool rboxc_archive_cleanup_registered;
static int
rboxc_close_archive_handle (void)
{
  int fd = archive;
  archive = -1;
  return rmtclose (fd);
}

static void
rboxc_release_local_archive (void)
{
  int saved_errno = errno;
  if (archive > STDERR_FILENO && !_isrmt (archive))
    rboxc_close_archive_handle ();
  errno = saved_errno;
}
''')
            replace('open_archive (enum access_mode wanted_access)\n{\n  flush_read_ptr', '''open_archive (enum access_mode wanted_access)
{
  if (!rboxc_archive_cleanup_registered)
    {
      if (atexit (rboxc_release_local_archive))
        xalloc_die ();
      rboxc_archive_cleanup_registered = true;
    }
  flush_read_ptr''')
            # This local header owns its formatted name and account strings.
            # file_name aliases orig_file_name, unlike normal member records.
            replace('''      simple_finish_header (write_extended (false, &st, blk));
      free (st.orig_file_name);''', '''      simple_finish_header (write_extended (false, &st, blk));
      st.file_name = NULL;
      tar_stat_destroy (&st);''')
            # The PAX continuation header's strings/xhdr belong to dummy;
            # volume decoders copy their values into independent globals.
            # Release the temporary metadata on every volume-read return.
            replace('try_new_volume (void)\n{', 'rboxc_try_new_volume_inner (void)\n{')
            anchor = '#define VOLUME_TEXT " Volume "'
            replace(anchor, '''static bool
try_new_volume (void)
{
  bool result = rboxc_try_new_volume_inner ();
  int saved_errno = errno;
  tar_stat_destroy (&dummy);
  errno = saved_errno;
  return result;
}

'''+anchor)
        elif name == 'incremen':
            anchor = 'static void\nread_incr_db_2 (void)'
            replace(anchor, '''static struct obstack rboxc_snapshot_obstack;
static bool rboxc_snapshot_active;
static bool rboxc_snapshot_cleanup_registered;
static void
rboxc_release_snapshot_obstack (void)
{
  int saved_errno = errno;
  if (rboxc_snapshot_active)
    {
      obstack_free (&rboxc_snapshot_obstack, NULL);
      rboxc_snapshot_active = false;
    }
  errno = saved_errno;
}

'''+anchor)
            start = text.index(anchor)
            end = text.index('\n/* Display (to stdout)', start)
            part = text[start:end]
            assert part.count('  struct obstack stk;') == 1
            part = part.replace('  struct obstack stk;',
                                '  struct obstack *stk = &rboxc_snapshot_obstack;')
            part = part.replace('&stk', 'stk')
            before = '  obstack_init (stk);'
            assert part.count(before) == 1
            part = part.replace(before, '''  if (!rboxc_snapshot_cleanup_registered)
    {
      if (atexit (rboxc_release_snapshot_obstack))
        xalloc_die ();
      rboxc_snapshot_cleanup_registered = true;
    }
  rboxc_release_snapshot_obstack ();
  obstack_init (stk);
  rboxc_snapshot_active = true;''')
            before = '\treturn; /* Normal return */'
            assert part.count(before) == 1
            # note_directory copies both the name and dump contents. Release
            # the complete workspace only once all records have been read.
            part = part.replace(before, '''        {
          rboxc_release_snapshot_obstack ();
          return; /* Normal return */
        }''')
            text = text[:start]+part+text[end:]
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
            anchor = 'static int\nhandle_option (const char *str, struct name_elt const *ent)'
            replace(anchor, '''struct rboxc_file_options
{
  struct wordsplit words;
  struct rboxc_file_options *next;
};
static struct rboxc_file_options *rboxc_owned_file_options;
static bool rboxc_file_options_cleanup_registered;
static void
rboxc_release_file_options (void)
{
  int saved_errno = errno;
  while (rboxc_owned_file_options)
    {
      struct rboxc_file_options *p = rboxc_owned_file_options;
      rboxc_owned_file_options = p->next;
      wordsplit_free (&p->words);
      wordsplit_clearerr (&p->words);
      free (p);
    }
  errno = saved_errno;
}

'''+anchor)
            start = text.index(anchor)
            end = text.index('\nstatic int\nread_next_name', start)
            part = text[start:end]
            assert part.count('  struct wordsplit ws;') == 1
            part = part.replace('  struct wordsplit ws;', '  struct wordsplit *ws;')
            before = '  ws.ws_offs = 1;'
            assert part.count(before) == 1
            part = part.replace(before, '''  if (!rboxc_file_options_cleanup_registered)
    {
      if (atexit (rboxc_release_file_options))
        xalloc_die ();
      rboxc_file_options_cleanup_registered = true;
    }
  struct rboxc_file_options *owned = xzalloc (sizeof (*owned));
  owned->next = rboxc_owned_file_options;
  rboxc_owned_file_options = owned;
  ws = &owned->words;
  ws.ws_offs = 1;''')
            before = '''  for (i = 0; i < ws.ws_wordc+ws.ws_offs; i++)
    ws.ws_wordv[i] = NULL;

  wordsplit_free (&ws);'''
            assert part.count(before) == 1
            part = part.replace(before, '  /* GNU option state borrows these words until command completion. */')
            part = part.replace('  int i;\n', '')
            part = part.replace('ws.', 'ws->').replace('&ws', 'ws')
            text = text[:start]+part+text[end:]
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
    report = {'scope': 'Keep GNU selection and directory behavior, while tracking live name allocations independently of discarded selection cursors, releasing consumed directory/option records, closing/freeing owned working-directory state, retaining/freeing the aligned comparison allocation, freeing consumed wordsplit nodes after copying their output, retaining/freeing option-file words after GNU finishes borrowing them, and releasing the incremental snapshot parser obstack on return or process exit. Map input line buffers are freed after parsing, and compression probe headers start initialized before reading an empty or partial archive. Native oracle objects remain unchanged.',
              'driver_sha256': fingerprint(Path(__file__)), 'adaptations': evidence}
    (root/'evidence/tar-native-cleanup.json').write_text(json.dumps(report, indent=2)+'\n')
    return outputs
