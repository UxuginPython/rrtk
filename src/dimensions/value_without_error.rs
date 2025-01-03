use super::*;
#[derive(Clone, Copy)]
pub enum ValueWithoutError {
    WithoutUnit(f32),
    WithUnit(ValueWithUnitWithoutError),
}
impl From<f32> for ValueWithoutError {
    fn from(was: f32) -> Self {
        Self::WithoutUnit(was)
    }
}
macro_rules! impl_op {
    ($op_trait: ident, $op_func: ident, $op_symbol: tt) => {
        impl $op_trait for ValueWithoutError {
            type Output = Self;
            fn $op_func(self, rhs: Self) -> Self {
                match self {
                    Self::WithoutUnit(self_wo_unit) => match rhs {
                        Self::WithoutUnit(rhs_wo_unit) => {
                            Self::WithoutUnit(self_wo_unit $op_symbol rhs_wo_unit)
                        }
                        Self::WithUnit(rhs_w_unit) => Self::WithUnit(self_wo_unit $op_symbol rhs_w_unit),
                    },
                    Self::WithUnit(self_w_unit) => Self::WithUnit(match rhs {
                        Self::WithoutUnit(rhs_wo_unit) => self_w_unit $op_symbol rhs_wo_unit,
                        Self::WithUnit(rhs_w_unit) => self_w_unit $op_symbol rhs_w_unit,
                    }),
                }
            }
        }
    }
}
impl_op!(Add, add, +);
impl_op!(Sub, sub, -);
impl_op!(Mul, mul, *);
impl_op!(Div, div, /);
impl_all_ops_with_assign_for_superior!(ValueWithoutError, f32);
impl_all_ops_for_inferior!(ValueWithoutError, ValueWithUnitWithoutError);
