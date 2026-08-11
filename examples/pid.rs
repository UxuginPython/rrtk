// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2024-2026 UxuginPython
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::rc::Rc;
#[cfg(feature = "alloc")]
use core::cell::RefCell;
#[cfg(feature = "alloc")]
use rrtk::streams::converters::*;
#[cfg(feature = "alloc")]
use rrtk::streams::math::*;
#[cfg(feature = "alloc")]
use rrtk::*;
//Note that RRTK includes the rrtk::streams::control::PIDControllerStream type which should be a
//bit faster than this. This example is to show how streams can be chained for more complex data
//processing and control theory. Using the PID controller shown here in production is discouraged.
#[cfg(feature = "alloc")]
struct StreamPID {
    //It is possible to avoid dynamic dispatch by using the actual stream types instead of
    //`dyn Getter`, but doing that fully makes the types very complicated.
    //Here, to make this example legible, we don't do that. However, if you're actually making
    //something for production, expanding the type fully to avoid dynamic dispatch may be a good
    //idea. It really depends on how much readability you're willing to give up for a small
    //performance boost.
    //Also note that you should almost always use a more specific error type than (). This example
    //is not focused on error handling.
    output: SumStream<3, Box<dyn Getter<f32, ()>>>,
}
#[cfg(feature = "alloc")]
impl StreamPID {
    pub fn new(
        //One would generally prefer to use a type parameter to Rc<RefCell<dyn Getter<_, _>>>. This
        //example uses the latter for simplicity.
        input: Rc<RefCell<dyn Getter<f32, ()>>>,
        setpoint: f32,
        kp: f32,
        ki: f32,
        kd: f32,
    ) -> Self {
        let time_getter = Rc::new(RefCell::new(TimeGetterFromGetter::new(input.clone(), ())));
        let setpoint = ConstantGetter::new(time_getter.clone(), setpoint);
        let kp = ConstantGetter::new(time_getter.clone(), kp);
        let ki = ConstantGetter::new(time_getter.clone(), ki);
        let kd = ConstantGetter::new(time_getter.clone(), kd);
        let error = Rc::new(RefCell::new(DifferenceStream::new(setpoint, input.clone())));
        //Notice how one can directly use a Getter as an input for a stream OR put it in an
        //Rc<RefCell<T>> first if multiple things need access to it. Rc<RefCell<T>> passes through
        //the Getter implementation of its referent. Using an Rc<RefCell<T>>> or similar to a getter
        //as a stream input is often necessary and not discouraged, but where possible, directly
        //using the getter will be slightly faster.
        //Rc<RefCell<T>>, Arc<RwLock<T>>, Arc<Mutex<T>>, and Box<T> all have this functionality,
        //and similar functionality can be achieved with *mut T, *const RwLock<T>, and *const Mutex<T>
        //through PointerDereferencer. There are also implementations for Updatable (which is
        //required for Getter), Settable, and TimeGetter. Chronology and Device work a bit
        //differently.
        let int = Rc::new(RefCell::new(IntegralStream::new(Rc::clone(&error))));
        let drv = Rc::new(RefCell::new(DerivativeStream::new(Rc::clone(&error))));
        //`ProductStream`'s behavior is to treat all `None` values as 1.0 so that it's as if they
        //were not included. However, this is not what we want with the coefficient. `NoneToValue`
        //is used to convert all `None` values to `Some(0.0)` to effectively exlude them from the
        //final sum.
        let int_zeroer = NoneToValue::new(int.clone(), time_getter.clone(), 0.0);
        let drv_zeroer = NoneToValue::new(drv.clone(), time_getter.clone(), 0.0);
        let kp_mul = Product2::new(kp, error.clone());
        let ki_mul = Product2::new(ki, int_zeroer);
        let kd_mul = Product2::new(kd, drv_zeroer);
        let output = SumStream::new([
            Box::new(kp_mul) as Box<dyn Getter<f32, ()>>,
            Box::new(ki_mul) as Box<dyn Getter<f32, ()>>,
            Box::new(kd_mul) as Box<dyn Getter<f32, ()>>,
        ]);
        Self { output: output }
    }
}
#[cfg(feature = "alloc")]
impl Getter<f32, ()> for StreamPID {
    fn get(&self) -> Output<f32, ()> {
        self.output.get()
    }
}
#[cfg(feature = "alloc")]
impl Updatable<()> for StreamPID {
    fn update(&mut self) -> NothingOrError<()> {
        //All builtin streams update their inputs. If you make your own, it is strongly recommended
        //that they do the same.
        self.output.update()?;
        Ok(())
    }
}
#[cfg(feature = "alloc")]
struct MyStream {
    time: Time,
}
#[cfg(feature = "alloc")]
impl MyStream {
    pub fn new() -> Self {
        Self { time: Time::ZERO }
    }
}
//In a real system, obviously, the process variable must be dependent on the command. This is a
//very rudimentary placeholder and a poor model of an actual system. All this example is
//intended to do is to show the PID controller's command values and not model a real system by
//assuming a constant velocity.
#[cfg(feature = "alloc")]
impl Getter<f32, ()> for MyStream {
    fn get(&self) -> Output<f32, ()> {
        Ok(Some(Datum::new(
            self.time,
            self.time.as_seconds_f32() * 0.5,
        )))
    }
}
#[cfg(feature = "alloc")]
impl Updatable<()> for MyStream {
    fn update(&mut self) -> NothingOrError<()> {
        self.time += Time::from_nanoseconds(2_000_000_000);
        Ok(())
    }
}
#[cfg(feature = "alloc")]
fn main() {
    const SETPOINT: f32 = 5.0;
    const KP: f32 = 1.0;
    const KI: f32 = 0.01;
    const KD: f32 = 0.1;
    println!("PID Controller using RRTK Streams");
    println!("kp = {:?}; ki = {:?}; kd = {:?}", KP, KI, KD);
    let input = Rc::new(RefCell::new(MyStream::new())) as Rc<RefCell<dyn Getter<f32, ()>>>;
    let mut stream = StreamPID::new(input.clone(), SETPOINT, KP, KI, KD);
    stream.update().unwrap();
    println!(
        "time: {:?}; setpoint: {:?}; process: {:?}; command: {:?}",
        stream.get().unwrap().unwrap().time.as_nanoseconds(),
        SETPOINT,
        input.borrow().get().unwrap().unwrap().value,
        stream.get().unwrap().unwrap().value
    );
    for _ in 0..6 {
        input.borrow_mut().update().unwrap();
        stream.update().unwrap();
        println!(
            "time: {:?}; setpoint: {:?}; process: {:?}; command: {:?}",
            stream.get().unwrap().unwrap().time,
            SETPOINT,
            input.borrow().get().unwrap().unwrap().value,
            stream.get().unwrap().unwrap().value
        );
    }
}
#[cfg(not(feature = "alloc"))]
fn main() {
    println!(
        "Enable the `alloc` feature to run this example.\nAssuming you're using Cargo, add the `--features alloc` flag to your command."
    );
}
