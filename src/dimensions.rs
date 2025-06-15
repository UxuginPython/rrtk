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
    pub fn from_compile_time_quantity(value: Second<f32>) -> Self {
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
    pub const fn as_compile_time_quantity(self) -> Second<f32> {
        Second::new(self.as_seconds())
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
