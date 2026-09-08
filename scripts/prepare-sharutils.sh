#!/bin/sh
# SPDX-License-Identifier: GPL-3.0-or-later
set -eu
root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
source=${GNU_SHARUTILS_SOURCE:-/opt/src/sharutils-4.15.2}
build="$root/build/gnu-sharutils"
mkdir -p "$build" "$root/evidence/raw" "$root/build/sharutils-cc-records"
python3 - "$root" "$source" <<'PY'
import hashlib,json,sys
from pathlib import Path
root,source=map(Path,sys.argv[1:])
pin=json.loads((root/'inventory/sources.json').read_text())['sharutils']
for name,expected in pin['entry_source_sha256'].items():
    assert hashlib.sha256((source/name).read_bytes()).hexdigest()==expected
PY
cd "$build"
if [ ! -f Makefile ]; then
    CFLAGS='-g -O2 -std=gnu11' "$source/configure" --prefix="$root/build/oracle/sharutils" > "$root/evidence/raw/sharutils-configure-gnu11.log" 2>&1
fi
export RBOXC_CC_RECORDS="$root/build/sharutils-cc-records"
# 'all' builds the generated portability headers before compiling the library.
make -C lib -j8 CC="python3 $root/scripts/record-cc.py" all > "$root/evidence/raw/sharutils-lib-all-build.log" 2>&1
make -C libopts -j8 CC="python3 $root/scripts/record-cc.py" libopts.a > "$root/evidence/raw/sharutils-libopts-build.log" 2>&1
# The released option headers define a tentative program_name in both units.
# GNU's historical common-symbol semantics are required on current GCC.
make -C src -j8 CC="python3 $root/scripts/record-cc.py" CFLAGS='-g -O2 -std=gnu11 -fcommon' \
    -W "$source/src/uuencode.c" -W "$source/src/uudecode.c" \
    -W "$source/src/uuencode-opts.c" -W "$source/src/uudecode-opts.c" \
    uuencode uudecode > "$root/evidence/raw/sharutils-entries-fcommon-build.log" 2>&1
python3 - "$root" "$source" <<'PY'
import hashlib,json,subprocess,sys
from pathlib import Path
root,source=map(Path,sys.argv[1:]);h=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
commands=('uuencode','uudecode')
profile={'provider':'GNU Sharutils 4.15.2','scope':'Two pinned native encoding oracles; GNU11 and historical common symbols restore released build semantics. No C source edits. Rust entries and runtime tests are separate.',
         'configure_arguments':subprocess.check_output(['./config.status','--config'],text=True).strip(),
         'entry_cflags':'-g -O2 -std=gnu11 -fcommon','config_header_sha256':h('config.h'),
         'oracles':{n:{'sha256':h('src/'+n),'version':subprocess.check_output(['src/'+n,'--version'],text=True).splitlines()[0]} for n in commands},
         'helper_inputs':{p:h(p) for p in ['src/uuencode-opts.o','src/uudecode-opts.o','lib/libgnu.a','libopts/libopts.a']},
         'compiler':subprocess.check_output(['gcc','--version'],text=True).splitlines()[0]}
(root/'evidence/sharutils-build-profile.json').write_text(json.dumps(profile,indent=2)+'\n')
PY
