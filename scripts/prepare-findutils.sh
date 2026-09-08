#!/bin/sh
# SPDX-License-Identifier: GPL-3.0-or-later
set -eu
root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
source=${GNU_FINDUTILS_SOURCE:-/opt/src/findutils-4.11.0}
build="$root/build/gnu-findutils"
mkdir -p "$build" "$root/evidence/raw" "$root/build/findutils-cc-records"
python3 - "$root" "$source" <<'PY'
import hashlib,json,sys
from pathlib import Path
root,source=map(Path,sys.argv[1:]);pin=json.loads((root/'inventory/sources.json').read_text())['findutils']
for name,path in pin['entry_source'].items():assert hashlib.sha256((source/path).read_bytes()).hexdigest()==pin['entry_sha256'][name]
PY
cd "$build"
if [ ! -f Makefile ]; then
    "$source/configure" --without-selinux --prefix="$root/build/oracle/findutils" > "$root/evidence/raw/findutils-configure.log" 2>&1
fi
RBOXC_CC_RECORDS="$root/build/findutils-cc-records" \
    make -j8 CC="python3 $root/scripts/record-cc.py" all > "$root/evidence/raw/findutils-build.log" 2>&1
python3 - "$root" <<'PY'
import hashlib,json,subprocess,sys
from pathlib import Path
root=Path(sys.argv[1]);h=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
names={'find':'find/find','xargs':'xargs/xargs','locate':'locate/locate','frcode':'locate/frcode'}
needed={n:subprocess.check_output(['readelf','-d',p],text=True) for n,p in names.items()}
assert all('libselinux' not in output for output in needed.values())
report={'provider':'GNU Findutils 4.11.0','scope':'Pinned native oracles, private database encoder, and configured updatedb shell program. SELinux disabled; runtime tests and ports are separate.',
        'configure_arguments':subprocess.check_output(['./config.status','--config'],text=True).strip(),
        'config_header_sha256':h('config.h'),'oracles':{n:{'path':str(root/'build/gnu-findutils'/p),'sha256':h(p)} for n,p in names.items()},
        'configured_updatedb_sha256':h('locate/updatedb'),
        'helper_archives':{p:h(p) for p in ('find/libfindtools.a','lib/libfind.a','gl/lib/libgnulib.a')},
        'compiler':subprocess.check_output(['gcc','--version'],text=True).splitlines()[0]}
(root/'evidence/findutils-build-profile.json').write_text(json.dumps(report,indent=2)+'\n')
PY
