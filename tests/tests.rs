// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2024-2025 UxuginPython
use rrtk::*;
#[test]
fn state_new_raw() {
    let state = State::new_raw(1.0, 2.0, 3.0);
    assert_eq!(state.position, 1.0);
    assert_eq!(state.velocity, 2.0);
    assert_eq!(state.acceleration, 3.0);
}
#[test]
fn state_update() {
    let mut state = State::new_raw(1.0, 2.0, 3.0);
    state.update(Time(4_000_000_000));
    assert_eq!(state.position, 33.0);
    assert_eq!(state.velocity, 14.0);
    assert_eq!(state.acceleration, 3.0);
}
#[test]
fn state_acceleration_raw() {
    let mut state = State::new_raw(1.0, 2.0, 3.0);
    state.set_constant_acceleration_raw(4.0);
    assert_eq!(state.acceleration, 4.0);
}
#[test]
fn state_velocity_raw() {
    let mut state = State::new_raw(1.0, 2.0, 3.0);
    state.set_constant_velocity_raw(4.0);
    assert_eq!(state.velocity, 4.0);
    assert_eq!(state.acceleration, 0.0);
}
#[test]
fn state_position_raw() {
    let mut state = State::new_raw(1.0, 2.0, 3.0);
    state.set_constant_position_raw(4.0);
    assert_eq!(state.position, 4.0);
    assert_eq!(state.velocity, 0.0);
    assert_eq!(state.acceleration, 0.0);
}
#[test]
fn state_get_value() {
    let state = State::new_raw(1.0, 2.0, 3.0);
    assert_eq!(
        state.get_value(PositionDerivative::Position),
        Quantity::new(1.0, MILLIMETER)
    );
    assert_eq!(
        state.get_value(PositionDerivative::Velocity),
        Quantity::new(2.0, MILLIMETER_PER_SECOND)
    );
    assert_eq!(
        state.get_value(PositionDerivative::Acceleration),
        Quantity::new(3.0, MILLIMETER_PER_SECOND_SQUARED)
    );
}
#[test]
fn state_ops() {
    assert_eq!(
        -State::new_raw(1.0, 2.0, 3.0),
        State::new_raw(-1.0, -2.0, -3.0)
    );
    assert_eq!(
        State::new_raw(1.0, 2.0, 3.0) + State::new_raw(4.0, 5.0, 6.0),
        State::new_raw(5.0, 7.0, 9.0)
    );
    assert_eq!(
        State::new_raw(1.0, 2.0, 3.0) - State::new_raw(4.0, 5.0, 6.0),
        State::new_raw(-3.0, -3.0, -3.0)
    );
    assert_eq!(
        State::new_raw(1.0, 2.0, 3.0) * 2.0,
        State::new_raw(2.0, 4.0, 6.0)
    );
    assert_eq!(
        State::new_raw(1.0, 2.0, 3.0) / 2.0,
        State::new_raw(0.5, 1.0, 1.5)
    );
    let mut state = State::new_raw(1.0, 2.0, 3.0);
    state += State::new_raw(4.0, 5.0, 6.0);
    assert_eq!(state, State::new_raw(5.0, 7.0, 9.0));
    let mut state = State::new_raw(1.0, 2.0, 3.0);
    state -= State::new_raw(4.0, 5.0, 6.0);
    assert_eq!(state, State::new_raw(-3.0, -3.0, -3.0));
    let mut state = State::new_raw(1.0, 2.0, 3.0);
    state *= 2.0;
    assert_eq!(state, State::new_raw(2.0, 4.0, 6.0));
    let mut state = State::new_raw(1.0, 2.0, 3.0);
    state /= 2.0;
    assert_eq!(state, State::new_raw(0.5, 1.0, 1.5));
}
#[test]
fn latest_datum() {
    assert_eq!(
        latest(Datum::new(Time(0), 0), Datum::new(Time(1), 1)),
        Datum::new(Time(1), 1)
    );
    assert_eq!(
        latest(Datum::new(Time(1), 0), Datum::new(Time(0), 1)),
        Datum::new(Time(1), 0)
    );
    assert_eq!(
        latest(Datum::new(Time(0), 0), Datum::new(Time(0), 1)),
        Datum::new(Time(0), 0)
    );
}
#[test]
fn datum_replace_if_older_than() {
    let mut x = Datum::new(Time(2_000_000_000), 2);
    let y = Datum::new(Time(1_000_000_000), 3);
    assert!(!x.replace_if_older_than(y));
    assert_eq!(x, Datum::new(Time(2_000_000_000), 2));
    let y = Datum::new(Time(2_000_000_000), 3);
    assert!(!x.replace_if_older_than(y));
    assert_eq!(x, Datum::new(Time(2_000_000_000), 2));
    let y = Datum::new(Time(3_000_000_000), 3);
    assert!(x.replace_if_older_than(y));
    assert_eq!(x, y);
}
#[test]
fn datum_replace_if_none_or_older_than() {
    let mut x = None;
    let y = Datum::new(Time(2_000_000_000), 2);
    assert!(x.replace_if_none_or_older_than(y));
    assert_eq!(x, Some(y));
    let y = Datum::new(Time(1_000_000_000), 3);
    assert!(!x.replace_if_none_or_older_than(y));
    assert_eq!(x, Some(Datum::new(Time(2_000_000_000), 2)));
    let y = Datum::new(Time(2_000_000_000), 3);
    assert!(!x.replace_if_none_or_older_than(y));
    assert_eq!(x, Some(Datum::new(Time(2_000_000_000), 2)));
    let y = Datum::new(Time(3_000_000_000), 3);
    assert!(x.replace_if_none_or_older_than(y));
    assert_eq!(x, Some(y));
}
#[test]
fn datum_replace_if_none_or_older_than_option() {
    let mut x = None;
    let y = None;
    assert!(!x.replace_if_none_or_older_than_option(y));
    assert_eq!(x, None);
    let y = Some(Datum::new(Time(2_000_000_000), 2));
    assert!(x.replace_if_none_or_older_than_option(y));
    assert_eq!(x, y);
}
#[test]
fn datum_not() {
    assert_eq!(!Datum::new(Time(0), false), Datum::new(Time(0), true));
}
#[test]
fn datum_neg() {
    assert_eq!(-Datum::new(Time(0), 1), Datum::new(Time(0), -1));
}
#[test]
fn datum_add() {
    assert_eq!(
        Datum::new(Time(0), 1) + Datum::new(Time(1), 1),
        Datum::new(Time(1), 2)
    );
    assert_eq!(
        Datum::new(Time(1), 1) + Datum::new(Time(0), 1),
        Datum::new(Time(1), 2)
    );

    let mut x = Datum::new(Time(0), 1);
    x += Datum::new(Time(1), 1);
    assert_eq!(x, Datum::new(Time(1), 2));

    let mut x = Datum::new(Time(1), 1);
    x += Datum::new(Time(0), 1);
    assert_eq!(x, Datum::new(Time(1), 2));

    assert_eq!(Datum::new(Time(0), 1) + 1, Datum::new(Time(0), 2));

    let mut x = Datum::new(Time(0), 1);
    x += 1;
    assert_eq!(x, Datum::new(Time(0), 2));
}
#[test]
fn datum_sub() {
    assert_eq!(
        Datum::new(Time(0), 1) - Datum::new(Time(1), 1),
        Datum::new(Time(1), 0)
    );
    assert_eq!(
        Datum::new(Time(1), 1) - Datum::new(Time(0), 1),
        Datum::new(Time(1), 0)
    );

    let mut x = Datum::new(Time(0), 1);
    x -= Datum::new(Time(1), 1);
    assert_eq!(x, Datum::new(Time(1), 0));

    let mut x = Datum::new(Time(1), 1);
    x -= Datum::new(Time(0), 1);
    assert_eq!(x, Datum::new(Time(1), 0));

    assert_eq!(Datum::new(Time(0), 1) - 1, Datum::new(Time(0), 0));

    let mut x = Datum::new(Time(0), 1);
    x -= 1;
    assert_eq!(x, Datum::new(Time(0), 0));
}
#[test]
fn datum_mul() {
    assert_eq!(
        Datum::new(Time(0), 2) * Datum::new(Time(1), 3),
        Datum::new(Time(1), 6)
    );
    assert_eq!(
        Datum::new(Time(1), 2) * Datum::new(Time(0), 3),
        Datum::new(Time(1), 6)
    );

    let mut x = Datum::new(Time(0), 2);
    x *= Datum::new(Time(1), 3);
    assert_eq!(x, Datum::new(Time(1), 6));

    let mut x = Datum::new(Time(1), 2);
    x *= Datum::new(Time(0), 3);
    assert_eq!(x, Datum::new(Time(1), 6));

    assert_eq!(Datum::new(Time(0), 2) * 3, Datum::new(Time(0), 6));

    let mut x = Datum::new(Time(0), 2);
    x *= 3;
    assert_eq!(x, Datum::new(Time(0), 6));
}
#[test]
fn datum_div() {
    assert_eq!(
        Datum::new(Time(0), 6) / Datum::new(Time(1), 2),
        Datum::new(Time(1), 3)
    );
    assert_eq!(
        Datum::new(Time(1), 6) / Datum::new(Time(0), 2),
        Datum::new(Time(1), 3)
    );

    let mut x = Datum::new(Time(0), 6);
    x /= Datum::new(Time(1), 2);
    assert_eq!(x, Datum::new(Time(1), 3));

    let mut x = Datum::new(Time(1), 6);
    x /= Datum::new(Time(0), 2);
    assert_eq!(x, Datum::new(Time(1), 3));

    assert_eq!(Datum::new(Time(0), 6) / 2, Datum::new(Time(0), 3));

    let mut x = Datum::new(Time(0), 6);
    x /= 2;
    assert_eq!(x, Datum::new(Time(0), 3));
}
#[test]
fn datum_state_mul() {
    assert_eq!(
        Datum::new(Time(0), State::new_raw(1.0, 2.0, 3.0)) * Datum::new(Time(1), 3.0),
        Datum::new(Time(1), State::new_raw(3.0, 6.0, 9.0))
    );
    assert_eq!(
        Datum::new(Time(1), State::new_raw(1.0, 2.0, 3.0)) * Datum::new(Time(0), 3.0),
        Datum::new(Time(1), State::new_raw(3.0, 6.0, 9.0))
    );

    let mut x = Datum::new(Time(0), State::new_raw(1.0, 2.0, 3.0));
    x *= Datum::new(Time(1), 3.0);
    assert_eq!(x, Datum::new(Time(1), State::new_raw(3.0, 6.0, 9.0)));

    let mut x = Datum::new(Time(1), State::new_raw(1.0, 2.0, 3.0));
    x *= Datum::new(Time(0), 3.0);
    assert_eq!(x, Datum::new(Time(1), State::new_raw(3.0, 6.0, 9.0)));

    assert_eq!(
        Datum::new(Time(0), State::new_raw(1.0, 2.0, 3.0)) * 3.0,
        Datum::new(Time(0), State::new_raw(3.0, 6.0, 9.0))
    );

    let mut x = Datum::new(Time(0), State::new_raw(1.0, 2.0, 3.0));
    x *= 3.0;
    assert_eq!(x, Datum::new(Time(0), State::new_raw(3.0, 6.0, 9.0)));
}
#[test]
fn datum_state_div() {
    assert_eq!(
        Datum::new(Time(0), State::new_raw(2.0, 4.0, 6.0)) / Datum::new(Time(1), 2.0),
        Datum::new(Time(1), State::new_raw(1.0, 2.0, 3.0))
    );
    assert_eq!(
        Datum::new(Time(1), State::new_raw(2.0, 4.0, 6.0)) / Datum::new(Time(0), 2.0),
        Datum::new(Time(1), State::new_raw(1.0, 2.0, 3.0))
    );

    let mut x = Datum::new(Time(0), State::new_raw(2.0, 4.0, 6.0));
    x /= Datum::new(Time(1), 2.0);
    assert_eq!(x, Datum::new(Time(1), State::new_raw(1.0, 2.0, 3.0)));

    let mut x = Datum::new(Time(1), State::new_raw(2.0, 4.0, 6.0));
    x /= Datum::new(Time(0), 2.0);
    assert_eq!(x, Datum::new(Time(1), State::new_raw(1.0, 2.0, 3.0)));

    assert_eq!(
        Datum::new(Time(0), State::new_raw(2.0, 4.0, 6.0)) / 2.0,
        Datum::new(Time(0), State::new_raw(1.0, 2.0, 3.0))
    );

    let mut x = Datum::new(Time(0), State::new_raw(2.0, 4.0, 6.0));
    x /= 2.0;
    assert_eq!(x, Datum::new(Time(0), State::new_raw(1.0, 2.0, 3.0)));
}
#[test]
fn pid_k_values_evaluate() {
    let kvals = PIDKValues::new(1.0, 2.0, 3.0);
    assert_eq!(kvals.evaluate(4.0, 5.0, 6.0), 32.0);
    let posderkvals = PositionDerivativeDependentPIDKValues::new(
        PIDKValues::new(1.0, 2.0, 3.0),
        PIDKValues::new(4.0, 5.0, 6.0),
        PIDKValues::new(7.0, 8.0, 9.0),
    );
    assert_eq!(
        posderkvals.get_k_values(PositionDerivative::Position),
        PIDKValues::new(1.0, 2.0, 3.0)
    );
    assert_eq!(
        posderkvals.get_k_values(PositionDerivative::Velocity),
        PIDKValues::new(4.0, 5.0, 6.0)
    );
    assert_eq!(
        posderkvals.get_k_values(PositionDerivative::Acceleration),
        PIDKValues::new(7.0, 8.0, 9.0)
    );
    assert_eq!(
        posderkvals.evaluate(PositionDerivative::Position, 1.0, 2.0, 3.0),
        14.0
    );
    assert_eq!(
        posderkvals.evaluate(PositionDerivative::Velocity, 1.0, 2.0, 3.0),
        32.0
    );
    assert_eq!(
        posderkvals.evaluate(PositionDerivative::Acceleration, 1.0, 2.0, 3.0),
        50.0
    );
}
#[test]
fn motion_profile_get_mode() {
    let motion_profile = MotionProfile::new(
        State::new_raw(0.0, 0.0, 0.0),
        State::new_raw(3.0, 0.0, 0.0),
        Quantity::new(0.1, MILLIMETER_PER_SECOND),
        Quantity::new(0.01, MILLIMETER_PER_SECOND_SQUARED),
    );
    assert_eq!(
        motion_profile.get_mode(Time(5_000_000_000)),
        Some(PositionDerivative::Acceleration)
    );
    assert_eq!(
        motion_profile.get_mode(Time(25_000_000_000)),
        Some(PositionDerivative::Velocity)
    );
    assert_eq!(
        motion_profile.get_mode(Time(35_000_000_000)),
        Some(PositionDerivative::Acceleration)
    );
}
#[test]
fn motion_profile_get_acceleration() {
    let motion_profile = MotionProfile::new(
        State::new_raw(0.0, 0.0, 0.0),
        State::new_raw(3.0, 0.0, 0.0),
        Quantity::new(0.1, MILLIMETER_PER_SECOND),
        Quantity::new(0.01, MILLIMETER_PER_SECOND_SQUARED),
    );
    assert_eq!(motion_profile.get_acceleration(Time(-1_000_000_000)), None);
    assert_eq!(
        motion_profile.get_acceleration(Time(5_000_000_000)),
        Some(Quantity::new(0.01, MILLIMETER_PER_SECOND_SQUARED))
    );
    assert_eq!(
        motion_profile.get_acceleration(Time(25_000_000_000)),
        Some(Quantity::new(0.0, MILLIMETER_PER_SECOND_SQUARED))
    );
    assert_eq!(
        motion_profile.get_acceleration(Time(35_000_000_000)),
        Some(Quantity::new(-0.01, MILLIMETER_PER_SECOND_SQUARED))
    );
    assert_eq!(
        motion_profile.get_acceleration(Time(500_000_000_000)),
        Some(Quantity::new(0.0, MILLIMETER_PER_SECOND_SQUARED))
    );
}
#[test]
fn motion_profile_get_velocity() {
    let motion_profile = MotionProfile::new(
        State::new_raw(0.0, 0.0, 0.0),
        State::new_raw(3.0, 0.0, 0.0),
        Quantity::new(0.1, MILLIMETER_PER_SECOND),
        Quantity::new(0.01, MILLIMETER_PER_SECOND_SQUARED),
    );
    assert_eq!(motion_profile.get_velocity(Time(-1_000_000_000)), None);
    let gv5 = motion_profile
        .get_velocity(Time(5_000_000_000))
        .unwrap()
        .value;
    assert!(0.049 < gv5 && gv5 < 0.051);
    let gv25 = motion_profile
        .get_velocity(Time(25_000_000_000))
        .unwrap()
        .value;
    assert!(0.099 < gv25 && gv25 < 0.101);
    let gv35 = motion_profile
        .get_velocity(Time(35_000_000_000))
        .unwrap()
        .value;
    assert!(0.049 < gv35 && gv35 < 0.051);
    assert_eq!(
        motion_profile.get_velocity(Time(500_000_000_000)),
        Some(Quantity::new(0.0, MILLIMETER_PER_SECOND))
    );
}
#[test]
fn motion_profile_get_velocity_2() {
    let motion_profile = MotionProfile::new(
        State::new_raw(1.0, 0.0, 0.03),
        State::new_raw(4.0, 0.0, 0.0),
        Quantity::new(0.1, MILLIMETER_PER_SECOND),
        Quantity::new(0.01, MILLIMETER_PER_SECOND_SQUARED),
    );
    let gv5 = motion_profile
        .get_velocity(Time(5_000_000_000))
        .unwrap()
        .value;
    assert!(0.049 < gv5 && gv5 < 0.051);
    let gv25 = motion_profile
        .get_velocity(Time(25_000_000_000))
        .unwrap()
        .value;
    assert!(0.099 < gv25 && gv25 < 0.101);
    let gv35 = motion_profile
        .get_velocity(Time(35_000_000_000))
        .unwrap()
        .value;
    assert!(0.049 < gv35 && gv35 < 0.051);
}
#[test]
fn motion_profile_get_velocity_3() {
    let motion_profile = MotionProfile::new(
        State::new_raw(1.0, 0.1, 0.03),
        State::new_raw(6.0, 0.1, 0.0),
        Quantity::new(0.2, MILLIMETER_PER_SECOND),
        Quantity::new(0.01, MILLIMETER_PER_SECOND_SQUARED),
    );
    assert_eq!(
        motion_profile.get_velocity(Time(5_000_000_000)),
        Some(Quantity::new(0.15, MILLIMETER_PER_SECOND))
    );
    let gv15 = motion_profile
        .get_velocity(Time(15_000_000_000))
        .unwrap()
        .value;
    assert!(0.199 < gv15 && gv15 < 0.201);
    assert_eq!(
        motion_profile.get_velocity(Time(25_000_000_000)),
        Some(Quantity::new(0.15, MILLIMETER_PER_SECOND))
    );
}
#[test]
fn motion_profile_get_position() {
    let motion_profile = MotionProfile::new(
        State::new_raw(0.0, 0.0, 0.0),
        State::new_raw(3.0, 0.0, 0.0),
        Quantity::new(0.1, MILLIMETER_PER_SECOND),
        Quantity::new(0.01, MILLIMETER_PER_SECOND_SQUARED),
    );
    assert_eq!(motion_profile.get_position(Time(-1_000_000_000)), None);
    let gp5 = motion_profile
        .get_position(Time(5_000_000_000))
        .unwrap()
        .value;
    assert!(0.124 < gp5 && gp5 < 0.126);
    assert_eq!(
        motion_profile
            .get_position(Time(25_000_000_000))
            .unwrap()
            .value,
        2.0
    );
    assert_eq!(
        motion_profile
            .get_position(Time(35_000_000_000))
            .unwrap()
            .value,
        2.875
    );
    assert_eq!(
        motion_profile
            .get_position(Time(500_000_000_000))
            .unwrap()
            .value,
        3.0
    );
}
#[test]
fn motion_profile_get_position_2() {
    let motion_profile = MotionProfile::new(
        State::new_raw(1.0, 0.0, 0.03),
        State::new_raw(4.0, 0.0, 0.0),
        Quantity::new(0.1, MILLIMETER_PER_SECOND),
        Quantity::new(0.01, MILLIMETER_PER_SECOND_SQUARED),
    );
    assert_eq!(
        motion_profile
            .get_position(Time(5_000_000_000))
            .unwrap()
            .value,
        1.125
    );
    assert_eq!(
        motion_profile
            .get_position(Time(25_000_000_000))
            .unwrap()
            .value,
        3.0
    );
    assert_eq!(
        motion_profile
            .get_position(Time(35_000_000_000))
            .unwrap()
            .value,
        3.875
    );
}
#[test]
fn motion_profile_get_position_3() {
    let motion_profile = MotionProfile::new(
        State::new_raw(1.0, 0.1, 0.03),
        State::new_raw(6.0, 0.1, 0.0),
        Quantity::new(0.2, MILLIMETER_PER_SECOND),
        Quantity::new(0.01, MILLIMETER_PER_SECOND_SQUARED),
    );
    assert_eq!(
        motion_profile
            .get_position(Time(5_000_000_000))
            .unwrap()
            .value,
        1.625
    );
    assert_eq!(
        motion_profile
            .get_position(Time(15_000_000_000))
            .unwrap()
            .value,
        3.5
    );
    assert_eq!(
        motion_profile
            .get_position(Time(25_000_000_000))
            .unwrap()
            .value,
        5.375
    );
}
#[test]
fn motion_profile_history() {
    let motion_profile = MotionProfile::new(
        State::new_raw(0.0, 0.0, 0.0),
        State::new_raw(3.0, 0.0, 0.0),
        Quantity::new(0.1, MILLIMETER_PER_SECOND),
        Quantity::new(0.01, MILLIMETER_PER_SECOND_SQUARED),
    );
    let mut motion_profile = Box::new(motion_profile) as Box<dyn History<Command, ()>>;
    let _ = motion_profile.update().unwrap(); //This should do nothing.
    assert_eq!(motion_profile.get(Time(-20_000_000_000)), None);
    assert_eq!(
        motion_profile.get(Time(5_000_000_000)).unwrap().value,
        Command::new(PositionDerivative::Acceleration, 0.01)
    );
    let g25 = motion_profile.get(Time(25_000_000_000)).unwrap().value;
    assert_eq!(PositionDerivative::from(g25), PositionDerivative::Velocity);
    assert!(0.099 < f32::from(g25) && f32::from(g25) < 0.101f32);
    assert_eq!(
        motion_profile.get(Time(35_000_000_000)).unwrap().value,
        Command::new(PositionDerivative::Acceleration, -0.01)
    );
    assert_eq!(
        motion_profile.get(Time(99999_000_000_000)).unwrap().value,
        Command::new(PositionDerivative::Position, 3.0)
    );
}
#[test]
fn motion_profile_piece() {
    let motion_profile = MotionProfile::new(
        State::new_raw(0.0, 0.0, 0.0),
        State::new_raw(3.0, 0.0, 0.0),
        Quantity::new(0.1, MILLIMETER_PER_SECOND),
        Quantity::new(0.01, MILLIMETER_PER_SECOND_SQUARED),
    );
    assert_eq!(
        motion_profile.get_piece(Time(-20_000_000_000)),
        MotionProfilePiece::BeforeStart
    );
    assert_eq!(
        motion_profile.get_piece(Time(5_000_000_000)),
        MotionProfilePiece::InitialAcceleration
    );
    assert_eq!(
        motion_profile.get_piece(Time(25_000_000_000)),
        MotionProfilePiece::ConstantVelocity
    );
    assert_eq!(
        motion_profile.get_piece(Time(35_000_000_000)),
        MotionProfilePiece::EndAcceleration
    );
    assert_eq!(
        motion_profile.get_piece(Time(500_000_000_000)),
        MotionProfilePiece::Complete
    );
}
#[test]
fn command() {
    let command = Command::new(PositionDerivative::Position, 5.0);
    assert_eq!(command.get_position(), Some(Quantity::new(5.0, MILLIMETER)));
    assert_eq!(
        command.get_velocity(),
        Some(Quantity::new(0.0, MILLIMETER_PER_SECOND))
    );
    assert_eq!(
        command.get_acceleration(),
        Quantity::new(0.0, MILLIMETER_PER_SECOND_SQUARED)
    );
    let command = Command::new(PositionDerivative::Velocity, 5.0);
    assert_eq!(command.get_position(), None);
    assert_eq!(
        command.get_velocity(),
        Some(Quantity::new(5.0, MILLIMETER_PER_SECOND))
    );
    assert_eq!(
        command.get_acceleration(),
        Quantity::new(0.0, MILLIMETER_PER_SECOND_SQUARED)
    );
    let command = Command::new(PositionDerivative::Acceleration, 5.0);
    assert_eq!(command.get_position(), None);
    assert_eq!(command.get_velocity(), None);
    assert_eq!(
        command.get_acceleration(),
        Quantity::new(5.0, MILLIMETER_PER_SECOND_SQUARED)
    );
}
#[test]
fn command_from_state() {
    let command = Command::from(State::new_raw(1.0, 2.0, 3.0));
    assert_eq!(command, Command::new(PositionDerivative::Acceleration, 3.0));
    let command = Command::from(State::new_raw(1.0, 2.0, 0.0));
    assert_eq!(command, Command::new(PositionDerivative::Velocity, 2.0));
    let command = Command::from(State::new_raw(1.0, 0.0, 0.0));
    assert_eq!(command, Command::new(PositionDerivative::Position, 1.0));
}
#[test]
fn command_ops() {
    assert_eq!(-Command::Position(1.0), Command::Position(-1.0));
    assert_eq!(
        Command::Position(2.0) + Command::Position(3.0),
        Command::Position(5.0)
    );
    assert_eq!(
        Command::Position(3.0) - Command::Position(2.0),
        Command::Position(1.0)
    );
    assert_eq!(Command::Position(3.0) * 2.0, Command::Position(6.0),);
    assert_eq!(Command::Position(4.0) / 2.0, Command::Position(2.0));
    let mut x = Command::Position(2.0);
    let y = Command::Position(3.0);
    x += y;
    assert_eq!(x, Command::Position(5.0));
    let mut x = Command::Position(3.0);
    let y = Command::Position(2.0);
    x -= y;
    assert_eq!(x, Command::Position(1.0));
    let mut x = Command::Position(3.0);
    let y = 2.0;
    x *= y;
    assert_eq!(x, Command::Position(6.0));
    let mut x = Command::Position(4.0);
    let y = 2.0;
    x /= y;
    assert_eq!(x, Command::Position(2.0));
}
#[test]
fn time_getter_from_stream() {
    struct Stream {
        time: Time,
    }
    impl Stream {
        const fn new() -> Self {
            Self { time: Time(0) }
        }
    }
    impl Getter<(), ()> for Stream {
        fn get(&self) -> Output<(), ()> {
            match self.time {
                Time(0) => Ok(Some(Datum::new(self.time, ()))),
                Time(1) => Ok(None),
                Time(2) => Err(Error::Other(())),
                _ => panic!("should always be 0, 1, or 2"),
            }
        }
    }
    impl Updatable<()> for Stream {
        fn update(&mut self) -> NothingOrError<()> {
            self.time += Time(1);
            Ok(())
        }
    }
    unsafe {
        static mut STREAM: Stream = Stream::new();
        let stream = Reference::from_ptr(core::ptr::addr_of_mut!(STREAM));
        let mut time_getter = TimeGetterFromGetter::new(stream.clone());
        time_getter.update().unwrap(); //This should do nothing.
        assert_eq!(time_getter.get(), Ok(Time(0)));
        stream.borrow_mut().update().unwrap();
        assert_eq!(time_getter.get(), Err(Error::FromNone));
        stream.borrow_mut().update().unwrap();
        assert_eq!(time_getter.get(), Err(Error::Other(())));
    }
}
#[test]
fn settable() {
    struct MyGetter {
        none: bool,
        value: u8,
    }
    impl MyGetter {
        const fn new() -> Self {
            Self {
                none: true,
                value: 5,
            }
        }
    }
    impl Getter<u8, ()> for MyGetter {
        fn get(&self) -> Output<u8, ()> {
            if self.none {
                return Ok(None);
            }
            Ok(Some(Datum::new(Time(0), self.value)))
        }
    }
    impl Updatable<()> for MyGetter {
        fn update(&mut self) -> NothingOrError<()> {
            self.none = false;
            self.value += 1;
            Ok(())
        }
    }
    struct MySettable {
        settable_data: SettableData<u8, ()>,
    }
    impl MySettable {
        fn new() -> Self {
            Self {
                settable_data: SettableData::new(),
            }
        }
    }
    impl Settable<u8, ()> for MySettable {
        fn get_settable_data_ref(&self) -> &SettableData<u8, ()> {
            &self.settable_data
        }
        fn get_settable_data_mut(&mut self) -> &mut SettableData<u8, ()> {
            &mut self.settable_data
        }
        fn impl_set(&mut self, _: u8) -> NothingOrError<()> {
            Ok(())
        }
    }
    impl Updatable<()> for MySettable {
        fn update(&mut self) -> NothingOrError<()> {
            self.update_following_data()?;
            Ok(())
        }
    }
    let mut my_settable = MySettable::new();
    assert_eq!(my_settable.get_last_request(), None);
    my_settable.set(3).unwrap();
    assert_eq!(my_settable.get_last_request(), Some(3));
    unsafe {
        static mut MY_GETTER: MyGetter = MyGetter::new();
        let my_getter = Reference::from_ptr(core::ptr::addr_of_mut!(MY_GETTER));
        //let my_getter_dyn: Reference<dyn Getter<u8, ()>> = my_getter.clone();
        let x = my_getter.clone();
        let my_getter_dyn = to_dyn!(Getter<u8, ()>, x);
        my_settable.follow(my_getter_dyn);
        my_settable.update().unwrap();
        assert_eq!(my_settable.get_last_request(), Some(3));
        my_getter.borrow_mut().update().unwrap();
        my_settable.update().unwrap();
        assert_eq!(my_settable.get_last_request(), Some(6));
        my_getter.borrow_mut().update().unwrap();
        my_settable.update().unwrap();
        assert_eq!(my_settable.get_last_request(), Some(7));
        my_settable.stop_following();
        my_getter.borrow_mut().update().unwrap();
        my_settable.update().unwrap();
        assert_eq!(my_settable.get_last_request(), Some(7));
    }
}
#[test]
fn getter_from_history() {
    enum UpdateTestState {
        Unneeded,
        Waiting,
        Updated,
        ReturnNone,
    }
    struct MyHistory {
        update_test_state: UpdateTestState,
    }
    impl MyHistory {
        fn new() -> Self {
            Self {
                update_test_state: UpdateTestState::Unneeded,
            }
        }
        fn set_update_test(&mut self) {
            self.update_test_state = UpdateTestState::Waiting;
        }
        fn set_none_test(&mut self) {
            self.update_test_state = UpdateTestState::ReturnNone;
        }
    }
    impl History<i64, ()> for MyHistory {
        fn get(&self, time: Time) -> Option<Datum<i64>> {
            match self.update_test_state {
                UpdateTestState::Unneeded | UpdateTestState::Waiting => {
                    Some(Datum::new(time, time.into()))
                }
                UpdateTestState::Updated => Some(Datum::new(time, 30)),
                UpdateTestState::ReturnNone => None,
            }
        }
    }
    impl Updatable<()> for MyHistory {
        fn update(&mut self) -> NothingOrError<()> {
            match self.update_test_state {
                UpdateTestState::Waiting => self.update_test_state = UpdateTestState::Updated,
                _ => (),
            }
            Ok(())
        }
    }
    struct MyTimeGetter {
        time: Time,
    }
    impl MyTimeGetter {
        const fn new() -> Self {
            Self { time: Time(5) }
        }
    }
    impl TimeGetter<()> for MyTimeGetter {
        fn get(&self) -> TimeOutput<()> {
            Ok(self.time)
        }
    }
    impl Updatable<()> for MyTimeGetter {
        fn update(&mut self) -> NothingOrError<()> {
            self.time += Time(1);
            Ok(())
        }
    }

    let mut my_history = MyHistory::new();
    unsafe {
        static mut TIME_GETTER: MyTimeGetter = MyTimeGetter::new();
        let my_time_getter = Reference::from_ptr(core::ptr::addr_of_mut!(TIME_GETTER));

        {
            let no_delta = GetterFromHistory::new_no_delta(&mut my_history, my_time_getter.clone());
            assert_eq!(no_delta.get().unwrap().unwrap(), Datum::new(Time(5), 5));
            my_time_getter.borrow_mut().update().unwrap();
            assert_eq!(no_delta.get().unwrap().unwrap(), Datum::new(Time(6), 6));
        }

        {
            let start_at_zero =
                GetterFromHistory::new_start_at_zero(&mut my_history, my_time_getter.clone())
                    .unwrap();
            assert_eq!(
                start_at_zero.get().unwrap().unwrap(),
                Datum::new(Time(6), 0)
            );
            my_time_getter.borrow_mut().update().unwrap();
            assert_eq!(
                start_at_zero.get().unwrap().unwrap(),
                Datum::new(Time(7), 1)
            );
        }

        {
            let custom_start = GetterFromHistory::new_custom_start(
                &mut my_history,
                my_time_getter.clone(),
                Time(10),
            )
            .unwrap();
            assert_eq!(
                custom_start.get().unwrap().unwrap(),
                Datum::new(Time(7), 10)
            );
            my_time_getter.borrow_mut().update().unwrap();
            assert_eq!(
                custom_start.get().unwrap().unwrap(),
                Datum::new(Time(8), 11)
            );
        }

        {
            let custom_delta = GetterFromHistory::new_custom_delta(
                &mut my_history,
                my_time_getter.clone(),
                Time(5),
            );
            assert_eq!(
                custom_delta.get().unwrap().unwrap(),
                Datum::new(Time(8), 13)
            );
            my_time_getter.borrow_mut().update().unwrap();
            assert_eq!(
                custom_delta.get().unwrap().unwrap(),
                Datum::new(Time(9), 14)
            );
        }

        {
            let mut getter =
                GetterFromHistory::new_no_delta(&mut my_history, my_time_getter.clone());
            assert_eq!(getter.get().unwrap().unwrap(), Datum::new(Time(9), 9));
            getter.set_delta(Time(5));
            assert_eq!(getter.get().unwrap().unwrap(), Datum::new(Time(9), 14));
            getter.set_time(Time(20)).unwrap();
            assert_eq!(getter.get().unwrap().unwrap(), Datum::new(Time(9), 20));
        }

        {
            my_history.set_update_test();
            let mut getter =
                GetterFromHistory::new_no_delta(&mut my_history, my_time_getter.clone());
            assert_eq!(getter.get().unwrap().unwrap(), Datum::new(Time(9), 9));
            getter.update().unwrap();
            assert_eq!(getter.get().unwrap().unwrap(), Datum::new(Time(10), 30));
        }

        {
            my_history.set_none_test();
            let getter = GetterFromHistory::new_no_delta(&mut my_history, my_time_getter.clone());
            assert_eq!(getter.get().unwrap(), None);
        }
    }
}
#[test]
fn constant_getter() {
    struct MyTimeGetter;
    impl TimeGetter<()> for MyTimeGetter {
        fn get(&self) -> TimeOutput<()> {
            Ok(Time(0))
        }
    }
    impl Updatable<()> for MyTimeGetter {
        fn update(&mut self) -> NothingOrError<()> {
            Ok(())
        }
    }
    unsafe {
        static mut MY_TIME_GETTER: MyTimeGetter = MyTimeGetter;
        let mut constant_getter = ConstantGetter::new(
            Reference::from_ptr(core::ptr::addr_of_mut!(MY_TIME_GETTER)),
            10,
        );
        assert_eq!(constant_getter.get().unwrap().unwrap().value, 10);
        constant_getter.update().unwrap(); //This should do nothing.
        assert_eq!(constant_getter.get().unwrap().unwrap().value, 10);
        constant_getter.set(20).unwrap();
        assert_eq!(constant_getter.get().unwrap().unwrap().value, 20);
    }
}
#[test]
fn none_getter() {
    let mut getter = NoneGetter::new();
    assert_eq!(<NoneGetter as Getter<(), ()>>::get(&getter), Ok(None));
    <NoneGetter as Updatable<()>>::update(&mut getter).unwrap();
    assert_eq!(<NoneGetter as Getter<(), ()>>::get(&getter), Ok(None));
}
