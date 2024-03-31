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
- - Spline-based path planning
- Feed forward control
- Jerk control
- - S-curve motion profile

## Changes
### 0.1.0
Initial release.
### 0.1.1
Fix motion profile issue.
### 0.2.0-alpha.1
New motor-encoder system.
### 0.2.0-alpha.2
Function for motors to follow motion profiles.
### 0.2.0-beta.1
Allow the user to run a custom update loop for motion profile following as an alternative to the single function.
### 0.2.0-beta.2
Add an update method to encoders.
### 0.2.0
Add an update method to motors and reorganize the package to use features with the motor-encoder system in a module.
