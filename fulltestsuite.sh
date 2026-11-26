#!/bin/bash
# SPDX-License-Identifier: BSD-3-Clause
# Copyright 2024-2026 UxuginPython
set -e
python3 generatetestall.py
rustup update
rustup default 1.95
./testall.sh
rustup default nightly
./testallmiri.sh
