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
fn transmute_unit_safe() {
    use dimensions::transmute_safe::*;
    let w: f32 = transmute_unit_safe(MillimeterPerSecond::new(39.0));
    assert_eq!(w, 39.0);
    let x: i64 = transmute_unit_safe(DimensionlessInteger(20));
    assert_eq!(x, 20);
    let y: DimensionlessInteger = transmute_unit_safe(Dimensionless::new(40i64));
    assert_eq!(y, DimensionlessInteger(40));
    let z: Dimensionless<i64> = transmute_unit_safe(DimensionlessInteger(39));
    assert_eq!(z, Dimensionless::new(39));
}
#[test]
fn transmute_memory_safe() {
    use dimensions::transmute_safe::*;
    let w: f32 = transmute_memory_safe(MillimeterPerSecond::new(39.0));
    assert_eq!(w, 39.0);
    let x: i64 = transmute_memory_safe(DimensionlessInteger(20));
    assert_eq!(x, 20);
    let y: DimensionlessInteger = transmute_memory_safe(Dimensionless::new(40i64));
    assert_eq!(y, DimensionlessInteger(40));
    let z: Dimensionless<i64> = transmute_memory_safe(DimensionlessInteger(39));
    assert_eq!(z, Dimensionless::new(39));

    let a: MillimeterPerSecond<f32> = transmute_memory_safe(20.0f32);
    assert_eq!(a, MillimeterPerSecond::new(20.0));
    let b: DimensionlessInteger = transmute_memory_safe(21i64);
    assert_eq!(b, DimensionlessInteger(21));
    let c: Time = transmute_memory_safe(3_000_000_000_i64);
    assert_eq!(c, Time::from_seconds_f32(3.0));
}
