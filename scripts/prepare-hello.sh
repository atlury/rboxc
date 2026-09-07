#!/bin/sh
set -eu
root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
source=${GNU_HELLO_SOURCE:-/opt/src/hello-2.12.3}
build="$root/build/gnu-hello"
mkdir -p "$build" "$root/evidence/raw" "$root/build/hello-cc-records"
python3 - "$root" "$source" <<'PY'
import hashlib, json
from pathlib import Path
import sys
root, source = map(Path, sys.argv[1:])
pin = json.loads((root/'inventory/sources.json').read_text())['hello']
assert hashlib.sha256((source/'src/hello.c').read_bytes()).hexdigest() == pin['entry_sha256']
PY
cd "$build"
if [ ! -f Makefile ]; then
    "$source/configure" --prefix="$root/build/oracle/hello" \
        > "$root/evidence/raw/hello-configure.log" 2>&1
fi
RBOXC_CC_RECORDS="$root/build/hello-cc-records" \
    make -j8 CC="python3 $root/scripts/record-cc.py" all \
    > "$root/evidence/raw/hello-build.log" 2>&1
python3 - "$root" <<'PY'
import hashlib, json
from pathlib import Path
import subprocess, sys
root = Path(sys.argv[1])
def digest(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()
report = {'provider': 'GNU Hello 2.12.3',
          'scope': 'Authoritative native oracle and helper build; not a Rust port.',
          'configure_arguments': subprocess.check_output(['./config.status', '--config'], text=True).strip(),
          'config_header_sha256': digest('config.h'),
          'oracle_sha256': digest('hello'),
          'helper_archive_sha256': digest('lib/libhello.a'),
          'compiler': subprocess.check_output(['gcc', '--version'], text=True).splitlines()[0]}
(root/'evidence/hello-build-profile.json').write_text(json.dumps(report, indent=2)+'\n')
PY
