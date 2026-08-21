#!/bin/bash
# SPDX-License-Identifier: BSD-3-Clause
# Copyright 2024-2026 UxuginPython
set -e
echo no default features
cargo check --no-default-features
echo default features
cargo check
echo default + devices
cargo check --features devices
echo all features
cargo check --all-features
