#!/bin/bash
#Generated automatically by rrtk 0.7.0-beta.3
set -e
echo
cargo test --no-default-features
echo alloc
cargo test --no-default-features --features alloc
echo std
cargo test --no-default-features --features std
echo devices
cargo test --no-default-features --features devices
echo libm
cargo test --no-default-features --features libm
echo micromath
cargo test --no-default-features --features micromath
echo alloc devices
cargo test --no-default-features --features alloc,devices
echo alloc libm
cargo test --no-default-features --features alloc,libm
echo alloc micromath
cargo test --no-default-features --features alloc,micromath
echo std devices
cargo test --no-default-features --features std,devices
echo std libm
cargo test --no-default-features --features std,libm
echo std micromath
cargo test --no-default-features --features std,micromath
echo devices libm
cargo test --no-default-features --features devices,libm
echo devices micromath
cargo test --no-default-features --features devices,micromath
echo libm micromath
cargo test --no-default-features --features libm,micromath
echo alloc devices libm
cargo test --no-default-features --features alloc,devices,libm
echo alloc devices micromath
cargo test --no-default-features --features alloc,devices,micromath
echo alloc libm micromath
cargo test --no-default-features --features alloc,libm,micromath
echo std devices libm
cargo test --no-default-features --features std,devices,libm
echo std devices micromath
cargo test --no-default-features --features std,devices,micromath
echo std libm micromath
cargo test --no-default-features --features std,libm,micromath
echo devices libm micromath
cargo test --no-default-features --features devices,libm,micromath
echo alloc devices libm micromath
cargo test --no-default-features --features alloc,devices,libm,micromath
echo std devices libm micromath
cargo test --no-default-features --features std,devices,libm,micromath
