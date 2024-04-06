use crate::*;
#[cfg(feature = "std")]
use std::rc::Rc;
#[cfg(feature = "std")]
use std::cell::RefCell;
#[cfg(feature = "std")]
use std::fmt::Debug;
#[cfg(not(feature = "std"))]
extern crate alloc;
#[cfg(not(feature = "std"))]
use alloc::rc::Rc;
#[cfg(not(feature = "std"))]
use core::cell::RefCell;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;
#[cfg(not(feature = "std"))]
use core::fmt::Debug;
pub mod errors;
pub trait TimeGetter {
    fn get(&self) -> f32;
    fn update(&mut self);
}
pub trait Stream<T: Clone, E: Copy + Debug> {
    fn get(&self) -> Result<Datum<T>, errors::StreamError<E>>;
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
impl<T: Clone, E: Copy + Debug> Stream<T, E> for Constant<T> {
    fn get(&self) -> Result<Datum<T>, errors::StreamError<E>> {
        let time = self.time_getter.borrow().get();
        Ok(Datum::new(time, self.value.clone()))
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
    fn get(&self) -> Result<Datum<f32>, errors::StreamError<E>> {
        if self.addends.is_empty() {
            return Err(errors::StreamError::EmptyAddendVec)
        }
        let mut outputs = Vec::new();
        for i in &self.addends {
            outputs.push(i.borrow().get());
        }
        let mut value = 0.0;
        for i in &outputs {
            match i {
                Ok(output) => {value += output.value},
                Err(error) => {return Err(*error);},
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
pub struct DifferenceStream<E> {
    minuend: Rc<RefCell<dyn Stream<f32, E>>>,
    subtrahend: Rc<RefCell<dyn Stream<f32, E>>>,
}
impl<E> DifferenceStream<E> {
    pub fn new(minuend: Rc<RefCell<dyn Stream<f32, E>>>, subtrahend: Rc<RefCell<dyn Stream<f32, E>>>) -> Self {
        Self {
            minuend: minuend,
            subtrahend: subtrahend,
        }
    }
}
impl<E: Copy + Debug> Stream<f32, E> for DifferenceStream<E> {
    fn get(&self) -> Result<Datum<f32>, errors::StreamError<E>> {
        let minuend_output = self.minuend.borrow().get();
        let subtrahend_output = self.subtrahend.borrow().get();
        match minuend_output {
            Ok(_) => {},
            Err(error) => {return Err(error)},
        }
        match subtrahend_output {
            Ok(_) => {},
            Err(error) => {return Err(error)},
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
pub struct ProductStream<E> {
    factors: Vec<Rc<RefCell<dyn Stream<f32, E>>>>,
}
impl<E> ProductStream<E> {
    pub fn new(factors: Vec<Rc<RefCell<dyn Stream<f32, E>>>>) -> Self {
        Self { factors: factors }
    }
}
impl<E: Copy + Debug> Stream<f32, E> for ProductStream<E> {
    fn get(&self) -> Result<Datum<f32>, errors::StreamError<E>> {
        if self.factors.is_empty() {
            return Err(errors::StreamError::EmptyFactorVec);
        }
        let mut outputs = Vec::new();
        for i in &self.factors {
            outputs.push(i.borrow().get());
        }
        let mut value = 1.0;
        for i in &outputs {
            match i {
                Ok(output) => {value *= output.value;},
                Err(error) => {return Err(*error);}
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
pub struct QuotientStream<E> {
    dividend: Rc<RefCell<dyn Stream<f32, E>>>,
    divisor: Rc<RefCell<dyn Stream<f32, E>>>,
}
impl<E> QuotientStream<E> {
    pub fn new(dividend: Rc<RefCell<dyn Stream<f32, E>>>, divisor: Rc<RefCell<dyn Stream<f32, E>>>) -> Self {
        Self {
            dividend: dividend,
            divisor: divisor,
        }
    }
}
impl<E: Copy + Debug> Stream<f32, E> for QuotientStream<E> {
    fn get(&self) -> Result<Datum<f32>, errors::StreamError<E>> {
        let dividend_output = self.dividend.borrow().get();
        let divisor_output = self.divisor.borrow().get();
        match dividend_output {
            Ok(_) => {},
            Err(error) => {return Err(error);},
        }
        match divisor_output {
            Ok(_) => {},
            Err(error) => {return Err(error);},
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
pub struct ExponentStream<E> {
    base: Rc<RefCell<dyn Stream<f32, E>>>,
    exponent: Rc<RefCell<dyn Stream<f32, E>>>,
}
#[cfg(feature = "std")]
impl<E> ExponentStream<E> {
    pub fn new(base: Rc<RefCell<dyn Stream<f32, E>>>, exponent: Rc<RefCell<dyn Stream<f32, E>>>) -> Self {
        Self {
            base: base,
            exponent: exponent,
        }
    }
}
#[cfg(feature = "std")]
impl<E: Copy + Debug> Stream<f32, E> for ExponentStream<E> {
    fn get(&self) -> Result<Datum<f32>, errors::StreamError<E>> {
        let base_output = self.base.borrow().get();
        let exponent_output = self.exponent.borrow().get();
        match base_output {
            Ok(_) => {},
            Err(error) => {return Err(error);}
        }
        match exponent_output {
            Ok(_) => {},
            Err(error) => {return Err(error);}
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
