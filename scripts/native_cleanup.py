"""Compile a narrowly modified copy of an explicitly retained GNU C entry."""
import hashlib
import json
from pathlib import Path
import subprocess


def sort_cleanup(root, row):
    build = root/'build/gnu-coreutils'
    records = [json.loads(p.read_text()) for p in (root/'build/cc-records').glob('*.json')]
    record, = [r for r in records if r['output'] == row['gnu_object']]
    source = Path(record['file'])
    original = source.read_bytes()
    assert hashlib.sha256(original).hexdigest() == row['source_sha256']
    text = original.decode()
    anchor = '  main_exit (EXIT_SUCCESS);\n}'
    assert text.count(anchor) == 1
    # --files0-from owns its token storage through a separate token object.
    # This increment releases only the independent argv-name vector.
    text = text.replace(anchor, '  if (!files_from)\n    free (files);\n\n'+anchor)
    stage = root/'build/native-cleanup'
    stage.mkdir(exist_ok=True)
    translated = stage/'sort.c'
    translated.write_text(text)
    obj = stage/Path(record['output']).name
    args = []
    source_args = iter(record['arguments'])
    for arg in source_args:
        if arg in ('-o', '-MT', '-MF'):
            next(source_args)
        elif arg not in ('-MD', '-MP', str(source)):
            args.append(arg)
    subprocess.run([*args, '-o', str(obj), str(translated)], cwd=build, check=True)
    return obj, {'command': 'sort', 'original_sha256': row['source_sha256'],
                 'adapted_sha256': hashlib.sha256(translated.read_bytes()).hexdigest(),
                 'scope': 'free independent argv-name vector on normal completion; remains a C entry'}
