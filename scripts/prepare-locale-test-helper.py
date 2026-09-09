#!/usr/bin/env python3
"""Build a source-pinned native locale test helper with enumeration cleanup."""
# SPDX-License-Identifier: GPL-3.0-or-later
import hashlib
import json
from pathlib import Path
import subprocess
ROOT=Path(__file__).resolve().parents[1]
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
def adapt(text):
 old='#define PUT(name) tsearch (name, &all_data, \\\n\t\t\t   (int (*) (const void *, const void *)) strcoll)'
 new='''#define PUT(name) do { \\
  char *owned_name = (name); \\
  char **node = tsearch (owned_name, &all_data, \\
                        (int (*) (const void *, const void *)) strcoll); \\
  if (node == NULL || *node != owned_name) free (owned_name); \\
} while (0)'''
 assert text.count(old)==1;text=text.replace(old,new)
 for name in ['C','POSIX']:
  old='PUT ("'+name+'");';assert text.count(old)==1;text=text.replace(old,'PUT (xstrdup ("'+name+'"));')
 old='\t  PUT (xstrdup (dirents[cnt]->d_name));\n\t}\n    }\n  if (ndirents > 0)'
 new='\t  PUT (xstrdup (dirents[cnt]->d_name));\n\t}\n      free (dirents[cnt]);\n    }\n  if (ndirents >= 0)'
 assert text.count(old)==1;text=text.replace(old,new)
 old='''  if (! verbose)
    {
      twalk (all_data, print_names);
    }
}'''
 new='''  free (alias_path);
  if (! verbose)
    {
      twalk (all_data, print_names);
    }
  tdestroy (all_data, free);
}'''
 assert text.count(old)==1;text=text.replace(old,new)
 old='  twalk (all_data, print_names);\n}'
 assert text.count(old)==1;text=text.replace(old,'  twalk (all_data, print_names);\n  tdestroy (all_data, free);\n}')
 return text
if __name__=='__main__':
 stage=ROOT/'build/locale-test-helper';stage.mkdir(exist_ok=True)
 source=Path('/opt/src/glibc-2.43/locale/programs/locale.c')
 record_path=ROOT/'build/glibc-cc-records/58dddf789b9c99243710328e3b533c98bd3186f754bef0f727b8fc0caecc2bd1.json'
 record=json.loads(record_path.read_text());assert record['file']==str(source)
 objects=[ROOT/'build/gnu-glibc/locale'/(n+'.o') for n in ['locale-spec','charmap-dir','record-status','simple-hash','xasprintf','xmalloc','xstrdup']]
 configuration={'COMPLOCALEDIR':'/usr/lib/locale','LOCALE_ALIAS_PATH':'/usr/share/locale','LOCALE_PATH':'/usr/lib/locale:/usr/share/i18n','CHARMAP_PATH':'/usr/share/i18n/charmaps','REPERTOIREMAP_PATH':'/usr/share/i18n/repertoiremaps','LOCSRCDIR':'/usr/share/i18n/locales'}
 builds={}
 for name in ['original','cleanup']:
  destination=stage/name;destination.mkdir(exist_ok=True)
  src=destination/'locale.c';src.write_text(source.read_text() if name=='original' else adapt(source.read_text()))
  obj=destination/'locale.o';binary=destination/'locale'
  command=record['arguments'][:record['arguments'].index('-MD')]
  command[1]=str(src);command[command.index('-o')+1]=str(obj)
  for key,value in configuration.items():
   index,=[i for i,a in enumerate(command) if a.startswith('-D'+key+'=')];command[index]='-D'+key+'="'+value+'"'
  command+=['-iquote',str(source.parent)]
  link=['gcc','-o',str(binary),str(obj),*map(str,objects)]
  log=destination/'build.log'
  with log.open('w') as out:
   subprocess.run(command,cwd=record['directory'],stdout=out,stderr=subprocess.STDOUT,check=True)
   subprocess.run(link,stdout=out,stderr=subprocess.STDOUT,check=True)
  builds[name]={'source':str(src),'source_sha256':sha(src),'object':str(obj),'object_sha256':sha(obj),'binary':str(binary),'binary_sha256':sha(binary),'compile':command,'link':link,'log':str(log),'log_sha256':sha(log)}
 report={'scope':'Native test dependency only, not an integrated applet. GNU glibc 2.43 locale enumeration retains output while releasing directory entries, owned tree keys/nodes and alias-path storage; duplicate tree keys are freed. Host data paths configured identically for both builds.',
 'driver_sha256':sha(__file__),'original_source':str(source),'original_sha256':sha(source),'record':str(record_path),'record_sha256':sha(record_path),'directory':record['directory'],'configuration':configuration,'helper_objects':{str(p):sha(p) for p in objects},'builds':builds}
 (ROOT/'evidence/locale-test-helper-build.json').write_text(json.dumps(report,indent=2)+'\n')
 print('Built original and cleanup native locale test helpers')
