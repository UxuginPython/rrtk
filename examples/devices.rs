use rrtk::*;
struct Encoder {
    time: i64,
    velocity: f32,
}
impl Encoder {
    pub fn new() -> Self {
        Self {
            time: 0,
            velocity: 0.0,
        }
    }
}
impl Getter<State, ()> for Encoder {
    fn get(&self) -> Output<State, ()> {
        //We don't care about position and acceleration here, so don't worry about them.
        println!(
            "Encoder returning time: {:?}; velocity: {:?}",
            self.time, self.velocity
        );
        Ok(Some(Datum::new(
            self.time,
            State::new(0.0, self.velocity, 0.0),
        )))
    }
}
impl Updatable<()> for Encoder {
    fn update(&mut self) -> NothingOrError<()> {
        self.time += 2;
        self.velocity += 1.0;
        Ok(())
    }
}
struct DCMotor {
    pub power: f32,
    pub time: i64,
    settable_data: SettableData<f32, ()>,
}
impl DCMotor {
    pub fn new() -> Self {
        Self {
            power: 0.0,
            time: 0,
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
        println!("DC Motor power set to {:?}", value);
        self.time += 2;
        self.power = value;
        Ok(())
    }
}
impl Updatable<()> for DCMotor {
    fn update(&mut self) -> NothingOrError<()> {
        Ok(())
    }
}
struct ServoMotorWriteOnly {
    settable_data: SettableData<Command, ()>,
    pub command: Option<Command>,
    pub time: i64,
}
impl ServoMotorWriteOnly {
    pub fn new() -> Self {
        Self {
            settable_data: SettableData::new(),
            command: None,
            time: 0,
        }
    }
}
impl Settable<Command, ()> for ServoMotorWriteOnly {
    fn get_settable_data_ref(&self) -> &SettableData<Command, ()> {
        &self.settable_data
    }
    fn get_settable_data_mut(&mut self) -> &mut SettableData<Command, ()> {
        &mut self.settable_data
    }
    fn direct_set(&mut self, new_command: Command) -> NothingOrError<()> {
        println!(
            "Write-Only Servo Motor {:?} commanded to {:?}",
            new_command.position_derivative, new_command.value
        );
        self.command = Some(new_command);
        Ok(())
    }
}
impl Updatable<()> for ServoMotorWriteOnly {
    fn update(&mut self) -> NothingOrError<()> {
        self.time += 2;
        Ok(())
    }
}
struct ServoMotorReadWrite {
    settable_data: SettableData<Command, ()>,
    pub command: Option<Command>,
    pub time: i64,
}
impl ServoMotorReadWrite {
    pub fn new() -> Self {
        Self {
            settable_data: SettableData::new(),
            command: None,
            time: 0,
        }
    }
}
impl GetterSettable<State, Command, ()> for ServoMotorReadWrite {}
impl Settable<Command, ()> for ServoMotorReadWrite {
    fn get_settable_data_ref(&self) -> &SettableData<Command, ()> {
        &self.settable_data
    }
    fn get_settable_data_mut(&mut self) -> &mut SettableData<Command, ()> {
        &mut self.settable_data
    }
    fn direct_set(&mut self, new_command: Command) -> NothingOrError<()> {
        println!(
            "Read-Write Servo Motor {:?} commanded to {:?}",
            new_command.position_derivative, new_command.value
        );
        self.command = Some(new_command);
        Ok(())
    }
}
impl Getter<State, ()> for ServoMotorReadWrite {
    fn get(&self) -> Output<State, ()> {
        Ok(None)
    }
}
impl Updatable<()> for ServoMotorReadWrite {
    fn update(&mut self) -> NothingOrError<()> {
        self.time += 2;
        Ok(())
    }
}
fn main() {
    println!("RRTK Device System Demo");
    let encoder = Device::Read(Box::new(Encoder::new()));
    let dc_motor = Device::ImpreciseWrite(
        Box::new(DCMotor::new()),
        PositionDerivativeDependentPIDKValues::new(
            PIDKValues::new(1.0, 0.01, 0.1),
            PIDKValues::new(1.0, 0.01, 0.1),
            PIDKValues::new(1.0, 0.01, 0.1),
        ),
    );
    let servo_motor_1 = Device::PreciseWrite(Box::new(ServoMotorWriteOnly::new()));
    let servo_motor_2 = Device::ReadWrite(Box::new(ServoMotorReadWrite::new()));
    let mut axle = Axle::new([encoder, dc_motor, servo_motor_1, servo_motor_2]);
    axle.set(Command::new(PositionDerivative::Velocity, 5.0))
        .unwrap();
    for _ in 0..10 {
        axle.update().unwrap();
    }
}
