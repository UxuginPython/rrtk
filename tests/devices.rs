// SPDX-License-Identifier: LGPL-3.0-only
/*
Copyright 2024 UxuginPython on GitHub

     This file is part of Rust Robotics ToolKit.

    Rust Robotics ToolKit is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, version 3.

    Rust Robotics ToolKit is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Lesser General Public License for more details.

    You should have received a copy of the GNU Lesser General Public License along with Rust Robotics ToolKit. If not, see <https://www.gnu.org/licenses/>.
*/
use rrtk::*;
#[test]
fn encoder() {
    struct DummyEncoder {}
    impl DummyEncoder {
        fn new() -> DummyEncoder {
            DummyEncoder {}
        }
    }
    impl Encoder for DummyEncoder {
        fn get_state(&mut self) -> Datum<State> {
            Datum::new(1.0, State::new(2.0, 3.0, 4.0))
        }
    }
    let mut my_encoder = DummyEncoder::new();
    let output = my_encoder.get_state();
    assert_eq!(output.time, 1.0);
    assert_eq!(output.value.position, 2.0);
    assert_eq!(output.value.velocity, 3.0);
    assert_eq!(output.value.acceleration, 4.0);
}
#[test]
fn velocity_encoder() {
    struct DummyVelocityEncoder {
        velocity_encoder_data: VelocityEncoderData,
        time: f32,
        value: f32,
    }
    impl DummyVelocityEncoder {
        fn new(state: Datum<State>) -> DummyVelocityEncoder {
            DummyVelocityEncoder {
                velocity_encoder_data: VelocityEncoderData::new(state.clone()),
                time: state.time,
                value: state.value.velocity,
            }
        }
    }
    impl VelocityEncoder for DummyVelocityEncoder {
        fn get_velocity_encoder_data_ref(&self) -> &VelocityEncoderData {
            &self.velocity_encoder_data
        }
        fn get_velocity_encoder_data_mut(&mut self) -> &mut VelocityEncoderData {
            &mut self.velocity_encoder_data
        }
        fn device_update(&mut self) -> Datum<f32> {
            self.time += 0.1;
            self.value += 2.0;
            Datum::new(self.time, self.value)
        }
    }
    let mut my_velocity_encoder = DummyVelocityEncoder::new(Datum::new(1.0, State::new(2.0, 3.0, 4.0)));
    let state = my_velocity_encoder.get_state();
    assert_eq!(state.time, 1.0);
    assert_eq!(state.value.position, 2.0);
    assert_eq!(state.value.velocity, 3.0);
    assert_eq!(state.value.acceleration, 4.0);
    my_velocity_encoder.update();
    let state = my_velocity_encoder.get_state();
    assert_eq!(state.time, 1.1);
    assert_eq!(state.value.position, 2.4);
    assert_eq!(state.value.velocity, 5.0);
    assert!(19.999 < state.value.acceleration && state.value.acceleration < 20.001); //float error
}
