// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2024 UxuginPython
use crate::*;
///A one-dimensional motion state with position, velocity, and acceleration.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct State {
    ///Where you are. This should be in millimeters.
    pub position: f32,
    ///How fast you're going. This should be in millimeters per second.
    pub velocity: f32,
    ///How fast how fast you're going's changing. This should be in millimeters per second squared.
    pub acceleration: f32,
}
impl State {
    ///Constructor for [`State`] using [`Quantity`] objects for position, velocity, and acceleration.
    pub const fn new(position: Quantity, velocity: Quantity, acceleration: Quantity) -> Self {
        position.unit.assert_eq_assume_ok(&MILLIMETER);
        velocity.unit.assert_eq_assume_ok(&MILLIMETER_PER_SECOND);
        acceleration
            .unit
            .assert_eq_assume_ok(&MILLIMETER_PER_SECOND_SQUARED);
        State {
            position: position.value,
            velocity: velocity.value,
            acceleration: acceleration.value,
        }
    }
    ///Constructor for [`State`] using raw [`f32`]s for position, velocity, and acceleration.
    pub const fn new_raw(position: f32, velocity: f32, acceleration: f32) -> Self {
        State {
            position: position,
            velocity: velocity,
            acceleration: acceleration,
        }
    }
    ///Calculate the future state assuming a constant acceleration.
    pub fn update(&mut self, delta_time: Time) {
        let delta_time = Quantity::from(delta_time);
        let old_acceleration = self.get_acceleration();
        let old_velocity = self.get_velocity();
        let old_position = self.get_position();
        let new_velocity = old_velocity + delta_time * old_acceleration;
        let new_position = old_position
            + delta_time * (old_velocity + new_velocity) / Quantity::dimensionless(2.0);
        self.position = new_position.value;
        self.velocity = new_velocity.value;
    }
    ///Set the acceleration with a [`Quantity`]. With dimension checking enabled, sets the
    ///acceleration and returns [`Ok`] if the argument's [`Unit`] is correct, otherwise leaves it
    ///unchanged and returns [`Err`]. With dimension checking disabled, always sets the acceleration
    ///to the [`Quantity`]'s value and returns [`Ok`], ignoring the [`Unit`].
    pub const fn set_constant_acceleration(&mut self, acceleration: Quantity) -> Result<(), ()> {
        if acceleration
            .unit
            .eq_assume_true(&MILLIMETER_PER_SECOND_SQUARED)
        {
            self.acceleration = acceleration.value;
            Ok(())
        } else {
            Err(())
        }
    }
    ///Set the acceleration with an [`f32`] of millimeters per second squared.
    #[inline]
    pub const fn set_constant_acceleration_raw(&mut self, acceleration: f32) {
        self.acceleration = acceleration;
    }
    ///Set the velocity to a given value with a [`Quantity`], and set acceleration to zero. With
    ///dimension checking enabled, sets the velocity and acceleration and returns [`Ok`] if the
    ///argument's [`Unit`] is correct, otherwise leaves them unchanged and returns [`Err`]. With
    ///dimension checking disabled, ignores the [`Unit`] and always sets velocity and acceleration
    ///and returns [`Ok`].
    pub const fn set_constant_velocity(&mut self, velocity: Quantity) -> Result<(), ()> {
        if velocity.unit.eq_assume_true(&MILLIMETER_PER_SECOND) {
            self.acceleration = 0.0;
            self.velocity = velocity.value;
            Ok(())
        } else {
            Err(())
        }
    }
    ///Set the velocity to a given value with an [`f32`] of millimeters per second, and set acceleration to zero.
    #[inline]
    pub const fn set_constant_velocity_raw(&mut self, velocity: f32) {
        self.acceleration = 0.0;
        self.velocity = velocity;
    }
    ///Set the position to a given value with a [`Quantity`], and set velocity and acceleration to
    ///zero. With dimension checking enabled, sets the position, velocity, and acceleration and
    ///returns [`Ok`] if the argument's [`Unit`] is correct, otherwise leaves them unchanged and
    ///returns [`Err`]. With dimension checking disabled, always sets the position, velocity, and
    ///acceleration and returns [`Ok`], ignoring the [`Unit`].
    pub const fn set_constant_position(&mut self, position: Quantity) -> Result<(), ()> {
        if position.unit.eq_assume_true(&MILLIMETER) {
            self.acceleration = 0.0;
            self.velocity = 0.0;
            self.position = position.value;
            Ok(())
        } else {
            Err(())
        }
    }
    ///Set the position to a given value with an [`f32`] of millimeters, and set velocity and acceleration to zero.
    #[inline]
    pub const fn set_constant_position_raw(&mut self, position: f32) {
        self.acceleration = 0.0;
        self.velocity = 0.0;
        self.position = position;
    }
    ///Get the position as a [`Quantity`].
    #[inline]
    pub const fn get_position(&self) -> Quantity {
        Quantity::new(self.position, MILLIMETER)
    }
    ///Get the velocity as a [`Quantity`].
    #[inline]
    pub const fn get_velocity(&self) -> Quantity {
        Quantity::new(self.velocity, MILLIMETER_PER_SECOND)
    }
    ///Get the acceleration as a [`Quantity`].
    #[inline]
    pub const fn get_acceleration(&self) -> Quantity {
        Quantity::new(self.acceleration, MILLIMETER_PER_SECOND_SQUARED)
    }
    ///State contains a position, velocity, and acceleration. This gets the respective field of a
    ///given position derivative.
    pub fn get_value(&self, position_derivative: PositionDerivative) -> Quantity {
        match position_derivative {
            PositionDerivative::Position => self.get_position(),
            PositionDerivative::Velocity => self.get_velocity(),
            PositionDerivative::Acceleration => self.get_acceleration(),
        }
    }
}
impl Neg for State {
    type Output = Self;
    fn neg(self) -> Self {
        State::new_raw(-self.position, -self.velocity, -self.acceleration)
    }
}
impl Add for State {
    type Output = Self;
    fn add(self, other: State) -> Self {
        State::new_raw(
            self.position + other.position,
            self.velocity + other.velocity,
            self.acceleration + other.acceleration,
        )
    }
}
impl Sub for State {
    type Output = Self;
    fn sub(self, other: State) -> Self {
        State::new_raw(
            self.position - other.position,
            self.velocity - other.velocity,
            self.acceleration - other.acceleration,
        )
    }
}
impl Mul<f32> for State {
    type Output = Self;
    fn mul(self, coef: f32) -> Self {
        State::new_raw(
            self.position * coef,
            self.velocity * coef,
            self.acceleration * coef,
        )
    }
}
impl Div<f32> for State {
    type Output = Self;
    fn div(self, dvsr: f32) -> Self {
        State::new_raw(
            self.position / dvsr,
            self.velocity / dvsr,
            self.acceleration / dvsr,
        )
    }
}
impl AddAssign for State {
    fn add_assign(&mut self, other: State) {
        *self = *self + other;
    }
}
impl SubAssign for State {
    fn sub_assign(&mut self, other: State) {
        *self = *self - other;
    }
}
impl MulAssign<f32> for State {
    fn mul_assign(&mut self, coef: f32) {
        *self = *self * coef;
    }
}
impl DivAssign<f32> for State {
    fn div_assign(&mut self, dvsr: f32) {
        *self = *self / dvsr;
    }
}
