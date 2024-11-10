#!/bin/bash
#Generated automatically by rrtk 0.6.0-alpha.1
set -e
echo
cargo test --no-default-features
echo alloc
cargo test --no-default-features --features alloc
echo std
cargo test --no-default-features --features std
echo devices
cargo test --no-default-features --features devices
echo dim_check_debug
cargo test --no-default-features --features dim_check_debug
echo dim_check_release
cargo test --no-default-features --features dim_check_release
echo alloc devices
cargo test --no-default-features --features alloc,devices
echo alloc dim_check_debug
cargo test --no-default-features --features alloc,dim_check_debug
echo alloc dim_check_release
cargo test --no-default-features --features alloc,dim_check_release
echo std devices
cargo test --no-default-features --features std,devices
echo std dim_check_debug
cargo test --no-default-features --features std,dim_check_debug
echo std dim_check_release
cargo test --no-default-features --features std,dim_check_release
echo devices dim_check_debug
cargo test --no-default-features --features devices,dim_check_debug
echo devices dim_check_release
cargo test --no-default-features --features devices,dim_check_release
echo alloc devices dim_check_debug
cargo test --no-default-features --features alloc,devices,dim_check_debug
echo alloc devices dim_check_release
cargo test --no-default-features --features alloc,devices,dim_check_release
echo std devices dim_check_debug
cargo test --no-default-features --features std,devices,dim_check_debug
echo std devices dim_check_release
cargo test --no-default-features --features std,devices,dim_check_release
