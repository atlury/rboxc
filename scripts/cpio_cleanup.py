"""Release GNU Cpio's temporary current-directory string after copying it."""
# SPDX-License-Identifier: GPL-3.0-or-later
import json
from pathlib import Path
import subprocess
from cpio_helpers import fingerprint


def prepare(root):
    pin=json.loads((root/'inventory/sources.json').read_text())['cpio']
    original=Path(pin['source'])/'src/copypass.c'
    assert fingerprint(original)==pin['source_and_header_sha256']['src/copypass.c']
    text=original.read_text()
    before='      ds_concat (&output_name, pwd);\n'
    assert text.count(before)==1
    text=text.replace(before,before+'      free (pwd);\n')
    stage=root/'build/cpio-cleanup';stage.mkdir(exist_ok=True)
    adapted=stage/'copypass.c';adapted.write_text(text)
    records=[json.loads(p.read_text()) for p in (root/'build/cpio-cc-records').glob('*.json')]
    records=[r for r in records if Path(r['file'])==original];assert len(records)==1
    record=records[0];arguments=record['arguments'].copy();output=stage/'copypass.o'
    arguments[arguments.index('-o')+1]=str(output)
    arguments[arguments.index(str(original))]=str(adapted)
    arguments+=['-I'+str(original.parent),'-g']
    log=root/'evidence/raw/cpio-directory-cleanup-build.log'
    with log.open('w') as out:subprocess.run(arguments,cwd=record['directory'],stdout=out,stderr=subprocess.STDOUT,check=True)
    report={'scope':'Release xgetcwd storage immediately after ds_concat copies the path. Directory resolution and copy-pass behavior remain unchanged.',
            'driver_sha256':fingerprint(Path(__file__)),'original_sha256':fingerprint(original),
            'adapted_source_sha256':fingerprint(adapted),'object_sha256':fingerprint(output),
            'compiler_arguments':arguments,'log':str(log.relative_to(root)),'log_sha256':fingerprint(log)}
    (root/'evidence/cpio-native-cleanup.json').write_text(json.dumps(report,indent=2)+'\n')
    return output
