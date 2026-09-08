#!/usr/bin/env python3
"""Prepare the original Gawk Russian UTF-8 test locale from pinned glibc data."""
# SPDX-License-Identifier: GPL-3.0-or-later
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
ROOT=Path(__file__).resolve().parents[1]
def digest(p):return hashlib.sha256(p.read_bytes()).hexdigest()
source=Path(json.loads((ROOT/'inventory/sources.json').read_text())['glibc']['source'])/'localedata'
name='ru_RU.UTF-8';stage=ROOT/'build/gawk-test-locales';stage.mkdir(exist_ok=True)
runtime=Path('/usr/lib/locale/rboxc-gawk-tests');runtime.mkdir(exist_ok=True)
report_path=ROOT/'evidence/gawk-test-locales.json';assert not report_path.exists()
inputs={str(p):digest(p) for p in (source/'locales').iterdir() if p.is_file()}
inputs[str(source/'charmaps/UTF-8')]=digest(source/'charmaps/UTF-8')
inputs['/usr/bin/localedef']=digest(Path('/usr/bin/localedef'))
inputs['/usr/bin/locale']=digest(Path('/usr/bin/locale'))
assert not (stage/name).exists() and not (runtime/name).exists()
command=['/usr/bin/localedef','--no-archive','-i',str(source/'locales/ru_RU'),'-f',str(source/'charmaps/UTF-8'),str(stage/name)]
env={**os.environ,'I18NPATH':str(source)}
log=ROOT/'evidence/raw/gawk-test-locale-build.log'
with log.open('w') as out:subprocess.run(command,env=env,stdout=out,stderr=subprocess.STDOUT,check=True)
shutil.copytree(stage/name,runtime/name)
(runtime/'ru_RU.utf8').symlink_to(name)
probe=subprocess.run(['/usr/bin/locale','charmap'],env={'PATH':'/usr/bin:/bin','LOCPATH':str(runtime),'LC_ALL':name},capture_output=True,check=True)
assert probe.stdout==b'UTF-8\n' and not probe.stderr
files={str(p.relative_to(runtime/name)):digest(p) for p in (runtime/name).rglob('*') if p.is_file()}
assert files=={str(p.relative_to(stage/name)):digest(p) for p in (stage/name).rglob('*') if p.is_file()}
assert all(digest(Path(p))==h for p,h in inputs.items())
report_path.write_text(json.dumps({'scope':'Separate Russian UTF-8 collection for the unchanged original mtchi18n Make recipe. Pinned glibc locale and UTF-8 source data; system locale archive unchanged. Explicit LOCPATH is supplied only to this test.','driver_sha256':digest(Path(__file__)),'runtime_path':str(runtime),'name':name,'alias':'ru_RU.utf8','inputs':inputs,'files':files,'build_command':command,'build_log':str(log.relative_to(ROOT)),'build_log_sha256':digest(log),'probe':{'status':probe.returncode,'stdout':probe.stdout.decode(),'stderr':probe.stderr.decode()}},indent=2)+'\n')
print('Prepared and verified private Russian UTF-8 locale')
