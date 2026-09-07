"""Materialize the pinned GNU factor test template without editing GNU sources."""
import hashlib
from pathlib import Path
import re
import subprocess


def factor_script(source, script):
    assert re.fullmatch(r'tests/factor/t(?:[0-3][0-9]|40)\.sh', script)
    inputs = {name: hashlib.sha256((source/name).read_bytes()).hexdigest() for name in
              ('tests/factor/create-test.sh', 'tests/factor/run.sh')}
    data = subprocess.check_output(['/bin/sh', source/'tests/factor/create-test.sh',
                                    script, 'tests/factor/run.sh'], cwd=source)
    return inputs, data


def materialize(root, source, row):
    inputs, data = factor_script(source, row['script'])
    assert row['generator_inputs'] == inputs, 'GNU generator changed'
    assert row['sha256'] == hashlib.sha256(data).hexdigest(), 'generated test changed'
    path = root/'build/generated-gnu-tests'/row['script']
    path.parent.mkdir(parents=True, exist_ok=True)
    if not path.exists() or path.read_bytes() != data:
        temporary = path.with_suffix('.sh.tmp')
        temporary.write_bytes(data)
        temporary.replace(path)
    return path
