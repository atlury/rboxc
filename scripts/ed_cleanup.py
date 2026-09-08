"""Release GNU Ed's temporary filter command after its last use."""
# SPDX-License-Identifier: GPL-3.0-or-later
import json
from pathlib import Path
import subprocess
from ed_helpers import fingerprint


def prepare(root):
    pin=json.loads((root/'inventory/sources.json').read_text())['ed']
    original=Path(pin['source'])/'main_loop.c'
    assert fingerprint(original)==pin['helper_source_sha256']['main_loop']
    text=original.read_text()
    before='  if( write_file( command, "w", first_addr, second_addr ) < 0 )\n'
    assert text.count(before)==1
    text=text.replace(before,'''  const int filter_result = write_file( command, "w", first_addr, second_addr );
  const int filter_errno = errno;
  free( command );
  errno = filter_errno;
  if( filter_result < 0 )
''')
    stage=root/'build/ed-cleanup';stage.mkdir(exist_ok=True)
    adapted=stage/'main_loop.c';adapted.write_text(text)
    records=[json.loads(p.read_text()) for p in (root/'build/ed-cc-records').glob('*.json')]
    records=[r for r in records if Path(r['file'])==original];assert len(records)==1
    record=records[0];arguments=record['arguments'].copy();output=stage/'main_loop.o'
    arguments[arguments.index('-o')+1]=str(output)
    arguments[arguments.index(str(original))]=str(adapted)
    arguments+=['-I'+str(original.parent),'-g']
    log=root/'evidence/raw/ed-filter-cleanup-build.log'
    with log.open('w') as out:subprocess.run(arguments,cwd=record['directory'],stdout=out,stderr=subprocess.STDOUT,check=True)
    report={'scope':'Release the temporary filter command after write_file returns, on both success and failure, preserving errno. Editing and shell command construction remain unchanged.',
            'driver_sha256':fingerprint(Path(__file__)),'original_sha256':fingerprint(original),
            'adapted_source_sha256':fingerprint(adapted),'object_sha256':fingerprint(output),
            'compiler_arguments':arguments,'log':str(log.relative_to(root)),'log_sha256':fingerprint(log)}
    (root/'evidence/ed-native-cleanup.json').write_text(json.dumps(report,indent=2)+'\n')
    return output
