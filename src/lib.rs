// SPDX-License-Identifier: LGPL-3.0-only
/*
Copyright 2024 Uxugin on GitHub

     This file is part of Rust Robotics ToolKit.

    Rust Robotics ToolKit is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, version 3.

    Rust Robotics ToolKit is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Lesser General Public License for more details.

    You should have received a copy of the GNU Lesser General Public License along with Rust Robotics ToolKit. If not, see <https://www.gnu.org/licenses/>.
*/
#![cfg_attr(not(feature = "std"), no_std)]
#[cfg(feature = "PIDController")]
pub struct PIDController {
    setpoint: f32,
    kp: f32,
    ki: f32,
    kd: f32,
    last_update_time: Option<f32>,
    prev_error: Option<f32>,
    int_error: f32,
}
#[cfg(feature = "PIDController")]
impl PIDController {
    pub fn new(setpoint: f32, kp: f32, ki: f32, kd: f32) -> PIDController {
        PIDController {
            setpoint: setpoint,
            kp: kp,
            ki: ki,
            kd: kd,
            last_update_time: None,
            prev_error: None,
            int_error: 0.0,
        }
    }
    #[must_use]
    pub fn update(&mut self, time: f32, process: f32) -> f32 {
        let error = self.setpoint - process;
        let delta_time = match self.last_update_time {
            None => 0.0,
            Some(x) => time - x,
        };
        let drv_error = match self.prev_error {
            None => 0.0,
            Some(x) => (error - x) / delta_time,
        };
        self.int_error += match self.prev_error {
            Some(x) => delta_time * (x + error) / 2.0,
            None => 0.0,
        };
        self.last_update_time = Some(time);
        self.prev_error = Some(error);
        self.kp * error + self.ki * self.int_error + self.kd * drv_error
    }
}
#[cfg(feature = "PIDControllerShift")]
pub struct PIDControllerShift {
    setpoint: f32,
    kp: f32,
    ki: f32,
    kd: f32,
    last_update_time: Option<f32>,
    prev_error: Option<f32>,
    int_error: f32,
    shifts: Vec<f32>,
}
#[cfg(feature = "PIDControllerShift")]
impl PIDControllerShift {
    pub fn new(setpoint: f32, kp: f32, ki: f32, kd: f32, shift: u8) -> PIDControllerShift {
        let mut shifts = Vec::new();
        for _ in 0..shift + 1 {
            shifts.push(0.0);
        }
        PIDControllerShift {
            setpoint: setpoint,
            kp: kp,
            ki: ki,
            kd: kd,
            last_update_time: None,
            prev_error: None,
            int_error: 0.0,
            shifts: shifts,
        }
    }
    #[must_use]
    pub fn update(&mut self, time: f32, process: f32) -> f32 {
        let error = self.setpoint - process;
        let delta_time = match self.last_update_time {
            None => 0.0,
            Some(x) => time - x,
        };
        let drv_error = match self.prev_error {
            None => 0.0,
            Some(x) => (error - x) / delta_time,
        };
        self.int_error += match self.prev_error {
            Some(x) => delta_time * (x + error) / 2.0,
            None => 0.0,
        };
        self.last_update_time = Some(time);
        self.prev_error = Some(error);
        let control = self.kp * error + self.ki * self.int_error + self.kd * drv_error;
        let mut new_shifts = vec![control];
        for i in 1..self.shifts.len() {
            let prev_int = self.shifts[i];
            new_shifts.push(prev_int + delta_time * (self.shifts[i - 1] + new_shifts[i - 1]) / 2.0);
        }
        self.shifts = new_shifts;
        self.shifts[self.shifts.len() - 1]
    }
}
#[cfg(feature = "State")]
pub struct State {
    pub position: f32,
    pub velocity: f32,
    pub acceleration: f32,
}
#[cfg(feature = "State")]
impl State {
    pub fn new(position: f32, velocity: f32, acceleration: f32) -> State {
        State {
            position: position,
            velocity: velocity,
            acceleration: acceleration,
        }
    }
    pub fn update(&mut self, delta_time: f32) {
        let new_velocity = self.velocity + delta_time * self.acceleration;
        let new_position = self.position + delta_time * (self.velocity + new_velocity) / 2.0;
        self.position = new_position;
        self.velocity = new_velocity;
    }
    pub fn set_constant_acceleration(&mut self, acceleration: f32) {
        self.acceleration = acceleration;
    }
    pub fn set_constant_velocity(&mut self, velocity: f32) {
        self.acceleration = 0.0;
        self.velocity = velocity;
    }
    pub fn set_constant_position(&mut self, position: f32) {
        self.acceleration = 0.0;
        self.velocity = 0.0;
        self.position = position;
    }
}
/*If you are using a position-based encoder, ensure that it sums full rotations instead of
resetting to zero.*/
#[cfg(feature = "Encoder")]
pub struct Encoder {
    pub state: State,
    pub time: f32,
}
#[cfg(feature = "Encoder")]
impl Encoder {
    pub fn new(state: State, time: f32) -> Encoder {
        Encoder {
            state: state,
            time: time,
        }
    }
    pub fn update_acceleration(&mut self, time: f32, acceleration: f32) {
        let delta_time = time - self.time;
        let velocity =
            self.state.velocity + delta_time * (self.state.acceleration + acceleration) / 2.0;
        let position = self.state.position + delta_time * (self.state.velocity + velocity) / 2.0;
        self.state = State::new(position, velocity, acceleration);
        self.time = time;
    }
    pub fn update_velocity(&mut self, time: f32, velocity: f32) {
        let delta_time = time - self.time;
        let acceleration = (velocity - self.state.velocity) / delta_time;
        let position = self.state.position + delta_time * (self.state.velocity + velocity) / 2.0;
        self.state = State::new(position, velocity, acceleration);
        self.time = time;
    }
    pub fn update_position(&mut self, time: f32, position: f32) {
        let delta_time = time - self.time;
        let velocity = (position - self.state.position) / delta_time;
        let acceleration = (velocity - self.state.velocity) / delta_time;
        self.state = State::new(position, velocity, acceleration);
        self.time = time;
    }
}
#[cfg(feature = "MotorMode")]
#[derive(Debug, PartialEq)]
pub enum MotorMode {
    POSITION,
    VELOCITY,
    ACCELERATION,
}
#[cfg(feature = "Motor")]
pub struct Motor {
    pub encoder: Encoder,
    pid: PIDControllerShift,
    mode: MotorMode,
}
#[cfg(feature = "Motor")]
impl Motor {
    pub fn new(state: State, time: f32, mode: MotorMode, setpoint: f32) -> Motor {
        Motor {
            encoder: Encoder::new(state, time),
            pid: PIDControllerShift::new(
                setpoint,
                1.0,
                0.01,
                0.1,
                match mode {
                    MotorMode::POSITION => 0,
                    MotorMode::VELOCITY => 1,
                    MotorMode::ACCELERATION => 2,
                },
            ),
            mode: mode,
        }
    }
    pub fn set_constant(&mut self, mode: MotorMode, setpoint: f32) {
        self.mode = mode;
        self.pid = PIDControllerShift::new(
            setpoint,
            1.0,
            0.01,
            0.1,
            match self.mode {
                MotorMode::POSITION => 0,
                MotorMode::VELOCITY => 1,
                MotorMode::ACCELERATION => 2,
            },
        );
    }
    /*The recommended way of doing this is
    time = get_time();
    velocity = get_velocity();
    motor.encoder.update_velocity(time, velocity);
    run_motor_at_voltage(motor.update(time));
    (API will differ.)*/
    /*The reason the encoder is not updated with the motor update method
    is to allow for encoders reporting different metrics, as there are both
    velocity- and position-based encoders.*/
    #[must_use]
    pub fn update(&mut self, time: f32) -> f32 {
        self.pid.update(
            time,
            match &self.mode {
                MotorMode::POSITION => self.encoder.state.position,
                MotorMode::VELOCITY => self.encoder.state.velocity,
                MotorMode::ACCELERATION => self.encoder.state.acceleration,
            },
        )
    }
}
#[cfg(feature = "MotionProfile")]
pub struct MotionProfile {
    t1: f32,
    t2: f32,
    t3: f32,
    max_vel: f32,
    max_acc: f32,
}
#[cfg(feature = "MotionProfile")]
impl MotionProfile {
    pub fn new(start_state: State, end_state: State, max_vel: f32, max_acc: f32) -> MotionProfile {
        let sign = if end_state.position < start_state.position {
            -1.0
        } else {
            1.0
        };
        let max_vel = max_vel.abs() * sign;
        let max_acc = max_acc.abs() * sign;
        let d_t1_vel = max_vel - start_state.velocity;
        let t1 = d_t1_vel / max_acc;
        assert!(t1 >= 0.0);
        let d_t1_pos = (start_state.velocity + max_vel) / 2.0 * t1;
        let d_t3_vel = end_state.velocity - max_vel;
        let d_t3 = d_t3_vel / -max_acc;
        assert!(d_t3 >= 0.0);
        let d_t3_pos = (max_vel + end_state.velocity) / 2.0 * d_t3;
        let d_t2_pos = (end_state.position - start_state.position) - (d_t1_pos + d_t3_pos);
        let d_t2 = d_t2_pos / max_vel;
        assert!(d_t2 >= 0.0);
        let t2 = t1 + d_t2;
        let t3 = t2 + d_t3;
        MotionProfile {
            t1: t1,
            t2: t2,
            t3: t3,
            max_vel: max_vel,
            max_acc: max_acc,
        }
    }
    #[cfg(feature = "MotorMode")]
    pub fn get_mode(&self, t: f32) -> Result<MotorMode, &'static str> {
        if t < 0.0 {
            return Err("time invalid");
        } else if t < self.t1 {
            return Ok(MotorMode::ACCELERATION);
        } else if t < self.t2 {
            return Ok(MotorMode::VELOCITY);
        } else if t < self.t3 {
            return Ok(MotorMode::ACCELERATION);
        } else {
            return Err("time invalid");
        }
    }
}
#[cfg(feature = "Task")]
use core::cell::RefCell;
#[cfg(feature = "Task")]
use std::rc::Rc;
#[cfg(feature = "Task")]
pub struct TaskData {
    subtasks: RefCell<Vec<Rc<dyn Task>>>,
    subtask: usize,
    pub stopped: bool,
    pub terminated: bool,
}
#[cfg(feature = "Task")]
impl TaskData {
    pub fn new(subtasks: RefCell<Vec<Rc<dyn Task>>>) -> TaskData {
        TaskData {
            subtasks: subtasks,
            subtask: 0,
            stopped: false,
            terminated: false,
        }
    }
    pub fn new_empty() -> TaskData {
        TaskData {
            subtasks: RefCell::new(vec![]),
            subtask: 0,
            stopped: false,
            terminated: false,
        }
    }
}
#[cfg(feature = "Task")]
pub trait Task {
    fn get_task_data(&self) -> &TaskData;
    fn get_task_data_mut(&mut self) -> &mut TaskData;
    fn cycle(&mut self);
    fn tick(&mut self) {
        let task_data = self.get_task_data_mut();
        if task_data.terminated || task_data.stopped {
            return;
        }
        if task_data.subtasks.borrow().len() == 0 {
            task_data.subtask = 0;
        } else {
            task_data.subtask += 1;
            task_data.subtask %= task_data.subtasks.borrow().len() + 1;
        }
        if task_data.subtask == 0 {
            self.cycle();
        } else {
            let mut binding = task_data.subtasks.borrow_mut();
            let subtask = Rc::get_mut(&mut binding[task_data.subtask - 1]).unwrap();
            subtask.tick();
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[cfg(feature = "Motor")]
    fn motor_new() {
        let motor = Motor::new(State::new(1.0, 2.0, 3.0), 4.0, MotorMode::ACCELERATION, 3.0);
        assert_eq!(motor.encoder.state.position, 1.0);
        assert_eq!(motor.encoder.state.velocity, 2.0);
        assert_eq!(motor.encoder.state.acceleration, 3.0);
        assert_eq!(motor.encoder.time, 4.0);
        assert_eq!(motor.pid.setpoint, 3.0);
        assert_eq!(motor.pid.kp, 1.0);
        assert_eq!(motor.pid.ki, 0.01);
        assert_eq!(motor.pid.kd, 0.1);
        assert_eq!(motor.pid.shifts.len(), 3);
    }
    #[test]
    #[cfg(feature = "Motor")]
    fn motor_set_constant() {
        let mut motor = Motor::new(State::new(1.0, 2.0, 3.0), 4.0, MotorMode::ACCELERATION, 3.0);
        motor.set_constant(MotorMode::VELOCITY, 5.0);
        assert_eq!(motor.pid.shifts.len(), 2);
        assert_eq!(motor.pid.setpoint, 5.0);
    }
    #[test]
    #[cfg(feature = "PIDController")]
    fn pid_new() {
        let pid = PIDController::new(5.0, 1.0, 0.01, 0.1);
        assert_eq!(pid.setpoint, 5.0);
        assert_eq!(pid.kp, 1.0);
        assert_eq!(pid.ki, 0.01);
        assert_eq!(pid.kd, 0.1);
        assert_eq!(pid.last_update_time, None);
        assert_eq!(pid.prev_error, None);
        assert_eq!(pid.int_error, 0.0);
    }
    #[test]
    #[cfg(feature = "PIDController")]
    fn pid_initial_update() {
        let mut pid = PIDController::new(5.0, 1.0, 0.01, 0.1);
        let new_control = pid.update(1.0, 0.0);
        assert_eq!(new_control, 5.0);
        assert_eq!(pid.last_update_time, Some(1.0));
        assert_eq!(pid.prev_error, Some(5.0));
        assert_eq!(pid.int_error, 0.0);
    }
    #[test]
    #[cfg(feature = "PIDController")]
    fn pid_subsequent_update() {
        let mut pid = PIDController::new(5.0, 1.0, 0.01, 0.1);
        let _ = pid.update(1.0, 0.0);
        let new_control = pid.update(3.0, 1.0);
        assert_eq!(new_control, 4.04);
        assert_eq!(pid.int_error, 9.0);
    }
    #[test]
    #[cfg(feature = "PIDControllerShift")]
    fn pidshift_no_shift() {
        let mut pid = PIDControllerShift::new(5.0, 1.0, 0.01, 0.1, 0);
        let _ = pid.update(1.0, 0.0);
        let new_control = pid.update(3.0, 1.0);
        assert_eq!(new_control, 4.04);
        assert_eq!(pid.shifts, vec![4.04]);
    }
    #[test]
    #[cfg(feature = "Task")]
    fn task_data_new() {
        let task_data = TaskData::new(RefCell::new(vec![]));
        assert_eq!(task_data.subtask, 0usize);
        assert_eq!(task_data.terminated, false);
        assert_eq!(task_data.stopped, false);
    }
    #[test]
    #[cfg(feature = "Task")]
    fn task_data_new_empty() {
        let task_data = TaskData::new_empty();
        assert_eq!(task_data.subtask, 0usize);
        assert_eq!(task_data.terminated, false);
        assert_eq!(task_data.stopped, false);
    }
    #[test]
    #[cfg(feature = "Task")]
    fn task_implement() {
        struct MyTask {
            task_data: TaskData,
        }
        impl MyTask {
            fn new() -> MyTask {
                MyTask {
                    task_data: TaskData::new_empty(),
                }
            }
        }
        impl Task for MyTask {
            fn get_task_data(&self) -> &TaskData {
                &self.task_data
            }
            fn get_task_data_mut(&mut self) -> &mut TaskData {
                &mut self.task_data
            }
            fn cycle(&mut self) {}
        }
        let my_task = MyTask::new();
        assert_eq!(my_task.task_data.subtask, 0usize);
        assert_eq!(my_task.task_data.terminated, false);
        assert_eq!(my_task.task_data.stopped, false);
    }
    #[test]
    #[cfg(feature = "Task")]
    fn task_subtask() {
        struct Foo {
            task_data: TaskData,
        }
        impl Foo {
            fn new() -> Foo {
                Foo {
                    task_data: TaskData::new(RefCell::new(vec![Rc::new(Bar::new())])),
                }
            }
        }
        impl Task for Foo {
            fn get_task_data(&self) -> &TaskData {
                &self.task_data
            }
            fn get_task_data_mut(&mut self) -> &mut TaskData {
                &mut self.task_data
            }
            fn cycle(&mut self) {}
        }
        struct Bar {
            task_data: TaskData,
        }
        impl Bar {
            fn new() -> Bar {
                Bar {
                    task_data: TaskData::new_empty(),
                }
            }
        }
        impl Task for Bar {
            fn get_task_data(&self) -> &TaskData {
                &self.task_data
            }
            fn get_task_data_mut(&mut self) -> &mut TaskData {
                &mut self.task_data
            }
            fn cycle(&mut self) {}
        }
        let foo = Foo::new();
        let mut binding = foo.task_data.subtasks.borrow_mut();
        let bar = Rc::get_mut(&mut binding[0]).unwrap();
        let bar_data = bar.get_task_data();
        assert_eq!(bar_data.subtask, 0usize);
        assert_eq!(bar_data.terminated, false);
        assert_eq!(bar_data.stopped, false);
        let foo_data = foo.get_task_data();
        assert_eq!(foo_data.subtask, 0usize);
        assert_eq!(foo_data.terminated, false);
        assert_eq!(foo_data.stopped, false);
    }
    #[test]
    #[cfg(feature = "MotionProfile")]
    fn motion_profile_new_1() {
        let motion_profile = MotionProfile::new(
            State::new(0.0, 0.0, 0.0),
            State::new(3.0, 0.0, 0.0),
            1.0,
            1.0,
        );
        assert_eq!(motion_profile.t1, 1.0);
        assert_eq!(motion_profile.t2, 3.0);
        assert_eq!(motion_profile.t3, 4.0);
        assert_eq!(motion_profile.max_vel, 1.0);
        assert_eq!(motion_profile.max_acc, 1.0);
    }
    #[test]
    #[cfg(feature = "MotionProfile")]
    fn motion_profile_new_2() {
        let motion_profile = MotionProfile::new(
            State::new(1.0, 0.0, 0.0),
            State::new(3.0, 0.0, 0.0),
            1.0,
            1.0,
        );
        assert_eq!(motion_profile.t1, 1.0);
        assert_eq!(motion_profile.t2, 2.0);
        assert_eq!(motion_profile.t3, 3.0);
        assert_eq!(motion_profile.max_vel, 1.0);
        assert_eq!(motion_profile.max_acc, 1.0);
    }
    #[test]
    #[cfg(feature = "MotionProfile")]
    fn motion_profile_new_3() {
        let motion_profile = MotionProfile::new(
            State::new(0.0, 1.0, 0.0),
            State::new(3.0, 0.0, 0.0),
            1.0,
            1.0,
        );
        assert_eq!(motion_profile.t1, 0.0);
        assert_eq!(motion_profile.t2, 2.5);
        assert_eq!(motion_profile.t3, 3.5);
        assert_eq!(motion_profile.max_vel, 1.0);
        assert_eq!(motion_profile.max_acc, 1.0);
    }
    #[test]
    #[cfg(feature = "MotionProfile")]
    fn motion_profile_new_4() {
        let motion_profile = MotionProfile::new(
            State::new(0.0, 0.0, 1.0),
            State::new(3.0, 0.0, 0.0),
            1.0,
            1.0,
        );
        assert_eq!(motion_profile.t1, 1.0);
        assert_eq!(motion_profile.t2, 3.0);
        assert_eq!(motion_profile.t3, 4.0);
        assert_eq!(motion_profile.max_vel, 1.0);
        assert_eq!(motion_profile.max_acc, 1.0);
    }
    #[test]
    #[cfg(feature = "MotionProfile")]
    fn motion_profile_new_5() {
        let motion_profile = MotionProfile::new(
            State::new(0.0, 0.0, 0.0),
            State::new(6.0, 0.0, 0.0),
            2.0,
            1.0,
        );
        assert_eq!(motion_profile.t1, 2.0);
        assert_eq!(motion_profile.t2, 3.0);
        assert_eq!(motion_profile.t3, 5.0);
        assert_eq!(motion_profile.max_vel, 2.0);
        assert_eq!(motion_profile.max_acc, 1.0);
    }
    #[test]
    #[cfg(feature = "MotionProfile")]
    fn motion_profile_new_6() {
        let motion_profile = MotionProfile::new(
            State::new(0.0, 0.0, 0.0),
            State::new(3.0, 0.0, 0.0),
            1.0,
            2.0,
        );
        assert_eq!(motion_profile.t1, 0.5);
        assert_eq!(motion_profile.t2, 3.0);
        assert_eq!(motion_profile.t3, 3.5);
        assert_eq!(motion_profile.max_vel, 1.0);
        assert_eq!(motion_profile.max_acc, 2.0);
    }
    #[test]
    #[cfg(feature = "MotorMode")]
    #[cfg(feature = "MotionProfile")]
    fn motion_profile_new_7() {
        let motion_profile = MotionProfile::new(
            State::new(0.0, 0.0, 0.0),
            State::new(-3.0, 0.0, 0.0),
            1.0,
            1.0,
        );
        assert_eq!(motion_profile.t1, 1.0);
        assert_eq!(motion_profile.t2, 3.0);
        assert_eq!(motion_profile.t3, 4.0);
        assert_eq!(motion_profile.max_vel, -1.0);
        assert_eq!(motion_profile.max_acc, -1.0);
    }
}
