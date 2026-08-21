# RRTK: Rust Robotics ToolKit
**A data flow-based robotics framework designed for embedded systems.**

RRTK works almost entirely without `std` and `alloc`. It is not specific to any device or API.

## License: BSD 3-Clause
RRTK is free and open source software licensed under the permissive BSD 3-Clause "New" or "Revised" License. See the LICENSE file in the repository for more information.

## Features
- Architecture based on `Getter`, `Settable`, and `Updatable` traits
    - For getting or setting multiple things, `Getter` and `Settable` can be implemented multiple times using the newtype pattern.
    - All data returned by `Getter` are timestamped.
        - A `TimeGetter` trait is included for clocks used for timestamping data.
    - `Getter`, `Settable`, and `TimeGetter` require `Updatable`.
- Stream system for data processing (see below for more information)
    - Basic arithmetic including exponentiation
    - Numerical trapezoidal integration and differentiation
    - Logic and control flow management
    - PID control
    - Moving average
    - Exponentially weighted moving average (EWMA)
- Graph-based device control system (see below for more information)
    - Differential, gear train, and clutch
    - Wrappers for implementors of `Getter` and `Settable`, including streams
- Trapezoidal motion profile following
- No dependencies by default, but support for [libm](https://crates.io/crates/libm) and [micromath](https://crates.io/crates/micromath) available
    - No features of libm or micromath enabled
    - Compatible with almost any versions of libm and micromath
- Compile-time dimensional analysis system

## Stream system
Special `Getter` implementors called *streams* can be created that hold other Getters as input and, in their own `Getter` implementation, return values calculated from what is returned by those input Getters. They take input from Getters, process it, and return it from their own `Getter` implementations. Streams can be chained for more complex operations. Streams always update their inputs in their own `Updatable` implementations. RRTK includes many common and useful streams, and it is very easy to implement your own through the Getter trait. The stream system is designed to be a simple, efficient way of managing real-time data processing.

## Related Crates
Currently, neither of these works very well with the 0.7.* release series. They will be updated eventually, but improving RRTK itself is being prioritized more right now.

[RRTK Stream Builder](https://crates.io/crates/rrtk_stream_builder): Code generation from visual nodes for the stream system.

[RRTK Procedural Macros](https://crates.io/crates/rrtk_proc): Procedural `math!` macro making the stream system easier to use.

The changelog has been moved to CHANGELOG.md.
