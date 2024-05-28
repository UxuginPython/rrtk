#!/bin/bash
#Generated automatically by rrtk 0.3.0-alpha.5
set -e
echo
cargo test --no-default-features
echo std
cargo test --no-default-features --features std
