use crate::*;
#[cfg(feature = "std")]
use std::collections::vec_deque::VecDeque;
#[cfg(not(feature = "std"))]
use alloc::collections::vec_deque::VecDeque;
pub struct Constant<T, E> {
    time_getter: InputTimeGetter<E>,
    value: T,
}
impl<T, E> Constant<T, E> {
    pub fn new(time_getter: InputTimeGetter<E>, value: T) -> Self {
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
    input: InputStream<T, E>,
}
impl<T: Clone, E> NoneToError<T, E> {
    pub fn new(input: InputStream<T, E>) -> Self {
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
                return Err(Error::FromNone);
            }
        }
    }
    fn update(&mut self) {}
}
pub struct NoneToValue<T, E> {
    input: InputStream<T, E>,
    time_getter: InputTimeGetter<E>,
    none_value: T,
}
impl<T, E> NoneToValue<T, E> {
    pub fn new(input: InputStream<T, E>, time_getter: InputTimeGetter<E>, none_value: T) -> Self {
        Self {
            input: input,
            time_getter: time_getter,
            none_value: none_value,
        }
    }
}
impl<T: Clone, E: Copy + Debug> Stream<T, E> for NoneToValue<T, E> {
    fn get(&self) -> StreamOutput<T, E> {
        let output = self.input.borrow().get()?;
        match output {
            Some(_) => {
                return Ok(output);
            }
            None => {
                return Ok(Some(Datum::new(
                    self.time_getter.borrow().get()?,
                    self.none_value.clone(),
                )))
            }
        }
    }
    fn update(&mut self) {}
}
pub struct SumStream<E> {
    addends: Vec<InputStream<f32, E>>,
}
impl<E> SumStream<E> {
    pub fn new(addends: Vec<InputStream<f32, E>>) -> Self {
        Self { addends: addends }
    }
}
impl<E: Copy + Debug> Stream<f32, E> for SumStream<E> {
    fn get(&self) -> StreamOutput<f32, E> {
        if self.addends.is_empty() {
            return Err(Error::EmptyAddendVec);
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
    minuend: InputStream<f32, E>,
    subtrahend: InputStream<f32, E>,
}
impl<E> DifferenceStream<E> {
    pub fn new(minuend: InputStream<f32, E>, subtrahend: InputStream<f32, E>) -> Self {
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
    factors: Vec<InputStream<f32, E>>,
}
impl<E> ProductStream<E> {
    pub fn new(factors: Vec<InputStream<f32, E>>) -> Self {
        Self { factors: factors }
    }
}
impl<E: Copy + Debug> Stream<f32, E> for ProductStream<E> {
    fn get(&self) -> StreamOutput<f32, E> {
        if self.factors.is_empty() {
            return Err(Error::EmptyFactorVec);
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
    dividend: InputStream<f32, E>,
    divisor: InputStream<f32, E>,
}
impl<E> QuotientStream<E> {
    pub fn new(dividend: InputStream<f32, E>, divisor: InputStream<f32, E>) -> Self {
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
    base: InputStream<f32, E>,
    exponent: InputStream<f32, E>,
}
#[cfg(feature = "std")]
impl<E> ExponentStream<E> {
    pub fn new(base: InputStream<f32, E>, exponent: InputStream<f32, E>) -> Self {
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
    input: InputStream<f32, E>,
    value: StreamOutput<f32, E>,
    //doesn't matter if this is an Err or Ok(None) - we can't use it either way if it's not Some
    prev_output: Option<Datum<f32>>,
}
impl<E: Copy + Debug> DerivativeStream<E> {
    pub fn new(input: InputStream<f32, E>) -> Self {
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
            Ok(_) => {}
            Err(error) => {
                self.value = Err(error);
                self.prev_output = None;
                return;
            }
        }
        let output = output.unwrap();
        match output {
            Some(_) => {}
            None => {
                self.value = Ok(None);
                self.prev_output = None;
                return;
            }
        }
        let output = output.unwrap();
        match self.prev_output {
            Some(_) => {}
            None => {
                self.prev_output = Some(output);
                return;
            }
        }
        let prev_output = self.prev_output.as_ref().unwrap();
        let value = (output.value - prev_output.value) / (output.time - prev_output.time);
        self.value = Ok(Some(Datum::new(output.time, value)));
        self.prev_output = Some(output);
    }
}
pub struct IntegralStream<E: Copy + Debug> {
    input: InputStream<f32, E>,
    value: StreamOutput<f32, E>,
    prev_output: Option<Datum<f32>>,
}
impl<E: Copy + Debug> IntegralStream<E> {
    pub fn new(input: InputStream<f32, E>) -> Self {
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
            Ok(_) => {}
            Err(error) => {
                self.value = Err(error);
                self.prev_output = None;
                return;
            }
        }
        let output = output.unwrap();
        match output {
            Some(_) => {}
            None => {
                self.value = Ok(None);
                self.prev_output = None;
                return;
            }
        }
        let output = output.unwrap();
        match self.prev_output {
            Some(_) => {}
            None => {
                self.prev_output = Some(output);
                return;
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
            + (output.time - prev_output.time) * (prev_output.value + output.value) / 2.0;
        self.value = Ok(Some(Datum::new(output.time, value)));
        self.prev_output = Some(output);
    }
}
pub struct StreamPID<E: Copy + Debug> {
    int: InputStream<f32, E>,
    drv: InputStream<f32, E>,
    output: SumStream<E>,
}
impl<E: Copy + Debug + 'static> StreamPID<E> {
    pub fn new(input: InputStream<f32, E>, setpoint: f32, kp: f32, ki: f32, kd: f32) -> Self {
        let time_getter = make_time_getter_input!(TimeGetterFromStream::new(Rc::clone(&input)), E);
        let setpoint = make_stream_input!(Constant::new(Rc::clone(&time_getter), setpoint), f32, E);
        let kp = make_stream_input!(Constant::new(Rc::clone(&time_getter), kp), f32, E);
        let ki = make_stream_input!(Constant::new(Rc::clone(&time_getter), ki), f32, E);
        let kd = make_stream_input!(Constant::new(Rc::clone(&time_getter), kd), f32, E);
        let error = make_stream_input!(
            DifferenceStream::new(Rc::clone(&setpoint), Rc::clone(&input)),
            f32,
            E
        );
        let int = make_stream_input!(IntegralStream::new(Rc::clone(&error)), f32, E);
        let drv = make_stream_input!(DerivativeStream::new(Rc::clone(&error)), f32, E);
        //`ProductStream`'s behavior is to treat all `None` values as 1.0 so that it's as if they
        //were not included. However, this is not what we want with the coefficient. `NoneToValue`
        //is used to convert all `None` values to `Some(0.0)` to effectively exlude them from the
        //final sum.
        let int_zeroer = make_stream_input!(
            NoneToValue::new(Rc::clone(&int), Rc::clone(&time_getter), 0.0),
            f32,
            E
        );
        let drv_zeroer = make_stream_input!(
            NoneToValue::new(Rc::clone(&drv), Rc::clone(&time_getter), 0.0),
            f32,
            E
        );
        let kp_mul = make_stream_input!(
            ProductStream::new(vec![Rc::clone(&kp), Rc::clone(&error)]),
            f32,
            E
        );
        let ki_mul = make_stream_input!(
            ProductStream::new(vec![Rc::clone(&ki), Rc::clone(&int_zeroer)]),
            f32,
            E
        );
        let kd_mul = make_stream_input!(
            ProductStream::new(vec![Rc::clone(&kd), Rc::clone(&drv_zeroer)]),
            f32,
            E
        );
        let output = SumStream::new(vec![
            Rc::clone(&kp_mul),
            Rc::clone(&ki_mul),
            Rc::clone(&kd_mul),
        ]);
        Self {
            int: Rc::clone(&int),
            drv: Rc::clone(&drv),
            output: output,
        }
    }
}
impl<E: Copy + Debug + 'static> Stream<f32, E> for StreamPID<E> {
    fn get(&self) -> StreamOutput<f32, E> {
        self.output.get()
    }
    fn update(&mut self) {
        self.int.borrow_mut().update();
        self.drv.borrow_mut().update();
    }
}
//https://www.itl.nist.gov/div898/handbook/pmc/section3/pmc324.htm
pub struct EWMAStream<E: Copy + Debug> {
    input: InputStream<f32, E>,
    //As data may not come in at regular intervals as is assumed by a standard EWMA, this value
    //will be multiplied by delta time before being used.
    smoothing_constant: f32,
    value: StreamOutput<f32, E>,
    update_time: Option<f32>,
}
impl<E: Copy + Debug> EWMAStream<E> {
    pub fn new(input: InputStream<f32, E>, smoothing_constant: f32) -> Self {
        Self {
            input: input,
            smoothing_constant: smoothing_constant,
            value: Ok(None),
            update_time: None,
        }
    }
}
impl<E: Copy + Debug> Stream<f32, E> for EWMAStream<E> {
    fn get(&self) -> StreamOutput<f32, E> {
        self.value.clone()
    }
    fn update(&mut self) {
        let output = self.input.borrow().get();
        match output {
            Err(error) => {
                self.value = Err(error);
                self.update_time = None;
                return;
            }
            Ok(None) => {
                match self.value {
                    Err(_) => {
                        self.value = Ok(None);
                        self.update_time = None;
                    }
                    Ok(_) => {}
                }
                return;
            }
            Ok(Some(_)) => {}
        }
        let output = output.unwrap().unwrap();
        match self.value {
            Ok(Some(_)) => {}
            _ => {
                self.value = Ok(Some(output.clone()));
                self.update_time = Some(output.time);
            }
        }
        let prev_value = self.value.as_ref().unwrap().as_ref().unwrap();
        let prev_time = self
            .update_time
            .expect("update_time must be Some if value is");
        let delta_time = output.time - prev_time;
        let value = if delta_time * self.smoothing_constant < 1.0 {
            let value = prev_value.value;
            let value = value - (delta_time * self.smoothing_constant) * value;
            let value = value + (delta_time * self.smoothing_constant) * output.value;
            value
        } else {
            output.value
        };
        self.value = Ok(Some(Datum::new(output.time, value)));
        self.update_time = Some(output.time);
    }
}
//Expect the moving average to be delayed by one update compared to the actual data. This is
//because MovingAverageStream reports the average with the window ending at the timestamp of the
//new input value. Thus, the new input has a timespan of zero within the window when it is first
//received, and so it is weighted as zero and excluded.
pub struct MovingAverageStream<E: Copy + Debug> {
    input: InputStream<f32, E>,
    window: f32,
    value: StreamOutput<f32, E>,
    input_values: VecDeque<Datum<f32>>,
}
impl<E: Copy + Debug> MovingAverageStream<E> {
    pub fn new(input: InputStream<f32, E>, window: f32) -> Self {
        Self {
            input: input,
            window: window,
            value: Ok(None),
            input_values: VecDeque::new(),
        }
    }
}
impl<E: Copy + Debug> Stream<f32, E> for MovingAverageStream<E> {
    fn get(&self) -> StreamOutput<f32, E> {
        self.value.clone()
    }
    fn update(&mut self) {
        let output = self.input.borrow().get();
        println!("output is {:?}", output);
        match output {
            Err(error) => {
                self.value = Err(error);
                self.input_values = VecDeque::new();
                return;
            }
            Ok(None) => {
                match self.value {
                    Err(_) => {
                        self.value = Ok(None);
                        return;
                    }
                    Ok(_) => {}
                }
                return;
            }
            Ok(Some(_)) => {}
        }
        let output = output.unwrap().unwrap();
        let new_time = output.time;
        println!("self.input_values is {:?}", self.input_values);
        match self.input_values.len() {
            0 => {
                self.input_values.push_back(output.clone());
                self.value = Ok(Some(output.clone()));
                println!("self.input_values is {:?}", self.input_values);
                return;
            }
            _ => {}
        }
        println!("self.input_values is {:?}", self.input_values);
        if self.input_values.len() >= 2 {
            while new_time - self.input_values[1].time >= self.window {
                self.input_values.remove(0);
            }
        }
        self.input_values.push_back(output);
        println!("self.input_values is {:?}", self.input_values);
        let mut start_times = Vec::<f32>::with_capacity(self.input_values.len());
        start_times.push(new_time - self.window);
        for i in &self.input_values {
            start_times.push(i.time);
        }
        start_times.remove(1);
        println!("start_times is {:?}", start_times);
        let mut end_times = Vec::<f32>::with_capacity(self.input_values.len());
        for i in &self.input_values {
            end_times.push(i.time);
        }
        end_times.push(new_time);
        end_times.remove(0);
        println!("end_times is {:?}", end_times);
        let mut weights = Vec::<f32>::with_capacity(self.input_values.len());
        for i in 0..self.input_values.len() {
            weights.push(end_times[i] - start_times[i]);
        }
        println!("weights is {:?}", weights);
        let mut weights_sum = 0f32;
        for i in &weights {
            weights_sum += i;
        }
        let mut average = 0f32;
        for i in 0..self.input_values.len() {
            average += self.input_values[i].value * weights[i];
        }
        average /= weights_sum;
        self.value = Ok(Some(Datum::new(new_time, average)));
    }
}
