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
fn construct_dimensionless_fraction() {
    let _ = DimensionlessFraction::new(Time::from_nanoseconds(2), Time::from_nanoseconds(3));
    let _ = DimensionlessFraction::new(DimensionlessInteger(2), DimensionlessInteger(3));
    let _ = DimensionlessFraction::new(
        MillimeterPerSecond::new(2_i64),
        MillimeterPerSecond::new(3_i64),
    );
    let _ = DimensionlessFraction::new(DimensionlessInteger(2), Dimensionless::new(3_i64));
    let _ = DimensionlessFraction::new(Dimensionless::new(2_i64), DimensionlessInteger(3));
}
#[test]
fn as_dimensionless_integer() {
    assert_eq!(
        Dimensionless::new(2_i64).as_dimensionless_integer(),
        DimensionlessInteger(2)
    );
}
#[test]
fn as_quantity() {
    assert_eq!(
        DimensionlessInteger(-91).as_quantity(),
        Dimensionless::new(-91_i64),
    )
}
#[test]
fn as_i64() {
    use dimensions::layout_compatibility::as_i64;
    assert_eq!(as_i64(Time::from_nanoseconds(4_000_000_000)), 4_000_000_000);
    assert_eq!(as_i64(DimensionlessInteger(20)), 20);
    assert_eq!(as_i64(InverseSecondSquared::new(-523_i64)), -523);
}
#[test]
fn safe_transmute() {
    use dimensions::layout_compatibility::safe_transmute;
    assert_eq!(
        safe_transmute::<_, DimensionlessInteger>(Dimensionless::new(-60_i64)),
        DimensionlessInteger(-60)
    );
    assert_eq!(
        safe_transmute::<_, Dimensionless<i64>>(DimensionlessInteger(40)),
        Dimensionless::new(40)
    );
}
