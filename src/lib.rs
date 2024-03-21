// SPDX-License-Identifier: LGPL-3.0-only
/*
Copyright 2024 UxuginPython on GitHub

     This file is part of Rust Robotics ToolKit.

    Rust Robotics ToolKit is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, version 3.

    Rust Robotics ToolKit is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Lesser General Public License for more details.

    You should have received a copy of the GNU Lesser General Public License along with Rust Robotics ToolKit. If not, see <https://www.gnu.org/licenses/>.
*/
#![cfg_attr(not(feature = "std"), no_std)]
pub struct PIDController {
    setpoint: f32,
    kp: f32,
    ki: f32,
    kd: f32,
    last_update_time: Option<f32>,
    prev_error: Option<f32>,
    int_error: f32,
}
impl PIDController {
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
#[cfg(feature = "std")]
pub struct PIDControllerShift {
    setpoint: f32,
    kp: f32,
    ki: f32,
    kd: f32,
    last_update_time: Option<f32>,
    prev_error: Option<f32>,
    int_error: f32,
    shifts: Vec<f32>,
}
#[cfg(feature = "std")]
impl PIDControllerShift {
    pub fn new(setpoint: f32, kp: f32, ki: f32, kd: f32, shift: u8) -> PIDControllerShift {
        let mut shifts = Vec::new();
        for _ in 0..shift + 1 {
            shifts.push(0.0);
        }
        PIDControllerShift {
            setpoint: setpoint,
            kp: kp,
            ki: ki,
            kd: kd,
            last_update_time: None,
            prev_error: None,
            int_error: 0.0,
            shifts: shifts,
        }
    }
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
        let mut new_shifts = vec![control];
        for i in 1..self.shifts.len() {
            let prev_int = self.shifts[i];
            new_shifts.push(prev_int + delta_time * (self.shifts[i - 1] + new_shifts[i - 1]) / 2.0);
        }
        self.shifts = new_shifts;
        self.shifts[self.shifts.len() - 1]
    }
}
#[derive(Clone)]
pub struct State {
    pub position: f32,
    pub velocity: f32,
    pub acceleration: f32,
}
impl State {
    pub fn new(position: f32, velocity: f32, acceleration: f32) -> State {
        State {
            position: position,
            velocity: velocity,
            acceleration: acceleration,
        }
    }
    pub fn update(&mut self, delta_time: f32) {
        let new_velocity = self.velocity + delta_time * self.acceleration;
        let new_position = self.position + delta_time * (self.velocity + new_velocity) / 2.0;
        self.position = new_position;
        self.velocity = new_velocity;
    }
    pub fn set_constant_acceleration(&mut self, acceleration: f32) {
        self.acceleration = acceleration;
    }
    pub fn set_constant_velocity(&mut self, velocity: f32) {
        self.acceleration = 0.0;
        self.velocity = velocity;
    }
    pub fn set_constant_position(&mut self, position: f32) {
        self.acceleration = 0.0;
        self.velocity = 0.0;
        self.position = position;
    }
}
///A container for a time and something else, usually an `f32` or a `State`.
#[derive(Clone)]
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
///A trait for encoders.
pub trait Encoder {
    ///Get the current acceleration, velocity, and position and the time at which they were
    ///recorded.
    fn get_state(&mut self) -> Datum<State>;
}
///Data needed by all `SimpleEncoder` types.
pub struct SimpleEncoderData {
    pub encoder_type: MotorMode,
    pub time: f32,
    pub position: f32,
    pub velocity: f32,
    pub acceleration: f32,
}
///An encoder trait that does the calculus for you. You just need to supply a position, velocity,
///or acceleration, and the others will be calculated.
pub trait SimpleEncoder: Encoder {
    ///Get an immutable reference to the object's `SimpleEncoderData` object.
    fn get_simple_encoder_data_ref(&self) -> &SimpleEncoderData;
    ///Get a mutable reference to the object's `SimpleEncoderData` object.
    fn get_simple_encoder_data_mut(&mut self) -> &mut SimpleEncoderData;
    ///Get a new position, velocity, or acceleration from the encoder along with a time.
    fn device_update(&mut self) -> Datum<f32>;
    ///Get a new position, velocity, or acceleration from the encoder, calculate the others, and
    ///write it all the the object's `SimpleEncoderData`.
    fn update(&mut self) {
        let device_out = self.device_update();
        let data = self.get_simple_encoder_data_ref();
        let old_time = data.time;
        let old_pos = data.position;
        let old_vel = data.velocity;
        let old_acc = data.acceleration;
        let new_time = device_out.time;
        let delta_time = new_time - old_time;
        match data.encoder_type {
            MotorMode::POSITION => {
                let new_pos = device_out.value;
                let new_vel = (new_pos - old_pos) / delta_time;
                let new_acc = (new_vel - old_vel) / delta_time;
                let data = self.get_simple_encoder_data_mut();
                data.time = new_time;
                data.position = new_pos;
                data.velocity = new_vel;
                data.acceleration = new_acc;
            },
            MotorMode::VELOCITY => {
                let new_vel = device_out.value;
                let new_acc = (new_vel - old_vel) / delta_time;
                let new_pos = old_pos + delta_time * (old_vel + new_vel) / 2.0;
                let data = self.get_simple_encoder_data_mut();
                data.time = new_time;
                data.position = new_pos;
                data.velocity = new_vel;
                data.acceleration = new_acc;
            },
            MotorMode::ACCELERATION => {
                let new_acc = device_out.value;
                let new_vel = old_vel + delta_time * (old_acc + new_acc) / 2.0;
                let new_pos = old_pos + delta_time * (old_vel + new_vel) / 2.0;
                let data = self.get_simple_encoder_data_mut();
                data.time = new_time;
                data.position = new_pos;
                data.velocity = new_vel;
                data.acceleration = new_acc;
            },
        }
    }
}
impl<T: SimpleEncoder> Encoder for T {
    fn get_state(&mut self) -> Datum<State> {
        let data = self.get_simple_encoder_data_ref();
        Datum::new(data.time, State::new(data.position, data.velocity, data.acceleration))
    }
}
///A trait for motors with some form of feedback, regardless if we can see it or not.
pub trait FeedbackMotor {
    ///Get the motor's current acceleration, velocity, and position and the time at which they
    ///were recorded.
    fn get_state(&mut self) -> Datum<State>;
    ///Make the motor run at a given acceleration.
    fn set_acceleration(&mut self, acceleration: f32);
    ///Make the motor run at a given velocity.
    fn set_velocity(&mut self, velocity: f32);
    ///Make the mootr go to a given position.
    fn set_position(&mut self, position: f32);
}
///A container for data required by all `ServoMotor` objects.
pub struct ServoMotorData {
    pub acceleration: f32,
    pub velocity: f32,
    pub position: f32,
    pub time: f32,
}
///A trait for servo motors that do their own control theory and do not give us details about their
///measured state.
pub trait ServoMotor: FeedbackMotor {
    ///Get an immutable reference to the object's `ServoMotorData` field.
    fn get_servo_motor_data_ref(&self) -> &ServoMotorData;
    ///Get a mutable reference to the object's `ServoMotorData` field.
    fn get_servo_motor_data_mut(&mut self) -> &mut ServoMotorData;
    ///Get a new time from the computer.
    fn device_get_time(&mut self) -> f32;
    ///Tell the motor to accelerate at a given acceleration.
    fn device_set_acceleration(&mut self, acceleration: f32);
    ///Tell the motor to run at a constant velocity.
    fn device_set_velocity(&mut self, velocity: f32);
    ///Tell the motor to go to a position and stop.
    fn device_set_position(&mut self, position: f32);
}
impl<T: ServoMotor> FeedbackMotor for T {
    fn get_state(&mut self) -> Datum<State> {
        let data = self.get_servo_motor_data_ref();
        Datum::new(data.time, State::new(data.position, data.velocity, data.acceleration))
    }
    fn set_acceleration(&mut self, acceleration: f32) {
        self.device_set_acceleration(acceleration);
        let time = self.device_get_time();
        let data = self.get_servo_motor_data_mut();
        data.acceleration = acceleration;
        data.time = time;
    }
    fn set_velocity(&mut self, velocity: f32) {
        self.device_set_velocity(velocity);
        let time = self.device_get_time();
        let data = self.get_servo_motor_data_mut();
        data.acceleration = 0.0;
        data.velocity = velocity;
        data.time = time;
    }
    fn set_position(&mut self, position: f32) {
        self.device_set_position(position);
        let time = self.device_get_time();
        let data = self.get_servo_motor_data_mut();
        data.acceleration = 0.0;
        data.velocity = 0.0;
        data.position = position;
        data.time = time;
    }
}
///A trait for motors without feedback.
pub trait NonFeedbackMotor {
    ///Run the motor at a given power. You can use this for voltage, percentage, or anything
    ///roughly proportional to them.
    fn set_power(&mut self, power: f32);
}
#[derive(Debug, PartialEq)]
pub enum MotorMode {
    POSITION,
    VELOCITY,
    ACCELERATION,
}
//abs method of f32 does not exist in no_std
#[cfg(not(feature = "std"))]
fn my_abs_f32(num: f32) -> f32 {
    if num >= 0.0 {
        num
    } else {
        -num
    }
}
#[cfg(not(feature = "std"))]
fn my_square_f32(num: f32) -> f32 {
    num * num
}
pub struct MotionProfile {
    start_pos: f32,
    start_vel: f32,
    t1: f32,
    t2: f32,
    t3: f32,
    max_acc: f32,
}
impl MotionProfile {
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
    pub fn get_mode(&self, t: f32) -> Result<MotorMode, &'static str> {
        if t < 0.0 {
            return Err("time invalid");
        } else if t < self.t1 {
            return Ok(MotorMode::ACCELERATION);
        } else if t < self.t2 {
            return Ok(MotorMode::VELOCITY);
        } else if t < self.t3 {
            return Ok(MotorMode::ACCELERATION);
        } else {
            return Err("time invalid");
        }
    }
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
    pub fn get_position(&self, t: f32) -> Result<f32, &'static str> {
        if t < 0.0 {
            return Err("time invalid");
        } else if t < self.t1 {
            #[cfg(feature = "std")]
            return Ok(0.5 * self.max_acc * t.powi(2) + self.start_vel * t + self.start_pos);
            #[cfg(not(feature = "std"))]
            return Ok(0.5 * self.max_acc * my_square_f32(t) + self.start_vel * t + self.start_pos);
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
    /*#[test]
    #[cfg(feature = "std")]
    fn motor_new() {
        let motor = Motor::new(State::new(1.0, 2.0, 3.0), 4.0, MotorMode::ACCELERATION, 3.0);
        assert_eq!(motor.encoder.state.position, 1.0);
        assert_eq!(motor.encoder.state.velocity, 2.0);
        assert_eq!(motor.encoder.state.acceleration, 3.0);
        assert_eq!(motor.encoder.time, 4.0);
        assert_eq!(motor.pid.setpoint, 3.0);
        assert_eq!(motor.pid.kp, 1.0);
        assert_eq!(motor.pid.ki, 0.01);
        assert_eq!(motor.pid.kd, 0.1);
        assert_eq!(motor.pid.shifts.len(), 3);
    }
    #[test]
    #[cfg(feature = "std")]
    fn motor_set_constant() {
        let mut motor = Motor::new(State::new(1.0, 2.0, 3.0), 4.0, MotorMode::ACCELERATION, 3.0);
        motor.set_constant(MotorMode::VELOCITY, 5.0);
        assert_eq!(motor.pid.shifts.len(), 2);
        assert_eq!(motor.pid.setpoint, 5.0);
    }*/
    #[test]
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
    fn pid_initial_update() {
        let mut pid = PIDController::new(5.0, 1.0, 0.01, 0.1);
        let new_control = pid.update(1.0, 0.0);
        assert_eq!(new_control, 5.0);
        assert_eq!(pid.last_update_time, Some(1.0));
        assert_eq!(pid.prev_error, Some(5.0));
        assert_eq!(pid.int_error, 0.0);
    }
    #[test]
    fn pid_subsequent_update() {
        let mut pid = PIDController::new(5.0, 1.0, 0.01, 0.1);
        let _ = pid.update(1.0, 0.0);
        let new_control = pid.update(3.0, 1.0);
        assert_eq!(new_control, 4.04);
        assert_eq!(pid.int_error, 9.0);
    }
    #[test]
    #[cfg(feature = "std")]
    fn pidshift_no_shift() {
        let mut pid = PIDControllerShift::new(5.0, 1.0, 0.01, 0.1, 0);
        let _ = pid.update(1.0, 0.0);
        let new_control = pid.update(3.0, 1.0);
        assert_eq!(new_control, 4.04);
        assert_eq!(pid.shifts, vec![4.04]);
    }
    #[test]
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
