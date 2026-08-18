extern crate alloc;
use alloc::rc::Rc;
use core::{cell::RefCell, convert::Infallible};
use rrtk::prelude::*;
use rrtk::streams::{converters, math};
use rrtk::{ConstantGetter, NothingOrError, Output, Time};
struct Input {
    time: Time,
}
impl Updatable<Infallible> for Input {
    fn update(&mut self) -> NothingOrError<Infallible> {
        self.time += Time::from_nanoseconds(200_000_000);
        Ok(())
    }
}
static mut CONTROLLED_VALUE: f32 = 0.0;
impl Getter<f32, Infallible> for Input {
    fn get(&self) -> Output<f32, Infallible> {
        Ok(Some(Datum::new(self.time, unsafe { CONTROLLED_VALUE })))
    }
}
fn main() {
    const SETPOINT: f32 = 5.0;
    const KP: f32 = 1.0;
    const KI: f32 = 0.01;
    const KD: f32 = 0.1;
    let input = Input {
        time: Time::from_nanoseconds(0),
    };
    const FAKE_TIME_GETTER: Time = Time::from_nanoseconds(i64::MIN);
    let setpoint = ConstantGetter::new(FAKE_TIME_GETTER, SETPOINT);
    let error = Rc::new(RefCell::new(math::DifferenceStream::new(setpoint, input)));
    let kp = ConstantGetter::new(FAKE_TIME_GETTER, KP);
    let proportional_term =
        Box::new(math::Product2::new(Rc::clone(&error), kp)) as Box<dyn Getter<f32, Infallible>>;
    let integral = converters::PrioritizeA::new(converters::NoneToDefault::new(
        math::IntegralStream::new(Rc::clone(&error)),
        FAKE_TIME_GETTER,
    ));
    let ki = ConstantGetter::new(FAKE_TIME_GETTER, KI);
    let integral_term =
        Box::new(math::Product2::new(integral, ki)) as Box<dyn Getter<f32, Infallible>>;
    let derivative = converters::PrioritizeA::new(converters::NoneToDefault::new(
        math::DerivativeStream::new(Rc::clone(&error)),
        FAKE_TIME_GETTER,
    ));
    let kd = ConstantGetter::new(FAKE_TIME_GETTER, KD);
    let derivative_term =
        Box::new(math::Product2::new(derivative, kd)) as Box<dyn Getter<f32, Infallible>>;
    let mut pid = math::SumStream::new([proportional_term, integral_term, derivative_term]);
    for _ in 0..30 {
        pid.update().unwrap();
        let gotten = pid.get().unwrap().unwrap();
        println!(
            "time: {:?};\tcontrolled value: {:?};\tcommand: {:?}",
            gotten.time.as_nanoseconds(),
            unsafe { CONTROLLED_VALUE },
            gotten.value
        );
        unsafe {
            CONTROLLED_VALUE += 0.7 * gotten.value;
        }
    }
}
