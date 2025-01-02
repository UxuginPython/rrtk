use super::*;
///A time in nanoseconds.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Time(pub i64);
impl Time {
    ///The constructor for [`Time`].
    pub const fn new(value: i64) -> Self {
        Self(value)
    }
}
impl From<i64> for Time {
    fn from(was: i64) -> Self {
        Self(was)
    }
}
impl From<Time> for i64 {
    fn from(was: Time) -> i64 {
        was.0
    }
}
//TODO: figure out for to use the Error enum with this
impl TryFrom<Quantity> for Time {
    type Error = ();
    fn try_from(was: Quantity) -> Result<Self, ()> {
        if was.unit.eq_assume_true(&SECOND) {
            Ok(Self((was.value * 1_000_000_000.0) as i64))
        } else {
            Err(())
        }
    }
}
impl From<Time> for Quantity {
    fn from(was: Time) -> Quantity {
        Quantity::new(was.0 as f32 / 1_000_000_000.0, SECOND)
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
impl Mul for Time {
    type Output = Quantity;
    fn mul(self, rhs: Self) -> Quantity {
        Quantity::from(self) * Quantity::from(rhs)
    }
}
impl Div for Time {
    type Output = Quantity;
    fn div(self, rhs: Self) -> Quantity {
        Quantity::from(self) / Quantity::from(rhs)
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
impl Add<Quantity> for Time {
    type Output = Quantity;
    fn add(self, rhs: Quantity) -> Quantity {
        Quantity::from(self) + rhs
    }
}
impl Sub<Quantity> for Time {
    type Output = Quantity;
    fn sub(self, rhs: Quantity) -> Quantity {
        Quantity::from(self) - rhs
    }
}
impl Mul<Quantity> for Time {
    type Output = Quantity;
    fn mul(self, rhs: Quantity) -> Quantity {
        rhs * self
    }
}
impl Div<Quantity> for Time {
    type Output = Quantity;
    fn div(self, rhs: Quantity) -> Quantity {
        Quantity::from(self) / rhs
    }
}
