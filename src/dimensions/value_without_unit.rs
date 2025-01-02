use super::*;
//see reference module for why this is non_exhaustive
#[non_exhaustive]
pub enum ValueWithoutUnit {
    WithoutError(f32),
    #[cfg(feature = "error_propagation")]
    WithError(ValueWithoutUnitWithError),
}
impl Add for ValueWithoutUnit {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        match self {
            Self::WithoutError(self_wo_error) => match rhs {
                Self::WithoutError(rhs_wo_error) => {
                    Self::WithoutError(self_wo_error + rhs_wo_error)
                }
                #[cfg(feature = "error_propagation")]
                Self::WithError(rhs_w_error) => Self::WithError(self_wo_error + rhs_w_error),
            },
            #[cfg(feature = "error_propagation")]
            Self::WithError(self_w_error) => Self::WithError(match rhs {
                Self::WithoutError(rhs_wo_error) => self_w_error + rhs_wo_error,
                Self::WithError(rhs_w_error) => self_w_error + rhs_w_error,
            }),
        }
    }
}
