"""Retain explicit ownership of iconv's native charmap helpers."""
# SPDX-License-Identifier: GPL-3.0-or-later
import json,re,subprocess
from pathlib import Path
from entry_provider_helpers import fingerprint

def once(text,old,new):
    assert text.count(old)==1,old[:80]
    return text.replace(old,new)

def adapt(filename,text):
    if filename=='linereader.c':
        text=once(text,'  free (lr->buf);\n  free (lr);','  free (lr->buf);\n  free ((char *) lr->fname);\n  free (lr);')
        old="\t  lr->token.val.ucs4 = strtoul (lrb.buf + 1, NULL, 16);\n\n\t  return &lr->token;"
        return once(text,old,old.replace('\n\n\t  return','\n          free (lrb.buf);\n\n\t  return'))
    if filename=='charmap.c':
        anchor='bool enc_not_ascii_compatible;'
        owners='''

/* These maps belong to this isolated iconv command. Their names and character
   entries borrow storage from the map's obstacks and hash pools. */
struct rboxc_map_owner
{
  struct charmap_t *map;
  struct rboxc_map_owner *next;
};
static struct rboxc_map_owner *rboxc_maps;
static int rboxc_maps_registered;

static void
rboxc_release_maps (void)
{
  while (rboxc_maps != NULL)
    {
      struct rboxc_map_owner *owner = rboxc_maps;
      struct charmap_t *map = owner->map;
      rboxc_maps = owner->next;
      if (map->char_table.table != NULL) delete_hash (&map->char_table);
      if (map->byte_table.table != NULL) delete_hash (&map->byte_table);
      if (map->ucs4_table.table != NULL) delete_hash (&map->ucs4_table);
      obstack_free (&map->mem_pool, NULL);
      free (map);
      free (owner);
    }
}

/* The parser copies needed strings into its map before requesting a token.
   Numeric and error tokens may contain borrowed pointers and are not owners. */
static struct token *
rboxc_charmap_token (struct linereader *reader, int verbose, char **owner)
{
  struct token *token;
  free (*owner);
  *owner = NULL;
  token = lr_token (reader, NULL, NULL, NULL, verbose);
  if (token->tok == tok_ident || token->tok == tok_bsymbol
      || token->tok == tok_string)
    *owner = token->val.str.startmb;
  return token;
}
'''
        text=once(text,anchor,anchor+owners)
        # Register the initialized map before parsing its body. The parser's
        # normal result and ordinary fatal exits share the same final owner.
        anchor='  /* We use a state machine to describe the charmap description file'
        setup='''  if (!rboxc_maps_registered)
    {
      if (atexit (rboxc_release_maps) != 0)
        record_error (4, 0, "cannot register charmap cleanup");
      rboxc_maps_registered = 1;
    }
  struct rboxc_map_owner *rboxc_owner = xmalloc (sizeof (*rboxc_owner));
  rboxc_owner->map = result;
  rboxc_owner->next = rboxc_maps;
  rboxc_maps = rboxc_owner;

'''
        text=once(text,anchor,setup+anchor)
        text=once(text,'  int step = 1;','  int step = 1;\n  char *rboxc_token_owner = NULL;')
        old='lr_token (cmfile, NULL, NULL, NULL, verbose)';assert text.count(old)==2
        text=text.replace(old,'rboxc_charmap_token (cmfile, verbose, &rboxc_token_owner)')
        return once(text,'  lr_close (cmfile);','  free (rboxc_token_owner);\n  lr_close (cmfile);')
    assert filename=='iconv_charmap.c'
    text=once(text,'  } val[256];\n};','  } val[256];\n  struct convtable *rboxc_owner_next;\n};')
    start=text.index('static inline struct convtable *\nallocate_table')
    end=text.index('static inline int\nis_term',start)
    text=text[:start]+'''/* All table nodes belong to one command conversion, including nodes whose
   mappings are shadowed. Converted outputs are owned only in the from-map
   path; other terminal values borrow entries from a charmap. */
struct rboxc_output_owner
{
  struct charseq *value;
  struct rboxc_output_owner *next;
};
static struct convtable *rboxc_tables;
static struct rboxc_output_owner *rboxc_outputs;
static char *rboxc_input_buffer;
static size_t rboxc_input_capacity;
static int rboxc_tables_registered;

static void
rboxc_release_tables (void)
{
  while (rboxc_tables != NULL)
    {
      struct convtable *node = rboxc_tables;
      rboxc_tables = node->rboxc_owner_next;
      free (node);
    }
  while (rboxc_outputs != NULL)
    {
      struct rboxc_output_owner *owner = rboxc_outputs;
      rboxc_outputs = owner->next;
      free (owner->value);
      free (owner);
    }
  free (rboxc_input_buffer);
  rboxc_input_buffer = NULL;
  rboxc_input_capacity = 0;
}

static inline struct convtable *
allocate_table (void)
{
  struct convtable *table;
  if (!rboxc_tables_registered)
    {
      if (atexit (rboxc_release_tables) != 0)
        error (EXIT_FAILURE, 0, "cannot register conversion-table cleanup");
      rboxc_tables_registered = 1;
    }
  table = xcalloc (1, sizeof (*table));
  table->rboxc_owner_next = rboxc_tables;
  rboxc_tables = table;
  return table;
}

static inline void
free_table (struct convtable *tbl)
{
  (void) tbl;
  rboxc_release_tables ();
}


'''+text[end:]
    text=once(text,'\t    (struct convtable *) xcalloc (1, sizeof (struct convtable));','            allocate_table ();')
    old='''      struct charseq *in = data;
      struct charseq *newp = convert_charseq (cd, in);'''
    new='''      struct charseq *in = data;
      struct rboxc_output_owner *owner = xmalloc (sizeof (*owner));
      owner->value = NULL;
      owner->next = rboxc_outputs;
      rboxc_outputs = owner;
      struct charseq *newp = convert_charseq (cd, in);
      owner->value = newp;'''
    text=once(text,old,new)
    start=text.index('static int\nprocess_fd (');end=text.index('static int\nprocess_file (',start)
    body=text[start:end]
    body=once(body,'  static char *inbuf = NULL;\n  static size_t maxlen = 0;\n','')
    body=re.sub(r'\binbuf\b','rboxc_input_buffer',body)
    body=re.sub(r'\bmaxlen\b','rboxc_input_capacity',body)
    return text[:start]+body+text[end:]

