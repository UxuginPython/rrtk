// SPDX-License-Identifier: LGPL-3.0-only
/*
Copyright 2024 UxuginPython on GitHub

     This file is part of Rust Robotics ToolKit.

    Rust Robotics ToolKit is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, version 3.

    Rust Robotics ToolKit is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Lesser General Public License for more details.

    You should have received a copy of the GNU Lesser General Public License along with Rust Robotics ToolKit. If not, see <https://www.gnu.org/licenses/>.
*/
//!Control motors and encoders.
//!This module is available only with the `devices` feature enabled.
use crate::*;
///Data needed by all `SimpleEncoder` types.
pub struct SimpleEncoderData {
    pub encoder_type: MotorMode,
    pub time: f32,
    pub position: f32,
    pub velocity: f32,
    pub acceleration: f32,
}
impl SimpleEncoderData {
    ///Constructor for `SimpleEncoderData`.
    pub fn new(encoder_type: MotorMode, start_state: Datum<State>) -> SimpleEncoderData {
        SimpleEncoderData {
            encoder_type: encoder_type,
            time: start_state.time,
            position: start_state.value.position,
            velocity: start_state.value.velocity,
            acceleration: start_state.value.acceleration,
        }
    }
}
///An encoder trait that does the calculus for you. You just need to supply a position, velocity,
///or acceleration, and the others will be calculated.
pub trait SimpleEncoder<E> {
    ///Get an immutable reference to the object's `SimpleEncoderData` object.
    fn get_simple_encoder_data_ref(&self) -> &SimpleEncoderData;
    ///Get a mutable reference to the object's `SimpleEncoderData` object.
    fn get_simple_encoder_data_mut(&mut self) -> &mut SimpleEncoderData;
    ///Get a new position, velocity, or acceleration from the encoder along with a time.
    fn device_update(&mut self) -> Datum<f32>;
}
//impl<E: Copy + Debug> Stream<State, E> for dyn SimpleEncoder<E> {
impl<T: SimpleEncoder<E>, E: Copy + Debug> Stream<State, E> for T {
    fn get(&self) -> StreamOutput<State, E> {
        let data = self.get_simple_encoder_data_ref();
        Ok(Some(Datum::new(
            data.time,
            State::new(data.position, data.velocity, data.acceleration),
        )))
    }
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
            MotorMode::Position => {
                let new_pos = device_out.value;
                let new_vel = (new_pos - old_pos) / delta_time;
                let new_acc = (new_vel - old_vel) / delta_time;
                let data = self.get_simple_encoder_data_mut();
                data.time = new_time;
                data.position = new_pos;
                data.velocity = new_vel;
                data.acceleration = new_acc;
            }
            MotorMode::Velocity => {
                let new_vel = device_out.value;
                let new_acc = (new_vel - old_vel) / delta_time;
                let new_pos = old_pos + delta_time * (old_vel + new_vel) / 2.0;
                let data = self.get_simple_encoder_data_mut();
                data.time = new_time;
                data.position = new_pos;
                data.velocity = new_vel;
                data.acceleration = new_acc;
            }
            MotorMode::Acceleration => {
                let new_acc = device_out.value;
                let new_vel = old_vel + delta_time * (old_acc + new_acc) / 2.0;
                let new_pos = old_pos + delta_time * (old_vel + new_vel) / 2.0;
                let data = self.get_simple_encoder_data_mut();
                data.time = new_time;
                data.position = new_pos;
                data.velocity = new_vel;
                data.acceleration = new_acc;
            }
        }
    }
}
///Where you are in following a motion profile.
pub enum MotionProfileState {
    BeforeStart,
    InitialAccel,
    ConstantVel,
    EndAccel,
    Complete,
}
///Data needed by all `FeedbackMotor` objects.
pub enum FeedbackMotorData {
    WithoutMotionProfile,
    #[cfg(feature = "motionprofile")]
    WithMotionProfile {
        motion_profile: MotionProfile,
        start_time: Option<f32>,
        state: MotionProfileState,
    },
}
impl FeedbackMotorData {
    ///Constructor for `FeedbackMotorData`.
    pub fn new() -> FeedbackMotorData {
        FeedbackMotorData::WithoutMotionProfile
    }
    #[cfg(feature = "motionprofile")]
    fn new_with(motion_profile: MotionProfile) -> FeedbackMotorData {
        FeedbackMotorData::WithMotionProfile {
            motion_profile: motion_profile,
            start_time: None,
            state: MotionProfileState::BeforeStart,
        }
    }
}
///A trait for motors with some form of feedback, regardless if we can see it or not.
pub trait FeedbackMotor {
    fn get_feedback_motor_data_ref(&self) -> &FeedbackMotorData;
    fn get_feedback_motor_data_mut(&mut self) -> &mut FeedbackMotorData;
    ///Get the motor's current acceleration, velocity, and position and the time at which they
    ///were recorded.
    fn get_state(&mut self) -> Datum<State>;
    ///Make the motor run at a given acceleration.
    fn set_acceleration(&mut self, acceleration: f32);
    ///Make the motor run at a given velocity.
    fn set_velocity(&mut self, velocity: f32);
    ///Make the mootr go to a given position.
    fn set_position(&mut self, position: f32);
    ///This should be run continually while the device is enabled. If your motor does not need a
    ///function like this, just implement it as `{}`.
    fn update(&mut self);
    ///Set up the object to follow a motion profile.
    #[cfg(feature = "motionprofile")]
    fn start_motion_profile(&mut self, motion_profile: MotionProfile) {
        let data = self.get_feedback_motor_data_mut();
        *data = FeedbackMotorData::new_with(motion_profile);
    }
    ///Call this repeatedly until the motion profile finishes.
    #[cfg(feature = "motionprofile")]
    fn update_motion_profile(&mut self) {
        let output = self.get_state();
        let data = self.get_feedback_motor_data_mut();
        match data {
            FeedbackMotorData::WithoutMotionProfile => {}
            FeedbackMotorData::WithMotionProfile {
                motion_profile,
                start_time,
                state,
            } => {
                match state {
                    MotionProfileState::BeforeStart => {
                        *state = MotionProfileState::InitialAccel;
                        *start_time = Some(output.time);
                        let new_acc = motion_profile.max_acc;
                        self.set_acceleration(new_acc);
                    }
                    MotionProfileState::InitialAccel => {
                        if output.time
                            - start_time.expect("start_time is only none when state is BeforeStart")
                            >= motion_profile.t1
                        {
                            *state = MotionProfileState::ConstantVel;
                            let max_vel = motion_profile.max_acc * motion_profile.t1
                                + motion_profile.start_vel;
                            self.set_velocity(max_vel);
                        }
                    }
                    MotionProfileState::ConstantVel => {
                        if output.time - start_time.unwrap() >= motion_profile.t2 {
                            *state = MotionProfileState::EndAccel;
                            let new_acc = -motion_profile.max_acc;
                            self.set_acceleration(new_acc);
                        }
                    }
                    MotionProfileState::EndAccel => {
                        if output.time - start_time.unwrap() >= motion_profile.t3 {
                            *state = MotionProfileState::Complete;
                            let max_vel = motion_profile.max_acc * motion_profile.t1
                                + motion_profile.start_vel;
                            let t1_pos = 0.5
                                * motion_profile.max_acc
                                * motion_profile.t1 //easiest way to square without std
                                * motion_profile.t1
                                + motion_profile.start_vel * motion_profile.t1
                                + motion_profile.start_pos;
                            let t2_pos = max_vel * (motion_profile.t2 - motion_profile.t1) + t1_pos;
                            let t3_pos = 0.5
                                * -motion_profile.max_acc
                                * (motion_profile.t3 - motion_profile.t2)
                                * (motion_profile.t3 - motion_profile.t2)
                                + max_vel * (motion_profile.t3 - motion_profile.t2)
                                + t2_pos;
                            self.set_position(t3_pos);
                        }
                    }
                    MotionProfileState::Complete => {}
                }
            }
        }
    }
    ///Follow a motion profile, waiting for it to complete.
    #[cfg(feature = "motionprofile")]
    fn follow_motion_profile(&mut self, motion_profile: MotionProfile) {
        let max_vel = motion_profile.max_acc * motion_profile.t1 + motion_profile.start_vel;
        let t1_pos = 0.5 * motion_profile.max_acc * motion_profile.t1 * motion_profile.t1
            + motion_profile.start_vel * motion_profile.t1
            + motion_profile.start_pos;
        let t2_pos = max_vel * (motion_profile.t2 - motion_profile.t1) + t1_pos;
        let t3_pos = 0.5
            * -motion_profile.max_acc
            * (motion_profile.t3 - motion_profile.t2)
            * (motion_profile.t3 - motion_profile.t2)
            + max_vel * (motion_profile.t3 - motion_profile.t2)
            + t2_pos;
        let mut time = 0.0;
        let output = self.get_state();
        let start_time = output.time;
        self.set_acceleration(motion_profile.max_acc);
        while time - start_time < motion_profile.t1 {
            self.update();
            time = self.get_state().time;
        }
        self.set_velocity(max_vel);
        while time - start_time < motion_profile.t2 {
            self.update();
            time = self.get_state().time;
        }
        self.set_acceleration(-motion_profile.max_acc);
        while time - start_time < motion_profile.t3 {
            self.update();
            time = self.get_state().time;
        }
        self.set_position(t3_pos);
    }
}
///A container for data required by all `ServoMotor` objects.
pub struct ServoMotorData {
    pub feedback_motor_data: FeedbackMotorData,
    pub acceleration: f32,
    pub velocity: f32,
    pub position: f32,
    pub time: f32,
}
impl ServoMotorData {
    ///Constructor for `ServoMotorData`.
    pub fn new(start_state: Datum<State>) -> ServoMotorData {
        ServoMotorData {
            feedback_motor_data: FeedbackMotorData::new(),
            acceleration: start_state.value.acceleration,
            velocity: start_state.value.velocity,
            position: start_state.value.position,
            time: start_state.time,
        }
    }
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
    ///This should be run continually while the device is enabled. If your motor does not need a
    ///function like this, just implement it as `{}`.
    fn device_update(&mut self);
}
impl<T: ServoMotor> FeedbackMotor for T {
    fn get_feedback_motor_data_ref(&self) -> &FeedbackMotorData {
        let data = self.get_servo_motor_data_ref();
        &data.feedback_motor_data
    }
    fn get_feedback_motor_data_mut(&mut self) -> &mut FeedbackMotorData {
        let data = self.get_servo_motor_data_mut();
        &mut data.feedback_motor_data
    }
    fn get_state(&mut self) -> Datum<State> {
        let data = self.get_servo_motor_data_ref();
        Datum::new(
            data.time,
            State::new(data.position, data.velocity, data.acceleration),
        )
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
    fn update(&mut self) {
        self.device_update();
        let time = self.device_get_time();
        let data = self.get_servo_motor_data_mut();
        data.time = time;
    }
}
///A trait for motors without feedback.
pub trait NonFeedbackMotor {
    ///Run the motor at a given power. You can use this for voltage, percentage, or anything
    ///roughly proportional to them.
    fn set_power(&mut self, power: f32);
}
///Use an encoder connected directly to a motor without feedback and a PID controller to control it
///like a servo. Requires `std` and `pid` features.
#[cfg(all(feature = "std", feature = "pid"))]
pub struct MotorEncoderPair<E> {
    feedback_motor_data: FeedbackMotorData,
    motor: Box<dyn NonFeedbackMotor>,
    encoder: InputStream<State, E>,
    pid: Option<PIDControllerShift>,
    mode: Option<MotorMode>,
    pos_kp: f32,
    pos_ki: f32,
    pos_kd: f32,
    vel_kp: f32,
    vel_ki: f32,
    vel_kd: f32,
    acc_kp: f32,
    acc_ki: f32,
    acc_kd: f32,
}
#[cfg(all(feature = "std", feature = "pid"))]
impl<E> MotorEncoderPair<E> {
    ///Constructor for `MotorEncoderPair`.
    pub fn new(
        motor: Box<dyn NonFeedbackMotor>,
        encoder: InputStream<State, E>,
        pos_kp: f32,
        pos_ki: f32,
        pos_kd: f32,
        vel_kp: f32,
        vel_ki: f32,
        vel_kd: f32,
        acc_kp: f32,
        acc_ki: f32,
        acc_kd: f32,
    ) -> MotorEncoderPair<E> {
        MotorEncoderPair {
            feedback_motor_data: FeedbackMotorData::new(),
            motor: motor,
            encoder: encoder,
            pid: None,
            mode: None,
            pos_kp: pos_kp,
            pos_ki: pos_ki,
            pos_kd: pos_kd,
            vel_kp: vel_kp,
            vel_ki: vel_ki,
            vel_kd: vel_kd,
            acc_kp: acc_kp,
            acc_ki: acc_ki,
            acc_kd: acc_kd,
        }
    }
}
#[cfg(all(feature = "std", feature = "pid"))]
impl<E: Copy + Debug> FeedbackMotor for MotorEncoderPair<E> {
    fn get_feedback_motor_data_ref(&self) -> &FeedbackMotorData {
        &self.feedback_motor_data
    }
    fn get_feedback_motor_data_mut(&mut self) -> &mut FeedbackMotorData {
        &mut self.feedback_motor_data
    }
    fn get_state(&mut self) -> Datum<State> {
        self.encoder.borrow().get().unwrap().unwrap()
    }
    fn set_acceleration(&mut self, acceleration: f32) {
        self.mode = Some(MotorMode::Acceleration);
        self.pid = Some(PIDControllerShift::new(
            acceleration,
            self.acc_kp,
            self.acc_ki,
            self.acc_kd,
            2,
        ));
    }
    fn set_velocity(&mut self, velocity: f32) {
        self.mode = Some(MotorMode::Velocity);
        self.pid = Some(PIDControllerShift::new(
            velocity,
            self.vel_kp,
            self.vel_ki,
            self.vel_kd,
            1,
        ));
    }
    fn set_position(&mut self, position: f32) {
        self.mode = Some(MotorMode::Position);
        self.pid = Some(PIDControllerShift::new(
            position,
            self.pos_kp,
            self.pos_ki,
            self.pos_kd,
            0,
        ));
    }
    fn update(&mut self) {
        self.encoder.borrow_mut().update();
        let output = self.get_state();
        if self.pid.is_some() {
            let pid_out = self.pid.as_mut().unwrap().update(
                output.time,
                match self.mode.as_ref().expect("if pid is Some, mode is too") {
                    MotorMode::Position => output.value.position,
                    MotorMode::Velocity => output.value.velocity,
                    MotorMode::Acceleration => output.value.acceleration,
                },
            );
            self.motor.set_power(pid_out);
        }
    }
}
