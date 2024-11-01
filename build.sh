#!/usr/bin/env sh

set -xe

cargo zigbuild --target aarch64-unknown-linux-gnu --features encryption --release
cargo zigbuild --target x86_64-unknown-linux-gnu --features encryption --release
cargo zigbuild --target universal2-apple-darwin --features encryption --release
