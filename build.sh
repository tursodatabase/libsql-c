#!/usr/bin/env sh

set -xe

cargo build --target x86_64-apple-darwin --features encryption --release
cargo build --target aarch64-apple-darwin --features encryption --release
cargo build --target aarch64-unknown-linux-gnu --features encryption --release
cargo build --target x86_64-unknown-linux-gnu --features encryption --release
cargo build --target x86_64-pc-windows-gnu --features encryption --release

RUSTFLAGS="-C target-feature=-crt-static" cargo build --target aarch64-unknown-linux-musl --features encryption --release
RUSTFLAGS="-C target-feature=-crt-static" cargo build --target x86_64-unknown-linux-musl --features encryption --release
