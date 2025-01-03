use super::*;
#[cfg(feature = "error_propagation")]
impl From<ValueWithoutUnitWithError> for f32 {
    fn from(was: ValueWithoutUnitWithError) -> Self {
        was.value
    }
}
#[cfg(feature = "error_propagation")]
impl_all_ops_for_inferior!(f32, ValueWithoutUnitWithError);
impl_all_ops_for_inferior!(f32, ValueWithoutUnit);
impl Add<ValueWithUnitWithoutError> for f32 {
    type Output = ValueWithUnitWithoutError;
    fn add(self, rhs: ValueWithUnitWithoutError) -> ValueWithUnitWithoutError {
        let self_rhs_type = ValueWithUnitWithoutError::new(rhs.unit, self);
        self_rhs_type + rhs
    }
}
impl Sub<ValueWithUnitWithoutError> for f32 {
    type Output = ValueWithUnitWithoutError;
    fn sub(self, rhs: ValueWithUnitWithoutError) -> ValueWithUnitWithoutError {
        let self_rhs_type = ValueWithUnitWithoutError::new(rhs.unit, self);
        self_rhs_type - rhs
    }
}
impl Mul<ValueWithUnitWithoutError> for f32 {
    type Output = ValueWithUnitWithoutError;
    fn mul(self, rhs: ValueWithUnitWithoutError) -> ValueWithUnitWithoutError {
        let self_rhs_type = ValueWithUnitWithoutError::new(constants::DIMENSIONLESS, self);
        self_rhs_type * rhs
    }
}
impl Div<ValueWithUnitWithoutError> for f32 {
    type Output = ValueWithUnitWithoutError;
    fn div(self, rhs: ValueWithUnitWithoutError) -> ValueWithUnitWithoutError {
        let self_rhs_type = ValueWithUnitWithoutError::new(constants::DIMENSIONLESS, self);
        self_rhs_type / rhs
    }
}
