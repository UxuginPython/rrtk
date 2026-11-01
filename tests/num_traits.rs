// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2024-2026 UxuginPython
#![cfg(feature = "num-traits")]
use rrtk::dimensions::*;
#[test]
fn zero_zero() {
    use num_traits::Zero;
    assert_eq!(<Time as Zero>::zero(), Time::from_nanoseconds(0));
    assert_eq!(
        <DimensionlessInteger as Zero>::zero(),
        DimensionlessInteger(0)
    );
    assert!(<DimensionlessFraction as Zero>::zero().raw_eq(&DimensionlessFraction::from_raw(0, 1)));
    assert_eq!(
        <MillimeterPerSecondCubed<f32> as Zero>::zero(),
        MillimeterPerSecondCubed::new(0.0)
    );
}
#[test]
fn zero_is_zero() {
    use num_traits::Zero;
    assert!(Time::from_nanoseconds(0).is_zero());
    assert!(!Time::from_nanoseconds(1).is_zero());
    assert!(DimensionlessInteger(0).is_zero());
    assert!(!DimensionlessInteger(1).is_zero());
    assert!(!DimensionlessInteger(2).is_zero());
    assert!(DimensionlessFraction::from_raw(0, 1).is_zero());
    assert!(DimensionlessFraction::from_raw(0, 3).is_zero());
    assert!(!DimensionlessFraction::from_raw(5, 1).is_zero());
    assert!(MillimeterPerSecondCubed::<i8>::new(0).is_zero());
    assert!(!MillimeterPerSecondCubed::<i8>::new(8).is_zero());
    assert!(!MillimeterPerSecondCubed::<i8>::new(-13).is_zero());
}
#[test]
fn one() {
    use num_traits::One;
    assert_eq!(
        <DimensionlessInteger as One>::one(),
        DimensionlessInteger(1)
    );
    assert!(<DimensionlessFraction as One>::one().raw_eq(&DimensionlessFraction::from_raw(1, 1)));
    assert_eq!(<Dimensionless<f32> as One>::one(), Dimensionless::new(1.0));
}
#[test]
fn const_zero() {
    use num_traits::ConstZero;
    assert_eq!(<Time as ConstZero>::ZERO, Time::from_nanoseconds(0));
    assert_eq!(
        <DimensionlessInteger as ConstZero>::ZERO,
        DimensionlessInteger(0)
    );
    assert!(
        <DimensionlessFraction as ConstZero>::ZERO.raw_eq(&DimensionlessFraction::from_raw(0, 1))
    );
    assert_eq!(
        <MillimeterPerSecondCubed<f32> as ConstZero>::ZERO,
        MillimeterPerSecondCubed::new(0.0)
    );
}
#[test]
fn const_one() {
    use num_traits::ConstOne;
    assert_eq!(
        <DimensionlessInteger as ConstOne>::ONE,
        DimensionlessInteger(1)
    );
    assert!(
        <DimensionlessFraction as ConstOne>::ONE.raw_eq(&DimensionlessFraction::from_raw(1, 1))
    );
    assert_eq!(
        <Dimensionless<f32> as ConstOne>::ONE,
        Dimensionless::new(1.0)
    );
}
#[test]
fn bounded() {
    use num_traits::bounds::Bounded;
    assert_eq!(Time::min_value(), Time::from_nanoseconds(i64::MIN));
    assert_eq!(Time::max_value(), Time::from_nanoseconds(i64::MAX));
    assert_eq!(
        DimensionlessInteger::min_value(),
        DimensionlessInteger(i64::MIN)
    );
    assert_eq!(
        DimensionlessInteger::max_value(),
        DimensionlessInteger(i64::MAX)
    );
    assert!(
        DimensionlessFraction::min_value().raw_eq(&DimensionlessFraction::from_raw(i64::MIN, 1))
    );
    assert!(
        DimensionlessFraction::max_value().raw_eq(&DimensionlessFraction::from_raw(i64::MIN, -1))
    );
}
#[test]
fn upper_lower_bounded() {
    use num_traits::bounds::{LowerBounded, UpperBounded};
    assert_eq!(Millimeter::<i8>::min_value(), Millimeter::new(i8::MIN));
    assert_eq!(Millimeter::<i8>::max_value(), Millimeter::new(i8::MAX));
}
