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
macro_rules! impl_from_for_inner {
    ($name: ident, $was: ident) => {
        impl From<$was> for $name {
            fn from(was: $was) -> Self {
                was.value
            }
        }
    };
}

mod f32_impls {
    use super::*;
    #[cfg(feature = "error_propagation")]
    impl_all_ops_for_inferior!(f32, ValueWithoutUnitWithError);
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
    impl From<ValueWithoutUnit> for f32 {
        fn from(was: ValueWithoutUnit) -> Self {
            match was {
                ValueWithoutUnit::WithoutError(x) => x,
                #[cfg(feature = "error_propagation")]
                ValueWithoutUnit::WithError(x) => x.into(),
            }
        }
    }
    #[cfg(feature = "dimensional_analysis")]
    impl From<ValueWithUnit> for f32 {
        fn from(was: ValueWithUnit) -> Self {
            match was {
                ValueWithUnit::WithoutError(x) => x.into(),
                #[cfg(feature = "error_propagation")]
                ValueWithUnit::WithError(x) => x.into(),
            }
        }
    }
    impl From<ValueWithoutError> for f32 {
        fn from(was: ValueWithoutError) -> Self {
            match was {
                ValueWithoutError::WithoutUnit(x) => x,
                #[cfg(feature = "dimensional_analysis")]
                ValueWithoutError::WithUnit(x) => x.into(),
            }
        }
    }
    #[cfg(feature = "error_propagation")]
    impl From<ValueWithError> for f32 {
        fn from(was: ValueWithError) -> Self {
            match was {
                ValueWithError::WithoutUnit(x) => x.into(),
                #[cfg(feature = "dimensional_analysis")]
                ValueWithError::WithUnit(x) => x.into(),
            }
        }
    }
    impl From<Value> for f32 {
        fn from(was: Value) -> Self {
            match was {
                Value::WithoutUnit(x) => x.into(),
                #[cfg(feature = "dimensional_analysis")]
                Value::WithUnit(x) => x.into(),
            }
        }
    }
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
}
pub use value::*;
