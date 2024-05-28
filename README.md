# Rust Robotics ToolKit
### A set of algorithms and other tools for robotics in Rust.

It is partially `no_std`. It does not currently integrate with any API directly, but this may be added in the future.

## License
#### GNU Lesser General Public License, version 3 only

## Available
- PID controller\*\*
- Trapezoidal motion profile\*\*
- Motor and encoder control system\*

\*Partially available in `no_std`

\*\*Fully available in `no_std`

## Future
- Stream system
- Noise filtering
- Drive base control
    - Spline-based path planning
- Feed forward control
- Jerk control
    - S-curve motion profile

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
