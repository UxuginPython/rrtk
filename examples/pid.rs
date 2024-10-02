// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2024 UxuginPython
use rrtk::streams::converters::*;
use rrtk::streams::math::*;
use rrtk::*;
use std::rc::Rc;
//Note that RRTK includes the rrtk::streams::control::PIDControllerStream type which should be a
//bit faster than this. This example is to show how streams can be chained for more complex data
//processing and control theory. Using the PID controller shown here in production is discouraged.
struct StreamPID {
    int: InputGetter<f32, ()>,
    drv: InputGetter<f32, ()>,
    output: SumStream<f32, 3, ()>,
}
impl StreamPID {
    pub fn new(input: InputGetter<f32, ()>, setpoint: f32, kp: f32, ki: f32, kd: f32) -> Self {
        let time_getter = make_input_time_getter(TimeGetterFromGetter::new(Rc::clone(&input)));
        let setpoint = make_input_getter(ConstantGetter::new(Rc::clone(&time_getter), setpoint));
        let kp = make_input_getter(ConstantGetter::new(Rc::clone(&time_getter), kp));
        let ki = make_input_getter(ConstantGetter::new(Rc::clone(&time_getter), ki));
        let kd = make_input_getter(ConstantGetter::new(Rc::clone(&time_getter), kd));
        let error = make_input_getter(DifferenceStream::new(
            Rc::clone(&setpoint),
            Rc::clone(&input),
        ));
        let int = make_input_getter(IntegralStream::new(Rc::clone(&error)));
        let drv = make_input_getter(DerivativeStream::new(Rc::clone(&error)));
        //`ProductStream`'s behavior is to treat all `None` values as 1.0 so that it's as if they
        //were not included. However, this is not what we want with the coefficient. `NoneToValue`
        //is used to convert all `None` values to `Some(0.0)` to effectively exlude them from the
        //final sum.
        let int_zeroer = make_input_getter(NoneToValue::new(
            Rc::clone(&int),
            Rc::clone(&time_getter),
            0.0,
        ));
        let drv_zeroer = make_input_getter(NoneToValue::new(
            Rc::clone(&drv),
            Rc::clone(&time_getter),
            0.0,
        ));
        let kp_mul = make_input_getter(ProductStream::new([Rc::clone(&kp), Rc::clone(&error)]));
        let ki_mul =
            make_input_getter(ProductStream::new([Rc::clone(&ki), Rc::clone(&int_zeroer)]));
        let kd_mul =
            make_input_getter(ProductStream::new([Rc::clone(&kd), Rc::clone(&drv_zeroer)]));
        let output = SumStream::new([Rc::clone(&kp_mul), Rc::clone(&ki_mul), Rc::clone(&kd_mul)]);
        Self {
            int: Rc::clone(&int),
            drv: Rc::clone(&drv),
            output: output,
        }
    }
}
impl Getter<f32, ()> for StreamPID {
    fn get(&self) -> Output<f32, ()> {
        self.output.get()
    }
}
impl Updatable<()> for StreamPID {
    fn update(&mut self) -> NothingOrError<()> {
        //The other streams used that are not updated here do not need to be updated. Streams like
        //SumStream just calculate their output in the get method since they do not need to store
        //any data beyond the `Rc`s to their inputs. The non-math streams used here work in a
        //similar way.
        self.int.borrow_mut().update()?;
        self.drv.borrow_mut().update()?;
        Ok(())
    }
}
struct MyStream {
    time: i64,
}
impl MyStream {
    pub fn new() -> Self {
        Self { time: 0 }
    }
}
//In a real system, obviously, the process variable must be dependent on the command. This is a
//very rudimentary placeholder and a poor model of an actual system. All this example is
//intended to do is to show the PID controller's command values and not model a real system.
impl Getter<f32, ()> for MyStream {
    fn get(&self) -> Output<f32, ()> {
        Ok(Some(Datum::new(self.time, (self.time / 2) as f32)))
    }
}
impl Updatable<()> for MyStream {
    fn update(&mut self) -> NothingOrError<()> {
        self.time += 2;
        Ok(())
    }
}
fn main() {
    const SETPOINT: f32 = 5.0;
    const KP: f32 = 1.0;
    const KI: f32 = 0.01;
    const KD: f32 = 0.1;
    println!("PID Controller using RRTK Streams");
    println!("kp = {:?}; ki = {:?}; kd = {:?}", KP, KI, KD);
    let input = make_input_getter(MyStream::new());
    let mut stream = StreamPID::new(Rc::clone(&input), SETPOINT, KP, KI, KD);
    stream.update().unwrap();
    println!(
        "time: {:?}; setpoint: {:?}; process: {:?}; command: {:?}",
        stream.get().unwrap().unwrap().time,
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
