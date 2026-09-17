#!/bin/sh
set -eu
root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
source=${GNU_COREUTILS_SOURCE:-/opt/src/coreutils-9.11}
build="$root/build/gnu-coreutils"
reconfigure=no
case ${1-} in
    '') ;;
    --reconfigure) reconfigure=yes; shift ;;
    *) echo "usage: $0 [--reconfigure]" >&2; exit 2 ;;
esac
[ "$#" -eq 0 ] || { echo "usage: $0 [--reconfigure]" >&2; exit 2; }
mkdir -p "$build" "$root/evidence/raw" "$root/build/cc-records"
cd "$build"
if [ ! -f Makefile ] || [ "$reconfigure" = yes ]; then
    FORCE_UNSAFE_CONFIGURE=1 "$source/configure" \
        --prefix="$root/build/oracle" \
        --enable-single-binary=symlinks \
        --enable-install-program=arch,hostname,kill,uptime \
        --enable-no-install-program=chcon,runcon \
        --enable-acl --enable-xattr \
        --enable-libcap \
        --without-selinux --without-libsmack \
        > "$root/evidence/raw/coreutils-configure.log" 2>&1
fi
python3 - <<'PY'
from pathlib import Path
header = Path('lib/config.h').read_text().splitlines()
for feature in ('USE_ACL', 'USE_XATTR', 'HAVE_CAP'):
    if f'#define {feature} 1' not in header:
        raise SystemExit(f'{feature} is disabled: install libacl/libattr/libcap development headers '
                         'and rerun sh scripts/prepare-coreutils.sh --reconfigure')
PY
RBOXC_CC_RECORDS="$root/build/cc-records" \
    make -j8 CC="python3 $root/scripts/record-cc.py" all \
    > "$root/evidence/raw/coreutils-build.log" 2>&1
python3 - "$root" <<'PY'
import hashlib
import json
from pathlib import Path
import subprocess
import sys
root = Path(sys.argv[1])
def digest(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()
report = {
    'provider': 'GNU Coreutils 9.11',
    'features': {'acl': True, 'xattr': True, 'capabilities': True, 'selinux': False, 'smack': False},
    'configure_arguments': subprocess.check_output(['./config.status', '--config'], text=True).strip(),
    'config_header_sha256': digest('lib/config.h'),
    'oracle_sha256': digest('src/coreutils'),
    'compiler': subprocess.check_output(['gcc', '--version'], text=True).splitlines()[0],
}
(root/'evidence/gnu-build-profile.json').write_text(json.dumps(report, indent=2)+'\n')
PY
