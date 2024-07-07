// SPDX-License-Identifier: LGPL-3.0-only
/*
Copyright 2024 UxuginPython on GitHub

     This file is part of Rust Robotics ToolKit.

    Rust Robotics ToolKit is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, version 3.

    Rust Robotics ToolKit is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Lesser General Public License for more details.

    You should have received a copy of the GNU Lesser General Public License along with Rust Robotics ToolKit. If not, see <https://www.gnu.org/licenses/>.
*/
use crate::{Command, Datum, Debug, History, NothingOrError, PositionDerivative, State, Updatable};
//abs method of f32 does not exist in no_std
#[cfg(not(feature = "std"))]
#[inline]
fn my_abs_f32(num: f32) -> f32 {
    if num >= 0.0 {
        num
    } else {
        -num
    }
}
///Where you are in following a motion profile.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MotionProfilePiece {
    ///You have not yet started the motion profile.
    BeforeStart,
    ///You are changing velocity at the beginning.
    InitialAcceleration,
    ///You are moving at a constant speed.
    ConstantVelocity,
    ///You are changing velocity at the end.
    EndAcceleration,
    ///You are done with the motion profile.
    Complete,
}
///A motion profile for getting from one state to another.
#[derive(Clone, Debug, PartialEq)]
pub struct MotionProfile {
    start_pos: f32,
    start_vel: f32,
    t1: i64,
    t2: i64,
    t3: i64,
    max_acc: f32,
    end_command: Command,
}
impl<E: Copy + Debug> History<Command, E> for MotionProfile {
    fn get(&self, time: i64) -> Option<Datum<Command>> {
        let mode = match self.get_mode(time) {
            Some(value) => value,
            None => {
                return None;
            }
        };
        let value = match mode {
            PositionDerivative::Position => self.get_position(time).expect("If mode is Position, this should be Some."),
            PositionDerivative::Velocity => self.get_velocity(time).expect("If mode is Velocity, this should be Some."),
            PositionDerivative::Acceleration => self.get_acceleration(time).expect("If mode is Acceleration, this should be Some."),
        };
        Some(Datum::new(time, Command::new(mode, value)))
    }
}
impl<E: Copy + Debug> Updatable<E> for MotionProfile {
    fn update(&mut self) -> NothingOrError<E> {
        Ok(())
    }
}
impl MotionProfile {
    ///Constructor for `MotionProfile` using start and end states.
    pub fn new(start_state: State, end_state: State, max_vel: f32, max_acc: f32) -> MotionProfile {
        let sign = if end_state.position < start_state.position {
            -1.0
        } else {
            1.0
        };
        #[cfg(not(feature = "std"))]
        let max_vel = my_abs_f32(max_vel) * sign;
        #[cfg(not(feature = "std"))]
        let max_acc = my_abs_f32(max_acc) * sign;
        #[cfg(feature = "std")]
        let max_vel = max_vel.abs() * sign;
        #[cfg(feature = "std")]
        let max_acc = max_acc.abs() * sign;
        let d_t1_vel = max_vel - start_state.velocity;
        let t1 = d_t1_vel / max_acc;
        assert!(t1 >= 0.0);
        let d_t1_pos = (start_state.velocity + max_vel) / 2.0 * t1;
        let d_t3_vel = end_state.velocity - max_vel;
        let d_t3 = d_t3_vel / -max_acc;
        assert!(d_t3 >= 0.0);
        let d_t3_pos = (max_vel + end_state.velocity) / 2.0 * d_t3;
        let d_t2_pos = (end_state.position - start_state.position) - (d_t1_pos + d_t3_pos);
        let d_t2 = d_t2_pos / max_vel;
        assert!(d_t2 >= 0.0);
        let t2 = t1 + d_t2;
        let t3 = t2 + d_t3;
        let end_command = Command::from(end_state);
        MotionProfile {
            start_pos: start_state.position,
            start_vel: start_state.velocity,
            t1: t1 as i64,
            t2: t2 as i64,
            t3: t3 as i64,
            max_acc: max_acc,
            end_command: end_command,
        }
    }
    ///Get the intended `PositionDerivative` at a given time.
    pub fn get_mode(&self, t: i64) -> Option<PositionDerivative> {
        if t < 0 {
            return None;
        } else if t < self.t1 {
            return Some(PositionDerivative::Acceleration);
        } else if t < self.t2 {
            return Some(PositionDerivative::Velocity);
        } else if t < self.t3 {
            return Some(PositionDerivative::Acceleration);
        } else {
            return Some(self.end_command.position_derivative);
        }
    }
    ///Get the `MotionProfilePiece` at a given time.
    pub fn get_piece(&self, t: i64) -> MotionProfilePiece {
        if t < 0 {
            return MotionProfilePiece::BeforeStart;
        } else if t < self.t1 {
            return MotionProfilePiece::InitialAcceleration;
        } else if t < self.t2 {
            return MotionProfilePiece::ConstantVelocity;
        } else if t < self.t3 {
            return MotionProfilePiece::EndAcceleration;
        } else {
            return MotionProfilePiece::Complete;
        }
    }
    ///Get the intended acceleration at a given time.
    pub fn get_acceleration(&self, t: i64) -> Option<f32> {
        if t < 0 {
            return None;
        } else if t < self.t1 {
            return Some(self.max_acc);
        } else if t < self.t2 {
            return Some(0.0);
        } else if t < self.t3 {
            return Some(-self.max_acc);
        } else {
            return Some(self.end_command.get_acceleration());
        }
    }
    ///Get the intended velocity at a given time.
    pub fn get_velocity(&self, t: i64) -> Option<f32> {
        if t < 0 {
            return None;
        } else if t < self.t1 {
            return Some(self.max_acc * (t as f32) + self.start_vel);
        } else if t < self.t2 {
            return Some(self.max_acc * (self.t1 as f32) + self.start_vel);
        } else if t < self.t3 {
            return Some(self.max_acc * ((self.t1 + self.t2 - t) as f32) + self.start_vel);
        } else {
            return self.end_command.get_velocity();
        }
    }
    ///Get the intended position at a given time.
    pub fn get_position(&self, t: i64) -> Option<f32> {
        if t < 0 {
            return None;
        } else if t < self.t1 {
            let t = t as f32;
            return Some(0.5 * self.max_acc * t * t + self.start_vel * t + self.start_pos);
        } else if t < self.t2 {
            return Some(
                self.max_acc * ((self.t1 * (-self.t1 / 2 + t)) as f32)
                    + self.start_vel * (t as f32)
                    + self.start_pos,
            );
        } else if t < self.t3 {
            return Some(
                self.max_acc * ((self.t1 * (-self.t1 / 2 + self.t2)) as f32)
                    - 0.5 * self.max_acc * (((t - self.t2) * (t - 2 * self.t1 - self.t2)) as f32)
                    + self.start_vel * (t as f32)
                    + self.start_pos,
            );
        } else {
            return self.end_command.get_position();
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[cfg(not(feature = "std"))]
    fn my_abs_f32_() {
        assert_eq!(my_abs_f32(5.0), 5.0);
        assert_eq!(my_abs_f32(-5.0), 5.0);
        assert_eq!(my_abs_f32(0.0), 0.0);
    }
    #[test]
    fn motion_profile_new_1() {
        let motion_profile = MotionProfile::new(
            State::new(0.0, 0.0, 0.0),
            State::new(3.0, 0.0, 0.0),
            0.1,
            0.01,
        );
        assert_eq!(motion_profile.t1, 10);
        assert_eq!(motion_profile.t2, 30);
        assert_eq!(motion_profile.t3, 40);
        assert_eq!(motion_profile.max_acc, 0.01);
    }
    #[test]
    fn motion_profile_new_2() {
        let motion_profile = MotionProfile::new(
            State::new(1.0, 0.0, 0.0),
            State::new(3.0, 0.0, 0.0),
            0.1,
            0.01,
        );
        assert_eq!(motion_profile.t1, 10);
        assert_eq!(motion_profile.t2, 20);
        assert_eq!(motion_profile.t3, 30);
        assert_eq!(motion_profile.max_acc, 0.01);
    }
    #[test]
    fn motion_profile_new_3() {
        let motion_profile = MotionProfile::new(
            State::new(0.0, 0.1, 0.0),
            State::new(3.0, 0.0, 0.0),
            0.1,
            0.01,
        );
        assert_eq!(motion_profile.t1, 0);
        assert_eq!(motion_profile.t2, 25);
        assert_eq!(motion_profile.t3, 35);
        assert_eq!(motion_profile.max_acc, 0.01);
    }
    #[test]
    fn motion_profile_new_4() {
        let motion_profile = MotionProfile::new(
            State::new(0.0, 0.0, 0.01),
            State::new(3.0, 0.0, 0.0),
            0.1,
            0.01,
        );
        assert_eq!(motion_profile.t1, 10);
        assert_eq!(motion_profile.t2, 30);
        assert_eq!(motion_profile.t3, 40);
        assert_eq!(motion_profile.max_acc, 0.01);
    }
    #[test]
    fn motion_profile_new_5() {
        let motion_profile = MotionProfile::new(
            State::new(0.0, 0.0, 0.0),
            State::new(6.0, 0.0, 0.0),
            0.2,
            0.01,
        );
        assert_eq!(motion_profile.t1, 20);
        assert_eq!(motion_profile.t2, 30);
        assert_eq!(motion_profile.t3, 50);
        assert_eq!(motion_profile.max_acc, 0.01);
    }
    #[test]
    fn motion_profile_new_6() {
        let motion_profile = MotionProfile::new(
            State::new(0.0, 0.0, 0.0),
            State::new(3.0, 0.0, 0.0),
            0.1,
            0.02,
        );
        assert_eq!(motion_profile.t1, 5);
        assert_eq!(motion_profile.t2, 30);
        assert_eq!(motion_profile.t3, 35);
        assert_eq!(motion_profile.max_acc, 0.02);
    }
    #[test]
    fn motion_profile_new_7() {
        let motion_profile = MotionProfile::new(
            State::new(0.0, 0.0, 0.0),
            State::new(-3.0, 0.0, 0.0),
            0.1,
            0.01,
        );
        assert_eq!(motion_profile.t1, 10);
        assert_eq!(motion_profile.t2, 30);
        assert_eq!(motion_profile.t3, 40);
        assert_eq!(motion_profile.max_acc, -0.01);
    }
}
