use super::*;
use core::fmt;
macro_rules! impl_op_for_superior {
    ($op_trait: ident, $rhs: ident, $name: ident, $op_func: ident, $op_symbol: tt) => {
        impl $op_trait<$rhs> for $name {
            type Output = Self;
            fn $op_func(self, rhs: $rhs) -> Self {
                self $op_symbol Self::from(rhs)
            }
        }
    }
}
macro_rules! impl_all_ops_for_superior {
    ($name: ident, $rhs: ident) => {
        impl_op_for_superior!(Add, $rhs, $name, add, +);
        impl_op_for_superior!(Sub, $rhs, $name, sub, -);
        impl_op_for_superior!(Mul, $rhs, $name, mul, *);
        impl_op_for_superior!(Div, $rhs, $name, div, /);
    }
}
macro_rules! impl_assign {
    ($assign_trait: ident, $rhs: ident, $name: ident, $assign_func: ident, $op_symbol: tt) => {
        impl $assign_trait<$rhs> for $name {
            fn $assign_func(&mut self, rhs: $rhs) {
                *self = *self $op_symbol rhs;
            }
        }
    }
}
macro_rules! impl_all_assigns {
    ($name: ident, $rhs: ident) => {
        impl_assign!(AddAssign, $rhs, $name, add_assign, +);
        impl_assign!(SubAssign, $rhs, $name, sub_assign, -);
        impl_assign!(MulAssign, $rhs, $name, mul_assign, *);
        impl_assign!(DivAssign, $rhs, $name, div_assign, /);
    }
}
macro_rules! impl_op_for_inferior {
    ($op_trait: ident, $rhs: ident, $name: ident, $op_func: ident, $op_symbol: tt) => {
        impl $op_trait<$rhs> for $name {
            type Output = $rhs;
            fn $op_func(self, rhs: $rhs) -> $rhs {
                $rhs::from(self) $op_symbol rhs
            }
        }
    }
}
macro_rules! impl_all_ops_for_inferior {
    ($name: ident, $rhs: ident) => {
        impl_op_for_inferior!(Add, $rhs, $name, add, +);
        impl_op_for_inferior!(Sub, $rhs, $name, sub, -);
        impl_op_for_inferior!(Mul, $rhs, $name, mul, *);
        impl_op_for_inferior!(Div, $rhs, $name, div, /);
    }
}
macro_rules! impl_op_for_superior_add_unit {
    ($op_trait: ident, $rhs: ident, $name: ident, $op_func: ident, $op_symbol: tt) => {
        impl $op_trait<$rhs> for $name {
            type Output = Self;
            fn $op_func(self, rhs: $rhs) -> Self {
                Self::new(self.unit, self.value $op_symbol rhs)
            }
        }
    }
}
macro_rules! impl_all_ops_for_superior_add_unit {
    ($name: ident, $rhs: ident) => {
        impl_op_for_superior_add_unit!(Add, $rhs, $name, add, +);
        impl_op_for_superior_add_unit!(Sub, $rhs, $name, sub, -);
        impl_op_for_superior_add_unit!(Mul, $rhs, $name, mul, *);
        impl_op_for_superior_add_unit!(Div, $rhs, $name, div, /);
    }
}
macro_rules! impl_op_for_inferior_add_unit {
    ($op_trait: ident, $rhs: ident, $name: ident, $op_func: ident, $op_symbol: tt) => {
        impl $op_trait<$rhs> for $name {
            type Output = $rhs;
            fn $op_func(self, rhs: $rhs) -> $rhs {
                $rhs::new(rhs.unit, self $op_symbol rhs.value)
            }
        }
    }
}
macro_rules! impl_all_ops_for_inferior_add_unit {
    ($name: ident, $rhs: ident) => {
        impl_op_for_inferior_add_unit!(Add, $rhs, $name, add, +);
        impl_op_for_inferior_add_unit!(Sub, $rhs, $name, sub, -);
        impl_op_for_inferior_add_unit!(Mul, $rhs, $name, mul, *);
        impl_op_for_inferior_add_unit!(Div, $rhs, $name, div, /);
    }
}
macro_rules! impl_op_matching_rhs_error {
    ($op_trait: ident, $rhs: ident, $name: ident, $output: ident, $op_func: ident, $op_symbol: tt) => {
        impl $op_trait<$rhs> for $name {
            type Output = $output;
            fn $op_func(self, rhs: $rhs) -> $output {
                match rhs {
                    $rhs::WithoutError(x) => (self $op_symbol x).into(),
                    #[cfg(feature = "error_propagation")]
                    $rhs::WithError(x) => (self $op_symbol x).into(),
                }
            }
        }
    }
}
macro_rules! impl_all_ops_matching_rhs_error {
    ($name: ident, $rhs: ident, $output: ident) => {
        impl_op_matching_rhs_error!(Add, $rhs, $name, $output, add, +);
        impl_op_matching_rhs_error!(Sub, $rhs, $name, $output, sub, -);
        impl_op_matching_rhs_error!(Mul, $rhs, $name, $output, mul, *);
        impl_op_matching_rhs_error!(Div, $rhs, $name, $output, div, /);
    }
}
macro_rules! impl_op_matching_rhs_unit {
    ($op_trait: ident, $rhs: ident, $name: ident, $output: ident, $op_func: ident, $op_symbol: tt) => {
        impl $op_trait<$rhs> for $name {
            type Output = $output;
            fn $op_func(self, rhs: $rhs) -> $output {
                match rhs {
                    $rhs::WithoutUnit(x) => (self $op_symbol x).into(),
                    #[cfg(feature = "dimensional_analysis")]
                    $rhs::WithUnit(x) => (self $op_symbol x).into(),
                }
            }
        }
    }
}
macro_rules! impl_all_ops_matching_rhs_unit {
    ($name: ident, $rhs: ident, $output: ident) => {
        impl_op_matching_rhs_unit!(Add, $rhs, $name, $output, add, +);
        impl_op_matching_rhs_unit!(Sub, $rhs, $name, $output, sub, -);
        impl_op_matching_rhs_unit!(Mul, $rhs, $name, $output, mul, *);
        impl_op_matching_rhs_unit!(Div, $rhs, $name, $output, div, /);
    }
}
macro_rules! impl_op_matching_self_error {
    ($op_trait: ident, $rhs: ident, $name: ident, $output: ident, $op_func: ident, $op_symbol: tt) => {
        impl $op_trait<$rhs> for $name {
            type Output = $output;
            fn $op_func(self, rhs: $rhs) -> $output {
                match self {
                    Self::WithoutError(x) => (x $op_symbol rhs).into(),
                    #[cfg(feature = "error_propagation")]
                    Self::WithError(x) => (x $op_symbol rhs).into(),
                }
            }
        }
    }
}
macro_rules! impl_all_ops_matching_self_error {
    ($name: ident, $rhs: ident, $output: ident) => {
        impl_op_matching_self_error!(Add, $rhs, $name, $output, add, +);
        impl_op_matching_self_error!(Sub, $rhs, $name, $output, sub, -);
        impl_op_matching_self_error!(Mul, $rhs, $name, $output, mul, *);
        impl_op_matching_self_error!(Div, $rhs, $name, $output, div, /);
    }
}
macro_rules! impl_op_matching_self_unit {
    ($op_trait: ident, $rhs: ident, $name: ident, $output: ident, $op_func: ident, $op_symbol: tt) => {
        impl $op_trait<$rhs> for $name {
            type Output = $output;
            fn $op_func(self, rhs: $rhs) -> $output {
                match self {
                    Self::WithoutUnit(x) => (x $op_symbol rhs).into(),
                    #[cfg(feature = "dimensional_analysis")]
                    Self::WithUnit(x) => (x $op_symbol rhs).into(),
                }
            }
        }
    }
}
macro_rules! impl_all_ops_matching_self_unit {
    ($name: ident, $rhs: ident, $output: ident) => {
        impl_op_matching_self_unit!(Add, $rhs, $name, $output, add, +);
        impl_op_matching_self_unit!(Sub, $rhs, $name, $output, sub, -);
        impl_op_matching_self_unit!(Mul, $rhs, $name, $output, mul, *);
        impl_op_matching_self_unit!(Div, $rhs, $name, $output, div, /);
    }
}

