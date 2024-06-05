use std::rc::Rc;
use std::cell::RefCell;
use std::fmt::Debug;
use rrtk::*;
use rrtk::streams::*;
use rrtk::streams::math::*;
use rrtk::streams::converters::*;
pub struct StreamPID<E: Copy + Debug> {
    int: InputGetter<f32, E>,
    drv: InputGetter<f32, E>,
    output: SumStream<3, E>,
}
impl<E: Copy + Debug + 'static> StreamPID<E> {
    pub fn new(input: InputGetter<f32, E>, setpoint: f32, kp: f32, ki: f32, kd: f32) -> Self {
        let time_getter = make_input_time_getter!(TimeGetterFromStream::new(Rc::clone(&input)), E);
        let setpoint = make_input_getter!(Constant::new(Rc::clone(&time_getter), setpoint), f32, E);
        let kp = make_input_getter!(Constant::new(Rc::clone(&time_getter), kp), f32, E);
        let ki = make_input_getter!(Constant::new(Rc::clone(&time_getter), ki), f32, E);
        let kd = make_input_getter!(Constant::new(Rc::clone(&time_getter), kd), f32, E);
        let error = make_input_getter!(
            DifferenceStream::new(Rc::clone(&setpoint), Rc::clone(&input)),
            f32,
            E
        );
        let int = make_input_getter!(IntegralStream::new(Rc::clone(&error)), f32, E);
        let drv = make_input_getter!(DerivativeStream::new(Rc::clone(&error)), f32, E);
        //`ProductStream`'s behavior is to treat all `None` values as 1.0 so that it's as if they
        //were not included. However, this is not what we want with the coefficient. `NoneToValue`
        //is used to convert all `None` values to `Some(0.0)` to effectively exlude them from the
        //final sum.
        let int_zeroer = make_input_getter!(
            NoneToValue::new(Rc::clone(&int), Rc::clone(&time_getter), 0.0),
            f32,
            E
        );
        let drv_zeroer = make_input_getter!(
            NoneToValue::new(Rc::clone(&drv), Rc::clone(&time_getter), 0.0),
            f32,
            E
        );
        let kp_mul = make_input_getter!(
            ProductStream::new([Rc::clone(&kp), Rc::clone(&error)]),
            f32,
            E
        );
        let ki_mul = make_input_getter!(
            ProductStream::new([Rc::clone(&ki), Rc::clone(&int_zeroer)]),
            f32,
            E
        );
        let kd_mul = make_input_getter!(
            ProductStream::new([Rc::clone(&kd), Rc::clone(&drv_zeroer)]),
            f32,
            E
        );
        let output = SumStream::new([Rc::clone(&kp_mul), Rc::clone(&ki_mul), Rc::clone(&kd_mul)]);
        Self {
            int: Rc::clone(&int),
            drv: Rc::clone(&drv),
            output: output,
        }
    }
}
impl<E: Copy + Debug + 'static> Getter<f32, E> for StreamPID<E> {
    fn get(&self) -> Output<f32, E> {
        self.output.get()
    }
}
impl<E: Copy + Debug + 'static> Updatable<E> for StreamPID<E> {
    fn update(&mut self) -> NothingOrError<E> {
        self.int.borrow_mut().update()?;
        self.drv.borrow_mut().update()?;
        Ok(())
    }
}
fn main() {
    #[derive(Clone, Copy, Debug)]
    struct DummyError;
    struct DummyStream {
        time: i64,
    }
    impl DummyStream {
        pub fn new() -> Self {
            Self { time: 0 }
        }
    }
    impl Getter<f32, DummyError> for DummyStream {
        fn get(&self) -> Output<f32, DummyError> {
            Ok(Some(Datum::new(self.time, (self.time / 2) as f32)))
        }
    }
    impl Updatable<DummyError> for DummyStream {
        fn update(&mut self) -> NothingOrError<DummyError> {
            self.time += 2;
            Ok(())
        }
    }
    let input = make_input_getter!(DummyStream::new(), f32, DummyError);
    let mut stream = StreamPID::new(Rc::clone(&input), 5.0, 1.0, 0.01, 0.1);
    stream.update().unwrap();
    assert_eq!(stream.get().unwrap().unwrap().time, 0);
    assert_eq!(stream.get().unwrap().unwrap().value, 5.0);
    input.borrow_mut().update().unwrap();
    stream.update().unwrap();
    assert_eq!(stream.get().unwrap().unwrap().time, 2);
    assert_eq!(stream.get().unwrap().unwrap().value, 4.04);
}
