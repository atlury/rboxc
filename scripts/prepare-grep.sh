#!/bin/sh
# SPDX-License-Identifier: GPL-3.0-or-later
set -eu
root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
source=${GNU_GREP_SOURCE:-/opt/src/grep-3.12}
build="$root/build/gnu-grep"
mkdir -p "$build" "$root/evidence/raw" "$root/build/grep-cc-records"
python3 - "$root" "$source" <<'PY'
import hashlib, json
from pathlib import Path
import sys
root, source = map(Path, sys.argv[1:])
pin = json.loads((root/'inventory/sources.json').read_text())['grep']
assert hashlib.sha256((source/'src/grep.c').read_bytes()).hexdigest() == pin['entry_sha256']
assert hashlib.sha256((source/'src/egrep.sh').read_bytes()).hexdigest() == pin['alias_script_sha256']
PY
pkg-config --exists libpcre2-8 || { echo "GNU Grep preparation requires PCRE2 development headers" >&2; exit 1; }
cd "$build"
if [ ! -f Makefile ] || [ "${1:-}" = --reconfigure ]; then
    "$source/configure" --prefix="$root/build/oracle/grep" \
        > "$root/evidence/raw/grep-configure.log" 2>&1
fi
python3 - <<'PY_CHECK'
from pathlib import Path
assert '#define HAVE_LIBPCRE 1' in Path('config.h').read_text(), 'Reconfigure GNU Grep with PCRE2 support'
PY_CHECK
RBOXC_CC_RECORDS="$root/build/grep-cc-records" \
    make -j8 CC="python3 $root/scripts/record-cc.py" all \
    > "$root/evidence/raw/grep-build.log" 2>&1
python3 - "$root" <<'PY'
import hashlib, json
from pathlib import Path
import subprocess, sys
root = Path(sys.argv[1])
def digest(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()
report = {'provider': 'GNU Grep 3.12',
          'scope': 'Authoritative native oracle and helper build; entry translation and runtime suite validation are separate.',
          'configure_arguments': subprocess.check_output(['./config.status', '--config'], text=True).strip(),
          'config_header_sha256': digest('config.h'), 'oracle_sha256': digest('src/grep'),
          'pcre2_version': subprocess.check_output(['pkg-config', '--modversion', 'libpcre2-8'], text=True).strip(),
          'pcre_enabled': '#define HAVE_LIBPCRE 1' in Path('config.h').read_text(),
          'alias_sha256': {name: digest('src/'+name) for name in ('egrep', 'fgrep')},
          'helper_archive_sha256': digest('lib/libgreputils.a'),
          'compiler': subprocess.check_output(['gcc', '--version'], text=True).splitlines()[0]}
(root/'evidence/grep-build-profile.json').write_text(json.dumps(report, indent=2)+'\n')
PY
