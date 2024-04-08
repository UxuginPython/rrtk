use crate::*;
#[cfg(feature = "std")]
use std::cell::RefCell;
#[cfg(feature = "std")]
use std::fmt::Debug;
#[cfg(feature = "std")]
use std::rc::Rc;
#[cfg(not(feature = "std"))]
extern crate alloc;
#[cfg(not(feature = "std"))]
use alloc::rc::Rc;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;
#[cfg(not(feature = "std"))]
use core::cell::RefCell;
#[cfg(not(feature = "std"))]
use core::fmt::Debug;
pub mod errors;
pub type StreamOutput<T, E> = Result<Option<Datum<T>>, errors::StreamError<E>>;
pub trait TimeGetter<E: Copy + Debug> {
    fn get(&self) -> Result<f32, errors::StreamError<E>>;
    fn update(&mut self);
}
pub struct TimeGetterFromStream<T: Clone, E> {
    elevator: NoneToError<T, E>,
}
impl<T: Clone, E> TimeGetterFromStream<T, E> {
    pub fn new(stream: Rc<RefCell<dyn Stream<T, E>>>) -> Self {
        Self {
            elevator: NoneToError::new(Rc::clone(&stream)),
        }
    }
}
impl<T: Clone, E: Copy + Debug> TimeGetter<E> for TimeGetterFromStream<T, E> {
    fn get(&self) -> Result<f32, errors::StreamError<E>> {
        let output = self.elevator.get()?;
        let output = output.expect("`NoneToError` made all `Ok(None)`s into `Err(_)`s, and `?` returned all `Err(_)`s, so we're sure this is now an `Ok(Some(_))`.");
        return Ok(output.time);
    }
    fn update(&mut self) {}
}
pub trait Stream<T: Clone, E: Copy + Debug> {
    fn get(&self) -> StreamOutput<T, E>;
    fn update(&mut self);
}
pub struct Constant<T, E> {
    time_getter: Rc<RefCell<dyn TimeGetter<E>>>,
    value: T,
}
impl<T, E> Constant<T, E> {
    pub fn new(time_getter: Rc<RefCell<dyn TimeGetter<E>>>, value: T) -> Self {
        Self {
            time_getter: time_getter,
            value: value,
        }
    }
}
impl<T: Clone, E: Copy + Debug> Stream<T, E> for Constant<T, E> {
    fn get(&self) -> StreamOutput<T, E> {
        let time = self.time_getter.borrow().get()?;
        Ok(Some(Datum::new(time, self.value.clone())))
    }
    fn update(&mut self) {}
}
pub struct NoneToError<T: Clone, E> {
    input: Rc<RefCell<dyn Stream<T, E>>>,
}
impl<T: Clone, E> NoneToError<T, E> {
    pub fn new(input: Rc<RefCell<dyn Stream<T, E>>>) -> Self {
        Self { input: input }
    }
}
impl<T: Clone, E: Copy + Debug> Stream<T, E> for NoneToError<T, E> {
    fn get(&self) -> StreamOutput<T, E> {
        let output = self.input.borrow().get()?;
        match output {
            Some(_) => {
                return Ok(output);
            }
            None => {
                return Err(errors::StreamError::FromNone);
            }
        }
    }
    fn update(&mut self) {}
}
pub struct SumStream<E> {
    addends: Vec<Rc<RefCell<dyn Stream<f32, E>>>>,
}
impl<E> SumStream<E> {
    pub fn new(addends: Vec<Rc<RefCell<dyn Stream<f32, E>>>>) -> Self {
        Self { addends: addends }
    }
}
impl<E: Copy + Debug> Stream<f32, E> for SumStream<E> {
    fn get(&self) -> StreamOutput<f32, E> {
        if self.addends.is_empty() {
            return Err(errors::StreamError::EmptyAddendVec);
        }
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
    minuend: Rc<RefCell<dyn Stream<f32, E>>>,
    subtrahend: Rc<RefCell<dyn Stream<f32, E>>>,
}
impl<E> DifferenceStream<E> {
    pub fn new(
        minuend: Rc<RefCell<dyn Stream<f32, E>>>,
        subtrahend: Rc<RefCell<dyn Stream<f32, E>>>,
    ) -> Self {
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
pub struct ProductStream<E> {
    factors: Vec<Rc<RefCell<dyn Stream<f32, E>>>>,
}
impl<E> ProductStream<E> {
    pub fn new(factors: Vec<Rc<RefCell<dyn Stream<f32, E>>>>) -> Self {
        Self { factors: factors }
    }
}
impl<E: Copy + Debug> Stream<f32, E> for ProductStream<E> {
    fn get(&self) -> StreamOutput<f32, E> {
        if self.factors.is_empty() {
            return Err(errors::StreamError::EmptyFactorVec);
        }
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
    dividend: Rc<RefCell<dyn Stream<f32, E>>>,
    divisor: Rc<RefCell<dyn Stream<f32, E>>>,
}
impl<E> QuotientStream<E> {
    pub fn new(
        dividend: Rc<RefCell<dyn Stream<f32, E>>>,
        divisor: Rc<RefCell<dyn Stream<f32, E>>>,
    ) -> Self {
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
    base: Rc<RefCell<dyn Stream<f32, E>>>,
    exponent: Rc<RefCell<dyn Stream<f32, E>>>,
}
#[cfg(feature = "std")]
impl<E> ExponentStream<E> {
    pub fn new(
        base: Rc<RefCell<dyn Stream<f32, E>>>,
        exponent: Rc<RefCell<dyn Stream<f32, E>>>,
    ) -> Self {
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
    input: Rc<RefCell<dyn Stream<f32, E>>>,
    value: StreamOutput<f32, E>,
    //doesn't matter if this is an Err or Ok(None) - we can't use it either way if it's not Some
    prev_output: Option<Datum<f32>>,
}
impl<E: Copy + Debug> DerivativeStream<E> {
    pub fn new(input: Rc<RefCell<dyn Stream<f32, E>>>) -> Self {
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
            Ok(_) => {},
            Err(error) => {self.value = Err(error); self.prev_output = None; return;},
        }
        let output = output.unwrap();
        match output {
            Some(_) => {},
            None => {self.value = Ok(None); self.prev_output = None; return;}
        }
        let output = output.unwrap();
        match self.prev_output {
            Some(_) => {},
            None => {self.prev_output = Some(output); return;}
        }
        let prev_output = self.prev_output.as_ref().unwrap();
        let value = (output.value - prev_output.value) / (output.time - prev_output.time);
        self.value = Ok(Some(Datum::new(output.time, value)));
        self.prev_output = Some(output);
    }
}
pub struct IntegralStream<E: Copy + Debug> { //Luke, you're an idiot. Lucy, you're not. Brunk, be
                                             //careful.
    input: Rc<RefCell<dyn Stream<f32, E>>>,
    value: StreamOutput<f32, E>,
    prev_output: Option<Datum<f32>>,
}
impl<E: Copy + Debug> IntegralStream<E> {
    pub fn new(input: Rc<RefCell<dyn Stream<f32, E>>>) -> Self {
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
            Ok(_) => {},
            Err(error) => {self.value = Err(error); self.prev_output = None; return;}
        }
        let output = output.unwrap();
        match output {
            Some(_) => {},
            None => {self.value = Ok(None); self.prev_output = None; return;}
        }
        let output = output.unwrap();
        match self.prev_output {
            Some(_) => {},
            None => {self.prev_output = Some(output); return;}
        }
        let prev_output = self.prev_output.as_ref().unwrap();
        let prev_value = match &self.value {
            Ok(option_value) => match option_value {
                Some(real_value) => real_value.value,
                None => 0.0,
            },
            Err(_) => 0.0,
        };
        let value = prev_value + (output.time - prev_output.time) * (prev_output.value + output.value) / 2.0;
        self.value = Ok(Some(Datum::new(output.time, value)));
        self.prev_output = Some(output);
    }
}
pub struct StreamPIDController<E: Copy + Debug> {
    sum: SumStream<E>,
}
impl<E: Copy + Debug> StreamPIDController<E> {
    pub fn new(input: Rc<RefCell<dyn Stream<f32, E>>>, kp: f32, ki: f32, kd: f32) -> Self {
        let time_getter = Rc::new(RefCell::new(TimeGetterFromStream::new(Rc::clone(&input))));
        let kp = Rc::new(RefCell::new(Constant::new(Rc::clone(&time_getter), kp)));
        let ki = Rc::new(RefCell::new(Constant::new(Rc::clone(&time_getter), ki)));
        let kd = Rc::new(RefCell::new(Constant::new(Rc::clone(&time_getter), kd)));
        let kp_mul = Rc::new(RefCell::new(ProductStream::new(vec![Rc::clone(&input), Rc::clone(&kp)])));
        let ki_mul = Rc::new(RefCell::new(ProductStream::new(vec![Rc::clone(&input), Rc::clone(&ki)])));
        let kd_mul = Rc::new(RefCell::new(ProductStream::new(vec![Rc::clone(&input), Rc::clone(&kd)])));
        let sum = SumStream::new(vec![Rc::clone(&kp_mul), Rc::clone(&ki_mul), Rc::clone(&kd_mul)]);
        Self {
            sum: sum,
        }
    }
}
