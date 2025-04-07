// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2024-2025 UxuginPython
//!This module contains types related to RRTK's dimensional analysis system. RRTK uses nanoseconds
//!for time because they typically work nicely with computer clocks and are still precise when
//!stored in an integer, which is important because exponentially losing precision for time is bad,
//!and float time does that. However, floats are used for other quantities, including quantities
//!derived from time. These use seconds instead because numbers of the magnitude of nanoseconds
//!cause floats to lose precision. RRTK should handle the conversion mostly seamlessly for you, but
//!keep it in mind when thinking about how time-related types should work. The reasoning behind
//!this unorthodox system using both nanoseconds and seconds becomes more apparent when you know
//!how floating point numbers work. Everything in this module is reexported at the crate level.
//!
//!### Multiplication and Division Implementation Table
//!| A right; B down              | [`Quantity`]      | [`DimensionlessInteger`] | [`Time`]          |
//!|------------------------------|-------------------|--------------------------|-------------------|
//!| **[`Quantity`]**             | `*` `/` `*=` `/=` | `*` `/`                  | `*` `/`           |
//!| **[`DimensionlessInteger`]** | `*` `/` `*=` `/=` | `*` `/` `*=` `/=`        | `*` `/` `*=` `/=` |
//!| **[`Time`]**                 | `*` `/` `*=` `/=` | `*` `/`                  | `*` `/`           |
//!
//!`A <operation> B` compiles for any operation in the square of A and B. E.g., `*` is in the
//!square in the [`Quantity`] column and the [`DimensionlessInteger`] row, so the following works:
//!```
//!# use rrtk::*;
//!let x = Quantity::new(3.0, MILLIMETER);
//!let y = DimensionlessInteger(2);
//!let z = x * y;
//!```
//!A similar example for `*=`:
//!```
//!# use rrtk::*;
//!let mut x = Quantity::new(3.0, MILLIMETER);
//!let y = DimensionlessInteger(2);
//!x *= y;
//!```
//!Whenever `*` and `/` are in a square but `*=` and `/=` are not, `A * B` and `A / B`
//!return a type other than A. Since [`MulAssign`] and `DivAssign` require that A not change type in
//!`A *= B` and `A /= B`, it is not possible to implement them.
//!```
//!# use rrtk::*;
//!let x = Time::from_nanoseconds(2_000_000_000);
//!let y = Quantity::new(3.0, MILLIMETER_PER_SECOND);
//!let z = x * y;
//!assert_eq!(z, Quantity::new(6.0, MILLIMETER));
//!```
//!```compile_fail
//!# use rrtk::*;
//!let mut x = Time::from_nanoseconds(2_000_000_000);
//!let y = Quantity::new(3.0, MILLIMETER_PER_SECOND);
//!x *= y;
//!```
//!Note that this disparity is not necessarily symmetrical between types:
//!```
//!# use rrtk::*;
//!let mut x = Quantity::new(3.0, MILLIMETER_PER_SECOND);
//!let y = Time::from_nanoseconds(2_000_000_000);
//!x *= y;
//!assert_eq!(x, Quantity::new(6.0, MILLIMETER));
//!```
//!### Addition and Subtraction Implementation Table
//!| A right; B down              | [`Quantity`]             | [`DimensionlessInteger`] | [`Time`]                 |
//!|------------------------------|--------------------------|--------------------------|--------------------------|
//!| **[`Quantity`]**             | **P:** `+` `-` `+=` `-=` | **P:** `+` `-`           | **P:** `+` `-`           |
//!| **[`DimensionlessInteger`]** | **P:** `+` `-` `+=` `-=` | **G:** `+` `-` `+=` `-=` |                          |
//!| **[`Time`]**                 | **P:** `+` `-` `+=` `-=` |                          | **G:** `+` `-` `+=` `-=` |
//!
//!Addition and subtraction are a bit different because they can sometimes panic on a unit
//!mismatch. This table works the same way as the one above it except for the following:
//!- **P(anicking):** This operation may panic on a unit mismatch.
//The panic!() at the end of this example is so that it panics even when dimension checking is off.
//Cargo runs this with the other tests and, since it it marked should_panic, fails if it does not
//panic. This is a problem because it cannot panic with dimension checking off. A panic!() call at
//the end is the simplest way to ensure that this is not an issue, although it does eliminate the
//usefulness of this as a test. It is tested elsewhere, however; use quantity_add_failure in
//tests/dimensions.rs to test the panicking functionality.
//!```should_panic
//!# use rrtk::*;
//!let x = Quantity::new(2.0, MILLIMETER);
//!let y = Quantity::new(3.0, SECOND);
//!let z = x + y;
//!# panic!();
//!```
//!- **G(uaranteed):** Correct units are guaranteed by the types involved. This operation cannot panic.
//!
//!All operations in the multiplication and division table can be considered "Guaranteed."
//!### Conversion Implementation Table
//!| A right; B down              | [`Quantity`] | [`DimensionlessInteger`] | [`Time`]  | [`i64`] | [`f32`] |
//!|------------------------------|--------------|--------------------------|-----------|---------|---------|
//!| **[`Quantity`]**             | *is*         | `TryFrom`                | `TryFrom` |         | `From`  |
//!| **[`DimensionlessInteger`]** | `From`       | *is*                     |           | `From`  |         |
//!| **[`Time`]**                 | `From`       |                          | *is*      | `From`  |         |
//!| **[`i64`]**                  |              | `From`                   | `From`    | *is*    | [^lang] |
//!| **[`f32`]**                  | [^new]       |                          |           | [^lang] | *is*    |
//!
//![^lang]: See Rust language documentation.
//!
//![^new]: [`Quantity`] can be constructed from [`f32`] through [`Quantity::new`] by supplying a [`Unit`].
//!However, [`f32`] cannot be directly converted to [`Quantity`].
//!
//!This table is very similar: `A::<from/try_from>(B)` compiles for either `from`
//!or `try_from` depending on which is in the square of A and B, and you cannot convert between
//!types with nothing in their square. A [`From`] B implies B [`Into`] A and similarly for
//![`TryFrom`]/[`TryInto`] as is the case for all [`From`] implementations.
//!
//![`From`] is in the [`Quantity`] column and the [`DimensionlessInteger`] row, so the following works:
//!```
//!# use rrtk::*;
//!let x = DimensionlessInteger(3);
//!let y = Quantity::from(x);
//!```
//!And with [`Into`]:
//!```
//!# use rrtk::*;
//!let x = DimensionlessInteger(3);
//!let y: Quantity = x.into();
//!```
use super::*;
use compile_time_integer::*;
pub mod constants;
pub use constants::*;
///A time stored internally in `i64` nanoseconds. Mostly interacts with other types through `f32`
///seconds however.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Time(i64);
impl Time {
    ///Zero time. You would get this from `Time::from_nanoseconds(0)`.
    pub const ZERO: Self = Time(0);
    ///Construct a `Time` from `i64` nanoseconds, which is how the time is stored internally.
    pub const fn from_nanoseconds(value: i64) -> Self {
        Self(value)
    }
    ///Construct a `Time` from `f32` seconds.
    pub const fn from_seconds(value: f32) -> Self {
        Self((value * 1_000_000_000.0) as i64)
    }
    ///Construct a `Time` from compile-time [`Quantity`](compile_time_dimensions::Quantity) seconds stored using `f32`.
    pub fn from_compile_time_quantity(
        value: compile_time_dimensions::Quantity<f32, Zero, OnePlus<Zero>>,
    ) -> Self {
        Self::from_seconds(value.into_inner())
    }
    ///Get the internal `i64` nanoseconds from the `Time`.
    pub const fn as_nanoseconds(self) -> i64 {
        self.0
    }
    ///Get the value of the `Time` as `f32` seconds.
    pub const fn as_seconds(self) -> f32 {
        (self.0 as f32) / 1_000_000_000.0
    }
    ///Get the value of the `Time` as compile-time `Quantity` seconds stored using `f32`.
    ///Effectively a wrapper for [`as_seconds`](Self::as_seconds).
    pub const fn as_compile_time_quantity(
        self,
    ) -> compile_time_dimensions::Quantity<f32, Zero, OnePlus<Zero>> {
        compile_time_dimensions::Quantity::new(self.as_seconds())
    }
}
impl From<compile_time_dimensions::Quantity<f32, Zero, OnePlus<Zero>>> for Time {
    fn from(was: compile_time_dimensions::Quantity<f32, Zero, OnePlus<Zero>>) -> Self {
        Self::from_compile_time_quantity(was)
    }
}
impl From<Time> for compile_time_dimensions::Quantity<f32, Zero, OnePlus<Zero>> {
    fn from(was: Time) -> Self {
        was.as_compile_time_quantity()
    }
}
//TODO: figure out for to use the Error enum with this
impl TryFrom<Quantity> for Time {
    type Error = CannotConvert;
    fn try_from(was: Quantity) -> Result<Self, CannotConvert> {
        if was.unit.eq_assume_true(&SECOND) {
            Ok(Self((was.value * 1_000_000_000.0) as i64))
        } else {
            Err(CannotConvert)
        }
    }
}
impl From<Time> for Quantity {
    fn from(was: Time) -> Quantity {
        Quantity::new(was.0 as f32 / 1_000_000_000.0, SECOND)
    }
}
impl Add for Time {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
    }
}
impl AddAssign for Time {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}
impl Sub for Time {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0)
    }
}
impl SubAssign for Time {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}
impl Mul for Time {
    type Output = Quantity;
    fn mul(self, rhs: Self) -> Quantity {
        Quantity::from(self) * Quantity::from(rhs)
    }
}
impl Div for Time {
    type Output = Quantity;
    fn div(self, rhs: Self) -> Quantity {
        Quantity::from(self) / Quantity::from(rhs)
    }
}
impl Neg for Time {
    type Output = Self;
    fn neg(self) -> Self {
        Self(-self.0)
    }
}
impl Mul<DimensionlessInteger> for Time {
    type Output = Self;
    fn mul(self, rhs: DimensionlessInteger) -> Self {
        Self(self.0 * rhs.0)
    }
}
impl MulAssign<DimensionlessInteger> for Time {
    fn mul_assign(&mut self, rhs: DimensionlessInteger) {
        self.0 *= rhs.0;
    }
}
impl Div<DimensionlessInteger> for Time {
    type Output = Self;
    fn div(self, rhs: DimensionlessInteger) -> Self {
        Self(self.0 / rhs.0)
    }
}
impl DivAssign<DimensionlessInteger> for Time {
    fn div_assign(&mut self, rhs: DimensionlessInteger) {
        self.0 /= rhs.0;
    }
}
impl Add<Quantity> for Time {
    type Output = Quantity;
    fn add(self, rhs: Quantity) -> Quantity {
        Quantity::from(self) + rhs
    }
}
impl Sub<Quantity> for Time {
    type Output = Quantity;
    fn sub(self, rhs: Quantity) -> Quantity {
        Quantity::from(self) - rhs
    }
}
impl Mul<Quantity> for Time {
    type Output = Quantity;
    fn mul(self, rhs: Quantity) -> Quantity {
        rhs * self
    }
}
impl Div<Quantity> for Time {
    type Output = Quantity;
    fn div(self, rhs: Quantity) -> Quantity {
        Quantity::from(self) / rhs
    }
}
///Converts the time to `f32` seconds before the operation.
impl Mul<f32> for Time {
    type Output = f32;
    fn mul(self, rhs: f32) -> f32 {
        self.as_seconds() * rhs
    }
}
///Converts the time to `f32` seconds before the operation.
impl Mul<Time> for f32 {
    type Output = Self;
    fn mul(self, rhs: Time) -> Self {
        self * rhs.as_seconds()
    }
}
///Converts the time to `f32` seconds before the operation.
impl Div<f32> for Time {
    type Output = f32;
    fn div(self, rhs: f32) -> f32 {
        self.as_seconds() / rhs
    }
}
///Converts the time to `f32` seconds before the operation.
impl Div<Time> for f32 {
    type Output = Self;
    fn div(self, rhs: Time) -> Self {
        self / rhs.as_seconds()
    }
}
///A dimensionless quantity stored as an integer. Used almost exclusively for when a time, stored
///as an integer, must be multiplied by a constant factor as in numerical integrals and motion
///profiles.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct DimensionlessInteger(pub i64);
impl DimensionlessInteger {
    ///Constructor for [`DimensionlessInteger`].
    pub const fn new(value: i64) -> Self {
        Self(value)
    }
}
impl From<i64> for DimensionlessInteger {
    fn from(was: i64) -> Self {
        Self(was)
    }
}
impl From<DimensionlessInteger> for i64 {
    fn from(was: DimensionlessInteger) -> Self {
        was.0
    }
}
impl TryFrom<Quantity> for DimensionlessInteger {
    type Error = CannotConvert;
    fn try_from(was: Quantity) -> Result<Self, CannotConvert> {
        if was.unit.eq_assume_true(&DIMENSIONLESS) {
            Ok(Self(was.value as i64))
        } else {
            Err(CannotConvert)
        }
    }
}
impl From<DimensionlessInteger> for Quantity {
    fn from(was: DimensionlessInteger) -> Self {
        Quantity::new(was.0 as f32, DIMENSIONLESS)
    }
}
impl Add for DimensionlessInteger {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
    }
}
impl AddAssign for DimensionlessInteger {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}
impl Sub for DimensionlessInteger {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0)
    }
}
impl SubAssign for DimensionlessInteger {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}
impl Mul for DimensionlessInteger {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self(self.0 * rhs.0)
    }
}
impl MulAssign for DimensionlessInteger {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
    }
}
impl Div for DimensionlessInteger {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        Self(self.0 / rhs.0)
    }
}
impl DivAssign for DimensionlessInteger {
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0;
    }
}
impl Neg for DimensionlessInteger {
    type Output = Self;
    fn neg(self) -> Self {
        Self(-self.0)
    }
}
impl Mul<Time> for DimensionlessInteger {
    type Output = Time;
    fn mul(self, rhs: Time) -> Time {
        Time(self.0 * rhs.0)
    }
}
impl Div<Time> for DimensionlessInteger {
    type Output = Quantity;
    fn div(self, rhs: Time) -> Quantity {
        Quantity::from(self) / Quantity::from(rhs)
    }
}
impl Add<Quantity> for DimensionlessInteger {
    type Output = Quantity;
    fn add(self, rhs: Quantity) -> Quantity {
        Quantity::from(self) + rhs
    }
}
impl Sub<Quantity> for DimensionlessInteger {
    type Output = Quantity;
    fn sub(self, rhs: Quantity) -> Quantity {
        Quantity::from(self) - rhs
    }
}
impl Mul<Quantity> for DimensionlessInteger {
    type Output = Quantity;
    fn mul(self, rhs: Quantity) -> Quantity {
        rhs * self
    }
}
impl Div<Quantity> for DimensionlessInteger {
    type Output = Quantity;
    fn div(self, rhs: Quantity) -> Quantity {
        Quantity::from(self) / Quantity::from(rhs)
    }
}
///A unit of a quantity, like meters per second. Units can be represented as multiplied powers of
///the units that they're derived from, so meters per second squared, or m/s^2, can be m^1*s^-2.
///This struct stores the exponents of each base unit.
#[derive(Clone, Copy, Debug)]
#[cfg_attr(
    any(
        feature = "dim_check_release",
        all(debug_assertions, feature = "dim_check_debug")
    ),
    derive(PartialEq, Eq)
)]
pub struct Unit {
    ///Unit exponent for millimeters.
    #[cfg(any(
        feature = "dim_check_release",
        all(debug_assertions, feature = "dim_check_debug")
    ))]
    millimeter_exp: i8,
    ///Unit exponent for seconds.
    #[cfg(any(
        feature = "dim_check_release",
        all(debug_assertions, feature = "dim_check_debug")
    ))]
    second_exp: i8,
}
impl Unit {
    ///Constructor for `Unit`.
    #[allow(unused)]
    pub const fn new(millimeter_exp: i8, second_exp: i8) -> Self {
        Self {
            #[cfg(any(
                feature = "dim_check_release",
                all(debug_assertions, feature = "dim_check_debug")
            ))]
            millimeter_exp: millimeter_exp,
            #[cfg(any(
                feature = "dim_check_release",
                all(debug_assertions, feature = "dim_check_debug")
            ))]
            second_exp: second_exp,
        }
    }
    ///`foo.const_eq(&bar)` works exactly like `foo == bar` except that it works in a `const`
    ///context. Requires dimension checking to be enabled. Use [`eq_assume_true`](Unit::eq_assume_true) or
    ///[`eq_assume_false`](Unit::eq_assume_false) if you need similar functionality without dimension checking.
    #[cfg(any(
        feature = "dim_check_release",
        all(debug_assertions, feature = "dim_check_debug")
    ))]
    #[allow(unused)]
    pub const fn const_eq(&self, rhs: &Self) -> bool {
        #[cfg(any(
            feature = "dim_check_release",
            all(debug_assertions, feature = "dim_check_debug")
        ))]
        return self.millimeter_exp == rhs.millimeter_exp && self.second_exp == rhs.second_exp;
        #[cfg(not(any(
            feature = "dim_check_release",
            all(debug_assertions, feature = "dim_check_debug")
        )))]
        true
    }
    ///`foo.const_assert_eq(&bar)` works exactly like `assert_eq!(foo, bar)` except that it works
    ///in a `const` context. Requires dimension checking to be enabled. Use
    ///[`assert_eq_assume_ok`](Unit::assert_eq_assume_ok)
    ///or [`assert_eq_assume_not_ok`](Unit::assert_eq_assume_not_ok) if you need similar functionality without
    ///dimension checking.
    #[cfg(any(
        feature = "dim_check_release",
        all(debug_assertions, feature = "dim_check_debug")
    ))]
    pub const fn const_assert_eq(&self, rhs: &Self) {
        assert!(self.const_eq(rhs));
    }
    ///With dimension checking on, behaves exactly like [`const_eq`](Unit::const_eq).
    ///With dimension checking off, always returns true.
    #[allow(unused)]
    pub const fn eq_assume_true(&self, rhs: &Self) -> bool {
        #[cfg(any(
            feature = "dim_check_release",
            all(debug_assertions, feature = "dim_check_debug")
        ))]
        return self.const_eq(rhs);
        #[cfg(not(any(
            feature = "dim_check_release",
            all(debug_assertions, feature = "dim_check_debug")
        )))]
        true
    }
    ///With dimension checking on, behaves exactly like [`const_eq`](Unit::const_eq).
    ///With dimension checking off, always returns false.
    #[allow(unused)]
    pub const fn eq_assume_false(&self, rhs: &Self) -> bool {
        #[cfg(any(
            feature = "dim_check_release",
            all(debug_assertions, feature = "dim_check_debug")
        ))]
        return self.const_eq(rhs);
        #[cfg(not(any(
            feature = "dim_check_release",
            all(debug_assertions, feature = "dim_check_debug")
        )))]
        false
    }
    ///With dimension checking on, behaves exactly like [`const_assert_eq`](Unit::const_assert_eq).
    ///With dimension checking off, never panics.
    pub const fn assert_eq_assume_ok(&self, rhs: &Self) {
        assert!(self.eq_assume_true(rhs))
    }
    ///With dimension checking on, behaves exactly like [`const_assert_eq`](Unit::const_assert_eq).
    ///With dimension checking off, always panics.
    pub const fn assert_eq_assume_not_ok(&self, rhs: &Self) {
        assert!(self.eq_assume_false(rhs))
    }
}
impl From<PositionDerivative> for Unit {
    #[allow(unused)]
    fn from(was: PositionDerivative) -> Self {
        #[cfg(any(
            feature = "dim_check_release",
            all(debug_assertions, feature = "dim_check_debug")
        ))]
        return Self {
            millimeter_exp: 1,
            second_exp: match was {
                PositionDerivative::Position => 0,
                PositionDerivative::Velocity => -1,
                PositionDerivative::Acceleration => -2,
            },
        };
        #[cfg(not(any(
            feature = "dim_check_release",
            all(debug_assertions, feature = "dim_check_debug")
        )))]
        Self {}
    }
}
impl TryFrom<MotionProfilePiece> for Unit {
    type Error = CannotConvert;
    fn try_from(was: MotionProfilePiece) -> Result<Self, CannotConvert> {
        let pos_der: PositionDerivative = was.try_into()?;
        let unit: Self = pos_der.into();
        Ok(unit)
    }
}
///The [`Add`] implementation for [`Unit`] acts like you are trying to add quantities of the unit, not
///like you are trying to actually add the exponents. This should be more useful most of the time,
///but could be somewhat confusing. All this does is [`assert_eq!`] the [`Unit`] with the right-hand
///side and then return it because units should not change when quantities of the same unit are
///added.
///Performing operations on [`Unit`]s should behave exactly the same as performing the same
///operations on [`Quantity`] objects and taking the unit of the resulting [`Quantity`].
impl Add for Unit {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        self.assert_eq_assume_ok(&rhs);
        self
    }
}
impl AddAssign for Unit {
    fn add_assign(&mut self, rhs: Self) {
        self.assert_eq_assume_ok(&rhs);
    }
}
///The [`Sub`] implementation for [`Unit`] acts like you are trying to subtract quantities of the unit,
///not like you are trying to actually subtract the exponents. This should be more useful most of
///the time, but it could be somewhat confusing. All this does is [`assert_eq!`] the [`Unit`] with the
///right-hand side and then return it because units should not change when quantities of the same
///unit are subtracted.
///Performing operations on [`Unit`]s should behave exactly the same as performing the same
///operations on [`Quantity`] objects and taking the unit of the resulting [`Quantity`].
impl Sub for Unit {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        self.assert_eq_assume_ok(&rhs);
        self
    }
}
impl SubAssign for Unit {
    fn sub_assign(&mut self, rhs: Self) {
        self.assert_eq_assume_ok(&rhs);
    }
}
///The [`Mul`] implementation for [`Unit`] acts like you are trying to multiply quantities of the unit,
///not like you are trying to actually multiply the exponents. This should be more useful most of
///the time, but it could be somewhat confusing. This adds the exponents of the left-hand and
///right-hand sides, not multiplies them because that is what should happen when quantities are
///multiplied, not a multiplication of their unit exponents.
///Performing operations on [`Unit`]s should behave exactly the same as performing the same
///operations on [`Quantity`] objects and taking the unit of the resulting [`Quantity`].
impl Mul for Unit {
    type Output = Self;
    #[allow(unused)]
    fn mul(self, rhs: Self) -> Self {
        #[cfg(any(
            feature = "dim_check_release",
            all(debug_assertions, feature = "dim_check_debug")
        ))]
        return Self {
            millimeter_exp: self.millimeter_exp + rhs.millimeter_exp,
            second_exp: self.second_exp + rhs.second_exp,
        };
        #[cfg(not(any(
            feature = "dim_check_release",
            all(debug_assertions, feature = "dim_check_debug")
        )))]
        Self {}
    }
}
impl MulAssign for Unit {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}
///The [`Div`] implementation for [`Unit`] acts like you are trying to divide quantities of the unit,
///not like you are trying to actually divide the exponents. This should be more useful most of the
///time, but it could be somewhat confusing. This subtracts the exponents of the right-hand side
///from the left-hand side's exponents rather than dividing the exponents because that is what
///should happen when quantities are divided, not a division of their unit exponents.
///Performing operations on [`Unit`]s should behave exactly the same as performing the same
///operations on [`Quantity`] objects and taking the unit of the resulting [`Quantity`].
impl Div for Unit {
    type Output = Self;
    #[allow(unused)]
    fn div(self, rhs: Self) -> Self {
        #[cfg(any(
            feature = "dim_check_release",
            all(debug_assertions, feature = "dim_check_debug")
        ))]
        return Self {
            millimeter_exp: self.millimeter_exp - rhs.millimeter_exp,
            second_exp: self.second_exp - rhs.second_exp,
        };
        #[cfg(not(any(
            feature = "dim_check_release",
            all(debug_assertions, feature = "dim_check_debug")
        )))]
        Self {}
    }
}
impl DivAssign for Unit {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}
///The [`Neg`] implementation for [`Unit`] acts like you are trying to negate quantities of the unit,
///not like you are trying to actually negate the exponents. This should be more useful most of the
///time, but could be somewhat confusing. This just returns `self` unchanged because a quantity's
///units don't change when it is negated.
///Performing operations on [`Unit`]s should behave exactly the same as performing the same
///operations on [`Quantity`] objects and taking the unit of the resulting [`Quantity`].
impl Neg for Unit {
    type Output = Self;
    fn neg(self) -> Self {
        self
    }
}
///A quantity with a unit.
#[derive(Clone, Copy, Debug)]
#[cfg_attr(
    any(
        feature = "dim_check_release",
        all(debug_assertions, feature = "dim_check_debug")
    ),
    derive(PartialEq)
)]
pub struct Quantity {
    ///The value.
    pub value: f32,
    ///The unit.
    pub unit: Unit,
}
impl Quantity {
    ///Constructor for [`Quantity`].
    pub const fn new(value: f32, unit: Unit) -> Self {
        Self {
            value: value,
            unit: unit,
        }
    }
    ///Constructor for dimensionless [`Quantity`] objects that does not require a dimension to be
    ///provided.
    pub const fn dimensionless(value: f32) -> Self {
        Self::new(value, DIMENSIONLESS)
    }
    ///Take the absolute value of the quantity.
    #[inline]
    pub fn abs(self) -> Self {
        Self::new(
            #[cfg(feature = "std")]
            self.value.abs(),
            #[cfg(not(feature = "std"))]
            if self.value >= 0.0 {
                self.value
            } else {
                -self.value
            },
            self.unit,
        )
    }
}
impl From<Command> for Quantity {
    fn from(was: Command) -> Self {
        match was {
            Command::Position(pos) => Self::new(pos, MILLIMETER),
            Command::Velocity(vel) => Self::new(vel, MILLIMETER_PER_SECOND),
            Command::Acceleration(acc) => Self::new(acc, MILLIMETER_PER_SECOND_SQUARED),
        }
    }
}
impl From<Quantity> for f32 {
    fn from(was: Quantity) -> f32 {
        was.value
    }
}
impl Add for Quantity {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            value: self.value + rhs.value,
            unit: self.unit + rhs.unit,
        }
    }
}
impl AddAssign for Quantity {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}
impl Sub for Quantity {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            value: self.value - rhs.value,
            unit: self.unit - rhs.unit,
        }
    }
}
impl SubAssign for Quantity {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}
impl Mul for Quantity {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self {
            value: self.value * rhs.value,
            unit: self.unit * rhs.unit,
        }
    }
}
impl MulAssign for Quantity {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}
impl Div for Quantity {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        Self {
            value: self.value / rhs.value,
            unit: self.unit / rhs.unit,
        }
    }
}
impl DivAssign for Quantity {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}
impl Neg for Quantity {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            value: -self.value,
            unit: self.unit,
        }
    }
}
impl Add<Time> for Quantity {
    type Output = Self;
    fn add(self, rhs: Time) -> Self {
        self + Self::from(rhs)
    }
}
impl AddAssign<Time> for Quantity {
    fn add_assign(&mut self, rhs: Time) {
        *self = *self + rhs;
    }
}
impl Sub<Time> for Quantity {
    type Output = Self;
    fn sub(self, rhs: Time) -> Self {
        self - Self::from(rhs)
    }
}
impl SubAssign<Time> for Quantity {
    fn sub_assign(&mut self, rhs: Time) {
        *self = *self - rhs;
    }
}
impl Add<DimensionlessInteger> for Quantity {
    type Output = Self;
    fn add(self, rhs: DimensionlessInteger) -> Self {
        self + Self::from(rhs)
    }
}
impl AddAssign<DimensionlessInteger> for Quantity {
    fn add_assign(&mut self, rhs: DimensionlessInteger) {
        *self = *self + rhs;
    }
}
impl Sub<DimensionlessInteger> for Quantity {
    type Output = Self;
    fn sub(self, rhs: DimensionlessInteger) -> Self {
        self - Self::from(rhs)
    }
}
impl SubAssign<DimensionlessInteger> for Quantity {
    fn sub_assign(&mut self, rhs: DimensionlessInteger) {
        *self = *self - rhs;
    }
}
impl Mul<Time> for Quantity {
    type Output = Self;
    fn mul(self, rhs: Time) -> Self {
        self * Quantity::from(rhs)
    }
}
impl MulAssign<Time> for Quantity {
    fn mul_assign(&mut self, rhs: Time) {
        *self = *self * rhs;
    }
}
impl Div<Time> for Quantity {
    type Output = Self;
    fn div(self, rhs: Time) -> Self {
        self / Quantity::from(rhs)
    }
}
impl DivAssign<Time> for Quantity {
    fn div_assign(&mut self, rhs: Time) {
        *self = *self / rhs;
    }
}
impl Mul<DimensionlessInteger> for Quantity {
    type Output = Self;
    fn mul(self, rhs: DimensionlessInteger) -> Self {
        self * Quantity::from(rhs)
    }
}
impl MulAssign<DimensionlessInteger> for Quantity {
    fn mul_assign(&mut self, rhs: DimensionlessInteger) {
        *self = *self * rhs
    }
}
impl Div<DimensionlessInteger> for Quantity {
    type Output = Self;
    fn div(self, rhs: DimensionlessInteger) -> Self {
        self / Quantity::from(rhs)
    }
}
impl DivAssign<DimensionlessInteger> for Quantity {
    fn div_assign(&mut self, rhs: DimensionlessInteger) {
        *self = *self / rhs
    }
}
#[cfg(not(any(
    feature = "dim_check_release",
    all(debug_assertions, feature = "dim_check_debug")
)))]
impl PartialEq for Quantity {
    fn eq(&self, rhs: &Self) -> bool {
        if self.unit.eq_assume_true(&rhs.unit) {
            self.value == rhs.value
        } else {
            false
        }
    }
}
impl PartialOrd for Quantity {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.unit.assert_eq_assume_ok(&other.unit);
        self.value.partial_cmp(&other.value)
    }
}
impl Half for Quantity {
    fn half(self) -> Self {
        Self::new(self.value / 2.0, self.unit)
    }
}
