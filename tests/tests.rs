use rrtk::*;
use core::cell::RefCell;
use std::rc::Rc;
#[test]
#[cfg(feature = "PIDControllerShift")]
fn pidshift_shift() {
    let mut pid = PIDControllerShift::new(5.0, 1.0, 0.01, 0.1, 1);
    let _ = pid.update(1.0, 0.0);
    let new_control = pid.update(3.0, 1.0);
    assert_eq!(new_control, 9.04);
}
#[test]
#[cfg(feature = "State")]
fn state_new() {
    let state = State::new(1.0, 2.0, 3.0);
    assert_eq!(state.position, 1.0);
    assert_eq!(state.velocity, 2.0);
    assert_eq!(state.acceleration, 3.0);
}
#[test]
#[cfg(feature = "State")]
fn state_update() {
    let mut state = State::new(1.0, 2.0, 3.0);
    state.update(4.0);
    assert_eq!(state.position, 33.0);
    assert_eq!(state.velocity, 14.0);
    assert_eq!(state.acceleration, 3.0);
}
#[test]
#[cfg(feature = "State")]
fn state_acceleration() {
    let mut state = State::new(1.0, 2.0, 3.0);
    state.set_constant_acceleration(4.0);
    assert_eq!(state.acceleration, 4.0);
}
#[test]
#[cfg(feature = "State")]
fn state_velocity() {
    let mut state = State::new(1.0, 2.0, 3.0);
    state.set_constant_velocity(4.0);
    assert_eq!(state.velocity, 4.0);
    assert_eq!(state.acceleration, 0.0);
}
#[test]
#[cfg(feature = "State")]
fn state_position() {
    let mut state = State::new(1.0, 2.0, 3.0);
    state.set_constant_position(4.0);
    assert_eq!(state.position, 4.0);
    assert_eq!(state.velocity, 0.0);
    assert_eq!(state.acceleration, 0.0);
}
#[test]
#[cfg(feature = "Encoder")]
fn encoder_new() {
    let encoder = Encoder::new(State::new(1.0, 2.0, 3.0), 4.0);
    assert_eq!(encoder.state.position, 1.0);
    assert_eq!(encoder.state.velocity, 2.0);
    assert_eq!(encoder.state.acceleration, 3.0);
    assert_eq!(encoder.time, 4.0);
}
#[test]
#[cfg(feature = "Encoder")]
fn encoder_update_acceleration() {
    let mut encoder = Encoder::new(State::new(1.0, 2.0, 3.0), 4.0);
    encoder.update_acceleration(6.0, 5.0);
    assert_eq!(encoder.state.position, 13.0);
    assert_eq!(encoder.state.velocity, 10.0);
    assert_eq!(encoder.state.acceleration, 5.0);
}
#[test]
#[cfg(feature = "Encoder")]
fn encoder_update_velocity() {
    let mut encoder = Encoder::new(State::new(1.0, 2.0, 3.0), 4.0);
    encoder.update_velocity(6.0, 5.0);
    assert_eq!(encoder.state.position, 8.0);
    assert_eq!(encoder.state.velocity, 5.0);
    assert_eq!(encoder.state.acceleration, 1.5);
}
#[test]
#[cfg(feature = "Encoder")]
fn encoder_update_position() {
    let mut encoder = Encoder::new(State::new(1.0, 2.0, 3.0), 4.0);
    encoder.update_position(6.0, 5.0);
    assert_eq!(encoder.state.position, 5.0);
    assert_eq!(encoder.state.velocity, 2.0);
    assert_eq!(encoder.state.acceleration, 0.0);
}
#[test]
#[cfg(feature = "Motor")]
fn motor_update() {
    let mut motor = Motor::new(State::new(1.0, 2.0, 3.0), 4.0, MotorMode::ACCELERATION, 3.0);
    motor.encoder.update_acceleration(6.0, 3.0);
    let update = motor.update(6.0);
    assert_eq!(update, 0.0);
    assert_eq!(motor.encoder.state.position, 11.0);
    assert_eq!(motor.encoder.state.velocity, 8.0);
    assert_eq!(motor.encoder.state.acceleration, 3.0);
}
#[test]
#[cfg(feature = "MotorMode")]
#[cfg(feature = "MotionProfile")]
fn motion_profile_get_mode() {
    let motion_profile = MotionProfile::new(
        State::new(0.0, 0.0, 0.0),
        State::new(3.0, 0.0, 0.0),
        1.0,
        1.0,
    );
    assert_eq!(motion_profile.get_mode(0.5), Ok(MotorMode::ACCELERATION));
    assert_eq!(motion_profile.get_mode(2.5), Ok(MotorMode::VELOCITY));
    assert_eq!(motion_profile.get_mode(3.5), Ok(MotorMode::ACCELERATION));
}