use super::*;
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
impl TryFrom<Quantity> for DimensionlessInteger {
    type Error = ();
    fn try_from(was: Quantity) -> Result<Self, ()> {
        if was.unit.eq_assume_true(&DIMENSIONLESS) {
            Ok(Self(was.value as i64))
        } else {
            Err(())
        }
    }
}
impl From<DimensionlessInteger> for Quantity {
    fn from(was: DimensionlessInteger) -> Self {
        Quantity::new(was.0 as f32, DIMENSIONLESS)
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
impl Div<Time> for DimensionlessInteger {
    type Output = Quantity;
    fn div(self, rhs: Time) -> Quantity {
        Quantity::from(self) / Quantity::from(rhs)
    }
}
impl Add<Quantity> for DimensionlessInteger {
    type Output = Quantity;
    fn add(self, rhs: Quantity) -> Quantity {
        Quantity::from(self) + rhs
    }
}
impl Sub<Quantity> for DimensionlessInteger {
    type Output = Quantity;
    fn sub(self, rhs: Quantity) -> Quantity {
        Quantity::from(self) - rhs
    }
}
impl Mul<Quantity> for DimensionlessInteger {
    type Output = Quantity;
    fn mul(self, rhs: Quantity) -> Quantity {
        rhs * self
    }
}
impl Div<Quantity> for DimensionlessInteger {
    type Output = Quantity;
    fn div(self, rhs: Quantity) -> Quantity {
        Quantity::from(self) / Quantity::from(rhs)
    }
}
