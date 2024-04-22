// SPDX-License-Identifier: LGPL-3.0-only
/*
Copyright 2024 UxuginPython on GitHub

     This file is part of Rust Robotics ToolKit.

    Rust Robotics ToolKit is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, version 3.

    Rust Robotics ToolKit is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Lesser General Public License for more details.

    You should have received a copy of the GNU Lesser General Public License along with Rust Robotics ToolKit. If not, see <https://www.gnu.org/licenses/>.
*/
use crate::*;
use crate::streams::*;
pub struct SumStream<const N: usize, E> {
    addends: [InputStream<f32, E>; N]
}
impl<const N: usize, E> SumStream<N, E> {
    pub fn new(addends: [InputStream<f32, E>; N]) -> Self {
        if N < 1 {
            panic!("rrtk::streams::SumStream must have at least one input stream");
        }
        Self { addends: addends }
    }
}
impl<const N: usize, E: Copy + Debug> Stream<f32, E> for SumStream<N, E> {
    fn get(&self) -> StreamOutput<f32, E> {
        //Err(...) -> return Err immediately
        //Ok(None) -> skip
        //Ok(Some(...)) -> add to value
        let mut outputs = Vec::new();
        for i in &self.addends {
            outputs.push(i.borrow().get()?);
        }
        let mut value = 0.0;
        for i in &outputs {
            match i {
                Some(output) => {
                    value += output.value;
                }
                None => {}
            }
        }
        let mut time = None;
        for i in &outputs {
            match i {
                Some(output) => match time {
                    Some(old_time) => {
                        if output.time > old_time {
                            time = Some(output.time);
                        }
                    }
                    None => {
                        time = Some(output.time);
                    }
                },
                None => {}
            }
        }
        match time {
            Some(time_) => {
                return Ok(Some(Datum::new(time_, value)));
            }
            None => {
                return Ok(None);
            }
        }
    }
    fn update(&mut self) {}
}
pub struct DifferenceStream<E> {
    minuend: InputStream<f32, E>,
    subtrahend: InputStream<f32, E>,
}
impl<E> DifferenceStream<E> {
    pub fn new(minuend: InputStream<f32, E>, subtrahend: InputStream<f32, E>) -> Self {
        Self {
            minuend: minuend,
            subtrahend: subtrahend,
        }
    }
}
impl<E: Copy + Debug> Stream<f32, E> for DifferenceStream<E> {
    fn get(&self) -> StreamOutput<f32, E> {
        let minuend_output = self.minuend.borrow().get()?;
        let subtrahend_output = self.subtrahend.borrow().get()?;
        match minuend_output {
            Some(_) => {}
            None => {
                return Ok(None);
            }
        }
        let minuend_output = minuend_output.unwrap();
        match subtrahend_output {
            Some(_) => {}
            None => {
                return Ok(Some(minuend_output));
            }
        }
        let subtrahend_output = subtrahend_output.unwrap();
        let value = minuend_output.value - subtrahend_output.value;
        let time = if minuend_output.time > subtrahend_output.time {
            minuend_output.time
        } else {
            subtrahend_output.time
        };
        Ok(Some(Datum::new(time, value)))
    }
    fn update(&mut self) {}
}
pub struct ProductStream<const N: usize, E> {
    factors: [InputStream<f32, E>; N],
}
impl<const N: usize, E> ProductStream<N, E> {
    pub fn new(factors: [InputStream<f32, E>; N]) -> Self {
        if N < 1 {
            panic!("rrtk::streams::ProductStream must have at least one input stream");
        }
        Self { factors: factors }
    }
}
impl<const N: usize, E: Copy + Debug> Stream<f32, E> for ProductStream<N, E> {
    fn get(&self) -> StreamOutput<f32, E> {
        let mut outputs = Vec::new();
        for i in &self.factors {
            outputs.push(i.borrow().get()?);
        }
        let mut value = 1.0;
        for i in &outputs {
            match i {
                Some(output) => {
                    value *= output.value;
                }
                None => {}
            }
        }
        let mut time = None;
        for i in &outputs {
            match i {
                Some(output) => match time {
                    Some(old_time) => {
                        if output.time > old_time {
                            time = Some(output.time);
                        }
                    }
                    None => {
                        time = Some(output.time);
                    }
                },
                None => {}
            }
        }
        match time {
            Some(time_) => {
                return Ok(Some(Datum::new(time_, value)));
            }
            None => {
                return Ok(None);
            }
        }
    }
    fn update(&mut self) {}
}
pub struct QuotientStream<E> {
    dividend: InputStream<f32, E>,
    divisor: InputStream<f32, E>,
}
impl<E> QuotientStream<E> {
    pub fn new(dividend: InputStream<f32, E>, divisor: InputStream<f32, E>) -> Self {
        Self {
            dividend: dividend,
            divisor: divisor,
        }
    }
}
impl<E: Copy + Debug> Stream<f32, E> for QuotientStream<E> {
    fn get(&self) -> StreamOutput<f32, E> {
        let dividend_output = self.dividend.borrow().get()?;
        let divisor_output = self.divisor.borrow().get()?;
        match dividend_output {
            Some(_) => {}
            None => {
                return Ok(None);
            }
        }
        let dividend_output = dividend_output.unwrap();
        match divisor_output {
            Some(_) => {}
            None => {
                return Ok(Some(dividend_output));
            }
        }
        let divisor_output = divisor_output.unwrap();
        let value = dividend_output.value / divisor_output.value;
        let time = if dividend_output.time > divisor_output.time {
            dividend_output.time
        } else {
            divisor_output.time
        };
        Ok(Some(Datum::new(time, value)))
    }
    fn update(&mut self) {}
}
#[cfg(feature = "std")]
pub struct ExponentStream<E> {
    base: InputStream<f32, E>,
    exponent: InputStream<f32, E>,
}
#[cfg(feature = "std")]
impl<E> ExponentStream<E> {
    pub fn new(base: InputStream<f32, E>, exponent: InputStream<f32, E>) -> Self {
        Self {
            base: base,
            exponent: exponent,
        }
    }
}
#[cfg(feature = "std")]
impl<E: Copy + Debug> Stream<f32, E> for ExponentStream<E> {
    fn get(&self) -> StreamOutput<f32, E> {
        let base_output = self.base.borrow().get()?;
        let exponent_output = self.exponent.borrow().get()?;
        match base_output {
            Some(_) => {}
            None => {
                return Ok(None);
            }
        }
        let base_output = base_output.unwrap();
        match exponent_output {
            Some(_) => {}
            None => {
                return Ok(Some(base_output));
            }
        }
        let exponent_output = exponent_output.unwrap();
        let value = base_output.value.powf(exponent_output.value);
        let time = if base_output.time > exponent_output.time {
            base_output.time
        } else {
            exponent_output.time
        };
        Ok(Some(Datum::new(time, value)))
    }
    fn update(&mut self) {}
}
pub struct DerivativeStream<E: Copy + Debug> {
    input: InputStream<f32, E>,
    value: StreamOutput<f32, E>,
    //doesn't matter if this is an Err or Ok(None) - we can't use it either way if it's not Some
    prev_output: Option<Datum<f32>>,
}
impl<E: Copy + Debug> DerivativeStream<E> {
    pub fn new(input: InputStream<f32, E>) -> Self {
        Self {
            input: input,
            value: Ok(None),
            prev_output: None,
        }
    }
}
impl<E: Copy + Debug> Stream<f32, E> for DerivativeStream<E> {
    fn get(&self) -> StreamOutput<f32, E> {
        self.value.clone()
    }
    fn update(&mut self) {
        let output = self.input.borrow().get();
        match output {
            Ok(_) => {}
            Err(error) => {
                self.value = Err(error);
                self.prev_output = None;
                return;
            }
        }
        let output = output.unwrap();
        match output {
            Some(_) => {}
            None => {
                self.value = Ok(None);
                self.prev_output = None;
                return;
            }
        }
        let output = output.unwrap();
        match self.prev_output {
            Some(_) => {}
            None => {
                self.prev_output = Some(output);
                return;
            }
        }
        let prev_output = self.prev_output.as_ref().unwrap();
        let value = (output.value - prev_output.value) / (output.time - prev_output.time);
        self.value = Ok(Some(Datum::new(output.time, value)));
        self.prev_output = Some(output);
    }
}
pub struct IntegralStream<E: Copy + Debug> {
    input: InputStream<f32, E>,
    value: StreamOutput<f32, E>,
    prev_output: Option<Datum<f32>>,
}
impl<E: Copy + Debug> IntegralStream<E> {
    pub fn new(input: InputStream<f32, E>) -> Self {
        Self {
            input: input,
            value: Ok(None),
            prev_output: None,
        }
    }
}
impl<E: Copy + Debug> Stream<f32, E> for IntegralStream<E> {
    fn get(&self) -> StreamOutput<f32, E> {
        self.value.clone()
    }
    fn update(&mut self) {
        let output = self.input.borrow().get();
        match output {
            Ok(_) => {}
            Err(error) => {
                self.value = Err(error);
                self.prev_output = None;
                return;
            }
        }
        let output = output.unwrap();
        match output {
            Some(_) => {}
            None => {
                self.value = Ok(None);
                self.prev_output = None;
                return;
            }
        }
        let output = output.unwrap();
        match self.prev_output {
            Some(_) => {}
            None => {
                self.prev_output = Some(output);
                return;
            }
        }
        let prev_output = self.prev_output.as_ref().unwrap();
        let prev_value = match &self.value {
            Ok(option_value) => match option_value {
                Some(real_value) => real_value.value,
                None => 0.0,
            },
            Err(_) => 0.0,
        };
        let value = prev_value
            + (output.time - prev_output.time) * (prev_output.value + output.value) / 2.0;
        self.value = Ok(Some(Datum::new(output.time, value)));
        self.prev_output = Some(output);
    }
}
