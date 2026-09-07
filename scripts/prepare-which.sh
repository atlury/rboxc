#!/bin/sh
set -eu
root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
source=${GNU_WHICH_SOURCE:-/opt/src/which-2.25}
build="$root/build/gnu-which"
mkdir -p "$build" "$root/evidence/raw" "$root/build/which-cc-records"
python3 - "$root" "$source" <<'PY'
import hashlib, json
from pathlib import Path
import sys
root, source = map(Path, sys.argv[1:])
pin = json.loads((root/'inventory/sources.json').read_text())['which']
assert hashlib.sha256((source/'which.c').read_bytes()).hexdigest() == pin['entry_sha256']
PY
cd "$build"
if [ ! -f Makefile ]; then
    "$source/configure" --prefix="$root/build/oracle/which" \
        > "$root/evidence/raw/which-configure.log" 2>&1
fi
RBOXC_CC_RECORDS="$root/build/which-cc-records" \
    make -j8 CC="python3 $root/scripts/record-cc.py" all \
    > "$root/evidence/raw/which-build.log" 2>&1
python3 - "$root" <<'PY'
import hashlib, json
from pathlib import Path
import subprocess, sys
root = Path(sys.argv[1])
def digest(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()
report = {'provider': 'GNU Which 2.25',
          'scope': 'Authoritative native oracle and helper build; no runtime test scripts are registered by the source Makefile.',
          'configure_arguments': subprocess.check_output(['./config.status', '--config'], text=True).strip(),
          'config_header_sha256': digest('config.h'), 'oracle_sha256': digest('which'),
          'native_helper_objects': {name: digest(name) for name in ('getopt.o', 'getopt1.o', 'bash.o', 'tilde.o')},
          'compiler': subprocess.check_output(['gcc', '--version'], text=True).splitlines()[0]}
(root/'evidence/which-build-profile.json').write_text(json.dumps(report, indent=2)+'\n')
PY
