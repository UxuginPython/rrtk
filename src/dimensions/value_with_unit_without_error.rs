use super::*;
#[derive(Clone, Copy)]
pub struct ValueWithUnitWithoutError {
    unit: Unit,
    value: f32,
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
//TODO: superior to f32 and ValueWithoutError (doesn't exist yet)
