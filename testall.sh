#!/bin/bash
#Generated automatically by rrtk 0.3.0-alpha.3
set -e
echo 
cargo test --no-default-features 
echo std
cargo test --no-default-features --features std
echo devices
cargo test --no-default-features --features devices
echo motionprofile
cargo test --no-default-features --features motionprofile
echo std devices
cargo test --no-default-features --features std,devices
echo std motionprofile
cargo test --no-default-features --features std,motionprofile
echo devices motionprofile
cargo test --no-default-features --features devices,motionprofile
echo std devices motionprofile
cargo test --no-default-features --features std,devices,motionprofile
