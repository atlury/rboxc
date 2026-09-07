#!/bin/sh
set -eu
root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
source=${GNU_DIFFUTILS_SOURCE:-/opt/src/diffutils-3.12}
build="$root/build/gnu-diffutils"
mkdir -p "$build" "$root/evidence/raw" "$root/build/diffutils-cc-records"
python3 - "$root" "$source" <<'PY'
import hashlib, json
from pathlib import Path
import sys
root, source = map(Path, sys.argv[1:])
pin = json.loads((root/'inventory/sources.json').read_text())['diffutils']
for name, expected in pin['entry_sha256'].items():
    assert hashlib.sha256((source/f'src/{name}.c').read_bytes()).hexdigest() == expected
PY
cd "$build"
if [ ! -f Makefile ]; then
    "$source/configure" --prefix="$root/build/oracle/diffutils" \
        > "$root/evidence/raw/diffutils-configure.log" 2>&1
fi
RBOXC_CC_RECORDS="$root/build/diffutils-cc-records" \
    make -j8 CC="python3 $root/scripts/record-cc.py" all \
    > "$root/evidence/raw/diffutils-build.log" 2>&1
python3 - "$root" <<'PY'
import hashlib, json
from pathlib import Path
import subprocess, sys
root = Path(sys.argv[1])
def digest(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()
report = {'provider': 'GNU Diffutils 3.12',
          'scope': 'Authoritative native oracle and helper build; command translation and suite validation remain separate.',
          'configure_arguments': subprocess.check_output(['./config.status', '--config'], text=True).strip(),
          'config_header_sha256': digest('lib/config.h'),
          'oracles': {name: digest('src/'+name) for name in ('cmp', 'diff', 'diff3', 'sdiff')},
          'helper_archive_sha256': digest('lib/libdiffutils.a'),
          'version_archive_sha256': digest('src/libver.a'),
          'compiler': subprocess.check_output(['gcc', '--version'], text=True).splitlines()[0]}
(root/'evidence/diffutils-build-profile.json').write_text(json.dumps(report, indent=2)+'\n')
PY
