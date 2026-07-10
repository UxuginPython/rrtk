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
    let x = DimensionlessFraction::new(DimensionlessInteger(-1), DimensionlessInteger(2));
    assert!(x.is_valid());
    let y = DimensionlessFraction::new_unchecked(DimensionlessInteger(-1), DimensionlessInteger(0));
    assert!(!y.is_valid());
}
#[test]
fn reciprocal() {
    let x = DimensionlessFraction::new(DimensionlessInteger(2), DimensionlessInteger(-3));
    assert_eq!(
        x.reciprocal(),
        DimensionlessFraction::new(DimensionlessInteger(-3), DimensionlessInteger(2))
    );
}
#[test]
#[should_panic]
fn reciprocal_div_by_zero() {
    let x = DimensionlessFraction::new(DimensionlessInteger(0), DimensionlessInteger(-5));
    let _ = x.reciprocal();
}
#[test]
fn reciprocal_unchecked() {
    let x = DimensionlessFraction::new(DimensionlessInteger(2), DimensionlessInteger(-3));
    assert_eq!(
        x.reciprocal_unchecked(),
        DimensionlessFraction::new(DimensionlessInteger(-3), DimensionlessInteger(2))
    );
    let y = DimensionlessFraction::new(DimensionlessInteger(0), DimensionlessInteger(-3));
    assert_eq!(
        y.reciprocal_unchecked(),
        DimensionlessFraction::new_unchecked(DimensionlessInteger(-3), DimensionlessInteger(0))
    );
}
#[test]
fn as_f32() {
    let x = DimensionlessFraction::new(DimensionlessInteger(3), DimensionlessInteger(2));
    assert_eq!(x.as_f32(), 1.5f32);
}
#[test]
fn as_f64() {
    let x = DimensionlessFraction::new(DimensionlessInteger(3), DimensionlessInteger(2));
    assert_eq!(x.as_f64(), 1.5f64);
}
#[test]
fn as_quantity_f32() {
    let x = DimensionlessFraction::new(DimensionlessInteger(3), DimensionlessInteger(2));
    assert_eq!(x.as_quantity_f32(), Dimensionless::new(1.5f32));
}
#[test]
fn as_quantity_f64() {
    let x = DimensionlessFraction::new(DimensionlessInteger(3), DimensionlessInteger(2));
    assert_eq!(x.as_quantity_f64(), Dimensionless::new(1.5f64));
}
#[test]
fn add_self() {
    let x = dimensionless_fraction!(2, 3);
    let y = dimensionless_fraction!(1, 2);
    let z = dimensionless_fraction!(7, 6);
    assert_eq!(x + y, z);
}