macro_rules! impl_from_for_inner {
    ($name: ident, $was: ident) => {
        impl From<$was> for $name {
            fn from(was: $was) -> Self {
                was.value
            }
        }
    };
}
macro_rules! impl_from_matching_error {
    ($name: ident, $was: ident) => {
        impl From<$was> for $name {
            fn from(was: $was) -> Self {
                match was {
                    $was::WithoutError(x) => x.into(),
                    #[cfg(feature = "error_propagation")]
                    $was::WithError(x) => x.into(),
                }
            }
        }
    };
}
macro_rules! impl_from_matching_unit {
    ($name: ident, $was: ident) => {
        impl From<$was> for $name {
            fn from(was: $was) -> Self {
                match was {
                    $was::WithoutUnit(x) => x.into(),
                    #[cfg(feature = "dimensional_analysis")]
                    $was::WithUnit(x) => x.into(),
                }
            }
        }
    };
}
macro_rules! impl_from_variant {
    ($name: ident, $variant: ident, $was: ident) => {
        impl From<$was> for $name {
            fn from(was: $was) -> Self {
                Self::$variant(was.into())
            }
        }
    };
}

#[cfg(all(
    not(feature = "dimensional_analysis"),
    not(feature = "error_propagation")
))]
pub trait GenericValue:
    Add<f32>
    + Add<ValueWithoutUnit>
    + Add<ValueWithoutError>
    + Add<Value>
    + Sub<f32>
    + Sub<ValueWithoutUnit>
    + Sub<ValueWithoutError>
    + Sub<Value>
    + Mul<f32>
    + Mul<ValueWithoutUnit>
    + Mul<ValueWithoutError>
    + Mul<Value>
    + Div<f32>
    + Div<ValueWithoutUnit>
    + Div<ValueWithoutError>
    + Div<Value>
{
}
#[cfg(all(not(feature = "dimensional_analysis"), feature = "error_propagation"))]
pub trait GenericValue:
    Add<f32>
    + Add<ValueWithoutUnitWithError>
    + Add<ValueWithoutUnit>
    + Add<ValueWithoutError>
    + Add<ValueWithError>
    + Add<Value>
    + Sub<f32>
    + Sub<ValueWithoutUnitWithError>
    + Sub<ValueWithoutUnit>
    + Sub<ValueWithoutError>
    + Sub<ValueWithError>
    + Sub<Value>
    + Mul<f32>
    + Mul<ValueWithoutUnitWithError>
    + Mul<ValueWithoutUnit>
    + Mul<ValueWithoutError>
    + Mul<ValueWithError>
    + Mul<Value>
    + Div<f32>
    + Div<ValueWithoutUnitWithError>
    + Div<ValueWithoutUnit>
    + Div<ValueWithoutError>
    + Div<ValueWithError>
    + Div<Value>
{
}
#[cfg(all(feature = "dimensional_analysis", not(feature = "error_propagation")))]
pub trait GenericValue:
    From<ValueWithUnitWithoutError>
    + From<ValueWithUnit>
    + Add<f32>
    + Add<ValueWithUnitWithoutError>
    + Add<ValueWithoutUnit>
    + Add<ValueWithUnit>
    + Add<ValueWithoutError>
    + Add<Value>
    + Sub<f32>
    + Sub<ValueWithUnitWithoutError>
    + Sub<ValueWithoutUnit>
    + Sub<ValueWithUnit>
    + Sub<ValueWithoutError>
    + Sub<Value>
    + Mul<f32>
    + Mul<ValueWithUnitWithoutError>
    + Mul<ValueWithoutUnit>
    + Mul<ValueWithUnit>
    + Mul<ValueWithoutError>
    + Mul<Value>
    + Div<f32>
    + Div<ValueWithUnitWithoutError>
    + Div<ValueWithoutUnit>
    + Div<ValueWithUnit>
    + Div<ValueWithoutError>
    + Div<Value>
{
}
#[cfg(all(feature = "dimensional_analysis", feature = "error_propagation"))]
pub trait GenericValue:
    From<ValueWithUnitWithoutError>
    + From<ValueWithUnitWithError>
    + From<ValueWithUnit>
    + Add<f32>
    + Add<ValueWithoutUnitWithError>
    + Add<ValueWithUnitWithoutError>
    + Add<ValueWithUnitWithError>
    + Add<ValueWithoutUnit>
    + Add<ValueWithUnit>
    + Add<ValueWithoutError>
    + Add<ValueWithError>
    + Add<Value>
    + Sub<f32>
    + Sub<ValueWithoutUnitWithError>
    + Sub<ValueWithUnitWithoutError>
    + Sub<ValueWithUnitWithError>
    + Sub<ValueWithoutUnit>
    + Sub<ValueWithUnit>
    + Sub<ValueWithoutError>
    + Sub<ValueWithError>
    + Sub<Value>
    + Mul<f32>
    + Mul<ValueWithoutUnitWithError>
    + Mul<ValueWithUnitWithoutError>
    + Mul<ValueWithUnitWithError>
    + Mul<ValueWithoutUnit>
    + Mul<ValueWithUnit>
    + Mul<ValueWithoutError>
    + Mul<ValueWithError>
    + Mul<Value>
    + Div<f32>
    + Div<ValueWithoutUnitWithError>
    + Div<ValueWithUnitWithoutError>
    + Div<ValueWithUnitWithError>
    + Div<ValueWithoutUnit>
    + Div<ValueWithUnit>
    + Div<ValueWithoutError>
    + Div<ValueWithError>
    + Div<Value>
{
}
//Values certainly with units cannot be From<T> where T does not certainly have a unit.
#[cfg(not(feature = "error_propagation"))]
pub trait GenericValuePossiblyWithoutUnit:
    GenericValue + From<f32> + From<ValueWithoutUnit> + From<ValueWithoutError> + From<Value>
{
}
#[cfg(feature = "error_propagation")]
pub trait GenericValuePossiblyWithoutUnit:
    GenericValue
    + From<f32>
    + From<ValueWithoutUnitWithError>
    + From<ValueWithoutUnit>
    + From<ValueWithoutError>
    + From<ValueWithError>
    + From<Value>
{
}

