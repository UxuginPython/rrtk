// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2024-2025 UxuginPython
//!Streams that perform mathematical operations.
use crate::streams::*;
use core::mem::MaybeUninit;
//TODO: The behavior of SumStream and friends in relation to Ok(None) is maximally unhelpful for
//everyone. Either require Default and return that when all inputs return Ok(None) or return
//Ok(None) when any input returns Ok(None). This is the worst possible combination.
///A stream that adds all its inputs. If one input returns `Ok(None)`, it is excluded. If all inputs
///return `Ok(None)`, returns `Ok(None)`. If this is not the desired behavior, use
///[`NoneToValue`](converters::NoneToValue) or [`NoneToError`](converters::NoneToError).
///[`Sum2`] may also be a bit faster if you are only adding the outputs of two streams.
pub struct SumStream<T: AddAssign + Copy, const N: usize, E> {
    addends: [Reference<dyn Getter<T, E>>; N],
}
impl<T: AddAssign + Copy, const N: usize, E> SumStream<T, N, E> {
    ///Constructor for [`SumStream`].
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
            let mut value = outputs[0].assume_init();
            for i in 1..outputs_filled {
                value += outputs[i].assume_init();
            }
            Ok(Some(value))
        }
    }
}
impl<T: AddAssign + Copy, const N: usize, E: Copy + Debug> Updatable<E> for SumStream<T, N, E> {
    fn update(&mut self) -> NothingOrError<E> {
        Ok(())
    }
}
///A stream that adds two inputs. This should be a bit faster than [`SumStream`], which adds any
///number of inputs. If one inputs returns `Ok(None)`, the other input's output is returned. If
///both inputs return `Ok(None)`, returns `Ok(None)`. If this is not the desired behavior, use
///[`NoneToValue`](converters::NoneToValue) or [`NoneToError`](converters::NoneToError).
pub struct Sum2<T1: Add<T2>, T2, G1: Getter<T1, E>, G2: Getter<T2, E>, E: Copy + Debug> {
    addend1: G1,
    addend2: G2,
    phantom_t1: PhantomData<T1>,
    phantom_t2: PhantomData<T2>,
    phantom_e: PhantomData<E>,
}
impl<T1: Add<T2>, T2, G1: Getter<T1, E>, G2: Getter<T2, E>, E: Copy + Debug>
    Sum2<T1, T2, G1, G2, E>
{
    ///Constructor for [`Sum2`].
    pub const fn new(addend1: G1, addend2: G2) -> Self {
        Self {
            addend1: addend1,
            addend2: addend2,
            phantom_t1: PhantomData,
            phantom_t2: PhantomData,
            phantom_e: PhantomData,
        }
    }
}
impl<
    T1: Add<T2, Output = TO> + Into<TO>,
    T2: Into<TO>,
    TO,
    G1: Getter<T1, E>,
    G2: Getter<T2, E>,
    E: Copy + Debug,
> Getter<TO, E> for Sum2<T1, T2, G1, G2, E>
{
    fn get(&self) -> Output<TO, E> {
        let x = self.addend1.get()?;
        let x = match x {
            Some(x) => x,
            None => {
                return Ok(self
                    .addend2
                    .get()?
                    .map(|datum| Datum::new(datum.time, datum.value.into())));
            }
        };
        let y = self.addend2.get()?;
        let y = match y {
            Some(y) => y,
            None => return Ok(Some(Datum::new(x.time, x.value.into()))),
        };
        Ok(Some(Datum::new(
            core::cmp::max(x.time, y.time),
            x.value + y.value,
        )))
    }
}
impl<T1: Add<T2>, T2, G1: Getter<T1, E>, G2: Getter<T2, E>, E: Copy + Debug> Updatable<E>
    for Sum2<T1, T2, G1, G2, E>
{
    fn update(&mut self) -> NothingOrError<E> {
        Ok(())
    }
}
///A stream that subtracts one of its inputs from the other. If the subtrahend stream returns
///`Ok(None)`, the minuend's value will be returned directly.
pub struct DifferenceStream<TM: Sub<TS>, TS, GM: Getter<TM, E>, GS: Getter<TS, E>, E: Copy + Debug>
{
    minuend: GM,
    subtrahend: GS,
    phantom_tm: PhantomData<TM>,
    phantom_ts: PhantomData<TS>,
    phantom_e: PhantomData<E>,
}
impl<TM: Sub<TS>, TS, GM: Getter<TM, E>, GS: Getter<TS, E>, E: Copy + Debug>
    DifferenceStream<TM, TS, GM, GS, E>
{
    ///Constructor for [`DifferenceStream`].
    pub const fn new(minuend: GM, subtrahend: GS) -> Self {
        Self {
            minuend: minuend,
            subtrahend: subtrahend,
            phantom_tm: PhantomData,
            phantom_ts: PhantomData,
            phantom_e: PhantomData,
        }
    }
}
impl<
    TM: Sub<TS, Output = TO> + Into<TO>,
    TS,
    TO,
    GM: Getter<TM, E>,
    GS: Getter<TS, E>,
    E: Copy + Debug,
