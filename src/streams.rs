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
pub trait TimeGetter {
    fn get(&self) -> f32;
    fn update(&mut self);
}
pub trait Stream<T: Clone> {
    fn get(&self) -> Datum<T>;
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
    fn get(&self) -> Datum<T> {
        let time = self.time_getter.borrow().get();
        Datum::new(time, self.value.clone())
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
    fn get(&self) -> Datum<f32> {
        let mut outputs = Vec::new();
        for i in &self.addends {
            outputs.push(i.borrow().get());
        }
        let mut value = 0.0;
        for i in &outputs {
            value += i.value;
        }
        let mut time = 0.0;
        for i in &outputs {
            if i.time > time {
                time = i.time;
            }
        }
        Datum::new(time, value)
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
    fn get(&self) -> Datum<f32> {
        let minuend_output = self.minuend.borrow().get();
        let subtrahend_output = self.subtrahend.borrow().get();
        let value = minuend_output.value - subtrahend_output.value;
        let time = if minuend_output.time > subtrahend_output.time {
            minuend_output.time
        } else {
            subtrahend_output.time
        };
        Datum::new(time, value)
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
    //FIXME: undefined behavior when `factors` is empty
    fn get(&self) -> Datum<f32> {
        if self.factors.is_empty() {
            todo!();
        }
        let mut outputs = Vec::new();
        for i in &self.factors {
            outputs.push(i.borrow().get());
        }
        let mut value = 1.0;
        for i in &outputs {
            value *= i.value;
        }
        let mut time = 0.0;
        for i in &outputs {
            if i.time > time {
                time = i.time;
            }
        }
        Datum::new(time, value)
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
    fn get(&self) -> Datum<f32> {
        let dividend_output = self.dividend.borrow().get();
        let divisor_output = self.divisor.borrow().get();
        let value = dividend_output.value / divisor_output.value;
        let time = if dividend_output.time > divisor_output.time {
            dividend_output.time
        } else {
            divisor_output.time
        };
        Datum::new(time, value)
    }
    fn update(&mut self) {}
}
