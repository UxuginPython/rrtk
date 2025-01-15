use super::*;
use compile_time_integer::*;
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
