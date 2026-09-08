#!/bin/sh
# SPDX-License-Identifier: GPL-3.0-or-later
set -eu
root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
source=${GNU_TAR_SOURCE:-/opt/src/tar-1.35}
build="$root/build/gnu-tar"
mkdir -p "$build" "$root/evidence/raw" "$root/build/tar-cc-records"
python3 - "$root" "$source" <<'PY'
import hashlib,json,sys
from pathlib import Path
root,source=map(Path,sys.argv[1:]);pin=json.loads((root/'inventory/sources.json').read_text())['tar']
assert hashlib.sha256((source/pin['entry_source']).read_bytes()).hexdigest()==pin['entry_sha256']
PY
cd "$build"
if [ ! -f Makefile ]; then
    FORCE_UNSAFE_CONFIGURE=1 "$source/configure" --without-selinux --prefix="$root/build/oracle/tar" > "$root/evidence/raw/tar-configure.log" 2>&1
fi
RBOXC_CC_RECORDS="$root/build/tar-cc-records" \
    make -j8 CC="python3 $root/scripts/record-cc.py" all > "$root/evidence/raw/tar-build.log" 2>&1
python3 - "$root" <<'PY'
import hashlib,json,subprocess,sys
from pathlib import Path
root=Path(sys.argv[1]);h=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
needed=subprocess.check_output(['readelf','-d','src/tar'],text=True)
assert 'libselinux' not in needed
report={'provider':'GNU Tar 1.35','scope':'Pinned native oracle and helper build. SELinux disabled; runtime originals and the Rust port are separate.',
        'configure_arguments':subprocess.check_output(['./config.status','--config'],text=True).strip(),
        'config_header_sha256':h('config.h'),'oracle_sha256':h('src/tar'),
        'version':subprocess.check_output(['src/tar','--version'],text=True).splitlines()[0],
        'native_helper_objects':{str(p):h(p) for p in sorted(Path('src').glob('*.o')) if p.name!='tar.o'},
        'helper_archives':{p:h(p) for p in ('lib/libtar.a','gnu/libgnu.a')},
        'compiler':subprocess.check_output(['gcc','--version'],text=True).splitlines()[0]}
(root/'evidence/tar-build-profile.json').write_text(json.dumps(report,indent=2)+'\n')
PY
