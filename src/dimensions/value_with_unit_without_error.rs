use super::*;
#[derive(Clone, Copy)]
pub struct ValueWithUnitWithoutError {
    pub unit: Unit,
    pub value: f32,
}
impl ValueWithUnitWithoutError {
    pub fn new(unit: Unit, value: f32) -> Self {
        Self {
            unit: unit,
            value: value,
        }
    }
}
macro_rules! impl_op {
    ($op_trait: ident, $op_func: ident, $op_symbol: tt) => {
        impl $op_trait for ValueWithUnitWithoutError {
            type Output = Self;
            fn $op_func(self, rhs: Self) -> Self {
                Self::new(self.unit + rhs.unit, self.value + rhs.value)
            }
        }
    };
}
impl_op!(Add, add, +);
impl_op!(Sub, sub, -);
impl_op!(Mul, mul, *);
impl_op!(Div, div, /);
impl Add<f32> for ValueWithUnitWithoutError {
    type Output = Self;
    fn add(self, rhs: f32) -> Self {
        let rhs = Self::new(self.unit, rhs);
        self + rhs
    }
}
impl Sub<f32> for ValueWithUnitWithoutError {
    type Output = Self;
    fn sub(self, rhs: f32) -> Self {
        let rhs = Self::new(self.unit, rhs);
        self - rhs
    }
}
impl Mul<f32> for ValueWithUnitWithoutError {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        let rhs = Self::new(constants::DIMENSIONLESS, rhs);
        self * rhs
    }
}
impl Div<f32> for ValueWithUnitWithoutError {
    type Output = Self;
    fn div(self, rhs: f32) -> Self {
        let rhs = Self::new(constants::DIMENSIONLESS, rhs);
        self / rhs
    }
}
impl_all_ops_with_assign_for_superior!(ValueWithUnitWithoutError, ValueWithoutError);
