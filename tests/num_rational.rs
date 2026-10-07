// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2024-2026 UxuginPython
#![cfg(feature = "num-rational")]
use num_rational::Rational64;
use rrtk::dimensions::*;
#[test]
fn dimensionless_integer_to_ratio() {
    let start = DimensionlessInteger(5);
    let test = Rational64::from(start);
    assert_eq!(test.into_raw(), (5, 1));
}
#[test]
fn dimensionless_fraction_to_ratio() {
    let start = DimensionlessFraction::from_raw(4, 6);
    let test = Rational64::from(start);
    assert_eq!(test.into_raw(), (4, 6));
}
#[test]
fn ratio_to_dimensionless_fraction() {
    let start = Rational64::new_raw(15, -10);
    let test = DimensionlessFraction::try_from(start).expect("the denominator is not zero");
    assert!(test.raw_eq(&DimensionlessFraction::from_raw(15, -10)));
}
#[test]
fn ratio_to_dimensionless_fraction_zero_denominator() {
    let start = Rational64::new_raw(11, 0);
    let test = DimensionlessFraction::try_from(start);
    assert_eq!(test, Err(rrtk::error::ZeroDivision));
}
