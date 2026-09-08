#!/usr/bin/env python3
"""Build GNU's screen-replay configuration without replacing release artifacts."""
# SPDX-License-Identifier: GPL-3.0-or-later
from concurrent.futures import ThreadPoolExecutor
import argparse
import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT/'tests'))
from comparison_profile import fingerprint
from entry_provider_helpers import defined_symbols

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('--profile-name', default='less-original-profile')
parser.add_argument('--keyboard-cleanup', action='store_true')
options = parser.parse_args()
name = options.profile_name
assert re.fullmatch(r'less-[a-z0-9-]+', name)
stage = ROOT/'build'/name
assert not stage.exists(), 'preserve previous profile builds'
stage.mkdir()
report_path = ROOT/'evidence'/(name+'-build.json')
assert not report_path.exists()
pin = json.loads((ROOT/'inventory/sources.json').read_text())['less']
source = Path(pin['source'])
profile = json.loads((ROOT/'evidence/less-build-profile.json').read_text())
original_build = ROOT/'build/gnu-less'
assert fingerprint(original_build/'defines.h') == profile['config_header_sha256']
assert fingerprint(original_build/'less') == profile['oracle']['sha256']
records = [json.loads(p.read_text()) for p in (ROOT/'build/less-cc-records').glob('*.json')]
link = profile['link_record']['arguments']
objects = [w for w in link if w.endswith('.o')]
assert objects.count('main.o') == 1 and link[-1] == '-ltinfo'
inputs = {p: fingerprint(p) for p in [Path(__file__), ROOT/'src/generated/applet_less.rs',
          ROOT/'build/translation/less/out/src/main.rs', original_build/'defines.h',
          ROOT/'build/translation/less/compile_commands.json', ROOT/'evidence/less-translation.json']}
for p in source.rglob('*'):
    if p.is_file() and p.suffix in ('.c', '.h', '.uni'):
        inputs[p] = fingerprint(p)
shutil.copy2(original_build/'defines.h', stage/'defines.h')
compile_records = []
for obj in objects:
    matches = [r for r in records if r.get('kind') == 'compile' and
               (Path(r['directory'])/r['output']).resolve() == original_build/obj]
    assert len(matches) == 1
    record = matches[0]
    assert Path(record['file']).is_relative_to(source)
    argv = [*record['arguments'], '-DLESSTEST', '-DUSE_TERMCAP']
    compile_records.append({'directory': str(stage), 'arguments': argv, 'output': obj,
                            'file': record['file']})

def compile_one(record):
    log = stage/(record['output']+'.log')
    with log.open('wb') as stream:
        subprocess.run(record['arguments'], cwd=stage, stdout=stream,
                       stderr=subprocess.STDOUT, check=True)

with ThreadPoolExecutor(max_workers=4) as pool:
    list(pool.map(compile_one, compile_records))
subprocess.run(link, cwd=stage, check=True)
translation = json.loads((ROOT/'build/translation/less/compile_commands.json').read_text())
translation[0]['directory'] = str(stage)
translation[0]['arguments'] += ['-DLESSTEST', '-DUSE_TERMCAP']
database = stage/'compile_commands.json'
database.write_text(json.dumps(translation, indent=2)+'\n')
with (stage/'transpile.log').open('wb') as stream:
    subprocess.run([str(ROOT/'.tools/c2rust-build/release/c2rust'), 'transpile', '--emit-modules',
                    '--emit-no-std', '--fail-on-error', '--output-dir', str(stage/'translated'), str(database)],
                   env={**os.environ, 'RUSTUP_TOOLCHAIN': '1.93.0'}, stdout=stream,
                   stderr=subprocess.STDOUT, check=True)
assert not re.search(r'(^|\n).*?:[0-9]+:[0-9]+: (?:fatal )?error:', (stage/'transpile.log').read_text())
outputs = list((stage/'translated').rglob('*.rs'))
assert len(outputs) == 1
assert outputs[0].read_bytes() == (ROOT/'build/translation/less/out/src/main.rs').read_bytes(), \
    'review any test-configuration change to the translated entry before linking'
translation_report = json.loads((ROOT/'evidence/less-translation.json').read_text())
assert fingerprint(ROOT/translation_report['rust_file']) == translation_report['rust_sha256']
mapping = {s: 'rboxc_less_'+s for s in defined_symbols([stage/o for o in objects])-{'main'}}
for name, target in {**translation_report['helper_imports'], **translation_report['rust_exports']}.items():
    assert mapping[name] == target
