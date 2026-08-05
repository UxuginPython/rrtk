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
    const STATE_1: AngularState = AngularState::from_raw(1.0, 2.0, 3.0);
    const STATE_2: AngularState = AngularState::from_raw(4.0, 5.0, 6.0);
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
#[test]
fn set_to_node_state() {
    #[derive(Clone, Copy, Debug)]
    struct MyError;
    struct MySettable {
        index: u8,
    }
    impl Updatable<MyError> for MySettable {
        fn update(&mut self) -> NothingOrError<MyError> {
            self.index += 1;
            Ok(())
        }
    }
    const STATE: AngularState = AngularState::from_raw(2.0, 3.0, 5.0);
    static mut SET_CALLS: u8 = 0;
    impl Settable<AngularState, MyError> for MySettable {
        fn set(&mut self, value: AngularState) -> NothingOrError<MyError> {
            unsafe {
                SET_CALLS += 1;
            }
            match self.index {
                0 => {
                    panic!("set shouldn't be called when the node is node");
                }
                1 => {
                    assert_eq!(value, STATE);
                }
                2 => {
                    return Err(MyError);
                }
                _ => unimplemented!(),
            }
            Ok(())
        }
    }
    let mut system = System::<2>::new();
    let node_a = system.new_node().unwrap();
    let node_b = system.new_node().unwrap();
    system.connect(node_a, node_b);
    let mut settable = MySettable { index: 0 };

    //Everything is None. set shouldn't be called.
    wrappers::set_to_node_state(&mut settable, &mut system, node_a).unwrap();
    assert_eq!(unsafe { SET_CALLS }, 0);

    //get_state_connected should only read node_b, which is None. set shouldn't be called.
    system.set_state_local(node_a, Some(AngularState::from_raw(1000.0, 2000.0, 3000.0)));
    wrappers::set_to_node_state(&mut settable, &mut system, node_a).unwrap();
    assert_eq!(unsafe { SET_CALLS }, 0);

    //Now node_b is Some. set should be called but should not error yet.
    settable.update().unwrap();
    system.set_state_local(node_b, Some(STATE));
    wrappers::set_to_node_state(&mut settable, &mut system, node_a).unwrap();
    assert_eq!(unsafe { SET_CALLS }, 1);

    //Now set should be erroring.
    settable.update().unwrap();
    assert!(wrappers::set_to_node_state(&mut settable, &mut system, node_a).is_err());
    assert_eq!(unsafe { SET_CALLS }, 2);

    //Now set shouldn't be called and so shouldn't error.
    system.set_state_local(node_b, None);
    wrappers::set_to_node_state(&mut settable, &mut system, node_a).unwrap();
    assert_eq!(unsafe { SET_CALLS }, 2);
}
mod getter_wrapper {
    use super::*;
    #[test]
    fn update_error() {
        #[derive(Clone, Copy, Debug)]
        struct MyError;
        struct MyGetter;
        impl Updatable<MyError> for MyGetter {
            fn update(&mut self) -> NothingOrError<MyError> {
                Err(MyError)
            }
        }
        impl Getter<AngularState, MyError> for MyGetter {
            fn get(&self) -> Output<AngularState, MyError> {
                panic!("get must not be called since update errors");
            }
        }
        let mut system = System::<1>::new();
        let node = system.new_node().unwrap();
        system.set_state_local(node, Some(AngularState::from_raw(9.0, 8.0, 7.0)));
        let mut wrapper = wrappers::GetterWrapper::new(node, MyGetter);
        assert!(wrapper.device_update(&mut system).is_err());
        assert!(system.get_state_local(node).is_none());
    }
    #[test]
    fn get_error() {
        #[derive(Clone, Copy, Debug)]
        struct MyError;
        struct MyGetter;
        static mut UPDATE_CALLS: u8 = 0;
        impl Updatable<MyError> for MyGetter {
            fn update(&mut self) -> NothingOrError<MyError> {
                unsafe {
                    UPDATE_CALLS += 1;
                }
                Ok(())
            }
        }
        impl Getter<AngularState, MyError> for MyGetter {
            fn get(&self) -> Output<AngularState, MyError> {
                Err(MyError)
            }
        }
        let mut system = System::<1>::new();
        let node = system.new_node().unwrap();
        system.set_state_local(node, Some(AngularState::from_raw(9.0, 8.0, 7.0)));
        let mut wrapper = wrappers::GetterWrapper::new(node, MyGetter);
        assert!(wrapper.device_update(&mut system).is_err());
        assert!(system.get_state_local(node).is_none());
        assert_eq!(unsafe { UPDATE_CALLS }, 1);
    }
    #[test]
    fn ok_none() {
        #[derive(Clone, Copy, Debug)]
        struct MyError;
        struct MyGetter;
        static mut UPDATE_CALLS: u8 = 0;
        impl Updatable<MyError> for MyGetter {
            fn update(&mut self) -> NothingOrError<MyError> {
                unsafe {
                    UPDATE_CALLS += 1;
                }
                Ok(())
            }
        }
        impl Getter<AngularState, MyError> for MyGetter {
            fn get(&self) -> Output<AngularState, MyError> {
                Ok(None)
            }
        }
        let mut system = System::<1>::new();
        let node = system.new_node().unwrap();
        system.set_state_local(node, Some(AngularState::from_raw(9.0, 8.0, 7.0)));
        let mut wrapper = wrappers::GetterWrapper::new(node, MyGetter);
        assert!(wrapper.device_update(&mut system).is_ok());
        assert!(system.get_state_local(node).is_none());
        assert_eq!(unsafe { UPDATE_CALLS }, 1);
    }
    #[test]
    fn ok_some_1() {
        #[derive(Clone, Copy, Debug)]
        struct MyError;
        struct MyGetter;
        static mut UPDATE_CALLS: u8 = 0;
        impl Updatable<MyError> for MyGetter {
            fn update(&mut self) -> NothingOrError<MyError> {
                unsafe {
                    UPDATE_CALLS += 1;
                }
                Ok(())
            }
        }
        const GET_STATE: AngularState = AngularState::from_raw(1.0, 2.0, 4.0);
        impl Getter<AngularState, MyError> for MyGetter {
            fn get(&self) -> Output<AngularState, MyError> {
                Ok(Some(Datum::new(Time::ZERO, GET_STATE)))
            }
        }
        let mut system = System::<1>::new();
        let node = system.new_node().unwrap();
        system.set_state_local(node, Some(AngularState::from_raw(9.0, 8.0, 7.0)));
        let mut wrapper = wrappers::GetterWrapper::new(node, MyGetter);
        assert!(wrapper.device_update(&mut system).is_ok());
        assert_eq!(system.get_state_local(node), Some(GET_STATE));
        assert_eq!(unsafe { UPDATE_CALLS }, 1);
    }
    #[test]
    fn ok_some_2() {
        #[derive(Clone, Copy, Debug)]
        struct MyError;
        struct MyGetter;
        static mut UPDATE_CALLS: u8 = 0;
        impl Updatable<MyError> for MyGetter {
            fn update(&mut self) -> NothingOrError<MyError> {
                unsafe {
                    UPDATE_CALLS += 1;
                }
                Ok(())
            }
        }
        const GET_STATE: AngularState = AngularState::from_raw(1.0, 2.0, 4.0);
        impl Getter<AngularState, MyError> for MyGetter {
            fn get(&self) -> Output<AngularState, MyError> {
                Ok(Some(Datum::new(Time::ZERO, GET_STATE)))
            }
        }
        let mut system = System::<1>::new();
        let node = system.new_node().unwrap();
        //This is the only difference between ok_some_1 and ok_some_2. ok_some_1 has the node's
        //local state filled before the wrapper is applied, and ok_some_2 has it as None.
        assert!(system.get_state_local(node).is_none());
        let mut wrapper = wrappers::GetterWrapper::new(node, MyGetter);
        assert!(wrapper.device_update(&mut system).is_ok());
        assert_eq!(system.get_state_local(node), Some(GET_STATE));
        assert_eq!(unsafe { UPDATE_CALLS }, 1);
    }
}
