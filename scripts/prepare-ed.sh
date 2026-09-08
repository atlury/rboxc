#!/bin/sh
# SPDX-License-Identifier: GPL-3.0-or-later
set -eu
root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
source=${GNU_ED_SOURCE:-/opt/src/ed-1.22.6}
build="$root/build/gnu-ed"
mkdir -p "$build" "$root/evidence/raw" "$root/build/ed-cc-records"
python3 - "$root" "$source" <<'PY'
import hashlib,json,sys
from pathlib import Path
root,source=map(Path,sys.argv[1:])
pin=json.loads((root/'inventory/sources.json').read_text())['ed']
assert hashlib.sha256((source/'main.c').read_bytes()).hexdigest()==pin['entry_sha256']
PY
cd "$build"
if [ ! -f Makefile ]; then
    "$source/configure" --prefix="$root/build/oracle/ed" > "$root/evidence/raw/ed-configure.log" 2>&1
fi
RBOXC_CC_RECORDS="$root/build/ed-cc-records" \
    make -j8 CC="python3 $root/scripts/record-cc.py" all > "$root/evidence/raw/ed-build.log" 2>&1
python3 - "$root" <<'PY'
import hashlib,json,subprocess,sys
from pathlib import Path
root=Path(sys.argv[1])
h=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
report={'provider':'GNU Ed 1.22.6','scope':'Pinned native oracle and helper build. Runtime originals are separately inventoried and are not run by preparation.',
        'oracle_sha256':h('ed'),'version':subprocess.check_output(['./ed','--version'],text=True).splitlines()[0],
        'config_status_sha256':h('config.status'),'makefile_sha256':h('Makefile'),
        'native_helper_objects':{p.name:h(p) for p in sorted(Path('.').glob('*.o')) if p.name!='main.o'},
        'compiler':subprocess.check_output(['gcc','--version'],text=True).splitlines()[0]}
(root/'evidence/ed-build-profile.json').write_text(json.dumps(report,indent=2)+'\n')
PY