mod f32_impls {
    use super::*;
    #[cfg(feature = "error_propagation")]
    impl_from_for_inner!(f32, ValueWithoutUnitWithError);
    #[cfg(feature = "dimensional_analysis")]
    impl_from_for_inner!(f32, ValueWithUnitWithoutError);
    #[cfg(all(feature = "dimensional_analysis", feature = "error_propagation"))]
    impl From<ValueWithUnitWithError> for f32 {
        fn from(was: ValueWithUnitWithError) -> Self {
            was.value.value
        }
    }
    impl_from_matching_error!(f32, ValueWithoutUnit);
    #[cfg(feature = "dimensional_analysis")]
    impl_from_matching_error!(f32, ValueWithUnit);
    impl_from_matching_unit!(f32, ValueWithoutError);
    #[cfg(feature = "error_propagation")]
    impl_from_matching_unit!(f32, ValueWithError);
    impl_from_matching_unit!(f32, Value);
    #[cfg(feature = "error_propagation")]
    impl_all_ops_for_inferior!(f32, ValueWithoutUnitWithError);
    #[cfg(feature = "dimensional_analysis")]
    impl_all_ops_for_inferior_add_unit!(f32, ValueWithUnitWithoutError);
    #[cfg(all(feature = "dimensional_analysis", feature = "error_propagation"))]
    impl_all_ops_for_inferior_add_unit!(f32, ValueWithUnitWithError);
    impl_all_ops_matching_rhs_error!(f32, ValueWithoutUnit, ValueWithoutUnit);
    #[cfg(feature = "dimensional_analysis")]
    impl_all_ops_matching_rhs_error!(f32, ValueWithUnit, ValueWithUnit);
    impl_all_ops_matching_rhs_unit!(f32, ValueWithoutError, ValueWithoutError);
    #[cfg(feature = "error_propagation")]
    impl_all_ops_matching_rhs_unit!(f32, ValueWithError, ValueWithError);
    impl_all_ops_matching_rhs_unit!(f32, Value, Value);
    impl GenericValue for f32 {}
    impl GenericValuePossiblyWithoutUnit for f32 {}
}

