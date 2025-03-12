// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2024-2025 UxuginPython
#[cfg(all(feature = "devices", feature = "alloc"))]
const COMMAND: Command = Command::new(PositionDerivative::Position, 5.0);
#[cfg(all(feature = "devices", feature = "alloc"))]
const STATE: State = State::new_raw(0.0, 0.0, 0.0);
#[cfg(all(feature = "devices", feature = "alloc"))]
const K_VALUES: PositionDerivativeDependentPIDKValues = PositionDerivativeDependentPIDKValues::new(
    PIDKValues::new(1.0, 0.01, 0.1),
    PIDKValues::new(1.0, 0.01, 0.1),
    PIDKValues::new(1.0, 0.01, 0.1),
);
#[cfg(all(feature = "devices", feature = "alloc"))]
use rrtk::*;
#[cfg(all(feature = "devices", feature = "alloc"))]
struct Motor {
    settable_data: SettableData<f32, ()>,
}
#[cfg(all(feature = "devices", feature = "alloc"))]
impl Motor {
    fn new() -> Self {
        Self {
            settable_data: SettableData::new(),
        }
    }
}
#[cfg(all(feature = "devices", feature = "alloc"))]
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
#[cfg(all(feature = "devices", feature = "alloc"))]
impl Updatable<()> for Motor {
    fn update(&mut self) -> NothingOrError<()> {
        self.update_following_data().unwrap();
        Ok(())
    }
}
#[cfg(all(feature = "devices", feature = "alloc"))]
#[derive(Default)]
struct Encoder {
    time: Time,
}
#[cfg(all(feature = "devices", feature = "alloc"))]
impl Getter<State, ()> for Encoder {
    fn get(&self) -> Output<State, ()> {
        println!("Encoder returning state {:?}", STATE);
        Ok(Some(Datum::new(self.time, STATE)))
    }
}
#[cfg(all(feature = "devices", feature = "alloc"))]
impl Updatable<()> for Encoder {
    fn update(&mut self) -> NothingOrError<()> {
        self.time += Time::from_nanoseconds(1_000_000_000);
        Ok(())
    }
}
#[cfg(all(feature = "devices", feature = "alloc"))]
fn main() {
    println!("Commanding Motor to {:?}", COMMAND);
    println!(
        "K values are {:?}",
        K_VALUES.get_k_values(PositionDerivative::from(COMMAND))
    );
    let motor = Motor::new();
    let mut motor_wrapper =
        devices::wrappers::PIDWrapper::new(motor, Time::ZERO, STATE, COMMAND, K_VALUES);
    let encoder = Encoder::default();
    let mut encoder_wrapper = devices::wrappers::GetterStateDeviceWrapper::new(encoder);
    connect(motor_wrapper.get_terminal(), encoder_wrapper.get_terminal());
    for _ in 0..5 {
        motor_wrapper.update().unwrap();
        encoder_wrapper.update().unwrap();
    }
}
#[cfg(all(not(feature = "devices"), feature = "alloc"))]
fn main() {
    println!(
        "Enable the `devices` feature to run this example.\nAssuming you're using Cargo, add the `--features devices` flag to your command."
    );
}
#[cfg(all(not(feature = "alloc"), feature = "devices"))]
fn main() {
    println!(
        "Enable the `alloc` feature to run this example.\nAssuming you're using Cargo, add the `--features alloc` flag to your command."
    );
}
#[cfg(all(not(feature = "alloc"), not(feature = "devices")))]
fn main() {
    println!(
        "Enable the `alloc` and `devices` features to run this example.\nAssuming you're using Cargo, add the `--features alloc,devices` flag to your command."
    );
}
