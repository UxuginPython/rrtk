use crate::*;
#[cfg(feature = "std")]
use std::rc::Rc;
#[cfg(feature = "std")]
use std::cell::RefCell;
#[cfg(not(feature = "std"))]
extern crate alloc;
#[cfg(not(feature = "std"))]
use alloc::rc::Rc;
#[cfg(not(feature = "std"))]
use core::cell::RefCell;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;
#[derive(Debug)]
pub struct StreamError {}
pub trait TimeGetter {
    fn get(&self) -> f32;
    fn update(&mut self);
}
pub trait Stream<T: Clone> {
    fn get(&self) -> Result<Datum<T>, StreamError>;
    fn update(&mut self);
}
pub struct Constant<T> {
    time_getter: Rc<RefCell<dyn TimeGetter>>,
    value: T,
}
impl<T> Constant<T> {
    pub fn new(time_getter: Rc<RefCell<dyn TimeGetter>>, value: T) -> Self {
        Self {
            time_getter: time_getter,
            value: value,
        }
    }
}
impl<T: Clone> Stream<T> for Constant<T> {
    fn get(&self) -> Result<Datum<T>, StreamError> {
        let time = self.time_getter.borrow().get();
        Ok(Datum::new(time, self.value.clone()))
    }
    fn update(&mut self) {}
}
pub struct SumStream {
    addends: Vec<Rc<RefCell<dyn Stream<f32>>>>,
}
impl SumStream {
    pub fn new(addends: Vec<Rc<RefCell<dyn Stream<f32>>>>) -> Self {
        Self { addends: addends }
    }
}
impl Stream<f32> for SumStream {
    fn get(&self) -> Result<Datum<f32>, StreamError> {
        let mut outputs = Vec::new();
        for i in &self.addends {
            outputs.push(i.borrow().get());
        }
        let mut value = 0.0;
        for i in &outputs {
            match i {
                Ok(output) => {value += output.value},
                Err(_) => {return Err(StreamError {})},
            }
        }
        let mut time = 0.0;
        for i in &outputs {
            if i.as_ref().expect("caught by match in for loop").time > time {
                time = i.as_ref().unwrap().time;
            }
        }
        Ok(Datum::new(time, value))
    }
    fn update(&mut self) {}
}
pub struct DifferenceStream {
    minuend: Rc<RefCell<dyn Stream<f32>>>,
    subtrahend: Rc<RefCell<dyn Stream<f32>>>,
}
impl DifferenceStream {
    pub fn new(minuend: Rc<RefCell<dyn Stream<f32>>>, subtrahend: Rc<RefCell<dyn Stream<f32>>>) -> Self {
        Self {
            minuend: minuend,
            subtrahend: subtrahend,
        }
    }
}
impl Stream<f32> for DifferenceStream {
    fn get(&self) -> Result<Datum<f32>, StreamError> {
        let minuend_output = self.minuend.borrow().get();
        let subtrahend_output = self.subtrahend.borrow().get();
        if !(minuend_output.is_ok() && subtrahend_output.is_ok()) {
            return Err(StreamError {})
        }
        let minuend_output = minuend_output.unwrap();
        let subtrahend_output = subtrahend_output.unwrap();
        let value = minuend_output.value - subtrahend_output.value;
        let time = if minuend_output.time > subtrahend_output.time {
            minuend_output.time
        } else {
            subtrahend_output.time
        };
        Ok(Datum::new(time, value))
    }
    fn update(&mut self) {}
}
pub struct ProductStream {
    factors: Vec<Rc<RefCell<dyn Stream<f32>>>>,
}
impl ProductStream {
    pub fn new(factors: Vec<Rc<RefCell<dyn Stream<f32>>>>) -> Self {
        Self { factors: factors }
    }
}
impl Stream<f32> for ProductStream {
    fn get(&self) -> Result<Datum<f32>, StreamError> {
        if self.factors.is_empty() {
            return Err(StreamError {});
        }
        let mut outputs = Vec::new();
        for i in &self.factors {
            outputs.push(i.borrow().get());
        }
        let mut value = 1.0;
        for i in &outputs {
            match i {
                Ok(output) => {value *= output.value;},
                Err(_) => {return Err(StreamError {});}
            }
        }
        let mut time = 0.0;
        for i in &outputs {
            if i.as_ref().unwrap().time > time {
                time = i.as_ref().unwrap().time;
            }
        }
        Ok(Datum::new(time, value))
    }
    fn update(&mut self) {}
}
pub struct QuotientStream {
    dividend: Rc<RefCell<dyn Stream<f32>>>,
    divisor: Rc<RefCell<dyn Stream<f32>>>,
}
impl QuotientStream {
    pub fn new(dividend: Rc<RefCell<dyn Stream<f32>>>, divisor: Rc<RefCell<dyn Stream<f32>>>) -> Self {
        Self {
            dividend: dividend,
            divisor: divisor,
        }
    }
}
impl Stream<f32> for QuotientStream {
    fn get(&self) -> Result<Datum<f32>, StreamError> {
        let dividend_output = self.dividend.borrow().get();
        let divisor_output = self.divisor.borrow().get();
        if !(dividend_output.is_ok() && divisor_output.is_ok()) {
            return Err(StreamError {});
        }
        let dividend_output = dividend_output.unwrap();
        let divisor_output = divisor_output.unwrap();
        let value = dividend_output.value / divisor_output.value;
        let time = if dividend_output.time > divisor_output.time {
            dividend_output.time
        } else {
            divisor_output.time
        };
        Ok(Datum::new(time, value))
    }
    fn update(&mut self) {}
}
#[cfg(feature = "std")]
pub struct ExponentStream {
    base: Rc<RefCell<dyn Stream<f32>>>,
    exponent: Rc<RefCell<dyn Stream<f32>>>,
}
#[cfg(feature = "std")]
impl ExponentStream {
    pub fn new(base: Rc<RefCell<dyn Stream<f32>>>, exponent: Rc<RefCell<dyn Stream<f32>>>) -> Self {
        Self {
            base: base,
            exponent: exponent,
        }
    }
}
#[cfg(feature = "std")]
impl Stream<f32> for ExponentStream {
    fn get(&self) -> Result<Datum<f32>, StreamError> {
        let base_output = self.base.borrow().get();
        let exponent_output = self.exponent.borrow().get();
        if !(base_output.is_ok() && exponent_output.is_ok()) {
            return Err(StreamError {});
        }
        let base_output = base_output.unwrap();
        let exponent_output = exponent_output.unwrap();
        let value = base_output.value.powf(exponent_output.value);
        let time = if base_output.time > exponent_output.time {
            base_output.time
        } else {
            exponent_output.time
        };
        Ok(Datum::new(time, value))
    }
    fn update(&mut self) {}
}
