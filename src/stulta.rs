// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2024-2026 UxuginPython
//!"The stupid things" (Latin). These items really shouldn't exist but currently must, mostly
//!either due to language limitations or a desire to minimize external dependencies.
use super::*;
///A trait for getting half of a number or other quantity object.
pub trait Half {
    ///Get half of the number.
    fn half(self) -> Self;
}
macro_rules! impl_half_integer {
    ($num: ty) => {
        impl Half for $num {
            fn half(self) -> Self {
                self / 2
            }
        }
    };
}
impl_half_integer!(u8);
impl_half_integer!(u16);
impl_half_integer!(u32);
impl_half_integer!(u64);
impl_half_integer!(u128);
impl_half_integer!(usize);
impl_half_integer!(i8);
impl_half_integer!(i16);
impl_half_integer!(i32);
impl_half_integer!(i64);
impl_half_integer!(i128);
impl_half_integer!(isize);
impl Half for f32 {
    fn half(self) -> Self {
        self / 2.0
    }
}
impl Half for f64 {
    fn half(self) -> Self {
        self / 2.0
    }
}
///A trait for getting the absolute value of a number or other quantity object.
pub trait AbsoluteValue {
    ///Get the absolute value.
    fn rrtk_abs(self) -> Self;
}
macro_rules! impl_abs {
    ($num: ty) => {
        impl AbsoluteValue for $num {
            fn rrtk_abs(self) -> Self {
                self.abs()
            }
        }
    };
}
impl_abs!(i8);
impl_abs!(i16);
impl_abs!(i32);
impl_abs!(i64);
impl_abs!(i128);
impl_abs!(isize);
impl_abs!(f32);
impl_abs!(f64);
///Specialization workaround. Implement for any type that is not [`Datum`] itself
///including types using `Datum` as a type parameter or associated type and references to `Datum`.
pub trait NotDatum {}
impl NotDatum for u8 {}
impl NotDatum for u16 {}
impl NotDatum for u32 {}
impl NotDatum for u64 {}
impl NotDatum for u128 {}
impl NotDatum for usize {}
impl NotDatum for i8 {}
impl NotDatum for i16 {}
impl NotDatum for i32 {}
impl NotDatum for i64 {}
impl NotDatum for i128 {}
impl NotDatum for isize {}
impl NotDatum for f32 {}
impl NotDatum for f64 {}
impl<T> NotDatum for Option<T> {}
impl<T, E> NotDatum for Result<T, E> {}
impl<T: ?Sized> NotDatum for core::cell::UnsafeCell<T> {}
impl<T: ?Sized> NotDatum for core::cell::Cell<T> {}
impl<T: ?Sized> NotDatum for core::cell::RefCell<T> {}
impl<T: ?Sized> NotDatum for &T {}
impl<T: ?Sized> NotDatum for &mut T {}
impl<T: ?Sized> NotDatum for *const T {}
impl<T: ?Sized> NotDatum for *mut T {}
impl<T, const N: usize> NotDatum for [T; N] {}
#[cfg(feature = "alloc")]
impl NotDatum for alloc::string::String {}
#[cfg(feature = "alloc")]
impl<T> NotDatum for Vec<T> {}
#[cfg(feature = "alloc")]
impl<T: ?Sized> NotDatum for Rc<T> {}
#[cfg(feature = "std")]
impl<T: ?Sized> NotDatum for Arc<T> {}
#[cfg(feature = "std")]
impl<T: ?Sized> NotDatum for Mutex<T> {}
#[cfg(feature = "std")]
impl<T: ?Sized> NotDatum for RwLock<T> {}
impl NotDatum for LinearState {}
impl NotDatum for AngularState {}
impl NotDatum for LinearCommand {}
impl NotDatum for AngularCommand {}
impl NotDatum for PositionDerivative {}
impl NotDatum for error::CannotConvert {}
impl NotDatum for Time {}
impl NotDatum for DimensionlessInteger {}
impl NotDatum for compile_time_integer::Zero {}
impl<T: compile_time_integer::Integer> NotDatum for compile_time_integer::OnePlus<T> {}
impl<T: compile_time_integer::Integer> NotDatum for compile_time_integer::NegativeOnePlus<T> {}
impl<T, MM, S> NotDatum for Quantity<T, MM, S>
where
    MM: compile_time_integer::Integer,
    S: compile_time_integer::Integer,
{
}
impl NotDatum for MotionProfilePiece {}
impl NotDatum for PIDKValues {}
impl NotDatum for PositionDerivativeDependentPIDKValues {}