#[cfg(feature = "error_propagation")]
mod value_without_unit_with_error {
    use super::*;
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
    #[cfg(feature = "dimensional_analysis")]
    impl From<ValueWithUnitWithoutError> for ValueWithoutUnitWithError {
        fn from(was: ValueWithUnitWithoutError) -> Self {
            was.value.into()
        }
    }
    #[cfg(feature = "dimensional_analysis")]
    impl_from_for_inner!(ValueWithoutUnitWithError, ValueWithUnitWithError);
    impl_from_matching_error!(ValueWithoutUnitWithError, ValueWithoutUnit);
    #[cfg(feature = "dimensional_analysis")]
    impl_from_matching_error!(ValueWithoutUnitWithError, ValueWithUnit);
    impl_from_matching_unit!(ValueWithoutUnitWithError, ValueWithoutError);
    impl_from_matching_unit!(ValueWithoutUnitWithError, ValueWithError);
    impl_from_matching_unit!(ValueWithoutUnitWithError, Value);
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
    impl_all_assigns!(ValueWithoutUnitWithError, Self);
    impl fmt::Display for ValueWithoutUnitWithError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{} ± {}", self.value, self.error)
        }
    }
    impl_all_ops_for_superior!(ValueWithoutUnitWithError, f32);
    impl_all_assigns!(ValueWithoutUnitWithError, f32);
    #[cfg(feature = "dimensional_analysis")]
    impl_all_ops_for_inferior_add_unit!(ValueWithoutUnitWithError, ValueWithUnitWithError);
    #[cfg(feature = "dimensional_analysis")]
    macro_rules! impl_op_value_w_unit_wo_error {
        ($op_trait: ident, $op_func: ident, $op_symbol: tt) => {
            impl $op_trait<ValueWithUnitWithoutError> for ValueWithoutUnitWithError {
                type Output = ValueWithUnitWithError;
                fn $op_func(self, rhs: ValueWithUnitWithoutError) -> ValueWithUnitWithError {
                    ValueWithUnitWithError::new(rhs.unit, self $op_symbol rhs.value)
                }
            }
        }
    }
    #[cfg(feature = "dimensional_analysis")]
    impl_op_value_w_unit_wo_error!(Add, add, +);
    #[cfg(feature = "dimensional_analysis")]
    impl_op_value_w_unit_wo_error!(Sub, sub, -);
    #[cfg(feature = "dimensional_analysis")]
    impl_op_value_w_unit_wo_error!(Mul, mul, *);
    #[cfg(feature = "dimensional_analysis")]
    impl_op_value_w_unit_wo_error!(Div, div, /);
    impl_all_ops_matching_rhs_error!(
        ValueWithoutUnitWithError,
        ValueWithoutUnit,
        ValueWithoutUnitWithError
    );
    #[cfg(feature = "dimensional_analysis")]
    impl_all_ops_matching_rhs_error!(
        ValueWithoutUnitWithError,
        ValueWithUnit,
        ValueWithUnitWithError
    );
    impl_all_ops_matching_rhs_unit!(ValueWithoutUnitWithError, ValueWithoutError, ValueWithError);
    impl_all_ops_matching_rhs_unit!(ValueWithoutUnitWithError, ValueWithError, ValueWithError);
    impl_all_ops_matching_rhs_unit!(ValueWithoutUnitWithError, Value, ValueWithError);
    impl GenericValue for ValueWithoutUnitWithError {}
    impl GenericValuePossiblyWithoutUnit for ValueWithoutUnitWithError {}
}
#[cfg(feature = "error_propagation")]
pub use value_without_unit_with_error::*;

