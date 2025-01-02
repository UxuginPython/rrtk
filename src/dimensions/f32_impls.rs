use super::*;
impl From<ValueWithoutUnitWithError> for f32 {
    fn from(was: ValueWithoutUnitWithError) -> Self {
        was.value
    }
}
impl_all_ops_for_inferior!(f32, ValueWithoutUnitWithError);
