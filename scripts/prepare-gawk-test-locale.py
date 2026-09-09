#!/usr/bin/env python3
"""Prepare isolated locale data for original Gawk language tests."""
# SPDX-License-Identifier: GPL-3.0-or-later
import argparse
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess

ROOT = Path(__file__).resolve().parents[1]
parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('profile', choices=('greek','french'))
choice = parser.parse_args().profile
name, locale_source, charmap = {'greek':('el_GR.iso88597@euro','el_GR','ISO-8859-7'),
                               'french':('fr_FR.UTF-8','fr_FR','UTF-8')}[choice]
source = Path(json.loads((ROOT/'inventory/sources.json').read_text())['glibc']['source'])/'localedata'
stage = ROOT/('build/gawk-'+choice+'-locale-stage')
runtime = Path('/usr/lib/locale')/('rboxc-gawk-'+choice)
report = ROOT/('evidence/gawk-'+choice+'-locales.json')
assert not stage.exists() and not runtime.exists() and not report.exists()
stage.mkdir()
runtime.mkdir()
digest = lambda p: hashlib.sha256(p.read_bytes()).hexdigest()
inputs = {str(p):digest(p) for p in (source/'locales').iterdir() if p.is_file()}
for p in (source/'charmaps'/charmap, Path('/usr/bin/localedef'), Path('/usr/bin/locale')):
    inputs[str(p)] = digest(p)
command = ['/usr/bin/localedef','--no-archive','-i',str(source/'locales'/locale_source),
           '-f',str(source/'charmaps'/charmap),str(stage/name)]
log = ROOT/('evidence/raw/gawk-'+choice+'-locale-build.log')
with log.open('w') as out:
    subprocess.run(command, env={**os.environ,'I18NPATH':str(source)}, stdout=out, stderr=subprocess.STDOUT, check=True)
shutil.copytree(stage/name, runtime/name)
probe = subprocess.run(['/usr/bin/locale','charmap'], env={'PATH':'/usr/bin:/bin','LOCPATH':str(runtime),'LC_ALL':name}, capture_output=True, check=True)
assert probe.stdout == (charmap+'\n').encode() and not probe.stderr
files = {str(p.relative_to(runtime/name)):digest(p) for p in (runtime/name).rglob('*') if p.is_file()}
assert files == {str(p.relative_to(stage/name)):digest(p) for p in (stage/name).rglob('*') if p.is_file()}
assert all(digest(Path(p)) == h for p, h in inputs.items())
report.write_text(json.dumps({'scope':'Private locale collection built from pinned GNU glibc data for unchanged Gawk recipes. The system locale archive is unchanged; only the selected test receives this LOCPATH.',
    'preparation_driver':str(Path(__file__).relative_to(ROOT)), 'driver_sha256':digest(Path(__file__)),
    'runtime_path':str(runtime), 'name':name, 'inputs':inputs, 'files':files,
    'build_command':command, 'build_log':str(log.relative_to(ROOT)), 'build_log_sha256':digest(log),
    'probe':{'status':probe.returncode,'stdout':probe.stdout.decode(),'stderr':probe.stderr.decode()}}, indent=2)+'\n')
print('Prepared private', name, 'locale')