#[cfg(feature = "dimensional_analysis")]
mod value_with_unit_without_error {
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
    #[cfg(feature = "error_propagation")]
    impl From<ValueWithUnitWithError> for ValueWithUnitWithoutError {
        fn from(was: ValueWithUnitWithError) -> Self {
            Self::new(was.unit, was.value.into())
        }
    }
    impl_from_matching_error!(ValueWithUnitWithoutError, ValueWithUnit);
    macro_rules! impl_op {
        ($op_trait: ident, $op_func: ident, $op_symbol: tt) => {
            impl $op_trait for ValueWithUnitWithoutError {
                type Output = Self;
                fn $op_func(self, rhs: Self) -> Self {
                    Self::new(self.unit $op_symbol rhs.unit, self.value $op_symbol rhs.value)
                }
            }
        }
    }
    impl_op!(Add, add, +);
    impl_op!(Sub, sub, -);
    impl_op!(Mul, mul, *);
    impl_op!(Div, div, /);
    impl Neg for ValueWithUnitWithoutError {
        type Output = Self;
        fn neg(self) -> Self {
            Self::new(-self.unit, -self.value)
        }
    }
    impl_all_ops_for_superior_add_unit!(ValueWithUnitWithoutError, f32);
    impl_all_assigns!(ValueWithUnitWithoutError, f32);
    #[cfg(feature = "error_propagation")]
    impl_all_ops_for_inferior!(ValueWithUnitWithoutError, ValueWithUnitWithError);
    macro_rules! impl_op_value_wo_unit_w_error {
        ($op_trait: ident, $op_func: ident, $op_symbol: tt) => {
            impl $op_trait<ValueWithoutUnitWithError> for ValueWithUnitWithoutError {
                type Output = ValueWithUnitWithError;
                fn $op_func(self, rhs: ValueWithoutUnitWithError) -> ValueWithUnitWithError {
                    ValueWithUnitWithError::new(self.unit, self.value $op_symbol rhs)
                }
            }
        }
    }
    #[cfg(feature = "error_propagation")]
    impl_op_value_wo_unit_w_error!(Add, add, +);
    #[cfg(feature = "error_propagation")]
    impl_op_value_wo_unit_w_error!(Sub, sub, -);
    #[cfg(feature = "error_propagation")]
    impl_op_value_wo_unit_w_error!(Mul, mul, *);
    #[cfg(feature = "error_propagation")]
    impl_op_value_wo_unit_w_error!(Div, div, /);
    impl_all_ops_matching_rhs_error!(ValueWithUnitWithoutError, ValueWithoutUnit, ValueWithUnit);
    impl_all_ops_matching_rhs_error!(ValueWithUnitWithoutError, ValueWithUnit, ValueWithUnit);
    impl_all_ops_matching_rhs_unit!(
        ValueWithUnitWithoutError,
        ValueWithoutError,
        ValueWithUnitWithoutError
    );
    #[cfg(feature = "error_propagation")]
    impl_all_ops_matching_rhs_unit!(
        ValueWithUnitWithoutError,
        ValueWithError,
        ValueWithUnitWithError
    );
    impl_all_ops_matching_rhs_unit!(ValueWithUnitWithoutError, Value, ValueWithUnit);
    impl GenericValue for ValueWithUnitWithoutError {}
}
#[cfg(feature = "dimensional_analysis")]
pub use value_with_unit_without_error::*;

