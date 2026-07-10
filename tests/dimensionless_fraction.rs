use rrtk::*;
#[test]
fn new_new_unchecked_eq() {
    let x = DimensionlessFraction::new(DimensionlessInteger(2), DimensionlessInteger(3));
    let y = DimensionlessFraction::new_unchecked(DimensionlessInteger(2), DimensionlessInteger(3));
    assert_eq!(x, y);
    let _ = DimensionlessFraction::new_unchecked(DimensionlessInteger(1), DimensionlessInteger(0));
}
#[test]
fn constructor_macro() {
    let x = DimensionlessFraction::new(DimensionlessInteger(1), DimensionlessInteger(4));
    let y = dimensionless_fraction!(1, 4);
    assert_eq!(x, y);
}
#[test]
fn constructor_macro_unchecked() {
    let x = DimensionlessFraction::new_unchecked(DimensionlessInteger(1), DimensionlessInteger(0));
    let y = dimensionless_fraction_unchecked!(1, 0);
    assert_eq!(x, y);
}
#[test]
#[should_panic]
fn div_by_zero_constructor_validation() {
    let _ = DimensionlessFraction::new(DimensionlessInteger(1), DimensionlessInteger(0));
}
#[test]
#[should_panic]
fn div_by_zero_constructor_validation_macro() {
    let _ = dimensionless_fraction!(1, 0);
}
#[test]
fn is_valid() {
    let x = dimensionless_fraction!(-1, 2);
    assert!(x.is_valid());
    let y = dimensionless_fraction_unchecked!(-1, 0);
    assert!(!y.is_valid());
}
#[test]
fn reciprocal() {
    let x = dimensionless_fraction!(2, -3);
    assert_eq!(x.reciprocal(), dimensionless_fraction!(-3, 2),);
}
#[test]
#[should_panic]
fn reciprocal_div_by_zero() {
    let x = dimensionless_fraction!(0, -5);
    let _ = x.reciprocal();
}
#[test]
fn reciprocal_unchecked() {
    let x = dimensionless_fraction!(2, -3);
    assert_eq!(x.reciprocal_unchecked(), dimensionless_fraction!(-3, 2),);
    let y = dimensionless_fraction!(0, -3);
    assert_eq!(
        y.reciprocal_unchecked(),
        dimensionless_fraction_unchecked!(-3, 0),
    );
}
#[test]
fn as_f32() {
    let x = dimensionless_fraction!(3, 2);
    assert_eq!(x.as_f32(), 1.5f32);
}
#[test]
fn as_f64() {
    let x = dimensionless_fraction!(3, 2);
    assert_eq!(x.as_f64(), 1.5f64);
}
#[test]
fn as_quantity_f32() {
    let x = dimensionless_fraction!(3, 2);
    assert_eq!(x.as_quantity_f32(), Dimensionless::new(1.5f32));
}
#[test]
fn as_quantity_f64() {
    let x = dimensionless_fraction!(3, 2);
    assert_eq!(x.as_quantity_f64(), Dimensionless::new(1.5f64));
}
#[test]
fn neg() {
    let x = dimensionless_fraction!(1, 2);
    assert_eq!(-x, dimensionless_fraction!(-1, 2));
}
#[test]
fn equality() {
    let x = dimensionless_fraction!(1, 2);
    let y = dimensionless_fraction!(2, 4);
    let z = dimensionless_fraction!(2, 5);
    assert_eq!(x, y);
    assert!(x != z);
    assert!(y != z)
}
#[test]
fn order() {
    let mut fracs = [
        dimensionless_fraction!(-2, 6),
        dimensionless_fraction!(-20, 5),
        dimensionless_fraction!(81, 4),
        dimensionless_fraction!(-3, 9),
        dimensionless_fraction!(1, 300),
        dimensionless_fraction!(2, 400),
        dimensionless_fraction!(5000, 400),
        dimensionless_fraction!(-3, 5),
        dimensionless_fraction!(160, 8),
        dimensionless_fraction!(80, 4),
        dimensionless_fraction!(5000, 1),
        dimensionless_fraction!(0, 3),
        dimensionless_fraction!(-4, 1),
        dimensionless_fraction!(5000, 2),
        dimensionless_fraction!(0, 5),
        dimensionless_fraction!(1, 200),
        dimensionless_fraction!(2, 300),
        dimensionless_fraction!(0, 2),
        dimensionless_fraction!(-1, 3),
        dimensionless_fraction!(1, 100),
    ];
    let fracs_correct = [
        dimensionless_fraction!(-20, 5),
        dimensionless_fraction!(-4, 1),
        dimensionless_fraction!(-3, 5),
        dimensionless_fraction!(-2, 6),
        dimensionless_fraction!(-3, 9),
        dimensionless_fraction!(-1, 3),
        dimensionless_fraction!(0, 3),
        dimensionless_fraction!(0, 5),
        dimensionless_fraction!(0, 2),
        dimensionless_fraction!(1, 300),
        dimensionless_fraction!(2, 400),
        dimensionless_fraction!(1, 200),
        dimensionless_fraction!(2, 300),
        dimensionless_fraction!(1, 100),
        dimensionless_fraction!(5000, 400),
        dimensionless_fraction!(160, 8),
        dimensionless_fraction!(80, 4),
        dimensionless_fraction!(81, 4),
        dimensionless_fraction!(5000, 2),
        dimensionless_fraction!(5000, 1),
    ];
    fracs.sort();
    assert_eq!(fracs, fracs_correct);
}
#[test]
fn neg_zero() {
    let x = dimensionless_fraction!(0, 1);
    assert_eq!(-x, x);
}
#[test]
fn mul_self() {
    let x = dimensionless_fraction!(2, 3);
    let y = dimensionless_fraction!(1, 2);
    let z = dimensionless_fraction!(1, 3);
    assert_eq!(x * y, z);
}
#[test]
fn mul_assign_self() {
    let mut x = dimensionless_fraction!(2, 3);
    let y = dimensionless_fraction!(1, 2);
    let z = dimensionless_fraction!(1, 3);
    x *= y;
    assert_eq!(x, z);
}
#[test]
fn div_self() {
    let x = dimensionless_fraction!(2, 3);
    let y = dimensionless_fraction!(1, 2);
    let z = dimensionless_fraction!(4, 3);
    assert_eq!(x / y, z);
}
#[test]
fn div_assign_self() {
    let mut x = dimensionless_fraction!(2, 3);
    let y = dimensionless_fraction!(1, 2);
    let z = dimensionless_fraction!(4, 3);
    x /= y;
    assert_eq!(x, z);
}
#[test]
fn add_self() {
    let x = dimensionless_fraction!(2, 3);
    let y = dimensionless_fraction!(1, 2);
    let z = dimensionless_fraction!(7, 6);
    assert_eq!(x + y, z);
}
#[test]
fn add_assign_self() {
    let mut x = dimensionless_fraction!(2, 3);
    let y = dimensionless_fraction!(1, 2);
    let z = dimensionless_fraction!(7, 6);
    x += y;
    assert_eq!(x, z);
}
#[test]
fn sub_self() {
    let x = dimensionless_fraction!(2, 3);
    let y = dimensionless_fraction!(1, 2);
    let z = dimensionless_fraction!(1, 6);
    assert_eq!(x - y, z);
}
#[test]
fn sub_assign_self() {
    let mut x = dimensionless_fraction!(2, 3);
    let y = dimensionless_fraction!(1, 2);
    let z = dimensionless_fraction!(1, 6);
    x -= y;
    assert_eq!(x, z);
}
#[test]
fn mul_int() {
    assert_eq!(
        dimensionless_fraction!(5, 3) * DimensionlessInteger(2),
        dimensionless_fraction!(10, 3)
    );
}
#[test]
fn mul_assign_int() {
    let mut x = dimensionless_fraction!(5, 3);
    x *= DimensionlessInteger(2);
    assert_eq!(x, dimensionless_fraction!(10, 3));
}
#[test]
fn div_int() {
    assert_eq!(
        dimensionless_fraction!(5, 3) / DimensionlessInteger(2),
        dimensionless_fraction!(5, 6)
    );
}
#[test]
fn div_assign_int() {
    let mut x = dimensionless_fraction!(5, 3);
    x /= DimensionlessInteger(2);
    assert_eq!(x, dimensionless_fraction!(5, 6));
}
#[test]
fn add_int() {
    assert_eq!(
        dimensionless_fraction!(5, 3) + DimensionlessInteger(2),
        dimensionless_fraction!(11, 3)
    );
}
#[test]
fn add_assign_int() {
    let mut x = dimensionless_fraction!(5, 3);
    x += DimensionlessInteger(2);
    assert_eq!(x, dimensionless_fraction!(11, 3));
}
#[test]
fn sub_int() {
    assert_eq!(
        dimensionless_fraction!(5, 3) - DimensionlessInteger(2),
        dimensionless_fraction!(-1, 3)
    );
}
#[test]
fn sub_assign_int() {
    let mut x = dimensionless_fraction!(5, 3);
    x -= DimensionlessInteger(2);
    assert_eq!(x, dimensionless_fraction!(-1, 3));
}
#[test]
fn mul_time() {
    let x = dimensionless_fraction!(2, 3);
    let y = Time::from_nanoseconds(6_000_000);
    assert_eq!(x * y, Time::from_nanoseconds(4_000_000));
}
#[test]
fn mul_int_reverse() {
    let x = DimensionlessInteger(2);
    let y = dimensionless_fraction!(5, 3);
    assert_eq!(x * y, dimensionless_fraction!(10, 3));
}
#[test]
fn div_int_reverse() {
    let x = DimensionlessInteger(2);
    let y = dimensionless_fraction!(5, 3);
    assert_eq!(x / y, dimensionless_fraction!(6, 5));
}
#[test]
fn mul_time_reverse() {
    let x = Time::from_nanoseconds(6_000_000);
    let y = dimensionless_fraction!(2, 3);
    assert_eq!(x * y, Time::from_nanoseconds(4_000_000));
}
#[test]
fn mul_time_reverse_assign() {
    let mut x = Time::from_nanoseconds(6_000_000);
    x *= dimensionless_fraction!(2, 3);
    assert_eq!(x, Time::from_nanoseconds(4_000_000));
}
#[test]
fn time_div_by_dim_frac() {
    let x = Time::from_nanoseconds(6_000_000);
    let y = dimensionless_fraction!(2, 3);
    assert_eq!(x / y, Time::from_nanoseconds(9_000_000));
}
#[test]
fn time_div_by_dim_frac_assign() {
    let mut x = Time::from_nanoseconds(6_000_000);
    x /= dimensionless_fraction!(2, 3);
    assert_eq!(x, Time::from_nanoseconds(9_000_000));
}
#[test]
fn add_int_reverse() {
    assert_eq!(
        DimensionlessInteger(2) + dimensionless_fraction!(5, 3),
        dimensionless_fraction!(11, 3)
    );
}
#[test]
fn sub_int_reverse() {
    assert_eq!(
        DimensionlessInteger(2) - dimensionless_fraction!(5, 3),
        dimensionless_fraction!(1, 3)
    );
}
