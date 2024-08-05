# Rust Robotics ToolKit
### A set of algorithms and other tools for robotics in Rust.

It is partially `no_std`. It does not currently integrate with any API directly, but this may be added in the future.

## License
#### GNU Lesser General Public License, version 3 only

## Features
- Node-like stream system for data processing
    - Basic arithmetic + integral and derivative
    - PID
    - Moving average
    - EWMA
- Simple device control system
- Trapezoidal motion profile following

[RRTK Stream Builder](https://crates.io/crates/rrtk_stream_builder) now available: Code generation from visual nodes for the stream system.

## Changes
### 0.1.0
Initial release.
### 0.1.1
Fix motion profile issue.
### 0.2.0-alpha.1
Start new motor-encoder system.
### 0.2.0-alpha.2
Function for motors to follow motion profiles.
### 0.2.0-beta.1
Allow the user to run a custom update loop for motion profile following as an alternative to the single function.
### 0.2.0-beta.2
Add an update method to encoders.
### 0.2.0
Add an update method to motors, allow easier detection of parts of motion profiles, and reorganize the package to use features with the motor-encoder system in a module.
### 0.3.0-alpha.1
Start new stream system.
### 0.3.0-alpha.2
Reorganize a bit and add [EWMA](https://www.itl.nist.gov/div898/handbook/pmc/section3/pmc324.htm) stream.
### 0.3.0-alpha.3
Add moving average stream.
### 0.3.0-alpha.4
- performance improvements
    - use array instead of vec for inputs to `SumStream` and `ProductStream`
    - avoid unnecessary weight sum calculation in `MovingAverageStream`
    - make the number of shifts in `PIDControllerShift` a constant
- replace all instances of `MotionProfileState` with `MotionProfilePiece`
- add `History` trait, which is like a `Stream` but you specify a time when you `get`
- reorganize streams into modules
- remove unnecessary `std` requirement for a couple types
### 0.3.0-alpha.5
- Move from `Stream` and the previous device system to `Getter` and `Settable`. `Getter` is like a stream or encoder and `Settable` is like a writable device.
- Add `Device` type which makes raw `Getter`s and `Settable`s work together better as mechanical devices in a system. This should represent a physical device.
- Add `Axle` type which contains multiple `Device` objects. It uses the capabilities of each device to control the real-life system. Eg. Data is gathered from `Getter` devices (`Device::Read` for encoders and `Device::ReadWrite` for servos) and used to control motors that do not contain their own control theory processing (`Device::ImpreciseWrite`), but motors that can do this on their own (`Device::ReadWrite` and `Device::PreciseWrite` depending on whether the internal data can be read) do not need this control. This object should represent a physical linkage between devices.
- Don't require a feature to be enabled for PID controller types
- Change API for PID controller types to be constructed with a k-values type rather than three individual `f32`s.
### 0.3.0-beta.1
- Don't require a feature to be enabled for motion profiles.
- Make `Settable` able to follow `Getter`s of the same type.
- Add `GetterFromHistory` struct allowing `History` objects to be used as `Getter`s.
### 0.3.0
- Add `set_delta` and `set_time` methods to `GetterFromHistory`.
- Move `streams::Constant` to `ConstantGetter`.
- Implement `Settable` for `ConstantGetter`.
- Add `get_last_request` method to `Settable`.
- Move `MotionProfile` `get_*` methods to `Option` instead of `Result`.
- Rename `UpdateOutput` to `NothingOrError`.
- Fix `Axle` bug where it would try to use nonexistent PID controllers for `Device::ImpreciseWrite` objects if it had not yet received a `Command`.
- Instead of directly implementing `set` in `Settable`, you now implement `direct_set`. You should still *call* just `set` though. This is a workaround required to make `SettableData` and `get_last_request` work correctly.
- Move `MotionProfile` to `History<Command, E>` instead of `History<State, E>`.
- Move timestamps to `i64` instread of `f32`. The recommended unit is nanoseconds. This is not `u64` due to the use of deltas.
- Fix `MovingAverageStream` panicing issue.
- Rename `StreamPID` to `PIDControllerStream`.
- Improve performance of `PIDControllerStream`.
- Mark `Error` enum as non-exhaustive.
- Write three example files.
- Derive additional traits for a few structs.
- Give `MotionProfile` a return value after it has completed. This is based on the end state provided to the constructor. It will choose the lowest possible position derivative to satisfy the end state. This means that if acceleration is 0, the position derivative in the command will be velocity, otherwise acceleration. If velocity is also 0, it will be position, otherwise just velocity.
- Add `get_(position|velocity|acceleration)` methods to `Command`.
- Add `Latest` stream allowing you to choose the output of whichever of a set of streams has the later timestamp.
- Implement `From<State>` for `Command`.
- Rename `TimeGetterFromStream` to `TimeGetterFromGetter`.
### 0.3.1
- Implement several `core::ops` traits and `Copy` for `State`
- Fix name of `PositionToState`
- Slightly improve performance of `MotionProfile` and `(Position|Velocity|Acceleration)ToState` by removing unnecessary code
- Improve tests
- Minor documentation fixes
- Add missing LGPL license notice to a few files
### 0.4.0-alpha.1
- Begin new device system.
### 0.4.0-alpha.2
- Make everything use `&RefCell<Terminal>` rather than `Rc<RefCell<Terminal>>`
- Make math streams use generics.
- Add `SettableCommandDeviceWrapper` and `GetterStateDeviceWrapper` allowing types only implementing `Settable<Command, _>` to be used as motors and types only implementing `Getter<State, _>` to be used as encoders.
- Revive `PositionDerivativeDependentPIDKValues`, now with a `get_k_values` method for getting the k-values for a specific position derivative.
- Add `evaluate` methods for `PIDKValues` and `PositionDerivativeDependentPIDKValues`.
- Add `CommandPID`, an easier and faster way to use PID control to turn a standard DC motor and an encoder into a de facto servo.
- Add `latest` function which gets the newer of two `Datum` objects.
### 0.4.0-alpha.3
- Add new `streams` submodules `flow` and `logic`.
- Add new streams
    - `Expirer`
    - `flow::IfStream`
    - `flow::IfElseStream`
    - `flow::FreezeStream`
    - `logic::AndStream`
    - `logic::OrStream`
    - `logic::NotStream`
- Pass through `Not` for `Datum<T>` where `T` implements `Not`.
- Add `NoneGetter`.
- Add `Axle` very similar to 0.4.0-alpha.1 one.
- Move `(SettableCommand|GetterState)DeviceWrapper` to `devices::wrappers` module.
- Add experimental `Device` implementor for a differential mechanism.
- Remove now-unused `GetterSettable` marker trait.
- Move new device system to a new `devices` feature.
- Minor documentation fix for `devices` module.
### 0.4.0-beta.1
- Make differential calculations able to trust all branches equally instead of ignoring one.
- Remove unnecessary `Box`ing from `InputGetter` and `InputTimeGetter`.
