// SPDX-License-Identifier: LGPL-3.0-only
/*
Copyright 2024 UxuginPython on GitHub

     This file is part of Rust Robotics ToolKit.

    Rust Robotics ToolKit is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, version 3.

    Rust Robotics ToolKit is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Lesser General Public License for more details.

    You should have received a copy of the GNU Lesser General Public License along with Rust Robotics ToolKit. If not, see <https://www.gnu.org/licenses/>.
*/
use crate::streams::converters::*;
use crate::streams::math::*;
use crate::streams::*;
//This does store the timestamp twice, once in prev_error and once in output. Processor performance
//and readability would suggest doing it this way, but 8 bytes could technically be saved here if
//needed in the future. The difference is extremely minimal.
pub struct StreamPID<E: Copy + Debug> {
    input: InputGetter<f32, E>,
    setpoint: f32,
    kvals: PIDKValues,
    prev_error: Option<Datum<f32>>,
    int_error: f32,
    output: Output<f32, E>,
}
impl<E: Copy + Debug> StreamPID<E> {
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
impl<E: Copy + Debug> Getter<f32, E> for StreamPID<E> {
    fn get(&self) -> Output<f32, E> {
        self.output.clone()
    }
}
impl<E: Copy + Debug> Updatable<E> for StreamPID<E> {
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
        let error = self.setpoint- process.value;
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
        self.output = Ok(Some(Datum::new(process.time, self.kvals.kp * error + self.kvals.ki * self.int_error + self.kvals.kd * drv_error)));
        self.prev_error = Some(Datum::new(process.time, error));
        Ok(())
    }
}
/*pub struct StreamPID<E: Copy + Debug> {
    int: InputGetter<f32, E>,
    drv: InputGetter<f32, E>,
    output: SumStream<3, E>,
}
impl<E: Copy + Debug + 'static> StreamPID<E> {
    pub fn new(input: InputGetter<f32, E>, setpoint: f32, kp: f32, ki: f32, kd: f32) -> Self {
        let time_getter = make_input_time_getter!(TimeGetterFromStream::new(Rc::clone(&input)), E);
        let setpoint = make_input_getter!(Constant::new(Rc::clone(&time_getter), setpoint), f32, E);
        let kp = make_input_getter!(Constant::new(Rc::clone(&time_getter), kp), f32, E);
        let ki = make_input_getter!(Constant::new(Rc::clone(&time_getter), ki), f32, E);
        let kd = make_input_getter!(Constant::new(Rc::clone(&time_getter), kd), f32, E);
        let error = make_input_getter!(
            DifferenceStream::new(Rc::clone(&setpoint), Rc::clone(&input)),
            f32,
            E
        );
        let int = make_input_getter!(IntegralStream::new(Rc::clone(&error)), f32, E);
        let drv = make_input_getter!(DerivativeStream::new(Rc::clone(&error)), f32, E);
        //`ProductStream`'s behavior is to treat all `None` values as 1.0 so that it's as if they
        //were not included. However, this is not what we want with the coefficient. `NoneToValue`
        //is used to convert all `None` values to `Some(0.0)` to effectively exlude them from the
        //final sum.
        let int_zeroer = make_input_getter!(
            NoneToValue::new(Rc::clone(&int), Rc::clone(&time_getter), 0.0),
            f32,
            E
        );
        let drv_zeroer = make_input_getter!(
            NoneToValue::new(Rc::clone(&drv), Rc::clone(&time_getter), 0.0),
            f32,
            E
        );
        let kp_mul = make_input_getter!(
            ProductStream::new([Rc::clone(&kp), Rc::clone(&error)]),
            f32,
            E
        );
        let ki_mul = make_input_getter!(
            ProductStream::new([Rc::clone(&ki), Rc::clone(&int_zeroer)]),
            f32,
            E
        );
        let kd_mul = make_input_getter!(
            ProductStream::new([Rc::clone(&kd), Rc::clone(&drv_zeroer)]),
            f32,
            E
        );
        let output = SumStream::new([Rc::clone(&kp_mul), Rc::clone(&ki_mul), Rc::clone(&kd_mul)]);
        Self {
            int: Rc::clone(&int),
            drv: Rc::clone(&drv),
            output: output,
        }
    }
}
impl<E: Copy + Debug + 'static> Getter<f32, E> for StreamPID<E> {
    fn get(&self) -> Output<f32, E> {
        self.output.get()
    }
}
impl<E: Copy + Debug + 'static> Updatable<E> for StreamPID<E> {
    fn update(&mut self) -> NothingOrError<E> {
        self.int.borrow_mut().update()?;
        self.drv.borrow_mut().update()?;
        Ok(())
    }
}*/
//https://www.itl.nist.gov/div898/handbook/pmc/section3/pmc324.htm
pub struct EWMAStream<E: Copy + Debug> {
    input: InputGetter<f32, E>,
    //As data may not come in at regular intervals as is assumed by a standard EWMA, this value
    //will be multiplied by delta time before being used.
    smoothing_constant: f32,
    value: Output<f32, E>,
    update_time: Option<i64>,
}
impl<E: Copy + Debug> EWMAStream<E> {
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
pub struct MovingAverageStream<E: Copy + Debug> {
    input: InputGetter<f32, E>,
    window: i64,
    value: Output<f32, E>,
    input_values: VecDeque<Datum<f32>>,
}
impl<E: Copy + Debug> MovingAverageStream<E> {
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
