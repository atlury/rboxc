#!/bin/sh
set -eu
root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
revision=e1e5bf257863107c54e9f42b345c2aeccd925458
source="$root/.tools/c2rust-source"
mkdir -p "$root/.tools" "$root/evidence/raw"
if [ ! -d "$source/.git" ]; then
    git clone --filter=blob:none --no-checkout https://github.com/immunant/c2rust.git "$source"
fi
if [ -n "$(git -C "$source" status --porcelain)" ] && [ -f "$source/Cargo.toml" ]; then
    echo 'Preserving modified C2Rust tool source; use a clean pinned checkout.' >&2
    exit 1
fi
git -C "$source" checkout --detach "$revision"
test "$(git -C "$source" rev-parse HEAD)" = "$revision"
LLVM_CONFIG_PATH=${LLVM_CONFIG_PATH:-/usr/bin/llvm-config-21} \
CLANG_PATH=${CLANG_PATH:-/usr/bin/clang-21} \
LIBCLANG_PATH=${LIBCLANG_PATH:-/usr/lib/llvm-21/lib} \
RUSTUP_TOOLCHAIN=1.93.0 CARGO_TARGET_DIR="$root/.tools/c2rust-build" \
    cargo build --manifest-path "$source/Cargo.toml" --locked --release -p c2rust \
    > "$root/evidence/raw/c2rust-build.log" 2>&1
"$root/.tools/c2rust-build/release/c2rust" --version
