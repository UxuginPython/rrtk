// SPDX-License-Identifier: LGPL-3.0-only
/*
Copyright 2024 UxuginPython on GitHub

     This file is part of Rust Robotics ToolKit.

    Rust Robotics ToolKit is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, version 3.

    Rust Robotics ToolKit is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Lesser General Public License for more details.

    You should have received a copy of the GNU Lesser General Public License along with Rust Robotics ToolKit. If not, see <https://www.gnu.org/licenses/>.
*/
use rrtk::*;
#[test]
fn pidshift_shift() {
    let mut pid = PIDControllerShift::<2>::new(5.0, PIDKValues::new(1.0, 0.01, 0.1));
    let _ = pid.update(1, 0.0);
    let new_control = pid.update(3, 1.0);
    assert_eq!(new_control, 9.04);
}
#[test]
fn state_new() {
    let state = State::new(1.0, 2.0, 3.0);
    assert_eq!(state.position, 1.0);
    assert_eq!(state.velocity, 2.0);
    assert_eq!(state.acceleration, 3.0);
}
#[test]
fn state_update() {
    let mut state = State::new(1.0, 2.0, 3.0);
    state.update(4);
    assert_eq!(state.position, 33.0);
    assert_eq!(state.velocity, 14.0);
    assert_eq!(state.acceleration, 3.0);
}
#[test]
fn state_acceleration() {
    let mut state = State::new(1.0, 2.0, 3.0);
    state.set_constant_acceleration(4.0);
    assert_eq!(state.acceleration, 4.0);
}
#[test]
fn state_velocity() {
    let mut state = State::new(1.0, 2.0, 3.0);
    state.set_constant_velocity(4.0);
    assert_eq!(state.velocity, 4.0);
    assert_eq!(state.acceleration, 0.0);
}
#[test]
fn state_position() {
    let mut state = State::new(1.0, 2.0, 3.0);
    state.set_constant_position(4.0);
    assert_eq!(state.position, 4.0);
    assert_eq!(state.velocity, 0.0);
    assert_eq!(state.acceleration, 0.0);
}
#[test]
fn motion_profile_get_mode() {
    let motion_profile = MotionProfile::new(
        State::new(0.0, 0.0, 0.0),
        State::new(3.0, 0.0, 0.0),
        0.1,
        0.01,
    );
    assert_eq!(
        motion_profile.get_mode(5),
        Some(PositionDerivative::Acceleration)
    );
    assert_eq!(
        motion_profile.get_mode(25),
        Some(PositionDerivative::Velocity)
    );
    assert_eq!(
        motion_profile.get_mode(35),
        Some(PositionDerivative::Acceleration)
    );
}
#[test]
fn motion_profile_get_acceleration() {
    let motion_profile = MotionProfile::new(
        State::new(0.0, 0.0, 0.0),
        State::new(3.0, 0.0, 0.0),
        0.1,
        0.01,
    );
    assert_eq!(motion_profile.get_acceleration(5), Some(0.01));
    assert_eq!(motion_profile.get_acceleration(25), Some(0.0));
    assert_eq!(motion_profile.get_acceleration(35), Some(-0.01));
}
#[test]
fn motion_profile_get_velocity() {
    let motion_profile = MotionProfile::new(
        State::new(0.0, 0.0, 0.0),
        State::new(3.0, 0.0, 0.0),
        0.1,
        0.01,
    );
    let gv5 = motion_profile.get_velocity(5).unwrap();
    assert!(0.049 < gv5 && gv5 < 0.051);
    let gv25 = motion_profile.get_velocity(25).unwrap();
    assert!(0.099 < gv25 && gv25 < 0.101);
    let gv35 = motion_profile.get_velocity(35).unwrap();
    assert!(0.049 < gv35 && gv35 < 0.051);
}
#[test]
fn motion_profile_get_velocity_2() {
    let motion_profile = MotionProfile::new(
        State::new(1.0, 0.0, 0.03),
        State::new(4.0, 0.0, 0.0),
        0.1,
        0.01,
    );
    let gv5 = motion_profile.get_velocity(5).unwrap();
    assert!(0.049 < gv5 && gv5 < 0.051);
    let gv25 = motion_profile.get_velocity(25).unwrap();
    assert!(0.099 < gv25 && gv25 < 0.101);
    let gv35 = motion_profile.get_velocity(35).unwrap();
    assert!(0.049 < gv35 && gv35 < 0.051);
}
#[test]
fn motion_profile_get_velocity_3() {
    let motion_profile = MotionProfile::new(
        State::new(1.0, 0.1, 0.03),
        State::new(6.0, 0.1, 0.0),
        0.2,
        0.01,
    );
    assert_eq!(motion_profile.get_velocity(5), Some(0.15));
    let gv15 = motion_profile.get_velocity(15).unwrap();
    assert!(0.199 < gv15 && gv15 < 0.201);
    assert_eq!(motion_profile.get_velocity(25), Some(0.15));
}
#[test]
fn motion_profile_get_position() {
    let motion_profile = MotionProfile::new(
        State::new(0.0, 0.0, 0.0),
        State::new(3.0, 0.0, 0.0),
        0.1,
        0.01,
    );
    let gp5 = motion_profile.get_position(5).unwrap();
    assert!(0.124 < gp5 && gp5 < 0.126);
    assert_eq!(motion_profile.get_position(25), Some(2.0));
    assert_eq!(motion_profile.get_position(35), Some(2.875));
}
#[test]
fn motion_profile_get_position_2() {
    let motion_profile = MotionProfile::new(
        State::new(1.0, 0.0, 0.03),
        State::new(4.0, 0.0, 0.0),
        0.1,
        0.01,
    );
    assert_eq!(motion_profile.get_position(5), Some(1.125));
    assert_eq!(motion_profile.get_position(25), Some(3.0));
    assert_eq!(motion_profile.get_position(35), Some(3.875));
}
#[test]
fn motion_profile_get_position_3() {
    let motion_profile = MotionProfile::new(
        State::new(1.0, 0.1, 0.03),
        State::new(6.0, 0.1, 0.0),
        0.2,
        0.01,
    );
    assert_eq!(motion_profile.get_position(5), Some(1.625));
    assert_eq!(motion_profile.get_position(15), Some(3.5));
    assert_eq!(motion_profile.get_position(25), Some(5.375));
}
#[test]
fn command() {
    let command = Command::new(PositionDerivative::Position, 5.0);
    assert_eq!(command.get_position(), Some(5.0));
    assert_eq!(command.get_velocity(), Some(0.0));
    assert_eq!(command.get_acceleration(), 0.0);
    let command = Command::new(PositionDerivative::Velocity, 5.0);
    assert_eq!(command.get_position(), None);
    assert_eq!(command.get_velocity(), Some(5.0));
    assert_eq!(command.get_acceleration(), 0.0);
    let command = Command::new(PositionDerivative::Acceleration, 5.0);
    assert_eq!(command.get_position(), None);
    assert_eq!(command.get_velocity(), None);
    assert_eq!(command.get_acceleration(), 5.0);
}
