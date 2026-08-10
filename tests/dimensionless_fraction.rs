// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2024-2026 UxuginPython
use rrtk::*;
#[test]
fn new_new_unchecked_eq() {
    let x = DimensionlessFraction::new(DimensionlessInteger(2), DimensionlessInteger(3));
    let y = DimensionlessFraction::new_unchecked(DimensionlessInteger(2), DimensionlessInteger(3));
    assert_eq!(x, y);
    let _ = DimensionlessFraction::new_unchecked(DimensionlessInteger(1), DimensionlessInteger(0));
}
#[test]
fn from_raw() {
    let x = DimensionlessFraction::new(DimensionlessInteger(1), DimensionlessInteger(4));
    let y = DimensionlessFraction::from_raw(1, 4);
    assert_eq!(x, y);
}
#[test]
fn from_raw_unchecked() {
    let x = DimensionlessFraction::new_unchecked(DimensionlessInteger(1), DimensionlessInteger(0));
    let y = DimensionlessFraction::from_raw_unchecked(1, 0);
    assert!(x.raw_eq(&y));
}
#[test]
fn raw_eq() {
    assert!(DimensionlessFraction::from_raw(2, 3).raw_eq(&DimensionlessFraction::from_raw(2, 3)));
    assert!(!DimensionlessFraction::from_raw(1, 3).raw_eq(&DimensionlessFraction::from_raw(1, 2)));
    assert!(!DimensionlessFraction::from_raw(1, 3).raw_eq(&DimensionlessFraction::from_raw(2, 3)));
    assert!(!DimensionlessFraction::from_raw(1, 3).raw_eq(&DimensionlessFraction::from_raw(3, 4)));
    assert!(!DimensionlessFraction::from_raw(1, 2).raw_eq(&DimensionlessFraction::from_raw(2, 4)));
    assert!(!DimensionlessFraction::from_raw(0, 1).raw_eq(&DimensionlessFraction::from_raw(0, 2)));
    assert!(
        !DimensionlessFraction::from_raw(1, 2).raw_eq(&DimensionlessFraction::from_raw(-1, -2))
    );
    assert!(
        !DimensionlessFraction::from_raw_unchecked(1, 0)
            .raw_eq(&DimensionlessFraction::from_raw_unchecked(2, 0))
    );
    assert!(
        DimensionlessFraction::from_raw_unchecked(2, 0)
            .raw_eq(&DimensionlessFraction::from_raw_unchecked(2, 0))
    );
}
#[test]
#[should_panic]
fn div_by_zero_constructor_validation() {
    let _ = DimensionlessFraction::new(DimensionlessInteger(1), DimensionlessInteger(0));
}
#[test]
#[should_panic]
fn div_by_zero_constructor_validation_macro() {
    let _ = DimensionlessFraction::from_raw(1, 0);
}
#[test]
fn is_valid() {
    let x = DimensionlessFraction::from_raw(-1, 2);
    assert!(x.is_valid());
    let y = DimensionlessFraction::from_raw_unchecked(-1, 0);
    assert!(!y.is_valid());
}
#[test]
fn reciprocal() {
    let x = DimensionlessFraction::from_raw(2, -3);
    assert_eq!(x.reciprocal(), DimensionlessFraction::from_raw(-3, 2),);
}
#[test]
#[should_panic]
fn reciprocal_div_by_zero() {
    let x = DimensionlessFraction::from_raw(0, -5);
    let _ = x.reciprocal();
}
#[test]
fn reciprocal_unchecked() {
    let x = DimensionlessFraction::from_raw(2, -3);
    assert_eq!(
        x.reciprocal_unchecked(),
        DimensionlessFraction::from_raw(-3, 2),
    );
    let y = DimensionlessFraction::from_raw(0, -3);
    assert!(
        y.reciprocal_unchecked()
            .raw_eq(&DimensionlessFraction::from_raw_unchecked(-3, 0))
    );
}
#[test]
fn as_f32() {
    let x = DimensionlessFraction::from_raw(3, 2);
    assert_eq!(x.as_f32(), 1.5f32);
    assert_eq!(f32::from(x), 1.5f32);
}
#[test]
fn as_f64() {
    let x = DimensionlessFraction::from_raw(3, 2);
    assert_eq!(x.as_f64(), 1.5f64);
    assert_eq!(f64::from(x), 1.5f64);
}
#[test]
fn as_quantity_f32() {
    let x = DimensionlessFraction::from_raw(3, 2);
    assert_eq!(x.as_quantity_f32(), Dimensionless::new(1.5f32));
    assert_eq!(Dimensionless::<f32>::from(x), Dimensionless::new(1.5f32));
}
#[test]
fn as_quantity_f64() {
    let x = DimensionlessFraction::from_raw(3, 2);
    assert_eq!(x.as_quantity_f64(), Dimensionless::new(1.5f64));
    assert_eq!(Dimensionless::<f64>::from(x), Dimensionless::new(1.5f64));
}
#[test]
fn neg() {
    let x = DimensionlessFraction::from_raw(1, 2);
    assert_eq!(-x, DimensionlessFraction::from_raw(-1, 2));
}
#[test]
fn equality() {
    let x = DimensionlessFraction::from_raw(1, 2);
    let y = DimensionlessFraction::from_raw(2, 4);
    let z = DimensionlessFraction::from_raw(2, 5);
    assert_eq!(x, y);
    assert!(x != z);
    assert!(y != z)
}
#[test]
fn order() {
    let mut fracs = [
        DimensionlessFraction::from_raw(-2, 6),
        DimensionlessFraction::from_raw(-20, 5),
        DimensionlessFraction::from_raw(81, 4),
        DimensionlessFraction::from_raw(-3, 9),
        DimensionlessFraction::from_raw(1, 300),
        DimensionlessFraction::from_raw(2, 400),
        DimensionlessFraction::from_raw(5000, 400),
        DimensionlessFraction::from_raw(-3, 5),
        DimensionlessFraction::from_raw(160, 8),
        DimensionlessFraction::from_raw(80, 4),
        DimensionlessFraction::from_raw(5000, 1),
        DimensionlessFraction::from_raw(0, 3),
        DimensionlessFraction::from_raw(-4, 1),
        DimensionlessFraction::from_raw(5000, 2),
        DimensionlessFraction::from_raw(0, 5),
        DimensionlessFraction::from_raw(1, 200),
        DimensionlessFraction::from_raw(2, 300),
        DimensionlessFraction::from_raw(0, 2),
        DimensionlessFraction::from_raw(-1, 3),
        DimensionlessFraction::from_raw(1, 100),
    ];
    let fracs_correct = [
        DimensionlessFraction::from_raw(-20, 5),
        DimensionlessFraction::from_raw(-4, 1),
        DimensionlessFraction::from_raw(-3, 5),
        DimensionlessFraction::from_raw(-2, 6),
        DimensionlessFraction::from_raw(-3, 9),
        DimensionlessFraction::from_raw(-1, 3),
        DimensionlessFraction::from_raw(0, 3),
        DimensionlessFraction::from_raw(0, 5),
        DimensionlessFraction::from_raw(0, 2),
        DimensionlessFraction::from_raw(1, 300),
        DimensionlessFraction::from_raw(2, 400),
        DimensionlessFraction::from_raw(1, 200),
        DimensionlessFraction::from_raw(2, 300),
        DimensionlessFraction::from_raw(1, 100),
        DimensionlessFraction::from_raw(5000, 400),
        DimensionlessFraction::from_raw(160, 8),
        DimensionlessFraction::from_raw(80, 4),
        DimensionlessFraction::from_raw(81, 4),
        DimensionlessFraction::from_raw(5000, 2),
        DimensionlessFraction::from_raw(5000, 1),
    ];
    fracs.sort();
    assert_eq!(fracs, fracs_correct);
}
#[test]
fn order_2() {
    let a = DimensionlessFraction::from_raw(3, 2);
    let b = DimensionlessFraction::from_raw(1, 2);
    assert!(a > b);
    let c = DimensionlessFraction::from_raw(-3, -2);
    let d = DimensionlessFraction::from_raw(-1, -2);
    assert_eq!(a, c);
    assert_eq!(b, d);
    assert!(c > d);
    assert!(a > d);
    assert!(c > b);
}
#[test]
fn order_3() {
    let fracs = [
        DimensionlessFraction::from_raw(-3, 2),
        DimensionlessFraction::from_raw(3, -2),
        DimensionlessFraction::from_raw(-1, 2),
        DimensionlessFraction::from_raw(1, -2),
        DimensionlessFraction::from_raw(0, 2),
        DimensionlessFraction::from_raw(0, -2),
        DimensionlessFraction::from_raw(1, 2),
        DimensionlessFraction::from_raw(-1, -2),
        DimensionlessFraction::from_raw(3, 2),
        DimensionlessFraction::from_raw(-3, -2),
    ];
    for a in fracs {
        for b in fracs {
            assert_eq!(a.cmp(&b), a.as_f32().partial_cmp(&b.as_f32()).unwrap());
        }
    }
}
#[test]
fn neg_zero() {
    let x = DimensionlessFraction::from_raw(0, 1);
    assert_eq!(-x, x);
}
#[test]
fn mul_self() {
    let x = DimensionlessFraction::from_raw(2, 3);
    let y = DimensionlessFraction::from_raw(1, 2);
    let z = DimensionlessFraction::from_raw(1, 3);
    assert_eq!(x * y, z);
}
#[test]
fn mul_assign_self() {
    let mut x = DimensionlessFraction::from_raw(2, 3);
    let y = DimensionlessFraction::from_raw(1, 2);
    let z = DimensionlessFraction::from_raw(1, 3);
    x *= y;
    assert_eq!(x, z);
}
#[test]
fn div_self() {
    let x = DimensionlessFraction::from_raw(2, 3);
    let y = DimensionlessFraction::from_raw(1, 2);
    let z = DimensionlessFraction::from_raw(4, 3);
    assert_eq!(x / y, z);
}
#[test]
fn div_assign_self() {
    let mut x = DimensionlessFraction::from_raw(2, 3);
    let y = DimensionlessFraction::from_raw(1, 2);
    let z = DimensionlessFraction::from_raw(4, 3);
    x /= y;
    assert_eq!(x, z);
}
#[test]
fn add_self() {
    let x = DimensionlessFraction::from_raw(2, 3);
    let y = DimensionlessFraction::from_raw(1, 2);
    let z = DimensionlessFraction::from_raw(7, 6);
    assert_eq!(x + y, z);
}
#[test]
fn add_assign_self() {
    let mut x = DimensionlessFraction::from_raw(2, 3);
    let y = DimensionlessFraction::from_raw(1, 2);
    let z = DimensionlessFraction::from_raw(7, 6);
    x += y;
    assert_eq!(x, z);
}
#[test]
fn sub_self() {
    let x = DimensionlessFraction::from_raw(2, 3);
    let y = DimensionlessFraction::from_raw(1, 2);
    let z = DimensionlessFraction::from_raw(1, 6);
    assert_eq!(x - y, z);
}
#[test]
fn sub_assign_self() {
    let mut x = DimensionlessFraction::from_raw(2, 3);
    let y = DimensionlessFraction::from_raw(1, 2);
    let z = DimensionlessFraction::from_raw(1, 6);
    x -= y;
    assert_eq!(x, z);
}
#[test]
fn mul_int() {
    assert_eq!(
        DimensionlessFraction::from_raw(5, 3) * DimensionlessInteger(2),
        DimensionlessFraction::from_raw(10, 3)
    );
}
#[test]
fn mul_assign_int() {
    let mut x = DimensionlessFraction::from_raw(5, 3);
    x *= DimensionlessInteger(2);
    assert_eq!(x, DimensionlessFraction::from_raw(10, 3));
}
#[test]
fn div_int() {
    assert_eq!(
        DimensionlessFraction::from_raw(5, 3) / DimensionlessInteger(2),
        DimensionlessFraction::from_raw(5, 6)
    );
}
#[test]
fn div_assign_int() {
    let mut x = DimensionlessFraction::from_raw(5, 3);
    x /= DimensionlessInteger(2);
    assert_eq!(x, DimensionlessFraction::from_raw(5, 6));
}
#[test]
fn add_int() {
    assert_eq!(
        DimensionlessFraction::from_raw(5, 3) + DimensionlessInteger(2),
        DimensionlessFraction::from_raw(11, 3)
    );
}
#[test]
fn add_assign_int() {
    let mut x = DimensionlessFraction::from_raw(5, 3);
    x += DimensionlessInteger(2);
    assert_eq!(x, DimensionlessFraction::from_raw(11, 3));
}
#[test]
fn sub_int() {
    assert_eq!(
        DimensionlessFraction::from_raw(5, 3) - DimensionlessInteger(2),
        DimensionlessFraction::from_raw(-1, 3)
    );
}
#[test]
fn sub_assign_int() {
    let mut x = DimensionlessFraction::from_raw(5, 3);
    x -= DimensionlessInteger(2);
    assert_eq!(x, DimensionlessFraction::from_raw(-1, 3));
}
#[test]
fn mul_time() {
    let x = DimensionlessFraction::from_raw(2, 3);
    let y = Time::from_nanoseconds(6_000_000);
    assert_eq!(x * y, Time::from_nanoseconds(4_000_000));
}
#[test]
fn mul_int_reverse() {
    let x = DimensionlessInteger(2);
    let y = DimensionlessFraction::from_raw(5, 3);
    assert_eq!(x * y, DimensionlessFraction::from_raw(10, 3));
}
#[test]
fn div_int_reverse() {
    let x = DimensionlessInteger(2);
    let y = DimensionlessFraction::from_raw(5, 3);
    assert_eq!(x / y, DimensionlessFraction::from_raw(6, 5));
}
#[test]
fn mul_time_reverse() {
    let x = Time::from_nanoseconds(6_000_000);
    let y = DimensionlessFraction::from_raw(2, 3);
    assert_eq!(x * y, Time::from_nanoseconds(4_000_000));
}
#[test]
fn mul_time_reverse_assign() {
    let mut x = Time::from_nanoseconds(6_000_000);
    x *= DimensionlessFraction::from_raw(2, 3);
    assert_eq!(x, Time::from_nanoseconds(4_000_000));
}
#[test]
fn time_div_by_dim_frac() {
    let x = Time::from_nanoseconds(6_000_000);
    let y = DimensionlessFraction::from_raw(2, 3);
    assert_eq!(x / y, Time::from_nanoseconds(9_000_000));
}
#[test]
fn time_div_by_dim_frac_assign() {
    let mut x = Time::from_nanoseconds(6_000_000);
    x /= DimensionlessFraction::from_raw(2, 3);
    assert_eq!(x, Time::from_nanoseconds(9_000_000));
}
#[test]
fn add_int_reverse() {
    assert_eq!(
        DimensionlessInteger(2) + DimensionlessFraction::from_raw(5, 3),
        DimensionlessFraction::from_raw(11, 3)
    );
}
#[test]
fn sub_int_reverse() {
    assert_eq!(
        DimensionlessInteger(2) - DimensionlessFraction::from_raw(5, 3),
        DimensionlessFraction::from_raw(1, 3)
    );
}
