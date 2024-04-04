use crate::*;
#[cfg(feature = "std")]
use std::rc::Rc;
#[cfg(not(feature = "std"))]
extern crate alloc;
#[cfg(not(feature = "std"))]
use alloc::rc::Rc;
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
    time_getter: Rc<dyn TimeGetter>,
    value: T,
}
impl<T: Clone> Stream<T> for Constant<T> {
    fn get(&self) -> Datum<T> {
        let time = self.time_getter.get();
        Datum::new(time, self.value.clone())
    }
    fn update(&mut self) {}
}
pub struct SumStream {
    addends: Vec<Rc<dyn Stream<f32>>>,
}
impl SumStream {
    pub fn new(addends: Vec<Rc<dyn Stream<f32>>>) -> Self {
        Self { addends: addends }
    }
}
impl Stream<f32> for SumStream {
    fn get(&self) -> Datum<f32> {
        let mut outputs = Vec::new();
        for i in &self.addends {
            outputs.push(i.get());
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
    minuend: Rc<dyn Stream<f32>>,
    subtrahend: Rc<dyn Stream<f32>>,
}
impl DifferenceStream {
    pub fn new(minuend: Rc<dyn Stream<f32>>, subtrahend: Rc<dyn Stream<f32>>) -> Self {
        Self {
            minuend: minuend,
            subtrahend: subtrahend,
        }
    }
}
impl Stream<f32> for DifferenceStream {
    fn get(&self) -> Datum<f32> {
        let minuend_output = self.minuend.get();
        let subtrahend_output = self.subtrahend.get();
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
    factors: Vec<Rc<dyn Stream<f32>>>,
}
impl ProductStream {
    pub fn new(factors: Vec<Rc<dyn Stream<f32>>>) -> Self {
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
            outputs.push(i.get());
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
    dividend: Rc<dyn Stream<f32>>,
    divisor: Rc<dyn Stream<f32>>,
}
impl QuotientStream {
    pub fn new(dividend: Rc<dyn Stream<f32>>, divisor: Rc<dyn Stream<f32>>) -> Self {
        Self {
            dividend: dividend,
            divisor: divisor,
        }
    }
}
impl Stream<f32> for QuotientStream {
    fn get(&self) -> Datum<f32> {
        let dividend_output = self.dividend.get();
        let divisor_output = self.divisor.get();
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
