"""Release GNU Screen's owned action lengths when clearing a key binding."""
# SPDX-License-Identifier: GPL-3.0-or-later
import json
from pathlib import Path
import subprocess
from entry_provider_helpers import fingerprint


def adapt(text):
    anchor = '\tfree((char *)act->args);\n\tact->args = noargs;\n\tact->argl = NULL;'
    assert text.count(anchor) == 1
    return text.replace(anchor, '\tfree((char *)act->args);\n\tfree(act->argl);\n\tact->args = noargs;\n\tact->argl = NULL;')


def prepare(root):
    pin = json.loads((root/'inventory/sources.json').read_text())['screen']
    source = Path(pin['source'])/'process.c'
    assert fingerprint(source) == pin['source_and_header_sha256']['process.c']
    stage = root/'build/screen-key-cleanup'
    stage.mkdir(exist_ok=True)
    adapted = stage/'process.c'
    adapted.write_text(adapt(source.read_text()))
    output = stage/'process.o'
    records = [json.loads(p.read_text()) for p in (root/'build/screen-cc-records').glob('*.json')]
    matching = [r for r in records if r.get('file') == str(source)]
    assert len(matching) == 1
    record = matching[0]
    args = record['arguments'].copy()
    args[args.index(str(source))] = str(adapted)
    args[args.index('-o')+1] = str(output)
    args += ['-iquote',str(source.parent)]
    log = stage/'build.log'
    with log.open('w') as stream:
        subprocess.run(args,cwd=record['directory'],stdout=stream,stderr=subprocess.STDOUT,check=True)
    report = {'scope':'GNU SaveAction allocates an argument-length array with its argument strings. '
        'ClearAction now frees that array before clearing its pointer; original early returns and '
        'binding replacement behavior are retained. The immutable GNU source and Rust entry are unchanged.',
        'driver_sha256':fingerprint(Path(__file__)), 'original_sha256':fingerprint(source),
        'adapted_source_sha256':fingerprint(adapted),'object_sha256':fingerprint(output),
        'compiler_arguments':args,'log':str(log.relative_to(root)),'log_sha256':fingerprint(log)}
    (root/'evidence/screen-key-native-cleanup.json').write_text(json.dumps(report,indent=2)+'\n')
    return {'process.o':output}
