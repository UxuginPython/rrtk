// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2024-2025 UxuginPython
use super::*;
//You can add pretty much whatever bounds here as long as they're implemented for Command and
//AngularCommand. Most of the strange or redundant seeming ones here have to do with MotionProfile.
///This trait allows one to write code generically over [`Command`] and [`AngularCommand`]. Each of
///those has a corresponding method to each method of this trait without the `generic_` prefix.
///This is necessary because many of the implementations should be const fn and can't be in a
///trait. Calling the direct methods (without `generic_`) is preferred where possible.
pub trait GenericCommand:
    Copy
    + Debug
    + PartialEq
    + From<Self::Position>
    + From<Self::Velocity>
    + From<Self::Acceleration>
    + From<Self::CorrespondingState>
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Dimensionless<f32>>
    + Div<Dimensionless<f32>>
    + Neg<Output = Self>
    + AddAssign
    + SubAssign
    + MulAssign<Dimensionless<f32>>
    + DivAssign<Dimensionless<f32>>
    + Into<PositionDerivative>
{
    ///The type that position is stored as. Almost certainly a [`Quantity`] of some type.
    type Position: Copy
        + Debug
        + Default
        + fmt::Display
        + PartialOrd
        + Neg<Output = Self::Position>
        + Add<Output = Self::Position>
        + Sub<Output = Self::Position>
        + Mul<Dimensionless<f32>, Output = Self::Position>
        + Div<Dimensionless<f32>, Output = Self::Position>
        + Div<Time, Output = Self::Velocity>
        + Div<Self::Velocity, Output = Second<f32>>
        + stulta::AbsoluteValue;
    ///The type that velocity is stored as. Almost certainly a [`Quantity`] of some type.
    type Velocity: Copy
        + Debug
        + Default
        + fmt::Display
        + PartialOrd
        + Neg<Output = Self::Velocity>
        + Add<Output = Self::Velocity>
        + Sub<Output = Self::Velocity>
        + Mul<Dimensionless<f32>, Output = Self::Velocity>
        + Div<Dimensionless<f32>, Output = Self::Velocity>
        + Mul<Time, Output = Self::Position>
        + Div<Time, Output = Self::Acceleration>
        + Div<Self::Acceleration, Output = Second<f32>>
        + Mul<Second<f32>, Output = Self::Position>
        + stulta::AbsoluteValue;
    ///The type that acceleration is stored as. Almost certainly a [`Quantity`] of some type.
    type Acceleration: Copy
        + Debug
        + Default
        + fmt::Display
        + PartialOrd
        + Neg<Output = Self::Acceleration>
        + Add<Output = Self::Acceleration>
        + Sub<Output = Self::Acceleration>
        + Mul<Dimensionless<f32>, Output = Self::Acceleration>
        + Div<Dimensionless<f32>, Output = Self::Acceleration>
        + Mul<Time, Output = Self::Velocity>
        + Mul<Second<f32>, Output = Self::Velocity>
        + stulta::AbsoluteValue;
    ///The corresponding state type with the same types for position, velocity, and acceleration.
    type CorrespondingState: GenericState<
            Position = Self::Position,
            Velocity = Self::Velocity,
            Acceleration = Self::Acceleration,
        >;
    ///Constructor from a position derivative and value.
    fn generic_new(position_derivative: PositionDerivative, value: f32) -> Self;
    ///If the command requires a known constant position, get it; otherwise, return `None`. This
    ///will only return `Some` with the `Position` variant.
    fn generic_get_position(&self) -> Option<Self::Position>;
    ///If the command requires a known constant velocity, get it; otherwise, return `None`. This
    ///will return `Some` with either the `Position` or `Velocity` variant. More specifically, if
    ///the command is the `Position` variant, this will always return `Some` with a value of zero.
    ///This returns `None` with the `Acceleration` variant because either velocity is not constant
    ///(most cases) or the constant velocity is not known (with a fixed acceleration of zero).
    fn generic_get_velocity(&self) -> Option<Self::Velocity>;
    ///Get the (constant) acceleration required by the command. Returns zero with the `Position` or
    ///`Velocity` variant, and, of course, returns the specified acceleration with the
    ///`Acceleration` variant.
    fn generic_get_acceleration(&self) -> Self::Acceleration;
}
macro_rules! build_command_enum {
    ($name: ident, $pos: ty, $vel: ty, $acc: ty, $corresponding_state: ty) => {
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
            ///Constructor from a position derivative and value.
            pub const fn new(position_derivative: PositionDerivative, value: f32) -> Self {
                match position_derivative {
                    PositionDerivative::Position => Self::Position(<$pos>::new(value)),
                    PositionDerivative::Velocity => Self::Velocity(<$vel>::new(value)),
                    PositionDerivative::Acceleration => Self::Acceleration(<$acc>::new(value)),
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
                    Self::Position(_) => Some(<$vel>::new(0.0)),
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
                    <$acc>::new(0.0)
                }
            }
        }
        impl GenericCommand for $name {
            type Position = $pos;
            type Velocity = $vel;
            type Acceleration = $acc;
            type CorrespondingState = $corresponding_state;
            #[inline]
            fn generic_new(position_derivative: PositionDerivative, value: f32) -> Self {
                Self::new(position_derivative, value)
            }
            #[inline]
            fn generic_get_position(&self) -> Option<$pos> {
                self.get_position()
            }
            #[inline]
            fn generic_get_velocity(&self) -> Option<$vel> {
                self.get_velocity()
            }
            #[inline]
            fn generic_get_acceleration(&self) -> $acc {
                self.get_acceleration()
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
        impl From<$corresponding_state> for $name {
            fn from(state: $corresponding_state) -> Self {
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
        impl From<$name> for PositionDerivative {
            fn from(was: $name) -> Self {
                match was {
                    $name::Position(_) => Self::Position,
                    $name::Velocity(_) => Self::Velocity,
                    $name::Acceleration(_) => Self::Acceleration,
                }
            }
        }
    };
}
build_command_enum!(
    LinearCommand,
    Millimeter<f32>,
    MillimeterPerSecond<f32>,
    MillimeterPerSecondSquared<f32>,
    State
);
build_command_enum!(
    AngularCommand,
    Dimensionless<f32>,
    InverseSecond<f32>,
    InverseSecondSquared<f32>,
    AngularState
);
impl Mul<Millimeter<f32>> for AngularCommand {
    type Output = LinearCommand;
    fn mul(self, rhs: Millimeter<f32>) -> LinearCommand {
        match self {
            Self::Position(pos) => LinearCommand::Position(pos * rhs),
            Self::Velocity(vel) => LinearCommand::Velocity(vel * rhs),
            Self::Acceleration(acc) => LinearCommand::Acceleration(acc * rhs),
        }
    }
}
impl Div<Millimeter<f32>> for LinearCommand {
    type Output = AngularCommand;
    fn div(self, rhs: Millimeter<f32>) -> AngularCommand {
        match self {
            Self::Position(pos) => AngularCommand::Position(pos / rhs),
            Self::Velocity(vel) => AngularCommand::Velocity(vel / rhs),
            Self::Acceleration(acc) => AngularCommand::Acceleration(acc / rhs),
        }
    }
}
