use super::*;
pub trait Num {
    type PlusOne: Num;
    type MinusOne: Num;
    type Negative: Num;
    type Plus<T: Num>: Num;
    type Minus<T: Num>: Num;
    fn new() -> Self;
}
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct Zero;
impl Num for Zero {
    type PlusOne = OnePlus<Self>;
    type MinusOne = NegativeOnePlus<Self>;
    type Negative = Self;
    type Plus<T: Num> = T;
    type Minus<T: Num> = Self::Plus<T::Negative>;
    fn new() -> Self {
        Self
    }
}
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct OnePlus<T: Num>(T);
impl<T: Num> Num for OnePlus<T> {
    type PlusOne = OnePlus<Self>;
    type MinusOne = T;
    type Negative = NegativeOnePlus<T::Negative>;
    type Plus<A: Num> = T::Plus<A::PlusOne>;
    type Minus<S: Num> = Self::Plus<S::Negative>;
    fn new() -> Self {
        Self(T::new())
    }
}
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct NegativeOnePlus<T: Num>(T);
impl<T: Num> Num for NegativeOnePlus<T> {
    type PlusOne = T;
    type MinusOne = NegativeOnePlus<Self>;
    type Negative = OnePlus<T::Negative>;
    type Plus<A: Num> = T::Plus<A::MinusOne>;
    type Minus<S: Num> = Self::Plus<S::Negative>;
    fn new() -> Self {
        Self(T::new())
    }
}
