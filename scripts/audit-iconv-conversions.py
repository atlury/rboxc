#!/usr/bin/env python3
"""Audit charmap ownership, every original conversion call and retained baselines."""
# SPDX-License-Identifier: GPL-3.0-or-later
from collections import Counter
import hashlib,importlib.util,json,re,subprocess,sys
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
sys.path[:0]=[str(ROOT/'scripts'),str(ROOT/'tests/gnu')]
from iconv_charmap_cleanup import adapt
spec=importlib.util.spec_from_file_location('reviewed',ROOT/'tests/gnu/reviewed-original.py')
runner=importlib.util.module_from_spec(spec);spec.loader.exec_module(runner)
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
target=ROOT/'evidence/iconv-charmap-validation.json';assert not target.exists()
raw={};reports={}
def pin(path,expected=None):
    p=Path(path)
    if not p.is_absolute():p=ROOT/p
    key=str(p.relative_to(ROOT)) if p.is_relative_to(ROOT) else str(p)
    h=raw.get(key) or sha(p)
    if expected is not None:assert h==expected,str(p)
    raw[key]=h
    return h
def report(name):
    p=ROOT/f'evidence/{name}.json';reports[str(p.relative_to(ROOT))]=pin(p)
    return json.loads(p.read_text())
def memory(record,path,digest,strict=False):
    pin(path,digest);text=(ROOT/path).read_text()
    pid,=set(re.findall(r'^==([0-9]+)==',text,re.M))
    parsed=runner.parse_memory_log(text,pid,exec_only=True)
    assert all(record[k]==v for k,v in parsed.items())
    assert parsed['complete_exec_log']
    clean=parsed['errors']==parsed['non_inherited_descriptors']==0 and not any(parsed['heap_bytes'].get(k,0) for k in ('definitely lost','indirectly lost','possibly lost'))
    if strict:assert clean
    return text,pid,clean
binary=ROOT/'target/iconv-charmap-cleanup-candidate/release/rboxc';digest=pin(binary)
assert pin('target/iconv-charmap-cleanup-repro/release/rboxc')==digest
assert len(subprocess.check_output([binary,'--list']).splitlines())==187
pin('target/release/rboxc','8979835878e7dbe575440b5d0c7c86a01c75782ff5ddc735707ccada50e88e86')
cleanup=report('iconv-charmap-cleanup');pin('scripts/iconv_charmap_cleanup.py',cleanup['driver_sha256'])
assert len(cleanup['files'])==3
for f in cleanup['files']:
    for key in ('original','source','object','native_object','record','log'):pin(f[key],f[key+'_sha256'])
    assert adapt(Path(f['original']).name,Path(f['original']).read_text())==Path(f['source']).read_text()
    original_record=json.loads(Path(f['record']).read_text())
    expected=original_record['arguments'].copy();expected[1]=f['source']
    for flag,value in [('-o',f['object']),('-MF',str(Path(f['source']).with_suffix('.d'))),('-MT',f['object'])]:expected[expected.index(flag)+1]=value
    expected+=['-iquote',str(Path(f['original']).parent)]
    assert f['compile_arguments']==expected and f['directory']==original_record['directory']
