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
fn time_try_from_quantity_success() {
    let x = Quantity::new(5.0, SECOND);
    let x = Time::try_from(x).unwrap();
    let y = Time::from_nanoseconds(5_000_000_000);
    assert_eq!(x, y);
}
#[test]
#[cfg(any(
    feature = "dim_check_release",
    all(debug_assertions, feature = "dim_check_debug")
))]
fn time_try_from_quantity_failure() {
    let x = Quantity::new(5.0, MILLIMETER);
    let x = Time::try_from(x);
    assert!(x.is_err());
}
#[test]
fn quantity_from_time() {
    let x = Quantity::from(Time::from_nanoseconds(5_000_000_000));
    let y = Quantity::new(5.0, SECOND);
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

    let x = Time::from_nanoseconds(2_000_000_000);
    let y = Quantity::new(3.0, MILLIMETER_PER_SECOND_SQUARED);
    assert_eq!(x * y, Quantity::new(6.0, MILLIMETER_PER_SECOND));

    let x = Time::from_nanoseconds(4_000_000_000);
    let y = Quantity::new(2.0, MILLIMETER_PER_SECOND_SQUARED);
    assert_eq!(x / y, Quantity::new(2.0, SECOND_CUBED_PER_MILLIMETER));
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
#[cfg(any(
    feature = "dim_check_release",
    all(debug_assertions, feature = "dim_check_debug")
))]
fn dimensionless_integer_try_from_quantity_failure() {
    let x = Quantity::new(5.0, MILLIMETER);
    let x = DimensionlessInteger::try_from(x);
    assert!(x.is_err());
}
#[test]
fn quantity_from_dimensionless_integer() {
    let x = Quantity::from(DimensionlessInteger(5));
    let y = Quantity::new(5.0, DIMENSIONLESS);
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

    let x = DimensionlessInteger(2);
    let y = Quantity::dimensionless(3.0);
    assert_eq!(x + y, Quantity::dimensionless(5.0));

    let x = DimensionlessInteger(3);
    let y = Quantity::dimensionless(2.0);
    assert_eq!(x - y, Quantity::dimensionless(1.0));
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

    let x = DimensionlessInteger(2);
    let y = Quantity::new(3.0, MILLIMETER_PER_SECOND_SQUARED);
    assert_eq!(x * y, Quantity::new(6.0, MILLIMETER_PER_SECOND_SQUARED));

    let x = DimensionlessInteger(4);
    let y = Quantity::new(2.0, MILLIMETER_PER_SECOND_SQUARED);
    assert_eq!(x / y, Quantity::new(2.0, SECOND_SQUARED_PER_MILLIMETER));
}
#[test]
fn dimensionless_integer_neg() {
    assert_eq!(-DimensionlessInteger(1), DimensionlessInteger(-1));
}
#[test]
#[allow(unused)]
fn unit_new() {
    //There's really not a lot we can test here since it's the only constructor for the type.
    let x: Unit = Unit::new(2, 3);
}
#[test]
#[cfg(any(
    feature = "dim_check_release",
    all(debug_assertions, feature = "dim_check_debug")
))]
fn unit_const_eq() {
    assert!(MILLIMETER.const_eq(&MILLIMETER));
    assert!(!MILLIMETER.const_eq(&MILLIMETER_PER_SECOND));
}
#[test]
#[cfg(any(
    feature = "dim_check_release",
    all(debug_assertions, feature = "dim_check_debug")
))]
fn unit_const_assert_eq_success() {
    MILLIMETER.const_assert_eq(&MILLIMETER);
}
#[test]
#[should_panic]
#[cfg(any(
    feature = "dim_check_release",
    all(debug_assertions, feature = "dim_check_debug")
))]
fn unit_const_assert_eq_failure() {
    MILLIMETER.const_assert_eq(&SECOND);
}
#[test]
#[cfg(not(any(
    feature = "dim_check_release",
    all(debug_assertions, feature = "dim_check_debug")
)))]
fn unit_eq_assume_true_no_dim_check() {
    assert!(MILLIMETER.eq_assume_true(&MILLIMETER));
    assert!(MILLIMETER.eq_assume_true(&SECOND));
}
#[test]
#[cfg(any(
    feature = "dim_check_release",
    all(debug_assertions, feature = "dim_check_debug")
))]
fn unit_eq_assume_true_dim_check() {
    assert!(MILLIMETER.eq_assume_true(&MILLIMETER));
    assert!(!MILLIMETER.eq_assume_true(&SECOND));
}
#[test]
#[cfg(not(any(
    feature = "dim_check_release",
    all(debug_assertions, feature = "dim_check_debug")
)))]
fn unit_eq_assume_false_no_dim_check() {
    assert!(!MILLIMETER.eq_assume_false(&MILLIMETER));
    assert!(!MILLIMETER.eq_assume_false(&SECOND));
}
#[test]
#[cfg(any(
    feature = "dim_check_release",
    all(debug_assertions, feature = "dim_check_debug")
))]
fn unit_eq_assume_false_dim_check() {
    assert!(MILLIMETER.eq_assume_false(&MILLIMETER));
    assert!(!MILLIMETER.eq_assume_false(&SECOND));
}
#[test]
#[cfg(any(
    feature = "dim_check_release",
    all(debug_assertions, feature = "dim_check_debug")
))]
fn unit_assert_eq_assume_ok_dim_check_success() {
    MILLIMETER.assert_eq_assume_ok(&MILLIMETER);
}
#[test]
#[should_panic]
#[cfg(any(
    feature = "dim_check_release",
    all(debug_assertions, feature = "dim_check_debug")
))]
fn unit_assert_eq_assume_ok_dim_check_failure() {
    MILLIMETER.assert_eq_assume_ok(&SECOND);
}
#[test]
#[cfg(not(any(
    feature = "dim_check_release",
    all(debug_assertions, feature = "dim_check_debug")
)))]
fn unit_assert_eq_assume_ok_no_dim_check() {
    MILLIMETER.assert_eq_assume_ok(&MILLIMETER);
}
#[test]
#[cfg(any(
    feature = "dim_check_release",
    all(debug_assertions, feature = "dim_check_debug")
))]
fn unit_assert_eq_assume_not_ok_dim_check_success() {
    MILLIMETER.assert_eq_assume_not_ok(&MILLIMETER);
}
#[test]
#[should_panic]
#[cfg(any(
    feature = "dim_check_release",
    all(debug_assertions, feature = "dim_check_debug")
))]
fn unit_assert_eq_assume_not_ok_dim_check_failure() {
    MILLIMETER.assert_eq_assume_not_ok(&SECOND);
}
#[test]
#[should_panic]
#[cfg(not(any(
    feature = "dim_check_release",
    all(debug_assertions, feature = "dim_check_debug")
)))]
fn unit_assert_eq_assume_not_ok_no_dim_check() {
    MILLIMETER.assert_eq_assume_not_ok(&SECOND);
}
#[test]
#[cfg(any(
    feature = "dim_check_release",
    all(debug_assertions, feature = "dim_check_debug")
))]
fn unit_from_position_derivative() {
    assert_eq!(Unit::from(PositionDerivative::Position), MILLIMETER);
    assert_eq!(
        Unit::from(PositionDerivative::Velocity),
        MILLIMETER_PER_SECOND
    );
    assert_eq!(
        Unit::from(PositionDerivative::Acceleration),
        MILLIMETER_PER_SECOND_SQUARED
    );
}
#[test]
#[cfg(any(
    feature = "dim_check_release",
    all(debug_assertions, feature = "dim_check_debug")
))]
fn unit_try_from_motion_profile_piece() {
    assert_eq!(
        Unit::try_from(MotionProfilePiece::BeforeStart),
        Err(error::CannotConvert)
    );
    assert_eq!(
        Unit::try_from(MotionProfilePiece::InitialAcceleration),
        Ok(MILLIMETER_PER_SECOND_SQUARED)
    );
    assert_eq!(
        Unit::try_from(MotionProfilePiece::ConstantVelocity),
        Ok(MILLIMETER_PER_SECOND)
    );
    assert_eq!(
        Unit::try_from(MotionProfilePiece::EndAcceleration),
        Ok(MILLIMETER_PER_SECOND_SQUARED)
    );
    assert_eq!(
        Unit::try_from(MotionProfilePiece::Complete),
        Err(error::CannotConvert)
    );
}
#[test]
fn unit_add_sub_success() {
    let x = MILLIMETER_PER_SECOND_SQUARED;
    let y = MILLIMETER_PER_SECOND_SQUARED;
    let z = x + y;
    z.assert_eq_assume_ok(&MILLIMETER_PER_SECOND_SQUARED);

    let mut x = MILLIMETER_PER_SECOND_SQUARED;
    let y = MILLIMETER_PER_SECOND_SQUARED;
    x += y;
    x.assert_eq_assume_ok(&MILLIMETER_PER_SECOND_SQUARED);

    let x = MILLIMETER_PER_SECOND_SQUARED;
    let y = MILLIMETER_PER_SECOND_SQUARED;
    let z = x - y;
    z.assert_eq_assume_ok(&MILLIMETER_PER_SECOND_SQUARED);

    let mut x = MILLIMETER_PER_SECOND_SQUARED;
    let y = MILLIMETER_PER_SECOND_SQUARED;
    x -= y;
    x.assert_eq_assume_ok(&MILLIMETER_PER_SECOND_SQUARED);
}
#[test]
#[should_panic]
#[cfg(any(
    feature = "dim_check_release",
    all(debug_assertions, feature = "dim_check_debug")
))]
fn unit_add_failure() {
    let x = MILLIMETER_PER_SECOND;
    let y = MILLIMETER_PER_SECOND_SQUARED;
    let _ = x + y;
}
#[test]
#[should_panic]
#[cfg(any(
    feature = "dim_check_release",
    all(debug_assertions, feature = "dim_check_debug")
))]
fn unit_add_assign_failure() {
    let mut x = MILLIMETER_PER_SECOND;
    let y = MILLIMETER_PER_SECOND_SQUARED;
    x += y;
}
#[test]
#[should_panic]
#[cfg(any(
    feature = "dim_check_release",
    all(debug_assertions, feature = "dim_check_debug")
))]
fn unit_sub_failure() {
    let x = MILLIMETER_PER_SECOND;
    let y = MILLIMETER_PER_SECOND_SQUARED;
    let _ = x - y;
}
#[test]
#[should_panic]
#[cfg(any(
    feature = "dim_check_release",
    all(debug_assertions, feature = "dim_check_debug")
))]
fn unit_sub_assign_failure() {
    let mut x = MILLIMETER_PER_SECOND;
    let y = MILLIMETER_PER_SECOND_SQUARED;
    x -= y;
}
#[test]
fn unit_mul_div() {
    let x = MILLIMETER_PER_SECOND_SQUARED;
    let y = SECOND;
    let z = x * y;
    z.assert_eq_assume_ok(&MILLIMETER_PER_SECOND);

    let mut x = MILLIMETER_PER_SECOND_SQUARED;
    let y = SECOND;
    x *= y;
    x.assert_eq_assume_ok(&MILLIMETER_PER_SECOND);

    let x = MILLIMETER_PER_SECOND_SQUARED;
    let y = SECOND;
    let z = x / y;
    z.assert_eq_assume_ok(&MILLIMETER_PER_SECOND_CUBED);

    let mut x = MILLIMETER_PER_SECOND_SQUARED;
    let y = SECOND;
    x /= y;
    x.assert_eq_assume_ok(&MILLIMETER_PER_SECOND_CUBED);
}
#[test]
fn unit_neg() {
    MILLIMETER_PER_SECOND.assert_eq_assume_ok(&-MILLIMETER_PER_SECOND);
}
#[test]
fn quantity_new() {
    let x = Quantity::new(5.0, MILLIMETER_PER_SECOND);
    let y = Quantity {
        value: 5.0,
        unit: MILLIMETER_PER_SECOND,
    };
    assert_eq!(x, y);
}
#[test]
fn quantity_dimensionless() {
    let x = Quantity::dimensionless(5.0);
    let y = Quantity {
        value: 5.0,
        unit: DIMENSIONLESS,
    };
    assert_eq!(x, y);
}
#[test]
fn quantity_abs() {
    assert_eq!(
        Quantity::new(-5.0, MILLIMETER).abs(),
        Quantity::new(5.0, MILLIMETER)
    );
    assert_eq!(
        Quantity::new(0.0, MILLIMETER).abs(),
        Quantity::new(0.0, MILLIMETER)
    );
    assert_eq!(
        Quantity::new(5.0, MILLIMETER).abs(),
        Quantity::new(5.0, MILLIMETER)
    );
}
#[test]
fn quantity_from_command() {
    assert_eq!(
        Quantity::from(Command::Position(5.0)),
        Quantity::new(5.0, MILLIMETER)
    );
    assert_eq!(
        Quantity::from(Command::Velocity(5.0)),
        Quantity::new(5.0, MILLIMETER_PER_SECOND)
    );
    assert_eq!(
        Quantity::from(Command::Acceleration(5.0)),
        Quantity::new(5.0, MILLIMETER_PER_SECOND_SQUARED)
    );
}
#[test]
fn f32_from_quantity() {
    assert_eq!(5.0, f32::from(Quantity::new(5.0, MILLIMETER_PER_SECOND)));
}
#[test]
fn quantity_add_sub_success() {
    let x = Quantity::new(2.0, MILLIMETER_PER_SECOND);
    let y = Quantity::new(3.0, MILLIMETER_PER_SECOND);
    let z = x + y;
    assert_eq!(z, Quantity::new(5.0, MILLIMETER_PER_SECOND));

    let mut x = Quantity::new(2.0, MILLIMETER_PER_SECOND);
    let y = Quantity::new(3.0, MILLIMETER_PER_SECOND);
    x += y;
    assert_eq!(x, Quantity::new(5.0, MILLIMETER_PER_SECOND));

    let x = Quantity::new(3.0, MILLIMETER_PER_SECOND);
    let y = Quantity::new(2.0, MILLIMETER_PER_SECOND);
    let z = x - y;
    assert_eq!(z, Quantity::new(1.0, MILLIMETER_PER_SECOND));

    let mut x = Quantity::new(3.0, MILLIMETER_PER_SECOND);
    let y = Quantity::new(2.0, MILLIMETER_PER_SECOND);
    x -= y;
    assert_eq!(x, Quantity::new(1.0, MILLIMETER_PER_SECOND));

    let x = Quantity::new(2.0, SECOND);
    let y = Time::from_nanoseconds(3_000_000_000);
    assert_eq!(x + y, Quantity::new(5.0, SECOND));

    let mut x = Quantity::new(2.0, SECOND);
    let y = Time::from_nanoseconds(3_000_000_000);
    x += y;
    assert_eq!(x, Quantity::new(5.0, SECOND));

    let x = Quantity::new(3.0, SECOND);
    let y = Time::from_nanoseconds(2_000_000_000);
    assert_eq!(x - y, Quantity::new(1.0, SECOND));

    let mut x = Quantity::new(3.0, SECOND);
    let y = Time::from_nanoseconds(2_000_000_000);
    x -= y;
    assert_eq!(x, Quantity::new(1.0, SECOND));

    let x = Quantity::new(2.0, DIMENSIONLESS);
    let y = DimensionlessInteger(3);
    assert_eq!(x + y, Quantity::new(5.0, DIMENSIONLESS));

    let mut x = Quantity::new(2.0, DIMENSIONLESS);
    let y = DimensionlessInteger(3);
    x += y;
    assert_eq!(x, Quantity::new(5.0, DIMENSIONLESS));

    let x = Quantity::new(3.0, DIMENSIONLESS);
    let y = DimensionlessInteger(2);
    assert_eq!(x - y, Quantity::new(1.0, DIMENSIONLESS));

    let mut x = Quantity::new(3.0, DIMENSIONLESS);
    let y = DimensionlessInteger(2);
    x -= y;
    assert_eq!(x, Quantity::new(1.0, DIMENSIONLESS));
}
#[test]
#[should_panic]
#[cfg(any(
    feature = "dim_check_release",
    all(debug_assertions, feature = "dim_check_debug")
))]
fn quantity_add_failure() {
    let x = Quantity::new(2.0, MILLIMETER);
    let y = Quantity::new(3.0, MILLIMETER_PER_SECOND);
    let _ = x + y;
}
#[test]
#[should_panic]
#[cfg(any(
    feature = "dim_check_release",
    all(debug_assertions, feature = "dim_check_debug")
))]
fn quantity_add_assign_failure() {
    let mut x = Quantity::new(2.0, MILLIMETER);
    let y = Quantity::new(3.0, MILLIMETER_PER_SECOND);
    x += y;
}
#[test]
#[should_panic]
#[cfg(any(
    feature = "dim_check_release",
    all(debug_assertions, feature = "dim_check_debug")
))]
fn quantity_sub_failure() {
    let x = Quantity::new(3.0, MILLIMETER);
    let y = Quantity::new(2.0, MILLIMETER_PER_SECOND);
    let _ = x - y;
}
#[test]
#[should_panic]
#[cfg(any(
    feature = "dim_check_release",
    all(debug_assertions, feature = "dim_check_debug")
))]
fn quantity_sub_assign_failure() {
    let mut x = Quantity::new(3.0, MILLIMETER);
    let y = Quantity::new(2.0, MILLIMETER_PER_SECOND);
    x -= y;
}
#[test]
fn quantity_mul_div() {
    let x = Quantity::new(2.0, MILLIMETER_PER_SECOND_SQUARED);
    let y = Quantity::new(3.0, SECOND);
    let z = x * y;
    assert_eq!(z, Quantity::new(6.0, MILLIMETER_PER_SECOND));

    let mut x = Quantity::new(2.0, MILLIMETER_PER_SECOND_SQUARED);
    let y = Quantity::new(3.0, SECOND);
    x *= y;
    assert_eq!(x, Quantity::new(6.0, MILLIMETER_PER_SECOND));

    let x = Quantity::new(4.0, MILLIMETER_PER_SECOND_SQUARED);
    let y = Quantity::new(2.0, SECOND);
    let z = x / y;
    assert_eq!(z, Quantity::new(2.0, MILLIMETER_PER_SECOND_CUBED));

    let mut x = Quantity::new(4.0, MILLIMETER_PER_SECOND_SQUARED);
    let y = Quantity::new(2.0, SECOND);
    x /= y;
    assert_eq!(x, Quantity::new(2.0, MILLIMETER_PER_SECOND_CUBED));

    let x = Quantity::new(3.0, MILLIMETER_PER_SECOND_SQUARED);
    let y = Time::from_nanoseconds(2_000_000_000);
    assert_eq!(x * y, Quantity::new(6.0, MILLIMETER_PER_SECOND));

    let mut x = Quantity::new(3.0, MILLIMETER_PER_SECOND_SQUARED);
    let y = Time::from_nanoseconds(2_000_000_000);
    x *= y;
    assert_eq!(x, Quantity::new(6.0, MILLIMETER_PER_SECOND));

    let x = Quantity::new(4.0, MILLIMETER_PER_SECOND);
    let y = Time::from_nanoseconds(2_000_000_000);
    assert_eq!(x / y, Quantity::new(2.0, MILLIMETER_PER_SECOND_SQUARED));

    let mut x = Quantity::new(4.0, MILLIMETER_PER_SECOND);
    let y = Time::from_nanoseconds(2_000_000_000);
    x /= y;
    assert_eq!(x, Quantity::new(2.0, MILLIMETER_PER_SECOND_SQUARED));

    let x = Quantity::new(3.0, MILLIMETER_PER_SECOND_SQUARED);
    let y = DimensionlessInteger(2);
    assert_eq!(x * y, Quantity::new(6.0, MILLIMETER_PER_SECOND_SQUARED));

    let mut x = Quantity::new(3.0, MILLIMETER_PER_SECOND_SQUARED);
    let y = DimensionlessInteger(2);
    x *= y;
    assert_eq!(x, Quantity::new(6.0, MILLIMETER_PER_SECOND_SQUARED));

    let x = Quantity::new(4.0, MILLIMETER_PER_SECOND);
    let y = DimensionlessInteger(2);
    assert_eq!(x / y, Quantity::new(2.0, MILLIMETER_PER_SECOND));

    let mut x = Quantity::new(4.0, MILLIMETER_PER_SECOND);
    let y = DimensionlessInteger(2);
    x /= y;
    assert_eq!(x, Quantity::new(2.0, MILLIMETER_PER_SECOND));
}
#[test]
fn quantity_neg() {
    assert_eq!(
        -Quantity::new(5.0, MILLIMETER_PER_SECOND),
        Quantity::new(-5.0, MILLIMETER_PER_SECOND)
    );
}
#[test]
fn quantity_partial_ord() {
    assert_eq!(
        Quantity::new(5.0, MILLIMETER_PER_SECOND),
        Quantity::new(5.0, MILLIMETER_PER_SECOND)
    );
    assert!(Quantity::new(5.0, MILLIMETER_PER_SECOND) < Quantity::new(8.0, MILLIMETER_PER_SECOND));
    assert!(Quantity::new(5.0, MILLIMETER_PER_SECOND) > Quantity::new(2.0, MILLIMETER_PER_SECOND));
}
#[test]
#[should_panic]
#[cfg(any(
    feature = "dim_check_release",
    all(debug_assertions, feature = "dim_check_debug")
))]
fn quantity_partial_ord_failure() {
    let _ = Quantity::new(5.0, MILLIMETER_PER_SECOND)
        < Quantity::new(8.0, MILLIMETER_PER_SECOND_SQUARED);
}
