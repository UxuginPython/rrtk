//!Control motors and encoders.
//!This module is available only with the `devices` feature enabled.
use crate::*;
///A trait for encoders.
pub trait Encoder {
    ///Get the current acceleration, velocity, and position and the time at which they were
    ///recorded.
    fn get_state(&mut self) -> Datum<State>;
    ///This should be run continually while the device is enabled. If your encoder does not need a
    ///function like this, just implement it as {}.
    fn update(&mut self);
}
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
pub trait SimpleEncoder: Encoder {
    ///Get an immutable reference to the object's `SimpleEncoderData` object.
    fn get_simple_encoder_data_ref(&self) -> &SimpleEncoderData;
    ///Get a mutable reference to the object's `SimpleEncoderData` object.
    fn get_simple_encoder_data_mut(&mut self) -> &mut SimpleEncoderData;
    ///Get a new position, velocity, or acceleration from the encoder along with a time.
    fn device_update(&mut self) -> Datum<f32>;
}
impl<T: SimpleEncoder> Encoder for T {
    fn get_state(&mut self) -> Datum<State> {
        let data = self.get_simple_encoder_data_ref();
        Datum::new(data.time, State::new(data.position, data.velocity, data.acceleration))
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
///Where you are in following a motion profile.
enum MotionProfileState {
    BeforeStart,
    InitialAccel,
    ConstantVel,
    EndAccel,
    Complete,
}
///Data needed by all `FeedbackMotor` objects.
pub struct FeedbackMotorData {
    motion_profile: Option<MotionProfile>,
    mp_start_time: Option<f32>,
    mp_state: Option<MotionProfileState>,
}
impl FeedbackMotorData {
    ///Constructor for `FeedbackMotorData`.
    pub fn new() -> FeedbackMotorData {
        FeedbackMotorData {
            motion_profile: None,
            mp_start_time: None,
            mp_state: None,
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
    ///function like this, just implement it as {}.
    fn update(&mut self);
    ///Set up the object to follow a motion profile.
    fn start_motion_profile(&mut self, motion_profile: MotionProfile) {
        let data = self.get_feedback_motor_data_mut();
        data.motion_profile = Some(motion_profile);
        data.mp_state = Some(MotionProfileState::BeforeStart);
        data.mp_start_time = None;
    }
    ///Call this repeatedly until the motion profile finishes.
    fn update_motion_profile(&mut self) {
        let output = self.get_state();
        let data = self.get_feedback_motor_data_mut();
        if data.mp_state.is_some() {
            match data.mp_state.as_ref().unwrap() {
                MotionProfileState::BeforeStart => {
                    data.mp_state = Some(MotionProfileState::InitialAccel);
                    let new_acc = data.motion_profile.as_ref().unwrap().max_acc;
                    self.set_acceleration(new_acc);
                },
                MotionProfileState::InitialAccel => {
                    if output.time >= data.motion_profile.as_ref().unwrap().t1 {
                        data.mp_state = Some(MotionProfileState::ConstantVel);
                        let max_vel = data.motion_profile.as_ref().unwrap().max_acc * data.motion_profile.as_ref().unwrap().t1 + data.motion_profile.as_ref().unwrap().start_vel;
                        self.set_velocity(max_vel);
                    }
                },
                MotionProfileState::ConstantVel => {
                    if output.time >= data.motion_profile.as_ref().unwrap().t2 {
                        data.mp_state = Some(MotionProfileState::EndAccel);
                        let new_acc = -data.motion_profile.as_ref().unwrap().max_acc;
                        self.set_acceleration(new_acc);
                    }
                },
                MotionProfileState::EndAccel => {
                    if output.time >= data.motion_profile.as_ref().unwrap().t3 {
                        data.mp_state = Some(MotionProfileState::Complete);
                        let max_vel = data.motion_profile.as_ref().unwrap().max_acc * data.motion_profile.as_ref().unwrap().t1 + data.motion_profile.as_ref().unwrap().start_vel;
                        #[cfg(feature = "std")]
                        let t1_pos = 0.5 * data.motion_profile.as_ref().unwrap().max_acc * data.motion_profile.as_ref().unwrap().t1.powi(2) + data.motion_profile.as_ref().unwrap().start_vel * data.motion_profile.as_ref().unwrap().t1 + data.motion_profile.as_ref().unwrap().start_pos;
                        #[cfg(not(feature = "std"))]
                        let t1_pos = 0.5 * data.motion_profile.as_ref().unwrap().max_acc * my_square_f32(data.motion_profile.as_ref().unwrap().t1) + data.motion_profile.as_ref().unwrap().start_vel * data.motion_profile.as_ref().unwrap().t1 + data.motion_profile.as_ref().unwrap().start_pos;
                        let t2_pos = max_vel * (data.motion_profile.as_ref().unwrap().t2 - data.motion_profile.as_ref().unwrap().t1) + t1_pos;
                        #[cfg(feature = "std")]
                        let t3_pos = 0.5 * -data.motion_profile.as_ref().unwrap().max_acc * (data.motion_profile.as_ref().unwrap().t3 - data.motion_profile.as_ref().unwrap().t2).powi(2) + max_vel * (data.motion_profile.as_ref().unwrap().t3 - data.motion_profile.as_ref().unwrap().t2) + t2_pos;
                        #[cfg(not(feature = "std"))]
                        let t3_pos = 0.5 * -data.motion_profile.as_ref().unwrap().max_acc * my_square_f32(data.motion_profile.as_ref().unwrap().t3 - data.motion_profile.as_ref().unwrap().t2) + max_vel * (data.motion_profile.as_ref().unwrap().t3 - data.motion_profile.as_ref().unwrap().t2) + t2_pos;
                        self.set_position(t3_pos);
                    }
                },
                MotionProfileState::Complete => {},
            }
        }
    }
    ///Follow a motion profile, waiting for it to complete.
    fn follow_motion_profile(&mut self, motion_profile: MotionProfile) {
        let max_vel = motion_profile.max_acc * motion_profile.t1 + motion_profile.start_vel;
        #[cfg(feature = "std")]
        let t1_pos = 0.5 * motion_profile.max_acc * motion_profile.t1.powi(2) + motion_profile.start_vel * motion_profile.t1 + motion_profile.start_pos;
        #[cfg(not(feature = "std"))]
        let t1_pos = 0.5 * motion_profile.max_acc * my_square_f32(motion_profile.t1) + motion_profile.start_vel * motion_profile.t1 + motion_profile.start_pos;
        let t2_pos = max_vel * (motion_profile.t2 - motion_profile.t1) + t1_pos;
        #[cfg(feature = "std")]
        let t3_pos = 0.5 * -motion_profile.max_acc * (motion_profile.t3 - motion_profile.t2).powi(2) + max_vel * (motion_profile.t3 - motion_profile.t2) + t2_pos;
        #[cfg(not(feature = "std"))]
        let t3_pos = 0.5 * -motion_profile.max_acc * my_square_f32(motion_profile.t3 - motion_profile.t2) + max_vel * (motion_profile.t3 - motion_profile.t2) + t2_pos;
        let mut time = 0.0;
        let output = self.get_state();
        let start_time = output.time;
        self.set_acceleration(motion_profile.max_acc);
        while time-start_time < motion_profile.t1 {
            self.update();
            time = self.get_state().time;
        }
        self.set_velocity(max_vel);
        while time-start_time < motion_profile.t2 {
            self.update();
            time = self.get_state().time;
        }
        self.set_acceleration(-motion_profile.max_acc);
        while time-start_time < motion_profile.t3 {
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
    ///function like this, just implement it as {}.
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
///like a servo.
#[cfg(feature = "std")]
pub struct MotorEncoderPair {
    feedback_motor_data: FeedbackMotorData,
    motor: Box<dyn NonFeedbackMotor>,
    encoder: Box<dyn Encoder>,
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
#[cfg(feature = "std")]
impl MotorEncoderPair {
    ///Constructor for `MotorEncoderPair`.
    pub fn new(motor: Box<dyn NonFeedbackMotor>, encoder: Box<dyn Encoder>, pos_kp: f32, pos_ki: f32, pos_kd: f32, vel_kp: f32, vel_ki: f32, vel_kd: f32, acc_kp: f32, acc_ki: f32, acc_kd: f32) -> MotorEncoderPair {
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
#[cfg(feature = "std")]
impl FeedbackMotor for MotorEncoderPair {
    fn get_feedback_motor_data_ref(&self) -> &FeedbackMotorData {
        &self.feedback_motor_data
    }
    fn get_feedback_motor_data_mut(&mut self) -> &mut FeedbackMotorData {
        &mut self.feedback_motor_data
    }
    fn get_state(&mut self) -> Datum<State> {
        self.encoder.get_state()
    }
    fn set_acceleration(&mut self, acceleration: f32) {
        self.mode = Some(MotorMode::ACCELERATION);
        self.pid = Some(PIDControllerShift::new(acceleration, self.acc_kp, self.acc_ki, self.acc_kd, 2));
    }
    fn set_velocity(&mut self, velocity: f32) {
        self.mode = Some(MotorMode::VELOCITY);
        self.pid = Some(PIDControllerShift::new(velocity, self.vel_kp, self.vel_ki, self.vel_kd, 1));
    }
    fn set_position(&mut self, position: f32) {
        self.mode = Some(MotorMode::POSITION);
        self.pid = Some(PIDControllerShift::new(position, self.pos_kp, self.pos_ki, self.pos_kd, 0));
    }
    fn update(&mut self) {
        self.encoder.update();
        let output = self.get_state();
        if self.pid.is_some() {
            let pid_out = self.pid.as_mut().unwrap().update(output.time, match self.mode.as_ref().expect("if pid is Some, mode is too") {
                MotorMode::POSITION => output.value.position,
                MotorMode::VELOCITY => output.value.velocity,
                MotorMode::ACCELERATION => output.value.acceleration,
            });
            self.motor.set_power(pid_out);
        }
    }
}
