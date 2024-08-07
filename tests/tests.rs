// SPDX-License-Identifier: LGPL-3.0-only
/*
Copyright 2024 UxuginPython on GitHub

     This file is part of Rust Robotics ToolKit.

    Rust Robotics ToolKit is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, version 3.

    Rust Robotics ToolKit is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Lesser General Public License for more details.

    You should have received a copy of the GNU Lesser General Public License along with Rust Robotics ToolKit. If not, see <https://www.gnu.org/licenses/>.
*/
#[cfg(feature = "std")]
use std::rc::Rc;
#[cfg(not(feature = "std"))]
extern crate alloc;
#[cfg(not(feature = "std"))]
use alloc::rc::Rc;
use rrtk::*;
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
fn state_get_value() {
    let state = State::new(1.0, 2.0, 3.0);
    assert_eq!(state.get_value(PositionDerivative::Position), 1.0);
    assert_eq!(state.get_value(PositionDerivative::Velocity), 2.0);
    assert_eq!(state.get_value(PositionDerivative::Acceleration), 3.0);
}
#[test]
fn state_ops() {
    assert_eq!(-State::new(1.0, 2.0, 3.0), State::new(-1.0, -2.0, -3.0));
    assert_eq!(
        State::new(1.0, 2.0, 3.0) + State::new(4.0, 5.0, 6.0),
        State::new(5.0, 7.0, 9.0)
    );
    assert_eq!(
        State::new(1.0, 2.0, 3.0) - State::new(4.0, 5.0, 6.0),
        State::new(-3.0, -3.0, -3.0)
    );
    assert_eq!(State::new(1.0, 2.0, 3.0) * 2.0, State::new(2.0, 4.0, 6.0));
    assert_eq!(State::new(1.0, 2.0, 3.0) / 2.0, State::new(0.5, 1.0, 1.5));
    let mut state = State::new(1.0, 2.0, 3.0);
    state += State::new(4.0, 5.0, 6.0);
    assert_eq!(state, State::new(5.0, 7.0, 9.0));
    let mut state = State::new(1.0, 2.0, 3.0);
    state -= State::new(4.0, 5.0, 6.0);
    assert_eq!(state, State::new(-3.0, -3.0, -3.0));
    let mut state = State::new(1.0, 2.0, 3.0);
    state *= 2.0;
    assert_eq!(state, State::new(2.0, 4.0, 6.0));
    let mut state = State::new(1.0, 2.0, 3.0);
    state /= 2.0;
    assert_eq!(state, State::new(0.5, 1.0, 1.5));
}
#[test]
fn latest_datum() {
    assert_eq!(latest(Datum::new(0, 0), Datum::new(1, 1)), Datum::new(1, 1));
    assert_eq!(latest(Datum::new(1, 0), Datum::new(0, 1)), Datum::new(1, 0));
    assert_eq!(latest(Datum::new(0, 0), Datum::new(0, 1)), Datum::new(0, 0));
}
#[test]
fn datum_not() {
    assert_eq!(!Datum::new(0, false), Datum::new(0, true));
}
#[test]
fn datum_neg() {
    assert_eq!(-Datum::new(0, 1), Datum::new(0, -1));
}
#[test]
fn datum_add() {
    assert_eq!(Datum::new(0, 1) + Datum::new(1, 1), Datum::new(1, 2));
    assert_eq!(Datum::new(1, 1) + Datum::new(0, 1), Datum::new(1, 2));

    let mut x = Datum::new(0, 1);
    x += Datum::new(1, 1);
    assert_eq!(x, Datum::new(1, 2));

    let mut x = Datum::new(1, 1);
    x += Datum::new(0, 1);
    assert_eq!(x, Datum::new(1, 2));

    assert_eq!(Datum::new(0, 1) + 1, Datum::new(0, 2));

    let mut x = Datum::new(0, 1);
    x += 1;
    assert_eq!(x, Datum::new(0, 2));
}
#[test]
fn datum_sub() {
    assert_eq!(Datum::new(0, 1) - Datum::new(1, 1), Datum::new(1, 0));
    assert_eq!(Datum::new(1, 1) - Datum::new(0, 1), Datum::new(1, 0));

    let mut x = Datum::new(0, 1);
    x -= Datum::new(1, 1);
    assert_eq!(x, Datum::new(1, 0));

    let mut x = Datum::new(1, 1);
    x -= Datum::new(0, 1);
    assert_eq!(x, Datum::new(1, 0));

    assert_eq!(Datum::new(0, 1) - 1, Datum::new(0, 0));

    let mut x = Datum::new(0, 1);
    x -= 1;
    assert_eq!(x, Datum::new(0, 0));
}
#[test]
fn datum_mul() {
    assert_eq!(Datum::new(0, 2) * Datum::new(1, 3), Datum::new(1, 6));
    assert_eq!(Datum::new(1, 2) * Datum::new(0, 3), Datum::new(1, 6));

    let mut x = Datum::new(0, 2);
    x *= Datum::new(1, 3);
    assert_eq!(x, Datum::new(1, 6));

    let mut x = Datum::new(1, 2);
    x *= Datum::new(0, 3);
    assert_eq!(x, Datum::new(1, 6));

    assert_eq!(Datum::new(0, 2) * 3, Datum::new(0, 6));

    let mut x = Datum::new(0, 2);
    x *= 3;
    assert_eq!(x, Datum::new(0, 6));
}
#[test]
fn datum_div() {
    assert_eq!(Datum::new(0, 6) / Datum::new(1, 2), Datum::new(1, 3));
    assert_eq!(Datum::new(1, 6) / Datum::new(0, 2), Datum::new(1, 3));

    let mut x = Datum::new(0, 6);
    x /= Datum::new(1, 2);
    assert_eq!(x, Datum::new(1, 3));

    let mut x = Datum::new(1, 6);
    x /= Datum::new(0, 2);
    assert_eq!(x, Datum::new(1, 3));

    assert_eq!(Datum::new(0, 6) / 2, Datum::new(0, 3));

    let mut x = Datum::new(0, 6);
    x /= 2;
    assert_eq!(x, Datum::new(0, 3));
}
#[test]
fn datum_state_mul() {
    assert_eq!(
        Datum::new(0, State::new(1.0, 2.0, 3.0)) * Datum::new(1, 3.0),
        Datum::new(1, State::new(3.0, 6.0, 9.0))
    );
    assert_eq!(
        Datum::new(1, State::new(1.0, 2.0, 3.0)) * Datum::new(0, 3.0),
        Datum::new(1, State::new(3.0, 6.0, 9.0))
    );

    let mut x = Datum::new(0, State::new(1.0, 2.0, 3.0));
    x *= Datum::new(1, 3.0);
    assert_eq!(x, Datum::new(1, State::new(3.0, 6.0, 9.0)));

    let mut x = Datum::new(1, State::new(1.0, 2.0, 3.0));
    x *= Datum::new(0, 3.0);
    assert_eq!(x, Datum::new(1, State::new(3.0, 6.0, 9.0)));

    assert_eq!(
        Datum::new(0, State::new(1.0, 2.0, 3.0)) * 3.0,
        Datum::new(0, State::new(3.0, 6.0, 9.0))
    );

    let mut x = Datum::new(0, State::new(1.0, 2.0, 3.0));
    x *= 3.0;
    assert_eq!(x, Datum::new(0, State::new(3.0, 6.0, 9.0)));
}
#[test]
fn datum_state_div() {
    assert_eq!(
        Datum::new(0, State::new(2.0, 4.0, 6.0)) / Datum::new(1, 2.0),
        Datum::new(1, State::new(1.0, 2.0, 3.0))
    );
    assert_eq!(
        Datum::new(1, State::new(2.0, 4.0, 6.0)) / Datum::new(0, 2.0),
        Datum::new(1, State::new(1.0, 2.0, 3.0))
    );

    let mut x = Datum::new(0, State::new(2.0, 4.0, 6.0));
    x /= Datum::new(1, 2.0);
    assert_eq!(x, Datum::new(1, State::new(1.0, 2.0, 3.0)));

    let mut x = Datum::new(1, State::new(2.0, 4.0, 6.0));
    x /= Datum::new(0, 2.0);
    assert_eq!(x, Datum::new(1, State::new(1.0, 2.0, 3.0)));

    assert_eq!(
        Datum::new(0, State::new(2.0, 4.0, 6.0)) / 2.0,
        Datum::new(0, State::new(1.0, 2.0, 3.0))
    );

    let mut x = Datum::new(0, State::new(2.0, 4.0, 6.0));
    x /= 2.0;
    assert_eq!(x, Datum::new(0, State::new(1.0, 2.0, 3.0)));
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
    assert_eq!(motion_profile.get_acceleration(-1), None);
    assert_eq!(motion_profile.get_acceleration(5), Some(0.01));
    assert_eq!(motion_profile.get_acceleration(25), Some(0.0));
    assert_eq!(motion_profile.get_acceleration(35), Some(-0.01));
    assert_eq!(motion_profile.get_acceleration(500), Some(0.0));
}
#[test]
fn motion_profile_get_velocity() {
    let motion_profile = MotionProfile::new(
        State::new(0.0, 0.0, 0.0),
        State::new(3.0, 0.0, 0.0),
        0.1,
        0.01,
    );
    assert_eq!(motion_profile.get_velocity(-1), None);
    let gv5 = motion_profile.get_velocity(5).unwrap();
    assert!(0.049 < gv5 && gv5 < 0.051);
    let gv25 = motion_profile.get_velocity(25).unwrap();
    assert!(0.099 < gv25 && gv25 < 0.101);
    let gv35 = motion_profile.get_velocity(35).unwrap();
    assert!(0.049 < gv35 && gv35 < 0.051);
    assert_eq!(motion_profile.get_velocity(500), Some(0.0));
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
    assert_eq!(motion_profile.get_position(-1), None);
    let gp5 = motion_profile.get_position(5).unwrap();
    assert!(0.124 < gp5 && gp5 < 0.126);
    assert_eq!(motion_profile.get_position(25), Some(2.0));
    assert_eq!(motion_profile.get_position(35), Some(2.875));
    assert_eq!(motion_profile.get_position(500), Some(3.0));
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
fn motion_profile_history() {
    let motion_profile = MotionProfile::new(
        State::new(0.0, 0.0, 0.0),
        State::new(3.0, 0.0, 0.0),
        0.1,
        0.01,
    );
    let mut motion_profile = Box::new(motion_profile) as Box<dyn History<Command, ()>>;
    let _ = motion_profile.update().unwrap(); //This should do nothing.
    assert_eq!(motion_profile.get(-20), None);
    assert_eq!(
        motion_profile.get(5).unwrap().value,
        Command::new(PositionDerivative::Acceleration, 0.01)
    );
    let g25 = motion_profile.get(25).unwrap().value;
    assert_eq!(g25.position_derivative, PositionDerivative::Velocity);
    assert!(0.099 < g25.value && g25.value < 0.101);
    assert_eq!(
        motion_profile.get(35).unwrap().value,
        Command::new(PositionDerivative::Acceleration, -0.01)
    );
    assert_eq!(
        motion_profile.get(99999).unwrap().value,
        Command::new(PositionDerivative::Position, 3.0)
    );
}
#[test]
fn motion_profile_piece() {
    let motion_profile = MotionProfile::new(
        State::new(0.0, 0.0, 0.0),
        State::new(3.0, 0.0, 0.0),
        0.1,
        0.01,
    );
    assert_eq!(
        motion_profile.get_piece(-20),
        MotionProfilePiece::BeforeStart
    );
    assert_eq!(
        motion_profile.get_piece(5),
        MotionProfilePiece::InitialAcceleration
    );
    assert_eq!(
        motion_profile.get_piece(25),
        MotionProfilePiece::ConstantVelocity
    );
    assert_eq!(
        motion_profile.get_piece(35),
        MotionProfilePiece::EndAcceleration
    );
    assert_eq!(motion_profile.get_piece(500), MotionProfilePiece::Complete);
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
#[test]
fn command_from_state() {
    let command = Command::from(State::new(1.0, 2.0, 3.0));
    assert_eq!(command, Command::new(PositionDerivative::Acceleration, 3.0));
    let command = Command::from(State::new(1.0, 2.0, 0.0));
    assert_eq!(command, Command::new(PositionDerivative::Velocity, 2.0));
    let command = Command::from(State::new(1.0, 0.0, 0.0));
    assert_eq!(command, Command::new(PositionDerivative::Position, 1.0));
}
#[test]
fn time_getter_from_stream() {
    struct Stream {
        time: i64,
    }
    impl Stream {
        fn new() -> Self {
            Self { time: 0 }
        }
    }
    impl Getter<(), ()> for Stream {
        fn get(&self) -> Output<(), ()> {
            match self.time {
                0 => Ok(Some(Datum::new(self.time, ()))),
                1 => Ok(None),
                2 => Err(Error::Other(())),
                _ => panic!("should always be 0, 1, or 2"),
            }
        }
    }
    impl Updatable<()> for Stream {
        fn update(&mut self) -> NothingOrError<()> {
            self.time += 1;
            Ok(())
        }
    }
    let stream = make_input_getter(Stream::new());
    let mut time_getter = TimeGetterFromGetter::new(Rc::clone(&stream));
    time_getter.update().unwrap(); //This should do nothing.
    assert_eq!(time_getter.get(), Ok(0));
    stream.borrow_mut().update().unwrap();
    assert_eq!(time_getter.get(), Err(Error::FromNone));
    stream.borrow_mut().update().unwrap();
    assert_eq!(time_getter.get(), Err(Error::Other(())));
}
#[test]
fn settable() {
    struct MyGetter {
        none: bool,
        value: u8,
    }
    impl MyGetter {
        fn new() -> Self {
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
            Ok(Some(Datum::new(0, self.value)))
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
    let my_getter = make_input_getter(MyGetter::new());
    my_settable.follow(Rc::clone(&my_getter));
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
        fn get(&self, time: i64) -> Option<Datum<i64>> {
            match self.update_test_state {
                UpdateTestState::Unneeded | UpdateTestState::Waiting => Some(Datum::new(time, time)),
                UpdateTestState::Updated => Some(Datum::new(time, 30)),
                UpdateTestState::ReturnNone => None,
            }
        }
    }
    impl Updatable<()> for MyHistory {
        fn update(&mut self) -> NothingOrError<()> {
            match self.update_test_state {
                UpdateTestState::Waiting => self.update_test_state = UpdateTestState::Updated,
                _ => ()
            }
            Ok(())
        }
    }
    struct MyTimeGetter {
        time: i64,
    }
    impl MyTimeGetter {
        fn new() -> Self {
            Self { time: 5 }
        }
    }
    impl TimeGetter<()> for MyTimeGetter {
        fn get(&self) -> TimeOutput<()> {
            Ok(self.time)
        }
    }
    impl Updatable<()> for MyTimeGetter {
        fn update(&mut self) -> NothingOrError<()> {
            self.time += 1;
            Ok(())
        }
    }

    let mut my_history = MyHistory::new();
    let my_time_getter = make_input_time_getter(MyTimeGetter::new());

    {
        let no_delta = GetterFromHistory::new_no_delta(&mut my_history, Rc::clone(&my_time_getter));
        assert_eq!(no_delta.get().unwrap().unwrap(), Datum::new(5, 5));
        my_time_getter.borrow_mut().update().unwrap();
        assert_eq!(no_delta.get().unwrap().unwrap(), Datum::new(6, 6));
    }

    {
        let start_at_zero =
            GetterFromHistory::new_start_at_zero(&mut my_history, Rc::clone(&my_time_getter))
                .unwrap();
        assert_eq!(start_at_zero.get().unwrap().unwrap(), Datum::new(6, 0));
        my_time_getter.borrow_mut().update().unwrap();
        assert_eq!(start_at_zero.get().unwrap().unwrap(), Datum::new(7, 1));
    }

    {
        let custom_start =
            GetterFromHistory::new_custom_start(&mut my_history, Rc::clone(&my_time_getter), 10)
                .unwrap();
        assert_eq!(custom_start.get().unwrap().unwrap(), Datum::new(7, 10));
        my_time_getter.borrow_mut().update().unwrap();
        assert_eq!(custom_start.get().unwrap().unwrap(), Datum::new(8, 11));
    }

    {
        let custom_delta =
            GetterFromHistory::new_custom_delta(&mut my_history, Rc::clone(&my_time_getter), 5);
        assert_eq!(custom_delta.get().unwrap().unwrap(), Datum::new(8, 13));
        my_time_getter.borrow_mut().update().unwrap();
        assert_eq!(custom_delta.get().unwrap().unwrap(), Datum::new(9, 14));
    }

    {
        let mut getter =
            GetterFromHistory::new_no_delta(&mut my_history, Rc::clone(&my_time_getter));
        assert_eq!(getter.get().unwrap().unwrap(), Datum::new(9, 9));
        getter.set_delta(5);
        assert_eq!(getter.get().unwrap().unwrap(), Datum::new(9, 14));
        getter.set_time(20).unwrap();
        assert_eq!(getter.get().unwrap().unwrap(), Datum::new(9, 20));
    }

    {
        my_history.set_update_test();
        let mut getter = GetterFromHistory::new_no_delta(&mut my_history, Rc::clone(&my_time_getter));
        assert_eq!(getter.get().unwrap().unwrap(), Datum::new(9, 9));
        getter.update().unwrap();
        assert_eq!(getter.get().unwrap().unwrap(), Datum::new(10, 30));
    }

    {
        my_history.set_none_test();
        let getter = GetterFromHistory::new_no_delta(&mut my_history, Rc::clone(&my_time_getter));
        assert_eq!(getter.get().unwrap(), None);
    }
}
#[test]
fn constant_getter() {
    struct MyTimeGetter;
    impl TimeGetter<()> for MyTimeGetter {
        fn get(&self) -> TimeOutput<()> {
            Ok(0)
        }
    }
    impl Updatable<()> for MyTimeGetter {
        fn update(&mut self) -> NothingOrError<()> {
            Ok(())
        }
    }
    let mut constant_getter = ConstantGetter::new(make_input_time_getter(MyTimeGetter), 10);
    assert_eq!(constant_getter.get().unwrap().unwrap().value, 10);
    constant_getter.update().unwrap(); //This should do nothing.
    assert_eq!(constant_getter.get().unwrap().unwrap().value, 10);
    constant_getter.set(20).unwrap();
    assert_eq!(constant_getter.get().unwrap().unwrap().value, 20);
}
