// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2024-2025 UxuginPython
use super::*;
///A command for a motor to perform: go to a position, run at a velocity, or accelerate at a rate.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Command {
    ///Where you want to be. This should be in millimeters.
    Position(Millimeter<f32>),
    ///How fast you want to be going. This should be in millimeters per second.
    Velocity(MillimeterPerSecond<f32>),
    ///How fast you want how fast you're going to change. This should be in millimeters per second squared.
    Acceleration(MillimeterPerSecondSquared<f32>),
}
impl Command {
    ///Constructor for [`Command`].
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
    pub const fn get_position(&self) -> Option<Millimeter<f32>> {
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
    pub const fn get_velocity(&self) -> Option<MillimeterPerSecond<f32>> {
        match self {
            Self::Position(_) => Some(MillimeterPerSecond::new(0.0)),
            Self::Velocity(vel) => Some(*vel),
            Self::Acceleration(_) => None,
        }
    }
    ///Get the commanded constant acceleration. If the position derivative is not
    ///acceleration, this will return 0 as acceleration should be zero with a constant velocity or
    ///position.
    pub const fn get_acceleration(&self) -> MillimeterPerSecondSquared<f32> {
        if let Self::Acceleration(acc) = self {
            *acc
        } else {
            MillimeterPerSecondSquared::new(0.0)
        }
    }
}
impl From<Millimeter<f32>> for Command {
    fn from(was: Millimeter<f32>) -> Self {
        Self::Position(was)
    }
}
impl From<MillimeterPerSecond<f32>> for Command {
    fn from(was: MillimeterPerSecond<f32>) -> Self {
        Self::Velocity(was)
    }
}
impl From<MillimeterPerSecondSquared<f32>> for Command {
    fn from(was: MillimeterPerSecondSquared<f32>) -> Self {
        Self::Acceleration(was)
    }
}
impl From<State> for Command {
    fn from(state: State) -> Self {
        if state.acceleration == MillimeterPerSecondSquared::new(0.0) {
            if state.velocity == MillimeterPerSecond::new(0.0) {
                Self::Position(state.position)
            } else {
                Self::Velocity(state.velocity)
            }
        } else {
            Self::Acceleration(state.acceleration)
        }
    }
}
impl From<Command> for f32 {
    fn from(was: Command) -> f32 {
        match was {
            Command::Position(pos) => pos.into_inner(),
            Command::Velocity(vel) => vel.into_inner(),
            Command::Acceleration(acc) => acc.into_inner(),
        }
    }
}
impl Add for Command {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let self_pos_der = PositionDerivative::from(self);
        assert_eq!(self_pos_der, PositionDerivative::from(rhs));
        Self::new(self_pos_der, f32::from(self) + f32::from(rhs))
    }
}
impl Sub for Command {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        let self_pos_der = PositionDerivative::from(self);
        assert_eq!(self_pos_der, PositionDerivative::from(rhs));
        Self::new(self_pos_der, f32::from(self) - f32::from(rhs))
    }
}
impl Mul<f32> for Command {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        let self_pos_der = PositionDerivative::from(self);
        let value = f32::from(self) * rhs;
        Self::new(self_pos_der, value)
    }
}
impl Div<f32> for Command {
    type Output = Self;
    fn div(self, rhs: f32) -> Self {
        let self_pos_der = PositionDerivative::from(self);
        let value = f32::from(self) / rhs;
        Self::new(self_pos_der, value)
    }
}
impl Neg for Command {
    type Output = Self;
    fn neg(self) -> Self {
        match self {
            Self::Position(pos) => Self::Position(-pos),
            Self::Velocity(vel) => Self::Velocity(-vel),
            Self::Acceleration(acc) => Self::Acceleration(-acc),
        }
    }
}
impl AddAssign for Command {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}
impl SubAssign for Command {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}
impl MulAssign<f32> for Command {
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs;
    }
}
impl DivAssign<f32> for Command {
    fn div_assign(&mut self, rhs: f32) {
        *self = *self / rhs;
    }
}
