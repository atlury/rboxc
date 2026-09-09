#!/usr/bin/env python3
"""Build the original Bash internationalization recipe's private locale archive."""
# SPDX-License-Identifier: GPL-3.0-or-later
import argparse,hashlib,json,subprocess
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
parser=argparse.ArgumentParser(description=__doc__)
parser.add_argument('--destination',type=Path,required=True)
args=parser.parse_args();stage=args.destination.resolve()
assert stage.is_relative_to(ROOT/'build') and not stage.exists()
(stage/'usr/lib/locale').mkdir(parents=True)
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
locales=[('en_US','UTF-8','en_US.UTF-8'),('de_DE','UTF-8','de_DE.UTF-8'),
         ('fr_FR','ISO-8859-1','fr_FR.ISO-8859-1'),('ja_JP','SHIFT_JIS','ja_JP.SJIS'),
         ('zh_TW','BIG5','zh_TW.BIG5'),('ru_RU','CP1251','ru_RU.CP1251')]
inputs={str(p):sha(p) for p in [Path('/usr/bin/localedef'),Path('/usr/bin/locale'),
        *Path('/usr/share/i18n').rglob('*')] if p.is_file()}
rows=[]
for source,charmap,name in locales:
    command=['/usr/bin/localedef','--no-warnings=ascii','--prefix='+str(stage),
             '-i',source,'-f',charmap,name]
    log=stage/(name+'.log')
    with log.open('w') as out:
        done=subprocess.run(command,stdout=out,stderr=subprocess.STDOUT)
    rows.append({'command':command,'status':done.returncode,'log':str(log),'log_sha256':sha(log)})
    assert done.returncode==0,(name,done.returncode)
assert all(sha(Path(p))==h for p,h in inputs.items())
archive=stage/'usr/lib/locale/locale-archive'
(stage/'build.json').write_text(json.dumps({'scope':'Private fixture archive; system archive unchanged.',
    'driver_sha256':sha(Path(__file__)),'inputs':inputs,'builds':rows,
    'archive':str(archive),'archive_sha256':sha(archive)},indent=2)+'\n')
print('Built private locale archive:',archive.stat().st_size,'bytes')
