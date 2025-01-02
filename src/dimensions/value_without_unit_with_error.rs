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
impl Sub for ValueWithoutUnitWithError {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        self + -rhs
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
impl Neg for ValueWithoutUnitWithError {
    type Output = Self;
    fn neg(self) -> Self {
        Self::new(-self.value, self.error)
    }
}
impl_all_assign_for_superior!(ValueWithoutUnitWithError, Self);
impl_all_ops_with_assign_for_superior!(ValueWithoutUnitWithError, f32);
impl fmt::Display for ValueWithoutUnitWithError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ± {}", self.value, self.error)
    }
}
