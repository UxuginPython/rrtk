// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2024 UxuginPython
//!Streams that perform mathematical operations.
use crate::streams::*;
use core::mem::MaybeUninit;
//TODO: Make SumStream and ProductStream not use Vec
///A stream that adds all its inputs. If an input returns `Ok(None)`, it is excluded.
pub struct SumStream<T: AddAssign + Copy, const N: usize, E> {
    addends: [Reference<dyn Getter<T, E>>; N],
}
impl<T: AddAssign + Copy, const N: usize, E> SumStream<T, N, E> {
    ///Constructor for `SumStream`.
    pub const fn new(addends: [Reference<dyn Getter<T, E>>; N]) -> Self {
        if N < 1 {
            panic!("rrtk::streams::SumStream must have at least one input stream");
        }
        Self { addends: addends }
    }
}
impl<T: AddAssign + Copy, const N: usize, E: Copy + Debug> Getter<T, E> for SumStream<T, N, E> {
    fn get(&self) -> Output<T, E> {
        //Err(...) -> return Err immediately
        //Ok(None) -> skip
        //Ok(Some(...)) -> add to value
        let mut outputs = [MaybeUninit::uninit(); N];
        //This is always equal to the index of the next uninitialized slot if there is one.
        let mut outputs_filled = 0;
        for i in &self.addends {
            match i.borrow().get()? {
                Some(x) => {
                    outputs[outputs_filled].write(x);
                    outputs_filled += 1;
                }
                None => (),
            }
        }
        if outputs_filled == 0 {
            return Ok(None);
        }
        //We can safely assume_init on outputs indexes within 0..outputs_filled.
        unsafe {
            //We now know that at least 1 input is filled. It is at index 0.
            let mut value = outputs[0].assume_init().value;
            //This works because outputs_filled is equal to the index of the next free slot and the
            //range thing cuts off one before it.
            for i in 1..outputs_filled {
                value += outputs[i].assume_init().value;
            }
            let mut time = outputs[0].assume_init().time;
            for i in 1..outputs_filled {
                let maybe_new_time = outputs[i].assume_init().time;
                if maybe_new_time > time {
                    time = maybe_new_time;
                }
            }
            return Ok(Some(Datum::new(time, value)));
        }
    }
}
impl<T: AddAssign + Copy, const N: usize, E: Copy + Debug> Updatable<E> for SumStream<T, N, E> {
    fn update(&mut self) -> NothingOrError<E> {
        Ok(())
    }
}
///A stream that subtracts one of its inputs from the other. If the subtrahend stream returns
///`Ok(None)`, the minuend's value will be returned directly.
pub struct DifferenceStream<
    T: Sub<Output = T>,
    GM: Getter<T, E> + ?Sized,
    GS: Getter<T, E> + ?Sized,
    E: Copy + Debug,
