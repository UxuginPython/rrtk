use super::*;
pub trait Integer {
    type PlusOne: Integer;
    type MinusOne: Integer;
    type Negative: Integer;
    type Plus<T: Integer>: Integer;
    type Minus<T: Integer>: Integer;
    fn new() -> Self;
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
}
