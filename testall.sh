#!/bin/bash
#Generated automatically by rrtk 0.7.0-alpha.9
set -e
echo
cargo test --no-default-features
echo alloc
cargo test --no-default-features --features alloc
echo std
cargo test --no-default-features --features std
echo devices
cargo test --no-default-features --features devices
echo new_devices
cargo test --no-default-features --features new_devices
echo libm
cargo test --no-default-features --features libm
echo micromath
cargo test --no-default-features --features micromath
echo alloc devices
cargo test --no-default-features --features alloc,devices
echo alloc new_devices
cargo test --no-default-features --features alloc,new_devices
echo alloc libm
cargo test --no-default-features --features alloc,libm
echo alloc micromath
cargo test --no-default-features --features alloc,micromath
echo std devices
cargo test --no-default-features --features std,devices
echo std new_devices
cargo test --no-default-features --features std,new_devices
echo std libm
cargo test --no-default-features --features std,libm
echo std micromath
cargo test --no-default-features --features std,micromath
echo devices new_devices
cargo test --no-default-features --features devices,new_devices
echo devices libm
cargo test --no-default-features --features devices,libm
echo devices micromath
cargo test --no-default-features --features devices,micromath
echo new_devices libm
cargo test --no-default-features --features new_devices,libm
echo new_devices micromath
cargo test --no-default-features --features new_devices,micromath
echo libm micromath
cargo test --no-default-features --features libm,micromath
echo alloc devices new_devices
cargo test --no-default-features --features alloc,devices,new_devices
echo alloc devices libm
cargo test --no-default-features --features alloc,devices,libm
echo alloc devices micromath
cargo test --no-default-features --features alloc,devices,micromath
echo alloc new_devices libm
cargo test --no-default-features --features alloc,new_devices,libm
echo alloc new_devices micromath
cargo test --no-default-features --features alloc,new_devices,micromath
echo alloc libm micromath
cargo test --no-default-features --features alloc,libm,micromath
echo std devices new_devices
cargo test --no-default-features --features std,devices,new_devices
echo std devices libm
cargo test --no-default-features --features std,devices,libm
echo std devices micromath
cargo test --no-default-features --features std,devices,micromath
echo std new_devices libm
cargo test --no-default-features --features std,new_devices,libm
echo std new_devices micromath
cargo test --no-default-features --features std,new_devices,micromath
echo std libm micromath
cargo test --no-default-features --features std,libm,micromath
echo devices new_devices libm
cargo test --no-default-features --features devices,new_devices,libm
echo devices new_devices micromath
cargo test --no-default-features --features devices,new_devices,micromath
echo devices libm micromath
cargo test --no-default-features --features devices,libm,micromath
echo new_devices libm micromath
cargo test --no-default-features --features new_devices,libm,micromath
echo alloc devices new_devices libm
cargo test --no-default-features --features alloc,devices,new_devices,libm
echo alloc devices new_devices micromath
cargo test --no-default-features --features alloc,devices,new_devices,micromath
echo alloc devices libm micromath
cargo test --no-default-features --features alloc,devices,libm,micromath
echo alloc new_devices libm micromath
cargo test --no-default-features --features alloc,new_devices,libm,micromath
echo std devices new_devices libm
cargo test --no-default-features --features std,devices,new_devices,libm
echo std devices new_devices micromath
cargo test --no-default-features --features std,devices,new_devices,micromath
echo std devices libm micromath
cargo test --no-default-features --features std,devices,libm,micromath
echo std new_devices libm micromath
cargo test --no-default-features --features std,new_devices,libm,micromath
echo devices new_devices libm micromath
cargo test --no-default-features --features devices,new_devices,libm,micromath
echo alloc devices new_devices libm micromath
cargo test --no-default-features --features alloc,devices,new_devices,libm,micromath
echo std devices new_devices libm micromath
cargo test --no-default-features --features std,devices,new_devices,libm,micromath
