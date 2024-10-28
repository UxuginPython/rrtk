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
use super::*;
///A time in nanoseconds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct Time(pub i64);
impl Time {
    ///The constructor for `Time`.
    pub const fn new(value: i64) -> Self {
        Self(value)
    }
}
impl From<i64> for Time {
    fn from(was: i64) -> Self {
        Self(was)
    }
}
impl From<Time> for i64 {
    fn from(was: Time) -> i64 {
        was.0
    }
}
impl Add for Time {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
    }
}
impl Sub for Time {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0)
    }
}
impl Mul for Time {
    type Output = Quantity;
    fn mul(self, rhs: Self) -> Quantity {
        Quantity::new(
            (self.0 as f32 / 1_000_000_000.0) * (rhs.0 as f32 / 1_000_000_000.0),
            SECOND_SQUARED,
        )
    }
}
impl Div for Time {
    type Output = Quantity;
    fn div(self, rhs: Self) -> Quantity {
        Quantity::new(
            (self.0 as f32 / 1_000_000_000.0) / (rhs.0 as f32 / 1_000_000_000.0),
            DIMENSIONLESS,
        )
    }
}
///A unit of a quantity, like meters per second. Units can be represented as multiplied powers of
///the units that they're derived from, so meters per second squared, or m/s^2, can be m^1*s^-2.
///This struct stores the exponents of each base unit.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Unit {
    ///Unit exponent for millimeters.
    pub millimeter_exp: i8,
    ///Unit exponent for seconds.
    pub second_exp: i8,
}
impl Unit {
    ///Constructor for `Unit`.
    pub const fn new(millimeter_exp: i8, second_exp: i8) -> Self {
        Self {
            millimeter_exp: millimeter_exp,
            second_exp: second_exp,
        }
    }
}
//TODO: Document these really, really well. How they work is confusing.
///The `Add` implementation for `Unit` acts like you are trying to add quantities of the unit, not
///like you are trying to actually add the exponents. This should be more useful most of the time,
///but could be somewhat confusing. All this does is `assert_eq!` the `Unit` with the right-hand
///side and then return it because units should not change when quantities of the same unit are
///added.
impl Add for Unit {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        assert_eq!(self, rhs);
        self
    }
}
///The `Sub` implementation for `Unit` acts like you are trying to subtract quantities of the unit,
///not like you are trying to actually subtract the exponents. This should be more useful most of
///the time, but it could be somewhat confusing. All this does is `assert_eq!` the `Unit` with the
///right-hand side and then return it because units should not change when quantities of the same
///unit are subtracted.
impl Sub for Unit {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        assert_eq!(self, rhs);
        self
    }
}
///The `Mul` implementation for `Unit` acts like you are trying to multiply quantities of the unit,
///not like you are trying to actually multiply the exponents. This should be more useful most of
///the time, but it could be somewhat confusing. This adds the exponents of the left-hand and
///right-hand sides, not multiplies them because that is what should happen when quantities are
///multiplied, not a multiplication of their unit exponents.
impl Mul for Unit {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self {
            millimeter_exp: self.millimeter_exp + rhs.millimeter_exp,
            second_exp: self.second_exp + rhs.second_exp,
        }
    }
}
///The `Div` implementation for `Unit` acts like you are trying to divide quantities of the unit,
///not like you are trying to actually divide the exponents. This should be more useful most of the
///time, but it could be somewhat confusing. This subtracts the exponents of the right-hand side
///from the left-hand side's exponents rather than dividing the exponents because that is what
///should happen when quantities are divided, not a division of their unit exponents.
impl Div for Unit {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        Self {
            millimeter_exp: self.millimeter_exp - rhs.millimeter_exp,
            second_exp: self.second_exp - rhs.second_exp,
        }
    }
}
///The `Unit` for a dimensionless quantity.
pub const DIMENSIONLESS: Unit = Unit::new(0, 0);
///The `Unit` for a distance in millimeters.
pub const MILLIMETER: Unit = Unit::new(1, 0);
///The `Unit` for a velocity in millimeters per second.
pub const MILLIMETER_PER_SECOND: Unit = Unit::new(1, -1);
///The `Unit` for an acceleration in millimeters per second squared.
pub const MILLIMETER_PER_SECOND_SQUARED: Unit = Unit::new(1, -2);
///The `Unit` for a time in seconds.
pub const SECOND: Unit = Unit::new(1, 0);
///The `Unit` for a quantity with units of seconds squared. This is mostly useless but is used for
///multiplication of `Time` objects.
pub const SECOND_SQUARED: Unit = Unit::new(0, 2);
///A quantity with a unit.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Quantity {
    ///The value.
    pub value: f32,
    ///The unit.
    pub unit: Unit,
}
impl Quantity {
    ///Constructor for `Quantity`.
    pub const fn new(value: f32, unit: Unit) -> Self {
        Self {
            value: value,
            unit: unit,
        }
    }
}
impl From<Quantity> for f32 {
    fn from(was: Quantity) -> f32 {
        was.value
    }
}
impl Add for Quantity {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            value: self.value + rhs.value,
            unit: self.unit + rhs.unit,
        }
    }
}
impl Sub for Quantity {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            value: self.value - rhs.value,
            unit: self.unit - rhs.unit,
        }
    }
}
impl Mul for Quantity {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self {
            value: self.value * rhs.value,
            unit: self.unit * rhs.unit,
        }
    }
}
impl Div for Quantity {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        Self {
            value: self.value / rhs.value,
            unit: self.unit / rhs.unit,
        }
    }
}
impl Mul<Time> for Quantity {
    type Output = Self;
    fn mul(self, rhs: Time) -> Self {
        Self {
            value: self.value * rhs.0 as f32,
            unit: Unit::new(self.unit.millimeter_exp, self.unit.second_exp + 1),
        }
    }
}
impl Div<Time> for Quantity {
    type Output = Self;
    fn div(self, rhs: Time) -> Self {
        Self {
            value: self.value / rhs.0 as f32,
            unit: Unit::new(self.unit.millimeter_exp, self.unit.second_exp - 1),
        }
    }
}
