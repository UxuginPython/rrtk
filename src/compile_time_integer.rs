//!RRTK's compile-time integer system. This is basically a simpler version of
//![Typenum](https://crates.io/crates/typenum/). It is used for compile-time dimensional analysis.
//!0 is represented by the [`Zero`] struct. Positive integers are created by wrapping [`Zero`] with
//!with [`OnePlus`] a given number of times, e.g., 2 is represented by `OnePlus<OnePlus<Zero>>`.
//!Negative numbers are created similarly but with [`NegativeOnePlus`] instead of [`OnePlus`]: -2
//!is `NegativeOnePlus<NegativeOnePlus<Zero>>`. You should be able to read a number's type
//!left-to-right and get a mathematical expression that will evaluate to the number. Using both [`OnePlus`] and
//![`NegativeOnePlus`] in the same number is discouraged, e.g., using
//!`NegativeOnePlus<OnePlus<Zero>>` for 0.
use super::*;
///A trait used for defining numbers in RRTK's compile-time integer system based on operations on
///them. You should probably not implement this yourself; instead, use the [provided
///types](super::compile_time_integer) for constructing compile-time integers.
pub trait Integer: Copy + Debug + fmt::Display {
    ///The type representing **n + 1** where **n** is the implementor's value.
    type PlusOne: Integer;
    ///The type representing **n - 1** where **n** is the implementor's value.
    type MinusOne: Integer;
    ///The type representing **-n** where **n** is the implementor's value.
    type Negative: Integer;
    ///The type representing **n + t** where **n** is the implementor's value and **t** is `T`'s
    ///value.
    type Plus<T: Integer>: Integer;
    ///The type representing **n - t** where **n** is the implementor's value and **t** is `T`'s
    ///value.
    type Minus<T: Integer>: Integer;
    ///Create an instance of the number object. This should be zero-sized unless you are
    ///implementing the trait yourself for some reason. There's really no reason you should need an
    ///instance of any compile-time number, and it does not give you any additional functionality
    ///over just having the type in scope. The only potential use case might be something with
    ///[`core::any`].
    fn new() -> Self;
    ///Create an [`i8`] with the same value as the number. Note that this is an associated function
    ///and not a method. You do not need an instance of the number for any of its functionality. As
    ///with anything involving [`i8`], watch out for overflows.
    fn as_i8() -> i8;
}
///Type representing zero in RRTK's compile-time integer system.
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct Zero;
impl Integer for Zero {
    type PlusOne = OnePlus<Self>;
    type MinusOne = NegativeOnePlus<Self>;
    type Negative = Self;
    type Plus<T: Integer> = T;
    type Minus<T: Integer> = Self::Plus<T::Negative>;
    fn new() -> Self {
        Self
    }
    fn as_i8() -> i8 {
        0
    }
}
//For some reason this cannot be a blanket impl. All the fmt::Display impls in this module are identical.
impl fmt::Display for Zero {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Self::as_i8())
    }
}
///Type representing something added to one in RRTK's compile-time integer system.
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct OnePlus<T: Integer>(T);
impl<T: Integer> Integer for OnePlus<T> {
    type PlusOne = OnePlus<Self>;
    type MinusOne = T;
    type Negative = NegativeOnePlus<T::Negative>;
    type Plus<A: Integer> = T::Plus<A::PlusOne>;
    type Minus<S: Integer> = Self::Plus<S::Negative>;
    fn new() -> Self {
        Self(T::new())
    }
    fn as_i8() -> i8 {
        1 + T::as_i8()
    }
}
impl<T: Integer> fmt::Display for OnePlus<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Self::as_i8())
    }
}
///Type representing something added to negative one, or one subtracted from something, in RRTK's
///compile-time integer system.
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct NegativeOnePlus<T: Integer>(T);
impl<T: Integer> Integer for NegativeOnePlus<T> {
    type PlusOne = T;
    type MinusOne = NegativeOnePlus<Self>;
    type Negative = OnePlus<T::Negative>;
    type Plus<A: Integer> = T::Plus<A::MinusOne>;
    type Minus<S: Integer> = Self::Plus<S::Negative>;
    fn new() -> Self {
        Self(T::new())
    }
    fn as_i8() -> i8 {
        -1 + T::as_i8()
    }
}
impl<T: Integer> fmt::Display for NegativeOnePlus<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Self::as_i8())
    }
}
