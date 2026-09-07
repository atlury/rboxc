#!/bin/sh
# SPDX-License-Identifier: GPL-3.0-or-later
set -eu
root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
source=${GNU_SED_SOURCE:-/opt/src/sed-4.10}
build="$root/build/gnu-sed"
mkdir -p "$build" "$root/evidence/raw" "$root/build/sed-cc-records"
python3 - "$root" "$source" <<'PY'
import hashlib, json
from pathlib import Path
import sys
root, source = map(Path, sys.argv[1:])
pin = json.loads((root/'inventory/sources.json').read_text())['sed']
assert hashlib.sha256((source/'sed/sed.c').read_bytes()).hexdigest() == pin['entry_sha256']

PY
cd "$build"
if [ ! -f Makefile ]; then
    "$source/configure" --without-selinux --prefix="$root/build/oracle/sed" > "$root/evidence/raw/sed-configure.log" 2>&1
fi
RBOXC_CC_RECORDS="$root/build/sed-cc-records" \
    make -j8 CC="python3 $root/scripts/record-cc.py" all > "$root/evidence/raw/sed-build.log" 2>&1
python3 - "$root" <<'PY'
import hashlib, json
from pathlib import Path
import subprocess, sys
root = Path(sys.argv[1])
def digest(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()
needed = subprocess.check_output(['readelf', '-d', 'sed/sed'], text=True)
assert 'libselinux' not in needed
report = {'provider':'GNU Sed 4.10',
          'scope':'Authoritative native oracle and helper build; translation and original suite validation are separate.',
          'configure_arguments':subprocess.check_output(['./config.status','--config'],text=True).strip(),
          'config_header_sha256':digest('config.h'),'oracle_sha256':digest('sed/sed'),
          'helper_archive_sha256':{name:digest(name) for name in ('sed/libver.a','lib/libsed.a')},
          'selinux_enabled':False,
          'compiler':subprocess.check_output(['gcc','--version'],text=True).splitlines()[0]}
(root/'evidence/sed-build-profile.json').write_text(json.dumps(report,indent=2)+'\n')
PY
