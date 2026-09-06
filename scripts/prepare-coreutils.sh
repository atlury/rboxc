#!/bin/sh
set -eu
root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
source=${GNU_COREUTILS_SOURCE:-/opt/src/coreutils-9.11}
build="$root/build/gnu-coreutils"
mkdir -p "$build" "$root/evidence/raw" "$root/build/cc-records"
cd "$build"
if [ ! -f Makefile ]; then
    FORCE_UNSAFE_CONFIGURE=1 "$source/configure" \
        --prefix="$root/build/oracle" \
        --enable-single-binary=symlinks \
        --enable-install-program=arch,hostname,kill,uptime \
        --enable-no-install-program=chcon,runcon \
        --without-selinux --without-libsmack \
        > "$root/evidence/raw/coreutils-configure.log" 2>&1
fi
RBOXC_CC_RECORDS="$root/build/cc-records" \
    make -j8 CC="python3 $root/scripts/record-cc.py" all \
    > "$root/evidence/raw/coreutils-build.log" 2>&1