#[cfg(all(feature = "dimensional_analysis", feature = "error_propagation"))]
mod value_with_unit_with_error {
    use super::*;
    #[derive(Clone, Copy)]
    pub struct ValueWithUnitWithError {
        pub unit: Unit,
        pub value: ValueWithoutUnitWithError,
    }
    impl ValueWithUnitWithError {
        pub fn new(unit: Unit, value: ValueWithoutUnitWithError) -> Self {
            Self {
                unit: unit,
                value: value,
            }
        }
    }
    impl From<ValueWithUnitWithoutError> for ValueWithUnitWithError {
        fn from(was: ValueWithUnitWithoutError) -> Self {
            Self::new(was.unit, was.value.into())
        }
    }
    impl_from_matching_error!(ValueWithUnitWithError, ValueWithUnit);
    macro_rules! impl_op {
        ($op_trait: ident, $op_func: ident, $op_symbol: tt) => {
            impl $op_trait for ValueWithUnitWithError {
                type Output = Self;
                fn $op_func(self, rhs: Self) -> Self {
                    Self::new(self.unit $op_symbol rhs.unit, self.value $op_symbol rhs.value)
                }
            }
        }
    }
    impl_op!(Add, add, +);
    impl_op!(Sub, sub, -);
    impl_op!(Mul, mul, *);
    impl_op!(Div, div, /);
    impl Neg for ValueWithUnitWithError {
        type Output = Self;
        fn neg(self) -> Self {
            Self::new(-self.unit, -self.value)
        }
    }
    impl_all_ops_for_superior_add_unit!(ValueWithUnitWithError, f32);
    impl_all_assigns!(ValueWithUnitWithError, f32);
    impl_all_ops_for_superior_add_unit!(ValueWithUnitWithError, ValueWithoutUnitWithError);
    impl_all_assigns!(ValueWithUnitWithError, ValueWithoutUnitWithError);
    impl_all_ops_for_superior!(ValueWithUnitWithError, ValueWithUnitWithoutError);
    impl_all_assigns!(ValueWithUnitWithError, ValueWithUnitWithoutError);
    impl_all_ops_matching_rhs_error!(
        ValueWithUnitWithError,
        ValueWithoutUnit,
        ValueWithUnitWithError
    );
    impl_all_ops_matching_rhs_error!(
        ValueWithUnitWithError,
        ValueWithUnit,
        ValueWithUnitWithError
    );
    impl_all_ops_matching_rhs_unit!(
        ValueWithUnitWithError,
        ValueWithoutError,
        ValueWithUnitWithError
    );
    impl_all_ops_matching_rhs_unit!(
        ValueWithUnitWithError,
        ValueWithError,
        ValueWithUnitWithError
    );
    impl_all_ops_matching_rhs_unit!(ValueWithUnitWithError, Value, ValueWithUnitWithError);
    impl GenericValue for ValueWithUnitWithError {}
}
#[cfg(all(feature = "dimensional_analysis", feature = "error_propagation"))]
pub use value_with_unit_with_error::*;

mod value_without_unit {
    use super::*;
    #[derive(Clone, Copy)]
    #[non_exhaustive]
    pub enum ValueWithoutUnit {
        WithoutError(f32),
        #[cfg(feature = "error_propagation")]
        WithError(ValueWithoutUnitWithError),
    }
    impl_from_variant!(ValueWithoutUnit, WithoutError, f32);
    #[cfg(feature = "error_propagation")]
    impl_from_variant!(ValueWithoutUnit, WithError, ValueWithoutUnitWithError);
    #[cfg(feature = "dimensional_analysis")]
    impl_from_variant!(ValueWithoutUnit, WithoutError, ValueWithUnitWithoutError);
    #[cfg(all(feature = "dimensional_analysis", feature = "error_propagation"))]
    impl_from_variant!(ValueWithoutUnit, WithError, ValueWithUnitWithError);
    #[cfg(feature = "dimensional_analysis")]
    impl_from_matching_error!(ValueWithoutUnit, ValueWithUnit);
    impl_from_matching_unit!(ValueWithoutUnit, ValueWithoutError);
    #[cfg(feature = "error_propagation")]
    impl_from_matching_unit!(ValueWithoutUnit, ValueWithError);
    impl_from_matching_unit!(ValueWithoutUnit, Value);
    impl_all_ops_matching_self_error!(ValueWithoutUnit, f32, Self);
    #[cfg(feature = "error_propagation")]
    impl_all_ops_matching_self_error!(
        ValueWithoutUnit,
        ValueWithoutUnitWithError,
        ValueWithoutUnitWithError
    );
    #[cfg(feature = "dimensional_analysis")]
    impl_all_ops_matching_self_error!(ValueWithoutUnit, ValueWithUnitWithoutError, ValueWithUnit);
    #[cfg(all(feature = "dimensional_analysis", feature = "error_propagation"))]
    impl_all_ops_matching_self_error!(
        ValueWithoutUnit,
        ValueWithUnitWithError,
        ValueWithUnitWithError
    );
    impl_all_ops_matching_self_error!(ValueWithoutUnit, Self, Self);
    #[cfg(feature = "dimensional_analysis")]
    impl_all_ops_matching_self_error!(ValueWithoutUnit, ValueWithUnit, ValueWithUnit);
    impl_all_ops_matching_self_error!(ValueWithoutUnit, ValueWithoutError, Value);
    #[cfg(feature = "error_propagation")]
    impl_all_ops_matching_self_error!(ValueWithoutUnit, ValueWithError, ValueWithError);
    impl_all_ops_matching_self_error!(ValueWithoutUnit, Value, Value);
    impl GenericValue for ValueWithoutUnit {}
    impl GenericValuePossiblyWithoutUnit for ValueWithoutUnit {}
}
pub use value_without_unit::*;

