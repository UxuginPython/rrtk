// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2024-2026 UxuginPython
//!Streams that perform mathematical operations.
use crate::streams::*;
use core::mem::MaybeUninit;
///A stream that adds all its inputs sequentially with `AddAssign`. For this stream to return a
///value, all of its inputs must return the `Ok(Some(_))` variant; if an input returns `Ok(None)` or
///`Err(_)`, that value is returned immediately.
///
///If your usecase requires excluding `None` values from the sum as opposed to this behavior,
///consider wrapping inputs in [`converters::NoneToDefault`] or [`converters::NoneToValue`] with a
///value of 0 for the `None` variant.
///
///If you are only adding the outputs of two getters or if your input getters are of different
///types, consider using [`Sum2`] instead.
pub struct SumStream<const N: usize, G> {
    addends: [G; N],
}
impl<const N: usize, G> SumStream<N, G> {
    ///Constructor for [`SumStream`].
    pub const fn new(addends: [G; N]) -> Self {
        if N < 1 {
            panic!("rrtk::streams::SumStream must have at least one input stream");
        }
        Self { addends }
    }
}
impl<T, const N: usize, G, E> Getter<T, E> for SumStream<N, G>
where
    T: AddAssign + Copy,
    G: Getter<T, E>,
    E: Clone + Debug,
{
    fn get(&self) -> Output<T, E> {
        let mut addend_values = [MaybeUninit::uninit(); N];
        for (i, input) in self.addends.iter().enumerate() {
            let gotten = input.get();
            if let Ok(Some(value)) = gotten {
                addend_values[i].write(value);
            } else {
                return gotten;
            }
        }
        unsafe {
            let mut total = addend_values[0].assume_init();
            for addend_value in addend_values.into_iter().skip(1) {
                total += addend_value.assume_init();
            }
            Ok(Some(total))
        }
    }
}
impl<const N: usize, G, E> Updatable<E> for SumStream<N, G>
where
    G: Updatable<E>,
    E: Clone + Debug,
{
    fn update(&mut self) -> NothingOrError<E> {
        for getter in &mut self.addends {
            getter.update()?;
        }
        Ok(())
    }
}
///A stream that adds two inputs. This should be a bit faster than [`SumStream`], which adds any
///number of inputs. Returns `Ok(None)` if either input does. If this is not the desired behavior,
///[`NoneToValue`](converters::NoneToValue) may be of interest.
pub struct Sum2<T1, T2, G1, G2> {
    addend1: G1,
    addend2: G2,
    phantom_t1: PhantomData<T1>,
    phantom_t2: PhantomData<T2>,
}
impl<T1, T2, G1, G2> Sum2<T1, T2, G1, G2> {
    ///Constructor for [`Sum2`].
    pub const fn new(addend1: G1, addend2: G2) -> Self {
        Self {
            addend1,
            addend2,
            phantom_t1: PhantomData,
            phantom_t2: PhantomData,
        }
    }
}
impl<T1, T2, TO, G1, G2, E> Getter<TO, E> for Sum2<T1, T2, G1, G2>
where
    T1: Add<T2, Output = TO>,
    G1: Getter<T1, E>,
    G2: Getter<T2, E>,
    E: Clone + Debug,
{
    fn get(&self) -> Output<TO, E> {
        let x = self.addend1.get()?;
        let x = match x {
            Some(x) => x,
            None => {
                return Ok(None);
            }
        };
        let y = self.addend2.get()?;
        let y = match y {
            Some(y) => y,
            None => return Ok(None),
        };
        Ok(Some(Datum::new(
            core::cmp::max(x.time, y.time),
            x.value + y.value,
        )))
    }
}
impl<T1, T2, G1, G2, E> Updatable<E> for Sum2<T1, T2, G1, G2>
where
    G1: Updatable<E>,
    G2: Updatable<E>,
    E: Clone + Debug,
{
    fn update(&mut self) -> NothingOrError<E> {
        self.addend1.update()?;
        self.addend2.update()?;
        Ok(())
    }
}
///A stream that subtracts one of its inputs from the other. Returns `Ok(None)` if either input
///does. [`NoneToValue`](converters::NoneToValue) may be of interest if this is not the desired
///behavior.
pub struct DifferenceStream<TM, TS, GM, GS> {
    minuend: GM,
    subtrahend: GS,
    phantom_tm: PhantomData<TM>,
    phantom_ts: PhantomData<TS>,
}
impl<TM, TS, GM, GS> DifferenceStream<TM, TS, GM, GS> {
    ///Constructor for [`DifferenceStream`].
    pub const fn new(minuend: GM, subtrahend: GS) -> Self {
        Self {
            minuend,
            subtrahend,
            phantom_tm: PhantomData,
            phantom_ts: PhantomData,
        }
    }
}
impl<TM, TS, TO, GM, GS, E> Getter<TO, E> for DifferenceStream<TM, TS, GM, GS>
where
    TM: Sub<TS, Output = TO>,
    GM: Getter<TM, E>,
    GS: Getter<TS, E>,
    E: Clone + Debug,
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
                return Ok(None);
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
impl<TM, TS, GM, GS, E> Updatable<E> for DifferenceStream<TM, TS, GM, GS>
where
    GM: Updatable<E>,
    GS: Updatable<E>,
    E: Clone + Debug,
{
    fn update(&mut self) -> NothingOrError<E> {
        self.minuend.update()?;
        self.subtrahend.update()?;
        Ok(())
    }
}
///A stream that multiplies all its inputs sequentially with `MulAssign`. For this stream to return a
///value, all of its inputs must return the `Ok(Some(_))` variant; if an input returns `Ok(None)` or
///`Err(_)`, that value is returned immediately.
///
///If your usecase requires excluding `None` values from the product as opposed to this behavior,
///consider wrapping inputs in [`converters::NoneToValue`] with a value of 1 for the `None` variant.
///
///If you are only multiplying the outputs of two getters or if your input getters are of different
///types, consider using [`Product2`] instead.
pub struct ProductStream<const N: usize, G> {
    factors: [G; N],
}
impl<const N: usize, G> ProductStream<N, G> {
    ///Constructor for [`ProductStream`].
    pub const fn new(factors: [G; N]) -> Self {
        if N < 1 {
            panic!("rrtk::streams::ProductStream must have at least one input stream");
        }
        Self { factors }
    }
}
impl<T, const N: usize, G, E> Getter<T, E> for ProductStream<N, G>
where
    T: MulAssign + Copy,
    G: Getter<T, E>,
    E: Clone + Debug,
{
    fn get(&self) -> Output<T, E> {
        let mut factor_values = [MaybeUninit::uninit(); N];
        for (i, input) in self.factors.iter().enumerate() {
            let gotten = input.get();
            if let Ok(Some(value)) = gotten {
                factor_values[i].write(value);
            } else {
                return gotten;
            }
        }
        unsafe {
            let mut product = factor_values[0].assume_init();
            for factor_value in factor_values.into_iter().skip(1) {
                product *= factor_value.assume_init();
            }
            Ok(Some(product))
        }
    }
}
impl<const N: usize, G, E> Updatable<E> for ProductStream<N, G>
where
    G: Updatable<E>,
    E: Clone + Debug,
{
    fn update(&mut self) -> NothingOrError<E> {
        for getter in &mut self.factors {
            getter.update()?;
        }
        Ok(())
    }
}
///A stream that multiplies two inputs. It should be a bit faster than [`ProductStream`], which
///adds any number of inputs. Returns `Ok(None)` if either of its inputs does. If this is not the
///desired behavior, [`NoneToValue`](converters::NoneToValue) may be of interest.
pub struct Product2<T1, T2, G1, G2> {
    factor1: G1,
    factor2: G2,
    phantom_t1: PhantomData<T1>,
    phantom_t2: PhantomData<T2>,
}
impl<T1, T2, G1, G2> Product2<T1, T2, G1, G2> {
    ///Constructor for [`Product2`].
    pub const fn new(factor1: G1, factor2: G2) -> Self {
        Self {
            factor1,
            factor2,
            phantom_t1: PhantomData,
            phantom_t2: PhantomData,
        }
    }
}
impl<T1, T2, TO, G1, G2, E> Getter<TO, E> for Product2<T1, T2, G1, G2>
where
    T1: Mul<T2, Output = TO>,
    G1: Getter<T1, E>,
    G2: Getter<T2, E>,
    E: Clone + Debug,
{
    fn get(&self) -> Output<TO, E> {
        let x = self.factor1.get()?;
        let x = match x {
            Some(x) => x,
            None => return Ok(None),
        };
        let y = self.factor2.get()?;
        let y = match y {
            Some(y) => y,
            None => return Ok(None),
        };
        Ok(Some(Datum::new(
            core::cmp::max(x.time, y.time),
            x.value * y.value,
        )))
    }
}
impl<T1, T2, G1, G2, E> Updatable<E> for Product2<T1, T2, G1, G2>
where
    G1: Updatable<E>,
    G2: Updatable<E>,
    E: Clone + Debug,
{
    fn update(&mut self) -> NothingOrError<E> {
        self.factor1.update()?;
        self.factor2.update()?;
        Ok(())
    }
}
///A stream that divides one if its inputs by the other. Returns `Ok(None)` if either input does.
pub struct QuotientStream<TD, TS, GD, GS> {
    dividend: GD,
    divisor: GS,
    phantom_td: PhantomData<TD>,
    phantom_ts: PhantomData<TS>,
}
impl<TD, TS, GD, GS> QuotientStream<TD, TS, GD, GS> {
    ///Constructor for [`QuotientStream`].
    pub const fn new(dividend: GD, divisor: GS) -> Self {
        Self {
            dividend,
            divisor,
            phantom_td: PhantomData,
            phantom_ts: PhantomData,
        }
    }
}
impl<TD, TS, TO, GD, GS, E> Getter<TO, E> for QuotientStream<TD, TS, GD, GS>
where
    TD: Div<TS, Output = TO>,
    GD: Getter<TD, E>,
    GS: Getter<TS, E>,
    E: Clone + Debug,
{
    fn get(&self) -> Output<TO, E> {
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
                return Ok(None);
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
impl<TD, TS, GD, GS, E> Updatable<E> for QuotientStream<TD, TS, GD, GS>
where
    GD: Updatable<E>,
    GS: Updatable<E>,
    E: Clone + Debug,
{
    fn update(&mut self) -> NothingOrError<E> {
        self.dividend.update()?;
        self.divisor.update()?;
        Ok(())
    }
}
///A stream that exponentiates one of its inputs to the other. If the exponent input returns
///`Ok(None)`, the base's value is returned directly. Only available with `std`.
#[cfg(feature = "internal_enhanced_float")]
pub struct ExponentStream<GB, GE> {
    base: GB,
    exponent: GE,
}
#[cfg(feature = "internal_enhanced_float")]
impl<GB, GE> ExponentStream<GB, GE> {
    ///Constructor for [`ExponentStream`].
    pub const fn new(base: GB, exponent: GE) -> Self {
        Self { base, exponent }
    }
}
#[cfg(feature = "internal_enhanced_float")]
impl<GB, GE, E> Getter<f32, E> for ExponentStream<GB, GE>
where
    GB: Getter<f32, E>,
    GE: Getter<f32, E>,
    E: Clone + Debug,
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
impl<GB, GE, E> Updatable<E> for ExponentStream<GB, GE>
where
    GB: Updatable<E>,
    GE: Updatable<E>,
    E: Clone + Debug,
{
    fn update(&mut self) -> NothingOrError<E> {
        self.base.update()?;
        self.exponent.update()?;
        Ok(())
    }
}
///A stream that computes the numerical derivative of its input.
pub struct DerivativeStream<T, O, G, E> {
    input: G,
    value: Output<O, E>,
    //doesn't matter if this is an Err or Ok(None) - we can't use it either way if it's not Some
    prev_output: Option<Datum<T>>,
}
impl<T, O, G, E> DerivativeStream<T, O, G, E> {
    ///Constructor for [`DerivativeStream`].
    pub const fn new(input: G) -> Self {
        Self {
            input,
            value: Ok(None),
            prev_output: None,
        }
    }
}
impl<T, O, G, E> Getter<O, E> for DerivativeStream<T, O, G, E>
where
    Self: Updatable<E>,
    Output<O, E>: Clone,
    E: Clone + Debug,
{
    fn get(&self) -> Output<O, E> {
        self.value.clone()
    }
}
impl<T, N1, O, G, E> Updatable<E> for DerivativeStream<T, O, G, E>
where
    T: Copy + Sub<Output = N1>,
    N1: Div<Time, Output = O>,
    G: Getter<T, E>,
    E: Clone + Debug,
{
    fn update(&mut self) -> NothingOrError<E> {
        self.input.update()?;
        let output = self.input.get();
        let output = match output {
            Ok(ok) => ok,
            Err(error) => {
                //XXX: This may change when you standardize when Updatable::update errors.
                //Remove this clone if you don't return the error.
                self.value = Err(error.clone());
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
pub struct IntegralStream<T, O, G, E> {
    input: G,
    value: Output<O, E>,
    prev_output: Option<Datum<T>>,
}
impl<T, O, G, E> IntegralStream<T, O, G, E> {
    ///Constructor for [`IntegralStream`].
    pub const fn new(input: G) -> Self {
        Self {
            input,
            value: Ok(None),
            prev_output: None,
        }
    }
}
impl<T, O, G, E> Getter<O, E> for IntegralStream<T, O, G, E>
where
    Self: Updatable<E>,
    Output<O, E>: Clone,
    E: Clone + Debug,
{
    fn get(&self) -> Output<O, E> {
        self.value.clone()
    }
}
impl<T, O, N1, G, E> Updatable<E> for IntegralStream<T, O, G, E>
where
    T: Copy + Add<Output = N1>,
    Time: Mul<N1, Output = O>,
    O: Copy + stulta::Half + Add<O, Output = O>,
    G: Getter<T, E>,
    E: Clone + Debug,
{
    fn update(&mut self) -> NothingOrError<E> {
        self.input.update()?;
        let output = self.input.get();
        let output = match output {
            Ok(ok) => ok,
            Err(error) => {
                //XXX: This may change when you standardize when Updatable::update errors.
                //Remove this clone if you don't return the error.
                self.value = Err(error.clone());
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
        Ok(())
    }
}
