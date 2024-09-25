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
#[cfg(feature = "alloc")]
use alloc::collections::vec_deque::VecDeque;
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
#[derive(Clone, Debug, PartialEq)]
struct Update0 {
    pub time: i64,
    pub output: f32,
    pub error: f32,
    pub maybe_update_1: Option<Update1>,
}
#[derive(Clone, Debug, PartialEq)]
struct Update1 {
    pub output_int: f32,
    pub error_int: f32,
    pub output_int_int: Option<f32>,
}
///Automatically integrates the command variable of a PID controller based on the position
///derivative of a `Command`. Designed to make it easier to use a standard DC motor and an encoder
///as a de facto servo.
pub struct CommandPID<E: Copy + Debug> {
    settable_data: SettableData<Command, E>,
    input: InputGetter<State, E>,
    command: Command,
    kvals: PositionDerivativeDependentPIDKValues,
    update_state: Result<Option<Update0>, Error<E>>,
}
impl<E: Copy + Debug> CommandPID<E> {
    ///Constructor for `CommandPID`.
    pub fn new(
        input: InputGetter<State, E>,
        command: Command,
        kvalues: PositionDerivativeDependentPIDKValues,
    ) -> Self {
        Self {
            settable_data: SettableData::new(),
            input: input,
            command: command,
            kvals: kvalues,
            update_state: Ok(None),
        }
    }
    ///Clear cached data for calculating integral and derivative. After this is called, the PID
    ///controller will use the next few updates to rebuild its cache in the same way as it does
    ///during the first few updates after initialization. This is called when the command changes
    ///so as not to cause issues with the integral and derivative.
    #[inline]
    pub fn reset(&mut self) {
        self.update_state = Ok(None);
    }
}
impl<E: Copy + Debug> Settable<Command, E> for CommandPID<E> {
    fn get_settable_data_ref(&self) -> &SettableData<Command, E> {
        &self.settable_data
    }
    fn get_settable_data_mut(&mut self) -> &mut SettableData<Command, E> {
        &mut self.settable_data
    }
    fn impl_set(&mut self, command: Command) -> NothingOrError<E> {
        if command != self.command {
            self.reset();
            self.command = command;
        }
        Ok(())
    }
}
impl<E: Copy + Debug> Getter<f32, E> for CommandPID<E> {
    fn get(&self) -> Output<f32, E> {
        match &self.update_state {
            Err(error) => Err(*error),
            Ok(None) => Ok(None),
            Ok(Some(update_0)) => match self.command.position_derivative {
                PositionDerivative::Position => {
                    Ok(Some(Datum::new(update_0.time, update_0.output)))
                }
                _ => match &update_0.maybe_update_1 {
                    None => Ok(None),
                    Some(update_1) => match self.command.position_derivative {
                        PositionDerivative::Position => unimplemented!(),
                        PositionDerivative::Velocity => {
                            Ok(Some(Datum::new(update_0.time, update_1.output_int)))
                        }
                        PositionDerivative::Acceleration => match update_1.output_int_int {
                            None => Ok(None),
                            Some(output_int_int) => {
                                Ok(Some(Datum::new(update_0.time, output_int_int)))
                            }
                        },
                    },
                },
            },
        }
    }
}
impl<E: Copy + Debug> Updatable<E> for CommandPID<E> {
    fn update(&mut self) -> NothingOrError<E> {
        self.update_following_data()?;
        let raw_get = self.input.borrow().get();
        let datum_state = match raw_get {
            Ok(Some(value)) => value,
            Ok(None) => {
                self.reset();
                return Ok(());
            }
            Err(error) => {
                self.update_state = Err(error);
                return Err(error);
            }
        };
        let error = self.command.value
            - datum_state
                .value
                .get_value(self.command.position_derivative);
        match &self.update_state {
            Ok(None) | Err(_) => {
                let output = self
                    .kvals
                    .evaluate(self.command.position_derivative, error, 0.0, 0.0);
                self.update_state = Ok(Some(Update0 {
                    time: datum_state.time,
                    output: output,
                    error: error,
                    maybe_update_1: None,
                }));
            }
            Ok(Some(update_0)) => {
                let delta_time = (datum_state.time - update_0.time) as f32;
                let error_drv = (error - update_0.error) / delta_time;
                let error_int_addend = (update_0.error + error) / 2.0 * delta_time;
                match &update_0.maybe_update_1 {
                    None => {
                        let output = self.kvals.evaluate(
                            self.command.position_derivative,
                            error,
                            error_int_addend,
                            error_drv,
                        );
                        let output_int = (update_0.output + output) / 2.0 * delta_time;
                        self.update_state = Ok(Some(Update0 {
                            time: datum_state.time,
                            output: output,
                            error: error,
                            maybe_update_1: Some(Update1 {
                                output_int: output_int,
                                error_int: error_int_addend,
                                output_int_int: None,
                            }),
                        }));
                    }
                    Some(update_1) => {
                        let error_int = update_1.error_int + error_int_addend;
                        let output = self.kvals.evaluate(
                            self.command.position_derivative,
                            error,
                            error_int,
                            error_drv,
                        );
                        let output_int =
                            update_1.output_int + (update_0.output + output) / 2.0 * delta_time;
                        let output_int_int_addend =
                            (update_1.output_int + output_int) / 2.0 * delta_time;
                        match &update_1.output_int_int {
                            None => {
                                self.update_state = Ok(Some(Update0 {
                                    time: datum_state.time,
                                    output: output,
                                    error: error,
                                    maybe_update_1: Some(Update1 {
                                        output_int: output_int,
                                        error_int: error_int,
                                        output_int_int: Some(output_int_int_addend),
                                    }),
                                }));
                            }
                            Some(output_int_int) => {
                                self.update_state = Ok(Some(Update0 {
                                    time: datum_state.time,
                                    output: output,
                                    error: error,
                                    maybe_update_1: Some(Update1 {
                                        output_int: output_int,
                                        error_int: error_int,
                                        output_int_int: Some(
                                            output_int_int + output_int_int_addend,
                                        ),
                                    }),
                                }));
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
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
