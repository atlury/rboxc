#!/usr/bin/env python3
"""Run GNU's selected conversion and charmap loops with their original assertions."""
# SPDX-License-Identifier: GPL-3.0-or-later
import importlib.util,json,os,re,shlex,signal,subprocess,sys,tempfile,time
from pathlib import Path
from comparison_profile import ComparisonProfile,fingerprint
ROOT=Path(__file__).resolve().parents[1]
sys.path.insert(0,str(ROOT/'tests/gnu'))
spec=importlib.util.spec_from_file_location('reviewed',ROOT/'tests/gnu/reviewed-original.py')
runner=importlib.util.module_from_spec(spec);spec.loader.exec_module(runner)
profile=ComparisonProfile('iconv-conversions-original',selections=True,oracle=ROOT/'build/gnu-glibc/iconv/iconv_prog')
mode,=profile.options.commands;assert mode in ('native','valgrind');instrument=mode=='valgrind'
review_path=ROOT/'evidence/iconv-conversion-selection.json';review=json.loads(review_path.read_text())
source=Path('/opt/src/glibc-2.43');script=Path(review['selected_script'])
assert fingerprint(script)==review['selected_sha256']
inputs={**review['fixtures'],**review['runtime'],str(script):review['selected_sha256'],
        review['source']:review['source_sha256']}
for p in (Path(__file__).resolve(),review_path,ROOT/'tests/gnu/reviewed-original.py',ROOT/'tests/comparison_profile.py',Path('/usr/bin/valgrind'),Path('/usr/bin/valgrind.bin')):inputs[str(p)]=fingerprint(p)
# Pin every data file and every charmap presence test, including skipped branches.
file_inventory={str(p):fingerprint(p) for p in (source/'iconvdata/testdata').iterdir() if p.is_file()}
for frm,to,subset,*targets in review['tables']['TESTS']:
    for name in [frm,*['UTF-8' if t=='UTF8' else t for t in targets]]:
        p=source/'localedata/charmaps'/name;file_inventory[str(p)]=fingerprint(p) if p.is_file() else None
