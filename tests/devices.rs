use std::rc::Rc;
use std::cell::RefCell;
use rrtk::*;
#[test]
fn devices() {
    struct DCMotor {
        pub power: f32,
        pub time: i64,
        settable_data: SettableData<f32, ()>,
    }
    impl DCMotor {
        pub fn new() -> Self {
            Self {
                power: 0.0,
                time: -1,
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
            self.time += 2;
            self.power = value;
            if self.time == 3 {
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
        time: i64,
        velocity: f32,
    }
    impl Encoder {
        pub fn new() -> Self {
            Self {
                time: -1,
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
            self.time += 2;
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
    assert!(axle.get().unwrap().unwrap().time > 3);
}
#[test]
fn follow_motion_profile() {
    struct ServoMotor {
        pub time_getter: InputTimeGetter<()>,
        pub state: State,
        settable_data: SettableData<Command, ()>
    }
    impl ServoMotor {
        pub fn new(time_getter: InputTimeGetter<()>) -> Self {
            Self {
                time_getter: time_getter,
                state: State::new(0.0, 0.0, 0.0),
                settable_data: SettableData::new(),
            }
        }
    }
    impl Settable<Command, ()> for ServoMotor {
        fn get_settable_data_ref(&self) -> &SettableData<Command, ()> {
            &self.settable_data
        }
        fn get_settable_data_mut(&mut self) -> &mut SettableData<Command, ()> {
            &mut self.settable_data
        }
        fn direct_set(&mut self, command: Command) -> NothingOrError<()> {
            match command.position_derivative {
                PositionDerivative::Position => {
                    self.state.set_constant_position(command.value);
                }
                PositionDerivative::Velocity => {
                    self.state.set_constant_velocity(command.value);
                }
                PositionDerivative::Acceleration => {
                    self.state.set_constant_acceleration(command.value);
                }
            }
            Ok(())
        }
    }
    impl Updatable<()> for ServoMotor {
        fn update(&mut self) -> NothingOrError<()> {
            let time = self.time_getter.borrow().get().unwrap();
            if time == 5 {
                assert_eq!(self.state.acceleration, 0.01);
            }
            if time == 25 {
                assert!(0.099 < self.state.velocity && self.state.velocity < 0.101)
            }
            if time == 35 {
                assert_eq!(self.state.acceleration, -0.01);
            }
            Ok(())
        }
    }
    struct MyTimeGetter {
        time: i64,
    }
    impl MyTimeGetter {
        pub fn new() -> Self {
            Self {
                time: 0,
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
            self.time += 1;
            Ok(())
        }
    }
    let time_getter = make_input_time_getter!(MyTimeGetter::new(), ());
    let motion_profile = MotionProfile::new(
        State::new(0.0, 0.0, 0.0),
        State::new(3.0, 0.0, 0.0),
        0.1,
        0.01,
    );
    let motion_profile = GetterFromHistory::new_for_motion_profile(motion_profile, Rc::clone(&time_getter)).unwrap();
    let motion_profile = make_input_getter!(motion_profile, Command, ());
    let servo = Device::PreciseWrite(Box::new(ServoMotor::new(Rc::clone(&time_getter))));
    let mut axle = Axle::new([servo]);
    axle.follow(motion_profile);
    for _ in 0..2000 {
        time_getter.borrow_mut().update().unwrap();
        axle.following_update().unwrap();
    }
    println!("{:?}", time_getter.borrow().get().unwrap());
}
