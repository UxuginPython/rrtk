// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2024-2025 UxuginPython
use crate::*;
///This trait allows one to write code generically over [`LinearState`] and [`AngularState`]. Each of
///those has a corresponding method to each method of this trait without the `generic_` prefix.
///This is necessary because many of the implementations should be const fn and can't be in a
///trait. Calling the direct methods (without `generic_`) is preferred where possible.
///
///`generic_(position|velocity|acceleration)(_mut)?`, to use regex syntax, are a bit different:
///they correspond to fields rather than methods. If you are directly using a state object, it is
///recommended to use the `position`, `velocity`, and `acceleration` fields directly, but this is
///not possible when using `GenericState`. Thus, these methods exist to allow access to the fields.
///`generic_(position|velocity|acceleration)` get the values themselves, and their corresponding
///`_mut` methods get `&mut` references to allow mutating the values inside the state struct.
pub trait GenericState:
    Copy
    + Debug
    + Default
    + PartialEq
    + Neg<Output = Self>
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Dimensionless<f32>, Output = Self>
    + Div<Dimensionless<f32>, Output = Self>
    + AddAssign
    + SubAssign
    + MulAssign<Dimensionless<f32>>
    + DivAssign<Dimensionless<f32>>
{
    ///The type that the position is stored as. Almost certainly a [`Quantity`] of some type.
    type Position: Copy
        + Debug
        + Default
        + fmt::Display
        + PartialOrd
        + Add<Output = Self::Position>
        + Sub<Output = Self::Position>
        + Mul<Dimensionless<f32>, Output = Self::Position>
        + Div<Dimensionless<f32>, Output = Self::Position>
        + Div<Time, Output = Self::Velocity>
        + stulta::AbsoluteValue;
    ///The type that the velocity is stored as. Almost certainly a [`Quantity`] of some type.
    type Velocity: Copy
        + Debug
        + Default
        + fmt::Display
        + PartialOrd
        + Add<Output = Self::Velocity>
        + Sub<Output = Self::Velocity>
        + Mul<Dimensionless<f32>, Output = Self::Velocity>
        + Div<Dimensionless<f32>, Output = Self::Velocity>
        + Mul<Time, Output = Self::Position>
        + Div<Time, Output = Self::Acceleration>
        + stulta::AbsoluteValue;
    ///The type that the acceleration is stored as. Almost certainly a [`Quantity`] of some type.
    type Acceleration: Copy
        + Debug
        + Default
        + fmt::Display
        + PartialOrd
        + Add<Output = Self::Acceleration>
        + Sub<Output = Self::Acceleration>
        + Mul<Dimensionless<f32>, Output = Self::Acceleration>
        + Div<Dimensionless<f32>, Output = Self::Acceleration>
        + Mul<Time, Output = Self::Velocity>
        + stulta::AbsoluteValue;
    ///Constructor from a position, velocity, and acceleration.
    fn generic_new(
        position: Self::Position,
        velocity: Self::Velocity,
        acceleration: Self::Acceleration,
    ) -> Self;
    ///Calculate the future state assuming a constant acceleration. This is unrelated to
    ///[`Updatable`].
    fn generic_update(&mut self, delta_time: Time);
    ///Set the position to a given value and set the velocity and acceleration to zero.
    fn generic_set_constant_position(&mut self, position: Self::Position);
    ///Set the velocity to a given value and set the acceleration to zero.
    fn generic_set_constant_velocity(&mut self, velocity: Self::Velocity);
    ///Set the acceleration.
    fn generic_set_constant_acceleration(&mut self, acceleration: Self::Acceleration);
    ///States contain a position, velocity, and acceleration. This gets the respective field of a
    ///given position derivative.
    fn generic_get_value(&self, position_derivative: PositionDerivative) -> f32;
    ///Generically get the position value of the state.
    fn generic_position(&self) -> Self::Position;
    ///Generically get the velocity value of the state.
    fn generic_velocity(&self) -> Self::Velocity;
    ///Generically get the acceleration value of the state.
    fn generic_acceleration(&self) -> Self::Acceleration;
    ///Get a mutable reference to the position value of the state.
    fn generic_position_mut(&mut self) -> &mut Self::Position;
    ///Get a mutable reference to the velocity value of the state.
    fn generic_velocity_mut(&mut self) -> &mut Self::Velocity;
    ///Get a mutable reference to the acceleration value of the state.
    fn generic_acceleration_mut(&mut self) -> &mut Self::Acceleration;
}
macro_rules! build_state_struct {
    ($name: ident, $pos: ty, $vel: ty, $acc: ty) => {
        ///A one-dimensional motion state with position, velocity, and acceleration.
        #[derive(Clone, Copy, Debug, Default, PartialEq)]
        pub struct $name {
            ///Where you are. This should be in millimeters.
            pub position: $pos,
            ///How fast you're going. This should be in millimeters per second.
            pub velocity: $vel,
            ///How fast how fast you're going's changing. This should be in millimeters per second squared.
            pub acceleration: $acc,
        }
        impl $name {
            pub const ZERO: Self = Self::new(<$pos>::new(0.0), <$vel>::new(0.0), <$acc>::new(0.0));
            ///Constructor using [`Quantity`] objects for position, velocity, and acceleration.
            pub const fn new(position: $pos, velocity: $vel, acceleration: $acc) -> Self {
                $name {
                    position,
                    velocity,
                    acceleration,
                }
            }
            //This could maybe be const fn if you're willing to let the code get a bit messy, maybe give up a
            //slight bit of performance (or not depending on optimization), and give up some of the
            //dimension guarantees here.
            ///Calculate the future state assuming a constant acceleration. This is unrelated to
            ///[`Updatable`].
            pub fn update(&mut self, delta_time: Time) {
                let old_acceleration = self.acceleration;
                let old_velocity = self.velocity;
                let old_position = self.position;
                let new_velocity = old_velocity + delta_time * old_acceleration;
                let new_position = old_position
                    + delta_time * (old_velocity + new_velocity) / Dimensionless::new(2.0);
                self.position = new_position;
                self.velocity = new_velocity;
            }
            ///Set the acceleration.
            #[inline]
            pub const fn set_constant_acceleration(&mut self, acceleration: $acc) {
                self.acceleration = acceleration;
            }
            ///Set the velocity to a given value and set the acceleration to zero.
            #[inline]
            pub const fn set_constant_velocity(&mut self, velocity: $vel) {
                self.acceleration = <$acc>::new(0.0);
                self.velocity = velocity;
            }
            ///Set the position to a given value and set the velocity and acceleration to zero.
            #[inline]
            pub const fn set_constant_position(&mut self, position: $pos) {
                self.acceleration = <$acc>::new(0.0);
                self.velocity = <$vel>::new(0.0);
                self.position = position;
            }
            //Might you want to rename Command to something more broad and make this return that?
            ///States contain a position, velocity, and acceleration. This gets the respective field of a
            ///given position derivative.
            pub fn get_value(&self, position_derivative: PositionDerivative) -> f32 {
                match position_derivative {
                    PositionDerivative::Position => self.position.into_inner(),
                    PositionDerivative::Velocity => self.velocity.into_inner(),
                    PositionDerivative::Acceleration => self.acceleration.into_inner(),
                }
            }
        }
        impl Neg for $name {
            type Output = Self;
            fn neg(self) -> Self {
                $name::new(-self.position, -self.velocity, -self.acceleration)
            }
        }
        impl Add for $name {
            type Output = Self;
            fn add(self, other: $name) -> Self {
                $name::new(
                    self.position + other.position,
                    self.velocity + other.velocity,
                    self.acceleration + other.acceleration,
                )
            }
        }
        impl Sub for $name {
            type Output = Self;
            fn sub(self, other: $name) -> Self {
                $name::new(
                    self.position - other.position,
                    self.velocity - other.velocity,
                    self.acceleration - other.acceleration,
                )
            }
        }
        impl Mul<Dimensionless<f32>> for $name {
            type Output = Self;
            fn mul(self, coef: Dimensionless<f32>) -> Self {
                $name::new(
                    self.position * coef,
                    self.velocity * coef,
                    self.acceleration * coef,
                )
            }
        }
        impl Div<Dimensionless<f32>> for $name {
            type Output = Self;
            fn div(self, dvsr: Dimensionless<f32>) -> Self {
                $name::new(
                    self.position / dvsr,
                    self.velocity / dvsr,
                    self.acceleration / dvsr,
                )
            }
        }
        impl AddAssign for $name {
            fn add_assign(&mut self, other: $name) {
                *self = *self + other;
            }
        }
        impl SubAssign for $name {
            fn sub_assign(&mut self, other: $name) {
                *self = *self - other;
            }
        }
        impl MulAssign<Dimensionless<f32>> for $name {
            fn mul_assign(&mut self, coef: Dimensionless<f32>) {
                *self = *self * coef;
            }
        }
        impl DivAssign<Dimensionless<f32>> for $name {
            fn div_assign(&mut self, dvsr: Dimensionless<f32>) {
                *self = *self / dvsr;
            }
        }
        impl GenericState for $name {
            type Position = $pos;
            type Velocity = $vel;
            type Acceleration = $acc;
            #[inline]
            fn generic_new(position: $pos, velocity: $vel, acceleration: $acc) -> Self {
                Self::new(position, velocity, acceleration)
            }
            #[inline]
            fn generic_update(&mut self, delta_time: Time) {
                self.update(delta_time);
            }
            #[inline]
            fn generic_set_constant_position(&mut self, position: $pos) {
                self.set_constant_position(position);
            }
            #[inline]
            fn generic_set_constant_velocity(&mut self, velocity: $vel) {
                self.set_constant_velocity(velocity);
            }
            #[inline]
            fn generic_set_constant_acceleration(&mut self, acceleration: $acc) {
                self.set_constant_acceleration(acceleration);
            }
            #[inline]
            fn generic_get_value(&self, position_derivative: PositionDerivative) -> f32 {
                self.get_value(position_derivative)
            }
            #[inline]
            fn generic_position(&self) -> $pos {
                self.position
            }
            #[inline]
            fn generic_velocity(&self) -> $vel {
                self.velocity
            }
            #[inline]
            fn generic_acceleration(&self) -> $acc {
                self.acceleration
            }
            #[inline]
            fn generic_position_mut(&mut self) -> &mut $pos {
                &mut self.position
            }
            #[inline]
            fn generic_velocity_mut(&mut self) -> &mut $vel {
                &mut self.velocity
            }
            #[inline]
            fn generic_acceleration_mut(&mut self) -> &mut $acc {
                &mut self.acceleration
            }
        }
    };
}
build_state_struct!(
    LinearState,
    Millimeter<f32>,
    MillimeterPerSecond<f32>,
    MillimeterPerSecondSquared<f32>
);
build_state_struct!(
    AngularState,
    Dimensionless<f32>,
    InverseSecond<f32>,
    InverseSecondSquared<f32>
);
impl Mul<Millimeter<f32>> for AngularState {
    type Output = LinearState;
    fn mul(self, rhs: Millimeter<f32>) -> LinearState {
        LinearState {
            position: self.position * rhs,
            velocity: self.velocity * rhs,
            acceleration: self.acceleration * rhs,
        }
    }
}
impl Div<Millimeter<f32>> for LinearState {
    type Output = AngularState;
    fn div(self, rhs: Millimeter<f32>) -> AngularState {
        AngularState {
            position: self.position / rhs,
            velocity: self.velocity / rhs,
            acceleration: self.acceleration / rhs,
        }
    }
}
