// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2024 UxuginPython
//!This module contains types related to RRTK's dimensional analysis system. RRTK uses nanoseconds
//!for time because they typically work nicely with computer clocks and are still precise when
//!stored in an integer, which is important because exponentially losing precision for time is bad,
//!and float time does that. However, floats are used for other quantities, including quantities
//!derived from time. These use seconds instead because numbers of the magnitude of nanoseconds
//!cause floats to lose precision. RRTK should handle the conversion mostly seamlessly for you, but
//!keep it in mind when thinking about how time-related types should work. The reasoning behind
//!this unorthodox system using both nanoseconds and seconds becomes more apparent when you know
//!how floating point numbers work. Everything in this module is reexported at the crate level.
//!
//!### Multiplication and Division Implementation Table
//!| A right; B down              | [`Quantity`]      | [`DimensionlessInteger`] | [`Time`]          |
//!|------------------------------|-------------------|--------------------------|-------------------|
//!| **[`Quantity`]**             | `*` `/` `*=` `/=` | `*` `/`                  | `*` `/`           |
//!| **[`DimensionlessInteger`]** | `*` `/` `*=` `/=` | `*` `/` `*=` `/=`        | `*` `/` `*=` `/=` |
//!| **[`Time`]**                 | `*` `/` `*=` `/=` | `*` `/`                  | `*` `/`           |
//!
//!`A <operation> B` compiles for any operation in the square of A and B. E.g., `*` is in the
//!square in the [`Quantity`] column and the [`DimensionlessInteger`] row, so the following works:
//!```
//!# use rrtk::*;
//!let x = Quantity::new(3.0, MILLIMETER);
//!let y = DimensionlessInteger(2);
//!let z = x * y;
//!```
//!A similar example for `*=`:
//!```
//!# use rrtk::*;
//!let mut x = Quantity::new(3.0, MILLIMETER);
//!let y = DimensionlessInteger(2);
//!x *= y;
//!```
//!Whenever `*` and `/` are in a square but `*=` and `/=` are not, `A * B` and `A / B`
//!return a type other than A. Since [`MulAssign`] and `DivAssign` require that A not change type in
//!`A *= B` and `A /= B`, it is not possible to implement them.
//!```
//!# use rrtk::*;
//!let x = Time(2_000_000_000);
//!let y = Quantity::new(3.0, MILLIMETER_PER_SECOND);
//!let z = x * y;
//!assert_eq!(z, Quantity::new(6.0, MILLIMETER));
//!```
//!```compile_fail
//!# use rrtk::*;
//!let mut x = Time(2_000_000_000);
//!let y = Quantity::new(3.0, MILLIMETER_PER_SECOND);
//!x *= y;
//!```
//!Note that this disparity is not necessarily symmetrical between types:
//!```
//!# use rrtk::*;
//!let mut x = Quantity::new(3.0, MILLIMETER_PER_SECOND);
//!let y = Time(2_000_000_000);
//!x *= y;
//!assert_eq!(x, Quantity::new(6.0, MILLIMETER));
//!```
//!### Addition and Subtraction Implementation Table
//!| A right; B down              | [`Quantity`]             | [`DimensionlessInteger`] | [`Time`]                 |
//!|------------------------------|--------------------------|--------------------------|--------------------------|
//!| **[`Quantity`]**             | **P:** `+` `-` `+=` `-=` | **P:** `+` `-`           | **P:** `+` `-`           |
//!| **[`DimensionlessInteger`]** | **P:** `+` `-` `+=` `-=` | **G:** `+` `-` `+=` `-=` |                          |
//!| **[`Time`]**                 | **P:** `+` `-` `+=` `-=` |                          | **G:** `+` `-` `+=` `-=` |
//!
//!Addition and subtraction are a bit different because they can sometimes panic on a unit
//!mismatch. This table works the same way as the one above it except for the following:
//!- **P(anicking):** This operation may panic on a unit mismatch.
//The panic!() at the end of this example is so that it panics even when dimension checking is off.
//Cargo runs this with the other tests and, since it it marked should_panic, fails if it does not
//panic. This is a problem because it cannot panic with dimension checking off. A panic!() call at
//the end is the simplest way to ensure that this is not an issue, although it does eliminate the
//usefulness of this as a test. It is tested elsewhere, however; use quantity_add_failure in
//tests/dimensions.rs to test the panicking functionality.
//!```should_panic
//!# use rrtk::*;
//!let x = Quantity::new(2.0, MILLIMETER);
//!let y = Quantity::new(3.0, SECOND);
//!let z = x + y;
//!# panic!();
//!```
//!- **G(uaranteed):** Correct units are guaranteed by the types involved. This operation cannot panic.
//!
//!All operations in the multiplication and division table can be considered "Guaranteed."
//!### Conversion Implementation Table
//!| A right; B down              | [`Quantity`] | [`DimensionlessInteger`] | [`Time`]  | [`i64`] | [`f32`] |
//!|------------------------------|--------------|--------------------------|-----------|---------|---------|
//!| **[`Quantity`]**             | *is*         | `TryFrom`                | `TryFrom` |         | `From`  |
//!| **[`DimensionlessInteger`]** | `From`       | *is*                     |           | `From`  |         |
//!| **[`Time`]**                 | `From`       |                          | *is*      | `From`  |         |
//!| **[`i64`]**                  |              | `From`                   | `From`    | *is*    | [^lang] |
//!| **[`f32`]**                  | [^new]       |                          |           | [^lang] | *is*    |
//!
//![^lang]: See Rust language documentation.
//!
//![^new]: [`Quantity`] can be constructed from [`f32`] through [`Quantity::new`] by supplying a [`Unit`].
//!However, [`f32`] cannot be directly converted to [`Quantity`].
//!
//!This table is very similar: `A::<from/try_from>(B)` compiles for either `from`
//!or `try_from` depending on which is in the square of A and B, and you cannot convert between
//!types with nothing in their square. A [`From`] B implies B [`Into`] A and similarly for
//![`TryFrom`]/[`TryInto`] as is the case for all [`From`] implementations.
//!
//![`From`] is in the [`Quantity`] column and the [`DimensionlessInteger`] row, so the following works:
//!```
//!# use rrtk::*;
//!let x = DimensionlessInteger(3);
//!let y = Quantity::from(x);
//!```
//!And with [`Into`]:
//!```
//!# use rrtk::*;
//!let x = DimensionlessInteger(3);
//!let y: Quantity = x.into();
//!```
use super::*;
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
macro_rules! impl_assign_for_superior {
    ($assign_trait: ident, $rhs: ident, $name: ident, $assign_func: ident, $op_symbol: tt) => {
        impl $assign_trait<$rhs> for $name {
            fn $assign_func(&mut self, rhs: $rhs) {
                *self = *self $op_symbol rhs;
            }
        }
    }
}
macro_rules! impl_all_assign_for_superior {
    ($name: ident, $rhs: ident) => {
        impl_assign_for_superior!(AddAssign, $rhs, $name, add_assign, +);
        impl_assign_for_superior!(SubAssign, $rhs, $name, sub_assign, -);
        impl_assign_for_superior!(MulAssign, $rhs, $name, mul_assign, *);
        impl_assign_for_superior!(DivAssign, $rhs, $name, div_assign, /);
    }
}
macro_rules! impl_all_ops_with_assign_for_superior {
    ($name: ident, $rhs: ident) => {
        impl_op_for_superior!(Add, $rhs, $name, add, +);
        impl_op_for_superior!(Sub, $rhs, $name, sub, -);
        impl_op_for_superior!(Mul, $rhs, $name, mul, *);
        impl_op_for_superior!(Div, $rhs, $name, div, /);
        impl_all_assign_for_superior!($name, $rhs);
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
pub mod constants;
pub use constants::*;
#[cfg(feature = "error_propagation")]
mod value_without_unit_with_error;
#[cfg(feature = "error_propagation")]
pub use value_without_unit_with_error::*;
mod f32_impls;
//pub use f32_impls::*; is not needed because there are only implementations there and not items.
mod time;
pub use time::*;
mod dimensionless_integer;
pub use dimensionless_integer::*;
mod unit;
pub use unit::*;
mod quantity;
pub use quantity::*;
pub type ValueWithoutUnitWithoutError = f32;
//see reference module for why this is non_exhaustive
#[non_exhaustive]
pub enum ValueWithoutUnit {
    WithoutError(f32),
    #[cfg(feature = "error_propagation")]
    WithError(ValueWithoutUnitWithError),
}
impl Add<f32> for ValueWithoutUnit {
    type Output = Self;
    fn add(self, rhs: f32) -> Self {
        match self {
            Self::WithoutError(x) => Self::WithoutError(x + rhs),
            #[cfg(feature = "error_propagation")]
            Self::WithError(x) => Self::WithError(x + rhs),
        }
    }
}
