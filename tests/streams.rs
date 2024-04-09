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
