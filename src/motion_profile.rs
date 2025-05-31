// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2024-2025 UxuginPython
use crate::*;
///Where you are in following a motion profile.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MotionProfilePiece {
    ///You have not yet started the motion profile.
    BeforeStart,
    ///You are changing velocity at the beginning.
    InitialAcceleration,
    ///You are moving at a constant speed.
    ConstantVelocity,
    ///You are changing velocity at the end.
    EndAcceleration,
    ///You are done with the motion profile.
    Complete,
}
///A motion profile for getting from one state to another.
#[derive(Clone, Debug, PartialEq)]
pub struct MotionProfile {
    start_pos: Quantity,
    start_vel: Quantity,
    t1: Time,
    t2: Time,
    t3: Time,
    max_acc: Quantity,
    end_command: Command,
}
impl Chronology<Command> for MotionProfile {
    fn get(&self, time: Time) -> Option<Datum<Command>> {
        let mode = match self.get_mode(time) {
            Some(value) => value,
            None => {
                return None;
            }
        };
        let value = match mode {
            PositionDerivative::Position => self
                .get_position(time)
                .expect("If mode is Position, this should be Some."),
            PositionDerivative::Velocity => self
                .get_velocity(time)
                .expect("If mode is Velocity, this should be Some."),
            PositionDerivative::Acceleration => self
                .get_acceleration(time)
                .expect("If mode is Acceleration, this should be Some."),
        };
        Some(Datum::new(time, Command::new(mode, value.into())))
    }
}
//Unfortunately this is one of the times when you might be able to get a bit more functionality
//(more const fns in this case) but at the significant expense of readability and simplicity. The
//real solution here is to stop using runtime Quantity, which will happen at some point. When that
//happens, TODO review what can be const fn again.
impl MotionProfile {
    ///Constructor for [`MotionProfile`] using start and end states.
    pub fn new(
        start_state: State,
        end_state: State,
        max_vel: Quantity,
        max_acc: Quantity,
    ) -> MotionProfile {
        let sign = Quantity::new(
            if end_state.position < start_state.position {
                -1.0
            } else {
                1.0
            },
            DIMENSIONLESS,
        );
        let max_vel = max_vel.abs() * sign;
        let max_acc = max_acc.abs() * sign;
        let d_t1_vel = max_vel - start_state.get_velocity();
        let t1 = d_t1_vel / max_acc;
        assert!(f32::from(t1) >= 0.0);
        let d_t1_pos = (start_state.get_velocity() + max_vel) / Quantity::dimensionless(2.0) * t1;
        let d_t3_vel = end_state.get_velocity() - max_vel;
        let d_t3 = d_t3_vel / -max_acc;
        assert!(f32::from(d_t3) >= 0.0);
        let d_t3_pos = (max_vel + end_state.get_velocity()) / Quantity::dimensionless(2.0) * d_t3;
        let d_t2_pos =
            (end_state.get_position() - start_state.get_position()) - (d_t1_pos + d_t3_pos);
        let d_t2 = d_t2_pos / max_vel;
        assert!(f32::from(d_t2) >= 0.0);
        let t2 = t1 + d_t2;
        let t3 = t2 + d_t3;
        let end_command = Command::from(end_state);
        MotionProfile {
            start_pos: start_state.get_position(),
            start_vel: start_state.get_velocity(),
            t1: Time::try_from(t1).expect(
                "t1 must always be in seconds in max_vel and max_acc have correct dimensions",
            ),
            t2: Time::try_from(t2).expect(
                "t2 must always be in seconds in max_vel and max_acc have correct dimensions",
            ),
            t3: Time::try_from(t3).expect(
                "t3 must always be in seconds in max_vel and max_acc have correct dimensions",
            ),
            max_acc,
            end_command,
        }
    }
    ///Get the intended [`PositionDerivative`] at a given time.
    pub fn get_mode(&self, t: Time) -> Option<PositionDerivative> {
        if t < Time::default() {
            None
        } else if t < self.t1 {
            return Some(PositionDerivative::Acceleration);
        } else if t < self.t2 {
            return Some(PositionDerivative::Velocity);
        } else if t < self.t3 {
            return Some(PositionDerivative::Acceleration);
        } else {
            return Some(self.end_command.into());
        }
    }
    ///Get the [`MotionProfilePiece`] at a given time.
    pub fn get_piece(&self, t: Time) -> MotionProfilePiece {
        if t < Time::default() {
            MotionProfilePiece::BeforeStart
        } else if t < self.t1 {
            return MotionProfilePiece::InitialAcceleration;
        } else if t < self.t2 {
            return MotionProfilePiece::ConstantVelocity;
        } else if t < self.t3 {
            return MotionProfilePiece::EndAcceleration;
        } else {
            return MotionProfilePiece::Complete;
        }
    }
    ///Get the intended acceleration at a given time.
    pub fn get_acceleration(&self, t: Time) -> Option<Quantity> {
        if t < Time::default() {
            None
        } else if t < self.t1 {
            return Some(self.max_acc);
        } else if t < self.t2 {
            return Some(Quantity::new(0.0, MILLIMETER_PER_SECOND_SQUARED));
        } else if t < self.t3 {
            return Some(-self.max_acc);
        } else {
            return Some(self.end_command.get_acceleration());
        }
    }
    ///Get the intended velocity at a given time.
    pub fn get_velocity(&self, t: Time) -> Option<Quantity> {
        if t < Time::default() {
            None
        } else if t < self.t1 {
            return Some(self.max_acc * Quantity::from(t) + self.start_vel);
        } else if t < self.t2 {
            return Some(self.max_acc * Quantity::from(self.t1) + self.start_vel);
        } else if t < self.t3 {
            return Some(self.max_acc * Quantity::from(self.t1 + self.t2 - t) + self.start_vel);
        } else {
            return self.end_command.get_velocity();
        }
    }
    ///Get the intended position at a given time.
    pub fn get_position(&self, t: Time) -> Option<Quantity> {
        if t < Time::default() {
            None
        } else if t < self.t1 {
            let t = Quantity::from(t);
            return Some(
                Quantity::dimensionless(0.5) * self.max_acc * t * t
                    + self.start_vel * t
                    + self.start_pos,
            );
        } else if t < self.t2 {
            return Some(
                self.max_acc * (self.t1 * (-self.t1 / DimensionlessInteger(2) + t))
                    + self.start_vel * Quantity::from(t)
                    + self.start_pos,
            );
        } else if t < self.t3 {
            return Some(
                self.max_acc * (self.t1 * (-self.t1 / DimensionlessInteger(2) + self.t2))
                    - Quantity::dimensionless(0.5)
                        * self.max_acc
                        * ((t - self.t2) * (t - DimensionlessInteger(2) * self.t1 - self.t2))
                    + self.start_vel * Quantity::from(t)
                    + self.start_pos,
            );
        } else {
            return self.end_command.get_position();
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn motion_profile_new_1() {
        let motion_profile = MotionProfile::new(
            State::new_raw(0.0, 0.0, 0.0),
            State::new_raw(3.0, 0.0, 0.0),
            Quantity::new(0.1, MILLIMETER_PER_SECOND),
            Quantity::new(0.01, MILLIMETER_PER_SECOND_SQUARED),
        );
        assert_eq!(motion_profile.t1, Time::from_nanoseconds(10_000_000_000));
        assert_eq!(
            motion_profile.t2 / DimensionlessInteger(1_000_000),
            Time::from_nanoseconds(30_000_000_000) / DimensionlessInteger(1_000_000)
        );
        assert_eq!(motion_profile.t3, Time::from_nanoseconds(40_000_000_000));
        assert_eq!(
            motion_profile.max_acc,
            Quantity::new(0.01, MILLIMETER_PER_SECOND_SQUARED)
        );
    }
    #[test]
    fn motion_profile_new_2() {
        let motion_profile = MotionProfile::new(
            State::new_raw(1.0, 0.0, 0.0),
            State::new_raw(3.0, 0.0, 0.0),
            Quantity::new(0.1, MILLIMETER_PER_SECOND),
            Quantity::new(0.01, MILLIMETER_PER_SECOND_SQUARED),
        );
        assert_eq!(motion_profile.t1, Time::from_nanoseconds(10_000_000_000));
        assert_eq!(motion_profile.t2, Time::from_nanoseconds(20_000_000_000));
        assert_eq!(
            motion_profile.t3 / DimensionlessInteger(1_000_000),
            Time::from_nanoseconds(30_000_000_000) / DimensionlessInteger(1_000_000)
        );
        assert_eq!(
            motion_profile.max_acc,
            Quantity::new(0.01, MILLIMETER_PER_SECOND_SQUARED)
        );
    }
    #[test]
    fn motion_profile_new_3() {
        let motion_profile = MotionProfile::new(
            State::new_raw(0.0, 0.1, 0.0),
            State::new_raw(3.0, 0.0, 0.0),
            Quantity::new(0.1, MILLIMETER_PER_SECOND),
            Quantity::new(0.01, MILLIMETER_PER_SECOND_SQUARED),
        );
        assert_eq!(motion_profile.t1, Time::from_nanoseconds(0));
        assert_eq!(
            (motion_profile.t2 + Time::from_nanoseconds(1000)) / DimensionlessInteger(1_000_000),
            Time::from_nanoseconds(25_000_000_000) / DimensionlessInteger(1_000_000)
        );
        assert_eq!(
            motion_profile.t3 / DimensionlessInteger(1_000_000),
            Time::from_nanoseconds(35_000_000_000) / DimensionlessInteger(1_000_000)
        );
        assert_eq!(
            motion_profile.max_acc,
            Quantity::new(0.01, MILLIMETER_PER_SECOND_SQUARED)
        );
    }
    #[test]
    fn motion_profile_new_4() {
        let motion_profile = MotionProfile::new(
            State::new_raw(0.0, 0.0, 0.01),
            State::new_raw(3.0, 0.0, 0.0),
            Quantity::new(0.1, MILLIMETER_PER_SECOND),
            Quantity::new(0.01, MILLIMETER_PER_SECOND_SQUARED),
        );
        assert_eq!(motion_profile.t1, Time::from_nanoseconds(10_000_000_000));
        assert_eq!(
            motion_profile.t2 / DimensionlessInteger(1_000_000),
            Time::from_nanoseconds(30_000_000_000) / DimensionlessInteger(1_000_000)
        );
        assert_eq!(motion_profile.t3, Time::from_nanoseconds(40_000_000_000));
        assert_eq!(
            motion_profile.max_acc,
            Quantity::new(0.01, MILLIMETER_PER_SECOND_SQUARED)
        );
    }
    #[test]
    fn motion_profile_new_5() {
        let motion_profile = MotionProfile::new(
            State::new_raw(0.0, 0.0, 0.0),
            State::new_raw(6.0, 0.0, 0.0),
            Quantity::new(0.2, MILLIMETER_PER_SECOND),
            Quantity::new(0.01, MILLIMETER_PER_SECOND_SQUARED),
        );
        assert_eq!(motion_profile.t1, Time::from_nanoseconds(20_000_000_000));
        assert_eq!(
            motion_profile.t2 / DimensionlessInteger(1_000_000),
            Time::from_nanoseconds(30_000_000_000) / DimensionlessInteger(1_000_000)
        );
        assert_eq!(
            (motion_profile.t3 + Time::from_nanoseconds(10000)) / DimensionlessInteger(1_000_000),
            Time::from_nanoseconds(50_000_000_000) / DimensionlessInteger(1_000_000)
        );
        assert_eq!(
            motion_profile.max_acc,
            Quantity::new(0.01, MILLIMETER_PER_SECOND_SQUARED)
        );
    }
    #[test]
    fn motion_profile_new_6() {
        let motion_profile = MotionProfile::new(
            State::new_raw(0.0, 0.0, 0.0),
            State::new_raw(3.0, 0.0, 0.0),
            Quantity::new(0.1, MILLIMETER_PER_SECOND),
            Quantity::new(0.02, MILLIMETER_PER_SECOND_SQUARED),
        );
        assert_eq!(motion_profile.t1, Time::from_nanoseconds(5_000_000_000));
        assert_eq!(
            motion_profile.t2 / DimensionlessInteger(1_000_000),
            Time::from_nanoseconds(30_000_000_000) / DimensionlessInteger(1_000_000)
        );
        assert_eq!(
            motion_profile.t3 / DimensionlessInteger(1_000_000),
            Time::from_nanoseconds(35_000_000_000) / DimensionlessInteger(1_000_000)
        );
        assert_eq!(
            motion_profile.max_acc,
            Quantity::new(0.02, MILLIMETER_PER_SECOND_SQUARED)
        );
    }
    #[test]
    fn motion_profile_new_7() {
        let motion_profile = MotionProfile::new(
            State::new_raw(0.0, 0.0, 0.0),
            State::new_raw(-3.0, 0.0, 0.0),
            Quantity::new(0.1, MILLIMETER_PER_SECOND),
            Quantity::new(0.01, MILLIMETER_PER_SECOND_SQUARED),
        );
        assert_eq!(motion_profile.t1, Time::from_nanoseconds(10_000_000_000));
        assert_eq!(
            motion_profile.t2 / DimensionlessInteger(1_000_000),
            Time::from_nanoseconds(30_000_000_000) / DimensionlessInteger(1_000_000)
        );
        assert_eq!(motion_profile.t3, Time::from_nanoseconds(40_000_000_000));
        assert_eq!(
            motion_profile.max_acc,
            Quantity::new(-0.01, MILLIMETER_PER_SECOND_SQUARED)
        );
    }
}
