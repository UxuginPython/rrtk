//In this module, the following numbers are used for errors (x: y = y returns x error number):
//1: getter.update()
//2: getter.get()
//3: settable.set()
//4: settable.update()
use rrtk::*;
const CORRECT_VALUE: Output<u8, u8> = Ok(Some(Datum::new(Time::from_nanoseconds(300_000_000), 39)));
#[test]
fn getter_update_fail_settable_update_ok() {
    struct MyGetter;
    impl Updatable<u8> for MyGetter {
        fn update(&mut self) -> NothingOrError<u8> {
            Err(1)
        }
    }
    impl Getter<u8, u8> for MyGetter {
        fn get(&self) -> Output<u8, u8> {
            panic!("Since MyGetter::update returns Err, this should never be called.");
        }
    }
    struct MySettable;
    impl Settable<u8, u8> for MySettable {
        fn set(&mut self, _: u8) -> NothingOrError<u8> {
            panic!("Since MyGetter::get doesn't run, this shouldn't either.");
        }
    }
    static mut MY_SETTABLE_UPDATE_CALL_COUNT: u8 = 0;
    impl Updatable<u8> for MySettable {
        fn update(&mut self) -> NothingOrError<u8> {
            unsafe {
                MY_SETTABLE_UPDATE_CALL_COUNT += 1;
            }
            Ok(())
        }
    }
    let mut feeder = Feeder::new(MyGetter, MySettable);
    let test = feeder.update();
    assert_eq!(test, Err(error::PossibleDoubleError::A(1)));
    assert_eq!(unsafe { MY_SETTABLE_UPDATE_CALL_COUNT }, 1);
}
#[test]
fn getter_update_fail_settable_update_fail() {
    struct MyGetter;
    impl Updatable<u8> for MyGetter {
        fn update(&mut self) -> NothingOrError<u8> {
            Err(1)
        }
    }
    impl Getter<u8, u8> for MyGetter {
        fn get(&self) -> Output<u8, u8> {
            panic!("Since MyGetter::update returns Err, this should never be called.");
        }
    }
    struct MySettable;
    impl Settable<u8, u8> for MySettable {
        fn set(&mut self, _: u8) -> NothingOrError<u8> {
            panic!("Since MyGetter::get doesn't run, this shouldn't either.");
        }
    }
    impl Updatable<u8> for MySettable {
        fn update(&mut self) -> NothingOrError<u8> {
            Err(4)
        }
    }
    let mut feeder = Feeder::new(MyGetter, MySettable);
    let test = feeder.update();
    assert_eq!(test, Err(error::PossibleDoubleError::AB(1, 4)));
}
#[test]
fn getter_get_fail_settable_update_ok() {
    struct MyGetter;
    static mut MY_GETTER_UPDATE_CALL_COUNT: u8 = 0;
    impl Updatable<u8> for MyGetter {
        fn update(&mut self) -> NothingOrError<u8> {
            unsafe {
                MY_GETTER_UPDATE_CALL_COUNT += 1;
            }
            Ok(())
        }
    }
    impl Getter<u8, u8> for MyGetter {
        fn get(&self) -> Output<u8, u8> {
            Err(2)
        }
    }
    struct MySettable;
    impl Settable<u8, u8> for MySettable {
        fn set(&mut self, _: u8) -> NothingOrError<u8> {
            panic!("Since MyGetter::get fails, this shouldn't run.");
        }
    }
    static mut MY_SETTABLE_UPDATE_CALL_COUNT: u8 = 0;
    impl Updatable<u8> for MySettable {
        fn update(&mut self) -> NothingOrError<u8> {
            unsafe {
                MY_SETTABLE_UPDATE_CALL_COUNT += 1;
            }
            Ok(())
        }
    }
    let mut feeder = Feeder::new(MyGetter, MySettable);
    let test = feeder.update();
    assert_eq!(test, Err(error::PossibleDoubleError::A(2)));
    assert_eq!(unsafe { MY_GETTER_UPDATE_CALL_COUNT }, 1);
    assert_eq!(unsafe { MY_SETTABLE_UPDATE_CALL_COUNT }, 1);
}
#[test]
fn getter_get_fail_settable_update_fail() {
    struct MyGetter;
    static mut MY_GETTER_UPDATE_CALL_COUNT: u8 = 0;
    impl Updatable<u8> for MyGetter {
        fn update(&mut self) -> NothingOrError<u8> {
            unsafe {
                MY_GETTER_UPDATE_CALL_COUNT += 1;
            }
            Ok(())
        }
    }
    impl Getter<u8, u8> for MyGetter {
        fn get(&self) -> Output<u8, u8> {
            Err(2)
        }
    }
    struct MySettable;
    impl Settable<u8, u8> for MySettable {
        fn set(&mut self, _: u8) -> NothingOrError<u8> {
            panic!("Since MyGetter::get fails, this shouldn't run.");
        }
    }
    impl Updatable<u8> for MySettable {
        fn update(&mut self) -> NothingOrError<u8> {
            Err(4)
        }
    }
    let mut feeder = Feeder::new(MyGetter, MySettable);
    let test = feeder.update();
    assert_eq!(test, Err(error::PossibleDoubleError::AB(2, 4)));
    assert_eq!(unsafe { MY_GETTER_UPDATE_CALL_COUNT }, 1);
}
#[test]
fn settable_set_fail() {
    struct MyGetter;
    static mut MY_GETTER_UPDATE_CALL_COUNT: u8 = 0;
    impl Updatable<u8> for MyGetter {
        fn update(&mut self) -> NothingOrError<u8> {
            unsafe {
                MY_GETTER_UPDATE_CALL_COUNT += 1;
            }
            Ok(())
        }
    }
    static mut MY_GETTER_GET_CALL_COUNT: u8 = 0;
    impl Getter<u8, u8> for MyGetter {
        fn get(&self) -> Output<u8, u8> {
            unsafe {
                MY_GETTER_GET_CALL_COUNT += 1;
            }
            CORRECT_VALUE
        }
    }
    struct MySettable;
    impl Settable<u8, u8> for MySettable {
        fn set(&mut self, value: u8) -> NothingOrError<u8> {
            assert_eq!(value, 39);
            Err(3)
        }
    }
    impl Updatable<u8> for MySettable {
        fn update(&mut self) -> NothingOrError<u8> {
            panic!("Since Self::set fails, this should never run.");
        }
    }
    let mut feeder = Feeder::new(MyGetter, MySettable);
    let test = feeder.update();
    assert_eq!(test, Err(error::PossibleDoubleError::B(3)));
    assert_eq!(unsafe { MY_GETTER_UPDATE_CALL_COUNT }, 1);
    assert_eq!(unsafe { MY_GETTER_GET_CALL_COUNT }, 1);
}
#[test]
fn getter_ok_settable_update_fail() {
    struct MyGetter;
    static mut MY_GETTER_UPDATE_CALL_COUNT: u8 = 0;
    impl Updatable<u8> for MyGetter {
        fn update(&mut self) -> NothingOrError<u8> {
            unsafe {
                MY_GETTER_UPDATE_CALL_COUNT += 1;
            }
            Ok(())
        }
    }
    static mut MY_GETTER_GET_CALL_COUNT: u8 = 0;
    impl Getter<u8, u8> for MyGetter {
        fn get(&self) -> Output<u8, u8> {
            unsafe {
                MY_GETTER_GET_CALL_COUNT += 1;
            }
            CORRECT_VALUE
        }
    }
    struct MySettable;
    static mut MY_SETTABLE_SET_CALL_COUNT: u8 = 0;
    impl Settable<u8, u8> for MySettable {
        fn set(&mut self, value: u8) -> NothingOrError<u8> {
            assert_eq!(value, 39);
            unsafe {
                MY_SETTABLE_SET_CALL_COUNT += 1;
            }
            Ok(())
        }
    }
    impl Updatable<u8> for MySettable {
        fn update(&mut self) -> NothingOrError<u8> {
            Err(4)
        }
    }
    let mut feeder = Feeder::new(MyGetter, MySettable);
    let test = feeder.update();
    assert_eq!(test, Err(error::PossibleDoubleError::B(4)));
    assert_eq!(unsafe { MY_GETTER_UPDATE_CALL_COUNT }, 1);
    assert_eq!(unsafe { MY_GETTER_GET_CALL_COUNT }, 1);
    assert_eq!(unsafe { MY_SETTABLE_SET_CALL_COUNT }, 1);
}
#[test]
fn everything_ok_some() {
    struct MyGetter;
    static mut MY_GETTER_UPDATE_CALL_COUNT: u8 = 0;
    impl Updatable<u8> for MyGetter {
        fn update(&mut self) -> NothingOrError<u8> {
            unsafe {
                MY_GETTER_UPDATE_CALL_COUNT += 1;
            }
            Ok(())
        }
    }
    static mut MY_GETTER_GET_CALL_COUNT: u8 = 0;
    impl Getter<u8, u8> for MyGetter {
        fn get(&self) -> Output<u8, u8> {
            unsafe {
                MY_GETTER_GET_CALL_COUNT += 1;
            }
            CORRECT_VALUE
        }
    }
    struct MySettable;
    static mut MY_SETTABLE_SET_CALL_COUNT: u8 = 0;
    impl Settable<u8, u8> for MySettable {
        fn set(&mut self, value: u8) -> NothingOrError<u8> {
            assert_eq!(value, 39);
            unsafe {
                MY_SETTABLE_SET_CALL_COUNT += 1;
            }
            Ok(())
        }
    }
    static mut MY_SETTABLE_UPDATE_CALL_COUNT: u8 = 0;
    impl Updatable<u8> for MySettable {
        fn update(&mut self) -> NothingOrError<u8> {
            unsafe {
                MY_SETTABLE_UPDATE_CALL_COUNT += 1;
            }
            Ok(())
        }
    }
    let mut feeder = Feeder::new(MyGetter, MySettable);
    let test = feeder.update();
    assert_eq!(test, Ok(()));
    assert_eq!(unsafe { MY_GETTER_UPDATE_CALL_COUNT }, 1);
    assert_eq!(unsafe { MY_GETTER_GET_CALL_COUNT }, 1);
    assert_eq!(unsafe { MY_SETTABLE_SET_CALL_COUNT }, 1);
    assert_eq!(unsafe { MY_SETTABLE_UPDATE_CALL_COUNT }, 1);
}
