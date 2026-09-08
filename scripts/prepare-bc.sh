#!/bin/sh
# SPDX-License-Identifier: GPL-3.0-or-later
set -eu
root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
source=${GNU_BC_SOURCE:-/opt/src/bc-1.08.2}
build="$root/build/gnu-bc"
mkdir -p "$build" "$root/evidence/raw" "$root/build/bc-cc-records"
python3 - "$root" "$source" <<'PY'
import hashlib, json
from pathlib import Path
import sys
root, source = map(Path, sys.argv[1:])
pin = json.loads((root/'inventory/sources.json').read_text())['bc']
for name, entry in pin['entry_source'].items():
    assert hashlib.sha256((source/entry).read_bytes()).hexdigest() == pin['entry_sha256'][name]
PY
cd "$build"
if [ ! -f Makefile ]; then
    "$source/configure" --without-readline --without-libedit --prefix="$root/build/oracle/bc" > "$root/evidence/raw/bc-configure.log" 2>&1
fi
RBOXC_CC_RECORDS="$root/build/bc-cc-records" \
    make -j8 SUBDIRS='lib bc dc' CC="python3 $root/scripts/record-cc.py" all > "$root/evidence/raw/bc-build.log" 2>&1
python3 - "$root" <<'PY'
import hashlib, json
from pathlib import Path
import subprocess, sys
root = Path(sys.argv[1])
def digest(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()
report = {'provider':'GNU BC 1.08.2',
          'scope':'Pinned native bc/dc oracles without optional line-editing libraries. Translation and original input validation remain separate.',
          'build_subdirectories':['lib','bc','dc'],
          'documentation_build':'Not part of this oracle build; the initial documentation attempt requires unavailable makeinfo.',
          'configure_arguments':subprocess.check_output(['./config.status','--config'],text=True).strip(),
          'config_header_sha256':digest('config.h'),
          'oracles':{name:{'sha256':digest(name+'/'+name),'version':subprocess.check_output([name+'/'+name,'--version'],text=True).splitlines()[0]} for name in ('bc','dc')},
          'helper_archive_sha256':digest('lib/libbc.a'),
          'compiler':subprocess.check_output(['gcc','--version'],text=True).splitlines()[0]}
(root/'evidence/bc-build-profile.json').write_text(json.dumps(report,indent=2)+'\n')
PY