inputs.update({p:h for p,h in file_inventory.items() if h is not None})
assert all(fingerprint(Path(p))==h for p,h in inputs.items())
results=[]
for implementation,binary in [('gnu',profile.oracle),('rboxc',profile.binary)]:
    logs=profile.logs/implementation;logs.mkdir();(logs/'calls').mkdir()
    with tempfile.TemporaryDirectory(prefix='rboxc-iconv-conversions-') as directory:
        work=Path(directory);(work/'exec').mkdir();(work/'obj/iconvdata').mkdir(parents=True)
        (work/'src/iconvdata').mkdir(parents=True);(work/'src/localedata').mkdir()
        for name in ('TESTS','TESTS2','testdata'):(work/'src/iconvdata'/name).symlink_to(source/'iconvdata'/name)
        (work/'src/localedata/charmaps').symlink_to(source/'localedata/charmaps')
        alias=work/'exec/iconv';alias.symlink_to(binary)
        command=[str(alias)]
        if instrument:command=['/usr/bin/valgrind','--leak-check=full','--show-leak-kinds=all',
            '--track-fds=yes','--log-file='+str(logs/'process-%p.log'),*command]
        wrapper=logs/'run-iconv'
        wrapper.write_text('#!/bin/sh\nset -e\n'
            '[ "$#" -ge 6 ] && [ "$1" = '+shlex.quote(str(work/'obj/elf/ld.so'))+' ] && '
            '[ "$2" = --library-path ] && [ "$4" = --inhibit-rpath ] && '
            '[ "$6" = '+shlex.quote(str(work/'obj/iconv/iconv_prog'))+' ] || exit 77\n'
            'printf "%s\\0" "$@" > '+shlex.quote(str(logs/'calls'))+'/call-$$.argv\n'
            'shift 6\nexec '+shlex.join(command)+' "$@"\n')
        wrapper.chmod(0o755)
        argv=['/bin/sh','-f',str(script),str(work/'obj'),str(wrapper),'','']
        started=time.monotonic()
        proc=subprocess.Popen(argv,cwd=work/'src/iconvdata',stdin=subprocess.DEVNULL,stdout=subprocess.PIPE,stderr=subprocess.PIPE,start_new_session=True,
            env={'PATH':'/usr/bin:/bin','LC_ALL':'C','LANGUAGE':'C','HOME':directory,'TMPDIR':directory})
        timed_out=False
        try:stdout,stderr=proc.communicate(timeout=3600)
        except subprocess.TimeoutExpired:
            timed_out=True;os.killpg(proc.pid,signal.SIGTERM)
            try:stdout,stderr=proc.communicate(timeout=5)
            except subprocess.TimeoutExpired:
                os.killpg(proc.pid,signal.SIGKILL);stdout,stderr=proc.communicate()
        (logs/'stdout').write_bytes(stdout);(logs/'stderr').write_bytes(stderr)
        calls=[]
        for p in sorted((logs/'calls').glob('*.argv')):
            args=p.read_bytes().split(b'\0');assert args.pop()==b'' and len(args)>=6
            calls.append({'record':str(p.relative_to(ROOT)),'sha256':fingerprint(p),
                'arguments':[x.decode() for x in args],'normalized':[x.decode().replace(directory,'<fixture>') for x in args[6:]]})
        sections={'test_data':len(re.findall(rb'^   test data:',stdout,re.M)),
            'ascii_roundtrip':len(re.findall(rb'^      suntzu:',stdout,re.M)),
            'charmap':len(re.findall(rb'^test charmap:',stdout,re.M)),
            'endianness':len(re.findall(rb'^test (?:encoder|decoder|non-BOM):',stdout,re.M))}
        memory=[]
        for p in sorted(logs.glob('process-*.log')):
            text=p.read_text();pid,=set(re.findall(r'^==([0-9]+)==',text,re.M))
            parsed=runner.parse_memory_log(text,pid,exec_only=True)
            clean=parsed['complete_exec_log'] and parsed['errors']==parsed['non_inherited_descriptors']==0 and not any(parsed['heap_bytes'].get(k,0) for k in ('definitely lost','indirectly lost','possibly lost'))
            memory.append({'log':str(p.relative_to(ROOT)),'sha256':fingerprint(p),'clean':clean,**parsed})
        passed=not timed_out and proc.returncode==0 and sections==review['expected_sections'] and len(calls)==review['expected_calls'] and b'FAILED' not in stdout
        if instrument:
            passed=passed and len(memory)==len(calls)
            if implementation=='rboxc':passed=passed and all(m['clean'] for m in memory)
        else:assert not memory
        if results:
            previous=results[0]
            passed=passed and stdout.hex()==previous['stdout'] and stderr.hex()==previous['stderr']
        raw={str(p.relative_to(ROOT)):fingerprint(p) for p in [logs/'stdout',logs/'stderr',wrapper]}
        result={'implementation':implementation,'status':proc.returncode,'timed_out':timed_out,
            'elapsed_seconds':round(time.monotonic()-started,3),'sections':sections,'calls':calls,
            'memory':memory,'raw':raw,'stdout':stdout.hex(),'stderr':stderr.hex(),'pass':passed}
        results.append(result)
    assert all(fingerprint(Path(p))==h for p,h in inputs.items())
    assert all(Path(p).is_file() if h else not Path(p).exists() for p,h in file_inventory.items())
    report={**profile.metadata(),'mode':mode,'inputs':inputs,'file_inventory':file_inventory,
        'scope':'Exact original conversion/charmap/endianness loops using complete TESTS and TESTS2 tables and their unchanged data/assertions. Only the build-tree loader prefix is replaced with the selected command alias; host glibc/modules and fixture tools are pinned. The separate decoder crash loop is excluded. Every invocation is recorded and traced candidate processes require complete clean memory/descriptor summaries.',
        'complete':len(results)==2,'planned_total':2,'total':len(results),'passed':sum(x['pass'] for x in results),'results':results}
    profile.report.write_text(json.dumps(report,indent=2)+'\n')
    print(implementation,mode,len(calls),'calls',sum(m['clean'] for m in memory),'clean logs','PASS' if passed else 'OPEN',flush=True)
raise SystemExit(report['passed']!=report['total'])
