// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2024 UxuginPython
use super::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct Time(pub i64);
impl Time {
    pub fn new(value: i64) -> Self {
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
        Quantity::new((self.0 * rhs.0) as f32, 0, 2)
    }
}
impl Div for Time {
    type Output = Quantity;
    fn div(self, rhs: Self) -> Quantity {
        Quantity::new((self.0 / rhs.0) as f32, 0, 0)
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Quantity {
    pub value: f32,
    pub millimeter_exp: i8,
    pub second_exp: i8,
}
impl Quantity {
    pub fn new(value: f32, millimeter_exp: i8, second_exp: i8) -> Self {
        Self {
            value: value,
            millimeter_exp: millimeter_exp,
            second_exp: second_exp,
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
        assert_eq!(self.millimeter_exp, rhs.millimeter_exp);
        assert_eq!(self.second_exp, rhs.second_exp);
        Self {
            value: self.value + rhs.value,
            millimeter_exp: self.millimeter_exp,
            second_exp: self.second_exp,
        }
    }
}
impl Sub for Quantity {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        assert_eq!(self.millimeter_exp, rhs.millimeter_exp);
        assert_eq!(self.second_exp, rhs.second_exp);
        Self {
            value: self.value - rhs.value,
            millimeter_exp: self.millimeter_exp,
            second_exp: self.second_exp,
        }
    }
}
impl Mul for Quantity {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self {
            value: self.value * rhs.value,
            millimeter_exp: self.millimeter_exp + rhs.millimeter_exp,
            second_exp: self.second_exp + rhs.second_exp,
        }
    }
}
impl Div for Quantity {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        Self {
            value: self.value / rhs.value,
            millimeter_exp: self.millimeter_exp - rhs.millimeter_exp,
            second_exp: self.second_exp - rhs.second_exp,
        }
    }
}
impl Mul<Time> for Quantity {
    type Output = Self;
    fn mul(self, rhs: Time) -> Self {
        Self {
            value: self.value * rhs.0 as f32,
            millimeter_exp: self.millimeter_exp,
            second_exp: self.second_exp + 1,
        }
    }
}
impl Div<Time> for Quantity {
    type Output = Self;
    fn div(self, rhs: Time) -> Self {
        Self {
            value: self.value / rhs.0 as f32,
            millimeter_exp: self.millimeter_exp,
            second_exp: self.second_exp - 1,
        }
    }
}
