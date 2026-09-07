#!/usr/bin/env python3
"""Build private locale data used by reviewed GNU calendar and text tests."""
# SPDX-License-Identifier: GPL-3.0-or-later
import hashlib
import json
from pathlib import Path
import subprocess
import shutil

ROOT = Path(__file__).resolve().parents[1]
destination = ROOT/'build/test-locales'
destination.mkdir(exist_ok=True)
rows = []
for language, encoding in [('am_ET', 'UTF-8'), ('fa_IR', 'UTF-8'),
                           ('th_TH', 'UTF-8'), ('ru_RU', 'KOI8-R'),
                           ('en_US', 'UTF-8'), ('en_US', 'ISO-8859-1')]:
    name = language+'.'+encoding
    target = destination/name
    subprocess.run(['localedef', '--no-archive', '-i', language, '-f', encoding, target], check=True)
    rows.append({'name': name, 'source_sha256': hashlib.sha256(
                    (Path('/usr/share/i18n/locales')/language).read_bytes()).hexdigest(),
                 'files': {str(p.relative_to(target)): hashlib.sha256(p.read_bytes()).hexdigest()
                           for p in sorted(target.rglob('*')) if p.is_file()}})
# Ubuntu's locale utility is confined to standard locale-data directories.
# Install this separate collection there without changing the system archive.
runtime = Path('/usr/lib/locale/rboxc-tests')
runtime.mkdir(exist_ok=True)
for row in rows:
    target = runtime/row['name']
    if target.exists():
        assert {str(p.relative_to(target)): hashlib.sha256(p.read_bytes()).hexdigest()
                for p in target.rglob('*') if p.is_file()} == row['files'], 'existing test locale differs'
    else:
        shutil.copytree(destination/row['name'], target)
report = {'scope': 'Separate test LOCPATH; system locale archive unchanged',
          'localedef': subprocess.check_output(['localedef', '--version'], text=True).splitlines()[0],
          'path': str(destination.relative_to(ROOT)), 'runtime_path': str(runtime), 'locales': rows}
(ROOT/'evidence/test-locales.json').write_text(json.dumps(report, indent=2)+'\n')
print('Prepared', len(rows), 'private test locales')