helper_rebuild=report('iconv-charmap-helper-rebuild')
assert helper_rebuild['byte_identical'] and helper_rebuild['cleanup_profile_sha256']==reports['evidence/iconv-charmap-cleanup.json']
assert len(helper_rebuild['files'])==9
for p,r in helper_rebuild['files'].items():pin(p,r['sha256']);pin(r['snapshot'],r['sha256'])
old=report('iconv-before-charmap-link');new=report('iconv-link')
assert old['original_inputs']==new['original_inputs'] and old['helper_inputs'].keys()==new['helper_inputs'].keys()
changed=[p for p in old['helper_inputs'] if old['helper_inputs'][p]!=new['helper_inputs'][p]]
assert changed==['build/helpers/iconv-000-iconv_charmap.o','build/helpers/iconv-001-charmap.o','build/helpers/iconv-003-linereader.o']
for p,h in {**new['original_inputs'],**new['helper_inputs']}.items():pin(p,h)
selection=report('iconv-conversion-selection')
pin('scripts/prepare-iconv-conversion-selection.py',selection['driver_sha256'])
pin(selection['source'],selection['source_sha256']);pin(selection['selected_script'],selection['selected_sha256'])
original=Path(selection['source']).read_bytes();marker=b'# Check for crashes in decoders.\n'
assert original.count(marker)==1 and original.index(marker)==selection['prefix_bytes']
assert Path(selection['selected_script']).read_bytes()==original.split(marker)[0]+b'exit $failed\n'
assert selection['expected_sections']=={'test_data':136,'ascii_roundtrip':134,'charmap':63,'endianness':8}
assert selection['expected_calls']==676
for p,h in {**selection['fixtures'],**selection['runtime']}.items():pin(p,h)
for table,rows in selection['tables'].items():
    source_rows=[line.split() for line in (Path('/opt/src/glibc-2.43/iconvdata')/table).read_text().splitlines() if line.strip() and not line.startswith('#')]
    assert rows==source_rows
focused=report('iconv-charmap-contract');before=report('iconv-charmap-before-contract')
assert focused['binary_sha256']==digest and focused['passed']==focused['total']==8
assert before['passed']==0 and before['total']==8
for r in (before,focused):
    assert r['complete'] and r['planned_total']==8
    pin(r['binary'],r['binary_sha256']);pin('tests/iconv-charmap-contract.py',r['driver_sha256'])
    for p,h in r['fixtures'].items():pin(p,h)
    for o in r['oracles'].values():pin(o['path'],o['sha256'])
    for x in r['results']:
        assert x['equivalent']
        for key,o in x['outcomes'].items():
            assert all(o[k]==x['outcomes']['gnu'][k] for k in ('status','stdout','stderr','tree'))
            if o.get('log'):memory(o['memory'],o['log'],o['log_sha256'],r is focused and key=='rboxc-valgrind')
        if r is focused:assert x['pass'] and x['memory_clean'] and not x['outcomes']['rboxc-valgrind']['memory']['heap_bytes']
reference_calls=None;matrix_logs={}
for name,mode in [('iconv-conversions-original','native'),('iconv-charmap-conversions-native','native'),('iconv-charmap-conversions-valgrind','valgrind')]:
    r=report(name);assert r['complete'] and r['passed']==r['total']==r['planned_total']==2 and r['mode']==mode
    pin(r['binary'],r['binary_sha256'])
    if name!='iconv-conversions-original':assert r['binary_sha256']==digest
    for p,h in {**r['inputs'],**r['runtime_helpers']}.items():pin(p,h)
    for p,h in r['file_inventory'].items():
        if h is None:assert not Path(p).exists()
        else:pin(p,h)
    assert {x['implementation'] for x in r['results']}=={'gnu','rboxc'}
    for x in r['results']:
        assert x['pass'] and x['status']==0 and not x['timed_out']
        assert x['sections']==selection['expected_sections'] and len(x['calls'])==676
        for p,h in x['raw'].items():pin(p,h)
        stdout=next(ROOT/p for p in x['raw'] if p.endswith('/stdout')).read_bytes()
        stderr=next(ROOT/p for p in x['raw'] if p.endswith('/stderr')).read_bytes()
        assert stdout.hex()==x['stdout'] and stderr.hex()==x['stderr'] and b'FAILED' not in stdout
        assert x['stdout']==r['results'][0]['stdout'] and x['stderr']==r['results'][0]['stderr']
        counters=Counter();calls_by_pid={};directories=set()
        for call in x['calls']:
            pin(call['record'],call['sha256']);values=(ROOT/call['record']).read_bytes().split(b'\0')
            assert values.pop()==b'' and [v.decode() for v in values]==call['arguments']
            args=call['arguments'];directory,=re.findall(r'^(/tmp/rboxc-iconv-conversions-[a-z0-9_]{8})/obj/elf/ld.so$',args[0]);directories.add(directory)
            assert args[1:4]==['--library-path',':'+directory+'/obj:'+directory+'/obj/iconvdata','--inhibit-rpath']
            assert args[5]==directory+'/obj/iconv/iconv_prog'
            modules={row[0] for row in selection['tables']['TESTS']} | {row[1] for row in selection['tables']['TESTS2']}
            assert args[4] in {name+'.so' for name in modules}
            normalized=[v.replace(directory,'<fixture>') for v in args[6:]]
            assert call['normalized']==normalized;counters[tuple(normalized)]+=1
            pid=Path(call['record']).stem.removeprefix('call-');assert pid not in calls_by_pid
            calls_by_pid[pid]=(directory,args[6:])
        assert len(directories)==1
        if reference_calls is None:reference_calls=counters
        else:assert counters==reference_calls
        assert len(x['memory'])==(676 if mode=='valgrind' else 0)
        seen=set();clean=0
        for m in x['memory']:
            text,pid,is_clean=memory(m,m['log'],m['sha256'],x['implementation']=='rboxc')
            assert m['clean']==is_clean and pid in calls_by_pid and pid not in seen;seen.add(pid);clean+=is_clean
            directory,args=calls_by_pid[pid]
            image,=re.findall(r'^==[0-9]+== Command: (.*)$',text,re.M)
            assert image==directory+'/exec/iconv '+' '.join(args)
        if mode=='valgrind':
            assert seen==calls_by_pid.keys();matrix_logs[x['implementation']]={'total':len(seen),'clean':clean}
