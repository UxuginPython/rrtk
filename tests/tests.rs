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
    let _ = pid.update(1.0, 0.0);
    let new_control = pid.update(3.0, 1.0);
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
    state.update(4.0);
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
        1.0,
        1.0,
    );
    assert_eq!(
        motion_profile.get_mode(0.5),
        Some(PositionDerivative::Acceleration)
    );
    assert_eq!(
        motion_profile.get_mode(2.5),
        Some(PositionDerivative::Velocity)
    );
    assert_eq!(
        motion_profile.get_mode(3.5),
        Some(PositionDerivative::Acceleration)
    );
}
#[test]
fn motion_profile_get_acceleration() {
    let motion_profile = MotionProfile::new(
        State::new(0.0, 0.0, 0.0),
        State::new(3.0, 0.0, 0.0),
        1.0,
        1.0,
    );
    assert_eq!(motion_profile.get_acceleration(0.5), Some(1.0));
    assert_eq!(motion_profile.get_acceleration(2.5), Some(0.0));
    assert_eq!(motion_profile.get_acceleration(3.5), Some(-1.0));
}
#[test]
fn motion_profile_get_velocity() {
    let motion_profile = MotionProfile::new(
        State::new(0.0, 0.0, 0.0),
        State::new(3.0, 0.0, 0.0),
        1.0,
        1.0,
    );
    assert_eq!(motion_profile.get_velocity(0.5), Some(0.5));
    assert_eq!(motion_profile.get_velocity(2.5), Some(1.0));
    assert_eq!(motion_profile.get_velocity(3.5), Some(0.5));
}
#[test]
fn motion_profile_get_velocity_2() {
    let motion_profile = MotionProfile::new(
        State::new(1.0, 0.0, 3.0),
        State::new(4.0, 0.0, 0.0),
        1.0,
        1.0,
    );
    assert_eq!(motion_profile.get_velocity(0.5), Some(0.5));
    assert_eq!(motion_profile.get_velocity(2.5), Some(1.0));
    assert_eq!(motion_profile.get_velocity(3.5), Some(0.5));
}
#[test]
fn motion_profile_get_velocity_3() {
    let motion_profile = MotionProfile::new(
        State::new(1.0, 1.0, 3.0),
        State::new(6.0, 1.0, 0.0),
        2.0,
        1.0,
    );
    assert_eq!(motion_profile.get_velocity(0.5), Some(1.5));
    assert_eq!(motion_profile.get_velocity(1.5), Some(2.0));
    assert_eq!(motion_profile.get_velocity(2.5), Some(1.5));
}
#[test]
fn motion_profile_get_position() {
    let motion_profile = MotionProfile::new(
        State::new(0.0, 0.0, 0.0),
        State::new(3.0, 0.0, 0.0),
        1.0,
        1.0,
    );
    assert_eq!(motion_profile.get_position(0.5), Some(0.125));
    assert_eq!(motion_profile.get_position(2.5), Some(2.0));
    assert_eq!(motion_profile.get_position(3.5), Some(2.875));
}
#[test]
fn motion_profile_get_position_2() {
    let motion_profile = MotionProfile::new(
        State::new(1.0, 0.0, 3.0),
        State::new(4.0, 0.0, 0.0),
        1.0,
        1.0,
    );
    assert_eq!(motion_profile.get_position(0.5), Some(1.125));
    assert_eq!(motion_profile.get_position(2.5), Some(3.0));
    assert_eq!(motion_profile.get_position(3.5), Some(3.875));
}
#[test]
fn motion_profile_get_position_3() {
    let motion_profile = MotionProfile::new(
        State::new(1.0, 1.0, 3.0),
        State::new(6.0, 1.0, 0.0),
        2.0,
        1.0,
    );
    assert_eq!(motion_profile.get_position(0.5), Some(1.625));
    assert_eq!(motion_profile.get_position(1.5), Some(3.5));
    assert_eq!(motion_profile.get_position(2.5), Some(5.375));
}
