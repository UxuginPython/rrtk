// SPDX-License-Identifier: LGPL-3.0-only
/*
Copyright 2024 UxuginPython on GitHub

     This file is part of Rust Robotics ToolKit.

    Rust Robotics ToolKit is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, version 3.

    Rust Robotics ToolKit is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Lesser General Public License for more details.

    You should have received a copy of the GNU Lesser General Public License along with Rust Robotics ToolKit. If not, see <https://www.gnu.org/licenses/>.
*/
//!Streams performing control theory operations.
use crate::streams::*;
//This does store the timestamp twice, once in prev_error and once in output. Processor performance
//and readability would suggest doing it this way, but 8 bytes could technically be saved here if
//needed in the future. The difference is extremely minimal.
///A PID controller for use with the stream system.
pub struct PIDControllerStream<E: Copy + Debug> {
    input: InputGetter<f32, E>,
    setpoint: f32,
    kvals: PIDKValues,
    prev_error: Option<Datum<f32>>,
    int_error: f32,
    output: Output<f32, E>,
}
impl<E: Copy + Debug> PIDControllerStream<E> {
    ///Constructor for `PIDControllerStream`.
    pub fn new(input: InputGetter<f32, E>, setpoint: f32, kvals: PIDKValues) -> Self {
        Self {
            input: input,
            setpoint: setpoint,
            kvals: kvals,
            prev_error: None,
            int_error: 0.0,
            output: Ok(None),
        }
    }
    #[inline]
    fn reset(&mut self) {
        self.prev_error = None;
        self.int_error = 0.0;
        self.output = Ok(None);
    }
}
impl<E: Copy + Debug> Getter<f32, E> for PIDControllerStream<E> {
    fn get(&self) -> Output<f32, E> {
        self.output.clone()
    }
}
impl<E: Copy + Debug> Updatable<E> for PIDControllerStream<E> {
    fn update(&mut self) -> NothingOrError<E> {
        let process = self.input.borrow().get();
        let process = match process {
            Ok(Some(value)) => value,
            Ok(None) => {
                self.reset();
                return Ok(());
            }
            Err(error) => {
                self.reset();
                self.output = Err(error);
                return Err(error);
            }
        };
        let error = self.setpoint - process.value;
        let [int_error_addend, drv_error] = match &self.prev_error {
            Some(prev_error) => {
                let delta_time = (process.time - prev_error.time) as f32;
                let drv_error = (error - prev_error.value) / delta_time;
                //Trapezoidal integral approximation is more precise than rectangular.
                let int_error_addend = delta_time * (prev_error.value + error) / 2.0;
                [int_error_addend, drv_error]
            }
            None => {
                debug_assert_eq!(self.int_error, 0.0);
                [0.0, 0.0]
            }
        };
        self.int_error += int_error_addend;
        self.output = Ok(Some(Datum::new(
            process.time,
            self.kvals.kp * error + self.kvals.ki * self.int_error + self.kvals.kd * drv_error,
        )));
        self.prev_error = Some(Datum::new(process.time, error));
        Ok(())
    }
}
enum CommandPIDUpdateState<E: Copy + Debug> {
    Error(Error<E>),
    Ready,
    Update0 {
        timestamp: i64,
        output: f32,
        error: f32,
    },
    Update1 {
        timestamp: i64,
        output: f32,
        output_int: f32,
        error: f32,
        error_int: f32,
    },
    Operational {
        timestamp: i64,
        output: f32,
        output_int: f32,
        output_int_int: f32,
        error: f32,
        error_int: f32,
    },
}
//TODO: test this
///Almost literally three PID controllers in a trenchcoat.
pub struct CommandPID<E: Copy + Debug> {
    input: InputGetter<State, E>,
    command: Command,
    kvals: PositionDerivativeDependentPIDKValues,
    update_state: CommandPIDUpdateState<E>,
}
impl<E: Copy + Debug> CommandPID<E> {
    pub fn new(
        input: InputGetter<State, E>,
        command: Command,
        kvalues: PositionDerivativeDependentPIDKValues,
    ) -> Self {
        Self {
            input: input,
            command: command,
            kvals: kvalues,
            update_state: CommandPIDUpdateState::Ready,
        }
    }
    #[inline]
    pub fn reset(&mut self) {
        self.update_state = CommandPIDUpdateState::Ready;
    }
    #[inline]
    pub fn set_command(&mut self, command: Command) {
        self.reset();
        self.command = command;
    }
}
impl<E: Copy + Debug> Getter<f32, E> for CommandPID<E> {
    fn get(&self) -> Output<f32, E> {
        match self.update_state {
            CommandPIDUpdateState::Error(error) => Err(error),
            CommandPIDUpdateState::Ready => Ok(None),
            CommandPIDUpdateState::Update0 {
                timestamp, output, ..
            } => match self.command.position_derivative {
                PositionDerivative::Position => Ok(Some(Datum::new(timestamp, output))),
                _ => Ok(None),
            },
            CommandPIDUpdateState::Update1 {
                timestamp,
                output,
                output_int,
                ..
            } => match self.command.position_derivative {
                PositionDerivative::Position => Ok(Some(Datum::new(timestamp, output))),
                PositionDerivative::Velocity => Ok(Some(Datum::new(timestamp, output_int))),
                _ => Ok(None),
            },
            CommandPIDUpdateState::Operational {
                timestamp,
                output,
                output_int,
                output_int_int,
                ..
            } => match self.command.position_derivative {
                PositionDerivative::Position => Ok(Some(Datum::new(timestamp, output))),
                PositionDerivative::Velocity => Ok(Some(Datum::new(timestamp, output_int))),
                PositionDerivative::Acceleration => Ok(Some(Datum::new(timestamp, output_int_int))),
            },
        }
    }
}
impl<E: Copy + Debug> Updatable<E> for CommandPID<E> {
    fn update(&mut self) -> NothingOrError<E> {
        let raw_get = self.input.borrow().get();
        let new_datum_state = match raw_get {
            Ok(Some(value)) => value,
            Ok(None) => {
                self.reset();
                return Ok(());
            }
            Err(error) => {
                self.update_state = CommandPIDUpdateState::Error(error);
                return Err(error);
            }
        };
        let new_error = self.command.value
            - new_datum_state
                .value
                .get_value(self.command.position_derivative);
        //TODO: Figure out how to do this with less repeated code
        match &self.update_state {
            CommandPIDUpdateState::Error(_) | CommandPIDUpdateState::Ready => {
                self.update_state = CommandPIDUpdateState::Update0 {
                    timestamp: new_datum_state.time,
                    output: self.kvals.evaluate(
                        self.command.position_derivative,
                        new_error,
                        0.0,
                        0.0,
                    ),
                    error: new_error,
                }
            }
            CommandPIDUpdateState::Update0 {
                timestamp,
                output,
                error,
            } => {
                let delta_time = (new_datum_state.time - timestamp) as f32;
                let error_drv = (new_error - error) / delta_time;
                let error_int = (error + new_error) / 2.0 * delta_time;
                let new_output = self.kvals.evaluate(
                    self.command.position_derivative,
                    new_error,
                    error_int,
                    error_drv,
                );
                let output_int = (output + new_output) / 2.0 * delta_time;
                self.update_state = CommandPIDUpdateState::Update1 {
                    timestamp: new_datum_state.time,
                    output: new_output,
                    output_int: output_int,
                    error: new_error,
                    error_int: error_int,
                }
            }
            CommandPIDUpdateState::Update1 {
                timestamp,
                output,
                output_int,
                error,
                error_int,
            } => {
                let delta_time = (new_datum_state.time - timestamp) as f32;
                let error_drv = (new_error - error) / delta_time;
                let new_error_int = error_int + (error + new_error) / 2.0 * delta_time;
                let new_output = self.kvals.evaluate(
                    self.command.position_derivative,
                    new_error,
                    new_error_int,
                    error_drv,
                );
                let new_output_int = output_int + (output + new_output) / 2.0 * delta_time;
                let output_int_int = (output_int + new_output_int) / 2.0 * delta_time;
                self.update_state = CommandPIDUpdateState::Operational {
                    timestamp: new_datum_state.time,
                    output: new_output,
                    output_int: new_output_int,
                    output_int_int: output_int_int,
                    error: new_error,
                    error_int: new_error_int,
                }
            }
            CommandPIDUpdateState::Operational {
                timestamp,
                output,
                output_int,
                output_int_int,
                error,
                error_int,
            } => {
                let delta_time = (new_datum_state.time - timestamp) as f32;
                let error_drv = (new_error - error) / delta_time;
                let new_error_int = error_int + (error + new_error) / 2.0 * delta_time;
                let new_output = self.kvals.evaluate(
                    self.command.position_derivative,
                    new_error,
                    new_error_int,
                    error_drv,
                );
                let new_output_int = output_int + (output + new_output) / 2.0 * delta_time;
                let new_output_int_int = output_int_int + (output_int + new_output_int) / 2.0 * delta_time;
                self.update_state = CommandPIDUpdateState::Operational {
                    timestamp: new_datum_state.time,
                    output: new_output,
                    output_int: new_output_int,
                    output_int_int: new_output_int_int,
                    error: new_error,
                    error_int: new_error_int,
                }
            }
        }
        Ok(())
    }
    /*fn update(&mut self) -> NothingOrError<E> {
        let raw_get = self.input.borrow().get();
        let new_state = match raw_get {
            Ok(Some(value)) => value,
            Ok(None) => {
                self.reset();
                return Ok(());
            }
            Err(error) => {
                self.reset();
                self.output = Err(error);
                return Ok(());
            }
        };
        //The fact that this could be any of position, velocity, and jerk is not great.
        let error = self.command.value - new_state.value.get_value(self.command.position_derivative);
        match self.prev_error {
            Some(prev_error) => {
                let delta_time = (new_state.time - prev_error.time) as f32;
                self.int_error += (prev_error.value + error) / 2.0 * delta_time;
                let drv_error = (error - prev_error.value) / delta_time;
                let output = self.kvals.evaluate(
                    self.command.position_derivative,
                    error,
                    self.int_error,
                    drv_error,
                );
                let prev_out = self.output.expect("If self.prev_error is Some, then self.output should be Ok(Some).").unwrap();
                todo!();
            }
            None => {
                let output = error * self.kvals.get_k_values(self.command.position_derivative).kp;
            }
        }
        self.prev_error = Some(Datum::new(new_state.time, error));
        todo!();
    }*/
} //lol
  /*fn make_none_zero(it: &mut Option<f32>) {
      match it {
          Some(_) => (),
          None => *it = Some(0.0),
      }
  }
  #[test]
  fn make_none_zero_test() {
      let mut x = None;
      make_none_zero(&mut x);
      assert_eq!(x, Some(0.0));
      let mut y = Some(4.0);
      make_none_zero(&mut y);
      assert_eq!(y, Some(4.0));
  }*/
