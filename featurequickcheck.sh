#!/bin/bash
set -e
echo no default features
cargo check --no-default-features
echo default features
cargo check
echo default + devices
cargo check --features devices
echo all features
cargo check --all-features
