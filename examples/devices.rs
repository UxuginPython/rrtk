// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2024-2026 UxuginPython
#[cfg(feature = "devices")]
use rrtk::devices::provided::Differential;
#[cfg(feature = "devices")]
use rrtk::devices::*;
#[cfg(feature = "devices")]
use rrtk::*;
///Format [`AngularState`] like a row vector for ease of reading.
#[cfg(feature = "devices")]
fn format_state(state: AngularState) -> String {
    format!(
        "[ {} {} {} ]",
        state.position.into_inner(),
        state.velocity.into_inner(),
        state.acceleration.into_inner()
    )
}
#[cfg(feature = "devices")]
fn main() {
    let mut system = System::<6>::new();
    let diff_left = system.new_node().unwrap();
    let diff_right = system.new_node().unwrap();
    let diff_top = system.new_node().unwrap();
    let enc_left = system.new_node().unwrap();
    let enc_right = system.new_node().unwrap();
    let demo_top = system.new_node().unwrap();
    system.connect(diff_left, enc_left);
    system.connect(diff_right, enc_right);
    system.connect(diff_top, demo_top);
    //The provided Differential device adds the states of its left and right nodes.
    let mut differential = Differential::new(diff_left, diff_right, diff_top);
    //Fake encoders by directly setting the states.
    system.set_state_local(
        enc_left,
        Some(AngularState::new(
            Dimensionless::new(1.0),
            InverseSecond::new(2.0),
            InverseSecondSquared::new(3.0),
        )),
    );
    system.set_state_local(
        enc_right,
        Some(AngularState::new(
            Dimensionless::new(4.0),
            InverseSecond::new(5.0),
            InverseSecondSquared::new(6.0),
        )),
    );
    //There's only one device in this example, and it's not actually a continuously updating
    //system, so we only update once. However, when you actually use the device system, you should
    //call device_update for all your devices in a loop. The order of the devices in this loop
    //shouldn't usually matter significantly, but you may want to experiment with it if you need
    //the utmost updating speed.
    //As for the fully qualified syntax, it's needed right now because of the way that the
    //infallible devices are implemented. It will not be needed in a future version.
    <Differential as DeviceUpdatable<core::convert::Infallible>>::device_update(
        &mut differential,
        &mut system,
    );
    //We directly set the encoder states, so we use get_state_local to access them.
    let enc_left_state = system.get_state_local(enc_left).unwrap();
    let enc_right_state = system.get_state_local(enc_right).unwrap();
    //demo_top is connected to diff_top, and diff_top is providing the state, so we use
    //get_state_connected. This is one of a few cases where get_state_true would also be fine to
    //use.
    let demo_state = system.get_state_connected(demo_top).unwrap();
    println!(
        "{} + {} = {}",
        format_state(enc_left_state),
        format_state(enc_right_state),
        format_state(demo_state)
    );
}
#[cfg(not(feature = "devices"))]
fn main() {
    println!(
        "Enable the `devices` feature to run this example.\nAssuming you're using Cargo, add the `--features devices` flag to your command."
    );
}