///An Exponentially Weighted Moving Average stream for use with the stream system. See <https://www.itl.nist.gov/div898/handbook/pmc/section3/pmc324.htm> for more information.
pub struct EWMAStream<E: Copy + Debug> {
    input: InputGetter<f32, E>,
    //As data may not come in at regular intervals as is assumed by a standard EWMA, this value
    //will be multiplied by delta time before being used.
    smoothing_constant: f32,
    value: Output<f32, E>,
    update_time: Option<i64>,
}
impl<E: Copy + Debug> EWMAStream<E> {
    ///Constructor for `EWMAStream`.
    pub fn new(input: InputGetter<f32, E>, smoothing_constant: f32) -> Self {
        Self {
            input: input,
            smoothing_constant: smoothing_constant,
            value: Ok(None),
            update_time: None,
        }
    }
}
impl<E: Copy + Debug> Getter<f32, E> for EWMAStream<E> {
    fn get(&self) -> Output<f32, E> {
        self.value.clone()
    }
}
impl<E: Copy + Debug> Updatable<E> for EWMAStream<E> {
    fn update(&mut self) -> NothingOrError<E> {
        let output = self.input.borrow().get();
        match output {
            Err(error) => {
                self.value = Err(error);
                self.update_time = None;
                return Err(error);
            }
            Ok(None) => {
                match self.value {
                    Err(_) => {
                        self.value = Ok(None);
                        self.update_time = None;
                    }
                    Ok(_) => {}
                }
                return Ok(());
            }
            Ok(Some(_)) => {}
        }
        let output = output.unwrap().unwrap();
        match self.value {
            Ok(Some(_)) => {}
            _ => {
                self.value = Ok(Some(output.clone()));
                self.update_time = Some(output.time);
            }
        }
        let prev_value = self.value.as_ref().unwrap().as_ref().unwrap();
        let prev_time = self
            .update_time
            .expect("update_time must be Some if value is");
        let delta_time = (output.time - prev_time) as f32;
        let value = if delta_time * self.smoothing_constant < 1.0 {
            let value = prev_value.value;
            let value = value - (delta_time * self.smoothing_constant) * value;
            let value = value + (delta_time * self.smoothing_constant) * output.value;
            value
        } else {
            output.value
        };
        self.value = Ok(Some(Datum::new(output.time, value)));
        self.update_time = Some(output.time);
        Ok(())
    }
}
///A moving average stream for use with the stream system.
pub struct MovingAverageStream<E: Copy + Debug> {
    input: InputGetter<f32, E>,
    window: i64,
    value: Output<f32, E>,
    input_values: VecDeque<Datum<f32>>,
}
impl<E: Copy + Debug> MovingAverageStream<E> {
    ///Constructor for `MovingAverageStream`.
    pub fn new(input: InputGetter<f32, E>, window: i64) -> Self {
        Self {
            input: input,
            window: window,
            value: Ok(None),
            input_values: VecDeque::new(),
        }
    }
}
impl<E: Copy + Debug> Getter<f32, E> for MovingAverageStream<E> {
    fn get(&self) -> Output<f32, E> {
        self.value.clone()
    }
}
impl<E: Copy + Debug> Updatable<E> for MovingAverageStream<E> {
    fn update(&mut self) -> NothingOrError<E> {
        let output = self.input.borrow().get();
        let output = match output {
            Ok(Some(thing)) => thing,
            Ok(None) => {
                match self.value {
                    Ok(_) => {}
                    Err(_) => {
                        //We got an Ok(None) from input, so there's not a problem anymore, but we
                        //still don't have a value. Set it to Ok(None) and leave input_values
                        //empty.
                        self.value = Ok(None);
                    }
                }
                return Ok(());
            }
            Err(error) => {
                self.value = Err(error);
                self.input_values.clear();
                return Err(error);
            }
        };
        self.input_values.push_back(output.clone());
        if self.input_values.len() == 0 {
            self.value = Ok(Some(output));
            return Ok(());
        }
        while self.input_values[0].time <= output.time - self.window {
            self.input_values.pop_front();
        }
        let mut end_times = Vec::new();
        for i in &self.input_values {
            end_times.push(i.time);
        }
        let mut start_times = VecDeque::from(end_times.clone());
        start_times.pop_back();
        start_times.push_front(output.time - self.window);
        let mut weights = Vec::with_capacity(self.input_values.len());
        for i in 0..self.input_values.len() {
            weights.push((end_times[i] - start_times[i]) as f32);
        }
        let mut value = 0.0;
        for i in 0..self.input_values.len() {
            value += self.input_values[i].value * weights[i] as f32;
        }
        value /= self.window as f32;
        self.value = Ok(Some(Datum::new(output.time, value)));
        Ok(())
    }
}