#[cfg(feature = "dimensional_analysis")]
mod value_with_unit {
    use super::*;
    #[derive(Clone, Copy)]
    #[non_exhaustive]
    pub enum ValueWithUnit {
        WithoutError(ValueWithUnitWithoutError),
        #[cfg(feature = "error_propagation")]
        WithError(ValueWithUnitWithError),
    }
    impl_from_variant!(ValueWithUnit, WithoutError, ValueWithUnitWithoutError);
    #[cfg(feature = "error_propagation")]
    impl_from_variant!(ValueWithUnit, WithError, ValueWithUnitWithError);
    impl_all_ops_matching_self_error!(ValueWithUnit, f32, Self);
    #[cfg(feature = "error_propagation")]
    impl_all_ops_matching_self_error!(
        ValueWithUnit,
        ValueWithoutUnitWithError,
        ValueWithUnitWithError
    );
    impl_all_ops_matching_self_error!(ValueWithUnit, ValueWithUnitWithoutError, Self);
    #[cfg(feature = "error_propagation")]
    impl_all_ops_matching_self_error!(
        ValueWithUnit,
        ValueWithUnitWithError,
        ValueWithUnitWithError
    );
    impl_all_ops_matching_self_error!(ValueWithUnit, ValueWithoutUnit, Self);
    impl_all_ops_matching_self_error!(ValueWithUnit, Self, Self);
    impl_all_ops_matching_self_error!(ValueWithUnit, ValueWithoutError, Self);
    #[cfg(feature = "error_propagation")]
    impl_all_ops_matching_self_error!(ValueWithUnit, ValueWithError, ValueWithUnitWithError);
    impl_all_ops_matching_self_error!(ValueWithUnit, Value, Self);
    impl GenericValue for ValueWithUnit {}
}
#[cfg(feature = "dimensional_analysis")]
pub use value_with_unit::*;

mod value_without_error {
    use super::*;
    #[derive(Clone, Copy)]
    #[non_exhaustive]
    pub enum ValueWithoutError {
        WithoutUnit(f32),
        #[cfg(feature = "dimensional_analysis")]
        WithUnit(ValueWithUnitWithoutError),
    }
    impl_from_variant!(ValueWithoutError, WithoutUnit, f32);
    #[cfg(feature = "error_propagation")]
    impl_from_variant!(ValueWithoutError, WithoutUnit, ValueWithoutUnitWithError);
    #[cfg(feature = "dimensional_analysis")]
    impl_from_variant!(ValueWithoutError, WithUnit, ValueWithUnitWithoutError);
    #[cfg(all(feature = "dimensional_analysis", feature = "error_propagation"))]
    impl_from_variant!(ValueWithoutError, WithUnit, ValueWithUnitWithError);
    impl_from_matching_error!(ValueWithoutError, ValueWithoutUnit);
    #[cfg(feature = "dimensional_analysis")]
    impl_from_matching_error!(ValueWithoutError, ValueWithUnit);
    #[cfg(feature = "error_propagation")]
    impl_from_matching_unit!(ValueWithoutError, ValueWithError);
    impl_from_matching_unit!(ValueWithoutError, Value);
    impl_all_ops_matching_self_unit!(ValueWithoutError, f32, Self);
    #[cfg(feature = "error_propagation")]
    impl_all_ops_matching_self_unit!(ValueWithoutError, ValueWithoutUnitWithError, ValueWithError);
    #[cfg(feature = "dimensional_analysis")]
    impl_all_ops_matching_self_unit!(
        ValueWithoutError,
        ValueWithUnitWithoutError,
        ValueWithUnitWithoutError
    );
    #[cfg(all(feature = "dimensional_analysis", feature = "error_propagation"))]
    impl_all_ops_matching_self_unit!(
        ValueWithoutError,
        ValueWithUnitWithError,
        ValueWithUnitWithError
    );
    impl_all_ops_matching_self_unit!(ValueWithoutError, ValueWithoutUnit, Value);
    #[cfg(feature = "dimensional_analysis")]
    impl_all_ops_matching_self_unit!(ValueWithoutError, ValueWithUnit, ValueWithUnit);
    impl_all_ops_matching_self_unit!(ValueWithoutError, Self, Self);
    #[cfg(feature = "error_propagation")]
    impl_all_ops_matching_self_unit!(ValueWithoutError, ValueWithError, ValueWithError);
    impl_all_ops_matching_self_unit!(ValueWithoutError, Value, Value);
    impl GenericValue for ValueWithoutError {}
    impl GenericValuePossiblyWithoutUnit for ValueWithoutError {}
}
pub use value_without_error::*;

