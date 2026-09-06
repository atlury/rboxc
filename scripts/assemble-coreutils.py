#!/usr/bin/env python3
"""Link translated entries with GNU helper archives, excluding C entry objects."""
import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import sys
from native_cleanup import sort_cleanup

ROOT = Path(__file__).resolve().parents[1]
BUILD = ROOT/'build/gnu-coreutils'
SOURCE = Path(os.environ.get('GNU_COREUTILS_SOURCE', '/opt/src/coreutils-9.11'))
rows = json.loads((ROOT/'evidence/translation.json').read_text())
for row in rows:
    row['active_rust'] = row['translated'] and row['name'] not in ('printf', 'sort')
    if row['name'] in ('printf', 'sort'):
        row['hold_reason'] = 'x87 long double ABI requires native helper boundary; generated IEEE binary128 is not equivalent'
failed = [row['name'] for row in rows if not row['active_rust']]
if failed and '--allow-c-entries' not in sys.argv:
    raise SystemExit('Translation incomplete: ' + ', '.join(failed))
modules = []
registry = []
for row in rows:
    if row['active_rust']:
        module = Path(row['rust_file']).stem
        modules.append(f'#[path = "generated/{module}.rs"]\nmod {module};\n')
        function = f'{module}::{row["entry"]}'
    else:
        function = row['entry']
        modules.append(f'extern "C" {{ fn {function}(argc: c_int, argv: *mut *mut c_char) -> c_int; }}\n')
    registry.append(f'    (b"{row["name"]}", {function}),\n')
(ROOT/'src/registry.rs').write_text(''.join(modules)+
    '\ntype Entry = unsafe extern "C" fn(c_int, *mut *mut c_char) -> c_int;\n'+
    'static APPLETS: &[(&[u8], Entry)] = &[\n'+''.join(registry)+'];\n')

# Use the authoritative GNU link order. Remove every C command entry object
# from helper copies so a missing Rust entry can never silently fall back to C.
print_makefile = ROOT/'build/print-link.mk'
print_makefile.write_text(".PHONY: rboxc-print\nrboxc-print:\n\t@printf '%s\\n' $(src_coreutils_LDADD)\n")
inputs = subprocess.check_output(['make', '--no-print-directory', '-s', '-f', 'Makefile',
                                  '-f', str(print_makefile), 'rboxc-print'], cwd=BUILD, text=True).splitlines()
target = ROOT/'build/helpers'
target.mkdir(exist_ok=True)
entries = {Path(row['gnu_object']).name for row in rows if row['active_rust']}
allowed = {row['entry'] for row in rows if not row['active_rust']}
prepared = {}
link = []
native_changes = []
for item in inputs:
    if item.startswith('-'):
        link.append(item)
        continue
    path = BUILD/item
    if path.name.startswith('libsinglebin_'):
        if item not in prepared:
            copied = target/path.name
            shutil.copy2(path, copied)
            members = subprocess.check_output(['ar', 't', copied], text=True).splitlines()
            remove = [member for member in members if member in entries]
            if remove:
                subprocess.run(['ar', 'd', copied, *remove], check=True)
            if path.name == 'libsinglebin_sort.a' and 'single_binary_main_sort' in allowed:
                row = next(row for row in rows if row['name'] == 'sort')
                obj, adaptation = sort_cleanup(ROOT, row)
                subprocess.run(['ar', 'r', copied, obj], check=True)
                native_changes.append(adaptation)
            symbols = subprocess.check_output(['nm', '-g', '--defined-only', copied], text=True, stderr=subprocess.DEVNULL)
            remaining = set(re.findall(r' T (single_binary_main_\w+)', symbols))
            assert remaining <= allowed, (item, remaining-allowed)
            prepared[item] = copied
        path = prepared[item]
    link.append(str(path))
bridge = target/'cpu-supports.o'
subprocess.run(['gcc', '-O2', '-c', ROOT/'src/bridges/cpu-supports.c', '-o', bridge], check=True)
link.append(str(bridge))
allocation_bridge = target/'aligned-alloc.o'
subprocess.run(['gcc', '-O2', '-Wall', '-Wextra', '-Werror', '-c',
                ROOT/'src/bridges/aligned-alloc.c', '-o', allocation_bridge], check=True)
link.extend([str(allocation_bridge), '-Wl,--wrap=aligned_alloc'])
system = (SOURCE/'src/system.h').read_text()
begin = system.index('static inline void\noprintf_ (')
opening = system.index('{', begin)
depth, end = 1, opening + 1
while depth:
    depth += (system[end] == '{') - (system[end] == '}')
    end += 1
body = system[begin:end].replace('static inline void\noprintf_ (', 'void\nrboxc_oprintf (', 1)
source = ROOT/'src/bridges/oprintf.c'
source.write_text(system[:system.index('*/')+2]+'\n#include <config.h>\n#include "system.h"\n'+body+'\n')
formatted = target/'oprintf.o'
subprocess.run(['gcc', '-O2', '-I'+str(BUILD/'lib'), '-I'+str(SOURCE/'lib'),
                '-I'+str(BUILD/'src'), '-I'+str(SOURCE/'src'), '-c', source, '-o', formatted], check=True)
# Put this helper before archives that satisfy its own dependencies.
link.insert(0, str(formatted))
(ROOT/'build/rust-link-inputs.txt').write_text('\n'.join(link)+'\n')
(ROOT/'evidence/link.json').write_text(json.dumps({'entries':len(rows),
    'rust_entries':sum(row['active_rust'] for row in rows), 'temporary_C_entries':failed,
    'C_entry_objects_removed_for_all_active_Rust_commands':True, 'helper_archives':list(prepared),
    'aligned_allocation_adapter': 'round backing size to alignment multiple; GNU oracle unchanged',
    'native_entry_cleanups': native_changes}, indent=2)+'\n')
(ROOT/'evidence/translation.json').write_text(json.dumps(rows,indent=2)+'\n')
print(f'Prepared {len(rows)-len(failed)} Rust entries, {len(failed)} explicit temporary C entries, and {len(prepared)} helper archives')
