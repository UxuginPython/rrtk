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
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PositionDerivative {
    Position,
    Velocity,
    Acceleration,
}
pub struct PIDKValues {
    pub kp: f32,
    pub ki: f32,
    pub kd: f32,
}
pub struct PositionDerivativeDependentPIDKValues {
    pub position: PIDKValues,
    pub velocity: PIDKValues,
    pub acceleration: PIDKValues,
}
enum PositionDerivativeDependentPIDControllerShift {
    Position(PIDControllerShift<1>),
    Velocity(PIDControllerShift<2>),
    Acceleration(PIDControllerShift<3>),
}
pub type Output<T, E> = Result<Option<Datum<T>>, Error<E>>;
pub type TimeOutput<E> = Result<f32, Error<E>>;
pub type InputGetter<T, E> = Rc<RefCell<Box<dyn Getter<T, E>>>>;
pub type InputTimeGetter<E> = Rc<RefCell<Box<dyn TimeGetter<E>>>>;
pub trait TimeGetter<E: Copy + Debug>: Updatable {
    fn get(&self) -> TimeOutput<E>;
}
pub struct TimeGetterFromStream<T: Clone, E> {
    elevator: streams::converters::NoneToError<T, E>,
}
impl<T: Clone, E> TimeGetterFromStream<T, E> {
    pub fn new(stream: Rc<RefCell<Box<dyn Getter<T, E>>>>) -> Self {
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
}
impl<T: Clone, E: Copy + Debug> Updatable for TimeGetterFromStream<T, E> {
    fn update(&mut self) {}
}
pub trait History<T: Clone>: Updatable {
    fn get(&self, time: f32) -> Option<Datum<T>>;
}
#[derive(Clone, Copy)]
pub struct Command {
    pub position_derivative: PositionDerivative,
    pub value: f32,
}
pub trait Updatable {
    fn update(&mut self);
}
pub trait Getter<G, E: Copy + Debug>: Updatable {
    fn get(&self) -> Output<G, E>;
}
pub trait Settable<S, E: Copy + Debug>: Updatable {
    fn set(&mut self, value: S) -> Result<(), Error<E>>;
}
pub trait GetterSettable<G, S, E: Copy + Debug>: Getter<G, E> + Settable<S, E> {}
pub enum Device<E> {
    Read(Box<dyn Getter<State, E>>),
    ImpreciseWrite(Box<dyn Settable<f32, E>>, PositionDerivativeDependentPIDKValues),
    PreciseWrite(Box<dyn Settable<Command, E>>),
    ReadWrite(Box<dyn GetterSettable<State, Command, E>>),
}
impl<E: Copy + Debug> Updatable for Device<E> {
    fn update(&mut self) {
        match self {
            Self::Read(device) => {device.update();}
            Self::ImpreciseWrite(device, _) => {device.update();}
            Self::PreciseWrite(device) => {device.update();}
            Self::ReadWrite(device) => {device.update();}
        }
    }
}
pub struct Axle<const N: usize, E: Copy + Debug> {
    devices: [Device<E>; N],
    pids: [Option<PositionDerivativeDependentPIDControllerShift>; N],
    has_imprecise_write: bool,
}
impl<const N: usize, E: Copy + Debug> Axle<N, E> {
    pub fn new(devices: [Device<E>; N]) -> Self {
        let mut has_imprecise_write = false;
        for i in &devices {
            match i {
                Device::ImpreciseWrite(_, _) => {has_imprecise_write = true;}
                _ => {}
            }
        }
        const RUST_IS_PEDANTIC: Option<PositionDerivativeDependentPIDControllerShift> = None;
        Self {
            devices: devices,
            pids: [RUST_IS_PEDANTIC; N],
            has_imprecise_write: has_imprecise_write
        }
    }
}
impl<const N: usize, E: Copy + Debug> GetterSettable<State, Command, E> for Axle<N, E> {}
impl<const N: usize, E: Copy + Debug> Updatable for Axle<N, E> {
    fn update(&mut self) {
        if self.has_imprecise_write {
            todo!();
        }
        for i in &mut self.devices {
            i.update();
        }
    }
}
impl<const N: usize, E: Copy + Debug> Getter<State, E> for Axle<N, E> {
    fn get(&self) -> Output<State, E> {
        let mut time = -999999999f32;
        let mut pos_sum = 0f32;
        let mut vel_sum = 0f32;
        let mut acc_sum = 0f32;
        let mut valid_read_count = 0u8;
        for i in &self.devices {
            match i {
                Device::Read(device) => {
                    match device.get()? {
                        Some(datum) => {
                            if datum.time > time {
                                time = datum.time;
                            }
                            pos_sum += datum.value.position;
                            vel_sum += datum.value.velocity;
                            acc_sum += datum.value.acceleration;
                            valid_read_count += 1;
                        }
                        None => {}
                    }
                }
                Device::ReadWrite(device) => {
                    match device.get()? {
                        Some(datum) => {
                            if datum.time > time {
                                time = datum.time;
                            }
                            pos_sum += datum.value.position;
                            vel_sum += datum.value.velocity;
                            acc_sum += datum.value.acceleration;
                            valid_read_count += 1;
                        }
                        None => {}
                    }
                }
                _ => {}
            }
        }
        let valid_read_count = valid_read_count as f32;
        if valid_read_count < 1 {
            return Ok(None);
        }
        let pos = pos_sum / valid_read_count;
        let vel = vel_sum / valid_read_count;
        let acc = acc_sum / valid_read_count;
        Ok(Some(Datum::new(time, State::new(pos, vel, acc))))
    }
}
impl<const N: usize, E: Copy + Debug> Settable<Command, E> for Axle<N, E> {
    fn set(&mut self, value: Command) -> Result<(), Error<E>> {
        for i in 0..N {
            match &mut self.devices[i] {
                Device::ImpreciseWrite(device, posderdepkvals) => {
                    match value.position_derivative {
                        PositionDerivative::Position => {
                            self.pids[i] = Some(PositionDerivativeDependentPIDControllerShift::Position(PIDControllerShift::<1>::new(value.value, posderdepkvals.position.kp, posderdepkvals.position.ki, posderdepkvals.position.kd)));
                        }
                        PositionDerivative::Velocity => {
                            self.pids[i] = Some(PositionDerivativeDependentPIDControllerShift::Velocity(PIDControllerShift::<2>::new(value.value, posderdepkvals.velocity.kp, posderdepkvals.velocity.ki, posderdepkvals.velocity.kd)));
                        }
                        PositionDerivative::Acceleration => {
                            self.pids[i] = Some(PositionDerivativeDependentPIDControllerShift::Acceleration(PIDControllerShift::<3>::new(value.value, posderdepkvals.acceleration.kp, posderdepkvals.acceleration.ki, posderdepkvals.acceleration.kd)));
                        }
                    }
                }
                Device::PreciseWrite(device) => {
                    device.set(value)?;
                }
                Device::ReadWrite(device) => {
                    device.set(value)?;
                }
                Device::Read(_) => {}
            }
        }
        Ok(())
    }
}
#[macro_export]
macro_rules! make_input_getter {
    ($stream:expr, $ttype:tt, $etype:tt) => {
        Rc::new(RefCell::new(
            Box::new($stream) as Box<dyn Getter<$ttype, $etype>>
        ))
    };
}
#[macro_export]
macro_rules! make_input_time_getter {
    ($time_getter:expr, $etype:tt) => {
        Rc::new(RefCell::new(
            Box::new($time_getter) as Box<dyn TimeGetter<$etype>>
        ))
    };
}
///A proportional-integral-derivative controller. Requires `pid` feature.
#[cfg(feature = "pid")]
pub struct PIDController {
    setpoint: f32,
    kp: f32,
    ki: f32,
    kd: f32,
    last_update_time: Option<f32>,
    prev_error: Option<f32>,
    int_error: f32,
}
#[cfg(feature = "pid")]
impl PIDController {
    ///Constructor for `PIDController`.
    pub fn new(setpoint: f32, kp: f32, ki: f32, kd: f32) -> PIDController {
        PIDController {
            setpoint: setpoint,
            kp: kp,
            ki: ki,
            kd: kd,
            last_update_time: None,
            prev_error: None,
            int_error: 0.0,
        }
    }
    ///Update the PID controller. Give it a new time and process variable value, and it will give
    ///you a new control variable value.
    #[must_use]
    pub fn update(&mut self, time: f32, process: f32) -> f32 {
        let error = self.setpoint - process;
        let delta_time = match self.last_update_time {
            None => 0.0,
            Some(x) => time - x,
        };
        let drv_error = match self.prev_error {
            None => 0.0,
            Some(x) => (error - x) / delta_time,
        };
        self.int_error += match self.prev_error {
            Some(x) => delta_time * (x + error) / 2.0,
            None => 0.0,
        };
        self.last_update_time = Some(time);
        self.prev_error = Some(error);
        self.kp * error + self.ki * self.int_error + self.kd * drv_error
    }
}
///A PID controller that will integrate the control variable a given number of times to simplify
///control of some systems such as motors. Requires `pid` feature.
//N is one more than shift count
#[cfg(feature = "pid")]
pub struct PIDControllerShift<const N: usize> {
    setpoint: f32,
    kp: f32,
    ki: f32,
    kd: f32,
    last_update_time: Option<f32>,
    prev_error: Option<f32>,
    int_error: f32,
    shifts: [f32; N],
}
#[cfg(feature = "pid")]
impl<const N: usize> PIDControllerShift<N> {
    ///Constructor for `PIDControllerShift`.
    pub fn new(setpoint: f32, kp: f32, ki: f32, kd: f32) -> Self {
        Self {
            setpoint: setpoint,
            kp: kp,
            ki: ki,
            kd: kd,
            last_update_time: None,
            prev_error: None,
            int_error: 0.0,
            shifts: [0.0; N],
        }
    }
    ///Update the PID controller. Give it a new time and process variable value, and it will give
    ///you a new control variable value.
    #[must_use]
    pub fn update(&mut self, time: f32, process: f32) -> f32 {
        let error = self.setpoint - process;
        let delta_time = match self.last_update_time {
            None => 0.0,
            Some(x) => time - x,
        };
        let drv_error = match self.prev_error {
            None => 0.0,
            Some(x) => (error - x) / delta_time,
        };
        self.int_error += match self.prev_error {
            Some(x) => delta_time * (x + error) / 2.0,
            None => 0.0,
        };
        self.last_update_time = Some(time);
        self.prev_error = Some(error);
        let control = self.kp * error + self.ki * self.int_error + self.kd * drv_error;
        let mut new_shifts = [0.0; N];
        new_shifts[0] = control;
        for i in 1..N {
            let prev_int = self.shifts[i];
            new_shifts[i] = prev_int + delta_time * (self.shifts[i - 1] + new_shifts[i - 1]) / 2.0;
        }
        self.shifts = new_shifts;
        self.shifts[self.shifts.len() - 1]
    }
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
            Err(_) => {
                return None;
            }
        };
        let vel = match self.get_velocity(time) {
            Ok(value) => value,
            Err(_) => {
                return None;
            }
        };
        let acc = match self.get_acceleration(time) {
            Ok(value) => value,
            Err(_) => {
                return None;
            }
        };
        Some(Datum::new(time, State::new(pos, vel, acc)))
    }
}
#[cfg(feature = "motionprofile")]
impl Updatable for MotionProfile {
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
    ///Get the intended `PositionDerivative` at a given time.
    pub fn get_mode(&self, t: f32) -> Result<PositionDerivative, &'static str> {
        if t < 0.0 {
            return Err("time invalid");
        } else if t < self.t1 {
            return Ok(PositionDerivative::Acceleration);
        } else if t < self.t2 {
            return Ok(PositionDerivative::Velocity);
        } else if t < self.t3 {
            return Ok(PositionDerivative::Acceleration);
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
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[cfg(feature = "pid")]
    fn pid_new() {
        let pid = PIDController::new(5.0, 1.0, 0.01, 0.1);
        assert_eq!(pid.setpoint, 5.0);
        assert_eq!(pid.kp, 1.0);
        assert_eq!(pid.ki, 0.01);
        assert_eq!(pid.kd, 0.1);
        assert_eq!(pid.last_update_time, None);
        assert_eq!(pid.prev_error, None);
        assert_eq!(pid.int_error, 0.0);
    }
    #[test]
    #[cfg(feature = "pid")]
    fn pid_initial_update() {
        let mut pid = PIDController::new(5.0, 1.0, 0.01, 0.1);
        let new_control = pid.update(1.0, 0.0);
        assert_eq!(new_control, 5.0);
        assert_eq!(pid.last_update_time, Some(1.0));
        assert_eq!(pid.prev_error, Some(5.0));
        assert_eq!(pid.int_error, 0.0);
    }
    #[test]
    #[cfg(feature = "pid")]
    fn pid_subsequent_update() {
        let mut pid = PIDController::new(5.0, 1.0, 0.01, 0.1);
        let _ = pid.update(1.0, 0.0);
        let new_control = pid.update(3.0, 1.0);
        assert_eq!(new_control, 4.04);
        assert_eq!(pid.int_error, 9.0);
    }
    #[test]
    #[cfg(feature = "pid")]
    fn pidshift_no_shift() {
        let mut pid = PIDControllerShift::<1>::new(5.0, 1.0, 0.01, 0.1);
        let _ = pid.update(1.0, 0.0);
        let new_control = pid.update(3.0, 1.0);
        assert_eq!(new_control, 4.04);
        assert_eq!(pid.shifts, [4.04]);
    }
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
