#!/usr/bin/env sh

set -xe

RUSTFLAGS+="--remap-path-prefix=$HOME=~"

extra=${1:-}

cargo build --target x86_64-apple-darwin --features encryption $extra
cargo build --target aarch64-apple-darwin --features encryption $extra
cargo build --target aarch64-unknown-linux-gnu --features encryption $extra
cargo build --target x86_64-unknown-linux-gnu --features encryption $extra 
cargo build --target x86_64-pc-windows-gnu --features encryption $extra

RUSTFLAGS+=" -C target-feature=-crt-static" cargo build --target aarch64-unknown-linux-musl --features encryption $extra
RUSTFLAGS+=" -C target-feature=-crt-static" cargo build --target x86_64-unknown-linux-musl --features encryption $extra
