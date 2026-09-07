#!/bin/sh
set -eu
root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
source=${GNU_TIME_SOURCE:-/opt/src/time-1.10}
build="$root/build/gnu-time"
mkdir -p "$build" "$root/evidence/raw" "$root/build/time-cc-records"
python3 - "$root" "$source" <<'PY'
import hashlib, json
from pathlib import Path
import sys
root, source = map(Path, sys.argv[1:])
pin = json.loads((root/'inventory/sources.json').read_text())['time']
assert hashlib.sha256((source/'src/time.c').read_bytes()).hexdigest() == pin['entry_sha256']
PY
cd "$build"
if [ ! -f Makefile ]; then
    "$source/configure" --prefix="$root/build/oracle/time" \
        > "$root/evidence/raw/time-configure.log" 2>&1
fi
RBOXC_CC_RECORDS="$root/build/time-cc-records" \
    make -j8 CC="python3 $root/scripts/record-cc.py" all tests/time-aux \
    > "$root/evidence/raw/time-build.log" 2>&1
python3 - "$root" <<'PY'
import hashlib, json
from pathlib import Path
import subprocess, sys
root = Path(sys.argv[1])
def digest(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()
report = {'provider': 'GNU Time 1.10',
          'scope': 'Authoritative native oracle and helper build; not a Rust port.',
          'configure_arguments': subprocess.check_output(['./config.status', '--config'], text=True).strip(),
          'config_header_sha256': digest('config.h'),
          'oracle_sha256': digest('src/time'),
          'helper_archive_sha256': digest('lib/libtime.a'),
          'version_archive_sha256': digest('src/libver.a'),
          'resource_helper_sha256': digest('src/rusage-kb.o'),
          'test_helper_sha256': digest('tests/time-aux'),
          'compiler': subprocess.check_output(['gcc', '--version'], text=True).splitlines()[0]}
(root/'evidence/time-build-profile.json').write_text(json.dumps(report, indent=2)+'\n')
PY