> {
    minuend: Reference<GM>,
    subtrahend: Reference<GS>,
    phantom_t: PhantomData<T>,
    phantom_e: PhantomData<E>,
}
impl<T: Sub<Output = T>, GM: Getter<T, E> + ?Sized, GS: Getter<T, E> + ?Sized, E: Copy + Debug>
    DifferenceStream<T, GM, GS, E>
{
    ///Constructor for `DifferenceStream`.
    pub const fn new(minuend: Reference<GM>, subtrahend: Reference<GS>) -> Self {
        Self {
            minuend: minuend,
            subtrahend: subtrahend,
            phantom_t: PhantomData,
            phantom_e: PhantomData,
        }
    }
}
impl<T: Sub<Output = T>, GM: Getter<T, E> + ?Sized, GS: Getter<T, E> + ?Sized, E: Copy + Debug>
    Getter<T, E> for DifferenceStream<T, GM, GS, E>
{
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
impl<T: Sub<Output = T>, GM: Getter<T, E> + ?Sized, GS: Getter<T, E> + ?Sized, E: Copy + Debug>
    Updatable<E> for DifferenceStream<T, GM, GS, E>
{
    fn update(&mut self) -> NothingOrError<E> {
        Ok(())
    }
}
///A stream that multiplies its inputs. If an input returns `Ok(None)`, it is excluded from the
///calculation, effectively treating it as though it had returned 1. If this is not the desired
///behavior, use `rrtk::streams::converters::NoneToValue` or
///`rrtk::streams::converters::NoneToError`.
pub struct ProductStream<T: MulAssign + Copy, const N: usize, E> {
    factors: [Reference<dyn Getter<T, E>>; N],
}
impl<T: MulAssign + Copy, const N: usize, E> ProductStream<T, N, E> {
    ///Constructor for `ProductStream`.
    pub const fn new(factors: [Reference<dyn Getter<T, E>>; N]) -> Self {
        if N < 1 {
            panic!("rrtk::streams::ProductStream must have at least one input stream");
        }
        Self { factors: factors }
    }
}
impl<T: MulAssign + Copy, const N: usize, E: Copy + Debug> Getter<T, E> for ProductStream<T, N, E> {
    fn get(&self) -> Output<T, E> {
        let mut outputs = [MaybeUninit::uninit(); N];
        let mut outputs_filled = 0;
        for i in &self.factors {
            match i.borrow().get()? {
                Some(x) => {
                    outputs[outputs_filled].write(x);
                    outputs_filled += 1;
                }
                None => (),
            }
        }
        if outputs_filled == 0 {
            return Ok(None);
        }
        unsafe {
            let mut value = outputs[0].assume_init().value;
            for i in 1..outputs_filled {
                value *= outputs[i].assume_init().value;
            }
            let mut time = outputs[0].assume_init().time;
            for i in 1..outputs_filled {
                let output = outputs[i].assume_init();
                if output.time > time {
                    time = output.time;
                }
            }
            Ok(Some(Datum::new(time, value)))
        }
    }
}
impl<T: MulAssign + Copy, const N: usize, E: Copy + Debug> Updatable<E> for ProductStream<T, N, E> {
    fn update(&mut self) -> NothingOrError<E> {
        Ok(())
    }
}
///A stream that divides one if its inputs by the other. If the divisor returns `Ok(None)`, the
///dividend's value is returned directly.
pub struct QuotientStream<
    T: Div<Output = T>,
    GD: Getter<T, E> + ?Sized,
    GS: Getter<T, E> + ?Sized,
    E: Copy + Debug,
> {
    dividend: Reference<GD>,
    divisor: Reference<GS>,
    phantom_t: PhantomData<T>,
    phantom_e: PhantomData<E>,
}
impl<T: Div<Output = T>, GD: Getter<T, E> + ?Sized, GS: Getter<T, E> + ?Sized, E: Copy + Debug>
    QuotientStream<T, GD, GS, E>
{
    ///Constructor for `QuotientStream`.
    pub const fn new(dividend: Reference<GD>, divisor: Reference<GS>) -> Self {
        Self {
            dividend: dividend,
            divisor: divisor,
            phantom_t: PhantomData,
            phantom_e: PhantomData,
        }
    }
}
impl<T: Div<Output = T>, GD: Getter<T, E> + ?Sized, GS: Getter<T, E> + ?Sized, E: Copy + Debug>
    Getter<T, E> for QuotientStream<T, GD, GS, E>
{
    fn get(&self) -> Output<T, E> {
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
impl<T: Div<Output = T>, GD: Getter<T, E> + ?Sized, GS: Getter<T, E> + ?Sized, E: Copy + Debug>
    Updatable<E> for QuotientStream<T, GD, GS, E>
{
    fn update(&mut self) -> NothingOrError<E> {
        Ok(())
    }
}
///A stream that exponentiates one of its inputs to the other. If the exponent input returns
///`Ok(None)`, the base's value is returned directly. Only available with `std`.
#[cfg(feature = "std")]
pub struct ExponentStream<GB: Getter<f32, E> + ?Sized, GE: Getter<f32, E> + ?Sized, E: Copy + Debug>
{
    base: Reference<GB>,
    exponent: Reference<GE>,
    phantom_e: PhantomData<E>,
}
#[cfg(feature = "std")]
impl<GB: Getter<f32, E> + ?Sized, GE: Getter<f32, E> + ?Sized, E: Copy + Debug>
    ExponentStream<GB, GE, E>
{
    ///Constructor for `ExponentStream`.
    pub const fn new(base: Reference<GB>, exponent: Reference<GE>) -> Self {
        Self {
            base: base,
            exponent: exponent,
            phantom_e: PhantomData,
        }
    }
}
#[cfg(feature = "std")]
impl<GB: Getter<f32, E> + ?Sized, GE: Getter<f32, E> + ?Sized, E: Copy + Debug> Getter<f32, E>
    for ExponentStream<GB, GE, E>
{
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
impl<GB: Getter<f32, E> + ?Sized, GE: Getter<f32, E> + ?Sized, E: Copy + Debug> Updatable<E>
    for ExponentStream<GB, GE, E>
{
    fn update(&mut self) -> NothingOrError<E> {
        Ok(())
    }
}
///A stream that computes the numerical derivative of its input.
pub struct DerivativeStream<G: Getter<f32, E> + ?Sized, E: Copy + Debug> {
    input: Reference<G>,
    value: Output<f32, E>,
    //doesn't matter if this is an Err or Ok(None) - we can't use it either way if it's not Some
    prev_output: Option<Datum<f32>>,
}
impl<G: Getter<f32, E> + ?Sized, E: Copy + Debug> DerivativeStream<G, E> {
    ///Constructor for `DerivativeStream`.
    pub const fn new(input: Reference<G>) -> Self {
        Self {
            input: input,
            value: Ok(None),
            prev_output: None,
        }
    }
}
impl<G: Getter<f32, E> + ?Sized, E: Copy + Debug> Getter<f32, E> for DerivativeStream<G, E> {
    fn get(&self) -> Output<f32, E> {
        self.value.clone()
    }
}
impl<G: Getter<f32, E> + ?Sized, E: Copy + Debug> Updatable<E> for DerivativeStream<G, E> {
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
pub struct IntegralStream<G: Getter<f32, E> + ?Sized, E: Copy + Debug> {
    input: Reference<G>,
    value: Output<f32, E>,
    prev_output: Option<Datum<f32>>,
}
impl<G: Getter<f32, E> + ?Sized, E: Copy + Debug> IntegralStream<G, E> {
    ///Constructor for `IntegralStream`.
    pub const fn new(input: Reference<G>) -> Self {
        Self {
            input: input,
            value: Ok(None),
            prev_output: None,
        }
    }
}
impl<G: Getter<f32, E> + ?Sized, E: Copy + Debug> Getter<f32, E> for IntegralStream<G, E> {
    fn get(&self) -> Output<f32, E> {
        self.value.clone()
    }
}
impl<G: Getter<f32, E> + ?Sized, E: Copy + Debug> Updatable<E> for IntegralStream<G, E> {
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
