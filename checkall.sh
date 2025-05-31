#!/bin/bash
#Generated automatically by rrtk 0.7.0-alpha.8
set -e
echo
cargo check --no-default-features
echo alloc
cargo check --no-default-features --features alloc
echo std
cargo check --no-default-features --features std
echo devices
cargo check --no-default-features --features devices
echo dim_check_debug
cargo check --no-default-features --features dim_check_debug
echo dim_check_release
cargo check --no-default-features --features dim_check_release
echo libm
cargo check --no-default-features --features libm
echo micromath
cargo check --no-default-features --features micromath
echo alloc devices
cargo check --no-default-features --features alloc,devices
echo alloc dim_check_debug
cargo check --no-default-features --features alloc,dim_check_debug
echo alloc dim_check_release
cargo check --no-default-features --features alloc,dim_check_release
echo alloc libm
cargo check --no-default-features --features alloc,libm
echo alloc micromath
cargo check --no-default-features --features alloc,micromath
echo std devices
cargo check --no-default-features --features std,devices
echo std dim_check_debug
cargo check --no-default-features --features std,dim_check_debug
echo std dim_check_release
cargo check --no-default-features --features std,dim_check_release
echo std libm
cargo check --no-default-features --features std,libm
echo std micromath
cargo check --no-default-features --features std,micromath
echo devices dim_check_debug
cargo check --no-default-features --features devices,dim_check_debug
echo devices dim_check_release
cargo check --no-default-features --features devices,dim_check_release
echo devices libm
cargo check --no-default-features --features devices,libm
echo devices micromath
cargo check --no-default-features --features devices,micromath
echo dim_check_debug libm
cargo check --no-default-features --features dim_check_debug,libm
echo dim_check_debug micromath
cargo check --no-default-features --features dim_check_debug,micromath
echo dim_check_release libm
cargo check --no-default-features --features dim_check_release,libm
echo dim_check_release micromath
cargo check --no-default-features --features dim_check_release,micromath
echo libm micromath
cargo check --no-default-features --features libm,micromath
echo alloc devices dim_check_debug
cargo check --no-default-features --features alloc,devices,dim_check_debug
echo alloc devices dim_check_release
cargo check --no-default-features --features alloc,devices,dim_check_release
echo alloc devices libm
cargo check --no-default-features --features alloc,devices,libm
echo alloc devices micromath
cargo check --no-default-features --features alloc,devices,micromath
echo alloc dim_check_debug libm
cargo check --no-default-features --features alloc,dim_check_debug,libm
echo alloc dim_check_debug micromath
cargo check --no-default-features --features alloc,dim_check_debug,micromath
echo alloc dim_check_release libm
cargo check --no-default-features --features alloc,dim_check_release,libm
echo alloc dim_check_release micromath
cargo check --no-default-features --features alloc,dim_check_release,micromath
echo alloc libm micromath
cargo check --no-default-features --features alloc,libm,micromath
echo std devices dim_check_debug
cargo check --no-default-features --features std,devices,dim_check_debug
echo std devices dim_check_release
cargo check --no-default-features --features std,devices,dim_check_release
echo std devices libm
cargo check --no-default-features --features std,devices,libm
echo std devices micromath
cargo check --no-default-features --features std,devices,micromath
echo std dim_check_debug libm
cargo check --no-default-features --features std,dim_check_debug,libm
echo std dim_check_debug micromath
cargo check --no-default-features --features std,dim_check_debug,micromath
echo std dim_check_release libm
cargo check --no-default-features --features std,dim_check_release,libm
echo std dim_check_release micromath
cargo check --no-default-features --features std,dim_check_release,micromath
echo std libm micromath
cargo check --no-default-features --features std,libm,micromath
echo devices dim_check_debug libm
cargo check --no-default-features --features devices,dim_check_debug,libm
echo devices dim_check_debug micromath
cargo check --no-default-features --features devices,dim_check_debug,micromath
echo devices dim_check_release libm
cargo check --no-default-features --features devices,dim_check_release,libm
echo devices dim_check_release micromath
cargo check --no-default-features --features devices,dim_check_release,micromath
echo devices libm micromath
cargo check --no-default-features --features devices,libm,micromath
echo dim_check_debug libm micromath
cargo check --no-default-features --features dim_check_debug,libm,micromath
echo dim_check_release libm micromath
cargo check --no-default-features --features dim_check_release,libm,micromath
echo alloc devices dim_check_debug libm
cargo check --no-default-features --features alloc,devices,dim_check_debug,libm
echo alloc devices dim_check_debug micromath
cargo check --no-default-features --features alloc,devices,dim_check_debug,micromath
echo alloc devices dim_check_release libm
cargo check --no-default-features --features alloc,devices,dim_check_release,libm
echo alloc devices dim_check_release micromath
cargo check --no-default-features --features alloc,devices,dim_check_release,micromath
echo alloc devices libm micromath
cargo check --no-default-features --features alloc,devices,libm,micromath
echo alloc dim_check_debug libm micromath
cargo check --no-default-features --features alloc,dim_check_debug,libm,micromath
echo alloc dim_check_release libm micromath
cargo check --no-default-features --features alloc,dim_check_release,libm,micromath
echo std devices dim_check_debug libm
cargo check --no-default-features --features std,devices,dim_check_debug,libm
echo std devices dim_check_debug micromath
cargo check --no-default-features --features std,devices,dim_check_debug,micromath
echo std devices dim_check_release libm
cargo check --no-default-features --features std,devices,dim_check_release,libm
echo std devices dim_check_release micromath
cargo check --no-default-features --features std,devices,dim_check_release,micromath
echo std devices libm micromath
cargo check --no-default-features --features std,devices,libm,micromath
echo std dim_check_debug libm micromath
cargo check --no-default-features --features std,dim_check_debug,libm,micromath
echo std dim_check_release libm micromath
cargo check --no-default-features --features std,dim_check_release,libm,micromath
echo devices dim_check_debug libm micromath
cargo check --no-default-features --features devices,dim_check_debug,libm,micromath
echo devices dim_check_release libm micromath
cargo check --no-default-features --features devices,dim_check_release,libm,micromath
echo alloc devices dim_check_debug libm micromath
cargo check --no-default-features --features alloc,devices,dim_check_debug,libm,micromath
echo alloc devices dim_check_release libm micromath
cargo check --no-default-features --features alloc,devices,dim_check_release,libm,micromath
echo std devices dim_check_debug libm micromath
cargo check --no-default-features --features std,devices,dim_check_debug,libm,micromath
echo std devices dim_check_release libm micromath
cargo check --no-default-features --features std,devices,dim_check_release,libm,micromath
