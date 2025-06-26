// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2024-2025 UxuginPython
//!"The stupid things" (Latin). These items really shouldn't exist but currently must, mostly
//!either due to language limitations or a desire to minimize external dependencies.
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
