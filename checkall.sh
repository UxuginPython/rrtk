#!/bin/bash
#Generated automatically for rrtk 0.7.2-alpha.0
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
echo num-rational
cargo check --no-default-features --features num-rational
echo alloc devices
cargo check --no-default-features --features alloc,devices
echo alloc libm
cargo check --no-default-features --features alloc,libm
echo alloc micromath
cargo check --no-default-features --features alloc,micromath
echo alloc num-rational
cargo check --no-default-features --features alloc,num-rational
echo std devices
cargo check --no-default-features --features std,devices
echo std libm
cargo check --no-default-features --features std,libm
echo std micromath
cargo check --no-default-features --features std,micromath
echo std num-rational
cargo check --no-default-features --features std,num-rational
echo devices libm
cargo check --no-default-features --features devices,libm
echo devices micromath
cargo check --no-default-features --features devices,micromath
echo devices num-rational
cargo check --no-default-features --features devices,num-rational
echo libm micromath
cargo check --no-default-features --features libm,micromath
echo libm num-rational
cargo check --no-default-features --features libm,num-rational
echo micromath num-rational
cargo check --no-default-features --features micromath,num-rational
echo alloc devices libm
cargo check --no-default-features --features alloc,devices,libm
echo alloc devices micromath
cargo check --no-default-features --features alloc,devices,micromath
echo alloc devices num-rational
cargo check --no-default-features --features alloc,devices,num-rational
echo alloc libm micromath
cargo check --no-default-features --features alloc,libm,micromath
echo alloc libm num-rational
cargo check --no-default-features --features alloc,libm,num-rational
echo alloc micromath num-rational
cargo check --no-default-features --features alloc,micromath,num-rational
echo std devices libm
cargo check --no-default-features --features std,devices,libm
echo std devices micromath
cargo check --no-default-features --features std,devices,micromath
echo std devices num-rational
cargo check --no-default-features --features std,devices,num-rational
echo std libm micromath
cargo check --no-default-features --features std,libm,micromath
echo std libm num-rational
cargo check --no-default-features --features std,libm,num-rational
echo std micromath num-rational
cargo check --no-default-features --features std,micromath,num-rational
echo devices libm micromath
cargo check --no-default-features --features devices,libm,micromath
echo devices libm num-rational
cargo check --no-default-features --features devices,libm,num-rational
echo devices micromath num-rational
cargo check --no-default-features --features devices,micromath,num-rational
echo libm micromath num-rational
cargo check --no-default-features --features libm,micromath,num-rational
echo alloc devices libm micromath
cargo check --no-default-features --features alloc,devices,libm,micromath
echo alloc devices libm num-rational
cargo check --no-default-features --features alloc,devices,libm,num-rational
echo alloc devices micromath num-rational
cargo check --no-default-features --features alloc,devices,micromath,num-rational
echo alloc libm micromath num-rational
cargo check --no-default-features --features alloc,libm,micromath,num-rational
echo std devices libm micromath
cargo check --no-default-features --features std,devices,libm,micromath
echo std devices libm num-rational
cargo check --no-default-features --features std,devices,libm,num-rational
echo std devices micromath num-rational
cargo check --no-default-features --features std,devices,micromath,num-rational
echo std libm micromath num-rational
cargo check --no-default-features --features std,libm,micromath,num-rational
echo devices libm micromath num-rational
cargo check --no-default-features --features devices,libm,micromath,num-rational
echo alloc devices libm micromath num-rational
cargo check --no-default-features --features alloc,devices,libm,micromath,num-rational
echo std devices libm micromath num-rational
cargo check --no-default-features --features std,devices,libm,micromath,num-rational