#[cfg(feature = "error_propagation")]
mod value_with_error {
    use super::*;
    #[derive(Clone, Copy)]
    #[non_exhaustive]
    pub enum ValueWithError {
        WithoutUnit(ValueWithoutUnitWithError),
        #[cfg(feature = "dimensional_analysis")]
        WithUnit(ValueWithUnitWithError),
    }
    //This calls .into()
    impl_from_variant!(ValueWithError, WithoutUnit, f32);
    impl_from_variant!(ValueWithError, WithoutUnit, ValueWithoutUnitWithError);
    #[cfg(feature = "dimensional_analysis")]
    impl_from_variant!(ValueWithError, WithUnit, ValueWithUnitWithoutError);
    #[cfg(feature = "dimensional_analysis")]
    impl_from_variant!(ValueWithError, WithUnit, ValueWithUnitWithError);
    impl_from_matching_error!(ValueWithError, ValueWithoutUnit);
    #[cfg(feature = "dimensional_analysis")]
    impl_from_matching_error!(ValueWithError, ValueWithUnit);
    impl_from_matching_unit!(ValueWithError, ValueWithoutError);
    impl_from_matching_unit!(ValueWithError, Value);
    impl_all_ops_matching_self_unit!(ValueWithError, f32, Self);
    impl_all_ops_matching_self_unit!(ValueWithError, ValueWithoutUnitWithError, Self);
    #[cfg(feature = "dimensional_analysis")]
    impl_all_ops_matching_self_unit!(
        ValueWithError,
        ValueWithUnitWithoutError,
        ValueWithUnitWithError
    );
    #[cfg(feature = "dimensional_analysis")]
    impl_all_ops_matching_self_unit!(
        ValueWithError,
        ValueWithUnitWithError,
        ValueWithUnitWithError
    );
    impl_all_ops_matching_self_unit!(ValueWithError, ValueWithoutUnit, Self);
    #[cfg(feature = "dimensional_analysis")]
    impl_all_ops_matching_self_unit!(ValueWithError, ValueWithUnit, ValueWithUnitWithError);
    impl_all_ops_matching_self_unit!(ValueWithError, ValueWithoutError, Self);
    impl_all_ops_matching_self_unit!(ValueWithError, Self, Self);
    impl_all_ops_matching_self_unit!(ValueWithError, Value, Self);
    impl GenericValue for ValueWithError {}
    impl GenericValuePossiblyWithoutUnit for ValueWithError {}
}
#[cfg(feature = "error_propagation")]
pub use value_with_error::*;

mod value {
    use super::*;
    #[derive(Clone, Copy)]
    #[non_exhaustive]
    pub enum Value {
        WithoutUnit(ValueWithoutUnit),
        #[cfg(feature = "dimensional_analysis")]
        WithUnit(ValueWithUnit),
    }
    impl_from_variant!(Value, WithoutUnit, f32);
    #[cfg(feature = "error_propagation")]
    impl_from_variant!(Value, WithoutUnit, ValueWithoutUnitWithError);
    #[cfg(feature = "dimensional_analysis")]
    impl_from_variant!(Value, WithUnit, ValueWithUnitWithoutError);
    #[cfg(all(feature = "dimensional_analysis", feature = "error_propagation"))]
    impl_from_variant!(Value, WithUnit, ValueWithUnitWithError);
    impl_from_matching_error!(Value, ValueWithoutUnit);
    #[cfg(feature = "dimensional_analysis")]
    impl_from_matching_error!(Value, ValueWithUnit);
    impl_from_matching_unit!(Value, ValueWithoutError);
    #[cfg(feature = "error_propagation")]
    impl_from_matching_unit!(Value, ValueWithError);
    impl_all_ops_matching_self_unit!(Value, f32, Self);
    #[cfg(feature = "error_propagation")]
    impl_all_ops_matching_self_unit!(Value, ValueWithoutUnitWithError, ValueWithError);
    #[cfg(feature = "dimensional_analysis")]
    impl_all_ops_matching_self_unit!(Value, ValueWithUnitWithoutError, ValueWithUnit);
    #[cfg(all(feature = "dimensional_analysis", feature = "error_propagation"))]
    impl_all_ops_matching_self_unit!(Value, ValueWithUnitWithError, ValueWithUnitWithError);
    impl_all_ops_matching_self_unit!(Value, ValueWithoutUnit, Self);
    #[cfg(feature = "dimensional_analysis")]
    impl_all_ops_matching_self_unit!(Value, ValueWithUnit, ValueWithUnit);
    impl_all_ops_matching_self_unit!(Value, ValueWithoutError, Self);
    #[cfg(feature = "error_propagation")]
    impl_all_ops_matching_self_unit!(Value, ValueWithError, ValueWithError);
    impl_all_ops_matching_self_unit!(Value, Self, Self);
    impl GenericValue for Value {}
    impl GenericValuePossiblyWithoutUnit for Value {}
}
pub use value::*;
