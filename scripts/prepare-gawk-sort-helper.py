#!/usr/bin/env python3
"""Build a private GNU recipe helper with normal-exit sort vector cleanup."""
# SPDX-License-Identifier: GPL-3.0-or-later
import hashlib
import json
from pathlib import Path
import shutil
import subprocess

ROOT = Path(__file__).resolve().parents[1]
BUILD = ROOT/'build/gnu-coreutils'
STAGE = ROOT/'build/gawk-sort-helper'
REPORT = ROOT/'evidence/gawk-sort-helper.json'
assert not STAGE.exists() and not REPORT.exists(), 'preserve previous builds'
STAGE.mkdir()
digest = lambda p: hashlib.sha256(p.read_bytes()).hexdigest()
records = [(p, json.loads(p.read_text())) for p in (ROOT/'build/cc-records').glob('*.json')]
record_path, record = next((p, r) for p, r in records if r['output']=='src/libsinglebin_sort_a-sort.o')
source = Path(record['file'])
text = source.read_text()
anchor = '  main_exit (EXIT_SUCCESS);\n}'
assert text.count(anchor) == 1
adapted = text.replace(anchor, '  if (!files_from)\n    free (files);\n\n'+anchor)
# Reuse the already reviewed native adaptation, without changing that artifact.
assert adapted == (ROOT/'build/native-cleanup/sort.c').read_text()
copied = STAGE/'sort.c'; copied.write_text(adapted)
obj = STAGE/Path(record['output']).name
args = []
original_args = iter(record['arguments'])
for arg in original_args:
    if arg in ('-o', '-MT', '-MF'):
        next(original_args)
    elif arg not in ('-MD', '-MP', str(source)):
        args.append(arg)
compile_command = [*args, '-o', str(obj), str(copied)]
log = STAGE/'build.log'
with log.open('wb') as output:
    subprocess.run(compile_command, cwd=BUILD, stdout=output, stderr=subprocess.STDOUT, check=True)
    archive = STAGE/'libsinglebin_sort.a'
    shutil.copy2(BUILD/'src/libsinglebin_sort.a', archive)
    subprocess.run(['ar', 'r', str(archive), str(obj)], stdout=output, stderr=subprocess.STDOUT, check=True)
    mk = STAGE/'link.mk'
    mk.write_text(".PHONY: rboxc-print\nrboxc-print:\n\t@printf '%s\\n' $(CCLD) $(AM_CFLAGS) $(CFLAGS) $(AM_LDFLAGS) $(LDFLAGS) $(src_coreutils_OBJECTS) $(src_coreutils_LDADD) $(LIBS)\n")
    original_link = subprocess.check_output(['make','--no-print-directory','-s','-f','Makefile','-f',str(mk),'rboxc-print'],cwd=BUILD,text=True).splitlines()
    assert original_link.count('src/libsinglebin_sort.a') >= 1
    link_command = [str(archive) if a=='src/libsinglebin_sort.a' else a for a in original_link]
    binary = STAGE/'coreutils'
    link_command += ['-o', str(binary)]
    subprocess.run(link_command, cwd=BUILD, stdout=output, stderr=subprocess.STDOUT, check=True)
inputs = {p:digest(p) for p in [source, record_path, BUILD/'Makefile', ROOT/'scripts/native_cleanup.py',
    ROOT/'build/native-cleanup/sort.c', Path(__file__), Path('/usr/bin/gcc').resolve(), Path('/usr/bin/ar').resolve()]}
for arg in original_link:
    path = BUILD/arg
    if path.is_file(): inputs[path] = digest(path)
for base in (BUILD, source.parent.parent):
    for directory in ('lib','src'):
        for path in (base/directory).glob('*.h'): inputs[path] = digest(path)
inputs[BUILD/'lib/config.h'] = digest(BUILD/'lib/config.h')
report = {'scope':'Private native GNU Coreutils 9.11 recipe helper. Only sort.c normal completion frees its independent argv-name vector, matching the existing reviewed native cleanup. Original helpers, translated applets and release binaries are unchanged. Error-exit and files0-from cleanup are outside this helper profile.',
    'binary':str(binary), 'binary_sha256':digest(binary), 'inputs':{str(p):h for p,h in inputs.items()},
    'compile_command':compile_command,'link_command':link_command,
    'artifacts':{str(p.relative_to(ROOT)):digest(p) for p in (copied,obj,archive,mk,log,binary)},
    'original_sort_archive_sha256':digest(BUILD/'src/libsinglebin_sort.a'),
    'original_helper_sha256':digest(BUILD/'src/coreutils'), 'driver_sha256':digest(Path(__file__))}
REPORT.write_text(json.dumps(report,indent=2)+'\n')
print('Built private native GNU helper:',binary)
