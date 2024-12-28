// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2024 UxuginPython
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
        let (value, other_outputs) = outputs.split_at(1);
        unsafe {
            let mut value = value[0].assume_init();
            for i in 0..outputs_filled - 1 {
                value += other_outputs[i].assume_init();
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
pub struct Sum2<
    T: Add<Output = T>,
    G1: Getter<T, E> + ?Sized,
    G2: Getter<T, E> + ?Sized,
    E: Copy + Debug,
> {
    addend1: Reference<G1>,
    addend2: Reference<G2>,
    phantom_t: PhantomData<T>,
    phantom_e: PhantomData<E>,
}
impl<T: Add<Output = T>, G1: Getter<T, E> + ?Sized, G2: Getter<T, E> + ?Sized, E: Copy + Debug>
    Sum2<T, G1, G2, E>
{
    ///Constructor for [`Sum2`].
    pub const fn new(addend1: Reference<G1>, addend2: Reference<G2>) -> Self {
        Self {
            addend1: addend1,
            addend2: addend2,
            phantom_t: PhantomData,
            phantom_e: PhantomData,
        }
    }
}
impl<T: Add<Output = T>, G1: Getter<T, E> + ?Sized, G2: Getter<T, E> + ?Sized, E: Copy + Debug>
    Getter<T, E> for Sum2<T, G1, G2, E>
{
    fn get(&self) -> Output<T, E> {
        let x = self.addend1.borrow().get()?;
        let x = match x {
            Some(x) => x,
            None => return self.addend2.borrow().get(),
        };
        let y = self.addend2.borrow().get()?;
        let y = match y {
            Some(y) => y,
            None => return Ok(Some(x)),
        };
        Ok(Some(x + y))
    }
}
impl<T: Add<Output = T>, G1: Getter<T, E> + ?Sized, G2: Getter<T, E> + ?Sized, E: Copy + Debug>
    Updatable<E> for Sum2<T, G1, G2, E>
{
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
    ///Constructor for [`DifferenceStream`].
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
        let (value, other_outputs) = outputs.split_at(1);
        unsafe {
            let mut value = value[0].assume_init();
            for i in 0..outputs_filled - 1 {
                value *= other_outputs[i].assume_init();
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
pub struct Product2<
    T: Mul<Output = T>,
    G1: Getter<T, E> + ?Sized,
    G2: Getter<T, E> + ?Sized,
    E: Copy + Debug,
> {
    addend1: Reference<G1>,
    addend2: Reference<G2>,
    phantom_t: PhantomData<T>,
    phantom_e: PhantomData<E>,
}
impl<T: Mul<Output = T>, G1: Getter<T, E> + ?Sized, G2: Getter<T, E> + ?Sized, E: Copy + Debug>
    Product2<T, G1, G2, E>
{
    ///Constructor for [`Product2`].
    pub const fn new(addend1: Reference<G1>, addend2: Reference<G2>) -> Self {
        Self {
            addend1: addend1,
            addend2: addend2,
            phantom_t: PhantomData,
            phantom_e: PhantomData,
        }
    }
}
impl<T: Mul<Output = T>, G1: Getter<T, E> + ?Sized, G2: Getter<T, E> + ?Sized, E: Copy + Debug>
    Getter<T, E> for Product2<T, G1, G2, E>
{
    fn get(&self) -> Output<T, E> {
        let x = self.addend1.borrow().get()?;
        let x = match x {
            Some(x) => x,
            None => return self.addend2.borrow().get(),
        };
        let y = self.addend2.borrow().get()?;
        let y = match y {
            Some(y) => y,
            None => return Ok(Some(x)),
        };
        Ok(Some(x * y))
    }
}
impl<T: Mul<Output = T>, G1: Getter<T, E> + ?Sized, G2: Getter<T, E> + ?Sized, E: Copy + Debug>
    Updatable<E> for Product2<T, G1, G2, E>
{
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
    ///Constructor for [`QuotientStream`].
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
#[cfg(any(feature = "std", feature = "libm"))]
pub struct ExponentStream<GB: Getter<f32, E> + ?Sized, GE: Getter<f32, E> + ?Sized, E: Copy + Debug>
{
    base: Reference<GB>,
    exponent: Reference<GE>,
    phantom_e: PhantomData<E>,
}
#[cfg(any(feature = "std", feature = "libm"))]
impl<GB: Getter<f32, E> + ?Sized, GE: Getter<f32, E> + ?Sized, E: Copy + Debug>
    ExponentStream<GB, GE, E>
{
    ///Constructor for [`ExponentStream`].
    pub const fn new(base: Reference<GB>, exponent: Reference<GE>) -> Self {
        Self {
            base: base,
            exponent: exponent,
            phantom_e: PhantomData,
        }
    }
}
#[cfg(any(feature = "std", feature = "libm"))]
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
        let value = powf(base_output.value, exponent_output.value);
        let time = if base_output.time > exponent_output.time {
            base_output.time
        } else {
            exponent_output.time
        };
        Ok(Some(Datum::new(time, value)))
    }
}
#[cfg(any(feature = "std", feature = "libm"))]
impl<GB: Getter<f32, E> + ?Sized, GE: Getter<f32, E> + ?Sized, E: Copy + Debug> Updatable<E>
    for ExponentStream<GB, GE, E>
{
    fn update(&mut self) -> NothingOrError<E> {
        Ok(())
    }
}
///A stream that computes the numerical derivative of its input.
pub struct DerivativeStream<G: Getter<Quantity, E> + ?Sized, E: Copy + Debug> {
    input: Reference<G>,
    value: Output<Quantity, E>,
    //doesn't matter if this is an Err or Ok(None) - we can't use it either way if it's not Some
    prev_output: Option<Datum<Quantity>>,
}
impl<G: Getter<Quantity, E> + ?Sized, E: Copy + Debug> DerivativeStream<G, E> {
    ///Constructor for [`DerivativeStream`].
    pub const fn new(input: Reference<G>) -> Self {
        Self {
            input: input,
            value: Ok(None),
            prev_output: None,
        }
    }
}
impl<G: Getter<Quantity, E> + ?Sized, E: Copy + Debug> Getter<Quantity, E>
    for DerivativeStream<G, E>
{
    fn get(&self) -> Output<Quantity, E> {
        self.value.clone()
    }
}
impl<G: Getter<Quantity, E> + ?Sized, E: Copy + Debug> Updatable<E> for DerivativeStream<G, E> {
    fn update(&mut self) -> NothingOrError<E> {
        let output = self.input.borrow().get();
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
        let value =
            (output.value - prev_output.value) / Quantity::from(output.time - prev_output.time);
        self.value = Ok(Some(Datum::new(output.time, value)));
        self.prev_output = Some(output);
        Ok(())
    }
}
///A stream that computes the trapezoidal numerical integral of its input.
pub struct IntegralStream<G: Getter<Quantity, E> + ?Sized, E: Copy + Debug> {
    input: Reference<G>,
    value: Output<Quantity, E>,
    prev_output: Option<Datum<Quantity>>,
}
impl<G: Getter<Quantity, E> + ?Sized, E: Copy + Debug> IntegralStream<G, E> {
    ///Constructor for [`IntegralStream`].
    pub const fn new(input: Reference<G>) -> Self {
        Self {
            input: input,
            value: Ok(None),
            prev_output: None,
        }
    }
}
impl<G: Getter<Quantity, E> + ?Sized, E: Copy + Debug> Getter<Quantity, E>
    for IntegralStream<G, E>
{
    fn get(&self) -> Output<Quantity, E> {
        self.value.clone()
    }
}
impl<G: Getter<Quantity, E> + ?Sized, E: Copy + Debug> Updatable<E> for IntegralStream<G, E> {
    fn update(&mut self) -> NothingOrError<E> {
        let output = self.input.borrow().get();
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
        let value_addend = Quantity::from(output.time - prev_output.time)
            * (prev_output.value + output.value)
            / Quantity::dimensionless(2.0);
        let value = match &self.value {
            Ok(Some(real_value)) => value_addend + real_value.value,
            _ => value_addend,
        };
        self.value = Ok(Some(Datum::new(output.time, value)));
        self.prev_output = Some(output);
        return Ok(());
    }
}
