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
