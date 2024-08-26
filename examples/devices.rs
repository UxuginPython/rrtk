// SPDX-License-Identifier: LGPL-3.0-only
/*
Copyright 2024 UxuginPython on GitHub

     This file is part of Rust Robotics ToolKit.

    Rust Robotics ToolKit is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, version 3.

    Rust Robotics ToolKit is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Lesser General Public License for more details.

    You should have received a copy of the GNU Lesser General Public License along with Rust Robotics ToolKit. If not, see <https://www.gnu.org/licenses/>.
*/
#[cfg(feature = "devices")]
const COMMAND: Command = Command::new(PositionDerivative::Position, 5.0);
#[cfg(feature = "devices")]
const STATE: State = State::new(0.0, 0.0, 0.0);
#[cfg(feature = "devices")]
const K_VALUES: PositionDerivativeDependentPIDKValues = PositionDerivativeDependentPIDKValues::new(
    PIDKValues::new(1.0, 0.01, 0.1),
    PIDKValues::new(1.0, 0.01, 0.1),
    PIDKValues::new(1.0, 0.01, 0.1),
);
#[cfg(feature = "devices")]
use rrtk::*;
#[cfg(feature = "devices")]
struct Motor {
    settable_data: SettableData<f32, ()>,
}
#[cfg(feature = "devices")]
impl Motor {
    fn new() -> Self {
        Self {
            settable_data: SettableData::new(),
        }
    }
}
#[cfg(feature = "devices")]
impl Settable<f32, ()> for Motor {
    fn impl_set(&mut self, value: f32) -> NothingOrError<()> {
        println!("Motor voltage set to {:?}", value);
        Ok(())
    }
    fn get_settable_data_ref(&self) -> &SettableData<f32, ()> {
        &self.settable_data
    }
    fn get_settable_data_mut(&mut self) -> &mut SettableData<f32, ()> {
        &mut self.settable_data
    }
}
#[cfg(feature = "devices")]
impl Updatable<()> for Motor {
    fn update(&mut self) -> NothingOrError<()> {
        self.update_following_data().unwrap();
        Ok(())
    }
}
#[cfg(feature = "devices")]
#[derive(Default)]
struct Encoder {
    time: i64,
}
#[cfg(feature = "devices")]
impl Getter<State, ()> for Encoder {
    fn get(&self) -> Output<State, ()> {
        println!("Encoder returning state {:?}", STATE);
        Ok(Some(Datum::new(self.time, STATE)))
    }
}
#[cfg(feature = "devices")]
impl Updatable<()> for Encoder {
    fn update(&mut self) -> NothingOrError<()> {
        self.time += 1;
        Ok(())
    }
}
#[cfg(feature = "devices")]
fn main() {
    println!("Commanding Motor to {:?}", COMMAND);
    println!(
        "K values are {:?}",
        K_VALUES.get_k_values(COMMAND.position_derivative)
    );
    let motor = Motor::new();
    let mut motor_wrapper = devices::wrappers::PIDWrapper::new(motor, 0, STATE, COMMAND, K_VALUES);
    let encoder = Encoder::default();
    let mut encoder_wrapper = devices::wrappers::GetterStateDeviceWrapper::new(encoder);
    connect(motor_wrapper.get_terminal(), encoder_wrapper.get_terminal());
    for _ in 0..5 {
        motor_wrapper.update().unwrap();
        encoder_wrapper.update().unwrap();
    }
}
#[cfg(not(feature = "devices"))]
fn main() {
    println!("Enable the `devices` feature to run this example.\nAssuming you're using Cargo, add the `--features devices` flag to your command.");
}
