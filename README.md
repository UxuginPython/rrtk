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
