#!/bin/bash
#Generated automatically by rrtk 0.7.0-beta.2
set -e
echo
cargo check --no-default-features
echo alloc
cargo check --no-default-features --features alloc
echo std
cargo check --no-default-features --features std
echo devices
cargo check --no-default-features --features devices
echo libm
cargo check --no-default-features --features libm
echo micromath
cargo check --no-default-features --features micromath
echo alloc devices
cargo check --no-default-features --features alloc,devices
echo alloc libm
cargo check --no-default-features --features alloc,libm
echo alloc micromath
cargo check --no-default-features --features alloc,micromath
echo std devices
cargo check --no-default-features --features std,devices
echo std libm
cargo check --no-default-features --features std,libm
echo std micromath
cargo check --no-default-features --features std,micromath
echo devices libm
cargo check --no-default-features --features devices,libm
echo devices micromath
cargo check --no-default-features --features devices,micromath
echo libm micromath
cargo check --no-default-features --features libm,micromath
echo alloc devices libm
cargo check --no-default-features --features alloc,devices,libm
echo alloc devices micromath
cargo check --no-default-features --features alloc,devices,micromath
echo alloc libm micromath
cargo check --no-default-features --features alloc,libm,micromath
echo std devices libm
cargo check --no-default-features --features std,devices,libm
echo std devices micromath
cargo check --no-default-features --features std,devices,micromath
echo std libm micromath
cargo check --no-default-features --features std,libm,micromath
echo devices libm micromath
cargo check --no-default-features --features devices,libm,micromath
echo alloc devices libm micromath
cargo check --no-default-features --features alloc,devices,libm,micromath
echo std devices libm micromath
cargo check --no-default-features --features std,devices,libm,micromath