def prepare(root,adapted):
    stage=root/'build/iconv-charmap-cleanup';stage.mkdir(exist_ok=True)
    records=[(p,json.loads(p.read_text())) for p in (root/'build/glibc-cc-records').glob('*.json')]
    files=[]
    for filename in ('charmap.c','linereader.c','iconv_charmap.c'):
        native=root/'build/gnu-glibc/iconv'/Path(filename).with_suffix('.o')
        record_path,record=next((p,r) for p,r in records if r.get('kind')=='compile' and r.get('output')==str(native))
        original=Path(record['file']);source=stage/filename
        source.write_text(adapt(filename,original.read_text()))
        obj=source.with_suffix('.o');log=source.with_suffix('.c.log')
        command=record['arguments'].copy()
        assert (Path(record['directory'])/command[1]).resolve()==original
        command[1]=str(source)
        for flag,value in [('-o',obj),('-MF',source.with_suffix('.d')),('-MT',obj)]:command[command.index(flag)+1]=str(value)
        command+=['-iquote',str(original.parent)]
        with log.open('w') as out:subprocess.run(command,cwd=record['directory'],stdout=out,stderr=subprocess.STDOUT,check=True)
        adapted[obj.name]=obj
        files.append({'original':str(original),'original_sha256':fingerprint(original),
            'source':str(source),'source_sha256':fingerprint(source),'object':str(obj),'object_sha256':fingerprint(obj),
            'native_object':str(native),'native_object_sha256':fingerprint(native),
            'record':str(record_path),'record_sha256':fingerprint(record_path),
            'compile_arguments':command,'directory':record['directory'],'log':str(log),'log_sha256':fingerprint(log)})
    (root/'evidence/iconv-charmap-cleanup.json').write_text(json.dumps({
        'scope':'Command-owned charmap pools and hashes retain exit cleanup. Parser strings are released after their copied values have been consumed; Unicode-symbol scratch buffers and reader filenames are released at final use. Conversion nodes and allocated from-map output sequences retain explicit owners; borrowed terminal values remain owned by their maps. The existing input buffer survives repeated input reads and is released after the conversion. GNU mapping, parsing and output logic remains intact.',
        'driver_sha256':fingerprint(Path(__file__)),'files':files},indent=2)+'\n')