> Getter<TO, E> for DifferenceStream<TM, TS, GM, GS, E>
{
    fn get(&self) -> Output<TO, E> {
        let minuend_output = self.minuend.get()?;
        let subtrahend_output = self.subtrahend.get()?;
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
                return Ok(Some(Datum::new(
                    minuend_output.time,
                    minuend_output.value.into(),
                )));
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
impl<TM: Sub<TS>, TS, GM: Getter<TM, E>, GS: Getter<TS, E>, E: Copy + Debug> Updatable<E>
    for DifferenceStream<TM, TS, GM, GS, E>
{
    fn update(&mut self) -> NothingOrError<E> {
        Ok(())
    }
}
///A stream that multiplies its inputs. If an input returns `Ok(None)`, it is excluded from the
///calculation, effectively treating it as though it had returned 1. If this is not the desired
///behavior, use [`rrtk::streams::converters::NoneToValue`](streams::converters::NoneToValue) or
///[`rrtk::streams::converters::NoneToError`](streams::converters::NoneToError). [`Product2`] may
///also be a bit faster if you are only multiplying the outputs of two streams.
pub struct ProductStream<T: MulAssign + Copy, const N: usize, E> {
    factors: [Reference<dyn Getter<T, E>>; N],
}
impl<T: MulAssign + Copy, const N: usize, E> ProductStream<T, N, E> {
    ///Constructor for [`ProductStream`].
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
            let mut value = outputs[0].assume_init();
            for i in 1..outputs_filled {
                value *= outputs[i].assume_init();
            }
            Ok(Some(value))
        }
    }
}
impl<T: MulAssign + Copy, const N: usize, E: Copy + Debug> Updatable<E> for ProductStream<T, N, E> {
    fn update(&mut self) -> NothingOrError<E> {
        Ok(())
    }
}
///A stream that multiplies two inputs. It should be a bit faster than [`ProductStream`], which
///adds any number of inputs. If one input returns `Ok(None)`, returns the other input's output. If
///both inputs return `Ok(None)`, returns `Ok(None)`. If this is not the desired behavior, use
///[`NoneToValue`](converters::NoneToValue) or [`NoneToError`](converters::NoneToError).
pub struct Product2<T: Mul<Output = T>, G1: Getter<T, E>, G2: Getter<T, E>, E: Copy + Debug> {
    addend1: G1,
    addend2: G2,
    phantom_t: PhantomData<T>,
    phantom_e: PhantomData<E>,
}
impl<T: Mul<Output = T>, G1: Getter<T, E>, G2: Getter<T, E>, E: Copy + Debug>
    Product2<T, G1, G2, E>
{
    ///Constructor for [`Product2`].
    pub const fn new(addend1: G1, addend2: G2) -> Self {
        Self {
            addend1: addend1,
            addend2: addend2,
            phantom_t: PhantomData,
            phantom_e: PhantomData,
        }
    }
}
impl<T: Mul<Output = T>, G1: Getter<T, E>, G2: Getter<T, E>, E: Copy + Debug> Getter<T, E>
    for Product2<T, G1, G2, E>
{
    fn get(&self) -> Output<T, E> {
        let x = self.addend1.get()?;
        let x = match x {
            Some(x) => x,
            None => return self.addend2.get(),
        };
        let y = self.addend2.get()?;
        let y = match y {
            Some(y) => y,
            None => return Ok(Some(x)),
        };
        Ok(Some(x * y))
    }
}
impl<T: Mul<Output = T>, G1: Getter<T, E>, G2: Getter<T, E>, E: Copy + Debug> Updatable<E>
    for Product2<T, G1, G2, E>
{
    fn update(&mut self) -> NothingOrError<E> {
        Ok(())
    }
}
///A stream that divides one if its inputs by the other. If the divisor returns `Ok(None)`, the
///dividend's value is returned directly.
pub struct QuotientStream<T: Div<Output = T>, GD: Getter<T, E>, GS: Getter<T, E>, E: Copy + Debug> {
    dividend: GD,
    divisor: GS,
    phantom_t: PhantomData<T>,
    phantom_e: PhantomData<E>,
}
impl<T: Div<Output = T>, GD: Getter<T, E>, GS: Getter<T, E>, E: Copy + Debug>
    QuotientStream<T, GD, GS, E>
{
    ///Constructor for [`QuotientStream`].
    pub const fn new(dividend: GD, divisor: GS) -> Self {
        Self {
            dividend: dividend,
            divisor: divisor,
            phantom_t: PhantomData,
            phantom_e: PhantomData,
        }
    }
}
impl<T: Div<Output = T>, GD: Getter<T, E>, GS: Getter<T, E>, E: Copy + Debug> Getter<T, E>
    for QuotientStream<T, GD, GS, E>
{
    fn get(&self) -> Output<T, E> {
        let dividend_output = self.dividend.get()?;
        let divisor_output = self.divisor.get()?;
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
impl<T: Div<Output = T>, GD: Getter<T, E>, GS: Getter<T, E>, E: Copy + Debug> Updatable<E>
    for QuotientStream<T, GD, GS, E>
{
    fn update(&mut self) -> NothingOrError<E> {
        Ok(())
    }
}
///A stream that exponentiates one of its inputs to the other. If the exponent input returns
///`Ok(None)`, the base's value is returned directly. Only available with `std`.
#[cfg(feature = "internal_enhanced_float")]
pub struct ExponentStream<GB: Getter<f32, E>, GE: Getter<f32, E>, E: Copy + Debug> {
    base: GB,
    exponent: GE,
    phantom_e: PhantomData<E>,
}
#[cfg(feature = "internal_enhanced_float")]
impl<GB: Getter<f32, E>, GE: Getter<f32, E>, E: Copy + Debug> ExponentStream<GB, GE, E> {
    ///Constructor for [`ExponentStream`].
    pub const fn new(base: GB, exponent: GE) -> Self {
        Self {
            base: base,
            exponent: exponent,
            phantom_e: PhantomData,
        }
    }
}
#[cfg(feature = "internal_enhanced_float")]
impl<GB: Getter<f32, E>, GE: Getter<f32, E>, E: Copy + Debug> Getter<f32, E>
    for ExponentStream<GB, GE, E>
{
    fn get(&self) -> Output<f32, E> {
        let base_output = self.base.get()?;
        let exponent_output = self.exponent.get()?;
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
        let value = powf(base_output.value, exponent_output.value);
        let time = if base_output.time > exponent_output.time {
            base_output.time
        } else {
            exponent_output.time
        };
        Ok(Some(Datum::new(time, value)))
    }
}
#[cfg(feature = "internal_enhanced_float")]
impl<GB: Getter<f32, E>, GE: Getter<f32, E>, E: Copy + Debug> Updatable<E>
    for ExponentStream<GB, GE, E>
{
    fn update(&mut self) -> NothingOrError<E> {
        Ok(())
    }
}
///A stream that computes the numerical derivative of its input.
pub struct DerivativeStream<T, O, G: Getter<T, E>, E: Copy + Debug> {
    input: G,
    value: Output<O, E>,
    //doesn't matter if this is an Err or Ok(None) - we can't use it either way if it's not Some
    prev_output: Option<Datum<T>>,
}
impl<T, O, G: Getter<T, E>, E: Copy + Debug> DerivativeStream<T, O, G, E> {
    ///Constructor for [`DerivativeStream`].
    pub const fn new(input: G) -> Self {
        Self {
            input: input,
            value: Ok(None),
            prev_output: None,
        }
    }
}
impl<T, O: Clone, G: Getter<T, E>, E: Copy + Debug> Getter<O, E> for DerivativeStream<T, O, G, E>
where
    DerivativeStream<T, O, G, E>: Updatable<E>,
{
    fn get(&self) -> Output<O, E> {
        self.value.clone()
    }
}
impl<T: Copy, N1, O, G: Getter<T, E>, E: Copy + Debug> Updatable<E> for DerivativeStream<T, O, G, E>
where
    T: Sub<Output = N1>,
    N1: Div<Time, Output = O>,
{
    fn update(&mut self) -> NothingOrError<E> {
        let output = self.input.get();
        let output = match output {
            Ok(ok) => ok,
            Err(error) => {
                self.value = Err(error);
                self.prev_output = None;
                return Err(error);
            }
        };
        let output = match output {
            Some(some) => some,
            None => {
                self.value = Ok(None);
                self.prev_output = None;
                return Ok(());
            }
        };
        let prev_output = match self.prev_output {
            Some(some) => some,
            None => {
                self.prev_output = Some(output);
                return Ok(());
            }
        };
        let value = (output.value - prev_output.value) / (output.time - prev_output.time);
        self.value = Ok(Some(Datum::new(output.time, value)));
        self.prev_output = Some(output);
        Ok(())
    }
}
///A stream that computes the trapezoidal numerical integral of its input.
pub struct IntegralStream<T, O, G: Getter<T, E>, E: Copy + Debug> {
    input: G,
    value: Output<O, E>,
    prev_output: Option<Datum<T>>,
}
impl<T, O, G: Getter<T, E>, E: Copy + Debug> IntegralStream<T, O, G, E> {
    ///Constructor for [`IntegralStream`].
    pub const fn new(input: G) -> Self {
        Self {
            input: input,
            value: Ok(None),
            prev_output: None,
        }
    }
}
impl<T, O: Clone, G: Getter<T, E>, E: Copy + Debug> Getter<O, E> for IntegralStream<T, O, G, E>
where
    IntegralStream<T, O, G, E>: Updatable<E>,
{
    fn get(&self) -> Output<O, E> {
        self.value.clone()
    }
}
impl<T: Copy, O: Copy + Half, N1, G: Getter<T, E>, E: Copy + Debug> Updatable<E>
    for IntegralStream<T, O, G, E>
where
    T: Add<Output = N1>,
    Time: Mul<N1, Output = O>,
    O: Add<O, Output = O>,
{
    fn update(&mut self) -> NothingOrError<E> {
        let output = self.input.get();
        let output = match output {
            Ok(ok) => ok,
            Err(error) => {
                self.value = Err(error);
                self.prev_output = None;
                return Err(error);
            }
        };
        let output = match output {
            Some(some) => some,
            None => {
                self.value = Ok(None);
                self.prev_output = None;
                return Ok(());
            }
        };
        let prev_output = match self.prev_output {
            Some(some) => some,
            None => {
                self.prev_output = Some(output);
                return Ok(());
            }
        };
        let delta_time = output.time - prev_output.time;
        let value_addend = (delta_time * (prev_output.value + output.value)).half();
        let value = match &self.value {
            Ok(Some(real_value)) => value_addend + real_value.value,
            _ => value_addend,
        };
        self.value = Ok(Some(Datum::new(output.time, value)));
        self.prev_output = Some(output);
        return Ok(());
    }
}
