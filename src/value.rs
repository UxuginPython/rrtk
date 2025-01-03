use super::*;
use core::fmt;
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
}
pub use value::*;
