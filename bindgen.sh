#!/usr/bin/env sh

set -xe

bindgen libsql.h -o src/bindings.rs \
    --with-derive-default \
    --allowlist-type "libsql_.*_t" \
    --allowlist-function "libsql_.*" \
    --rustified-enum "libsql_type_t" \
    --rustified-enum "libsql_cypher_t" \
    --rustified-enum "libsql_tracing_level_t"
