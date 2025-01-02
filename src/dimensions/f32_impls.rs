use super::*;
#[cfg(feature = "error_propagation")]
impl From<ValueWithoutUnitWithError> for f32 {
    fn from(was: ValueWithoutUnitWithError) -> Self {
        was.value
    }
}
#[cfg(feature = "error_propagation")]
impl_all_ops_for_inferior!(f32, ValueWithoutUnitWithError);
