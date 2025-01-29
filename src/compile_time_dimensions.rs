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
pub struct Quantity<MM: Integer, S: Integer>(PhantomData<MM>, PhantomData<S>, f32);
impl<MM: Integer, S: Integer> From<f32> for Quantity<MM, S> {
    fn from(was: f32) -> Self {
        Self(PhantomData, PhantomData, was)
    }
}
impl<MM: Integer, S: Integer> From<Quantity<MM, S>> for f32 {
    fn from(was: Quantity<MM, S>) -> f32 {
        was.2
    }
}
impl<MM: Integer, S: Integer> Add for Quantity<MM, S> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self::from(self.2 + rhs.2)
    }
}
impl<MM: Integer, S: Integer> Sub for Quantity<MM, S> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self::from(self.2 - rhs.2)
    }
}
impl<MM1: Integer, S1: Integer, MM2: Integer, S2: Integer> Mul<Quantity<MM2, S2>>
    for Quantity<MM1, S1>
{
    type Output = Quantity<MM1::Plus<MM2>, S1::Plus<S2>>;
    fn mul(self, rhs: Quantity<MM2, S2>) -> Quantity<MM1::Plus<MM2>, S1::Plus<S2>> {
        Quantity::from(self.2 * rhs.2)
    }
}
impl<MM1: Integer, S1: Integer, MM2: Integer, S2: Integer> Div<Quantity<MM2, S2>>
    for Quantity<MM1, S1>
{
    type Output = Quantity<MM1::Minus<MM2>, S1::Minus<S2>>;
    fn div(self, rhs: Quantity<MM2, S2>) -> Quantity<MM1::Minus<MM2>, S1::Minus<S2>> {
        Quantity::from(self.2 / rhs.2)
    }
}
impl<MM: Integer, S: Integer> fmt::Display for Quantity<MM, S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} mm^{}s^{}", self.2, MM::as_i8(), S::as_i8())
    }
}
