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
#[test]
fn dimensionless_fraction_comparisons_more_signs() {
    let a = dimensionless_fraction!(3, 2);
    let b = dimensionless_fraction!(1, 2);
    assert!(a > b);
    let c = dimensionless_fraction!(-3, -2);
    let d = dimensionless_fraction!(-1, -2);
    assert_eq!(a, c);
    assert_eq!(b, d);
    assert!(c > d);
    assert!(a > d);
    assert!(c > b);
}
#[test]
fn even_more_signs() {
    let fracs = [
        dimensionless_fraction!(-3, 2),
        dimensionless_fraction!(3, -2),
        dimensionless_fraction!(-1, 2),
        dimensionless_fraction!(1, -2),
        dimensionless_fraction!(0, 2),
        dimensionless_fraction!(0, -2),
        dimensionless_fraction!(1, 2),
        dimensionless_fraction!(-1, -2),
        dimensionless_fraction!(3, 2),
        dimensionless_fraction!(-3, -2),
    ];
    for a in fracs {
        for b in fracs {
            assert_eq!(a.cmp(&b), a.as_f32().partial_cmp(&b.as_f32()).unwrap());
        }
    }
}
