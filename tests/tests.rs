// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2024-2025 UxuginPython
use rrtk::*;
#[test]
fn state_new() {
    let state = State::new(
        Millimeter::new(1.0),
        MillimeterPerSecond::new(2.0),
        MillimeterPerSecondSquared::new(3.0),
    );
    assert_eq!(state.position, Millimeter::new(1.0));
    assert_eq!(state.velocity, MillimeterPerSecond::new(2.0));
    assert_eq!(state.acceleration, MillimeterPerSecondSquared::new(3.0));
}
#[test]
fn state_update() {
    let mut state = State::new(
        Millimeter::new(1.0),
        MillimeterPerSecond::new(2.0),
        MillimeterPerSecondSquared::new(3.0),
    );
    state.update(Time::from_nanoseconds(4_000_000_000));
    assert_eq!(state.position, Millimeter::new(33.0));
    assert_eq!(state.velocity, MillimeterPerSecond::new(14.0));
    assert_eq!(state.acceleration, MillimeterPerSecondSquared::new(3.0));
}
#[test]
fn state_acceleration() {
    let mut state = State::new(
        Millimeter::new(1.0),
        MillimeterPerSecond::new(2.0),
        MillimeterPerSecondSquared::new(3.0),
    );
    state.set_constant_acceleration(MillimeterPerSecondSquared::new(4.0));
    assert_eq!(state.acceleration, MillimeterPerSecondSquared::new(4.0));
}
#[test]
fn state_velocity_raw() {
    let mut state = State::new(
        Millimeter::new(1.0),
        MillimeterPerSecond::new(2.0),
        MillimeterPerSecondSquared::new(3.0),
    );
    state.set_constant_velocity(MillimeterPerSecond::new(4.0));
    assert_eq!(state.velocity, MillimeterPerSecond::new(4.0));
    assert_eq!(state.acceleration, MillimeterPerSecondSquared::new(0.0));
}
#[test]
fn state_position_raw() {
    let mut state = State::new(
        Millimeter::new(1.0),
        MillimeterPerSecond::new(2.0),
        MillimeterPerSecondSquared::new(3.0),
    );
    state.set_constant_position(Millimeter::new(4.0));
    assert_eq!(state.position, Millimeter::new(4.0));
    assert_eq!(state.velocity, MillimeterPerSecond::new(0.0));
    assert_eq!(state.acceleration, MillimeterPerSecondSquared::new(0.0));
}
#[test]
fn state_get_value() {
    let state = State::new(
        Millimeter::new(1.0),
        MillimeterPerSecond::new(2.0),
        MillimeterPerSecondSquared::new(3.0),
    );
    assert_eq!(state.get_value(PositionDerivative::Position), 1.0);
    assert_eq!(state.get_value(PositionDerivative::Velocity), 2.0);
    assert_eq!(state.get_value(PositionDerivative::Acceleration), 3.0);
}
#[test]
fn state_ops() {
    assert_eq!(
        -State::new(
            Millimeter::new(1.0),
            MillimeterPerSecond::new(2.0),
            MillimeterPerSecondSquared::new(3.0)
        ),
        State::new(
            Millimeter::new(-1.0),
            MillimeterPerSecond::new(-2.0),
            MillimeterPerSecondSquared::new(-3.0)
        )
    );
    assert_eq!(
        State::new(
            Millimeter::new(1.0),
            MillimeterPerSecond::new(2.0),
            MillimeterPerSecondSquared::new(3.0)
        ) + State::new(
            Millimeter::new(4.0),
            MillimeterPerSecond::new(5.0),
            MillimeterPerSecondSquared::new(6.0)
        ),
        State::new(
            Millimeter::new(5.0),
            MillimeterPerSecond::new(7.0),
            MillimeterPerSecondSquared::new(9.0)
        )
    );
    assert_eq!(
        State::new(
            Millimeter::new(1.0),
            MillimeterPerSecond::new(2.0),
            MillimeterPerSecondSquared::new(3.0)
        ) - State::new(
            Millimeter::new(4.0),
            MillimeterPerSecond::new(5.0),
            MillimeterPerSecondSquared::new(6.0)
        ),
        State::new(
            Millimeter::new(-3.0),
            MillimeterPerSecond::new(-3.0),
            MillimeterPerSecondSquared::new(-3.0)
        )
    );
    assert_eq!(
        State::new(
            Millimeter::new(1.0),
            MillimeterPerSecond::new(2.0),
            MillimeterPerSecondSquared::new(3.0)
        ) * Dimensionless::new(2.0),
        State::new(
            Millimeter::new(2.0),
            MillimeterPerSecond::new(4.0),
            MillimeterPerSecondSquared::new(6.0)
        )
    );
    assert_eq!(
        State::new(
            Millimeter::new(1.0),
            MillimeterPerSecond::new(2.0),
            MillimeterPerSecondSquared::new(3.0)
        ) / Dimensionless::new(2.0),
        State::new(
            Millimeter::new(0.5),
            MillimeterPerSecond::new(1.0),
            MillimeterPerSecondSquared::new(1.5)
        )
    );
    let mut state = State::new(
        Millimeter::new(1.0),
        MillimeterPerSecond::new(2.0),
        MillimeterPerSecondSquared::new(3.0),
    );
    state += State::new(
        Millimeter::new(4.0),
        MillimeterPerSecond::new(5.0),
        MillimeterPerSecondSquared::new(6.0),
    );
    assert_eq!(
        state,
        State::new(
            Millimeter::new(5.0),
            MillimeterPerSecond::new(7.0),
            MillimeterPerSecondSquared::new(9.0)
        )
    );
    let mut state = State::new(
        Millimeter::new(1.0),
        MillimeterPerSecond::new(2.0),
        MillimeterPerSecondSquared::new(3.0),
    );
    state -= State::new(
        Millimeter::new(4.0),
        MillimeterPerSecond::new(5.0),
        MillimeterPerSecondSquared::new(6.0),
    );
    assert_eq!(
        state,
        State::new(
            Millimeter::new(-3.0),
            MillimeterPerSecond::new(-3.0),
            MillimeterPerSecondSquared::new(-3.0)
        )
    );
    let mut state = State::new(
        Millimeter::new(1.0),
        MillimeterPerSecond::new(2.0),
        MillimeterPerSecondSquared::new(3.0),
    );
    state *= Dimensionless::new(2.0);
    assert_eq!(
        state,
        State::new(
            Millimeter::new(2.0),
            MillimeterPerSecond::new(4.0),
            MillimeterPerSecondSquared::new(6.0)
        )
    );
    let mut state = State::new(
        Millimeter::new(1.0),
        MillimeterPerSecond::new(2.0),
        MillimeterPerSecondSquared::new(3.0),
    );
    state /= Dimensionless::new(2.0);
    assert_eq!(
        state,
        State::new(
            Millimeter::new(0.5),
            MillimeterPerSecond::new(1.0),
            MillimeterPerSecondSquared::new(1.5)
        )
    );
}
#[test]
fn latest_datum() {
    assert_eq!(
        latest(
            Datum::new(Time::ZERO, 0),
            Datum::new(Time::from_nanoseconds(1), 1)
        ),
        Datum::new(Time::from_nanoseconds(1), 1)
    );
    assert_eq!(
        latest(
            Datum::new(Time::from_nanoseconds(1), 0),
            Datum::new(Time::ZERO, 1)
        ),
        Datum::new(Time::from_nanoseconds(1), 0)
    );
    assert_eq!(
        latest(Datum::new(Time::ZERO, 0), Datum::new(Time::ZERO, 1)),
        Datum::new(Time::ZERO, 0)
    );
}
#[test]
fn datum_replace_if_older_than() {
    let mut x = Datum::new(Time::from_nanoseconds(2_000_000_000), 2);
    let y = Datum::new(Time::from_nanoseconds(1_000_000_000), 3);
    assert!(!x.replace_if_older_than(y));
    assert_eq!(x, Datum::new(Time::from_nanoseconds(2_000_000_000), 2));
    let y = Datum::new(Time::from_nanoseconds(2_000_000_000), 3);
    assert!(!x.replace_if_older_than(y));
    assert_eq!(x, Datum::new(Time::from_nanoseconds(2_000_000_000), 2));
    let y = Datum::new(Time::from_nanoseconds(3_000_000_000), 3);
    assert!(x.replace_if_older_than(y));
    assert_eq!(x, y);
}
#[test]
fn datum_replace_if_none_or_older_than() {
    let mut x = None;
    let y = Datum::new(Time::from_nanoseconds(2_000_000_000), 2);
    assert!(x.replace_if_none_or_older_than(y));
    assert_eq!(x, Some(y));
    let y = Datum::new(Time::from_nanoseconds(1_000_000_000), 3);
    assert!(!x.replace_if_none_or_older_than(y));
    assert_eq!(
        x,
        Some(Datum::new(Time::from_nanoseconds(2_000_000_000), 2))
    );
    let y = Datum::new(Time::from_nanoseconds(2_000_000_000), 3);
    assert!(!x.replace_if_none_or_older_than(y));
    assert_eq!(
        x,
        Some(Datum::new(Time::from_nanoseconds(2_000_000_000), 2))
    );
    let y = Datum::new(Time::from_nanoseconds(3_000_000_000), 3);
    assert!(x.replace_if_none_or_older_than(y));
    assert_eq!(x, Some(y));
}
#[test]
fn datum_replace_if_none_or_older_than_option() {
    let mut x = None;
    let y = None;
    assert!(!x.replace_if_none_or_older_than_option(y));
    assert_eq!(x, None);
    let y = Some(Datum::new(Time::from_nanoseconds(2_000_000_000), 2));
    assert!(x.replace_if_none_or_older_than_option(y));
    assert_eq!(x, y);
}
#[test]
fn datum_not() {
    assert_eq!(!Datum::new(Time::ZERO, false), Datum::new(Time::ZERO, true));
}
#[test]
fn datum_neg() {
    assert_eq!(-Datum::new(Time::ZERO, 1), Datum::new(Time::ZERO, -1));
}
#[test]
fn datum_add() {
    assert_eq!(
        Datum::new(Time::ZERO, 1) + Datum::new(Time::from_nanoseconds(1), 1),
        Datum::new(Time::from_nanoseconds(1), 2)
    );
    assert_eq!(
        Datum::new(Time::from_nanoseconds(1), 1) + Datum::new(Time::ZERO, 1),
        Datum::new(Time::from_nanoseconds(1), 2)
    );

    let mut x = Datum::new(Time::ZERO, 1);
    x += Datum::new(Time::from_nanoseconds(1), 1);
    assert_eq!(x, Datum::new(Time::from_nanoseconds(1), 2));

    let mut x = Datum::new(Time::from_nanoseconds(1), 1);
    x += Datum::new(Time::ZERO, 1);
    assert_eq!(x, Datum::new(Time::from_nanoseconds(1), 2));

    assert_eq!(Datum::new(Time::ZERO, 1) + 1, Datum::new(Time::ZERO, 2));

    let mut x = Datum::new(Time::ZERO, 1);
    x += 1;
    assert_eq!(x, Datum::new(Time::ZERO, 2));
}
#[test]
fn datum_sub() {
    assert_eq!(
        Datum::new(Time::ZERO, 1) - Datum::new(Time::from_nanoseconds(1), 1),
        Datum::new(Time::from_nanoseconds(1), 0)
    );
    assert_eq!(
        Datum::new(Time::from_nanoseconds(1), 1) - Datum::new(Time::ZERO, 1),
        Datum::new(Time::from_nanoseconds(1), 0)
    );

    let mut x = Datum::new(Time::ZERO, 1);
    x -= Datum::new(Time::from_nanoseconds(1), 1);
    assert_eq!(x, Datum::new(Time::from_nanoseconds(1), 0));

    let mut x = Datum::new(Time::from_nanoseconds(1), 1);
    x -= Datum::new(Time::ZERO, 1);
    assert_eq!(x, Datum::new(Time::from_nanoseconds(1), 0));

    assert_eq!(Datum::new(Time::ZERO, 1) - 1, Datum::new(Time::ZERO, 0));

    let mut x = Datum::new(Time::ZERO, 1);
    x -= 1;
    assert_eq!(x, Datum::new(Time::ZERO, 0));
}
#[test]
fn datum_mul() {
    assert_eq!(
        Datum::new(Time::ZERO, 2) * Datum::new(Time::from_nanoseconds(1), 3),
        Datum::new(Time::from_nanoseconds(1), 6)
    );
    assert_eq!(
        Datum::new(Time::from_nanoseconds(1), 2) * Datum::new(Time::ZERO, 3),
        Datum::new(Time::from_nanoseconds(1), 6)
    );

    let mut x = Datum::new(Time::ZERO, 2);
    x *= Datum::new(Time::from_nanoseconds(1), 3);
    assert_eq!(x, Datum::new(Time::from_nanoseconds(1), 6));

    let mut x = Datum::new(Time::from_nanoseconds(1), 2);
    x *= Datum::new(Time::ZERO, 3);
    assert_eq!(x, Datum::new(Time::from_nanoseconds(1), 6));

    assert_eq!(Datum::new(Time::ZERO, 2) * 3, Datum::new(Time::ZERO, 6));

    let mut x = Datum::new(Time::ZERO, 2);
    x *= 3;
    assert_eq!(x, Datum::new(Time::ZERO, 6));
}
#[test]
fn datum_div() {
    assert_eq!(
        Datum::new(Time::ZERO, 6) / Datum::new(Time::from_nanoseconds(1), 2),
        Datum::new(Time::from_nanoseconds(1), 3)
    );
    assert_eq!(
        Datum::new(Time::from_nanoseconds(1), 6) / Datum::new(Time::ZERO, 2),
        Datum::new(Time::from_nanoseconds(1), 3)
    );

    let mut x = Datum::new(Time::ZERO, 6);
    x /= Datum::new(Time::from_nanoseconds(1), 2);
    assert_eq!(x, Datum::new(Time::from_nanoseconds(1), 3));

    let mut x = Datum::new(Time::from_nanoseconds(1), 6);
    x /= Datum::new(Time::ZERO, 2);
    assert_eq!(x, Datum::new(Time::from_nanoseconds(1), 3));

    assert_eq!(Datum::new(Time::ZERO, 6) / 2, Datum::new(Time::ZERO, 3));

    let mut x = Datum::new(Time::ZERO, 6);
    x /= 2;
    assert_eq!(x, Datum::new(Time::ZERO, 3));
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
        State::new(
            Millimeter::new(0.0),
            MillimeterPerSecond::new(0.0),
            MillimeterPerSecondSquared::new(0.0),
        ),
        State::new(
            Millimeter::new(3.0),
            MillimeterPerSecond::new(0.0),
            MillimeterPerSecondSquared::new(0.0),
        ),
        MillimeterPerSecond::new(0.1),
        MillimeterPerSecondSquared::new(0.01),
    );
    assert_eq!(
        motion_profile.get_mode(Time::from_nanoseconds(5_000_000_000)),
        Some(PositionDerivative::Acceleration)
    );
    assert_eq!(
        motion_profile.get_mode(Time::from_nanoseconds(25_000_000_000)),
        Some(PositionDerivative::Velocity)
    );
    assert_eq!(
        motion_profile.get_mode(Time::from_nanoseconds(35_000_000_000)),
        Some(PositionDerivative::Acceleration)
    );
}
#[test]
fn motion_profile_get_acceleration() {
    let motion_profile = MotionProfile::new(
        State::new(
            Millimeter::new(0.0),
            MillimeterPerSecond::new(0.0),
            MillimeterPerSecondSquared::new(0.0),
        ),
        State::new(
            Millimeter::new(3.0),
            MillimeterPerSecond::new(0.0),
            MillimeterPerSecondSquared::new(0.0),
        ),
        MillimeterPerSecond::new(0.1),
        MillimeterPerSecondSquared::new(0.01),
    );
    assert_eq!(
        motion_profile.get_acceleration(Time::from_nanoseconds(-1_000_000_000)),
        None
    );
    assert_eq!(
        motion_profile.get_acceleration(Time::from_nanoseconds(5_000_000_000)),
        Some(MillimeterPerSecondSquared::new(0.01))
    );
    assert_eq!(
        motion_profile.get_acceleration(Time::from_nanoseconds(25_000_000_000)),
        Some(MillimeterPerSecondSquared::new(0.0))
    );
    assert_eq!(
        motion_profile.get_acceleration(Time::from_nanoseconds(35_000_000_000)),
        Some(MillimeterPerSecondSquared::new(-0.01))
    );
    assert_eq!(
        motion_profile.get_acceleration(Time::from_nanoseconds(500_000_000_000)),
        Some(MillimeterPerSecondSquared::new(0.0))
    );
}
#[test]
fn motion_profile_get_velocity() {
    let motion_profile = MotionProfile::new(
        State::new(
            Millimeter::new(0.0),
            MillimeterPerSecond::new(0.0),
            MillimeterPerSecondSquared::new(0.0),
        ),
        State::new(
            Millimeter::new(3.0),
            MillimeterPerSecond::new(0.0),
            MillimeterPerSecondSquared::new(0.0),
        ),
        MillimeterPerSecond::new(0.1),
        MillimeterPerSecondSquared::new(0.01),
    );
    assert_eq!(
        motion_profile.get_velocity(Time::from_nanoseconds(-1_000_000_000)),
        None
    );
    let gv5 = motion_profile
        .get_velocity(Time::from_nanoseconds(5_000_000_000))
        .unwrap()
        .into_inner();
    assert!(0.049 < gv5 && gv5 < 0.051);
    let gv25 = motion_profile
        .get_velocity(Time::from_nanoseconds(25_000_000_000))
        .unwrap()
        .into_inner();
    assert!(0.099 < gv25 && gv25 < 0.101);
    let gv35 = motion_profile
        .get_velocity(Time::from_nanoseconds(35_000_000_000))
        .unwrap()
        .into_inner();
    assert!(0.049 < gv35 && gv35 < 0.051);
    assert_eq!(
        motion_profile.get_velocity(Time::from_nanoseconds(500_000_000_000)),
        Some(MillimeterPerSecond::new(0.0))
    );
}
#[test]
fn motion_profile_get_velocity_2() {
    let motion_profile = MotionProfile::new(
        State::new(
            Millimeter::new(1.0),
            MillimeterPerSecond::new(0.0),
            MillimeterPerSecondSquared::new(0.03),
        ),
        State::new(
            Millimeter::new(4.0),
            MillimeterPerSecond::new(0.0),
            MillimeterPerSecondSquared::new(0.0),
        ),
        MillimeterPerSecond::new(0.1),
        MillimeterPerSecondSquared::new(0.01),
    );
    let gv5 = motion_profile
        .get_velocity(Time::from_nanoseconds(5_000_000_000))
        .unwrap()
        .into_inner();
    assert!(0.049 < gv5 && gv5 < 0.051);
    let gv25 = motion_profile
        .get_velocity(Time::from_nanoseconds(25_000_000_000))
        .unwrap()
        .into_inner();
    assert!(0.099 < gv25 && gv25 < 0.101);
    let gv35 = motion_profile
        .get_velocity(Time::from_nanoseconds(35_000_000_000))
        .unwrap()
        .into_inner();
    assert!(0.049 < gv35 && gv35 < 0.051);
}
#[test]
fn motion_profile_get_velocity_3() {
    let motion_profile = MotionProfile::new(
        State::new(
            Millimeter::new(1.0),
            MillimeterPerSecond::new(0.1),
            MillimeterPerSecondSquared::new(0.03),
        ),
        State::new(
            Millimeter::new(6.0),
            MillimeterPerSecond::new(0.1),
            MillimeterPerSecondSquared::new(0.0),
        ),
        MillimeterPerSecond::new(0.2),
        MillimeterPerSecondSquared::new(0.01),
    );
    assert_eq!(
        motion_profile.get_velocity(Time::from_nanoseconds(5_000_000_000)),
        Some(MillimeterPerSecond::new(0.15))
    );
    let gv15 = motion_profile
        .get_velocity(Time::from_nanoseconds(15_000_000_000))
        .unwrap()
        .into_inner();
    assert!(0.199 < gv15 && gv15 < 0.201);
    assert_eq!(
        motion_profile.get_velocity(Time::from_nanoseconds(25_000_000_000)),
        Some(MillimeterPerSecond::new(0.15))
    );
}
#[test]
fn motion_profile_get_position() {
    let motion_profile = MotionProfile::new(
        State::new(
            Millimeter::new(0.0),
            MillimeterPerSecond::new(0.0),
            MillimeterPerSecondSquared::new(0.0),
        ),
        State::new(
            Millimeter::new(3.0),
            MillimeterPerSecond::new(0.0),
            MillimeterPerSecondSquared::new(0.0),
        ),
        MillimeterPerSecond::new(0.1),
        MillimeterPerSecondSquared::new(0.01),
    );
    assert_eq!(
        motion_profile.get_position(Time::from_nanoseconds(-1_000_000_000)),
        None
    );
    let gp5 = motion_profile
        .get_position(Time::from_nanoseconds(5_000_000_000))
        .unwrap()
        .into_inner();
    assert!(0.124 < gp5 && gp5 < 0.126);
    assert_eq!(
        motion_profile
            .get_position(Time::from_nanoseconds(25_000_000_000))
            .unwrap()
            .into_inner(),
        2.0
    );
    assert_eq!(
        motion_profile
            .get_position(Time::from_nanoseconds(35_000_000_000))
            .unwrap()
            .into_inner(),
        2.875
    );
    assert_eq!(
        motion_profile
            .get_position(Time::from_nanoseconds(500_000_000_000))
            .unwrap()
            .into_inner(),
        3.0
    );
}
#[test]
fn motion_profile_get_position_2() {
    let motion_profile = MotionProfile::new(
        State::new(
            Millimeter::new(1.0),
            MillimeterPerSecond::new(0.0),
            MillimeterPerSecondSquared::new(0.03),
        ),
        State::new(
            Millimeter::new(4.0),
            MillimeterPerSecond::new(0.0),
            MillimeterPerSecondSquared::new(0.0),
        ),
        MillimeterPerSecond::new(0.1),
        MillimeterPerSecondSquared::new(0.01),
    );
    assert_eq!(
        motion_profile
            .get_position(Time::from_nanoseconds(5_000_000_000))
            .unwrap()
            .into_inner(),
        1.125
    );
    assert_eq!(
        motion_profile
            .get_position(Time::from_nanoseconds(25_000_000_000))
            .unwrap()
            .into_inner(),
        3.0
    );
    assert_eq!(
        motion_profile
            .get_position(Time::from_nanoseconds(35_000_000_000))
            .unwrap()
            .into_inner(),
        3.875
    );
}
#[test]
fn motion_profile_get_position_3() {
    let motion_profile = MotionProfile::new(
        State::new(
            Millimeter::new(1.0),
            MillimeterPerSecond::new(0.1),
            MillimeterPerSecondSquared::new(0.03),
        ),
        State::new(
            Millimeter::new(6.0),
            MillimeterPerSecond::new(0.1),
            MillimeterPerSecondSquared::new(0.0),
        ),
        MillimeterPerSecond::new(0.2),
        MillimeterPerSecondSquared::new(0.01),
    );
    assert_eq!(
        motion_profile
            .get_position(Time::from_nanoseconds(5_000_000_000))
            .unwrap()
            .into_inner(),
        1.625
    );
    assert_eq!(
        motion_profile
            .get_position(Time::from_nanoseconds(15_000_000_000))
            .unwrap()
            .into_inner(),
        3.5
    );
    assert_eq!(
        motion_profile
            .get_position(Time::from_nanoseconds(25_000_000_000))
            .unwrap()
            .into_inner(),
        5.375
    );
}
#[test]
fn motion_profile_chronology() {
    let motion_profile = MotionProfile::new(
        State::new(
            Millimeter::new(0.0),
            MillimeterPerSecond::new(0.0),
            MillimeterPerSecondSquared::new(0.0),
        ),
        State::new(
            Millimeter::new(3.0),
            MillimeterPerSecond::new(0.0),
            MillimeterPerSecondSquared::new(0.0),
        ),
        MillimeterPerSecond::new(0.1),
        MillimeterPerSecondSquared::new(0.01),
    );
    let motion_profile = Box::new(motion_profile) as Box<dyn Chronology<Command>>;
    assert_eq!(
        motion_profile.get(Time::from_nanoseconds(-20_000_000_000)),
        None
    );
    assert_eq!(
        motion_profile
            .get(Time::from_nanoseconds(5_000_000_000))
            .unwrap()
            .value,
        Command::new(PositionDerivative::Acceleration, 0.01)
    );
    let g25 = motion_profile
        .get(Time::from_nanoseconds(25_000_000_000))
        .unwrap()
        .value;
    assert_eq!(PositionDerivative::from(g25), PositionDerivative::Velocity);
    assert!(0.099 < f32::from(g25) && f32::from(g25) < 0.101f32);
    assert_eq!(
        motion_profile
            .get(Time::from_nanoseconds(35_000_000_000))
            .unwrap()
            .value,
        Command::new(PositionDerivative::Acceleration, -0.01)
    );
    assert_eq!(
        motion_profile
            .get(Time::from_nanoseconds(99999_000_000_000))
            .unwrap()
            .value,
        Command::new(PositionDerivative::Position, 3.0)
    );
}
#[test]
fn motion_profile_piece() {
    let motion_profile = MotionProfile::new(
        State::new(
            Millimeter::new(0.0),
            MillimeterPerSecond::new(0.0),
            MillimeterPerSecondSquared::new(0.0),
        ),
        State::new(
            Millimeter::new(3.0),
            MillimeterPerSecond::new(0.0),
            MillimeterPerSecondSquared::new(0.0),
        ),
        MillimeterPerSecond::new(0.1),
        MillimeterPerSecondSquared::new(0.01),
    );
    assert_eq!(
        motion_profile.get_piece(Time::from_nanoseconds(-20_000_000_000)),
        MotionProfilePiece::BeforeStart
    );
    assert_eq!(
        motion_profile.get_piece(Time::from_nanoseconds(5_000_000_000)),
        MotionProfilePiece::InitialAcceleration
    );
    assert_eq!(
        motion_profile.get_piece(Time::from_nanoseconds(25_000_000_000)),
        MotionProfilePiece::ConstantVelocity
    );
    assert_eq!(
        motion_profile.get_piece(Time::from_nanoseconds(35_000_000_000)),
        MotionProfilePiece::EndAcceleration
    );
    assert_eq!(
        motion_profile.get_piece(Time::from_nanoseconds(500_000_000_000)),
        MotionProfilePiece::Complete
    );
}
#[test]
fn command() {
    let command = Command::new(PositionDerivative::Position, 5.0);
    assert_eq!(command.get_position(), Some(Millimeter::new(5.0)));
    assert_eq!(command.get_velocity(), Some(MillimeterPerSecond::new(0.0)));
    assert_eq!(
        command.get_acceleration(),
        MillimeterPerSecondSquared::new(0.0)
    );
    let command = Command::new(PositionDerivative::Velocity, 5.0);
    assert_eq!(command.get_position(), None);
    assert_eq!(command.get_velocity(), Some(MillimeterPerSecond::new(5.0)));
    assert_eq!(
        command.get_acceleration(),
        MillimeterPerSecondSquared::new(0.0)
    );
    let command = Command::new(PositionDerivative::Acceleration, 5.0);
    assert_eq!(command.get_position(), None);
    assert_eq!(command.get_velocity(), None);
    assert_eq!(
        command.get_acceleration(),
        MillimeterPerSecondSquared::new(5.0)
    );
}
#[test]
fn command_from_state() {
    let command = Command::from(State::new(
        Millimeter::new(1.0),
        MillimeterPerSecond::new(2.0),
        MillimeterPerSecondSquared::new(3.0),
    ));
    assert_eq!(command, Command::new(PositionDerivative::Acceleration, 3.0));
    let command = Command::from(State::new(
        Millimeter::new(1.0),
        MillimeterPerSecond::new(2.0),
        MillimeterPerSecondSquared::new(0.0),
    ));
    assert_eq!(command, Command::new(PositionDerivative::Velocity, 2.0));
    let command = Command::from(State::new(
        Millimeter::new(1.0),
        MillimeterPerSecond::new(0.0),
        MillimeterPerSecondSquared::new(0.0),
    ));
    assert_eq!(command, Command::new(PositionDerivative::Position, 1.0));
}
#[test]
fn command_ops() {
    assert_eq!(
        -Command::Position(Millimeter::new(1.0)),
        Command::Position(Millimeter::new(-1.0))
    );
    assert_eq!(
        Command::Position(Millimeter::new(2.0)) + Command::Position(Millimeter::new(3.0)),
        Command::Position(Millimeter::new(5.0))
    );
    assert_eq!(
        Command::Position(Millimeter::new(3.0)) - Command::Position(Millimeter::new(2.0)),
        Command::Position(Millimeter::new(1.0))
    );
    assert_eq!(
        Command::Position(Millimeter::new(3.0)) * Dimensionless::new(2.0),
        Command::Position(Millimeter::new(6.0))
    );
    assert_eq!(
        Command::Position(Millimeter::new(4.0)) / Dimensionless::new(2.0),
        Command::Position(Millimeter::new(2.0))
    );
    let mut x = Command::Position(Millimeter::new(2.0));
    let y = Command::Position(Millimeter::new(3.0));
    x += y;
    assert_eq!(x, Command::Position(Millimeter::new(5.0)));
    let mut x = Command::Position(Millimeter::new(3.0));
    let y = Command::Position(Millimeter::new(2.0));
    x -= y;
    assert_eq!(x, Command::Position(Millimeter::new(1.0)));
    let mut x = Command::Position(Millimeter::new(3.0));
    let y = Dimensionless::new(2.0);
    x *= y;
    assert_eq!(x, Command::Position(Millimeter::new(6.0)));
    let mut x = Command::Position(Millimeter::new(4.0));
    let y = Dimensionless::new(2.0);
    x /= y;
    assert_eq!(x, Command::Position(Millimeter::new(2.0)));
}
#[test]
fn time_getter_from_getter() {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum Error {
        GetterNone,
        GetterError,
    }
    struct Stream {
        time: Time,
    }
    impl Stream {
        const fn new() -> Self {
            Self { time: Time::ZERO }
        }
    }
    impl Getter<(), Error> for Stream {
        fn get(&self) -> Output<(), Error> {
            match self.time.as_nanoseconds() {
                0 => Ok(Some(Datum::new(self.time, ()))),
                1 => Ok(None),
                2 => Err(Error::GetterError),
                _ => panic!("should always be 0, 1, or 2"),
            }
        }
    }
    impl Updatable<Error> for Stream {
        fn update(&mut self) -> NothingOrError<Error> {
            self.time += Time::from_nanoseconds(1);
            Ok(())
        }
    }
    static mut STREAM: Stream = Stream::new();
    let mut stream = unsafe { PointerDereferencer::new(core::ptr::addr_of_mut!(STREAM)) };
    let mut time_getter = TimeGetterFromGetter::new(stream, Error::GetterNone);
    time_getter.update().unwrap(); //This should do nothing.
    assert_eq!(time_getter.get(), Ok(Time::ZERO));
    stream.update().unwrap();
    assert_eq!(time_getter.get(), Err(Error::GetterNone));
    stream.update().unwrap();
    assert_eq!(time_getter.get(), Err(Error::GetterError));
}
#[test]
fn settable() {
    struct MySettable {
        last_request: Option<u8>,
    }
    impl MySettable {
        fn new() -> Self {
            Self { last_request: None }
        }
    }
    impl Settable<u8, ()> for MySettable {
        fn set(&mut self, x: u8) -> NothingOrError<()> {
            self.last_request = Some(x);
            Ok(())
        }
    }
    impl Updatable<()> for MySettable {
        fn update(&mut self) -> NothingOrError<()> {
            Ok(())
        }
    }
    let mut my_settable = MySettable::new();
    assert_eq!(my_settable.last_request, None);
    my_settable.set(3).unwrap();
    assert_eq!(my_settable.last_request, Some(3));
}
#[test]
fn getter_from_chronology() {
    enum TestState {
        Standard,
        ReturnNone,
    }
    struct MyChronology {
        test_state: TestState,
    }
    impl MyChronology {
        fn new() -> Self {
            Self {
                test_state: TestState::Standard,
            }
        }
        fn set_none_test(&mut self) {
            self.test_state = TestState::ReturnNone;
        }
    }
    impl Chronology<i64> for MyChronology {
        fn get(&self, time: Time) -> Option<Datum<i64>> {
            match self.test_state {
                TestState::Standard => Some(Datum::new(time, time.as_nanoseconds())),
                TestState::ReturnNone => None,
            }
        }
    }
    struct MyTimeGetter {
        time: Time,
    }
    impl MyTimeGetter {
        const fn new() -> Self {
            Self {
                time: Time::from_nanoseconds(5),
            }
        }
    }
    impl TimeGetter<()> for MyTimeGetter {
        fn get(&self) -> TimeOutput<()> {
            Ok(self.time)
        }
    }
    impl Updatable<()> for MyTimeGetter {
        fn update(&mut self) -> NothingOrError<()> {
            self.time += Time::from_nanoseconds(1);
            Ok(())
        }
    }
    static mut TIME_GETTER: MyTimeGetter = MyTimeGetter::new();
    let mut my_time_getter =
        unsafe { PointerDereferencer::new(core::ptr::addr_of_mut!(TIME_GETTER)) };
    {
        let my_chronology = MyChronology::new();
        let no_delta = GetterFromChronology::new_no_delta(my_chronology, my_time_getter);
        assert_eq!(
            no_delta.get().unwrap().unwrap(),
            Datum::new(Time::from_nanoseconds(5), 5)
        );
        my_time_getter.update().unwrap();
        assert_eq!(
            no_delta.get().unwrap().unwrap(),
            Datum::new(Time::from_nanoseconds(6), 6)
        );
    }
    {
        let my_chronology = MyChronology::new();
        let start_at_zero =
            GetterFromChronology::new_start_at_zero(my_chronology, my_time_getter).unwrap();
        assert_eq!(
            start_at_zero.get().unwrap().unwrap(),
            Datum::new(Time::from_nanoseconds(6), 0)
        );
        my_time_getter.update().unwrap();
        assert_eq!(
            start_at_zero.get().unwrap().unwrap(),
            Datum::new(Time::from_nanoseconds(7), 1)
        );
    }
    {
        let my_chronology = MyChronology::new();
        let custom_start = GetterFromChronology::new_custom_start(
            my_chronology,
            my_time_getter,
            Time::from_nanoseconds(10),
        )
        .unwrap();
        assert_eq!(
            custom_start.get().unwrap().unwrap(),
            Datum::new(Time::from_nanoseconds(7), 10)
        );
        my_time_getter.update().unwrap();
        assert_eq!(
            custom_start.get().unwrap().unwrap(),
            Datum::new(Time::from_nanoseconds(8), 11)
        );
    }
    {
        let my_chronology = MyChronology::new();
        let custom_delta = GetterFromChronology::new_custom_delta(
            my_chronology,
            my_time_getter,
            Time::from_nanoseconds(5),
        );
        assert_eq!(
            custom_delta.get().unwrap().unwrap(),
            Datum::new(Time::from_nanoseconds(8), 13)
        );
        my_time_getter.update().unwrap();
        assert_eq!(
            custom_delta.get().unwrap().unwrap(),
            Datum::new(Time::from_nanoseconds(9), 14)
        );
    }
    {
        let my_chronology = MyChronology::new();
        let mut getter = GetterFromChronology::new_no_delta(my_chronology, my_time_getter);
        assert_eq!(
            getter.get().unwrap().unwrap(),
            Datum::new(Time::from_nanoseconds(9), 9)
        );
        getter.set_delta(Time::from_nanoseconds(5));
        assert_eq!(
            getter.get().unwrap().unwrap(),
            Datum::new(Time::from_nanoseconds(9), 14)
        );
        getter.set_time(Time::from_nanoseconds(20)).unwrap();
        assert_eq!(
            getter.get().unwrap().unwrap(),
            Datum::new(Time::from_nanoseconds(9), 20)
        );
    }
    {
        let mut my_chronology = MyChronology::new();
        my_chronology.set_none_test();
        let getter = GetterFromChronology::new_no_delta(my_chronology, my_time_getter);
        assert_eq!(getter.get().unwrap(), None);
    }
}
#[test]
fn constant_getter() {
    struct MyTimeGetter;
    impl TimeGetter<()> for MyTimeGetter {
        fn get(&self) -> TimeOutput<()> {
            Ok(Time::ZERO)
        }
    }
    impl Updatable<()> for MyTimeGetter {
        fn update(&mut self) -> NothingOrError<()> {
            Ok(())
        }
    }
    let mut constant_getter = ConstantGetter::new(MyTimeGetter, 10);
    assert_eq!(constant_getter.get().unwrap().unwrap().value, 10);
    constant_getter.update().unwrap(); //This should do nothing.
    assert_eq!(constant_getter.get().unwrap().unwrap().value, 10);
    constant_getter.set(20).unwrap();
    assert_eq!(constant_getter.get().unwrap().unwrap().value, 20);
}
#[test]
fn none_getter() {
    let mut getter = NoneGetter::new();
    assert_eq!(<NoneGetter as Getter<(), ()>>::get(&getter), Ok(None));
    <NoneGetter as Updatable<()>>::update(&mut getter).unwrap();
    assert_eq!(<NoneGetter as Getter<(), ()>>::get(&getter), Ok(None));
}
