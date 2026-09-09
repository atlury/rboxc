"""Release discarded recursive glob names and the added-current-directory node."""
# SPDX-License-Identifier: GPL-3.0-or-later
import json,os,shutil,subprocess
from pathlib import Path
from entry_provider_helpers import fingerprint


def prepare(root,adapted):
    pin=json.loads((root/'inventory/sources.json').read_text())['bash'];gnu=Path(pin['source']);name='lib/glob/glob.c';original=gnu/name
    assert fingerprint(original)==pin['source_and_header_sha256'][name]
    text=original.read_text()
    def replace(old,new):
        nonlocal text
        assert text.count(old)==1,old;text=text.replace(old,new)
    start=text.index('  if (add_current && lose == 0)');end=text.index('\n  if (lose == 0)',start)
    body=text[start:end];old='      else\n\t{\n\t  nextlink->name = nextname;';assert body.count(old)==1
    body=body.replace(old,'      else\n\t{\n\t  if (firstmalloc == 0) firstmalloc = nextlink;\n\t  nextlink->name = nextname;');text=text[:start]+body+text[end:]
    start=text.index('      if (all_starstar && filename[0]');end=text.index('\n      /* We have successfully',start)
    body=text[start:end];old='\t  free ((char *) directories);';assert body.count(old)==1
    body=body.replace(old,'\t  for (i = 0; directories[i]; i++) free (directories[i]);\n'+old);text=text[:start]+body+text[end:]
    old='\t\t      for (n = 0; temp_results[n] && *temp_results[n] == 0; n++)\n\t\t\t;'
    replace(old,old[:-1]+'free (temp_results[n]);')
    stage=root/'build/bash-globstar-cleanup';stage.mkdir(exist_ok=True);source=stage/'glob.c';source.write_text(text);obj=stage/'glob.o'
    records=[json.loads(p.read_text()) for p in (root/'build/bash-cc-records').glob('*.json')];matches=[r for r in records if r.get('file')==str(original)]
    if not matches:
        with (stage/'glob.record.log').open('w') as log:
            subprocess.run(['make','-C',str(root/'build/gnu-bash/lib/glob'),'-W',str(original),'glob.o','CC=python3 '+str(root/'scripts/record-provider-cc.py')],env={**os.environ,'RBOXC_CC_RECORDS':str(root/'build/bash-cc-records')},stdout=log,stderr=subprocess.STDOUT,check=True)
        matches=[json.loads(p.read_text()) for p in (root/'build/bash-cc-records').glob('*.json') if json.loads(p.read_text()).get('file')==str(original)]
    record,=matches;command=record['arguments'].copy();command[command.index(str(original))]=str(source)
    if '-o' in command:command[command.index('-o')+1]=str(obj)
    else:command+=['-o',str(obj)]
    command+=['-iquote',str(original.parent),'-iquote',str(gnu)]
    log=stage/'glob.c.log'
    with log.open('w') as out:subprocess.run(command,cwd=record['directory'],stdout=out,stderr=subprocess.STDOUT,check=True)
    archive=stage/'libglob.a';native=root/'build/gnu-bash/lib/glob/libglob.a';shutil.copyfile(native,archive)
    subprocess.run(['ar','r',str(archive),str(obj)],check=True);subprocess.run(['ranlib',str(archive)],check=True);adapted[archive.name]=archive
    (root/'evidence/bash-globstar-cleanup.json').write_text(json.dumps({'scope':'Retain GNU traversal and result ordering. Release directory names before discarding an already expanded directory vector, release empty placeholders before compacting a result vector, and register the heap list node used for the current-directory entry in the existing malloc cleanup chain.',
        'driver_sha256':fingerprint(Path(__file__)),'original':str(original),'original_sha256':fingerprint(original),'source':str(source),'source_sha256':fingerprint(source),'object':str(obj),'object_sha256':fingerprint(obj),'native_archive':str(native),'native_archive_sha256':fingerprint(native),'archive':str(archive),'archive_sha256':fingerprint(archive),'compile_arguments':command,'directory':record['directory'],'log':str(log),'log_sha256':fingerprint(log)},indent=2)+'\n')
