#[cfg(feature = "std")]
use std::rc::Rc;
#[cfg(feature = "std")]
use std::cell::RefCell;
#[cfg(not(feature = "std"))]
extern crate alloc;
#[cfg(not(feature = "std"))]
use alloc::rc::Rc;
#[cfg(not(feature = "std"))]
use core::cell::RefCell;
use rrtk::*;
#[test]
fn devices() {
    struct DCMotor {
        pub power: f32,
        pub time: f32,
        settable_data: SettableData<f32, ()>,
    }
    impl DCMotor {
        pub fn new() -> Self {
            Self {
                power: 0.0,
                time: -1.0,
                settable_data: SettableData::new(),
            }
        }
    }
    impl Settable<f32, ()> for DCMotor {
        fn get_settable_data_ref(&self) -> &SettableData<f32, ()> {
            &self.settable_data
        }
        fn get_settable_data_mut(&mut self) -> &mut SettableData<f32, ()> {
            &mut self.settable_data
        }
        fn direct_set(&mut self, value: f32) -> NothingOrError<()> {
            self.time += 2.0;
            self.power = value;
            if self.time == 3.0 {
                assert_eq!(self.power, 9.04);
            }
            Ok(())
        }
    }
    impl Updatable<()> for DCMotor {
        fn update(&mut self) -> NothingOrError<()> {
            Ok(())
        }
    }
    struct Encoder {
        time: f32,
        velocity: f32,
    }
    impl Encoder {
        pub fn new() -> Self {
            Self {
                time: -1.0,
                velocity: -1.0,
            }
        }
    }
    impl Getter<State, ()> for Encoder {
        fn get(&self) -> Output<State, ()> {
            //We don't care about position and acceleration here, so don't worry about them.
            Ok(Some(Datum::new(self.time, State::new(0.0, self.velocity, 0.0))))
        }
    }
    impl Updatable<()> for Encoder {
        fn update(&mut self) -> NothingOrError<()> {
            self.time += 2.0;
            self.velocity += 1.0;
            Ok(())
        }
    }
    let encoder = Device::Read(Box::new(Encoder::new()));
    let motor = Device::ImpreciseWrite(Box::new(DCMotor::new()), PositionDerivativeDependentPIDKValues::new(PIDKValues::new(1.0, 0.01, 0.1), PIDKValues::new(1.0, 0.01, 0.1), PIDKValues::new(1.0, 0.01, 0.1)));
    let mut axle = Axle::new([encoder, motor]);
    axle.set(Command::new(PositionDerivative::Velocity, 5.0)).unwrap();
    axle.update().unwrap();
    axle.update().unwrap();
    axle.update().unwrap();
    //Ensure that we actually ran the assert_eq! in DCMotor direct_set.
    assert!(axle.get().unwrap().unwrap().time > 3.0);
}
