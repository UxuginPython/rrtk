// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2024-2025 UxuginPython
//!Streams performing control theory operations.
use crate::streams::*;
#[cfg(feature = "alloc")]
use alloc::collections::vec_deque::VecDeque;
//This does store the timestamp twice, once in prev_error and once in output. Processor performance
//and readability would suggest doing it this way, but 8 bytes could technically be saved here if
//needed in the future. The difference is extremely minimal.
///A PID controller for use with the stream system.
pub struct PIDControllerStream<G: Getter<f32, E>, E: Copy + Debug> {
    input: G,
    setpoint: f32,
    kvals: PIDKValues,
    prev_error: Option<Datum<f32>>,
    int_error: f32,
    output: Output<f32, E>,
}
impl<G: Getter<f32, E>, E: Copy + Debug> PIDControllerStream<G, E> {
    ///Constructor for `PIDControllerStream`.
    pub const fn new(input: G, setpoint: f32, kvals: PIDKValues) -> Self {
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
impl<G: Getter<f32, E>, E: Copy + Debug> Getter<f32, E> for PIDControllerStream<G, E> {
    fn get(&self) -> Output<f32, E> {
        self.output.clone()
    }
}
impl<G: Getter<f32, E>, E: Copy + Debug> Updatable<E> for PIDControllerStream<G, E> {
    fn update(&mut self) -> NothingOrError<E> {
        let process = self.input.get();
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
                let delta_time = f32::from(Quantity::from(process.time - prev_error.time));
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
pub use command_pid::CommandPID;
mod command_pid {
    use super::*;
    #[derive(Clone, Debug, PartialEq)]
    struct Update0 {
        pub time: Time,
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
    ///derivative of a [`Command`]. Designed to make it easier to use a standard DC motor and an encoder
    ///as a de facto servo.
    pub struct CommandPID<G: Getter<State, E>, E: Copy + Debug> {
        settable_data: SettableData<Command, E>,
        input: G,
        command: Command,
        kvals: PositionDerivativeDependentPIDKValues,
        update_state: Result<Option<Update0>, Error<E>>,
    }
    impl<G: Getter<State, E>, E: Copy + Debug> CommandPID<G, E> {
        ///Constructor for `CommandPID`.
        pub const fn new(
            input: G,
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
    impl<G: Getter<State, E>, E: Copy + Debug> Settable<Command, E> for CommandPID<G, E> {
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
    impl<G: Getter<State, E>, E: Copy + Debug> Getter<f32, E> for CommandPID<G, E> {
        fn get(&self) -> Output<f32, E> {
            match &self.update_state {
                Err(error) => Err(*error),
                Ok(None) => Ok(None),
                Ok(Some(update_0)) => match self.command.into() {
                    PositionDerivative::Position => {
                        Ok(Some(Datum::new(update_0.time, update_0.output)))
                    }
                    _ => match &update_0.maybe_update_1 {
                        None => Ok(None),
                        Some(update_1) => match self.command.into() {
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
    impl<G: Getter<State, E>, E: Copy + Debug> Updatable<E> for CommandPID<G, E> {
        fn update(&mut self) -> NothingOrError<E> {
            self.update_following_data()?;
            let raw_get = self.input.get();
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
            let error = f32::from(self.command)
                - f32::from(datum_state.value.get_value(self.command.into()));
            match &self.update_state {
                Ok(None) | Err(_) => {
                    let output = self.kvals.evaluate(self.command.into(), error, 0.0, 0.0);
                    self.update_state = Ok(Some(Update0 {
                        time: datum_state.time,
                        output: output,
                        error: error,
                        maybe_update_1: None,
                    }));
                }
                Ok(Some(update_0)) => {
                    let delta_time = f32::from(Quantity::from(datum_state.time - update_0.time));
                    let error_drv = (error - update_0.error) / delta_time;
                    let error_int_addend = (update_0.error + error) / 2.0 * delta_time;
                    match &update_0.maybe_update_1 {
                        None => {
                            let output = self.kvals.evaluate(
                                self.command.into(),
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
                                self.command.into(),
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
}
///An Exponentially Weighted Moving Average stream for use with the stream system. See <https://www.itl.nist.gov/div898/handbook/pmc/section3/pmc324.htm> for more information. Because a standard EWMA requires that new data always arrive at the same interval, this implementation uses λ=1-(1-`smoothing_constant`)^Δt instead of the usual weighting factor.
#[cfg(feature = "internal_enhanced_float")]
pub struct EWMAStream<T: Clone + Add<Output = T>, G: Getter<T, E>, E: Copy + Debug> {
    input: G,
    //As data may not come in at regular intervals as is assumed by a standard EWMA, this value
    //will be multiplied by delta time before being used.
    smoothing_constant: f32,
    value: Output<T, E>,
    update_time: Option<Time>,
}
#[cfg(feature = "internal_enhanced_float")]
impl<T: Clone + Add<Output = T>, G: Getter<T, E>, E: Copy + Debug> EWMAStream<T, G, E> {
    ///Constructor for [`EWMAStream`].
    pub const fn new(input: G, smoothing_constant: f32) -> Self {
        Self {
            input: input,
            smoothing_constant: smoothing_constant,
            value: Ok(None),
            update_time: None,
        }
    }
}
#[cfg(feature = "internal_enhanced_float")]
impl<T: Clone + Add<Output = T>, G: Getter<T, E>, E: Copy + Debug> Getter<T, E>
    for EWMAStream<T, G, E>
where
    EWMAStream<T, G, E>: Updatable<E>,
{
    fn get(&self) -> Output<T, E> {
        self.value.clone()
    }
}
#[cfg(feature = "internal_enhanced_float")]
impl<T: Clone + Add<Output = T> + Mul<f32, Output = T>, G: Getter<T, E>, E: Copy + Debug>
    Updatable<E> for EWMAStream<T, G, E>
{
    fn update(&mut self) -> NothingOrError<E> {
        let output = self.input.get();
        let output = match output {
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
            Ok(Some(some)) => some,
        };
        let prev_value = match &self.value {
            Ok(Some(some)) => some.clone(),
            _ => {
                self.value = Ok(Some(output.clone()));
                self.update_time = Some(output.time);
                output.clone()
            }
        };
        let prev_time = self
            .update_time
            .expect("update_time must be Some if value is");
        let delta_time = f32::from(Quantity::from(output.time - prev_time));
        let lambda = 1.0 - powf(1.0 - self.smoothing_constant, delta_time);
        let value = prev_value.value * (1.0 - lambda) + output.value * lambda;
        self.value = Ok(Some(Datum::new(output.time, value)));
        self.update_time = Some(output.time);
        Ok(())
    }
}
#[cfg(feature = "internal_enhanced_float")]
impl<G: Getter<Quantity, E>, E: Copy + Debug> Updatable<E> for EWMAStream<Quantity, G, E> {
    fn update(&mut self) -> NothingOrError<E> {
        let output = self.input.get();
        let output = match output {
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
            Ok(Some(some)) => some,
        };
        let prev_value = match &self.value {
            Ok(Some(some)) => some.clone(),
            _ => {
                self.value = Ok(Some(output.clone()));
                self.update_time = Some(output.time);
                output.clone()
            }
        };
        let prev_time = self
            .update_time
            .expect("update_time must be Some if value is");
        let delta_time = f32::from(Quantity::from(output.time - prev_time));
        let lambda = Quantity::dimensionless(1.0 - powf(1.0 - self.smoothing_constant, delta_time));
        let value =
            prev_value.value * (Quantity::dimensionless(1.0) - lambda) + output.value * lambda;
        self.value = Ok(Some(Datum::new(output.time, value)));
        self.update_time = Some(output.time);
        Ok(())
    }
}
///A moving average stream for use with the stream system.
#[cfg(feature = "alloc")]
pub struct MovingAverageStream<T, G: Getter<T, E>, E: Copy + Debug> {
    input: G,
    window: Time,
    value: Output<T, E>,
    input_values: VecDeque<Datum<T>>,
}
#[cfg(feature = "alloc")]
impl<T, G: Getter<T, E>, E: Copy + Debug> MovingAverageStream<T, G, E> {
    ///Constructor for [`MovingAverageStream`].
    pub const fn new(input: G, window: Time) -> Self {
        Self {
            input: input,
            window: window,
            value: Ok(None),
            input_values: VecDeque::new(),
        }
    }
}
#[cfg(feature = "alloc")]
impl<T: Clone, G: Getter<T, E>, E: Copy + Debug> Getter<T, E> for MovingAverageStream<T, G, E>
where
    MovingAverageStream<T, G, E>: Updatable<E>,
{
    fn get(&self) -> Output<T, E> {
        self.value.clone()
    }
}
#[cfg(feature = "alloc")]
impl<T: Clone, N1: Default, G: Getter<T, E>, E: Copy + Debug> Updatable<E>
    for MovingAverageStream<T, G, E>
where
    T: Mul<Time, Output = N1>,
    N1: AddAssign + Div<Time, Output = T>,
{
    fn update(&mut self) -> NothingOrError<E> {
        let output = self.input.get();
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
            weights.push(end_times[i] - start_times[i]);
        }
        let mut value = N1::default();
        for i in 0..self.input_values.len() {
            value += self.input_values[i].value.clone() * weights[i];
        }
        let value = value / self.window;
        self.value = Ok(Some(Datum::new(output.time, value)));
        Ok(())
    }
}
#[cfg(feature = "alloc")]
impl<G: Getter<Quantity, E>, E: Copy + Debug> Updatable<E> for MovingAverageStream<Quantity, G, E> {
    fn update(&mut self) -> NothingOrError<E> {
        let output = self.input.get();
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
            weights.push(Quantity::from(end_times[i] - start_times[i]));
        }
        let mut value = self.input_values[0].value.clone() * weights[0];
        for i in 1..self.input_values.len() {
            value += self.input_values[i].value.clone() * weights[i];
        }
        value /= Quantity::from(self.window);
        self.value = Ok(Some(Datum::new(output.time, value)));
        Ok(())
    }
}
