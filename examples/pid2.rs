// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2024-2026 UxuginPython
#[cfg(feature = "alloc")]
mod example {
    //Note that RRTK includes streams::control::PIDControllerStream, which should be faster than
    //the PID controller demonstrated here.
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
            //In real code, you should never determine time like this. Instead, you should use a
            //real system API for getting time, probably by creating a type implementing
            //TimeGetter.
            self.time += Time::from_nanoseconds(200_000_000);
            Ok(())
        }
    }
    //Of course, in a real system, you never have access to a perfect value like this; there is
    //always some level of noise and some level of latency. For simplicity, this example does not
    //account for those.
    static mut CONTROLLED_VALUE: f32 = 0.0;
    impl Getter<f32, Infallible> for Input {
        fn get(&self) -> Output<f32, Infallible> {
            Ok(Some(Datum::new(self.time, unsafe { CONTROLLED_VALUE })))
        }
    }
    pub fn main() {
        const SETPOINT: f32 = 5.0;
        //This PID controller is intentionally not perfectly tuned. Try changing these values and
        //seeing how the output changes. In fact, there's a way to tune the controller so that it
        //reaches the setpoint in just one cycle! Hint: look at where CONTROLLED_VALUE is modified.
        //(Of course, this is not true in a typical system. It's a consequence of how this example
        //is written.)
        const KP: f32 = 1.0;
        const KI: f32 = 0.01;
        const KD: f32 = 0.1;
        let input = Input {
            time: Time::from_nanoseconds(0),
        };
        //The Time type implements TimeGetter for a quick and dirty way of satisfying requirements.
        //This implementation should be thought of in a similar way to unwrap(), as a nice shortcut
        //for simple and quick testing that's generally not recommended for production use.
        //
        //The reason we use i64::MIN nanoseconds, the earliest possible timestamp, is because RRTK
        //always prefers the newer of two timestamps. This value lets the code always use the
        //timestamp originally from Input without needing to Rc<RefCell<_>> it everywhere.
        const FAKE_TIME_GETTER: Time = Time::from_nanoseconds(i64::MIN);
        let setpoint = ConstantGetter::new(FAKE_TIME_GETTER, SETPOINT);
        //As you can see, several RRTK traits including Getter and Updatable are passed through
        //Box, Rc<RefCell<_>>, and a few other smart pointers. This is often necessary either to
        //use one Getter as input for multiple streams or to use dyn to make differently typed
        //Getters behave as the same type. The unsafe-to-construct PointerDereferencer type
        //provides similar functionality for raw pointers; see its documentation for more
        //information.
        let error = Rc::new(RefCell::new(math::DifferenceStream::new(setpoint, input)));
        let kp = ConstantGetter::new(FAKE_TIME_GETTER, KP);
        let proportional_term = Box::new(math::Product2::new(Rc::clone(&error), kp))
            as Box<dyn Getter<f32, Infallible>>;
        //The integral and derivative streams can't immediately return values because they require
        //multiple readings of their inputs at different times. SumStream returns None if any of
        //its inputs do (assuming none of them error). However, for out PID controller, it's better
        //to let the proportional term start acting immediately, before the integral and derivative
        //can be computed. Thus, we use NoneToDefault to replace Ok(None) values from the integral
        //and derivative with values of 0.0, which do not affect the final sum. We then apply
        //PrioritizeA to simplify some error handling.
        //
        //You can usually replace a NoneToDefault with a NoneToValue of the default value. Try it
        //by changing NoneToDefault to NoneToValue and adding an argument of 0.0_f32 to the
        //constructor.
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
                //Try changing how CONTROLLED_VALUE is modified and seeing what happens to the
                //output. You'll find that plants like this one where the controlled value is
                //proportional to the integral of the command work best.
                CONTROLLED_VALUE += 0.7 * gotten.value;
            }
        }
    }
}
#[cfg(feature = "alloc")]
fn main() {
    example::main();
}
#[cfg(not(feature = "alloc"))]
fn main() {
    eprintln!(
        "Enable the `alloc` feature to run this example.\nAssuming you're using Cargo, add `--features alloc` to your command."
    );
}
