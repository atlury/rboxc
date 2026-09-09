"""Retain brace length and pattern callers across expansion diagnostics."""
# SPDX-License-Identifier: GPL-3.0-or-later
import json,subprocess
from pathlib import Path
from entry_provider_helpers import fingerprint

def prepare(root,adapted):
    profile=json.loads((root/'evidence/bash-exit-scope-cleanup.json').read_text())
    record,=[r for r in profile['files'] if Path(r['source']).name=='subst.c']
    original=Path(record['source']);assert fingerprint(original)==record['source_sha256']
    text=original.read_text()
    def replace(old,new):
        nonlocal text
        assert text.count(old)==1,old
        text=text.replace(old,new)
    anchor='      number = parameter_brace_expand_length (name);'
    replace(anchor,'''      begin_unwind_frame ("rboxc-brace-length-name");
      add_unwind_protect (xfree, name);
'''+anchor+'''
      discard_unwind_frame ("rboxc-brace-length-name");''')
    anchor='\t      if (valid_length_expression (newname))\n\t\tnumber = parameter_brace_expand_length (newname);'
    replace(anchor,'''              begin_unwind_frame ("rboxc-brace-length-reference");
              add_unwind_protect (xfree, newname);
'''+anchor+'''
              discard_unwind_frame ("rboxc-brace-length-reference");''')
    anchor='\t  list = expand_string (newname, Q_DOUBLE_QUOTES);'
    replace(anchor,'''          begin_unwind_frame ("rboxc-brace-length-expansion");
          add_unwind_protect (xfree, newname);
'''+anchor+'''
          discard_unwind_frame ("rboxc-brace-length-expansion");''')
    anchor='      temp1 = parameter_brace_remove_pattern (name, temp, &es, value, c, quoted, (tflag & W_ARRAYIND) ? AV_USEIND : 0);'
    replace(anchor,'''      begin_unwind_frame ("rboxc-brace-pattern-call");
      add_unwind_protect (xfree, name);
      add_unwind_protect (xfree, temp);
      add_unwind_protect (xfree, value);
'''+anchor+'''
      discard_unwind_frame ("rboxc-brace-pattern-call");''')
    anchor='  temp1 = savestring (patstr);\n  pattern = getpattern (temp1, quoted, 1);'
    replace(anchor,'''  temp1 = savestring (patstr);
  begin_unwind_frame ("rboxc-brace-pattern-value");
  add_unwind_protect (xfree, temp1);
  /* Whole arrays and positional parameters are borrowed, not allocations. */
  if (vtype == VT_VARIABLE || vtype == VT_ARRAYMEMBER)
    add_unwind_protect (xfree, val);
  pattern = getpattern (temp1, quoted, 1);
  discard_unwind_frame ("rboxc-brace-pattern-value");''')
    stage=root/'build/bash-parameter-cleanup';stage.mkdir(exist_ok=True)
    source=stage/'subst.c';source.write_text(text)
    obj=source.with_suffix('.o');log=source.with_suffix('.c.log')
    command=record['compile_arguments'].copy()
    command[command.index(str(original))]=str(source)
    command[command.index('-o')+1]=str(obj)
    with log.open('w') as out:subprocess.run(command,cwd=record['directory'],stdout=out,stderr=subprocess.STDOUT,check=True)
    adapted[obj.name]=obj
    (root/'evidence/bash-parameter-cleanup.json').write_text(json.dumps({
        'scope':'Brace length names, recursive nameref strings and pattern caller allocations retain existing xfree unwind owners during nested expansion. Whole arrays and positional parameters remain borrowed. Normal calls discard the temporary registrations and retain original free calls.',
        'driver_sha256':fingerprint(Path(__file__)),
        'files':[{'original':str(original),'original_sha256':fingerprint(original),
            'source':str(source),'source_sha256':fingerprint(source),'object':str(obj),'object_sha256':fingerprint(obj),
            'compile_arguments':command,'directory':record['directory'],'log':str(log),'log_sha256':fingerprint(log)}]},indent=2)+'\n')