symbols = stage/'helper-symbol-map'
symbols.write_text(''.join(f'{name} {target}\n' for name, target in sorted(mapping.items())))
helpers = []
adapted_object = None
if options.keyboard_cleanup:
    from less_cleanup import adapt
    inputs[ROOT/'scripts/less_cleanup.py'] = fingerprint(ROOT/'scripts/less_cleanup.py')
    adapted_dir = stage/'adapted'
    adapted_dir.mkdir()
    adapted_source = adapted_dir/'ttyin.c'
    adapted_source.write_text(adapt((source/'ttyin.c').read_text()))
    adapted_object = adapted_dir/'ttyin.o'
    record = next(r for r in compile_records if r['output'] == 'ttyin.o')
    argv = record['arguments'].copy()
    argv[argv.index(str(source/'ttyin.c'))] = str(adapted_source)
    argv += ['-iquote'+str(source), '-o', str(adapted_object)]
    with (adapted_dir/'build.log').open('wb') as stream:
        subprocess.run(argv, cwd=stage, stdout=stream, stderr=subprocess.STDOUT, check=True)
for obj in objects:
    if obj == 'main.o':
        continue
    output = stage/('private-'+obj)
    helper_source = adapted_object if obj == 'ttyin.o' and adapted_object else stage/obj
    subprocess.run(['objcopy', '--redefine-syms='+str(symbols), str(helper_source), str(output)], check=True)
    helpers.append(output)
assert 'main' not in defined_symbols(helpers)
wrapper = stage/'main.rs'
wrapper.write_text('#![no_main]\n#![feature(c_variadic)]\n'
                   '#![allow(dead_code,non_camel_case_types,non_snake_case,non_upper_case_globals,unused_mut,unused_variables,unused_assignments)]\n'
                   '#[path = "'+str(ROOT/'src/generated/applet_less.rs')+'"]\nmod less;\n'
                   '#[no_mangle]\npub unsafe extern "C" fn main(argc: core::ffi::c_int, argv: *mut *mut core::ffi::c_char) -> core::ffi::c_int { less::single_binary_main_less(argc, argv) }\n')
binary = stage/'rust/less'
binary.parent.mkdir()
rust_command = ['rustc', '+nightly-2026-01-22', '--edition=2021', '-C', 'panic=abort', '-C',
                'opt-level=3', '-C', 'strip=debuginfo', str(wrapper), '-o', str(binary), '-l', 'tinfo']
for helper in helpers:
    rust_command += ['-C', 'link-arg='+str(helper)]
with (stage/'rust-build.log').open('wb') as stream:
    subprocess.run(rust_command, stdout=stream, stderr=subprocess.STDOUT, check=True)
tests = stage/'lesstest'
shutil.copytree(source/'lesstest', tests)
for table in source.glob('*.uni'):
    shutil.copy2(table, stage/table.name)
with (stage/'lesstest-build.log').open('wb') as stream:
    subprocess.run(['make', '-j4', 'TERMLIB=-ltinfo'], cwd=tests, stdout=stream,
                   stderr=subprocess.STDOUT, check=True)
versions = {str(p): subprocess.check_output([str(p), '-V']).decode() for p in (stage/'less', binary)}
assert len(set(versions.values())) == 1 and 'LESSTEST' in next(iter(versions.values()))
assert all(fingerprint(p) == h for p, h in inputs.items())
result = {'provider': 'GNU Less 704', 'scope': 'Separate GNU LESSTEST/USE_TERMCAP profile for original screen replays. '
          'The test-mode C2Rust output is byte-identical to the original translation. The current generated Rust entry '
          'is linked with namespaced test-mode GNU helpers, excluding native main.o. This standalone test artifact '
          'does not replace or certify the 187-command candidate or the installed release.',
          'inputs': {str(p): h for p, h in inputs.items()}, 'compile_records': compile_records,
          'native_link': link, 'rust_link': rust_command, 'translated_entry_identical': True,
          'keyboard_cleanup': options.keyboard_cleanup,
          'binary': str(binary), 'binary_sha256': fingerprint(binary),
          'oracle': str(stage/'less'), 'oracle_sha256': fingerprint(stage/'less'), 'versions': versions,
          'helper_inputs': {str(p): fingerprint(p) for p in helpers},
          'test_tools': {str(tests/n): fingerprint(tests/n) for n in ('lesstest', 'lt_screen', 'runtest')},
          'artifacts': {str(p): fingerprint(p) for p in stage.rglob('*') if p.is_file()},
          'driver_sha256': fingerprint(Path(__file__))}
report_path.write_text(json.dumps(result, indent=2)+'\n')
print('Built separate GNU and unchanged Rust Less entries with original LESSTEST helpers')
