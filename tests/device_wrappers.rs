// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2024-2026 UxuginPython
#![cfg(feature = "devices")]
use rrtk::devices::*;
use rrtk::*;
#[test]
fn get_and_write_to_node() {
    #[derive(Clone, Copy, Debug)]
    struct MyError;
    struct MyGetter {
        index: u8,
    }
    impl Updatable<MyError> for MyGetter {
        fn update(&mut self) -> NothingOrError<MyError> {
            self.index += 1;
            assert!(self.index <= 3);
            Ok(())
        }
    }
    const STATE_1: AngularState = AngularState::new(
        Dimensionless::new(1.0),
        InverseSecond::new(2.0),
        InverseSecondSquared::new(3.0),
    );
    const STATE_2: AngularState = AngularState::new(
        Dimensionless::new(4.0),
        InverseSecond::new(5.0),
        InverseSecondSquared::new(6.0),
    );
    impl Getter<AngularState, MyError> for MyGetter {
        fn get(&self) -> Output<AngularState, MyError> {
            match self.index {
                0 => Ok(Some(Datum::new(Time::ZERO, STATE_1))),
                1 => Ok(None),
                2 => Ok(Some(Datum::new(Time::ZERO, STATE_2))),
                3 => Err(MyError),
                _ => panic!("called update too many times"),
            }
        }
    }
    let mut system = System::<1>::new();
    let node = system
        .new_node()
        .expect("the system has a capacity of 1, and this is the first node we're requesting");
    let mut getter = MyGetter { index: 0 };
    wrappers::get_and_write_to_node(&getter, &mut system, node).unwrap();
    assert_eq!(system.get_state_local(node), Some(STATE_1));
    getter.update().unwrap();
    wrappers::get_and_write_to_node(&getter, &mut system, node).unwrap();
    assert!(system.get_state_local(node).is_none());
    getter.update().unwrap();
    wrappers::get_and_write_to_node(&getter, &mut system, node).unwrap();
    assert_eq!(system.get_state_local(node), Some(STATE_2));
    getter.update().unwrap();
    assert!(wrappers::get_and_write_to_node(&getter, &mut system, node).is_err());
    assert!(system.get_state_local(node).is_none());
}
