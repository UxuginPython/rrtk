#!/bin/bash
#Generated automatically by rrtk 0.4.0-alpha.2
set -e
echo
cargo test --no-default-features
echo std
cargo test --no-default-features --features std
echo devices
cargo test --no-default-features --features devices
echo std devices
cargo test --no-default-features --features std,devices
