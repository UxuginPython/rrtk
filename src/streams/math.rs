// SPDX-License-Identifier: LGPL-3.0-only
/*
Copyright 2024 UxuginPython on GitHub

     This file is part of Rust Robotics ToolKit.

    Rust Robotics ToolKit is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, version 3.

    Rust Robotics ToolKit is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Lesser General Public License for more details.

    You should have received a copy of the GNU Lesser General Public License along with Rust Robotics ToolKit. If not, see <https://www.gnu.org/licenses/>.
*/
//!Streams that perform mathematical operations.
use crate::streams::*;
///A stream that adds all its inputs. If an input returns `Ok(None)`, it is excluded.
pub struct SumStream<T: AddAssign + Clone + Default, const N: usize, E> {
    addends: [InputGetter<T, E>; N],
}
impl<T: AddAssign + Clone + Default, const N: usize, E> SumStream<T, N, E> {
    ///Constructor for `SumStream`.
    pub fn new(addends: [InputGetter<T, E>; N]) -> Self {
        if N < 1 {
            panic!("rrtk::streams::SumStream must have at least one input stream");
        }
        Self { addends: addends }
    }
}
impl<T: AddAssign + Clone + Default, const N: usize, E: Copy + Debug> Getter<T, E>
    for SumStream<T, N, E>
{
    fn get(&self) -> Output<T, E> {
        //Err(...) -> return Err immediately
        //Ok(None) -> skip
        //Ok(Some(...)) -> add to value
        let mut outputs = Vec::new();
        for i in &self.addends {
            outputs.push(i.borrow().get()?);
        }
        let mut value = T::default();
        for i in &outputs {
            match i {
                Some(output) => {
                    value += output.value.clone();
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
}
impl<T: AddAssign + Clone + Default, const N: usize, E: Copy + Debug> Updatable<E> for SumStream<T, N, E> {
    fn update(&mut self) -> NothingOrError<E> {
        Ok(())
    }
}
///A stream that subtracts one of its inputs from the other. If the subtrahend stream returns
///`Ok(None)`, the minuend's value will be returned directly.
pub struct DifferenceStream<T: Sub<Output = T>, E: Copy + Debug> {
    minuend: InputGetter<T, E>,
    subtrahend: InputGetter<T, E>,
}
impl<T: Sub<Output = T>, E: Copy + Debug> DifferenceStream<T, E> {
    ///Constructor for `DifferenceStream`.
    pub fn new(minuend: InputGetter<T, E>, subtrahend: InputGetter<T, E>) -> Self {
        Self {
            minuend: minuend,
            subtrahend: subtrahend,
        }
    }
}
impl<T: Sub<Output = T>, E: Copy + Debug> Getter<T, E> for DifferenceStream<T, E> {
    fn get(&self) -> Output<T, E> {
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
}
impl<T: Sub<Output = T>, E: Copy + Debug> Updatable<E> for DifferenceStream<T, E> {
    fn update(&mut self) -> NothingOrError<E> {
        Ok(())
    }
}
///A stream that multiplies its inputs. If an input returns `Ok(None)`, it is excluded from the
///calculation, effectively treating it as though it had returned 1. If this is not the desired
///behavior, use `rrtk::streams::converters::NoneToValue` or
///`rrtk::streams::converters::NoneToError`.
pub struct ProductStream<const N: usize, E> {
    factors: [InputGetter<f32, E>; N],
}
impl<const N: usize, E> ProductStream<N, E> {
    ///Constructor for `ProductStream`.
    pub fn new(factors: [InputGetter<f32, E>; N]) -> Self {
        if N < 1 {
            panic!("rrtk::streams::ProductStream must have at least one input stream");
        }
        Self { factors: factors }
    }
}
impl<const N: usize, E: Copy + Debug> Getter<f32, E> for ProductStream<N, E> {
    fn get(&self) -> Output<f32, E> {
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
}
impl<const N: usize, E: Copy + Debug> Updatable<E> for ProductStream<N, E> {
    fn update(&mut self) -> NothingOrError<E> {
        Ok(())
    }
}
///A stream that divides one if its inputs by the other. If the divisor returns `Ok(None)`, the
///dividend's value is returned directly.
pub struct QuotientStream<E> {
    dividend: InputGetter<f32, E>,
    divisor: InputGetter<f32, E>,
}
impl<E> QuotientStream<E> {
    ///Constructor for `QuotientStream`.
    pub fn new(dividend: InputGetter<f32, E>, divisor: InputGetter<f32, E>) -> Self {
        Self {
            dividend: dividend,
            divisor: divisor,
        }
    }
}
impl<E: Copy + Debug> Getter<f32, E> for QuotientStream<E> {
    fn get(&self) -> Output<f32, E> {
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
}
impl<E: Copy + Debug> Updatable<E> for QuotientStream<E> {
    fn update(&mut self) -> NothingOrError<E> {
        Ok(())
    }
}
///A stream that exponentiates one of its inputs to the other. If the exponent input returns
///`Ok(None)`, the base's value is returned directly. Only available with `std`.
#[cfg(feature = "std")]
pub struct ExponentStream<E> {
    base: InputGetter<f32, E>,
    exponent: InputGetter<f32, E>,
}
#[cfg(feature = "std")]
impl<E> ExponentStream<E> {
    ///Constructor for `ExponentStream`.
    pub fn new(base: InputGetter<f32, E>, exponent: InputGetter<f32, E>) -> Self {
        Self {
            base: base,
            exponent: exponent,
        }
    }
}
#[cfg(feature = "std")]
impl<E: Copy + Debug> Getter<f32, E> for ExponentStream<E> {
    fn get(&self) -> Output<f32, E> {
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
}
#[cfg(feature = "std")]
impl<E: Copy + Debug> Updatable<E> for ExponentStream<E> {
    fn update(&mut self) -> NothingOrError<E> {
        Ok(())
    }
}
///A stream that computes the numerical derivative of its input.
pub struct DerivativeStream<E: Copy + Debug> {
    input: InputGetter<f32, E>,
    value: Output<f32, E>,
    //doesn't matter if this is an Err or Ok(None) - we can't use it either way if it's not Some
    prev_output: Option<Datum<f32>>,
}
impl<E: Copy + Debug> DerivativeStream<E> {
    ///Constructor for `DerivativeStream`.
    pub fn new(input: InputGetter<f32, E>) -> Self {
        Self {
            input: input,
            value: Ok(None),
            prev_output: None,
        }
    }
}
impl<E: Copy + Debug> Getter<f32, E> for DerivativeStream<E> {
    fn get(&self) -> Output<f32, E> {
        self.value.clone()
    }
}
impl<E: Copy + Debug> Updatable<E> for DerivativeStream<E> {
    fn update(&mut self) -> NothingOrError<E> {
        let output = self.input.borrow().get();
        match output {
            Ok(_) => {}
            Err(error) => {
                self.value = Err(error);
                self.prev_output = None;
                return Err(error);
            }
        }
        let output = output.unwrap();
        match output {
            Some(_) => {}
            None => {
                self.value = Ok(None);
                self.prev_output = None;
                return Ok(());
            }
        }
        let output = output.unwrap();
        match self.prev_output {
            Some(_) => {}
            None => {
                self.prev_output = Some(output);
                return Ok(());
            }
        }
        let prev_output = self.prev_output.as_ref().unwrap();
        let value = (output.value - prev_output.value) / ((output.time - prev_output.time) as f32);
        self.value = Ok(Some(Datum::new(output.time, value)));
        self.prev_output = Some(output);
        Ok(())
    }
}
///A stream that computes the trapezoidal numerical integral of its input.
pub struct IntegralStream<E: Copy + Debug> {
    input: InputGetter<f32, E>,
    value: Output<f32, E>,
    prev_output: Option<Datum<f32>>,
}
impl<E: Copy + Debug> IntegralStream<E> {
    ///Constructor for `IntegralStream`.
    pub fn new(input: InputGetter<f32, E>) -> Self {
        Self {
            input: input,
            value: Ok(None),
            prev_output: None,
        }
    }
}
impl<E: Copy + Debug> Getter<f32, E> for IntegralStream<E> {
    fn get(&self) -> Output<f32, E> {
        self.value.clone()
    }
}
impl<E: Copy + Debug> Updatable<E> for IntegralStream<E> {
    fn update(&mut self) -> NothingOrError<E> {
        let output = self.input.borrow().get();
        match output {
            Ok(_) => {}
            Err(error) => {
                self.value = Err(error);
                self.prev_output = None;
                return Err(error);
            }
        }
        let output = output.unwrap();
        match output {
            Some(_) => {}
            None => {
                self.value = Ok(None);
                self.prev_output = None;
                return Ok(());
            }
        }
        let output = output.unwrap();
        match self.prev_output {
            Some(_) => {}
            None => {
                self.prev_output = Some(output);
                return Ok(());
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
            + ((output.time - prev_output.time) as f32) * (prev_output.value + output.value) / 2.0;
        self.value = Ok(Some(Datum::new(output.time, value)));
        self.prev_output = Some(output);
        return Ok(());
    }
}
