use super::*;
///A quantity with a unit.
#[derive(Clone, Copy, Debug)]
#[cfg_attr(
    any(
        feature = "dim_check_release",
        all(debug_assertions, feature = "dim_check_debug")
    ),
    derive(PartialEq)
)]
pub struct Quantity {
    ///The value.
    pub value: f32,
    ///The unit.
    pub unit: Unit,
}
impl Quantity {
    ///Constructor for [`Quantity`].
    pub const fn new(value: f32, unit: Unit) -> Self {
        Self {
            value: value,
            unit: unit,
        }
    }
    ///Constructor for dimensionless [`Quantity`] objects that does not require a dimension to be
    ///provided.
    pub const fn dimensionless(value: f32) -> Self {
        Self::new(value, DIMENSIONLESS)
    }
    ///Take the absolute value of the quantity.
    #[inline]
    pub fn abs(self) -> Self {
        Self::new(
            #[cfg(feature = "std")]
            self.value.abs(),
            #[cfg(not(feature = "std"))]
            if self.value >= 0.0 {
                self.value
            } else {
                -self.value
            },
            self.unit,
        )
    }
}
impl From<Command> for Quantity {
    fn from(was: Command) -> Self {
        match was {
            Command::Position(pos) => Self::new(pos, MILLIMETER),
            Command::Velocity(vel) => Self::new(vel, MILLIMETER_PER_SECOND),
            Command::Acceleration(acc) => Self::new(acc, MILLIMETER_PER_SECOND_SQUARED),
        }
    }
}
impl From<Quantity> for f32 {
    fn from(was: Quantity) -> f32 {
        was.value
    }
}
impl Add for Quantity {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            value: self.value + rhs.value,
            unit: self.unit + rhs.unit,
        }
    }
}
impl AddAssign for Quantity {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}
impl Sub for Quantity {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            value: self.value - rhs.value,
            unit: self.unit - rhs.unit,
        }
    }
}
impl SubAssign for Quantity {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}
impl Mul for Quantity {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self {
            value: self.value * rhs.value,
            unit: self.unit * rhs.unit,
        }
    }
}
impl MulAssign for Quantity {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}
impl Div for Quantity {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        Self {
            value: self.value / rhs.value,
            unit: self.unit / rhs.unit,
        }
    }
}
impl DivAssign for Quantity {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}
impl Neg for Quantity {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            value: -self.value,
            unit: self.unit,
        }
    }
}
impl Add<Time> for Quantity {
    type Output = Self;
    fn add(self, rhs: Time) -> Self {
        self + Self::from(rhs)
    }
}
impl AddAssign<Time> for Quantity {
    fn add_assign(&mut self, rhs: Time) {
        *self = *self + rhs;
    }
}
impl Sub<Time> for Quantity {
    type Output = Self;
    fn sub(self, rhs: Time) -> Self {
        self - Self::from(rhs)
    }
}
impl SubAssign<Time> for Quantity {
    fn sub_assign(&mut self, rhs: Time) {
        *self = *self - rhs;
    }
}
impl Add<DimensionlessInteger> for Quantity {
    type Output = Self;
    fn add(self, rhs: DimensionlessInteger) -> Self {
        self + Self::from(rhs)
    }
}
impl AddAssign<DimensionlessInteger> for Quantity {
    fn add_assign(&mut self, rhs: DimensionlessInteger) {
        *self = *self + rhs;
    }
}
impl Sub<DimensionlessInteger> for Quantity {
    type Output = Self;
    fn sub(self, rhs: DimensionlessInteger) -> Self {
        self - Self::from(rhs)
    }
}
impl SubAssign<DimensionlessInteger> for Quantity {
    fn sub_assign(&mut self, rhs: DimensionlessInteger) {
        *self = *self - rhs;
    }
}
impl Mul<Time> for Quantity {
    type Output = Self;
    fn mul(self, rhs: Time) -> Self {
        self * Quantity::from(rhs)
    }
}
impl MulAssign<Time> for Quantity {
    fn mul_assign(&mut self, rhs: Time) {
        *self = *self * rhs;
    }
}
impl Div<Time> for Quantity {
    type Output = Self;
    fn div(self, rhs: Time) -> Self {
        self / Quantity::from(rhs)
    }
}
impl DivAssign<Time> for Quantity {
    fn div_assign(&mut self, rhs: Time) {
        *self = *self / rhs;
    }
}
impl Mul<DimensionlessInteger> for Quantity {
    type Output = Self;
    fn mul(self, rhs: DimensionlessInteger) -> Self {
        self * Quantity::from(rhs)
    }
}
impl MulAssign<DimensionlessInteger> for Quantity {
    fn mul_assign(&mut self, rhs: DimensionlessInteger) {
        *self = *self * rhs
    }
}
impl Div<DimensionlessInteger> for Quantity {
    type Output = Self;
    fn div(self, rhs: DimensionlessInteger) -> Self {
        self / Quantity::from(rhs)
    }
}
impl DivAssign<DimensionlessInteger> for Quantity {
    fn div_assign(&mut self, rhs: DimensionlessInteger) {
        *self = *self / rhs
    }
}
#[cfg(not(any(
    feature = "dim_check_release",
    all(debug_assertions, feature = "dim_check_debug")
)))]
impl PartialEq for Quantity {
    fn eq(&self, rhs: &Self) -> bool {
        if self.unit.eq_assume_true(&rhs.unit) {
            self.value == rhs.value
        } else {
            false
        }
    }
}
impl PartialOrd for Quantity {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.unit.assert_eq_assume_ok(&other.unit);
        self.value.partial_cmp(&other.value)
    }
}
