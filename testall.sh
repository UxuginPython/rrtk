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
echo pid
cargo test --no-default-features --features pid
echo std devices
cargo test --no-default-features --features std,devices
echo std motionprofile
cargo test --no-default-features --features std,motionprofile
echo std pid
cargo test --no-default-features --features std,pid
echo devices motionprofile
cargo test --no-default-features --features devices,motionprofile
echo devices pid
cargo test --no-default-features --features devices,pid
echo motionprofile pid
cargo test --no-default-features --features motionprofile,pid
echo std devices motionprofile
cargo test --no-default-features --features std,devices,motionprofile
echo std devices pid
cargo test --no-default-features --features std,devices,pid
echo std motionprofile pid
cargo test --no-default-features --features std,motionprofile,pid
echo devices motionprofile pid
cargo test --no-default-features --features devices,motionprofile,pid
echo std devices motionprofile pid
cargo test --no-default-features --features std,devices,motionprofile,pid
