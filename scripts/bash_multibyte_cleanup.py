"""Initialize converted delimiter lists and release quoted diagnostic names."""
# SPDX-License-Identifier: GPL-3.0-or-later
import json,subprocess
from pathlib import Path
from entry_provider_helpers import fingerprint

def prepare(root,adapted):
    stage=root/'build/bash-multibyte-cleanup';stage.mkdir(exist_ok=True)
    files=[]
    for profile_name,filename in [('bash-parameter-cleanup','subst.c'),
                                  ('bash-redirection-command-cleanup','execute_cmd.c')]:
        profile=json.loads((root/f'evidence/{profile_name}.json').read_text())
        record,=[r for r in profile['files'] if Path(r['source']).name==filename]
        original=Path(record['source']);assert fingerprint(original)==record['source_sha256']
        text=original.read_text()
        if filename=='subst.c':
            old='\t\t  mbstowcs (wcharlist, charlist, len + 1);'
            new='''                  /* A failed sizing conversion above deliberately leaves an
                     empty wide delimiter set. Always terminate that set. */
                  if (len)
                    mbstowcs (wcharlist, charlist, len);
                  wcharlist[len] = L'\\0';'''
        else:
            old='''\t      pathname = printable_filename (pathname, 0);
\t      internal_error ("%s: %s", pathname, notfound_str);'''
            new='''              char *rboxc_display_name = printable_filename (pathname, 0);
              internal_error ("%s: %s", rboxc_display_name, notfound_str);
              if (rboxc_display_name != pathname)
                free (rboxc_display_name);'''
        assert text.count(old)==1
        source=stage/filename;source.write_text(text.replace(old,new))
        obj=source.with_suffix('.o');log=source.with_suffix('.c.log')
        command=record['compile_arguments'].copy()
        command[command.index(str(original))]=str(source)
        command[command.index('-o')+1]=str(obj)
        with log.open('w') as out:
            subprocess.run(command,cwd=record['directory'],stdout=out,stderr=subprocess.STDOUT,check=True)
        adapted[obj.name]=obj
        files.append({'original':str(original),'original_sha256':fingerprint(original),
            'source':str(source),'source_sha256':fingerprint(source),'object':str(obj),'object_sha256':fingerprint(obj),
            'compile_arguments':command,'directory':record['directory'],'log':str(log),'log_sha256':fingerprint(log)})
    (root/'evidence/bash-multibyte-cleanup.json').write_text(json.dumps({
        'scope':'Terminate the wide delimiter set explicitly, including the existing zero-length conversion fallback. Release allocated printable command names after reporting a missing command while preserving borrowed names. Original diagnostic text, byte matching and exit status remain unchanged.',
        'driver_sha256':fingerprint(Path(__file__)),'files':files},indent=2)+'\n')
