#!/usr/bin/env sh

set -xe

cargo zigbuild --target x86_64-unknown-linux-gnu --release
cargo zigbuild --target universal2-apple-darwin --release
