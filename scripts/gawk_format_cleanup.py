"""Release temporary string nodes returned while formatting non-finite numbers."""
# SPDX-License-Identifier: GPL-3.0-or-later
import json
from pathlib import Path
import subprocess
from entry_provider_helpers import fingerprint


def prepare(root):
    pin = json.loads((root/'inventory/sources.json').read_text())['gawk']
    source = Path(pin['source'])/'printf.c'
    assert fingerprint(source) == pin['source_and_header_sha256']['printf.c']
    text = source.read_text()
    start = text.index('NODE *\nformat_args(')
    end = text.index('/* printf_common', start)
    part = text[start:end]
    old = '\tNODE *arg;'
    assert part.count(old) == 1
    part = part.replace(old, old+'\n\tNODE *rboxc_formatted_string = NULL;')
    old = "\t\tcase 's':\n\t\t\tneed_format = false;\n\t\t\tparse_next_arg();\n\t\t\targ = force_string(arg);"
    assert part.count(old) == 1
    part = part.replace(old, """\t\tcase 's':
\t\t\tneed_format = false;
\t\t\tparse_next_arg();
\t\t\t/* Non-finite conversion can return a new node instead of
\t\t\t   updating the borrowed argument. Keep it until copying ends. */
\t\t\trboxc_formatted_string = force_string(arg);
\t\t\tif (rboxc_formatted_string == arg)
\t\t\t\trboxc_formatted_string = NULL;
\t\t\telse
\t\t\t\targ = rboxc_formatted_string;""")
    old = '\t\t\tbchunk(cp, copy_count);\n\t\t\twhile (fw > prec) {\n\t\t\t\tbchunk_one(fill);\n\t\t\t\tfw--;\n\t\t\t}\n\t\t\ts0 = s1;'
    assert part.count(old) == 1
    part = part.replace(old, old.replace('\t\t\ts0 = s1;', '''\t\t\tif (rboxc_formatted_string != NULL) {
\t\t\t\tunref(rboxc_formatted_string);
\t\t\t\trboxc_formatted_string = NULL;
\t\t\t}
\t\t\ts0 = s1;'''))
    text = text[:start]+part+text[end:]
    stage = root/'build/gawk-format-cleanup'
    stage.mkdir(exist_ok=True)
    adapted = stage/'printf.c'
    adapted.write_text(text)
    output = stage/'printf.o'
    records = [(p, json.loads(p.read_text())) for p in (root/'build/gawk-cc-records').glob('*.json')]
    records = [(p, r) for p, r in records if r.get('file') == str(source)]
    assert len(records) == 1
    record_path, record = records[0]
    args = record['arguments'].copy()
    args[args.index('-o')+1] = str(output)
    args[args.index(str(source))] = str(adapted)
    args += ['-I'+str(source.parent)]
    log = stage/'build.log'
    with log.open('w') as out:
        subprocess.run(args, cwd=record['directory'], stdout=out, stderr=subprocess.STDOUT, check=True)
    report = {
        'scope': 'Release only newly allocated string-conversion nodes in the printf string branch after copying their contents. Borrowed arguments, numeric state and output remain unchanged. Original GNU source is preserved.',
        'inputs': {str(p): fingerprint(p) for p in (Path(__file__), source, record_path, Path(pin['source'])/'node.c', Path(pin['source'])/'awk.h')},
        'adapted_source': str(adapted), 'adapted_source_sha256': fingerprint(adapted),
        'object': str(output), 'object_sha256': fingerprint(output),
        'compiler_arguments': args, 'log': str(log.relative_to(root)), 'log_sha256': fingerprint(log),
    }
    (root/'evidence/gawk-format-cleanup.json').write_text(json.dumps(report, indent=2)+'\n')
    return {'printf.o': output}
