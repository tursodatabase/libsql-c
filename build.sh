#!/usr/bin/env sh

set -xe

cargo build --target aarch64-apple-darwin --features encryption --release
cargo build --target aarch64-unknown-linux-gnu --features encryption --release
cargo build --target x86_64-unknown-linux-gnu --features encryption --release
