#!/usr/bin/env python3
"""Preserve GNU iconv's ordinary conversion loops and their original fixtures."""
# SPDX-License-Identifier: GPL-3.0-or-later
import hashlib,json,re
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
source=Path('/opt/src/glibc-2.43');script=source/'iconvdata/run-iconv-test.sh'
stage=ROOT/'build/iconv-conversion-selection';assert not stage.exists();stage.mkdir()
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
original=script.read_bytes();marker=b'# Check for crashes in decoders.\n'
assert original.count(marker)==1
selected=original.split(marker)[0]+b'exit $failed\n'
output=stage/'run-iconv-conversions.sh';output.write_bytes(selected)
tables={}
for name,fields in [('TESTS',4),('TESTS2',3)]:
    lines=[]
    for line in (source/'iconvdata'/name).read_text().splitlines():
        if not line.strip() or line.startswith('#'):continue
        row=line.split();assert len(row)>=fields
        assert all(re.fullmatch(r'[A-Za-z0-9_.-]+',x) for x in row)
        if name=='TESTS':assert row[2] in ('Y','N','-')
        lines.append(row)
    tables[name]=lines
assert len(tables['TESTS2'])==2
fixtures={source/'iconvdata'/n for n in ('TESTS','TESTS2','Makefile')}
sections={'test_data':0,'ascii_roundtrip':0,'charmap':0,'endianness':8}
for row in tables['TESTS']:
    frm,to,subset,*targets=row
    if subset=='N':sections['ascii_roundtrip']+=1
    for target in targets:
        data=source/'iconvdata/testdata'/frm
        if data.is_file():fixtures.add(data);sections['test_data']+=1
        expected=source/'iconvdata/testdata'/(frm+'..'+target)
        if expected.is_file():fixtures.add(expected)
        if subset=='Y':sections['ascii_roundtrip']+=1
        maps=[source/'localedata/charmaps'/n for n in (frm,'UTF-8' if target=='UTF8' else target)]
        if data.is_file() and all(p.is_file() for p in maps) and not re.search(r'<U....><U....>',maps[0].read_text()):
            fixtures.update(maps);sections['charmap']+=1
for utf8,encoding,stem in tables['TESTS2']:
    for suffix in (utf8,encoding+'.BE',encoding+'.LE'):fixtures.add(source/'iconvdata/testdata'/(stem+'..'+suffix))
fixtures.add(source/'iconvdata/testdata/suntzus')
runtime=[Path('/bin/sh').resolve(),Path('/usr/bin/cmp'),Path('/usr/bin/grep'),Path('/usr/bin/rm'),
         Path('/usr/lib/x86_64-linux-gnu/libc.so.6'),Path('/usr/lib/x86_64-linux-gnu/ld-linux-x86-64.so.2')]
runtime += [p for p in Path('/usr/lib/x86_64-linux-gnu/gconv').rglob('*') if p.is_file()]
report={'scope':'Complete TESTS and TESTS2 tables and ordinary conversion/charmap/endianness control flow reviewed. The selected script retains the exact original prefix and final exit status. The separate historical decoder crash loop remains unexecuted. Native GNU and Rboxc use the same pinned host glibc 2.43 conversion modules; this is command coverage, not a port of those modules.',
    'source':str(script),'source_sha256':sha(script),'selected_script':str(output),'selected_sha256':sha(output),
    'prefix_bytes':original.index(marker),'driver_sha256':sha(__file__),
    'tables':tables,'expected_sections':sections,
    'expected_calls':2*sum(sections[k] for k in ('test_data','ascii_roundtrip','charmap'))+5*len(tables['TESTS2']),
    'fixtures':{str(p):sha(p) for p in sorted(fixtures)},'runtime':{str(p):sha(p) for p in sorted(set(runtime))}}
(ROOT/'evidence/iconv-conversion-selection.json').write_text(json.dumps(report,indent=2)+'\n')
print('Prepared conversion selection:',sections,report['expected_calls'],'iconv calls per profile')
