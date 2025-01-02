use super::*;
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
    ///context. Requires dimension checking to be enabled. Use [`eq_assume_true`](Unit::eq_assume_true) or
    ///[`eq_assume_false`](Unit::eq_assume_false) if you need similar functionality without dimension checking.
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
    ///in a `const` context. Requires dimension checking to be enabled. Use
    ///[`assert_eq_assume_ok`](Unit::assert_eq_assume_ok)
    ///or [`assert_eq_assume_not_ok`](Unit::assert_eq_assume_not_ok) if you need similar functionality without
    ///dimension checking.
    #[cfg(any(
        feature = "dim_check_release",
        all(debug_assertions, feature = "dim_check_debug")
    ))]
    pub const fn const_assert_eq(&self, rhs: &Self) {
        assert!(self.const_eq(rhs));
    }
    ///With dimension checking on, behaves exactly like [`const_eq`](Unit::const_eq).
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
    ///With dimension checking on, behaves exactly like [`const_eq`](Unit::const_eq).
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
    ///With dimension checking on, behaves exactly like [`const_assert_eq`](Unit::const_assert_eq).
    ///With dimension checking off, never panics.
    pub const fn assert_eq_assume_ok(&self, rhs: &Self) {
        assert!(self.eq_assume_true(rhs))
    }
    ///With dimension checking on, behaves exactly like [`const_assert_eq`](Unit::const_assert_eq).
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
                PositionDerivative::Velocity => -1,
                PositionDerivative::Acceleration => -2,
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
///The [`Add`] implementation for [`Unit`] acts like you are trying to add quantities of the unit, not
///like you are trying to actually add the exponents. This should be more useful most of the time,
///but could be somewhat confusing. All this does is [`assert_eq!`] the [`Unit`] with the right-hand
///side and then return it because units should not change when quantities of the same unit are
///added.
///Performing operations on [`Unit`]s should behave exactly the same as performing the same
///operations on [`Quantity`] objects and taking the unit of the resulting [`Quantity`].
impl Add for Unit {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        self.assert_eq_assume_ok(&rhs);
        self
    }
}
impl AddAssign for Unit {
    fn add_assign(&mut self, rhs: Self) {
        self.assert_eq_assume_ok(&rhs);
    }
}
///The [`Sub`] implementation for [`Unit`] acts like you are trying to subtract quantities of the unit,
///not like you are trying to actually subtract the exponents. This should be more useful most of
///the time, but it could be somewhat confusing. All this does is [`assert_eq!`] the [`Unit`] with the
///right-hand side and then return it because units should not change when quantities of the same
///unit are subtracted.
///Performing operations on [`Unit`]s should behave exactly the same as performing the same
///operations on [`Quantity`] objects and taking the unit of the resulting [`Quantity`].
impl Sub for Unit {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        self.assert_eq_assume_ok(&rhs);
        self
    }
}
impl SubAssign for Unit {
    fn sub_assign(&mut self, rhs: Self) {
        self.assert_eq_assume_ok(&rhs);
    }
}
///The [`Mul`] implementation for [`Unit`] acts like you are trying to multiply quantities of the unit,
///not like you are trying to actually multiply the exponents. This should be more useful most of
///the time, but it could be somewhat confusing. This adds the exponents of the left-hand and
///right-hand sides, not multiplies them because that is what should happen when quantities are
///multiplied, not a multiplication of their unit exponents.
///Performing operations on [`Unit`]s should behave exactly the same as performing the same
///operations on [`Quantity`] objects and taking the unit of the resulting [`Quantity`].
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
impl MulAssign for Unit {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}
///The [`Div`] implementation for [`Unit`] acts like you are trying to divide quantities of the unit,
///not like you are trying to actually divide the exponents. This should be more useful most of the
///time, but it could be somewhat confusing. This subtracts the exponents of the right-hand side
///from the left-hand side's exponents rather than dividing the exponents because that is what
///should happen when quantities are divided, not a division of their unit exponents.
///Performing operations on [`Unit`]s should behave exactly the same as performing the same
///operations on [`Quantity`] objects and taking the unit of the resulting [`Quantity`].
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
impl DivAssign for Unit {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}
///The [`Neg`] implementation for [`Unit`] acts like you are trying to negate quantities of the unit,
///not like you are trying to actually negate the exponents. This should be more useful most of the
///time, but could be somewhat confusing. This just returns `self` unchanged because a quantity's
///units don't change when it is negated.
///Performing operations on [`Unit`]s should behave exactly the same as performing the same
///operations on [`Quantity`] objects and taking the unit of the resulting [`Quantity`].
impl Neg for Unit {
    type Output = Self;
    fn neg(self) -> Self {
        self
    }
}
