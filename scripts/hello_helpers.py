"""Keep GNU Hello's native helper definitions separate from Coreutils."""
# SPDX-License-Identifier: GPL-3.0-or-later
import hashlib
from pathlib import Path
import re
import subprocess


def fingerprint(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def symbol_map(root):
    archive = root/'build/gnu-hello/lib/libhello.a'
    output = subprocess.check_output(['nm', '-g', '--defined-only', '--format=posix', archive], text=True)
    symbols = set(re.findall(r'^(\w+) [A-Z] ', output, re.M))
    assert symbols and 'main' not in symbols
    assert not any(s.startswith('single_binary_main_') for s in symbols)
    return {s: 'rboxc_hello_'+s for s in sorted(symbols)}


def prepare_archive(root, mapping):
    stage = root/'build/translation/hello'
    stage.mkdir(parents=True, exist_ok=True)
    definitions = stage/'helper-symbol-map'
    definitions.write_text(''.join(f'{old} {new}\n' for old, new in mapping.items()))
    archive = root/'build/helpers/libhello-rboxc.a'
    archive.parent.mkdir(exist_ok=True)
    temporary = archive.with_suffix('.tmp.a')
    subprocess.run(['objcopy', '--redefine-syms='+str(definitions),
                    root/'build/gnu-hello/lib/libhello.a', temporary], check=True)
    subprocess.run(['ranlib', temporary], check=True)
    output = subprocess.check_output(['nm', '-g', '--defined-only', '--format=posix', temporary], text=True)
    assert set(re.findall(r'^(\w+) [A-Z] ', output, re.M)) == set(mapping.values())
    temporary.replace(archive)
    return archive, definitions
