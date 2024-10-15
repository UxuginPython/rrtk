// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2024 UxuginPython
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
    //`dyn Getter`, but doing that fully makes the types look like this:
    //Reference<IntegralStream<DifferenceStream<f32, ConstantGetter<f32, TimeGetterFromGetter<f32, dyn Getter<f32, ()>, ()>, ()>, dyn Getter<f32, ()>, ()>, ()>>
    //Here, to make this example legible, we don't do that. However, if you're actually making
    //something for production, expanding the type fully to avoid dynamic dispatch may be a good
    //idea. It really depends on how much readability you're willing to give up for a small
    //performance boost.
    int: Reference<dyn Getter<f32, ()>>,
    drv: Reference<dyn Getter<f32, ()>>,
    output: SumStream<f32, 3, ()>,
}
#[cfg(feature = "alloc")]
impl StreamPID {
    pub fn new(
        input: Reference<dyn Getter<f32, ()>>,
        setpoint: f32,
        kp: f32,
        ki: f32,
        kd: f32,
    ) -> Self {
        let time_getter = rc_ref_cell_reference(TimeGetterFromGetter::new(input.clone()));
        let setpoint = rc_ref_cell_reference(ConstantGetter::new(time_getter.clone(), setpoint));
        let kp = rc_ref_cell_reference(ConstantGetter::new(time_getter.clone(), kp));
        let ki = rc_ref_cell_reference(ConstantGetter::new(time_getter.clone(), ki));
        let kd = rc_ref_cell_reference(ConstantGetter::new(time_getter.clone(), kd));
        let error = rc_ref_cell_reference(DifferenceStream::new(setpoint.clone(), input.clone()));
        let int = rc_ref_cell_reference(IntegralStream::new(error.clone()));
        let drv = rc_ref_cell_reference(DerivativeStream::new(error.clone()));
        //`ProductStream`'s behavior is to treat all `None` values as 1.0 so that it's as if they
        //were not included. However, this is not what we want with the coefficient. `NoneToValue`
        //is used to convert all `None` values to `Some(0.0)` to effectively exlude them from the
        //final sum.
        let int_zeroer =
            rc_ref_cell_reference(NoneToValue::new(int.clone(), time_getter.clone(), 0.0));
        let drv_zeroer =
            rc_ref_cell_reference(NoneToValue::new(drv.clone(), time_getter.clone(), 0.0));
        let kp_mul = rc_ref_cell_reference(ProductStream::new([
            to_dyn!(Getter<f32, ()>, kp.clone()),
            to_dyn!(Getter<f32, ()>, error.clone()),
        ]));
        let ki_mul = rc_ref_cell_reference(ProductStream::new([
            to_dyn!(Getter<f32, ()>, ki.clone()),
            to_dyn!(Getter<f32, ()>, int_zeroer.clone()),
        ]));
        let kd_mul = rc_ref_cell_reference(ProductStream::new([
            to_dyn!(Getter<f32, ()>, kd.clone()),
            to_dyn!(Getter<f32, ()>, drv_zeroer.clone()),
        ]));
        let output = SumStream::new([
            to_dyn!(Getter<f32, ()>, kp_mul.clone()),
            to_dyn!(Getter<f32, ()>, ki_mul.clone()),
            to_dyn!(Getter<f32, ()>, kd_mul.clone()),
        ]);
        Self {
            int: to_dyn!(Getter<f32, ()>, int),
            drv: to_dyn!(Getter<f32, ()>, drv),
            output: output,
        }
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
        //The other streams used that are not updated here do not need to be updated. Streams like
        //SumStream just calculate their output in the get method since they do not need to store
        //any data beyond the `Reference`s to their inputs. The non-math streams used here work in
        //a similar way.
        self.int.borrow_mut().update()?;
        self.drv.borrow_mut().update()?;
        Ok(())
    }
}
#[cfg(feature = "alloc")]
struct MyStream {
    time: i64,
}
#[cfg(feature = "alloc")]
impl MyStream {
    pub fn new() -> Self {
        Self { time: 0 }
    }
}
//In a real system, obviously, the process variable must be dependent on the command. This is a
//very rudimentary placeholder and a poor model of an actual system. All this example is
//intended to do is to show the PID controller's command values and not model a real system.
#[cfg(feature = "alloc")]
impl Getter<f32, ()> for MyStream {
    fn get(&self) -> Output<f32, ()> {
        Ok(Some(Datum::new(self.time, (self.time / 2) as f32)))
    }
}
#[cfg(feature = "alloc")]
impl Updatable<()> for MyStream {
    fn update(&mut self) -> NothingOrError<()> {
        self.time += 2;
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
    let input = to_dyn!(Getter<f32, ()>, rc_ref_cell_reference(MyStream::new()));
    let mut stream = StreamPID::new(input.clone(), SETPOINT, KP, KI, KD);
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
#[cfg(not(feature = "alloc"))]
fn main() {
    println!("Enable the `alloc` feature to run this example.\nAssuming you're using Cargo, add the `--features alloc` flag to your command.");
}
