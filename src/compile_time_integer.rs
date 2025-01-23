use super::*;
pub trait Integer: fmt::Display + Clone + Copy + Debug {
    type PlusOne: Integer;
    type MinusOne: Integer;
    type Negative: Integer;
    type Plus<T: Integer>: Integer;
    type Minus<T: Integer>: Integer;
    fn new() -> Self;
    fn as_i8() -> i8;
}
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
