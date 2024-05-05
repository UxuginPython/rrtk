#!/bin/bash
#Generated automatically by rrtk 0.3.0-alpha.4
set -e
echo 
cargo test --no-default-features 
echo std
cargo test --no-default-features --features std
echo motionprofile
cargo test --no-default-features --features motionprofile
echo std motionprofile
cargo test --no-default-features --features std,motionprofile
