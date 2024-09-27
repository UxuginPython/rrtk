#!/bin/bash
#Generated automatically by rrtk 0.5.0-alpha.1
set -e
echo
cargo test --no-default-features
echo alloc
cargo test --no-default-features --features alloc
echo std
cargo test --no-default-features --features std
echo devices
cargo test --no-default-features --features devices
echo alloc devices
cargo test --no-default-features --features alloc,devices
echo std devices
cargo test --no-default-features --features std,devices
