use rrtk::*;
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
#[cfg(feature = "PIDControllerShift")]
fn pidshift_shift() {
    let mut pid = PIDControllerShift::new(5.0, 1.0, 0.01, 0.1, 1);
    let _ = pid.update(1.0, 0.0);
    let new_control = pid.update(3.0, 1.0);
    assert_eq!(new_control, 9.04);
}
#[test]
#[cfg(feature = "State")]
fn state_new() {
    let state = State::new(1.0, 2.0, 3.0);
    assert_eq!(state.position, 1.0);
    assert_eq!(state.velocity, 2.0);
    assert_eq!(state.acceleration, 3.0);
}
#[test]
#[cfg(feature = "State")]
fn state_update() {
    let mut state = State::new(1.0, 2.0, 3.0);
    state.update(4.0);
    assert_eq!(state.position, 33.0);
    assert_eq!(state.velocity, 14.0);
    assert_eq!(state.acceleration, 3.0);
}
#[test]
#[cfg(feature = "State")]
fn state_acceleration() {
    let mut state = State::new(1.0, 2.0, 3.0);
    state.set_constant_acceleration(4.0);
    assert_eq!(state.acceleration, 4.0);
}
#[test]
#[cfg(feature = "State")]
fn state_velocity() {
    let mut state = State::new(1.0, 2.0, 3.0);
    state.set_constant_velocity(4.0);
    assert_eq!(state.velocity, 4.0);
    assert_eq!(state.acceleration, 0.0);
}
#[test]
#[cfg(feature = "State")]
fn state_position() {
    let mut state = State::new(1.0, 2.0, 3.0);
    state.set_constant_position(4.0);
    assert_eq!(state.position, 4.0);
    assert_eq!(state.velocity, 0.0);
    assert_eq!(state.acceleration, 0.0);
}
#[test]
#[cfg(feature = "Encoder")]
fn encoder_new() {
    let encoder = Encoder::new(State::new(1.0, 2.0, 3.0), 4.0);
    assert_eq!(encoder.state.position, 1.0);
    assert_eq!(encoder.state.velocity, 2.0);
    assert_eq!(encoder.state.acceleration, 3.0);
    assert_eq!(encoder.time, 4.0);
}
#[test]
#[cfg(feature = "Encoder")]
fn encoder_update_acceleration() {
    let mut encoder = Encoder::new(State::new(1.0, 2.0, 3.0), 4.0);
    encoder.update_acceleration(6.0, 5.0);
    assert_eq!(encoder.state.position, 13.0);
    assert_eq!(encoder.state.velocity, 10.0);
    assert_eq!(encoder.state.acceleration, 5.0);
}
#[test]
#[cfg(feature = "Encoder")]
fn encoder_update_velocity() {
    let mut encoder = Encoder::new(State::new(1.0, 2.0, 3.0), 4.0);
    encoder.update_velocity(6.0, 5.0);
    assert_eq!(encoder.state.position, 8.0);
    assert_eq!(encoder.state.velocity, 5.0);
    assert_eq!(encoder.state.acceleration, 1.5);
}
#[test]
#[cfg(feature = "Encoder")]
fn encoder_update_position() {
    let mut encoder = Encoder::new(State::new(1.0, 2.0, 3.0), 4.0);
    encoder.update_position(6.0, 5.0);
    assert_eq!(encoder.state.position, 5.0);
    assert_eq!(encoder.state.velocity, 2.0);
    assert_eq!(encoder.state.acceleration, 0.0);
}
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
#[cfg(feature = "Motor")]
fn motor_update() {
    let mut motor = Motor::new(State::new(1.0, 2.0, 3.0), 4.0, MotorMode::ACCELERATION, 3.0);
    motor.encoder.update_acceleration(6.0, 3.0);
    let update = motor.update(6.0);
    assert_eq!(update, 0.0);
    assert_eq!(motor.encoder.state.position, 11.0);
    assert_eq!(motor.encoder.state.velocity, 8.0);
    assert_eq!(motor.encoder.state.acceleration, 3.0);
}
/*#[test]
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
#[test]
#[cfg(feature = "MotorMode")]
#[cfg(feature = "MotionProfile")]
fn motion_profile_get_mode() {
    let motion_profile = MotionProfile::new(
        State::new(0.0, 0.0, 0.0),
        State::new(3.0, 0.0, 0.0),
        1.0,
        1.0,
    );
    assert_eq!(motion_profile.get_mode(0.5), Ok(MotorMode::ACCELERATION));
    assert_eq!(motion_profile.get_mode(2.5), Ok(MotorMode::VELOCITY));
    assert_eq!(motion_profile.get_mode(3.5), Ok(MotorMode::ACCELERATION));
}*/
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
