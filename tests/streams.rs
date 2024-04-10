use rrtk::*;
use rrtk::streams::*;
#[cfg(feature = "std")]
use std::fmt::Debug;
#[cfg(feature = "std")]
use std::rc::Rc;
#[cfg(feature = "std")]
use std::cell::RefCell;
#[cfg(not(feature = "std"))]
extern crate alloc;
#[cfg(not(feature = "std"))]
use core::fmt::Debug;
#[cfg(not(feature = "std"))]
use alloc::rc::Rc;
#[cfg(not(feature = "std"))]
use core::cell::RefCell;
#[test]
fn time_getter_from_stream() {
    struct DummyStream {
        time: f32,
    }
    impl DummyStream {
        pub fn new() -> Self {
            Self {
                time: 0.0,
            }
        }
    }
    impl<E: Copy + Debug> Stream<f32, E> for DummyStream {
        fn get(&self) -> StreamOutput<f32, E> {
            Ok(Some(Datum::new(self.time, 0.0)))
        }
        fn update(&mut self) {
            self.time += 1.0;
        }
    }
    let stream = Rc::new(RefCell::new(Box::new(DummyStream::new()) as Box<dyn Stream<f32, u8>>));
    let time_getter = TimeGetterFromStream::new(Rc::clone(&stream));
    stream.borrow_mut().update();
    assert_eq!(time_getter.get().unwrap(), 1.0);
}
#[test]
fn make_stream_input_() {
    #[derive(Clone, Copy, Debug)]
    struct Nothing;
    struct DummyStream {
        time: f32,
    }
    impl DummyStream {
        pub fn new() -> Self {
            Self {
                time: 0.0,
            }
        }
    }
    impl<E: Copy + Debug> Stream<f32, E> for DummyStream {
        fn get(&self) -> StreamOutput<f32, E> {
            Ok(Some(Datum::new(self.time, 0.0)))
        }
        fn update(&mut self) {
            self.time += 1.0;
        }
    }
    let tg_stream = make_stream_input!(DummyStream::new(), f32, Nothing);
    let time_getter = make_time_getter_input!(TimeGetterFromStream::new(Rc::clone(&tg_stream)), Nothing);
    let stream = Constant::new(Rc::clone(&time_getter), 20u8);
    assert_eq!(stream.get().unwrap().unwrap().value, 20);
    tg_stream.borrow_mut().update();
    assert_eq!(stream.get().unwrap().unwrap().value, 20);
}
#[test]
fn constant() {
    #[derive(Clone, Copy, Debug)]
    struct Nothing;
    struct DummyStream {
        time: f32,
    }
    impl DummyStream {
        pub fn new() -> Self {
            Self {
                time: 0.0,
            }
        }
    }
    impl<E: Copy + Debug> Stream<f32, E> for DummyStream {
        fn get(&self) -> StreamOutput<f32, E> {
            Ok(Some(Datum::new(self.time, 0.0)))
        }
        fn update(&mut self) {
            self.time += 1.0;
        }
    }
    let tg_stream = Rc::new(RefCell::new(Box::new(DummyStream::new()) as Box<dyn Stream<f32, Nothing>>));
    let time_getter = Rc::new(RefCell::new(Box::new(TimeGetterFromStream::new(Rc::clone(&tg_stream))) as Box<dyn TimeGetter<Nothing>>));
    let mut stream = Constant::new(Rc::clone(&time_getter), 20u8);
    assert_eq!(stream.get().unwrap().unwrap().value, 20);
    tg_stream.borrow_mut().update();
    assert_eq!(stream.get().unwrap().unwrap().value, 20);
    stream.update();
    assert_eq!(stream.get().unwrap().unwrap().value, 20);
}
#[test]
fn none_to_error() {
    #[derive(Clone, Copy, Debug)]
    struct Nothing;
    struct DummyStream {
        index: u8,
    }
    impl DummyStream {
        pub fn new() -> Self {
            Self {
                index: 0,
            }
        }
    }
    impl Stream<f32, Nothing> for DummyStream {
        fn get(&self) -> StreamOutput<f32, Nothing> {
            if self.index == 1 {
                return Ok(None);
            } else if self.index == 2 {
                return Err(errors::StreamError::Other(Nothing));
            }
            return Ok(Some(Datum::new(0.0, 0.0)));
        }
        fn update(&mut self) {
            self.index += 1;
        }
    }
    let input = Rc::new(RefCell::new(Box::new(DummyStream::new()) as Box<dyn Stream<f32, Nothing>>));
    let stream = NoneToError::new(Rc::clone(&input));
    match stream.get() {
        Ok(option) => match option {
            Some(_) => {},
            None => {panic!("should not have None");}
        },
        Err(_) => {panic!("should not have Err now");}
    }
    input.borrow_mut().update();
    match stream.get() {
        Ok(_) => {panic!("should return Err");},
        Err(errors::StreamError::FromNone) => {},
        Err(_) => {panic!("should be FromNone");}
    }
    input.borrow_mut().update();
    match stream.get() {
        Ok(_) => {panic!("should return Err");},
        Err(errors::StreamError::FromNone) => {panic!("should return Nothing error");},
        Err(_) => {},
    }
}
#[test]
fn sum_stream() {
    #[derive(Clone, Copy, Debug)]
    struct Nothing;
    struct ErroringStream {
        index: u8,
    }
    impl ErroringStream {
        pub fn new() -> Self {
            Self {
                index: 0,
            }
        }
    }
    impl Stream<f32, Nothing> for ErroringStream {
        fn get(&self) -> StreamOutput<f32, Nothing> {
            if self.index == 0 {
                return Err(errors::StreamError::Other(Nothing));
            } else if self.index == 1 {
                return Ok(None);
            } else {
                return Ok(Some(Datum::new(2.0, 1.0)));
            }
        }
        fn update(&mut self) {
            self.index += 1;
        }
    }
    struct NormalStream;
    impl NormalStream {
        pub fn new() -> Self {
            Self {}
        }
    }
    impl Stream<f32, Nothing> for NormalStream {
        fn get(&self) -> StreamOutput<f32, Nothing> {
            Ok(Some(Datum::new(1.0, 1.0)))
        }
        fn update(&mut self) {}
    }
    let erroring = Rc::new(RefCell::new(Box::new(ErroringStream::new()) as Box<dyn Stream<f32, Nothing>>));
    let normal = Rc::new(RefCell::new(Box::new(NormalStream::new()) as Box<dyn Stream<f32, Nothing>>));
    let stream = SumStream::new(vec![Rc::clone(&erroring), Rc::clone(&normal)]);
    match stream.get() {
        Ok(_) => {panic!("error not propagated")},
        Err(_) => {},
    }
    //normal does not need update
    erroring.borrow_mut().update();
    assert_eq!(stream.get().unwrap().unwrap().time, 1.0);
    assert_eq!(stream.get().unwrap().unwrap().value, 1.0);
    erroring.borrow_mut().update();
    assert_eq!(stream.get().unwrap().unwrap().time, 2.0);
    assert_eq!(stream.get().unwrap().unwrap().value, 2.0);
}
#[test]
fn difference_stream() {
    #[derive(Clone, Copy, Debug)]
    struct DummyError;
    struct Stream1 {
        index: u8,
    }
    impl Stream1 {
        pub fn new() -> Self {
            Self {
                index: 0,
            }
        }
    }
    impl Stream<f32, DummyError> for Stream1 {
        fn get(&self) -> StreamOutput<f32, DummyError> {
            if self.index == 0 || self.index == 1 || self.index == 2 {
                return Err(errors::StreamError::Other(DummyError));
            } else if self.index == 3 || self.index == 4 || self.index == 5 {
                return Ok(None);
            }
            return Ok(Some(Datum::new(1.0, 10.0)));
        }
        fn update(&mut self) {
            self.index += 1;
        }
    }
    struct Stream2 {
        index: u8,
    }
    impl Stream2 {
        pub fn new() -> Self {
            Self {
                index: 0,
            }
        }
    }
    impl Stream<f32, DummyError> for Stream2 {
        fn get(&self) -> StreamOutput<f32, DummyError> {
            if self.index == 0 || self.index == 3 || self.index == 6 {
                return Err(errors::StreamError::Other(DummyError));
            } else if self.index == 1 || self.index == 4 || self.index == 7 {
                return Ok(None);
            }
            return Ok(Some(Datum::new(2.0, 3.0)));
        }
        fn update(&mut self) {
            self.index += 1;
        }
    }
    let stream1 = Rc::new(RefCell::new(Box::new(Stream1::new()) as Box<dyn Stream<f32, DummyError>>));
    let stream2 = Rc::new(RefCell::new(Box::new(Stream2::new()) as Box<dyn Stream<f32, DummyError>>));
    let stream = DifferenceStream::new(Rc::clone(&stream1), Rc::clone(&stream2));
    //Err, Err
    match stream.get() {
        Ok(_) => {panic!();}
        Err(_) => {}
    }
    stream1.borrow_mut().update();
    stream2.borrow_mut().update();
    //Err, None
    match stream.get() {
        Ok(_) => {panic!();}
        Err(_) => {}
    }
    stream1.borrow_mut().update();
    stream2.borrow_mut().update();
    //Err, Some
    match stream.get() {
        Ok(_) => {panic!();}
        Err(_) => {}
    }
    stream1.borrow_mut().update();
    stream2.borrow_mut().update();
    //None, Err
    match stream.get() {
        Ok(_) => {panic!();}
        Err(_) => {}
    }
    stream1.borrow_mut().update();
    stream2.borrow_mut().update();
    //None, None
    match stream.get() {
        Ok(Some(_)) => {panic!();}
        Ok(None) => {}
        Err(_) => {panic!();}
    }
    stream1.borrow_mut().update();
    stream2.borrow_mut().update();
    //None, Some
    match stream.get() {
        Ok(Some(_)) => {panic!();}
        Ok(None) => {}
        Err(_) => {panic!();}
    }
    stream1.borrow_mut().update();
    stream2.borrow_mut().update();
    //Some, Err
    match stream.get() {
        Ok(_) => {panic!();}
        Err(_) => {}
    }
    stream1.borrow_mut().update();
    stream2.borrow_mut().update();
    //Some, None
    match stream.get() {
        Ok(Some(x)) => {
            assert_eq!(x.time, 1.0);
            assert_eq!(x.value, 10.0);
        }
        Ok(None) => {panic!();}
        Err(_) => {panic!();}
    }
    stream1.borrow_mut().update();
    stream2.borrow_mut().update();
    //Some, Some
    match stream.get() {
        Ok(Some(x)) => {
            assert_eq!(x.time, 2.0);
            assert_eq!(x.value, 7.0);
        }
        Ok(None) => {panic!();}
        Err(_) => {panic!();}
    }
}
#[test]
fn product_stream() {
    #[derive(Clone, Copy, Debug)]
    struct Nothing;
    struct ErroringStream {
        index: u8,
    }
    impl ErroringStream {
        pub fn new() -> Self {
            Self {
                index: 0,
            }
        }
    }
    impl Stream<f32, Nothing> for ErroringStream {
        fn get(&self) -> StreamOutput<f32, Nothing> {
            if self.index == 0 {
                return Err(errors::StreamError::Other(Nothing));
            } else if self.index == 1 {
                return Ok(None);
            } else {
                return Ok(Some(Datum::new(2.0, 3.0)));
            }
        }
        fn update(&mut self) {
            self.index += 1;
        }
    }
    struct NormalStream;
    impl NormalStream {
        pub fn new() -> Self {
            Self {}
        }
    }
    impl Stream<f32, Nothing> for NormalStream {
        fn get(&self) -> StreamOutput<f32, Nothing> {
            Ok(Some(Datum::new(1.0, 5.0)))
        }
        fn update(&mut self) {}
    }
    let erroring = Rc::new(RefCell::new(Box::new(ErroringStream::new()) as Box<dyn Stream<f32, Nothing>>));
    let normal = Rc::new(RefCell::new(Box::new(NormalStream::new()) as Box<dyn Stream<f32, Nothing>>));
    let stream = ProductStream::new(vec![Rc::clone(&erroring), Rc::clone(&normal)]);
    match stream.get() {
        Ok(_) => {panic!("error not propagated")},
        Err(_) => {},
    }
    //normal does not need update
    erroring.borrow_mut().update();
    assert_eq!(stream.get().unwrap().unwrap().time, 1.0);
    assert_eq!(stream.get().unwrap().unwrap().value, 5.0);
    erroring.borrow_mut().update();
    assert_eq!(stream.get().unwrap().unwrap().time, 2.0);
    assert_eq!(stream.get().unwrap().unwrap().value, 15.0);
}
#[test]
fn quotient_stream() {
    #[derive(Clone, Copy, Debug)]
    struct DummyError;
    struct Stream1 {
        index: u8,
    }
    impl Stream1 {
        pub fn new() -> Self {
            Self {
                index: 0,
            }
        }
    }
    impl Stream<f32, DummyError> for Stream1 {
        fn get(&self) -> StreamOutput<f32, DummyError> {
            if self.index == 0 || self.index == 1 || self.index == 2 {
                return Err(errors::StreamError::Other(DummyError));
            } else if self.index == 3 || self.index == 4 || self.index == 5 {
                return Ok(None);
            }
            return Ok(Some(Datum::new(1.0, 12.0)));
        }
        fn update(&mut self) {
            self.index += 1;
        }
    }
    struct Stream2 {
        index: u8,
    }
    impl Stream2 {
        pub fn new() -> Self {
            Self {
                index: 0,
            }
        }
    }
    impl Stream<f32, DummyError> for Stream2 {
        fn get(&self) -> StreamOutput<f32, DummyError> {
            if self.index == 0 || self.index == 3 || self.index == 6 {
                return Err(errors::StreamError::Other(DummyError));
            } else if self.index == 1 || self.index == 4 || self.index == 7 {
                return Ok(None);
            }
            return Ok(Some(Datum::new(2.0, 3.0)));
        }
        fn update(&mut self) {
            self.index += 1;
        }
    }
    let stream1 = Rc::new(RefCell::new(Box::new(Stream1::new()) as Box<dyn Stream<f32, DummyError>>));
    let stream2 = Rc::new(RefCell::new(Box::new(Stream2::new()) as Box<dyn Stream<f32, DummyError>>));
    let stream = QuotientStream::new(Rc::clone(&stream1), Rc::clone(&stream2));
    //Err, Err
    match stream.get() {
        Ok(_) => {panic!();}
        Err(_) => {}
    }
    stream1.borrow_mut().update();
    stream2.borrow_mut().update();
    //Err, None
    match stream.get() {
        Ok(_) => {panic!();}
        Err(_) => {}
    }
    stream1.borrow_mut().update();
    stream2.borrow_mut().update();
    //Err, Some
    match stream.get() {
        Ok(_) => {panic!();}
        Err(_) => {}
    }
    stream1.borrow_mut().update();
    stream2.borrow_mut().update();
    //None, Err
    match stream.get() {
        Ok(_) => {panic!();}
        Err(_) => {}
    }
    stream1.borrow_mut().update();
    stream2.borrow_mut().update();
    //None, None
    match stream.get() {
        Ok(Some(_)) => {panic!();}
        Ok(None) => {}
        Err(_) => {panic!();}
    }
    stream1.borrow_mut().update();
    stream2.borrow_mut().update();
    //None, Some
    match stream.get() {
        Ok(Some(_)) => {panic!();}
        Ok(None) => {}
        Err(_) => {panic!();}
    }
    stream1.borrow_mut().update();
    stream2.borrow_mut().update();
    //Some, Err
    match stream.get() {
        Ok(_) => {panic!();}
        Err(_) => {}
    }
    stream1.borrow_mut().update();
    stream2.borrow_mut().update();
    //Some, None
    match stream.get() {
        Ok(Some(x)) => {
            assert_eq!(x.time, 1.0);
            assert_eq!(x.value, 12.0);
        }
        Ok(None) => {panic!();}
        Err(_) => {panic!();}
    }
    stream1.borrow_mut().update();
    stream2.borrow_mut().update();
    //Some, Some
    match stream.get() {
        Ok(Some(x)) => {
            assert_eq!(x.time, 2.0);
            assert_eq!(x.value, 4.0);
        }
        Ok(None) => {panic!();}
        Err(_) => {panic!();}
    }
}
#[test]
#[cfg(feature = "std")]
fn exponent_stream() {
    #[derive(Clone, Copy, Debug)]
    struct DummyError;
    struct Stream1 {
        index: u8,
    }
    impl Stream1 {
        pub fn new() -> Self {
            Self {
                index: 0,
            }
        }
    }
    impl Stream<f32, DummyError> for Stream1 {
        fn get(&self) -> StreamOutput<f32, DummyError> {
            if self.index == 0 || self.index == 1 || self.index == 2 {
                return Err(errors::StreamError::Other(DummyError));
            } else if self.index == 3 || self.index == 4 || self.index == 5 {
                return Ok(None);
            }
            return Ok(Some(Datum::new(1.0, 5.0)));
        }
        fn update(&mut self) {
            self.index += 1;
        }
    }
    struct Stream2 {
        index: u8,
    }
    impl Stream2 {
        pub fn new() -> Self {
            Self {
                index: 0,
            }
        }
    }
    impl Stream<f32, DummyError> for Stream2 {
        fn get(&self) -> StreamOutput<f32, DummyError> {
            if self.index == 0 || self.index == 3 || self.index == 6 {
                return Err(errors::StreamError::Other(DummyError));
            } else if self.index == 1 || self.index == 4 || self.index == 7 {
                return Ok(None);
            }
            return Ok(Some(Datum::new(2.0, 3.0)));
        }
        fn update(&mut self) {
            self.index += 1;
        }
    }
    let stream1 = Rc::new(RefCell::new(Box::new(Stream1::new()) as Box<dyn Stream<f32, DummyError>>));
    let stream2 = Rc::new(RefCell::new(Box::new(Stream2::new()) as Box<dyn Stream<f32, DummyError>>));
    let stream = ExponentStream::new(Rc::clone(&stream1), Rc::clone(&stream2));
    //Err, Err
    match stream.get() {
        Ok(_) => {panic!();}
        Err(_) => {}
    }
    stream1.borrow_mut().update();
    stream2.borrow_mut().update();
    //Err, None
    match stream.get() {
        Ok(_) => {panic!();}
        Err(_) => {}
    }
    stream1.borrow_mut().update();
    stream2.borrow_mut().update();
    //Err, Some
    match stream.get() {
        Ok(_) => {panic!();}
        Err(_) => {}
    }
    stream1.borrow_mut().update();
    stream2.borrow_mut().update();
    //None, Err
    match stream.get() {
        Ok(_) => {panic!();}
        Err(_) => {}
    }
    stream1.borrow_mut().update();
    stream2.borrow_mut().update();
    //None, None
    match stream.get() {
        Ok(Some(_)) => {panic!();}
        Ok(None) => {}
        Err(_) => {panic!();}
    }
    stream1.borrow_mut().update();
    stream2.borrow_mut().update();
    //None, Some
    match stream.get() {
        Ok(Some(_)) => {panic!();}
        Ok(None) => {}
        Err(_) => {panic!();}
    }
    stream1.borrow_mut().update();
    stream2.borrow_mut().update();
    //Some, Err
    match stream.get() {
        Ok(_) => {panic!();}
        Err(_) => {}
    }
    stream1.borrow_mut().update();
    stream2.borrow_mut().update();
    //Some, None
    match stream.get() {
        Ok(Some(x)) => {
            assert_eq!(x.time, 1.0);
            assert_eq!(x.value, 5.0);
        }
        Ok(None) => {panic!();}
        Err(_) => {panic!();}
    }
    stream1.borrow_mut().update();
    stream2.borrow_mut().update();
    //Some, Some
    match stream.get() {
        Ok(Some(x)) => {
            assert_eq!(x.time, 2.0);
            assert_eq!(x.value, 125.0);
        }
        Ok(None) => {panic!();}
        Err(_) => {panic!();}
    }
}
#[test]
fn derivative_stream() {
    #[derive(Clone, Copy, Debug)]
    struct DummyError;
    struct DummyStream {
        time: f32,
    }
    impl DummyStream {
        pub fn new() -> Self {
            Self {
                time: 0.0,
            }
        }
    }
    impl Stream<f32, DummyError> for DummyStream {
        fn get(&self) -> StreamOutput<f32, DummyError> {
            Ok(Some(Datum::new(self.time * 2.0, self.time * 3.0)))
        }
        fn update(&mut self) {
            self.time += 2.0;
        }
    }
    let input = Rc::new(RefCell::new(Box::new(DummyStream::new()) as Box<dyn Stream<f32, DummyError>>));
    let mut stream = DerivativeStream::new(Rc::clone(&input));
    input.borrow_mut().update();
    stream.update();
    input.borrow_mut().update();
    stream.update();
    assert_eq!(stream.get().unwrap().unwrap().time, 8.0);
    assert_eq!(stream.get().unwrap().unwrap().value, 1.5);
}
#[test]
fn integral_stream() {
    #[derive(Clone, Copy, Debug)]
    struct DummyError;
    struct DummyStream {
        time: f32,
    }
    impl DummyStream {
        pub fn new() -> Self {
            Self {
                time: 0.0,
            }
        }
    }
    impl Stream<f32, DummyError> for DummyStream {
        fn get(&self) -> StreamOutput<f32, DummyError> {
            Ok(Some(Datum::new(self.time, 1.0)))
        }
        fn update(&mut self) {
            self.time += 1.0;
        }
    }
    let input = make_stream_input!(DummyStream::new(), f32, DummyError);
    let mut stream = IntegralStream::new(Rc::clone(&input));
    input.borrow_mut().update();
    stream.update();
    input.borrow_mut().update();
    stream.update();
    assert_eq!(stream.get().unwrap().unwrap().time, 2.0);
    assert_eq!(stream.get().unwrap().unwrap().value, 1.0);
}
