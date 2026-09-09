#!/usr/bin/env python3
"""Build the unchanged GNU cat entry with its existing lint cleanup enabled."""
# SPDX-License-Identifier: GPL-3.0-or-later
import hashlib
import json
from pathlib import Path
import shutil
import subprocess

ROOT=Path(__file__).resolve().parents[1]
BUILD=ROOT/'build/gnu-coreutils'
STAGE=ROOT/'build/gawk-cat-helper'
REPORT=ROOT/'evidence/gawk-cat-helper.json'
assert not STAGE.exists() and not REPORT.exists()
STAGE.mkdir()
digest=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
records=[(p,json.loads(p.read_text())) for p in (ROOT/'build/cc-records').glob('*.json')]
record_path,record=next((p,r) for p,r in records if r['output']=='src/libsinglebin_cat_a-cat.o')
source=Path(record['file'])
assert source.read_text().count('#ifdef lint')==1
assert '#ifdef lint\n  alignfree (outbuf);\n  alignfree (inbuf);\n#endif' in source.read_text()
obj=STAGE/Path(record['output']).name
args=[]; original_args=iter(record['arguments'])
for arg in original_args:
    if arg in ('-o','-MT','-MF'):next(original_args)
    elif arg not in ('-MD','-MP',str(source)):args.append(arg)
compile_command=[*args,'-Dlint','-o',str(obj),str(source)]
sort_profile_path=ROOT/'evidence/gawk-sort-helper.json'
sort_profile=json.loads(sort_profile_path.read_text())
for filename,expected in sort_profile['inputs'].items():assert digest(Path(filename))==expected
archive=STAGE/'libsinglebin_cat.a'; binary=STAGE/'coreutils'; log=STAGE/'build.log'
with log.open('wb') as output:
    subprocess.run(compile_command,cwd=BUILD,stdout=output,stderr=subprocess.STDOUT,check=True)
    shutil.copy2(BUILD/'src/libsinglebin_cat.a',archive)
    subprocess.run(['ar','r',str(archive),str(obj)],stdout=output,stderr=subprocess.STDOUT,check=True)
    link_command=[str(archive) if a=='src/libsinglebin_cat.a' else a for a in sort_profile['link_command']]
    assert link_command[-2:]==['-o',sort_profile['binary']]
    link_command[-1]=str(binary)
    subprocess.run(link_command,cwd=BUILD,stdout=output,stderr=subprocess.STDOUT,check=True)
inputs={**sort_profile['inputs'],str(sort_profile_path):digest(sort_profile_path),str(source):digest(source),str(record_path):digest(record_path),str(Path(__file__)):digest(Path(__file__)),str(ROOT/'build/gawk-sort-helper/libsinglebin_sort.a'):digest(ROOT/'build/gawk-sort-helper/libsinglebin_sort.a')}
REPORT.write_text(json.dumps({'scope':'Private GNU Coreutils 9.11 recipe helper. Cat source is unchanged; -Dlint enables its existing normal-exit aligned input/output buffer frees. Reuses the separately recorded sort helper cleanup. Original helpers and Rboxc binaries are unchanged. This profile does not add error-exit cleanup.',
    'binary':str(binary),'binary_sha256':digest(binary),'inputs':inputs,
    'compile_command':compile_command,'link_command':link_command,
    'artifacts':{str(p.relative_to(ROOT)):digest(p) for p in (obj,archive,binary,log)},
    'original_cat_archive_sha256':digest(BUILD/'src/libsinglebin_cat.a'),
    'driver_sha256':digest(Path(__file__))},indent=2)+'\n')
print('Built GNU cat helper with its existing lint cleanup:',binary)
