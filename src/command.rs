// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2024-2025 UxuginPython
use super::*;
macro_rules! build_command_enum {
    ($name: ident, $pos: ty, $vel: ty, $acc: ty) => {
        ///A command for a motor to perform: go to a position, run at a velocity, or accelerate at a rate.
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum $name {
            ///Where you want to be. This should be in millimeters.
            Position($pos),
            ///How fast you want to be going. This should be in millimeters per second.
            Velocity($vel),
            ///How fast you want how fast you're going to change. This should be in millimeters per second squared.
            Acceleration($acc),
        }
        impl $name {
            ///Constructor for [`$name`].
            pub const fn new(position_derivative: PositionDerivative, value: f32) -> Self {
                match position_derivative {
                    PositionDerivative::Position => Self::Position(Millimeter::new(value)),
                    PositionDerivative::Velocity => Self::Velocity(MillimeterPerSecond::new(value)),
                    PositionDerivative::Acceleration => {
                        Self::Acceleration(MillimeterPerSecondSquared::new(value))
                    }
                }
            }
            ///Get the commanded constant position if there is one. If the position derivative is
            ///velocity or acceleration, this will return `None` as there is not a constant position.
            pub const fn get_position(&self) -> Option<$pos> {
                if let Self::Position(pos) = self {
                    Some(*pos)
                } else {
                    None
                }
            }
            ///Get the commanded constant velocity if there is one. If the position derivative is
            ///acceleration, this will return `None` as there is not a constant
            ///velocity. If the position derivative is position, this will return 0 as
            ///velocity should be zero with a constant position.
            pub const fn get_velocity(&self) -> Option<$vel> {
                match self {
                    Self::Position(_) => Some(MillimeterPerSecond::new(0.0)),
                    Self::Velocity(vel) => Some(*vel),
                    Self::Acceleration(_) => None,
                }
            }
            ///Get the commanded constant acceleration. If the position derivative is not
            ///acceleration, this will return 0 as acceleration should be zero with a constant velocity or
            ///position.
            pub const fn get_acceleration(&self) -> $acc {
                if let Self::Acceleration(acc) = self {
                    *acc
                } else {
                    MillimeterPerSecondSquared::new(0.0)
                }
            }
        }
        impl From<$pos> for $name {
            fn from(was: $pos) -> Self {
                Self::Position(was)
            }
        }
        impl From<$vel> for $name {
            fn from(was: $vel) -> Self {
                Self::Velocity(was)
            }
        }
        impl From<$acc> for $name {
            fn from(was: $acc) -> Self {
                Self::Acceleration(was)
            }
        }
        impl From<State> for $name {
            fn from(state: State) -> Self {
                if state.acceleration == <$acc>::new(0.0) {
                    if state.velocity == <$vel>::new(0.0) {
                        Self::Position(state.position)
                    } else {
                        Self::Velocity(state.velocity)
                    }
                } else {
                    Self::Acceleration(state.acceleration)
                }
            }
        }
        impl From<$name> for f32 {
            fn from(was: $name) -> f32 {
                match was {
                    $name::Position(pos) => pos.into_inner(),
                    $name::Velocity(vel) => vel.into_inner(),
                    $name::Acceleration(acc) => acc.into_inner(),
                }
            }
        }
        impl Add for $name {
            type Output = Self;
            fn add(self, rhs: Self) -> Self {
                let self_pos_der = PositionDerivative::from(self);
                assert_eq!(self_pos_der, PositionDerivative::from(rhs));
                Self::new(self_pos_der, f32::from(self) + f32::from(rhs))
            }
        }
        impl Sub for $name {
            type Output = Self;
            fn sub(self, rhs: Self) -> Self {
                let self_pos_der = PositionDerivative::from(self);
                assert_eq!(self_pos_der, PositionDerivative::from(rhs));
                Self::new(self_pos_der, f32::from(self) - f32::from(rhs))
            }
        }
        impl Mul<Dimensionless<f32>> for $name {
            type Output = Self;
            fn mul(self, rhs: Dimensionless<f32>) -> Self {
                match self {
                    Self::Position(pos) => Self::Position(pos * rhs),
                    Self::Velocity(vel) => Self::Velocity(vel * rhs),
                    Self::Acceleration(acc) => Self::Acceleration(acc * rhs),
                }
            }
        }
        impl Div<Dimensionless<f32>> for $name {
            type Output = Self;
            fn div(self, rhs: Dimensionless<f32>) -> Self {
                match self {
                    Self::Position(pos) => Self::Position(pos / rhs),
                    Self::Velocity(vel) => Self::Velocity(vel / rhs),
                    Self::Acceleration(vel) => Self::Acceleration(vel / rhs),
                }
            }
        }
        impl Neg for $name {
            type Output = Self;
            fn neg(self) -> Self {
                match self {
                    Self::Position(pos) => Self::Position(-pos),
                    Self::Velocity(vel) => Self::Velocity(-vel),
                    Self::Acceleration(acc) => Self::Acceleration(-acc),
                }
            }
        }
        impl AddAssign for $name {
            fn add_assign(&mut self, rhs: Self) {
                *self = *self + rhs;
            }
        }
        impl SubAssign for $name {
            fn sub_assign(&mut self, rhs: Self) {
                *self = *self - rhs;
            }
        }
        //You might be able to optimize this a bit more with an unsafe dereference of the field since it's
        //always an f32 and the variant never changes.
        impl MulAssign<Dimensionless<f32>> for $name {
            fn mul_assign(&mut self, rhs: Dimensionless<f32>) {
                match self {
                    Self::Position(pos) => *pos *= rhs,
                    Self::Velocity(vel) => *vel *= rhs,
                    Self::Acceleration(acc) => *acc *= rhs,
                }
            }
        }
        impl DivAssign<Dimensionless<f32>> for $name {
            fn div_assign(&mut self, rhs: Dimensionless<f32>) {
                match self {
                    Self::Position(pos) => *pos /= rhs,
                    Self::Velocity(vel) => *vel /= rhs,
                    Self::Acceleration(acc) => *acc /= rhs,
                }
            }
        }
    };
}
build_command_enum!(
    Command,
    Millimeter<f32>,
    MillimeterPerSecond<f32>,
    MillimeterPerSecondSquared<f32>
);
