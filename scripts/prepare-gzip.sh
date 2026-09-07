#!/bin/sh
# SPDX-License-Identifier: GPL-3.0-or-later
set -eu
root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
source=${GNU_GZIP_SOURCE:-/opt/src/gzip-1.14}
build="$root/build/gnu-gzip"
mkdir -p "$build" "$root/evidence/raw" "$root/build/gzip-cc-records"
python3 - "$root" "$source" <<'PY'
import hashlib, json
from pathlib import Path
import sys
root, source = map(Path, sys.argv[1:])
pin = json.loads((root/'inventory/sources.json').read_text())['gzip']
assert hashlib.sha256((source/'gzip.c').read_bytes()).hexdigest() == pin['entry_sha256']
for name, expected in pin['alias_source_sha256'].items():
    assert hashlib.sha256((source/(name+'.in')).read_bytes()).hexdigest() == expected
PY
cd "$build"
if [ ! -f Makefile ]; then
    "$source/configure" --prefix="$root/build/oracle/gzip" > "$root/evidence/raw/gzip-configure.log" 2>&1
fi
RBOXC_CC_RECORDS="$root/build/gzip-cc-records" \
    make -j8 CC="python3 $root/scripts/record-cc.py" all > "$root/evidence/raw/gzip-build.log" 2>&1
python3 - "$root" <<'PY'
import hashlib, json
from pathlib import Path
import subprocess, sys
root = Path(sys.argv[1])
def digest(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()
report = {'provider':'GNU Gzip 1.14',
          'scope':'Authoritative native oracle and helper build; translation and original suite validation are separate.',
          'configure_arguments':subprocess.check_output(['./config.status','--config'],text=True).strip(),
          'config_header_sha256':digest('lib/config.h'),'oracle_sha256':digest('gzip'),
          'alias_sha256':{name:digest(name) for name in ('gunzip','zcat')},
          'helper_archive_sha256':{name:digest(name) for name in ('libver.a','lib/libgzip.a')},
          'compiler':subprocess.check_output(['gcc','--version'],text=True).splitlines()[0]}
(root/'evidence/gzip-build-profile.json').write_text(json.dumps(report,indent=2)+'\n')
PY
