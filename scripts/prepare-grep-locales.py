#!/usr/bin/env python3
"""Prepare an isolated locale collection for reviewed Grep original tests."""
# SPDX-License-Identifier: GPL-3.0-or-later
import hashlib
import json
from pathlib import Path
import re
import subprocess

ROOT = Path(__file__).resolve().parents[1]
# The distro locale utility requires a standard locale directory. This
# separate collection is selected explicitly through LOCPATH, not the archive.
destination = Path('/usr/lib/locale/rboxc-grep-tests')
destination.mkdir(exist_ok=True)
def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()
report_path = ROOT/'evidence/grep-test-locales.json'
previous = {r['name']:r for r in json.loads(report_path.read_text())['locales']} if report_path.exists() else {}
rows = []
for language, encoding in [('en_US','UTF-8'), ('fr_FR','UTF-8'),
                           ('fr_FR','ISO-8859-1'), ('ru_RU','KOI8-R'),
                           ('ja_JP','EUC-JP'), ('zh_CN','GB18030'),
                           ('zh_HK','BIG5-HKSCS'), ('tr_TR','UTF-8'), ('cs_CZ','UTF-8')]:
    name = language+'.'+encoding
    target = destination/name
    if target.exists():
        assert name in previous
        assert {str(p.relative_to(target)):digest(p) for p in target.rglob('*') if p.is_file()} == previous[name]['files']
    else:
        subprocess.run(['localedef','--no-archive','-i',language,'-f',encoding,str(target)],check=True)
    aliases = [language+'.'+re.sub('[^a-z0-9]', '', encoding.lower())]
    if language == 'zh_CN':
        aliases.append('zh_CN')
    for alias in aliases:
        path = destination/alias
        if path.is_symlink():
            assert path.readlink() == Path(name)
        else:
            assert not path.exists()
            path.symlink_to(name)
    rows.append({'name':name, 'aliases':aliases,
                 'locale_source_sha256':digest(Path('/usr/share/i18n/locales')/language),
                 'charmap_sha256':digest(Path('/usr/share/i18n/charmaps')/(encoding+'.gz')),
                 'files':{str(p.relative_to(target)):digest(p) for p in sorted(target.rglob('*')) if p.is_file()}})
report = {'scope':'Explicit isolated LOCPATH for reviewed Grep tests; system locale archive unchanged.',
          'runtime_path':str(destination), 'driver_sha256':digest(Path(__file__)),
          'localedef_version':subprocess.check_output(['localedef','--version'],text=True).splitlines()[0],
          'localedef_sha256':digest(Path('/usr/bin/localedef')), 'locales':rows}
(ROOT/'evidence/grep-test-locales.json').write_text(json.dumps(report,indent=2)+'\n')
print('Prepared',len(rows),'isolated locales')
