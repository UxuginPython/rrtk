#!/bin/bash
#Generated automatically by rrtk 0.5.0
set -e
echo
cargo check --no-default-features
echo alloc
cargo check --no-default-features --features alloc
echo std
cargo check --no-default-features --features std
echo devices
cargo check --no-default-features --features devices
echo alloc devices
cargo check --no-default-features --features alloc,devices
echo std devices
cargo check --no-default-features --features std,devices
