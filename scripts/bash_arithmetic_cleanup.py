"""Keep arithmetic caller strings owned across nested index diagnostics."""
# SPDX-License-Identifier: GPL-3.0-or-later
import json, subprocess
from pathlib import Path
from entry_provider_helpers import fingerprint


def prepare(root, adapted):
    assignment = json.loads((root/'evidence/bash-assignment-cleanup.json').read_text())
    subshell = json.loads((root/'evidence/bash-subshell-cleanup.json').read_text())
    stage = root/'build/bash-arithmetic-cleanup'
    stage.mkdir(exist_ok=True)
    reports = []
    for name in ('arrayfunc.c', 'execute_cmd.c', 'subst.c'):
        profile = subshell if name == 'execute_cmd.c' else assignment
        record, = [r for r in profile['files'] if Path(r['source']).name == name]
        original = Path(record['source'])
        assert fingerprint(original) == record['source_sha256']
        text = original.read_text()

        def replace(old, new):
            nonlocal text
            assert text.count(old) == 1, (name, old)
            text = text.replace(old, new)

        if name == 'arrayfunc.c':
            anchor = '  entry = assign_array_element_internal (entry, name, vname, sub, sublen, value, flags, estatep);'
            replace(anchor, '''  begin_unwind_frame ("rboxc-array-element-name");
  add_unwind_protect (xfree, vname);
'''+anchor+'''
  discard_unwind_frame ("rboxc-array-element-name");''')
        elif name == 'execute_cmd.c':
            anchor = '      expresult = evalexp (exp, eflag, &expok);'
            replace(anchor, '''      begin_unwind_frame ("rboxc-arithmetic-command");
      add_unwind_protect (xfree, exp);
'''+anchor+'''
      discard_unwind_frame ("rboxc-arithmetic-command");''')
        else:
            anchor = '\t  number = evalexp (temp1, eflag, &expok);'
            replace(anchor, '''\t  begin_unwind_frame ("rboxc-arithmetic-substitution");
\t  add_unwind_protect (xfree, temp);
\t  add_unwind_protect (xfree, temp1);
'''+anchor+'''
\t  discard_unwind_frame ("rboxc-arithmetic-substitution");''')

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
    (root/'evidence/bash-arithmetic-cleanup.json').write_text(json.dumps({
        'scope':'Arithmetic command/substitution strings and array-element names register existing heap destructors while nested index evaluation can leave through a diagnostic. Normal evaluation transfers back to the original free calls; values and diagnostics are unchanged.',
        'driver_sha256':fingerprint(Path(__file__)), 'files':reports},indent=2)+'\n')
