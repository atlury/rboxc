#!/usr/bin/env python3
"""Translate command entry objects selected by GNU's own multicall build."""
from concurrent.futures import ThreadPoolExecutor
import hashlib
import json
import os
from pathlib import Path
import re
import subprocess
import time
import sys
from postprocess import normalize
from numeric_bridges import prepare

ROOT = Path(__file__).resolve().parents[1]
BUILD = ROOT/'build/gnu-coreutils'
TOOL = os.environ.get('C2RUST', str(ROOT/'.tools/c2rust-build/release/c2rust'))
records = [json.loads(path.read_text()) for path in sorted((ROOT/'build/cc-records').glob('*.json'))]
entries = {}
for record in records:
    output = record['output']
    if not output.startswith('src/libsinglebin_') and output != 'src/coreutils-coreutils.o':
        continue
    result = subprocess.run(['nm', '-g', '--defined-only', '--format=posix', BUILD/output],
                            capture_output=True, text=True, check=True)
    for symbol in re.findall(r'^(single_binary_main_\w+) T ', result.stdout, re.M):
        assert symbol not in entries, symbol
        entries[symbol] = record
    if output == 'src/coreutils-coreutils.o':
        entries['single_binary_main_coreutils'] = record
names = dict((symbol, 'install' if name == 'ginstall' else name)
             for name, symbol in re.findall(r'SINGLE_BINARY_PROGRAM\("([^\"]+)", (\w+)\)',
                                             (BUILD/'src/coreutils.h').read_text()))
names['coreutils'] = 'coreutils'
entries = {symbol:record for symbol,record in entries.items()
           if symbol.removeprefix('single_binary_main_') in names}

def translate(item):
    symbol, original = item
    suffix = symbol.removeprefix('single_binary_main_')
    name = names[suffix]
    stage = ROOT/'build/translation'/suffix
    stage.mkdir(parents=True, exist_ok=True)
    args = list(original['arguments'])
    args[0] = 'clang-21'
    args += ['-std=gnu17', '-Dunreachable()=__builtin_unreachable()', '-Dalignof=_Alignof']
    if suffix == 'factor':
        args += ['-DNO_ASM=1']  # GNU longlong.h's own portable implementation.
    if suffix == 'wc':
        args += ['-include', str(ROOT/'src/bridges/cpu-supports.h')]
    if suffix == 'coreutils':
        args += ['-Dmain=single_binary_main_coreutils']
    adapted_source, numeric = prepare(ROOT, suffix, Path(original['file']), stage)
    if numeric:
        args = [str(adapted_source) if arg == original['file'] else arg for arg in args]
    command = {**original, 'file': str(adapted_source), 'arguments': args}
    database = stage/'compile_commands.json'
    database.write_text(json.dumps([command], indent=2)+'\n')
    log_path = ROOT/'evidence/raw'/f'transpile-{suffix}.log'
    start = time.monotonic()
    with log_path.open('w') as log:
        result = subprocess.run([TOOL, 'transpile', '--emit-modules', '--emit-no-std',
                                 '--fail-on-error', '--overwrite-existing', '--output-dir', str(stage/'out'),
                                 str(database)], stdout=log, stderr=subprocess.STDOUT,
                                env={**os.environ, 'RUSTUP_TOOLCHAIN':'1.93.0'})
    outputs = sorted((stage/'out').rglob('*.rs')) if (stage/'out').exists() else []
    row = {'name':name, 'entry':symbol, 'source':original['file'],
           'source_sha256':hashlib.sha256(Path(original['file']).read_bytes()).hexdigest(),
           'gnu_object':original['output'], 'exit_status':result.returncode,
           'elapsed_seconds':round(time.monotonic()-start,3), 'log':str(log_path.relative_to(ROOT))}
    if numeric:
        row['native_numeric_helpers'] = numeric
    if result.returncode == 0 and len(outputs) == 1:
        text = outputs[0].read_text()
        # These GNU opaque types are used only through pointers. Preserve
        # those pointers without requiring Rust's unstable extern_types.
        opaque = re.findall(r'^    pub type (\w+);$', text, re.M)
        text = re.sub(r'^    pub type \w+;\n', '', text, flags=re.M)
        declarations = ''.join(f'#[repr(C)]\npub struct {ty} {{ _opaque: [u8; 0] }}\n' for ty in opaque)
        if suffix in ('sleep', 'tail', 'timeout'):
            # dtimespec_bound receives double. Fold GNU's sizeof dispatch
            # for this pinned double=8/long-double=16 profile; do not route
            # x87 long double through C2Rust's IEEE binary128 type.
            text, folded = re.subn(r'if ::core::mem::size_of::<::core::ffi::c_double>\(\)\s*\n?\s*== ::core::mem::size_of::<::f128::f128>\(\).*?\n        } != 0',
                                  'c.is_sign_negative()', text, flags=re.S)
            assert folded == 1, (suffix, folded)
            text = re.sub(r'^use ::f128;\n|^use ::num_traits;\n|^use ::num_traits::Float;\n', '', text, flags=re.M)
            assert '::f128' not in text
        target = ROOT/'src/generated'/f'applet_{suffix}.rs'
        header = ('// Generated from GNU Coreutils 9.11; FSF copyright notices and GPL-3.0-or-later\n'
                  '// terms are retained in the pinned original source and COPYING.\n'
                  '// C2Rust e1e5bf257863107c54e9f42b345c2aeccd925458; regenerated by scripts/translate-coreutils.py.\n'
                  f'// Source SHA-256: {row["source_sha256"]}\n')
        original_text = Path(original['file']).read_text()
        notice = re.match(r'\s*(/\*.*?\*/)', original_text, re.S)
        copyright_notice = notice[1]+'\n' if notice else ''
        target.write_text(header + copyright_notice + declarations + normalize(suffix, text))
        row.update({'translated':True, 'rust_file':str(target.relative_to(ROOT)),
                    'opaque_pointer_types':opaque, 'generated_lines':len(text.splitlines())})
    else:
        row['translated'] = False
    print(f'{name}: {"translated" if row["translated"] else "FAILED"}', flush=True)
    return row

previous = {row['entry']:row for row in json.loads((ROOT/'evidence/translation.json').read_text())} if (ROOT/'evidence/translation.json').exists() else {}
selected = set(sys.argv[1:])
pending = sorted((symbol,record) for symbol,record in entries.items()
                 if not selected or symbol.removeprefix('single_binary_main_') in selected)
with ThreadPoolExecutor(max_workers=4) as pool:
    for row in pool.map(translate, pending): previous[row['entry']] = row
results = sorted(previous.values(), key=lambda row:row['name'])
(ROOT/'evidence/translation.json').write_text(json.dumps(results, indent=2)+'\n')
print(f'Translated {sum(row["translated"] for row in results)}/{len(results)} entry units')
