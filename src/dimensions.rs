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
pub struct Quantity<T, MM: Integer, S: Integer>(PhantomData<MM>, PhantomData<S>, T);
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
impl<T: Half, MM: Integer, S: Integer> Half for Quantity<T, MM, S> {
    fn half(self) -> Self {
        Self::new(self.2.half())
    }
}
