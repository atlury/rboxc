"""Retain no-fork substitution descriptors and source strings through exits."""
# SPDX-License-Identifier: GPL-3.0-or-later
import json, subprocess
from pathlib import Path
from entry_provider_helpers import fingerprint


def prepare(root, adapted):
    baseline = json.loads((root/'evidence/bash-arithmetic-cleanup.json').read_text())
    stage = root/'build/bash-function-substitution-cleanup'
    stage.mkdir(exist_ok=True)
    reports = []
    for name in ('subst.c',):
        profile = baseline
        record, = [r for r in profile['files'] if Path(r['source']).name == name]
        original = Path(record['source'])
        assert fingerprint(original) == record['source_sha256']
        text = original.read_text()

        def replace(old, new):
            nonlocal text
            assert text.count(old) == 1, (name, old)
            text = text.replace(old, new)

        replace('#include "shell.h"', '#include "shell.h"\nextern void rboxc_bash_track_backup (int);')
        anchor = '  gs = sh_getopt_save_istate ();'
        replace(anchor, '  if (valsub == 0) rboxc_bash_track_backup (afd);\n\n'+anchor)
        anchor = '      saveout = stdout_valid ? move_to_high_fd (1, 1, -1) : -1;'
        replace(anchor, anchor+'\n      if (saveout >= 0) rboxc_bash_track_backup (saveout);')
        anchor = '\t      tdesc = function_substitute (temp, quoted, pflags&PF_ASSIGNRHS);'
        replace(anchor, '\t      begin_unwind_frame ("rboxc-function-substitution-source");\n\t      add_unwind_protect (xfree, temp);\n'+anchor+'\n\t      discard_unwind_frame ("rboxc-function-substitution-source");')

        source = stage/name
        source.write_text(text)
        obj = source.with_suffix('.o')
        command = record['compile_arguments'].copy()
        command[command.index(str(original))] = str(source)
        command[command.index('-o')+1] = str(obj)
        log = stage/(name+'.log')
        with log.open('w') as out:
            subprocess.run(command, cwd=record['directory'], stdout=out,
                           stderr=subprocess.STDOUT, check=True)
        adapted[obj.name] = obj
        reports.append({'original':str(original),'original_sha256':fingerprint(original),
            'source':str(source),'source_sha256':fingerprint(source),
            'object':str(obj),'object_sha256':fingerprint(obj),
            'compile_arguments':command,'directory':record['directory'],
            'log':str(log),'log_sha256':fingerprint(log)})
    (root/'evidence/bash-function-substitution-cleanup.json').write_text(json.dumps({
        'scope':'No-fork substitution anonymous files and saved stdout descriptors join the existing exit ownership registry; normal close still releases them at the original point. Its parsed source string registers the existing destructor across early exits. Normal results, variable scope and output restoration retain GNU behavior.',
        'driver_sha256':fingerprint(Path(__file__)), 'files':reports},indent=2)+'\n')
