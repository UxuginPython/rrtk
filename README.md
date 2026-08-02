# Rust Robotics ToolKit
**Systems and tools for robotics programming in Rust.**

RRTK works almost entirely without `std` and `alloc`. It is not specific to any device or API.

## License: BSD 3-Clause
RRTK is free and open source software licensed under the permissive BSD 3-Clause "New" or "Revised" License. See the LICENSE file in the repository for more information.

## Features
- Architecture based on `Getter`, `Settable`, and `Updatable` traits
- Node-like stream system for data processing
    - Basic arithmetic, exponentiation, and integral and derivative
    - Logic and control flow management
    - PID control
    - Moving average
    - Exponentially weighted moving average (EWMA)
    - New streams can be added easily with the `Getter` trait.
- Graph-based device control system
    - Devices hold terminals which can be connected together
    - Differential, gear train, and clutch provided
    - Easily connect streams to the device system through wrappers
- Trapezoidal motion profile following
- No dependencies by default, but support for [libm](https://crates.io/crates/libm) and [micromath](https://crates.io/crates/micromath) available
  - No features of libm or micromath enabled
- Compile-time dimensional analysis system

## Related Crates
Currently, neither of these works very well with the 0.7.* release series. They will be updated eventually, but improving RRTK itself is being prioritized more right now.

[RRTK Stream Builder](https://crates.io/crates/rrtk_stream_builder): Code generation from visual nodes for the stream system.

[RRTK Procedural Macros](https://crates.io/crates/rrtk_proc): Procedural `math!` macro making the stream system easier to use.

The changelog has been moved to CHANGELOG.md.
