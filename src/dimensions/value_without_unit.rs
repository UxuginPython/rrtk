use super::*;
//see reference module for why this is non_exhaustive
#[non_exhaustive]
#[derive(Clone, Copy)]
pub enum ValueWithoutUnit {
    WithoutError(f32),
    #[cfg(feature = "error_propagation")]
    WithError(ValueWithoutUnitWithError),
}
macro_rules! impl_op {
    ($op_trait: ident, $op_func: ident, $op_symbol: tt) => {
        impl $op_trait for ValueWithoutUnit {
            type Output = Self;
            fn $op_func(self, rhs: Self) -> Self {
                match self {
                    Self::WithoutError(self_wo_error) => match rhs {
                        Self::WithoutError(rhs_wo_error) => {
                            Self::WithoutError(self_wo_error $op_symbol rhs_wo_error)
                        }
                        #[cfg(feature = "error_propagation")]
                        Self::WithError(rhs_w_error) => Self::WithError(self_wo_error $op_symbol rhs_w_error),
                    },
                    #[cfg(feature = "error_propagation")]
                    Self::WithError(self_w_error) => Self::WithError(match rhs {
                        Self::WithoutError(rhs_wo_error) => self_w_error $op_symbol rhs_wo_error,
                        Self::WithError(rhs_w_error) => self_w_error $op_symbol rhs_w_error,
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
impl_all_assign_for_superior!(ValueWithoutUnit, Self);
