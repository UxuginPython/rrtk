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
pub mod constants;
pub use constants::*;
///A time in nanoseconds.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
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
//TODO: figure out for to use the Error enum with this
impl TryFrom<Quantity> for Time {
    type Error = ();
    fn try_from(was: Quantity) -> Result<Self, ()> {
        if was.unit.eq_assume_false(&SECOND) {
            Ok(Self((was.value * 1_000_000_000.0) as i64))
        } else {
            Err(())
        }
    }
}
impl From<Time> for Quantity {
    fn from(was: Time) -> Quantity {
        Quantity::new(was.0 as f32 / 1_000_000_000.0, SECOND)
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
impl Neg for Time {
    type Output = Self;
    fn neg(self) -> Self {
        Self(-self.0)
    }
}
impl Mul<DimensionlessInteger> for Time {
    type Output = Self;
    fn mul(self, rhs: DimensionlessInteger) -> Self {
        Self(self.0 * rhs.0)
    }
}
impl Div<DimensionlessInteger> for Time {
    type Output = Self;
    fn div(self, rhs: DimensionlessInteger) -> Self {
        Self(self.0 / rhs.0)
    }
}
///A dimensionless quantity stored as an integer. Used almost exclusively for when a time, stored
///as an integer, must be multiplied by a constant factor as in numerical integrals and motion
///profiles.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct DimensionlessInteger(pub i64);
impl DimensionlessInteger {
    ///Constructor for `DimensionlessInteger`.
    pub const fn new(value: i64) -> Self {
        Self(value)
    }
}
impl From<i64> for DimensionlessInteger {
    fn from(was: i64) -> Self {
        Self(was)
    }
}
impl From<DimensionlessInteger> for i64 {
    fn from(was: DimensionlessInteger) -> Self {
        was.0
    }
}
impl TryFrom<Quantity> for DimensionlessInteger {
    type Error = ();
    fn try_from(was: Quantity) -> Result<Self, ()> {
        if was.unit.eq_assume_true(&DIMENSIONLESS) {
            Ok(Self(was.value as i64))
        } else {
            Err(())
        }
    }
}
impl From<DimensionlessInteger> for Quantity {
    fn from(was: DimensionlessInteger) -> Self {
        Quantity::new(was.0 as f32, DIMENSIONLESS)
    }
}
impl Add for DimensionlessInteger {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
    }
}
impl Sub for DimensionlessInteger {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0)
    }
}
impl Mul for DimensionlessInteger {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self(self.0 * rhs.0)
    }
}
impl Div for DimensionlessInteger {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        Self(self.0 / rhs.0)
    }
}
impl Neg for DimensionlessInteger {
    type Output = Self;
    fn neg(self) -> Self {
        Self(-self.0)
    }
}
impl Mul<Time> for DimensionlessInteger {
    type Output = Time;
    fn mul(self, rhs: Time) -> Time {
        Time(self.0 * rhs.0)
    }
}
impl Div<Time> for DimensionlessInteger {
    type Output = Quantity;
    fn div(self, rhs: Time) -> Quantity {
        Quantity::from(self) / Quantity::from(rhs)
    }
}
///A unit of a quantity, like meters per second. Units can be represented as multiplied powers of
///the units that they're derived from, so meters per second squared, or m/s^2, can be m^1*s^-2.
///This struct stores the exponents of each base unit.
#[derive(Clone, Copy, Debug)]
#[cfg_attr(
    any(
        feature = "dim_check_release",
        all(debug_assertions, feature = "dim_check_debug")
    ),
    derive(PartialEq, Eq)
)]
pub struct Unit {
    ///Unit exponent for millimeters.
    #[cfg(any(
        feature = "dim_check_release",
        all(debug_assertions, feature = "dim_check_debug")
    ))]
    millimeter_exp: i8,
    ///Unit exponent for seconds.
    #[cfg(any(
        feature = "dim_check_release",
        all(debug_assertions, feature = "dim_check_debug")
    ))]
    second_exp: i8,
}
impl Unit {
    ///Constructor for `Unit`.
    #[allow(unused)]
    pub const fn new(millimeter_exp: i8, second_exp: i8) -> Self {
        Self {
            #[cfg(any(
                feature = "dim_check_release",
                all(debug_assertions, feature = "dim_check_debug")
            ))]
            millimeter_exp: millimeter_exp,
            #[cfg(any(
                feature = "dim_check_release",
                all(debug_assertions, feature = "dim_check_debug")
            ))]
            second_exp: second_exp,
        }
    }
    ///`foo.const_eq(&bar)` works exactly like `foo == bar` except that it works in a `const`
    ///context. Requires dimension checking to be enabled. Use `eq_assume_true` or
    ///`eq_assume_false` if you need similar functionality without dimension checking.
    #[cfg(any(
        feature = "dim_check_release",
        all(debug_assertions, feature = "dim_check_debug")
    ))]
    #[allow(unused)]
    pub const fn const_eq(&self, rhs: &Self) -> bool {
        #[cfg(any(
            feature = "dim_check_release",
            all(debug_assertions, feature = "dim_check_debug")
        ))]
        return self.millimeter_exp == rhs.millimeter_exp && self.second_exp == rhs.second_exp;
        #[cfg(not(any(
            feature = "dim_check_release",
            all(debug_assertions, feature = "dim_check_debug")
        )))]
        true
    }
    ///`foo.const_assert_eq(&bar)` works exactly like `assert_eq!(foo, bar)` except that it works
    ///in a `const` context. Requires dimension checking to be enabled. Use `assert_eq_assume_ok`
    ///or `assert_eq_assume_not_ok` if you need similar functionality without dimension checking.
    #[cfg(any(
        feature = "dim_check_release",
        all(debug_assertions, feature = "dim_check_debug")
    ))]
    pub const fn const_assert_eq(&self, rhs: &Self) {
        assert!(self.const_eq(rhs));
    }
    ///With dimension checking on, behaves exactly like `const_eq`.
    ///With dimension checking off, always returns true.
    #[allow(unused)]
    pub const fn eq_assume_true(&self, rhs: &Self) -> bool {
        #[cfg(any(
            feature = "dim_check_release",
            all(debug_assertions, feature = "dim_check_debug")
        ))]
        return self.const_eq(rhs);
        #[cfg(not(any(
            feature = "dim_check_release",
            all(debug_assertions, feature = "dim_check_debug")
        )))]
        true
    }
    ///With dimension checking on, behaves exactly like `const_eq`.
    ///With dimension checking off, always returns false.
    #[allow(unused)]
    pub const fn eq_assume_false(&self, rhs: &Self) -> bool {
        #[cfg(any(
            feature = "dim_check_release",
            all(debug_assertions, feature = "dim_check_debug")
        ))]
        return self.const_eq(rhs);
        #[cfg(not(any(
            feature = "dim_check_release",
            all(debug_assertions, feature = "dim_check_debug")
        )))]
        false
    }
    ///With dimension checking on, behaves exactly like `const_assert_eq`.
    ///With dimension checking off, never panics.
    pub const fn assert_eq_assume_ok(&self, rhs: &Self) {
        assert!(self.eq_assume_true(rhs))
    }
    ///With dimension checking on, behaves exactly like `const_assert_eq`.
    ///With dimension checking off, always panics.
    pub const fn assert_eq_assume_not_ok(&self, rhs: &Self) {
        assert!(self.eq_assume_false(rhs))
    }
}
impl From<PositionDerivative> for Unit {
    #[allow(unused)]
    fn from(was: PositionDerivative) -> Self {
        #[cfg(any(
            feature = "dim_check_release",
            all(debug_assertions, feature = "dim_check_debug")
        ))]
        return Self {
            millimeter_exp: 1,
            second_exp: match was {
                PositionDerivative::Position => 0,
                PositionDerivative::Velocity => 1,
                PositionDerivative::Acceleration => 2,
            },
        };
        #[cfg(not(any(
            feature = "dim_check_release",
            all(debug_assertions, feature = "dim_check_debug")
        )))]
        Self {}
    }
}
impl TryFrom<MotionProfilePiece> for Unit {
    type Error = ();
    fn try_from(was: MotionProfilePiece) -> Result<Self, ()> {
        let pos_der: PositionDerivative = was.try_into()?;
        let unit: Self = pos_der.into();
        Ok(unit)
    }
}
///The `Add` implementation for `Unit` acts like you are trying to add quantities of the unit, not
///like you are trying to actually add the exponents. This should be more useful most of the time,
///but could be somewhat confusing. All this does is `assert_eq!` the `Unit` with the right-hand
///side and then return it because units should not change when quantities of the same unit are
///added.
impl Add for Unit {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        self.assert_eq_assume_ok(&rhs);
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
        self.assert_eq_assume_ok(&rhs);
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
    #[allow(unused)]
    fn mul(self, rhs: Self) -> Self {
        #[cfg(any(
            feature = "dim_check_release",
            all(debug_assertions, feature = "dim_check_debug")
        ))]
        return Self {
            millimeter_exp: self.millimeter_exp + rhs.millimeter_exp,
            second_exp: self.second_exp + rhs.second_exp,
        };
        #[cfg(not(any(
            feature = "dim_check_release",
            all(debug_assertions, feature = "dim_check_debug")
        )))]
        Self {}
    }
}
///The `Div` implementation for `Unit` acts like you are trying to divide quantities of the unit,
///not like you are trying to actually divide the exponents. This should be more useful most of the
///time, but it could be somewhat confusing. This subtracts the exponents of the right-hand side
///from the left-hand side's exponents rather than dividing the exponents because that is what
///should happen when quantities are divided, not a division of their unit exponents.
impl Div for Unit {
    type Output = Self;
    #[allow(unused)]
    fn div(self, rhs: Self) -> Self {
        #[cfg(any(
            feature = "dim_check_release",
            all(debug_assertions, feature = "dim_check_debug")
        ))]
        return Self {
            millimeter_exp: self.millimeter_exp - rhs.millimeter_exp,
            second_exp: self.second_exp - rhs.second_exp,
        };
        #[cfg(not(any(
            feature = "dim_check_release",
            all(debug_assertions, feature = "dim_check_debug")
        )))]
        Self {}
    }
}
///A quantity with a unit.
#[derive(Clone, Copy, Debug)]
#[cfg_attr(
    any(
        feature = "dim_check_release",
        all(debug_assertions, feature = "dim_check_debug")
    ),
    derive(PartialEq)
)]
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
    ///Constructor for dimensionless `Quantity` objects that does not require a dimension to be
    ///provided.
    pub const fn dimensionless(value: f32) -> Self {
        Self::new(value, DIMENSIONLESS)
    }
    ///Take the absolute value of the quantity.
    #[inline]
    pub fn abs(self) -> Self {
        Self::new(
            #[cfg(feature = "std")]
            self.value.abs(),
            #[cfg(not(feature = "std"))]
            if self.value >= 0.0 {
                self.value
            } else {
                -self.value
            },
            self.unit,
        )
    }
}
impl From<Command> for Quantity {
    fn from(was: Command) -> Self {
        match was {
            Command::Position(pos) => Self::new(pos, MILLIMETER),
            Command::Velocity(vel) => Self::new(vel, MILLIMETER_PER_SECOND),
            Command::Acceleration(acc) => Self::new(acc, MILLIMETER_PER_SECOND_SQUARED),
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
impl Neg for Quantity {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            value: -self.value,
            unit: self.unit,
        }
    }
}
impl Mul<Time> for Quantity {
    type Output = Self;
    fn mul(self, rhs: Time) -> Self {
        self * Quantity::from(rhs)
    }
}
impl Div<Time> for Quantity {
    type Output = Self;
    fn div(self, rhs: Time) -> Self {
        self / Quantity::from(rhs)
    }
}
#[cfg(not(any(
    feature = "dim_check_release",
    all(debug_assertions, feature = "dim_check_debug")
)))]
impl PartialEq for Quantity {
    fn eq(&self, rhs: &Self) -> bool {
        self.unit.assert_eq_assume_ok(&rhs.unit);
        self.value == rhs.value
    }
}
impl PartialOrd for Quantity {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.unit.assert_eq_assume_ok(&other.unit);
        self.value.partial_cmp(&other.value)
    }
}
