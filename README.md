# Rust Robotics ToolKit
**A set of algorithms and other tools for robotics in Rust.**

It is almost entirely `no_std` and most things work without `alloc`. It does not currently integrate with any API directly. This may be added in the future, probably through another crate.

## License: BSD 3-Clause
This basically means that you can do whatever you want as long as you give me attribution and you don't remove the license notices or use my name to endorse stuff I don't. Read the actual license for details though.

**RRTK was previously licensed under LGPL. Versions 0.5.0-alpha.1 and earlier have been retroactively dual licensed under LGPL-3.0-only OR BSD-3-Clause. Versions after 0.5.0-alpha.1 are just BSD-3-Clause.** This transition does not remove any freedoms, and the LGPL for old versions is maintained solely due to its irrevocability. It is entirely redundant freedoms-wise.

## Features
- Architecture based on `Getter`, `Settable`, and `Updatable` traits
- Node-like stream system for data processing
    - Basic arithmetic + integral and derivative
    - Logic and control flow management
    - PID
    - Moving average
    - EWMA
    - Trait for making your own
- Graph-based device control system
    - Devices hold terminals which can be connected together
    - Differential, axle, and direction reversal builtin
    - Easily connect streams to the device system through wrappers
- Trapezoidal motion profile following

## Related Crates
[RRTK Stream Builder](https://crates.io/crates/rrtk_stream_builder): Code generation from visual nodes for the stream system.

[RRTK Procedural Macros](https://crates.io/crates/rrtk_proc) [HIGHLY EXPERIMENTAL]: Procedural `math!` macro making the stream system easier to use.

The changelog has been moved to CHANGELOG.md.
