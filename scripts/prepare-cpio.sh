#!/bin/sh
# SPDX-License-Identifier: GPL-3.0-or-later
set -eu
root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
source=${GNU_CPIO_SOURCE:-/opt/src/cpio-2.15}
build="$root/build/gnu-cpio"
mkdir -p "$build" "$root/evidence/raw" "$root/build/cpio-cc-records"
python3 - "$root" "$source" <<'PY'
import hashlib,json,sys
from pathlib import Path
root,source=map(Path,sys.argv[1:])
pin=json.loads((root/'inventory/sources.json').read_text())['cpio']
for name,expected in {**pin['entry_source_sha256'], **pin.get('source_and_header_sha256',{})}.items():
    assert hashlib.sha256((source/name).read_bytes()).hexdigest()==expected
PY
cd "$build"
if [ ! -f Makefile ]; then
    CFLAGS='-g -O2 -std=gnu17' "$source/configure" --enable-mt --prefix="$root/build/oracle/cpio" > "$root/evidence/raw/cpio-configure.log" 2>&1
fi
export RBOXC_CC_RECORDS="$root/build/cpio-cc-records"
make -j8 CC="python3 $root/scripts/record-cc.py" > "$root/evidence/raw/cpio-native-build.log" 2>&1
make -C tests -j8 CC="python3 $root/scripts/record-cc.py" genfile > "$root/evidence/raw/cpio-genfile-build.log" 2>&1
python3 - "$root" "$source" <<'PY'
import hashlib,json,subprocess,sys
from pathlib import Path
root,source=map(Path,sys.argv[1:]);h=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
helpers=[*sorted(Path('src').glob('*.o')),Path('lib/libpax.a'),Path('gnu/libgnu.a')]
profile={'provider':'GNU Cpio 2.15','scope':'Pinned native cpio/mt oracles and GNU helpers; GNU17 parser selected. No original C source edits.',
         'configure_arguments':subprocess.check_output(['./config.status','--config'],text=True).strip(),
         'config_header_sha256':h('config.h'),
         'oracles':{n:{'sha256':h('src/'+n),'version':subprocess.check_output(['src/'+n,'--version'],text=True).splitlines()[0]} for n in ('cpio','mt')},
         'native_inputs':{str(p):h(p) for p in helpers},
         'compiler':subprocess.check_output(['gcc','--version'],text=True).splitlines()[0]}
(root/'evidence/cpio-build-profile.json').write_text(json.dumps(profile,indent=2)+'\n')
PY
