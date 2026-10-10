#!/bin/bash
#Generated automatically for rrtk 0.7.2-alpha.1
set -e
echo
cargo miri test --no-default-features
echo alloc
cargo miri test --no-default-features --features alloc
echo std
cargo miri test --no-default-features --features std
echo devices
cargo miri test --no-default-features --features devices
echo libm
cargo miri test --no-default-features --features libm
echo micromath
cargo miri test --no-default-features --features micromath
echo num-rational
cargo miri test --no-default-features --features num-rational
echo num-traits
cargo miri test --no-default-features --features num-traits
echo alloc devices
cargo miri test --no-default-features --features alloc,devices
echo alloc libm
cargo miri test --no-default-features --features alloc,libm
echo alloc micromath
cargo miri test --no-default-features --features alloc,micromath
echo alloc num-rational
cargo miri test --no-default-features --features alloc,num-rational
echo alloc num-traits
cargo miri test --no-default-features --features alloc,num-traits
echo std devices
cargo miri test --no-default-features --features std,devices
echo std libm
cargo miri test --no-default-features --features std,libm
echo std micromath
cargo miri test --no-default-features --features std,micromath
echo std num-rational
cargo miri test --no-default-features --features std,num-rational
echo std num-traits
cargo miri test --no-default-features --features std,num-traits
echo devices libm
cargo miri test --no-default-features --features devices,libm
echo devices micromath
cargo miri test --no-default-features --features devices,micromath
echo devices num-rational
cargo miri test --no-default-features --features devices,num-rational
echo devices num-traits
cargo miri test --no-default-features --features devices,num-traits
echo libm micromath
cargo miri test --no-default-features --features libm,micromath
echo libm num-rational
cargo miri test --no-default-features --features libm,num-rational
echo libm num-traits
cargo miri test --no-default-features --features libm,num-traits
echo micromath num-rational
cargo miri test --no-default-features --features micromath,num-rational
echo micromath num-traits
cargo miri test --no-default-features --features micromath,num-traits
echo num-rational num-traits
cargo miri test --no-default-features --features num-rational,num-traits
echo alloc devices libm
cargo miri test --no-default-features --features alloc,devices,libm
echo alloc devices micromath
cargo miri test --no-default-features --features alloc,devices,micromath
echo alloc devices num-rational
cargo miri test --no-default-features --features alloc,devices,num-rational
echo alloc devices num-traits
cargo miri test --no-default-features --features alloc,devices,num-traits
echo alloc libm micromath
cargo miri test --no-default-features --features alloc,libm,micromath
echo alloc libm num-rational
cargo miri test --no-default-features --features alloc,libm,num-rational
echo alloc libm num-traits
cargo miri test --no-default-features --features alloc,libm,num-traits
echo alloc micromath num-rational
cargo miri test --no-default-features --features alloc,micromath,num-rational
echo alloc micromath num-traits
cargo miri test --no-default-features --features alloc,micromath,num-traits
echo alloc num-rational num-traits
cargo miri test --no-default-features --features alloc,num-rational,num-traits
echo std devices libm
cargo miri test --no-default-features --features std,devices,libm
echo std devices micromath
cargo miri test --no-default-features --features std,devices,micromath
echo std devices num-rational
cargo miri test --no-default-features --features std,devices,num-rational
echo std devices num-traits
cargo miri test --no-default-features --features std,devices,num-traits
echo std libm micromath
cargo miri test --no-default-features --features std,libm,micromath
echo std libm num-rational
cargo miri test --no-default-features --features std,libm,num-rational
echo std libm num-traits
cargo miri test --no-default-features --features std,libm,num-traits
echo std micromath num-rational
cargo miri test --no-default-features --features std,micromath,num-rational
echo std micromath num-traits
cargo miri test --no-default-features --features std,micromath,num-traits
echo std num-rational num-traits
cargo miri test --no-default-features --features std,num-rational,num-traits
echo devices libm micromath
cargo miri test --no-default-features --features devices,libm,micromath
echo devices libm num-rational
cargo miri test --no-default-features --features devices,libm,num-rational
echo devices libm num-traits
cargo miri test --no-default-features --features devices,libm,num-traits
echo devices micromath num-rational
cargo miri test --no-default-features --features devices,micromath,num-rational
echo devices micromath num-traits
cargo miri test --no-default-features --features devices,micromath,num-traits
echo devices num-rational num-traits
cargo miri test --no-default-features --features devices,num-rational,num-traits
echo libm micromath num-rational
cargo miri test --no-default-features --features libm,micromath,num-rational
echo libm micromath num-traits
cargo miri test --no-default-features --features libm,micromath,num-traits
echo libm num-rational num-traits
cargo miri test --no-default-features --features libm,num-rational,num-traits
echo micromath num-rational num-traits
cargo miri test --no-default-features --features micromath,num-rational,num-traits
echo alloc devices libm micromath
cargo miri test --no-default-features --features alloc,devices,libm,micromath
echo alloc devices libm num-rational
cargo miri test --no-default-features --features alloc,devices,libm,num-rational
echo alloc devices libm num-traits
cargo miri test --no-default-features --features alloc,devices,libm,num-traits
echo alloc devices micromath num-rational
cargo miri test --no-default-features --features alloc,devices,micromath,num-rational
echo alloc devices micromath num-traits
cargo miri test --no-default-features --features alloc,devices,micromath,num-traits
echo alloc devices num-rational num-traits
cargo miri test --no-default-features --features alloc,devices,num-rational,num-traits
echo alloc libm micromath num-rational
cargo miri test --no-default-features --features alloc,libm,micromath,num-rational
echo alloc libm micromath num-traits
cargo miri test --no-default-features --features alloc,libm,micromath,num-traits
echo alloc libm num-rational num-traits
cargo miri test --no-default-features --features alloc,libm,num-rational,num-traits
echo alloc micromath num-rational num-traits
cargo miri test --no-default-features --features alloc,micromath,num-rational,num-traits
echo std devices libm micromath
cargo miri test --no-default-features --features std,devices,libm,micromath
echo std devices libm num-rational
cargo miri test --no-default-features --features std,devices,libm,num-rational
echo std devices libm num-traits
cargo miri test --no-default-features --features std,devices,libm,num-traits
echo std devices micromath num-rational
cargo miri test --no-default-features --features std,devices,micromath,num-rational
echo std devices micromath num-traits
cargo miri test --no-default-features --features std,devices,micromath,num-traits
echo std devices num-rational num-traits
cargo miri test --no-default-features --features std,devices,num-rational,num-traits
echo std libm micromath num-rational
cargo miri test --no-default-features --features std,libm,micromath,num-rational
echo std libm micromath num-traits
cargo miri test --no-default-features --features std,libm,micromath,num-traits
echo std libm num-rational num-traits
cargo miri test --no-default-features --features std,libm,num-rational,num-traits
echo std micromath num-rational num-traits
cargo miri test --no-default-features --features std,micromath,num-rational,num-traits
echo devices libm micromath num-rational
cargo miri test --no-default-features --features devices,libm,micromath,num-rational
echo devices libm micromath num-traits
cargo miri test --no-default-features --features devices,libm,micromath,num-traits
echo devices libm num-rational num-traits
cargo miri test --no-default-features --features devices,libm,num-rational,num-traits
echo devices micromath num-rational num-traits
cargo miri test --no-default-features --features devices,micromath,num-rational,num-traits
echo libm micromath num-rational num-traits
cargo miri test --no-default-features --features libm,micromath,num-rational,num-traits
echo alloc devices libm micromath num-rational
cargo miri test --no-default-features --features alloc,devices,libm,micromath,num-rational
echo alloc devices libm micromath num-traits
cargo miri test --no-default-features --features alloc,devices,libm,micromath,num-traits
echo alloc devices libm num-rational num-traits
cargo miri test --no-default-features --features alloc,devices,libm,num-rational,num-traits
echo alloc devices micromath num-rational num-traits
cargo miri test --no-default-features --features alloc,devices,micromath,num-rational,num-traits
echo alloc libm micromath num-rational num-traits
cargo miri test --no-default-features --features alloc,libm,micromath,num-rational,num-traits
echo std devices libm micromath num-rational
cargo miri test --no-default-features --features std,devices,libm,micromath,num-rational
echo std devices libm micromath num-traits
cargo miri test --no-default-features --features std,devices,libm,micromath,num-traits
echo std devices libm num-rational num-traits
cargo miri test --no-default-features --features std,devices,libm,num-rational,num-traits
echo std devices micromath num-rational num-traits
cargo miri test --no-default-features --features std,devices,micromath,num-rational,num-traits
echo std libm micromath num-rational num-traits
cargo miri test --no-default-features --features std,libm,micromath,num-rational,num-traits
echo devices libm micromath num-rational num-traits
cargo miri test --no-default-features --features devices,libm,micromath,num-rational,num-traits
echo alloc devices libm micromath num-rational num-traits
cargo miri test --no-default-features --features alloc,devices,libm,micromath,num-rational,num-traits
echo std devices libm micromath num-rational num-traits
cargo miri test --no-default-features --features std,devices,libm,micromath,num-rational,num-traits
