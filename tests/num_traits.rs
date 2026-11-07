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
#[test]
fn as_primitive() {
    use num_traits::AsPrimitive;
    assert_eq!(InverseSecond::new(1.0).as_(), 1.0);
    assert_eq!(Time::from_nanoseconds(1).as_(), 1);
    assert_eq!(DimensionlessInteger(2).as_(), 2);
}
#[test]
fn inv() {
    use num_traits::Inv;
    assert_eq!(
        Time::from_nanoseconds(2_000_000_000).inv(),
        InverseSecond::new(0.5)
    );
    assert!(
        DimensionlessInteger(-4)
            .inv()
            .raw_eq(&DimensionlessFraction::from_raw(1, -4))
    );
    assert!(
        DimensionlessInteger(-1)
            .inv()
            .raw_eq(&DimensionlessFraction::from_raw(1, -1))
    );
    assert_eq!(
        MillimeterPerSecondCubed::new(8.0).inv(),
        SecondCubedPerMillimeter::new(0.125)
    );
}
#[test]
#[should_panic]
fn inv_zero_fraction() {
    use num_traits::Inv;
    let _ = DimensionlessFraction::from_raw(0, 1).inv();
}
#[test]
#[should_panic]
fn inv_zero_integer() {
    use num_traits::Inv;
    let _ = DimensionlessInteger(0).inv();
}
#[test]
fn inv_zero_quantity() {
    use num_traits::Inv;
    assert_eq!(
        MillimeterPerSecondCubed::new(0.0).inv(),
        SecondCubedPerMillimeter::new(f32::INFINITY)
    );
    assert_eq!(
        MillimeterPerSecondCubed::new(-0.0).inv(),
        SecondCubedPerMillimeter::new(f32::NEG_INFINITY)
    );
}
#[test]
fn inv_zero_time() {
    use num_traits::Inv;
    assert_eq!(
        Time::from_nanoseconds(0).inv(),
        InverseSecond::new(f32::INFINITY),
    );
    assert_eq!(
        Time::from_seconds_f32(0.0).inv(),
        InverseSecond::new(f32::INFINITY),
    );
    //Unlike in Quantity, Time converts the zero to an integer first, so it becomes a "positive
    //zero" when it's converted back to a float.
    assert_eq!(
        Time::from_seconds_f32(-0.0).inv(),
        InverseSecond::new(f32::INFINITY),
    );
}
mod checked_neg {
    use super::*;
    use num_traits::CheckedNeg;
    #[test]
    fn time() {
        todo!();
    }
    #[test]
    fn dimensionless_integer() {
        todo!();
    }
    #[test]
    fn quantity() {
        todo!();
    }
    #[test]
    fn dimensionless_fraction() {
        todo!();
    }
}
mod checked_add {
    use super::*;
    use num_traits::CheckedAdd;
    #[test]
    fn time() {
        assert!(
            Time::from_nanoseconds(i64::MAX)
                .checked_add(&Time::from_nanoseconds(1))
                .is_none()
        );
        assert!(
            Time::from_nanoseconds(i64::MIN)
                .checked_add(&Time::from_nanoseconds(-1))
                .is_none()
        );
        assert_eq!(
            Time::from_nanoseconds(2000).checked_add(&Time::from_nanoseconds(-3000)),
            Some(Time::from_nanoseconds(-1000))
        );
    }
    #[test]
    fn dimensionless_integer() {
        assert!(
            DimensionlessInteger(i64::MAX)
                .checked_add(&DimensionlessInteger(1))
                .is_none()
        );
        assert!(
            DimensionlessInteger(i64::MIN)
                .checked_add(&DimensionlessInteger(-1))
                .is_none()
        );
        assert_eq!(
            DimensionlessInteger(30).checked_add(&DimensionlessInteger(-10)),
            Some(DimensionlessInteger(20))
        );
    }
    #[test]
    fn quantity() {
        assert!(
            MillimeterPerSecondCubed::new(100i8)
                .checked_add(&MillimeterPerSecondCubed::new(30i8))
                .is_none()
        );
        assert!(
            MillimeterPerSecondCubed::new(-100i8)
                .checked_add(&MillimeterPerSecondCubed::new(-30i8))
                .is_none()
        );
        assert_eq!(
            MillimeterPerSecondCubed::new(5i8).checked_add(&MillimeterPerSecondCubed::new(3i8)),
            Some(MillimeterPerSecondCubed::new(8i8))
        );
    }
}
mod checked_sub {
    use super::*;
    use num_traits::CheckedSub;
    #[test]
    fn time() {
        assert!(
            Time::from_nanoseconds(i64::MAX)
                .checked_sub(&Time::from_nanoseconds(-1))
                .is_none()
        );
        assert!(
            Time::from_nanoseconds(i64::MIN)
                .checked_sub(&Time::from_nanoseconds(1))
                .is_none()
        );
        assert_eq!(
            Time::from_nanoseconds(2000).checked_sub(&Time::from_nanoseconds(3000)),
            Some(Time::from_nanoseconds(-1000))
        );
    }
    #[test]
    fn dimensionless_integer() {
        assert!(
            DimensionlessInteger(i64::MAX)
                .checked_sub(&DimensionlessInteger(-1))
                .is_none()
        );
        assert!(
            DimensionlessInteger(i64::MIN)
                .checked_sub(&DimensionlessInteger(1))
                .is_none()
        );
        assert_eq!(
            DimensionlessInteger(30).checked_sub(&DimensionlessInteger(10)),
            Some(DimensionlessInteger(20))
        );
    }
    #[test]
    fn quantity() {
        assert!(
            MillimeterPerSecondCubed::new(100i8)
                .checked_sub(&MillimeterPerSecondCubed::new(-30i8))
                .is_none()
        );
        assert!(
            MillimeterPerSecondCubed::new(-100i8)
                .checked_sub(&MillimeterPerSecondCubed::new(30i8))
                .is_none()
        );
        assert_eq!(
            MillimeterPerSecondCubed::new(5i8).checked_sub(&MillimeterPerSecondCubed::new(-3i8)),
            Some(MillimeterPerSecondCubed::new(8i8))
        );
    }
}
mod checked_mul {
    use super::*;
    use num_traits::CheckedMul;
    #[test]
    fn dimensionless_integer() {
        todo!();
    }
    #[test]
    fn quantity_dimensionless() {
        todo!();
    }
}
//Although there's only one test in here, we keep the module structure for consistency.
mod checked_div {
    use super::*;
    use num_traits::CheckedDiv;
    #[test]
    fn quantity_dimensionless() {
        todo!();
    }
}
