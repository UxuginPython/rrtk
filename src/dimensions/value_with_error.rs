#![cfg(feature = "error_propagation")]
use super::*;
use core::fmt;
#[derive(Clone, Copy)]
pub struct ValueWithError {
    pub value: f32,
    pub error: f32,
}
impl ValueWithError {
    fn new(value: f32, error: f32) -> Self {
        Self {
            value: value,
            error: error,
        }
    }
}
impl From<f32> for ValueWithError {
    fn from(was: f32) -> Self {
        Self::new(was, 0.0)
    }
}
impl Add for ValueWithError {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let value = self.value + rhs.value;
        let error = sqrt(self.error * self.error + rhs.error * rhs.error);
        Self::new(value, error)
    }
}
impl AddAssign for ValueWithError {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}
impl Sub for ValueWithError {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        self + -rhs
    }
}
impl SubAssign for ValueWithError {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}
impl Mul for ValueWithError {
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
impl MulAssign for ValueWithError {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}
impl Div for ValueWithError {
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
impl DivAssign for ValueWithError {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}
impl Neg for ValueWithError {
    type Output = Self;
    fn neg(self) -> Self {
        Self::new(-self.value, self.error)
    }
}
impl fmt::Display for ValueWithError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ± {}", self.value, self.error)
    }
}
