// SPDX-License-Identifier: LGPL-3.0-only
/*
Copyright 2024 UxuginPython on GitHub

     This file is part of Rust Robotics ToolKit.

    Rust Robotics ToolKit is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, version 3.

    Rust Robotics ToolKit is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Lesser General Public License for more details.

    You should have received a copy of the GNU Lesser General Public License along with Rust Robotics ToolKit. If not, see <https://www.gnu.org/licenses/>.
*/
#![cfg_attr(not(feature = "std"), no_std)]
#[cfg(feature = "std")]
use std::cell::RefCell;
#[cfg(feature = "std")]
use std::fmt::Debug;
#[cfg(feature = "std")]
use std::rc::Rc;
#[cfg(not(feature = "std"))]
extern crate alloc;
#[cfg(not(feature = "std"))]
use alloc::boxed::Box;
#[cfg(not(feature = "std"))]
use alloc::rc::Rc;
#[cfg(not(feature = "std"))]
use alloc::vec;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;
#[cfg(not(feature = "std"))]
use core::cell::RefCell;
#[cfg(not(feature = "std"))]
use core::fmt::Debug;
#[cfg(feature = "devices")]
pub mod devices;
pub mod streams;
#[derive(Clone, Copy, Debug)]
pub enum Error<O: Copy + Debug> {
    ///Returned when a `None` is elevated to an error by a `NoneToError`.
    FromNone,
    ///Returned when a `TimeGetterFromStream`'s `Stream` doesn't return `Ok(Some(_))`.
    StreamNotSome,
    Other(O),
}
///A one-dimensional motion state with position, velocity, and acceleration.
#[derive(Clone)]
pub struct State {
    pub position: f32,
    pub velocity: f32,
    pub acceleration: f32,
}
impl State {
    ///Constructor for `State`.
    pub fn new(position: f32, velocity: f32, acceleration: f32) -> State {
        State {
            position: position,
            velocity: velocity,
            acceleration: acceleration,
        }
    }
    ///Calculate the future state assuming a constant acceleration.
    pub fn update(&mut self, delta_time: f32) {
        let new_velocity = self.velocity + delta_time * self.acceleration;
        let new_position = self.position + delta_time * (self.velocity + new_velocity) / 2.0;
        self.position = new_position;
        self.velocity = new_velocity;
    }
    ///Set the acceleration.
    pub fn set_constant_acceleration(&mut self, acceleration: f32) {
        self.acceleration = acceleration;
    }
    ///Set the velocity to a given value and acceleration to zero.
    pub fn set_constant_velocity(&mut self, velocity: f32) {
        self.acceleration = 0.0;
        self.velocity = velocity;
    }
    ///Set the position to a given value and the velocity and acceleration to zero.
    pub fn set_constant_position(&mut self, position: f32) {
        self.acceleration = 0.0;
        self.velocity = 0.0;
        self.position = position;
    }
}
///A container for a time and something else, usually an `f32` or a `State`.
#[derive(Clone, Debug)]
pub struct Datum<T> {
    pub time: f32,
    pub value: T,
}
impl<T> Datum<T> {
    ///Constructor for Datum type.
    pub fn new(time: f32, value: T) -> Datum<T> {
        Datum {
            time: time,
            value: value,
        }
    }
}
///What a motor is currently controlling: position, velocity, or acceleration.
#[derive(Debug, PartialEq)]
pub enum MotorMode {
    Position,
    Velocity,
    Acceleration,
}
///Compute absolute value without the standard library. Requires `motionprofile` feature.
//abs method of f32 does not exist in no_std
#[cfg(all(not(feature = "std"), feature = "motionprofile"))]
#[inline]
fn my_abs_f32(num: f32) -> f32 {
    if num >= 0.0 {
        num
    } else {
        -num
    }
}
///Where you are in following a motion profile. Requires `motionprofile` feature.
#[cfg(feature = "motionprofile")]
pub enum MotionProfilePiece {
    BeforeStart,
    InitialAcceleration,
    ConstantVelocity,
    EndAcceleration,
    Complete,
}
///A motion profile for getting from one state to another. Requires `motionprofile` feature.
#[cfg(feature = "motionprofile")]
pub struct MotionProfile {
    start_pos: f32,
    start_vel: f32,
    t1: f32,
    t2: f32,
    t3: f32,
    max_acc: f32,
}
#[cfg(feature = "motionprofile")]
impl History<State> for MotionProfile {
    fn get(&self, time: f32) -> Option<Datum<State>> {
        let pos = match self.get_position(time) {
            Ok(value) => value,
            Err(_) => {return None;}
        };
        let vel = match self.get_velocity(time) {
            Ok(value) => value,
            Err(_) => {return None;}
        };
        let acc = match self.get_acceleration(time) {
            Ok(value) => value,
            Err(_) => {return None;}
        };
        Some(Datum::new(time, State::new(pos, vel, acc)))
    }
    fn update(&mut self) {}
}
#[cfg(feature = "motionprofile")]
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
        MotionProfile {
            start_pos: start_state.position,
            start_vel: start_state.velocity,
            t1: t1,
            t2: t2,
            t3: t3,
            max_acc: max_acc,
        }
    }
    ///Get the intended `MotorMode` at a given time.
    pub fn get_mode(&self, t: f32) -> Result<MotorMode, &'static str> {
        if t < 0.0 {
            return Err("time invalid");
        } else if t < self.t1 {
            return Ok(MotorMode::Acceleration);
        } else if t < self.t2 {
            return Ok(MotorMode::Velocity);
        } else if t < self.t3 {
            return Ok(MotorMode::Acceleration);
        } else {
            return Err("time invalid");
        }
    }
    ///Get the `MotionProfilePiece` at a given time.
    pub fn get_piece(&self, t: f32) -> MotionProfilePiece {
        if t < 0.0 {
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
    pub fn get_acceleration(&self, t: f32) -> Result<f32, &'static str> {
        if t < 0.0 {
            return Err("time invalid");
        } else if t < self.t1 {
            return Ok(self.max_acc);
        } else if t < self.t2 {
            return Ok(0.0);
        } else if t < self.t3 {
            return Ok(-self.max_acc);
        } else {
            return Err("time invalid");
        }
    }
    ///Get the intended velocity at a given time.
    pub fn get_velocity(&self, t: f32) -> Result<f32, &'static str> {
        if t < 0.0 {
            return Err("time invalid");
        } else if t < self.t1 {
            return Ok(self.max_acc * t + self.start_vel);
        } else if t < self.t2 {
            return Ok(self.max_acc * self.t1 + self.start_vel);
        } else if t < self.t3 {
            return Ok(self.max_acc * (self.t1 + self.t2 - t) + self.start_vel);
        } else {
            return Err("time invalid");
        }
    }
    ///Get the intended position at a given time.
    pub fn get_position(&self, t: f32) -> Result<f32, &'static str> {
        if t < 0.0 {
            return Err("time invalid");
        } else if t < self.t1 {
            return Ok(0.5 * self.max_acc * t * t + self.start_vel * t + self.start_pos);
        } else if t < self.t2 {
            return Ok(self.max_acc * self.t1 * (-0.5 * self.t1 + t)
                + self.start_vel * t
                + self.start_pos);
        } else if t < self.t3 {
            return Ok(self.max_acc * self.t1 * (-0.5 * self.t1 + self.t2)
                - 0.5 * self.max_acc * (t - self.t2) * (t - 2.0 * self.t1 - self.t2)
                + self.start_vel * t
                + self.start_pos);
        } else {
            return Err("time invalid");
        }
    }
}
pub type StreamOutput<T, E> = Result<Option<Datum<T>>, Error<E>>;
pub type TimeGetterOutput<E> = Result<f32, Error<E>>;
pub type InputStream<T, E> = Rc<RefCell<Box<dyn Stream<T, E>>>>;
pub type InputTimeGetter<E> = Rc<RefCell<Box<dyn TimeGetter<E>>>>;
pub trait TimeGetter<E: Copy + Debug> {
    fn get(&self) -> Result<f32, Error<E>>;
    fn update(&mut self);
}
pub struct TimeGetterFromStream<T: Clone, E> {
    elevator: streams::converters::NoneToError<T, E>,
}
impl<T: Clone, E> TimeGetterFromStream<T, E> {
    pub fn new(stream: Rc<RefCell<Box<dyn Stream<T, E>>>>) -> Self {
        Self {
            elevator: streams::converters::NoneToError::new(Rc::clone(&stream)),
        }
    }
}
impl<T: Clone, E: Copy + Debug> TimeGetter<E> for TimeGetterFromStream<T, E> {
    fn get(&self) -> Result<f32, Error<E>> {
        let output = self.elevator.get()?;
        let output = output.expect("`NoneToError` made all `Ok(None)`s into `Err(_)`s, and `?` returned all `Err(_)`s, so we're sure this is now an `Ok(Some(_))`.");
        return Ok(output.time);
    }
    fn update(&mut self) {}
}
pub trait Stream<T: Clone, E: Copy + Debug> {
    fn get(&self) -> StreamOutput<T, E>;
    fn update(&mut self);
}
pub trait History<T: Clone> {
    fn get(&self, time: f32) -> Option<Datum<T>>;
    fn update(&mut self);
}
#[macro_export]
macro_rules! make_stream_input {
    ($stream:expr, $ttype:tt, $etype:tt) => {
        Rc::new(RefCell::new(
            Box::new($stream) as Box<dyn Stream<$ttype, $etype>>
        ))
    };
}
#[macro_export]
macro_rules! make_time_getter_input {
    ($time_getter:expr, $etype:tt) => {
        Rc::new(RefCell::new(
            Box::new($time_getter) as Box<dyn TimeGetter<$etype>>
        ))
    };
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[cfg(feature = "motionprofile")]
    fn motion_profile_new_1() {
        let motion_profile = MotionProfile::new(
            State::new(0.0, 0.0, 0.0),
            State::new(3.0, 0.0, 0.0),
            1.0,
            1.0,
        );
        assert_eq!(motion_profile.t1, 1.0);
        assert_eq!(motion_profile.t2, 3.0);
        assert_eq!(motion_profile.t3, 4.0);
        assert_eq!(motion_profile.max_acc, 1.0);
    }
    #[test]
    #[cfg(feature = "motionprofile")]
    fn motion_profile_new_2() {
        let motion_profile = MotionProfile::new(
            State::new(1.0, 0.0, 0.0),
            State::new(3.0, 0.0, 0.0),
            1.0,
            1.0,
        );
        assert_eq!(motion_profile.t1, 1.0);
        assert_eq!(motion_profile.t2, 2.0);
        assert_eq!(motion_profile.t3, 3.0);
        assert_eq!(motion_profile.max_acc, 1.0);
    }
    #[test]
    #[cfg(feature = "motionprofile")]
    fn motion_profile_new_3() {
        let motion_profile = MotionProfile::new(
            State::new(0.0, 1.0, 0.0),
            State::new(3.0, 0.0, 0.0),
            1.0,
            1.0,
        );
        assert_eq!(motion_profile.t1, 0.0);
        assert_eq!(motion_profile.t2, 2.5);
        assert_eq!(motion_profile.t3, 3.5);
        assert_eq!(motion_profile.max_acc, 1.0);
    }
    #[test]
    #[cfg(feature = "motionprofile")]
    fn motion_profile_new_4() {
        let motion_profile = MotionProfile::new(
            State::new(0.0, 0.0, 1.0),
            State::new(3.0, 0.0, 0.0),
            1.0,
            1.0,
        );
        assert_eq!(motion_profile.t1, 1.0);
        assert_eq!(motion_profile.t2, 3.0);
        assert_eq!(motion_profile.t3, 4.0);
        assert_eq!(motion_profile.max_acc, 1.0);
    }
    #[test]
    #[cfg(feature = "motionprofile")]
    fn motion_profile_new_5() {
        let motion_profile = MotionProfile::new(
            State::new(0.0, 0.0, 0.0),
            State::new(6.0, 0.0, 0.0),
            2.0,
            1.0,
        );
        assert_eq!(motion_profile.t1, 2.0);
        assert_eq!(motion_profile.t2, 3.0);
        assert_eq!(motion_profile.t3, 5.0);
        assert_eq!(motion_profile.max_acc, 1.0);
    }
    #[test]
    #[cfg(feature = "motionprofile")]
    fn motion_profile_new_6() {
        let motion_profile = MotionProfile::new(
            State::new(0.0, 0.0, 0.0),
            State::new(3.0, 0.0, 0.0),
            1.0,
            2.0,
        );
        assert_eq!(motion_profile.t1, 0.5);
        assert_eq!(motion_profile.t2, 3.0);
        assert_eq!(motion_profile.t3, 3.5);
        assert_eq!(motion_profile.max_acc, 2.0);
    }
    #[test]
    #[cfg(feature = "motionprofile")]
    fn motion_profile_new_7() {
        let motion_profile = MotionProfile::new(
            State::new(0.0, 0.0, 0.0),
            State::new(-3.0, 0.0, 0.0),
            1.0,
            1.0,
        );
        assert_eq!(motion_profile.t1, 1.0);
        assert_eq!(motion_profile.t2, 3.0);
        assert_eq!(motion_profile.t3, 4.0);
        assert_eq!(motion_profile.max_acc, -1.0);
    }
}
