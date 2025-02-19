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
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Quantity<T, MM: Integer, S: Integer>(PhantomData<MM>, PhantomData<S>, T);
impl<T, MM: Integer, S: Integer> Quantity<T, MM, S> {
    ///Constructor for `Quantity`.
    pub const fn new(inner: T) -> Self {
        Self(PhantomData, PhantomData, inner)
    }
    ///Converts the `Quantity` into its inner contained object, consuming it.
    pub fn into_inner(self) -> T {
        self.2
    }
}
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
impl<T: Add<U, Output = O>, U, O, MM: Integer, S: Integer> Add<Quantity<U, MM, S>>
    for Quantity<T, MM, S>
{
    type Output = Quantity<O, MM, S>;
    fn add(self, rhs: Quantity<U, MM, S>) -> Quantity<O, MM, S> {
        Quantity::from(self.2 + rhs.2)
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
impl<T: Mul<U, Output = O>, U, O, MM1: Integer, S1: Integer, MM2: Integer, S2: Integer>
    Mul<Quantity<U, MM2, S2>> for Quantity<T, MM1, S1>
{
    type Output = Quantity<O, MM1::Plus<MM2>, S1::Plus<S2>>;
    fn mul(self, rhs: Quantity<U, MM2, S2>) -> Quantity<O, MM1::Plus<MM2>, S1::Plus<S2>> {
        Quantity::from(self.2 * rhs.2)
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
