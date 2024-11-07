// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2024 UxuginPython
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
impl<E: Copy + Debug> History<Command, E> for MotionProfile {
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
        Some(Datum::new(
            time,
            Command::try_from(value).expect(
                "This cannot return anything other than position, velocity, and acceleration.",
            ),
        ))
    }
}
impl<E: Copy + Debug> Updatable<E> for MotionProfile {
    fn update(&mut self) -> NothingOrError<E> {
        Ok(())
    }
}
impl MotionProfile {
    ///Constructor for `MotionProfile` using start and end states.
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
            max_acc: max_acc,
            end_command: end_command,
        }
    }
    ///Get the intended `PositionDerivative` at a given time.
    pub fn get_mode(&self, t: Time) -> Option<PositionDerivative> {
        if t < Time::default() {
            return None;
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
    ///Get the `MotionProfilePiece` at a given time.
    pub fn get_piece(&self, t: Time) -> MotionProfilePiece {
        if t < Time::default() {
            return MotionProfilePiece::BeforeStart;
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
            return None;
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
            return None;
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
            return None;
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
            State::new(0.0, 0.0, 0.0),
            State::new(3.0, 0.0, 0.0),
            0.1,
            0.01,
        );
        assert_eq!(motion_profile.t1, 10);
        assert_eq!(motion_profile.t2, 30);
        assert_eq!(motion_profile.t3, 40);
        assert_eq!(motion_profile.max_acc, 0.01);
    }
    #[test]
    fn motion_profile_new_2() {
        let motion_profile = MotionProfile::new(
            State::new(1.0, 0.0, 0.0),
            State::new(3.0, 0.0, 0.0),
            0.1,
            0.01,
        );
        assert_eq!(motion_profile.t1, 10);
        assert_eq!(motion_profile.t2, 20);
        assert_eq!(motion_profile.t3, 30);
        assert_eq!(motion_profile.max_acc, 0.01);
    }
    #[test]
    fn motion_profile_new_3() {
        let motion_profile = MotionProfile::new(
            State::new(0.0, 0.1, 0.0),
            State::new(3.0, 0.0, 0.0),
            0.1,
            0.01,
        );
        assert_eq!(motion_profile.t1, 0);
        assert_eq!(motion_profile.t2, 25);
        assert_eq!(motion_profile.t3, 35);
        assert_eq!(motion_profile.max_acc, 0.01);
    }
    #[test]
    fn motion_profile_new_4() {
        let motion_profile = MotionProfile::new(
            State::new(0.0, 0.0, 0.01),
            State::new(3.0, 0.0, 0.0),
            0.1,
            0.01,
        );
        assert_eq!(motion_profile.t1, 10);
        assert_eq!(motion_profile.t2, 30);
        assert_eq!(motion_profile.t3, 40);
        assert_eq!(motion_profile.max_acc, 0.01);
    }
    #[test]
    fn motion_profile_new_5() {
        let motion_profile = MotionProfile::new(
            State::new(0.0, 0.0, 0.0),
            State::new(6.0, 0.0, 0.0),
            0.2,
            0.01,
        );
        assert_eq!(motion_profile.t1, 20);
        assert_eq!(motion_profile.t2, 30);
        assert_eq!(motion_profile.t3, 50);
        assert_eq!(motion_profile.max_acc, 0.01);
    }
    #[test]
    fn motion_profile_new_6() {
        let motion_profile = MotionProfile::new(
            State::new(0.0, 0.0, 0.0),
            State::new(3.0, 0.0, 0.0),
            0.1,
            0.02,
        );
        assert_eq!(motion_profile.t1, 5);
        assert_eq!(motion_profile.t2, 30);
        assert_eq!(motion_profile.t3, 35);
        assert_eq!(motion_profile.max_acc, 0.02);
    }
    #[test]
    fn motion_profile_new_7() {
        let motion_profile = MotionProfile::new(
            State::new(0.0, 0.0, 0.0),
            State::new(-3.0, 0.0, 0.0),
            0.1,
            0.01,
        );
        assert_eq!(motion_profile.t1, 10);
        assert_eq!(motion_profile.t2, 30);
        assert_eq!(motion_profile.t3, 40);
        assert_eq!(motion_profile.max_acc, -0.01);
    }
}