assert matrix_logs['rboxc']=={'total':676,'clean':676}
buffer=report('iconv-charmap-buffer-validation')
assert buffer['candidate_sha256']==digest and buffer['clean_candidate_processes']==227
for p,h in {**buffer['reports'],**buffer['raw_files']}.items():pin(p,h)
old_inventory=ROOT/'evidence/raw/glibc-before-conversion-inventory.json'
pin(old_inventory,buffer['source_files']['inventory/glibc-utility-tests.json'])
prior=json.loads(old_inventory.read_text());current_path=ROOT/'inventory/glibc-utility-tests.json';current=json.loads(current_path.read_text());pin(current_path)
assert len(prior['tests'])==4 and len(current['tests'])==5
for a,b in zip(prior['tests'],current['tests']):
    if a['path']=='iconv/tst-iconv_prog-buffer.sh':
        assert b=={**a,'evidence':'evidence/iconv-charmap-buffer-original.json','audit':'evidence/iconv-charmap-buffer-validation.json'}
    else:assert a==b
row=current['tests'][-1]
assert row['path']=='iconvdata/run-iconv-test.sh' and row['state']=='reviewed-selected-passing' and not row['full_script']
assert row['source_sha256']==selection['source_sha256'] and row['selected_tables']==['TESTS','TESTS2']
for p in ('scripts/entry_provider_helpers.py','tests/iconv-conversions-original.py','tests/iconv-original.py'):pin(p)
target.write_text(json.dumps({'scope':'All 676 calls in the unchanged ordinary GNU conversion/charmap/endianness selection pass normally and under Valgrind on the corrected candidate. Every candidate image is complete and clean. Three original buffering recipes and 50 focused utility checks add 227 clean candidate images, and eight focused charmap checks leave no heap outstanding. Only three namespaced native helpers change; original GNU sources/objects and installed release remain unchanged. Native helper objects and candidate rebuild byte-identically. The separate decoder crash loop and broader glibc library certification remain outside this result.',
    'binary':str(binary),'binary_sha256':digest,'size_bytes':binary.stat().st_size,
    'byte_identical_candidate_rebuild':True,'byte_identical_helper_rebuild':True,
    'changed_helpers':changed,'original_conversion_calls_per_profile':676,'conversion_sections':selection['expected_sections'],
    'matrix_memory':matrix_logs,'clean_buffer_and_focused_logs':227,'clean_charmap_contract_logs':8,
    'reports':reports,'raw':raw,'accounting_pass':True,'selected_original_pass':True,'full_glibc_complete':False},indent=2)+'\n')
print('PASS: 676 original conversions; 227 buffer/utility and eight charmap process logs clean')
