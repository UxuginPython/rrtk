// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2024-2025 UxuginPython
use crate::*;
pub trait GenericState<P, V, A>:
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
    fn generic_new(position: P, velocity: V, acceleration: A) -> Self;
    fn generic_update(&mut self, delta_time: Time);
    fn generic_set_constant_position(&mut self, position: P);
    fn generic_set_constant_velocity(&mut self, velocity: V);
    fn generic_set_constant_acceleration(&mut self, acceleration: A);
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
            ///Calculate the future state assuming a constant acceleration.
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
        impl GenericState<$pos, $vel, $acc> for $name {
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
        }
    };
}
build_state_struct!(
    State,
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
