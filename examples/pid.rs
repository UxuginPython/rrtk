// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2024-2025 UxuginPython
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
    //Reference<IntegralStream<DifferenceStream<Quantity, ConstantGetter<Quantity, TimeGetterFromGetter<Quantity, dyn Getter<Quantity, ()>, ()>, ()>, dyn Getter<Quantity, ()>, ()>, ()>>
    //Here, to make this example legible, we don't do that. However, if you're actually making
    //something for production, expanding the type fully to avoid dynamic dispatch may be a good
    //idea. It really depends on how much readability you're willing to give up for a small
    //performance boost.
    int: Reference<dyn Getter<Quantity, ()>>,
    drv: Reference<dyn Getter<Quantity, ()>>,
    pro_float_maker: Reference<dyn Getter<f32, ()>>,
    int_float_maker: Reference<dyn Getter<f32, ()>>,
    drv_float_maker: Reference<dyn Getter<f32, ()>>,
    output: SumStream<f32, 3, ()>,
}
#[cfg(feature = "alloc")]
impl StreamPID {
    pub fn new(
        input: Reference<dyn Getter<Quantity, ()>>,
        setpoint: Quantity,
        kp: Quantity,
        ki: Quantity,
        kd: Quantity,
    ) -> Self {
        let time_getter = rc_ref_cell_reference(TimeGetterFromGetter::new(input.clone()));
        let setpoint = ConstantGetter::new(time_getter.clone(), setpoint);
        let kp = ConstantGetter::new(time_getter.clone(), kp);
        let ki = ConstantGetter::new(time_getter.clone(), ki);
        let kd = ConstantGetter::new(time_getter.clone(), kd);
        let error = rc_ref_cell_reference(DifferenceStream::new(setpoint, input.clone()));
        let int = rc_ref_cell_reference(IntegralStream::new(error.clone()));
        let drv = rc_ref_cell_reference(DerivativeStream::new(error.clone()));
        //`ProductStream`'s behavior is to treat all `None` values as 1.0 so that it's as if they
        //were not included. However, this is not what we want with the coefficient. `NoneToValue`
        //is used to convert all `None` values to `Some(0.0)` to effectively exlude them from the
        //final sum.
        let int_zeroer = NoneToValue::new(
            int.clone(),
            time_getter.clone(),
            Quantity::new(0.0, MILLIMETER),
        );
        let drv_zeroer = NoneToValue::new(
            drv.clone(),
            time_getter.clone(),
            Quantity::new(0.0, MILLIMETER),
        );
        let kp_mul = Product2::new(kp, error.clone());
        //The way a PID controller works necessitates that it adds quantities of different units.
        //Thus, QuantityToFloat streams are required to keep the dimensional analysis system from
        //stopping this.
        let pro_float_maker = rc_ref_cell_reference(QuantityToFloat::new(kp_mul));
        let ki_mul = Product2::new(ki, int_zeroer);
        let int_float_maker = rc_ref_cell_reference(QuantityToFloat::new(ki_mul));
        let kd_mul = Product2::new(kd, drv_zeroer);
        let drv_float_maker = rc_ref_cell_reference(QuantityToFloat::new(kd_mul));
        let output = SumStream::new([
            to_dyn!(Getter<f32, ()>, pro_float_maker.clone()),
            to_dyn!(Getter<f32, ()>, int_float_maker.clone()),
            to_dyn!(Getter<f32, ()>, drv_float_maker.clone()),
        ]);
        Self {
            int: to_dyn!(Getter<Quantity, ()>, int),
            drv: to_dyn!(Getter<Quantity, ()>, drv),
            pro_float_maker: to_dyn!(Getter<f32, ()>, pro_float_maker),
            int_float_maker: to_dyn!(Getter<f32, ()>, int_float_maker),
            drv_float_maker: to_dyn!(Getter<f32, ()>, drv_float_maker),
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
        self.int.update()?;
        self.drv.update()?;
        self.pro_float_maker.update()?;
        self.int_float_maker.update()?;
        self.drv_float_maker.update()?;
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
impl Getter<Quantity, ()> for MyStream {
    fn get(&self) -> Output<Quantity, ()> {
        Ok(Some(Datum::new(
            self.time,
            Quantity::from(self.time) * Quantity::new(0.5, MILLIMETER_PER_SECOND),
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
    const SETPOINT: Quantity = Quantity::new(5.0, MILLIMETER);
    const KP: Quantity = Quantity::dimensionless(1.0);
    const KI: Quantity = Quantity::dimensionless(0.01);
    const KD: Quantity = Quantity::dimensionless(0.1);
    println!("PID Controller using RRTK Streams");
    println!(
        "kp = {:?}; ki = {:?}; kd = {:?}",
        KP.value, KI.value, KD.value
    );
    let input = to_dyn!(Getter<Quantity, ()>, rc_ref_cell_reference(MyStream::new()));
    let mut stream = StreamPID::new(input.clone(), SETPOINT, KP, KI, KD);
    stream.update().unwrap();
    println!(
        "time: {:?}; setpoint: {:?}; process: {:?}; command: {:?}",
        stream.get().unwrap().unwrap().time.as_nanoseconds(),
        SETPOINT.value,
        input.borrow().get().unwrap().unwrap().value.value,
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
