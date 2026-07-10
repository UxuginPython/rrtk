// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2024-2025 UxuginPython
//!RRTK's compile-time dimensional analysis system. This system is simpler than ones like
//![`uom`](https://crates.io/crates/uom), but it serves a similar purpose: to protect users from
//!dimension mismatch errors at compile time without runtime overhead. This is done through a
//![semi-hack](compile_time_integer) representing integers as types and adding type parameters to a
//!special struct called [`Quantity`], which is a transparent struct holding only a value at
//!runtime.
use super::*;
use compile_time_integer::*;
//This attribute currently cannot be in the actual file with #![] for some reason.
#[rustfmt::skip]
pub mod dimension_aliases;
pub use dimension_aliases::*;
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
    pub const fn from_seconds_f32(value: f32) -> Self {
        Self((value * 1_000_000_000.0) as i64)
    }
    ///Construct a `Time` from [`Quantity`] seconds stored using `f32`.
    pub fn from_seconds(value: Second<f32>) -> Self {
        Self::from_seconds_f32(value.into_inner())
    }
    ///Get the internal `i64` nanoseconds from the `Time`.
    pub const fn as_nanoseconds(self) -> i64 {
        self.0
    }
    ///Get the value of the `Time` as `f32` seconds.
    pub const fn as_seconds_f32(self) -> f32 {
        (self.0 as f32) / 1_000_000_000.0
    }
    ///Get the value of the `Time` as [`Quantity`] seconds stored using `f32`.
    ///Effectively a wrapper for [`as_seconds`](Self::as_seconds).
    pub const fn as_seconds(self) -> Second<f32> {
        Second::new(self.as_seconds_f32())
    }
}
impl From<Second<f32>> for Time {
    fn from(was: Second<f32>) -> Self {
        Self::from_seconds(was)
    }
}
impl From<Time> for Second<f32> {
    fn from(was: Time) -> Self {
        was.as_seconds()
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
//XXX: wut
///Converts the time to `f32` seconds before the operation.
impl Mul<f32> for Time {
    type Output = f32;
    fn mul(self, rhs: f32) -> f32 {
        self.as_seconds_f32() * rhs
    }
}
///Converts the time to `f32` seconds before the operation.
impl Mul<Time> for f32 {
    type Output = Self;
    fn mul(self, rhs: Time) -> Self {
        self * rhs.as_seconds_f32()
    }
}
///Converts the time to `f32` seconds before the operation.
impl Div<f32> for Time {
    type Output = f32;
    fn div(self, rhs: f32) -> f32 {
        self.as_seconds_f32() / rhs
    }
}
///Converts the time to `f32` seconds before the operation.
impl Div<Time> for f32 {
    type Output = Self;
    fn div(self, rhs: Time) -> Self {
        self / rhs.as_seconds_f32()
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
    #[inline]
    pub const fn new(value: i64) -> Self {
        Self(value)
    }
    ///`x.const_eq(y)` is exactly equivalent to `x == y` except that it works in const contexts.
    #[inline]
    pub const fn const_eq(&self, rhs: &Self) -> bool {
        self.0 == rhs.0
    }
    ///Checks if the integer is zero.
    #[inline]
    pub const fn is_zero(&self) -> bool {
        self.0 == 0
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
    type Output = DimensionlessFraction;
    fn div(self, rhs: Self) -> DimensionlessFraction {
        assert_ne!(rhs, Self::new(0));
        DimensionlessFraction(self, rhs)
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
///Alias for `DimensionlessFraction::new(DimensionlessInteger($num), DimensionlessInteger($denom))`.
///This means that you can, for example, construct a [`DimensionlessFraction`] of 2/3 with
///`dimensionless_fraction!(2, 3)`.
#[macro_export]
macro_rules! dimensionless_fraction {
    ($num: expr, $denom: expr) => {
        DimensionlessFraction::new(DimensionlessInteger($num), DimensionlessInteger($denom))
    };
}
pub use dimensionless_fraction;
///Alias for `DimensionlessFraction::new_unchecked(DimensionlessInteger($num), DimensionlessInteger($denom))`.
///This means that you can, for example, construct a [`DimensionlessFraction`] of 2/3 with
///`dimensionless_fraction_unchecked!(2, 3)`. The difference between this and the
///[`dimensionless_fraction`] macro is that this one does not check if the denominator is zero.
///E.g., it is possible to construct 1/0 with this macro but not with `dimensionless_fraction!`.
#[macro_export]
macro_rules! dimensionless_fraction_unchecked {
    ($num: expr, $denom: expr) => {
        DimensionlessFraction::new_unchecked(
            DimensionlessInteger($num),
            DimensionlessInteger($denom),
        )
    };
}
pub use dimensionless_fraction_unchecked;
///An exact rational number type for dimensionless quantities. Used almost exclusively when a
///[`Time`] must be multiplied by a constant fractional factor.
#[derive(Clone, Copy, Debug)]
pub struct DimensionlessFraction(DimensionlessInteger, DimensionlessInteger);
impl DimensionlessFraction {
    ///Ensures that the denominator is not zero.
    #[inline]
    pub const fn is_valid(&self) -> bool {
        !self.1.is_zero()
    }
    ///Constructor that verifies that the denominator is not zero and panics if it is.
    #[inline]
    pub const fn new(num: DimensionlessInteger, denom: DimensionlessInteger) -> Self {
        let new = Self(num, denom);
        if new.is_valid() {
            new
        } else {
            panic!("attempted to construct a DimensionlessFraction with a zero denominator");
        }
    }
    ///Constructor that does not check if the denominator is zero.
    #[inline]
    pub const fn new_unchecked(num: DimensionlessInteger, denom: DimensionlessInteger) -> Self {
        Self(num, denom)
    }
    ///Reciprocal function (1/x) that panics if the new denominator is zero.
    #[inline]
    pub const fn reciprocal(&self) -> Self {
        Self::new(self.1, self.0)
    }
    ///Reciprocal function (1/x) that does not check if the new denominator is zero.
    #[inline]
    pub const fn reciprocal_unchecked(&self) -> Self {
        Self(self.1, self.0)
    }
    ///Converts the fraction into a tuple `(numerator, denominator)`.
    ///
    ///The following code is guaranteed to leave mutable `DimensionlessInteger` variables `x` and
    ///`y` with the same values that they had before the code was run.
    ///```
    ///# use rrtk::{DimensionlessFraction, DimensionlessInteger};
    ///# let mut x = DimensionlessInteger(2);
    ///# let mut y = DimensionlessInteger(3);
    ///let frac = DimensionlessFraction::new_unchecked(x, y);
    ///(x, y) = frac.into_components();
    ///# assert_eq!(x.0, 2);
    ///# assert_eq!(y.0, 3);
    ///```
    #[inline]
    pub const fn into_components(self) -> (DimensionlessInteger, DimensionlessInteger) {
        (self.0, self.1)
    }
    ///Converts the fraction to its closest `f32` approximation.
    ///There is also a [`From`] implementation that does this.
    #[inline]
    pub const fn as_f32(&self) -> f32 {
        self.0.0 as f32 / self.1.0 as f32
    }
    ///Converts the fraction to its closest `f64` approximation.
    ///There is also a [`From`] implementation that does this.
    #[inline]
    pub const fn as_f64(&self) -> f64 {
        self.0.0 as f64 / self.1.0 as f64
    }
    ///Wraps the output of [`as_f32`](Self::as_f32) in a `Dimensionless` wrapper.
    ///There is also a [`From`] implementation that does this.
    #[inline]
    pub const fn as_quantity_f32(&self) -> Dimensionless<f32> {
        Dimensionless::new(self.as_f32())
    }
    ///Wraps the output of [`as_f64`](Self::as_f64) in a `Dimensionless` wrapper.
    ///There is also a [`From`] implementation that does this.
    #[inline]
    pub const fn as_quantity_f64(&self) -> Dimensionless<f64> {
        Dimensionless::new(self.as_f64())
    }
}
impl From<DimensionlessInteger> for DimensionlessFraction {
    fn from(was: DimensionlessInteger) -> Self {
        Self(was, DimensionlessInteger::new(1))
    }
}
impl Ord for DimensionlessFraction {
    fn cmp(&self, rhs: &Self) -> core::cmp::Ordering {
        let a = self.0 * rhs.1;
        let b = self.1 * rhs.0;
        a.cmp(&b)
    }
}
impl PartialEq for DimensionlessFraction {
    fn eq(&self, rhs: &Self) -> bool {
        self.cmp(rhs) == core::cmp::Ordering::Equal
    }
}
impl Eq for DimensionlessFraction {}
impl PartialOrd for DimensionlessFraction {
    fn partial_cmp(&self, rhs: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(rhs))
    }
}
impl Neg for DimensionlessFraction {
    type Output = Self;
    fn neg(self) -> Self {
        Self(-self.0, self.1)
    }
}
impl Mul for DimensionlessFraction {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self(self.0 * rhs.0, self.1 * rhs.1)
    }
}
impl MulAssign for DimensionlessFraction {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}
impl Div for DimensionlessFraction {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        self * rhs.reciprocal()
    }
}
impl DivAssign for DimensionlessFraction {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}
impl Add for DimensionlessFraction {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self(self.0 * rhs.1 + rhs.0 * self.1, self.1 * rhs.1)
    }
}
impl AddAssign for DimensionlessFraction {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}
impl Sub for DimensionlessFraction {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        self + -rhs
    }
}
impl SubAssign for DimensionlessFraction {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}
impl Mul<DimensionlessInteger> for DimensionlessFraction {
    type Output = Self;
    fn mul(self, rhs: DimensionlessInteger) -> Self {
        Self(self.0 * rhs, self.1)
    }
}
impl MulAssign<DimensionlessInteger> for DimensionlessFraction {
    fn mul_assign(&mut self, rhs: DimensionlessInteger) {
        *self = *self * rhs;
    }
}
impl Div<DimensionlessInteger> for DimensionlessFraction {
    type Output = Self;
    fn div(self, rhs: DimensionlessInteger) -> Self {
        Self(self.0, self.1 * rhs)
    }
}
impl DivAssign<DimensionlessInteger> for DimensionlessFraction {
    fn div_assign(&mut self, rhs: DimensionlessInteger) {
        *self = *self / rhs;
    }
}
impl Add<DimensionlessInteger> for DimensionlessFraction {
    type Output = Self;
    fn add(self, rhs: DimensionlessInteger) -> Self {
        self + Self::from(rhs)
    }
}
impl AddAssign<DimensionlessInteger> for DimensionlessFraction {
    fn add_assign(&mut self, rhs: DimensionlessInteger) {
        *self = *self + rhs;
    }
}
impl Sub<DimensionlessInteger> for DimensionlessFraction {
    type Output = Self;
    fn sub(self, rhs: DimensionlessInteger) -> Self {
        self + Self::from(-rhs)
    }
}
impl SubAssign<DimensionlessInteger> for DimensionlessFraction {
    fn sub_assign(&mut self, rhs: DimensionlessInteger) {
        *self = *self - rhs;
    }
}
impl Mul<Time> for DimensionlessFraction {
    type Output = Time;
    fn mul(self, rhs: Time) -> Time {
        rhs * self.0 / self.1
    }
}
impl Mul<DimensionlessFraction> for DimensionlessInteger {
    type Output = DimensionlessFraction;
    fn mul(self, rhs: DimensionlessFraction) -> DimensionlessFraction {
        rhs * self
    }
}
impl Div<DimensionlessFraction> for DimensionlessInteger {
    type Output = DimensionlessFraction;
    fn div(self, rhs: DimensionlessFraction) -> DimensionlessFraction {
        self * rhs.reciprocal()
    }
}
impl Mul<DimensionlessFraction> for Time {
    type Output = Self;
    fn mul(self, rhs: DimensionlessFraction) -> Self {
        rhs * self
    }
}
impl MulAssign<DimensionlessFraction> for Time {
    fn mul_assign(&mut self, rhs: DimensionlessFraction) {
        *self = *self * rhs;
    }
}
impl Div<DimensionlessFraction> for Time {
    type Output = Self;
    fn div(self, rhs: DimensionlessFraction) -> Self {
        self * rhs.reciprocal()
    }
}
impl DivAssign<DimensionlessFraction> for Time {
    fn div_assign(&mut self, rhs: DimensionlessFraction) {
        *self = *self / rhs;
    }
}
impl Add<DimensionlessFraction> for DimensionlessInteger {
    type Output = DimensionlessFraction;
    fn add(self, rhs: DimensionlessFraction) -> DimensionlessFraction {
        rhs + self
    }
}
impl Sub<DimensionlessFraction> for DimensionlessInteger {
    type Output = DimensionlessFraction;
    fn sub(self, rhs: DimensionlessFraction) -> DimensionlessFraction {
        DimensionlessFraction::from(self) - rhs
    }
}
///This conversion is not lossless.
impl From<DimensionlessFraction> for f32 {
    fn from(was: DimensionlessFraction) -> Self {
        was.as_f32()
    }
}
///This conversion is not lossless.
impl From<DimensionlessFraction> for f64 {
    fn from(was: DimensionlessFraction) -> Self {
        was.as_f64()
    }
}
///This conversion is not lossless.
impl From<DimensionlessFraction> for Dimensionless<f32> {
    fn from(was: DimensionlessFraction) -> Self {
        was.as_quantity_f32()
    }
}
///This conversion is not lossless.
impl From<DimensionlessFraction> for Dimensionless<f64> {
    fn from(was: DimensionlessFraction) -> Self {
        was.as_quantity_f64()
    }
}
///Gets the resulting type from multiplying quantities of two types. Basically an alias for
///`<$a as Mul<$b>>::Output`. This is an important thing to be able to do when writing code that is
///generic over units as, since quantities of different units are technically different types, the
///fully qualified syntax gets unwieldy quickly when performing multiplication and division.
///Rust's scoping rules for macros is a bit odd, but you should be able to use `rrtk::mul!` and
///`rrtk::compile_time_dimensions::mul!` interchangably.
#[macro_export]
macro_rules! mul {
    ($a: ty, $b: ty) => {
        <$a as Mul<$b>>::Output
    };
}
pub use mul;
///Gets the resulting type from dividing quantities of two types. Basically an alias for
///`<$a as Div<$b>>::Output`. This is an important thing to be able to do when writing code that is
///generic over units as, since quantities of different units are technically different types, the
///fully qualified syntax gets unwieldy quickly when performing multiplication and division.
///Rust's scoping rules for macros is a bit odd, but you should be able to use `rrtk::div!` and
///`rrtk::compile_time_dimensions::div!` interchangably.
#[macro_export]
macro_rules! div {
    ($a: ty, $b: ty) => {
        <$a as Div<$b>>::Output
    };
}
pub use div;
///A quantity with a unit. Dimensional analysis is performed at compile time through the type
///parameters' representations of unit exponents.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Quantity<T, MM: Integer, S: Integer>(PhantomData<MM>, PhantomData<S>, pub(crate) T);
impl<T, MM: Integer, S: Integer> Quantity<T, MM, S> {
    ///Constructor for `Quantity`.
    #[inline]
    pub const fn new(inner: T) -> Self {
        Self(PhantomData, PhantomData, inner)
    }
    ///Converts the `Quantity` into its inner contained object, consuming it.
    #[inline]
    pub fn into_inner(self) -> T {
        self.2
    }
}
macro_rules! impl_quantity_abs {
    ($t: ty) => {
        impl<MM: Integer, S: Integer> Quantity<$t, MM, S> {
            ///Evaluate the absolute value of the quantity.
            pub const fn abs(self) -> Self {
                Self(PhantomData, PhantomData, self.2.abs())
            }
        }
    };
}
impl_quantity_abs!(f32);
impl_quantity_abs!(f64);
impl_quantity_abs!(i8);
impl_quantity_abs!(i16);
impl_quantity_abs!(i32);
impl_quantity_abs!(i64);
impl_quantity_abs!(i128);
impl_quantity_abs!(isize);
impl<T, MM: Integer, S: Integer> From<T> for Quantity<T, MM, S> {
    fn from(was: T) -> Self {
        Self(PhantomData, PhantomData, was)
    }
}
macro_rules! impl_const_ops {
    ($t: ty) => {
        impl<MM: Integer, S: Integer> Quantity<$t, MM, S> {
            ///Exactly like `+` except that it works in a const context.
            #[inline]
            pub const fn add_const(self, rhs: Self) -> Self {
                Quantity::new(self.2 + rhs.2)
            }
            ///Exactly like `+` except that it works in a const context.
            #[inline]
            pub const fn sub_const(self, rhs: Self) -> Self {
                Quantity::new(self.2 - rhs.2)
            }
        }
    };
}
impl_const_ops!(f32);
impl_const_ops!(f64);
impl_const_ops!(u8);
impl_const_ops!(u16);
impl_const_ops!(u32);
impl_const_ops!(u64);
impl_const_ops!(u128);
impl_const_ops!(usize);
impl_const_ops!(i8);
impl_const_ops!(i16);
impl_const_ops!(i32);
impl_const_ops!(i64);
impl_const_ops!(i128);
impl_const_ops!(isize);
//FIXME: E0210
/*impl<T, MM: Integer, S: Integer> From<Quantity<T, MM, S>> for T {
    fn from(was: Quantity<T, MM, S>) -> T {
        was.2
    }
}*/
//or, if you can't, FIXME instead: E0119
/*impl<T, MM: Integer, S: Integer> Into<T> for Quantity<T, MM, S> {
    fn into(self) -> T {
        self.2
    }
}*/
impl<T: Neg<Output = O>, O, MM: Integer, S: Integer> Neg for Quantity<T, MM, S> {
    type Output = Quantity<O, MM, S>;
    fn neg(self) -> Quantity<O, MM, S> {
        Quantity::new(-self.2)
    }
}
impl<T: Add<U, Output = O>, U, O, MM: Integer, S: Integer> Add<Quantity<U, MM, S>>
    for Quantity<T, MM, S>
{
    type Output = Quantity<O, MM, S>;
    fn add(self, rhs: Quantity<U, MM, S>) -> Quantity<O, MM, S> {
        Quantity::from(self.2 + rhs.2)
    }
}
impl<T: AddAssign<U>, U, MM: Integer, S: Integer> AddAssign<Quantity<U, MM, S>>
    for Quantity<T, MM, S>
{
    fn add_assign(&mut self, rhs: Quantity<U, MM, S>) {
        self.2 += rhs.2;
    }
}
impl<T: Sub<U, Output = O>, U, O, MM: Integer, S: Integer> Sub<Quantity<U, MM, S>>
    for Quantity<T, MM, S>
{
    type Output = Quantity<O, MM, S>;
    fn sub(self, rhs: Quantity<U, MM, S>) -> Quantity<O, MM, S> {
        Quantity::from(self.2 - rhs.2)
    }
}
impl<T: SubAssign<U>, U, MM: Integer, S: Integer> SubAssign<Quantity<U, MM, S>>
    for Quantity<T, MM, S>
{
    fn sub_assign(&mut self, rhs: Quantity<U, MM, S>) {
        self.2 -= rhs.2;
    }
}
impl<T: Mul<U, Output = O>, U, O, MM1: Integer, S1: Integer, MM2: Integer, S2: Integer>
    Mul<Quantity<U, MM2, S2>> for Quantity<T, MM1, S1>
{
    type Output = Quantity<O, MM1::Plus<MM2>, S1::Plus<S2>>;
    fn mul(self, rhs: Quantity<U, MM2, S2>) -> Quantity<O, MM1::Plus<MM2>, S1::Plus<S2>> {
        Quantity::from(self.2 * rhs.2)
    }
}
impl<T: MulAssign<U>, U, MM: Integer, S: Integer> MulAssign<Dimensionless<U>>
    for Quantity<T, MM, S>
{
    fn mul_assign(&mut self, rhs: Dimensionless<U>) {
        self.2 *= rhs.2;
    }
}
impl<T: Div<U, Output = O>, U, O, MM1: Integer, S1: Integer, MM2: Integer, S2: Integer>
    Div<Quantity<U, MM2, S2>> for Quantity<T, MM1, S1>
{
    type Output = Quantity<O, MM1::Minus<MM2>, S1::Minus<S2>>;
    fn div(self, rhs: Quantity<U, MM2, S2>) -> Quantity<O, MM1::Minus<MM2>, S1::Minus<S2>> {
        Quantity::from(self.2 / rhs.2)
    }
}
impl<T: DivAssign<U>, U, MM: Integer, S: Integer> DivAssign<Dimensionless<U>>
    for Quantity<T, MM, S>
{
    fn div_assign(&mut self, rhs: Dimensionless<U>) {
        self.2 /= rhs.2;
    }
}
impl<MM: Integer, S: Integer> Mul<Time> for Quantity<f32, MM, S>
where
    //MM + 0 = MM
    MM: Integer<Plus<Zero> = MM>,
{
    type Output = Quantity<f32, MM, S::Plus<OnePlus<Zero>>>;
    fn mul(self, rhs: Time) -> Quantity<f32, MM, S::Plus<OnePlus<Zero>>> {
        self * rhs.as_seconds()
    }
}
impl<MM: Integer, S: Integer> Mul<Quantity<f32, MM, S>> for Time {
    type Output = Quantity<f32, MM, S::PlusOne>;
    fn mul(self, rhs: Quantity<f32, MM, S>) -> Quantity<f32, MM, S::PlusOne> {
        self.as_seconds() * rhs
    }
}
impl<MM: Integer, S: Integer> Div<Time> for Quantity<f32, MM, S>
where
    //MM - 0 = MM
    MM: Integer<Minus<Zero> = MM>,
{
    type Output = Quantity<f32, MM, S::Minus<OnePlus<Zero>>>;
    fn div(self, rhs: Time) -> Quantity<f32, MM, S::Minus<OnePlus<Zero>>> {
        self / rhs.as_seconds()
    }
}
impl<MM: Integer, S: Integer> Div<Quantity<f32, MM, S>> for Time
where
    MM: Integer<Negative = MM>,
{
    //S - 1 = -S + 1
    type Output = Quantity<f32, MM, <<S as Integer>::Negative as Integer>::PlusOne>;
    fn div(
        self,
        rhs: Quantity<f32, MM, S>,
    ) -> Quantity<f32, MM, <<S as Integer>::Negative as Integer>::PlusOne> {
        self.as_seconds() / rhs
    }
}
impl<T: fmt::Display, MM: Integer, S: Integer> fmt::Display for Quantity<T, MM, S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} mm^{}s^{}", self.2, MM::as_i8(), S::as_i8())
    }
}
impl<T: stulta::Half, MM: Integer, S: Integer> stulta::Half for Quantity<T, MM, S> {
    fn half(self) -> Self {
        Self::new(self.2.half())
    }
}
impl<T: stulta::AbsoluteValue, MM: Integer, S: Integer> stulta::AbsoluteValue
    for Quantity<T, MM, S>
{
    fn rrtk_abs(self) -> Self {
        Self::new(self.2.rrtk_abs())
    }
}
