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
