// SPDX-License-Identifier: LGPL-3.0-only
/*
Copyright 2024 UxuginPython on GitHub

     This file is part of Rust Robotics ToolKit.

    Rust Robotics ToolKit is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, version 3.

    Rust Robotics ToolKit is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Lesser General Public License for more details.

    You should have received a copy of the GNU Lesser General Public License along with Rust Robotics ToolKit. If not, see <https://www.gnu.org/licenses/>.
*/
use rrtk::*;
use std::cell::RefCell;
use std::rc::Rc;
struct ServoMotor {
    pub state: State,
    pub time_getter: InputTimeGetter<()>,
    settable_data: SettableData<Command, ()>,
}
impl ServoMotor {
    pub fn new(time_getter: InputTimeGetter<()>) -> Self {
        Self {
            state: State::new(0.0, 0.0, 0.0),
            time_getter: time_getter,
            settable_data: SettableData::new(),
        }
    }
}
impl GetterSettable<State, Command, ()> for ServoMotor {}
impl Settable<Command, ()> for ServoMotor {
    fn get_settable_data_ref(&self) -> &SettableData<Command, ()> {
        &self.settable_data
    }
    fn get_settable_data_mut(&mut self) -> &mut SettableData<Command, ()> {
        &mut self.settable_data
    }
    fn direct_set(&mut self, command: Command) -> NothingOrError<()> {
        //println!("{:?} {:?}", command.position_derivative, command.value);
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
impl Getter<State, ()> for ServoMotor {
    fn get(&self) -> Output<State, ()> {
        Ok(Some(Datum::new(
            self.time_getter.borrow().get()?,
            self.state.clone(),
        )))
    }
}
impl Updatable<()> for ServoMotor {
    fn update(&mut self) -> NothingOrError<()> {
        self.state.update(1);
        Ok(())
    }
}
struct MyTimeGetter {
    time: i64,
}
impl MyTimeGetter {
    pub fn new() -> Self {
        Self { time: 0 }
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
fn main() {
    println!("RRTK Motion Profile Following Example");
    let time_getter = make_input_time_getter!(MyTimeGetter::new(), ());
    let motion_profile = MotionProfile::new(
        State::new(0.0, 0.0, 0.0),
        State::new(300.0, 0.0, 0.0),
        10.0,
        1.0,
    );
    let motion_profile =
        GetterFromHistory::new_for_motion_profile(motion_profile, Rc::clone(&time_getter)).unwrap();
    let motion_profile = make_input_getter!(motion_profile, Command, ());
    let servo = Device::ReadWrite(Box::new(ServoMotor::new(Rc::clone(&time_getter))));
    let mut axle = Axle::new([servo]);
    axle.follow(motion_profile);
    axle.following_update().unwrap();
    let statedatum = axle.get().unwrap().unwrap();
    println!("{:?}", statedatum);
    loop {
        time_getter.borrow_mut().update().unwrap();
        axle.following_update().unwrap();
        let statedatum = axle.get().unwrap().unwrap();
        println!("{:?}", statedatum);
        if statedatum.value.velocity == 0.0 {
            break;
        }
    }
}
