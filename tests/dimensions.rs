// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2024-2025 UxuginPython
use rrtk::*;
#[test]
fn i64_from_time() {
    let x = Time::from_nanoseconds(5_000_000_000).as_nanoseconds();
    let y = 5_000_000_000;
    assert_eq!(x, y);
}
#[test]
fn time_add_sub() {
    let x = Time::from_nanoseconds(2_000_000_000);
    let y = Time::from_nanoseconds(3_000_000_000);
    assert_eq!(x + y, Time::from_nanoseconds(5_000_000_000));

    let mut x = Time::from_nanoseconds(2_000_000_000);
    x += Time::from_nanoseconds(3_000_000_000);
    assert_eq!(x, Time::from_nanoseconds(5_000_000_000));

    let x = Time::from_nanoseconds(3_000_000_000);
    let y = Time::from_nanoseconds(2_000_000_000);
    assert_eq!(x - y, Time::from_nanoseconds(1_000_000_000));

    let mut x = Time::from_nanoseconds(3_000_000_000);
    x -= Time::from_nanoseconds(2_000_000_000);
    assert_eq!(x, Time::from_nanoseconds(1_000_000_000));

    let x = Time::from_nanoseconds(2_000_000_000);
    let y = Quantity::new(3.0, SECOND);
    assert_eq!(x + y, Quantity::new(5.0, SECOND));

    let x = Time::from_nanoseconds(3_000_000_000);
    let y = Quantity::new(2.0, SECOND);
    assert_eq!(x - y, Quantity::new(1.0, SECOND));
}
#[test]
fn time_mul_div() {
    let x = Time::from_nanoseconds(2_000_000_000);
    let y = Time::from_nanoseconds(3_000_000_000);
    assert_eq!(x * y, Quantity::new(6.0, SECOND_SQUARED));

    let x = Time::from_nanoseconds(4_000_000_000);
    let y = Time::from_nanoseconds(2_000_000_000);
    assert_eq!(x / y, Quantity::new(2.0, DIMENSIONLESS));

    let x = Time::from_nanoseconds(2_000_000_000);
    let y = DimensionlessInteger(3);
    assert_eq!(x * y, Time::from_nanoseconds(6_000_000_000));

    let mut x = Time::from_nanoseconds(2_000_000_000);
    let y = DimensionlessInteger(3);
    x *= y;
    assert_eq!(x, Time::from_nanoseconds(6_000_000_000));

    let x = Time::from_nanoseconds(4_000_000_000);
    let y = DimensionlessInteger(2);
    assert_eq!(x / y, Time::from_nanoseconds(2_000_000_000));

    let mut x = Time::from_nanoseconds(4_000_000_000);
    let y = DimensionlessInteger(2);
    x /= y;
    assert_eq!(x, Time::from_nanoseconds(2_000_000_000));
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
fn dimensionless_integer_try_from_quantity_success() {
    let x = Quantity::new(5.0, DIMENSIONLESS);
    let x = DimensionlessInteger::try_from(x).unwrap();
    let y = DimensionlessInteger(5);
    assert_eq!(x, y);
}
#[test]
fn dimensionless_integer_add_sub() {
    let x = DimensionlessInteger(2);
    let y = DimensionlessInteger(3);
    assert_eq!(x + y, DimensionlessInteger(5));

    let mut x = DimensionlessInteger(2);
    x += DimensionlessInteger(3);
    assert_eq!(x, DimensionlessInteger(5));

    let x = DimensionlessInteger(3);
    let y = DimensionlessInteger(2);
    assert_eq!(x - y, DimensionlessInteger(1));

    let mut x = DimensionlessInteger(3);
    x -= DimensionlessInteger(2);
    assert_eq!(x, DimensionlessInteger(1));
}
#[test]
fn dimensionless_integer_mul_div() {
    let x = DimensionlessInteger(2);
    let y = Time::from_nanoseconds(3_000_000_000);
    assert_eq!(x * y, Time::from_nanoseconds(6_000_000_000));

    let x = DimensionlessInteger(4);
    let y = Time::from_nanoseconds(2_000_000_000);
    assert_eq!(x / y, Quantity::new(2.0, INVERSE_SECOND));

    let x = DimensionlessInteger(2);
    let y = DimensionlessInteger(3);
    assert_eq!(x * y, DimensionlessInteger(6));

    let mut x = DimensionlessInteger(2);
    let y = DimensionlessInteger(3);
    x *= y;
    assert_eq!(x, DimensionlessInteger(6));

    let x = DimensionlessInteger(4);
    let y = DimensionlessInteger(2);
    assert_eq!(x / y, DimensionlessInteger(2));

    let mut x = DimensionlessInteger(4);
    let y = DimensionlessInteger(2);
    x /= y;
    assert_eq!(x, DimensionlessInteger(2));
}
#[test]
fn dimensionless_integer_neg() {
    assert_eq!(-DimensionlessInteger(1), DimensionlessInteger(-1));
}
