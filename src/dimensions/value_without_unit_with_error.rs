#![cfg(feature = "error_propagation")]
use super::*;
use core::fmt;
#[derive(Clone, Copy)]
pub struct ValueWithoutUnitWithError {
    pub value: f32,
    pub error: f32,
}
impl ValueWithoutUnitWithError {
    fn new(value: f32, error: f32) -> Self {
        Self {
            value: value,
            error: error,
        }
    }
}
impl From<f32> for ValueWithoutUnitWithError {
    fn from(was: f32) -> Self {
        Self::new(was, 0.0)
    }
}
impl Add for ValueWithoutUnitWithError {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let value = self.value + rhs.value;
        let error = sqrt(self.error * self.error + rhs.error * rhs.error);
        Self::new(value, error)
    }
}
impl AddAssign for ValueWithoutUnitWithError {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}
impl_op!(Add, f32, ValueWithoutUnitWithError, add, +);
impl AddAssign<f32> for ValueWithoutUnitWithError {
    fn add_assign(&mut self, rhs: f32) {
        *self = *self + rhs;
    }
}
impl Sub for ValueWithoutUnitWithError {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        self + -rhs
    }
}
impl SubAssign for ValueWithoutUnitWithError {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}
impl_op!(Sub, f32, ValueWithoutUnitWithError, sub, -);
impl SubAssign<f32> for ValueWithoutUnitWithError {
    fn sub_assign(&mut self, rhs: f32) {
        *self = *self - rhs;
    }
}
impl Mul for ValueWithoutUnitWithError {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        let value = self.value * rhs.value;
        let error = value
            * sqrt(
                (self.error / self.value) * (self.error / self.value)
                    + (rhs.error / rhs.value) * (rhs.error / rhs.value),
            );
        Self::new(value, error)
    }
}
impl MulAssign for ValueWithoutUnitWithError {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}
impl Div for ValueWithoutUnitWithError {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        let value = self.value / rhs.value;
        let error = value
            * sqrt(
                (self.error / self.value) * (self.error / self.value)
                    + (rhs.error / rhs.value) * (rhs.error / rhs.value),
            );
        Self::new(value, error)
    }
}
impl DivAssign for ValueWithoutUnitWithError {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}
impl Neg for ValueWithoutUnitWithError {
    type Output = Self;
    fn neg(self) -> Self {
        Self::new(-self.value, self.error)
    }
}
impl fmt::Display for ValueWithoutUnitWithError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ± {}", self.value, self.error)
    }
}
