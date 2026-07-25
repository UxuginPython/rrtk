// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2024-2026 UxuginPython
use rrtk::*;
#[test]
fn i64_from_time() {
    let x = Time::from_nanoseconds(5_000_000_000).as_nanoseconds();
    let y = 5_000_000_000;
    assert_eq!(x, y);
}
#[test]
fn time_neg() {
    assert_eq!(
        -Time::from_nanoseconds(1_000_000_000),
        Time::from_nanoseconds(-1_000_000_000)
    );
}
#[test]
fn dimensionless_integer_new() {
    let x = DimensionlessInteger::new(5);
    let y = DimensionlessInteger(5);
    assert_eq!(x, y);
}
#[test]
fn dimensionless_integer_from_i64() {
    let x = DimensionlessInteger::from(5);
    let y = DimensionlessInteger(5);
    assert_eq!(x, y);
}
#[test]
fn i64_from_dimensionless_integer() {
    let x = i64::from(DimensionlessInteger(5));
    let y = 5;
    assert_eq!(x, y);
}
#[test]
fn dimensionless_integer_neg() {
    assert_eq!(-DimensionlessInteger(1), DimensionlessInteger(-1));
}
