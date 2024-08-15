// SPDX-License-Identifier: LGPL-3.0-only
/*
Copyright 2024 UxuginPython on GitHub

     This file is part of Rust Robotics ToolKit.

    Rust Robotics ToolKit is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, version 3.

    Rust Robotics ToolKit is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Lesser General Public License for more details.

    You should have received a copy of the GNU Lesser General Public License along with Rust Robotics ToolKit. If not, see <https://www.gnu.org/licenses/>.
*/
use rrtk::streams::control::*;
use rrtk::streams::converters::*;
use rrtk::streams::logic::*;
use rrtk::streams::math::*;
use rrtk::streams::*;
use rrtk::*;
#[cfg(feature = "std")]
use std::fmt::Debug;
#[cfg(feature = "std")]
use std::rc::Rc;
#[cfg(not(feature = "std"))]
extern crate alloc;
#[cfg(not(feature = "std"))]
use alloc::rc::Rc;
#[cfg(not(feature = "std"))]
use core::fmt::Debug;
#[test]
fn time_getter_from_stream() {
    struct DummyStream {
        time: i64,
    }
    impl DummyStream {
        pub fn new() -> Self {
            Self { time: 0 }
        }
    }
    impl<E: Copy + Debug> Getter<f32, E> for DummyStream {
        fn get(&self) -> Output<f32, E> {
            Ok(Some(Datum::new(self.time, 0.0)))
        }
    }
    impl<E: Copy + Debug> Updatable<E> for DummyStream {
        fn update(&mut self) -> NothingOrError<E> {
            self.time += 1;
            Ok(())
        }
    }
    let stream: InputGetter<_, ()> = make_input_getter(DummyStream::new());
    let time_getter = TimeGetterFromGetter::new(Rc::clone(&stream));
    stream.borrow_mut().update().unwrap();
    assert_eq!(time_getter.get().unwrap(), 1);
}
#[test]
fn make_input_getter_() {
    struct DummyStream {
        time: i64,
    }
    impl DummyStream {
        pub fn new() -> Self {
            Self { time: 0 }
        }
    }
    impl<E: Copy + Debug> Getter<f32, E> for DummyStream {
        fn get(&self) -> Output<f32, E> {
            Ok(Some(Datum::new(self.time, 0.0)))
        }
    }
    impl<E: Copy + Debug> Updatable<E> for DummyStream {
        fn update(&mut self) -> NothingOrError<E> {
            self.time += 1;
            Ok(())
        }
    }
    let tg_stream: InputGetter<_, ()> = make_input_getter(DummyStream::new());
    let time_getter = make_input_time_getter(TimeGetterFromGetter::new(Rc::clone(&tg_stream)));
    let stream = ConstantGetter::new(Rc::clone(&time_getter), 20u8);
    assert_eq!(stream.get().unwrap().unwrap().value, 20);
    tg_stream.borrow_mut().update().unwrap();
    assert_eq!(stream.get().unwrap().unwrap().value, 20);
}
#[test]
fn expirer() {
    struct DummyStream;
    impl Getter<f32, ()> for DummyStream {
        fn get(&self) -> Output<f32, ()> {
            Ok(Some(Datum::new(0, 0.0)))
        }
    }
    impl Updatable<()> for DummyStream {
        fn update(&mut self) -> NothingOrError<()> {
            Ok(())
        }
    }
    struct DummyTimeGetter {
        time: i64,
    }
    impl TimeGetter<()> for DummyTimeGetter {
        fn get(&self) -> TimeOutput<()> {
            Ok(self.time)
        }
    }
    impl Updatable<()> for DummyTimeGetter {
        fn update(&mut self) -> NothingOrError<()> {
            self.time += 10;
            Ok(())
        }
    }
    let stream = make_input_getter(DummyStream);
    let time_getter = make_input_time_getter(DummyTimeGetter { time: 0 });
    let mut expirer = Expirer::new(stream, Rc::clone(&time_getter), 10);
    expirer.update().unwrap(); //This should do nothing.
    assert_eq!(expirer.get(), Ok(Some(Datum::new(0, 0.0))));
    time_getter.borrow_mut().update().unwrap();
    assert_eq!(expirer.get(), Ok(Some(Datum::new(0, 0.0))));
    time_getter.borrow_mut().update().unwrap();
    assert_eq!(expirer.get(), Ok(None));
}
#[test]
fn expirer_none() {
    struct DummyStream;
    impl Getter<f32, ()> for DummyStream {
        fn get(&self) -> Output<f32, ()> {
            Ok(None)
        }
    }
    impl Updatable<()> for DummyStream {
        fn update(&mut self) -> NothingOrError<()> {
            Ok(())
        }
    }
    struct DummyTimeGetter {
        time: i64,
    }
    impl TimeGetter<()> for DummyTimeGetter {
        fn get(&self) -> TimeOutput<()> {
            Ok(self.time)
        }
    }
    impl Updatable<()> for DummyTimeGetter {
        fn update(&mut self) -> NothingOrError<()> {
            self.time += 10;
            Ok(())
        }
    }
    let stream = make_input_getter(DummyStream);
    let time_getter = make_input_time_getter(DummyTimeGetter { time: 0 });
    let expirer = Expirer::new(stream, Rc::clone(&time_getter), 10);
    assert_eq!(expirer.get(), Ok(None));
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
            Self { index: 0 }
        }
    }
    impl Getter<f32, Nothing> for DummyStream {
        fn get(&self) -> Output<f32, Nothing> {
            if self.index == 1 {
                return Ok(None);
            } else if self.index == 2 {
                return Err(Error::Other(Nothing));
            }
            return Ok(Some(Datum::new(0, 0.0)));
        }
    }
    impl Updatable<Nothing> for DummyStream {
        fn update(&mut self) -> NothingOrError<Nothing> {
            self.index += 1;
            Ok(())
        }
    }
    let input = make_input_getter(DummyStream::new());
    let mut stream = NoneToError::new(Rc::clone(&input));
    stream.update().unwrap(); //This should do nothing.
    match stream.get() {
        Ok(option) => match option {
            Some(_) => {}
            None => {
                panic!("should not have None");
            }
        },
        Err(_) => {
            panic!("should not have Err now");
        }
    }
    input.borrow_mut().update().unwrap();
    match stream.get() {
        Ok(_) => {
            panic!("should return Err");
        }
        Err(Error::FromNone) => {}
        Err(_) => {
            panic!("should be FromNone");
        }
    }
    input.borrow_mut().update().unwrap();
    match stream.get() {
        Ok(_) => {
            panic!("should return Err");
        }
        Err(Error::FromNone) => {
            panic!("should return Nothing error");
        }
        Err(_) => {}
    }
}
#[test]
fn none_to_value() {
    #[derive(Clone, Copy, Debug)]
    struct Nothing;
    struct DummyStream {
        index: u8,
    }
    impl DummyStream {
        pub fn new() -> Self {
            Self { index: 0 }
        }
    }
    impl Getter<f32, Nothing> for DummyStream {
        fn get(&self) -> Output<f32, Nothing> {
            if self.index == 1 {
                return Ok(None);
            } else if self.index == 2 {
                return Err(Error::Other(Nothing));
            }
            return Ok(Some(Datum::new(0, 1.0)));
        }
    }
    impl Updatable<Nothing> for DummyStream {
        fn update(&mut self) -> NothingOrError<Nothing> {
            self.index += 1;
            Ok(())
        }
    }
    struct DummyTimeGetter {
        time: i64,
    }
    impl DummyTimeGetter {
        pub fn new() -> Self {
            Self { time: 0 }
        }
    }
    impl<E: Copy + Debug> TimeGetter<E> for DummyTimeGetter {
        fn get(&self) -> TimeOutput<E> {
            Ok(self.time)
        }
    }
    impl<E: Copy + Debug> Updatable<E> for DummyTimeGetter {
        fn update(&mut self) -> NothingOrError<E> {
            self.time += 1;
            Ok(())
        }
    }
    let input = make_input_getter(DummyStream::new());
    let mut stream = NoneToValue::new(
        Rc::clone(&input),
        make_input_time_getter(DummyTimeGetter::new()),
        2.0,
    );
    stream.update().unwrap(); //This should do nothing.
    match stream.get() {
        Ok(option) => match option {
            Some(datum) => {
                assert_eq!(datum.value, 1.0);
            }
            None => {
                panic!("should return Ok(Some(_)), returned Ok(None)");
            }
        },
        Err(_) => {
            panic!("should return Ok(Some(_)), returned Err(_)");
        }
    }
    input.borrow_mut().update().unwrap();
    match stream.get() {
        Ok(Some(datum)) => {
            assert_eq!(datum.value, 2.0);
        }
        Ok(None) => {
            panic!("should return Ok(Some(_)), returned Ok(None)")
        }
        Err(_) => {
            panic!("should return Ok(Some(_)), returned Err(_)");
        }
    }
    input.borrow_mut().update().unwrap();
    match stream.get() {
        Ok(_) => {
            panic!("should return Err(_), returned Ok(_)");
        }
        Err(_) => {}
    }
}
#[test]
fn acceleration_to_state() {
    struct AccGetter {
        time: i64,
    }
    impl AccGetter {
        fn new() -> Self {
            Self { time: 0 }
        }
    }
    impl Getter<f32, ()> for AccGetter {
        fn get(&self) -> Output<f32, ()> {
            Ok(Some(Datum::new(self.time, 1.0)))
        }
    }
    impl Updatable<()> for AccGetter {
        fn update(&mut self) -> NothingOrError<()> {
            self.time += 1;
            Ok(())
        }
    }
    let acc_getter = make_input_getter(AccGetter::new());
    let mut state_getter = AccelerationToState::new(Rc::clone(&acc_getter));
    let output = state_getter.get();
    assert!(output.unwrap().is_none());
    acc_getter.borrow_mut().update().unwrap();
    state_getter.update().unwrap();
    let output = state_getter.get();
    assert!(output.unwrap().is_none());
    acc_getter.borrow_mut().update().unwrap();
    state_getter.update().unwrap();
    let output = state_getter.get();
    assert!(output.unwrap().is_none());
    acc_getter.borrow_mut().update().unwrap();
    state_getter.update().unwrap();
    let output = state_getter.get();
    assert_eq!(
        output.unwrap().unwrap(),
        Datum::new(3, State::new(1.5, 2.0, 1.0))
    );
}
#[test]
fn velocity_to_state() {
    struct VelGetter {
        time: i64,
    }
    impl VelGetter {
        fn new() -> Self {
            Self { time: 0 }
        }
    }
    impl Getter<f32, ()> for VelGetter {
        fn get(&self) -> Output<f32, ()> {
            Ok(Some(Datum::new(self.time, self.time as f32)))
        }
    }
    impl Updatable<()> for VelGetter {
        fn update(&mut self) -> NothingOrError<()> {
            self.time += 1;
            Ok(())
        }
    }
    let vel_getter = make_input_getter(VelGetter::new());
    let mut state_getter = VelocityToState::new(Rc::clone(&vel_getter));
    let output = state_getter.get();
    assert!(output.unwrap().is_none());
    vel_getter.borrow_mut().update().unwrap();
    state_getter.update().unwrap();
    let output = state_getter.get();
    assert!(output.unwrap().is_none());
    vel_getter.borrow_mut().update().unwrap();
    state_getter.update().unwrap();
    let output = state_getter.get();
    assert_eq!(
        output.unwrap().unwrap(),
        Datum::new(2, State::new(1.5, 2.0, 1.0))
    );
}
#[test]
fn position_to_state() {
    struct PosGetter {
        time: i64,
    }
    impl PosGetter {
        fn new() -> Self {
            Self { time: 0 }
        }
    }
    impl Getter<f32, ()> for PosGetter {
        fn get(&self) -> Output<f32, ()> {
            Ok(Some(Datum::new(self.time, self.time as f32)))
        }
    }
    impl Updatable<()> for PosGetter {
        fn update(&mut self) -> NothingOrError<()> {
            self.time += 1;
            Ok(())
        }
    }
    let pos_getter = make_input_getter(PosGetter::new());
    let mut state_getter = PositionToState::new(Rc::clone(&pos_getter));
    let output = state_getter.get();
    assert!(output.unwrap().is_none());
    pos_getter.borrow_mut().update().unwrap();
    state_getter.update().unwrap();
    let output = state_getter.get();
    assert!(output.unwrap().is_none());
    pos_getter.borrow_mut().update().unwrap();
    state_getter.update().unwrap();
    let output = state_getter.get();
    assert!(output.unwrap().is_none());
    pos_getter.borrow_mut().update().unwrap();
    state_getter.update().unwrap();
    let output = state_getter.get();
    assert_eq!(
        output.unwrap().unwrap(),
        Datum::new(3, State::new(3.0, 1.0, 0.0))
    );
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
            Self { index: 0 }
        }
    }
    impl Getter<f32, Nothing> for ErroringStream {
        fn get(&self) -> Output<f32, Nothing> {
            if self.index == 0 {
                return Err(Error::Other(Nothing));
            } else if self.index == 1 {
                return Ok(None);
            } else {
                return Ok(Some(Datum::new(2, 1.0)));
            }
        }
    }
    impl Updatable<Nothing> for ErroringStream {
        fn update(&mut self) -> NothingOrError<Nothing> {
            self.index += 1;
            Ok(())
        }
    }
    struct NormalStream;
    impl NormalStream {
        pub fn new() -> Self {
            Self {}
        }
    }
    impl Getter<f32, Nothing> for NormalStream {
        fn get(&self) -> Output<f32, Nothing> {
            Ok(Some(Datum::new(1, 1.0)))
        }
    }
    impl Updatable<Nothing> for NormalStream {
        fn update(&mut self) -> NothingOrError<Nothing> {
            Ok(())
        }
    }
    let erroring = make_input_getter(ErroringStream::new());
    let normal = make_input_getter(NormalStream::new());
    let stream = SumStream::new([Rc::clone(&erroring), Rc::clone(&normal)]);
    match stream.get() {
        Ok(_) => {
            panic!("error not propagated")
        }
        Err(_) => {}
    }
    //normal does not need update
    erroring.borrow_mut().update().unwrap();
    assert_eq!(stream.get().unwrap().unwrap().time, 1);
    assert_eq!(stream.get().unwrap().unwrap().value, 1.0);
    erroring.borrow_mut().update().unwrap();
    assert_eq!(stream.get().unwrap().unwrap().time, 2);
    assert_eq!(stream.get().unwrap().unwrap().value, 2.0);
}
#[test]
#[should_panic]
fn empty_sum_stream() {
    let _: SumStream<f32, 0, ()> = SumStream::new([]);
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
            Self { index: 0 }
        }
    }
    impl Getter<f32, DummyError> for Stream1 {
        fn get(&self) -> Output<f32, DummyError> {
            if self.index == 0 || self.index == 1 || self.index == 2 {
                return Err(Error::Other(DummyError));
            } else if self.index == 3 || self.index == 4 || self.index == 5 {
                return Ok(None);
            }
            return Ok(Some(Datum::new(1, 10.0)));
        }
    }
    impl Updatable<DummyError> for Stream1 {
        fn update(&mut self) -> NothingOrError<DummyError> {
            self.index += 1;
            Ok(())
        }
    }
    struct Stream2 {
        index: u8,
    }
    impl Stream2 {
        pub fn new() -> Self {
            Self { index: 0 }
        }
    }
    impl Getter<f32, DummyError> for Stream2 {
        fn get(&self) -> Output<f32, DummyError> {
            if self.index == 0 || self.index == 3 || self.index == 6 {
                return Err(Error::Other(DummyError));
            } else if self.index == 1 || self.index == 4 || self.index == 7 {
                return Ok(None);
            }
            return Ok(Some(Datum::new(2, 3.0)));
        }
    }
    impl Updatable<DummyError> for Stream2 {
        fn update(&mut self) -> NothingOrError<DummyError> {
            self.index += 1;
            Ok(())
        }
    }
    let stream1 = make_input_getter(Stream1::new());
    let stream2 = make_input_getter(Stream2::new());
    let stream = DifferenceStream::new(Rc::clone(&stream1), Rc::clone(&stream2));
    //Err, Err
    match stream.get() {
        Ok(_) => {
            panic!();
        }
        Err(_) => {}
    }
    stream1.borrow_mut().update().unwrap();
    stream2.borrow_mut().update().unwrap();
    //Err, None
    match stream.get() {
        Ok(_) => {
            panic!();
        }
        Err(_) => {}
    }
    stream1.borrow_mut().update().unwrap();
    stream2.borrow_mut().update().unwrap();
    //Err, Some
    match stream.get() {
        Ok(_) => {
            panic!();
        }
        Err(_) => {}
    }
    stream1.borrow_mut().update().unwrap();
    stream2.borrow_mut().update().unwrap();
    //None, Err
    match stream.get() {
        Ok(_) => {
            panic!();
        }
        Err(_) => {}
    }
    stream1.borrow_mut().update().unwrap();
    stream2.borrow_mut().update().unwrap();
    //None, None
    match stream.get() {
        Ok(Some(_)) => {
            panic!();
        }
        Ok(None) => {}
        Err(_) => {
            panic!();
        }
    }
    stream1.borrow_mut().update().unwrap();
    stream2.borrow_mut().update().unwrap();
    //None, Some
    match stream.get() {
        Ok(Some(_)) => {
            panic!();
        }
        Ok(None) => {}
        Err(_) => {
            panic!();
        }
    }
    stream1.borrow_mut().update().unwrap();
    stream2.borrow_mut().update().unwrap();
    //Some, Err
    match stream.get() {
        Ok(_) => {
            panic!();
        }
        Err(_) => {}
    }
    stream1.borrow_mut().update().unwrap();
    stream2.borrow_mut().update().unwrap();
    //Some, None
    match stream.get() {
        Ok(Some(x)) => {
            assert_eq!(x.time, 1);
            assert_eq!(x.value, 10.0);
        }
        Ok(None) => {
            panic!();
        }
        Err(_) => {
            panic!();
        }
    }
    stream1.borrow_mut().update().unwrap();
    stream2.borrow_mut().update().unwrap();
    //Some, Some
    match stream.get() {
        Ok(Some(x)) => {
            assert_eq!(x.time, 2);
            assert_eq!(x.value, 7.0);
        }
        Ok(None) => {
            panic!();
        }
        Err(_) => {
            panic!();
        }
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
            Self { index: 0 }
        }
    }
    impl Getter<f32, Nothing> for ErroringStream {
        fn get(&self) -> Output<f32, Nothing> {
            if self.index == 0 {
                return Err(Error::Other(Nothing));
            } else if self.index == 1 {
                return Ok(None);
            } else {
                return Ok(Some(Datum::new(2, 3.0)));
            }
        }
    }
    impl Updatable<Nothing> for ErroringStream {
        fn update(&mut self) -> NothingOrError<Nothing> {
            self.index += 1;
            Ok(())
        }
    }
    struct NormalStream;
    impl NormalStream {
        pub fn new() -> Self {
            Self {}
        }
    }
    impl Getter<f32, Nothing> for NormalStream {
        fn get(&self) -> Output<f32, Nothing> {
            Ok(Some(Datum::new(1, 5.0)))
        }
    }
    impl Updatable<Nothing> for NormalStream {
        fn update(&mut self) -> NothingOrError<Nothing> {
            Ok(())
        }
    }
    let erroring = make_input_getter(ErroringStream::new());
    let normal = make_input_getter(NormalStream::new());
    let stream = ProductStream::new([Rc::clone(&erroring), Rc::clone(&normal)]);
    match stream.get() {
        Ok(_) => {
            panic!("error not propagated")
        }
        Err(_) => {}
    }
    //normal does not need update
    erroring.borrow_mut().update().unwrap();
    assert_eq!(stream.get().unwrap().unwrap().time, 1);
    assert_eq!(stream.get().unwrap().unwrap().value, 5.0);
    erroring.borrow_mut().update().unwrap();
    assert_eq!(stream.get().unwrap().unwrap().time, 2);
    assert_eq!(stream.get().unwrap().unwrap().value, 15.0);
}
#[test]
#[should_panic]
fn empty_product_stream() {
    let _: ProductStream<f32, 0, ()> = ProductStream::new([]);
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
            Self { index: 0 }
        }
    }
    impl Getter<f32, DummyError> for Stream1 {
        fn get(&self) -> Output<f32, DummyError> {
            if self.index == 0 || self.index == 1 || self.index == 2 {
                return Err(Error::Other(DummyError));
            } else if self.index == 3 || self.index == 4 || self.index == 5 {
                return Ok(None);
            }
            return Ok(Some(Datum::new(1, 12.0)));
        }
    }
    impl Updatable<DummyError> for Stream1 {
        fn update(&mut self) -> NothingOrError<DummyError> {
            self.index += 1;
            Ok(())
        }
    }
    struct Stream2 {
        index: u8,
    }
    impl Stream2 {
        pub fn new() -> Self {
            Self { index: 0 }
        }
    }
    impl Getter<f32, DummyError> for Stream2 {
        fn get(&self) -> Output<f32, DummyError> {
            if self.index == 0 || self.index == 3 || self.index == 6 {
                return Err(Error::Other(DummyError));
            } else if self.index == 1 || self.index == 4 || self.index == 7 {
                return Ok(None);
            }
            return Ok(Some(Datum::new(2, 3.0)));
        }
    }
    impl Updatable<DummyError> for Stream2 {
        fn update(&mut self) -> NothingOrError<DummyError> {
            self.index += 1;
            Ok(())
        }
    }
    let stream1 = make_input_getter(Stream1::new());
    let stream2 = make_input_getter(Stream2::new());
    let stream = QuotientStream::new(Rc::clone(&stream1), Rc::clone(&stream2));
    //Err, Err
    match stream.get() {
        Ok(_) => {
            panic!();
        }
        Err(_) => {}
    }
    stream1.borrow_mut().update().unwrap();
    stream2.borrow_mut().update().unwrap();
    //Err, None
    match stream.get() {
        Ok(_) => {
            panic!();
        }
        Err(_) => {}
    }
    stream1.borrow_mut().update().unwrap();
    stream2.borrow_mut().update().unwrap();
    //Err, Some
    match stream.get() {
        Ok(_) => {
            panic!();
        }
        Err(_) => {}
    }
    stream1.borrow_mut().update().unwrap();
    stream2.borrow_mut().update().unwrap();
    //None, Err
    match stream.get() {
        Ok(_) => {
            panic!();
        }
        Err(_) => {}
    }
    stream1.borrow_mut().update().unwrap();
    stream2.borrow_mut().update().unwrap();
    //None, None
    match stream.get() {
        Ok(Some(_)) => {
            panic!();
        }
        Ok(None) => {}
        Err(_) => {
            panic!();
        }
    }
    stream1.borrow_mut().update().unwrap();
    stream2.borrow_mut().update().unwrap();
    //None, Some
    match stream.get() {
        Ok(Some(_)) => {
            panic!();
        }
        Ok(None) => {}
        Err(_) => {
            panic!();
        }
    }
    stream1.borrow_mut().update().unwrap();
    stream2.borrow_mut().update().unwrap();
    //Some, Err
    match stream.get() {
        Ok(_) => {
            panic!();
        }
        Err(_) => {}
    }
    stream1.borrow_mut().update().unwrap();
    stream2.borrow_mut().update().unwrap();
    //Some, None
    match stream.get() {
        Ok(Some(x)) => {
            assert_eq!(x.time, 1);
            assert_eq!(x.value, 12.0);
        }
        Ok(None) => {
            panic!();
        }
        Err(_) => {
            panic!();
        }
    }
    stream1.borrow_mut().update().unwrap();
    stream2.borrow_mut().update().unwrap();
    //Some, Some
    match stream.get() {
        Ok(Some(x)) => {
            assert_eq!(x.time, 2);
            assert_eq!(x.value, 4.0);
        }
        Ok(None) => {
            panic!();
        }
        Err(_) => {
            panic!();
        }
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
            Self { index: 0 }
        }
    }
    impl Getter<f32, DummyError> for Stream1 {
        fn get(&self) -> Output<f32, DummyError> {
            if self.index == 0 || self.index == 1 || self.index == 2 {
                return Err(Error::Other(DummyError));
            } else if self.index == 3 || self.index == 4 || self.index == 5 {
                return Ok(None);
            }
            return Ok(Some(Datum::new(1, 5.0)));
        }
    }
    impl Updatable<DummyError> for Stream1 {
        fn update(&mut self) -> NothingOrError<DummyError> {
            self.index += 1;
            Ok(())
        }
    }
    struct Stream2 {
        index: u8,
    }
    impl Stream2 {
        pub fn new() -> Self {
            Self { index: 0 }
        }
    }
    impl Getter<f32, DummyError> for Stream2 {
        fn get(&self) -> Output<f32, DummyError> {
            if self.index == 0 || self.index == 3 || self.index == 6 {
                return Err(Error::Other(DummyError));
            } else if self.index == 1 || self.index == 4 || self.index == 7 {
                return Ok(None);
            }
            return Ok(Some(Datum::new(2, 3.0)));
        }
    }
    impl Updatable<DummyError> for Stream2 {
        fn update(&mut self) -> NothingOrError<DummyError> {
            self.index += 1;
            Ok(())
        }
    }
    let stream1 = make_input_getter(Stream1::new());
    let stream2 = make_input_getter(Stream2::new());
    let stream = ExponentStream::new(Rc::clone(&stream1), Rc::clone(&stream2));
    //Err, Err
    match stream.get() {
        Ok(_) => {
            panic!();
        }
        Err(_) => {}
    }
    stream1.borrow_mut().update().unwrap();
    stream2.borrow_mut().update().unwrap();
    //Err, None
    match stream.get() {
        Ok(_) => {
            panic!();
        }
        Err(_) => {}
    }
    stream1.borrow_mut().update().unwrap();
    stream2.borrow_mut().update().unwrap();
    //Err, Some
    match stream.get() {
        Ok(_) => {
            panic!();
        }
        Err(_) => {}
    }
    stream1.borrow_mut().update().unwrap();
    stream2.borrow_mut().update().unwrap();
    //None, Err
    match stream.get() {
        Ok(_) => {
            panic!();
        }
        Err(_) => {}
    }
    stream1.borrow_mut().update().unwrap();
    stream2.borrow_mut().update().unwrap();
    //None, None
    match stream.get() {
        Ok(Some(_)) => {
            panic!();
        }
        Ok(None) => {}
        Err(_) => {
            panic!();
        }
    }
    stream1.borrow_mut().update().unwrap();
    stream2.borrow_mut().update().unwrap();
    //None, Some
    match stream.get() {
        Ok(Some(_)) => {
            panic!();
        }
        Ok(None) => {}
        Err(_) => {
            panic!();
        }
    }
    stream1.borrow_mut().update().unwrap();
    stream2.borrow_mut().update().unwrap();
    //Some, Err
    match stream.get() {
        Ok(_) => {
            panic!();
        }
        Err(_) => {}
    }
    stream1.borrow_mut().update().unwrap();
    stream2.borrow_mut().update().unwrap();
    //Some, None
    match stream.get() {
        Ok(Some(x)) => {
            assert_eq!(x.time, 1);
            assert_eq!(x.value, 5.0);
        }
        Ok(None) => {
            panic!();
        }
        Err(_) => {
            panic!();
        }
    }
    stream1.borrow_mut().update().unwrap();
    stream2.borrow_mut().update().unwrap();
    //Some, Some
    match stream.get() {
        Ok(Some(x)) => {
            assert_eq!(x.time, 2);
            assert_eq!(x.value, 125.0);
        }
        Ok(None) => {
            panic!();
        }
        Err(_) => {
            panic!();
        }
    }
}
#[test]
fn derivative_stream() {
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
            Ok(Some(Datum::new(self.time * 2, (self.time * 3) as f32)))
        }
    }
    impl Updatable<DummyError> for DummyStream {
        fn update(&mut self) -> NothingOrError<DummyError> {
            self.time += 2;
            Ok(())
        }
    }
    let input = make_input_getter(DummyStream::new());
    let mut stream = DerivativeStream::new(Rc::clone(&input));
    input.borrow_mut().update().unwrap();
    stream.update().unwrap();
    input.borrow_mut().update().unwrap();
    stream.update().unwrap();
    assert_eq!(stream.get().unwrap().unwrap().time, 8);
    assert_eq!(stream.get().unwrap().unwrap().value, 1.5);
}
#[test]
fn integral_stream() {
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
            Ok(Some(Datum::new(self.time, 1.0)))
        }
    }
    impl Updatable<DummyError> for DummyStream {
        fn update(&mut self) -> NothingOrError<DummyError> {
            self.time += 1;
            Ok(())
        }
    }
    let input = make_input_getter(DummyStream::new());
    let mut stream = IntegralStream::new(Rc::clone(&input));
    input.borrow_mut().update().unwrap();
    stream.update().unwrap();
    input.borrow_mut().update().unwrap();
    stream.update().unwrap();
    assert_eq!(stream.get().unwrap().unwrap().time, 2);
    assert_eq!(stream.get().unwrap().unwrap().value, 1.0);
}
#[test]
fn stream_pid() {
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
    let input = make_input_getter(DummyStream::new());
    let mut stream =
        PIDControllerStream::new(Rc::clone(&input), 5.0, PIDKValues::new(1.0, 0.01, 0.1));
    stream.update().unwrap();
    assert_eq!(stream.get().unwrap().unwrap().time, 0);
    assert_eq!(stream.get().unwrap().unwrap().value, 5.0);
    input.borrow_mut().update().unwrap();
    stream.update().unwrap();
    assert_eq!(stream.get().unwrap().unwrap().time, 2);
    assert_eq!(stream.get().unwrap().unwrap().value, 4.04);
}
#[test]
fn ewma_stream() {
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
            let value = match self.time {
                2 => 110.0,
                4 => 111.0,
                6 => 116.0,
                8 => 97.0,
                10 => 102.0,
                12 => 111.0,
                14 => 111.0,
                16 => 100.0,
                _ => 0.0,
            };
            Ok(Some(Datum::new(self.time, value)))
        }
    }
    impl Updatable<DummyError> for DummyStream {
        fn update(&mut self) -> NothingOrError<DummyError> {
            self.time += 2;
            Ok(())
        }
    }
    let input = make_input_getter(DummyStream::new());
    let mut stream = EWMAStream::new(Rc::clone(&input), 0.25);
    input.borrow_mut().update().unwrap();
    stream.update().unwrap();
    assert_eq!(stream.get().unwrap().unwrap().value, 110.0);
    input.borrow_mut().update().unwrap();
    stream.update().unwrap();
    assert_eq!(stream.get().unwrap().unwrap().value, 110.5);
    input.borrow_mut().update().unwrap();
    stream.update().unwrap();
    assert_eq!(stream.get().unwrap().unwrap().value, 113.25);
    input.borrow_mut().update().unwrap();
    stream.update().unwrap();
    assert_eq!(stream.get().unwrap().unwrap().value, 105.125);
    input.borrow_mut().update().unwrap();
    stream.update().unwrap();
    assert_eq!(stream.get().unwrap().unwrap().value, 103.5625);
    input.borrow_mut().update().unwrap();
    stream.update().unwrap();
    //Floating-point stuff gets a bit weird after this because of rounding, but it still appears to
    //work correctly.
    assert_eq!(stream.get().unwrap().unwrap().value, 107.28125);
    input.borrow_mut().update().unwrap();
    stream.update().unwrap();
    assert_eq!(stream.get().unwrap().unwrap().value, 109.140625);
    input.borrow_mut().update().unwrap();
    stream.update().unwrap();
    assert_eq!(stream.get().unwrap().unwrap().value, 104.5703125);
}
#[test]
fn moving_average_stream() {
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
            let value = match self.time {
                2 => 110.0,
                4 => 111.0,
                6 => 116.0,
                8 => 97.0,
                10 => 102.0,
                12 => 111.0,
                14 => 111.0,
                16 => 100.0,
                _ => 0.0,
            };
            Ok(Some(Datum::new(self.time, value)))
        }
    }
    impl Updatable<DummyError> for DummyStream {
        fn update(&mut self) -> NothingOrError<DummyError> {
            self.time += 2;
            Ok(())
        }
    }
    let input = make_input_getter(DummyStream::new());
    let mut stream = MovingAverageStream::new(Rc::clone(&input), 5);
    input.borrow_mut().update().unwrap();
    stream.update().unwrap();
    assert_eq!(stream.get().unwrap().unwrap().value, 110.0);
    input.borrow_mut().update().unwrap();
    stream.update().unwrap();
    assert_eq!(stream.get().unwrap().unwrap().value, 110.4);
    input.borrow_mut().update().unwrap();
    stream.update().unwrap();
    assert_eq!(stream.get().unwrap().unwrap().value, 112.8);
    input.borrow_mut().update().unwrap();
    stream.update().unwrap();
    assert_eq!(stream.get().unwrap().unwrap().value, 107.4);
    input.borrow_mut().update().unwrap();
    stream.update().unwrap();
    assert_eq!(stream.get().unwrap().unwrap().value, 102.8);
    input.borrow_mut().update().unwrap();
    stream.update().unwrap();
    assert_eq!(stream.get().unwrap().unwrap().value, 104.6);
    input.borrow_mut().update().unwrap();
    stream.update().unwrap();
    assert_eq!(stream.get().unwrap().unwrap().value, 109.2);
    input.borrow_mut().update().unwrap();
    stream.update().unwrap();
    assert_eq!(stream.get().unwrap().unwrap().value, 106.6);
}
#[test]
fn latest() {
    struct Stream1 {
        time: i64,
    }
    impl Stream1 {
        pub fn new() -> Self {
            Self { time: 0 }
        }
    }
    impl Getter<u8, ()> for Stream1 {
        fn get(&self) -> Output<u8, ()> {
            match self.time {
                0 => Ok(Some(Datum::new(1, 1))), //Some, Some
                1 => Ok(Some(Datum::new(0, 0))), //Some, Some
                2 => Ok(Some(Datum::new(0, 1))), //Some, None
                3 => Ok(Some(Datum::new(0, 1))), //Some, Err
                4 => Ok(None),                   //None, None
                5 => Ok(None),                   //None, Err
                6 => Err(Error::Other(())),      //Err,  Err
                _ => panic!("should be unreachable"),
            }
        }
    }
    impl Updatable<()> for Stream1 {
        fn update(&mut self) -> NothingOrError<()> {
            self.time += 1;
            Ok(())
        }
    }
    struct Stream2 {
        time: i64,
    }
    impl Stream2 {
        pub fn new() -> Self {
            Self { time: 0 }
        }
    }
    impl Getter<u8, ()> for Stream2 {
        fn get(&self) -> Output<u8, ()> {
            match self.time {
                0 => Ok(Some(Datum::new(0, 0))), //Some, Some
                1 => Ok(Some(Datum::new(1, 2))), //Some, Some
                2 => Ok(None),                   //Some, None
                3 => Err(Error::Other(())),      //Some, Err
                4 => Ok(None),                   //None, None
                5 => Err(Error::Other(())),      //None, Err
                6 => Err(Error::Other(())),      //Err,  Err
                _ => panic!("should be unreachable"),
            }
        }
    }
    impl Updatable<()> for Stream2 {
        fn update(&mut self) -> NothingOrError<()> {
            self.time += 1;
            Ok(())
        }
    }
    let stream1 = make_input_getter(Stream1::new());
    let stream2 = make_input_getter(Stream2::new());
    let mut latest = Latest::new([Rc::clone(&stream1), Rc::clone(&stream2)]);
    latest.update().unwrap(); //This should do nothing.
    assert_eq!(latest.get(), Ok(Some(Datum::new(1, 1))));
    stream1.borrow_mut().update().unwrap();
    stream2.borrow_mut().update().unwrap();
    assert_eq!(latest.get(), Ok(Some(Datum::new(1, 2))));
    stream1.borrow_mut().update().unwrap();
    stream2.borrow_mut().update().unwrap();
    assert_eq!(latest.get(), Ok(Some(Datum::new(0, 1))));
    stream1.borrow_mut().update().unwrap();
    stream2.borrow_mut().update().unwrap();
    assert_eq!(latest.get(), Ok(Some(Datum::new(0, 1))));
    stream1.borrow_mut().update().unwrap();
    stream2.borrow_mut().update().unwrap();
    assert_eq!(latest.get(), Ok(None));
    stream1.borrow_mut().update().unwrap();
    stream2.borrow_mut().update().unwrap();
    assert_eq!(latest.get(), Ok(None));
    stream1.borrow_mut().update().unwrap();
    stream2.borrow_mut().update().unwrap();
    assert_eq!(latest.get(), Ok(None));
}
#[test]
#[should_panic]
fn empty_latest() {
    let _: Latest<(), 0, ()> = Latest::new([]);
}
#[test]
fn and_stream() {
    struct In1 {
        index: u8,
    }
    impl In1 {
        fn new() -> Self {
            Self { index: 0 }
        }
    }
    impl Getter<bool, ()> for In1 {
        fn get(&self) -> Output<bool, ()> {
            Ok(match self.index {
                0 => Some(Datum::new(0, false)),
                1 => None,
                2 => Some(Datum::new(0, true)),
                3 => Some(Datum::new(0, false)),
                4 => None,
                5 => Some(Datum::new(0, true)),
                6 => Some(Datum::new(0, false)),
                7 => None,
                8 => Some(Datum::new(0, true)),
                _ => unimplemented!(),
            })
        }
    }
    impl Updatable<()> for In1 {
        fn update(&mut self) -> NothingOrError<()> {
            self.index += 1;
            Ok(())
        }
    }
    struct In2 {
        index: u8,
    }
    impl In2 {
        fn new() -> Self {
            Self { index: 0 }
        }
    }
    impl Getter<bool, ()> for In2 {
        fn get(&self) -> Output<bool, ()> {
            Ok(match self.index {
                0..=2 => Some(Datum::new(0, false)),
                3..=5 => None,
                6..=8 => Some(Datum::new(0, true)),
                _ => unimplemented!(),
            })
        }
    }
    impl Updatable<()> for In2 {
        fn update(&mut self) -> NothingOrError<()> {
            self.index += 1;
            Ok(())
        }
    }
    let in1 = make_input_getter(In1::new());
    let in2 = make_input_getter(In2::new());
    let mut and = AndStream::new(Rc::clone(&in1), Rc::clone(&in2));
    assert_eq!(and.get().unwrap().unwrap().value, false);
    in1.borrow_mut().update().unwrap();
    in2.borrow_mut().update().unwrap();
    and.update().unwrap();
    assert_eq!(and.get().unwrap().unwrap().value, false);
    in1.borrow_mut().update().unwrap();
    in2.borrow_mut().update().unwrap();
    and.update().unwrap();
    assert_eq!(and.get().unwrap().unwrap().value, false);
    in1.borrow_mut().update().unwrap();
    in2.borrow_mut().update().unwrap();
    and.update().unwrap();
    assert_eq!(and.get().unwrap().unwrap().value, false);
    in1.borrow_mut().update().unwrap();
    in2.borrow_mut().update().unwrap();
    and.update().unwrap();
    assert_eq!(and.get().unwrap(), None);
    in1.borrow_mut().update().unwrap();
    in2.borrow_mut().update().unwrap();
    and.update().unwrap();
    assert_eq!(and.get().unwrap(), None);
    in1.borrow_mut().update().unwrap();
    in2.borrow_mut().update().unwrap();
    and.update().unwrap();
    assert_eq!(and.get().unwrap().unwrap().value, false);
    in1.borrow_mut().update().unwrap();
    in2.borrow_mut().update().unwrap();
    and.update().unwrap();
    assert_eq!(and.get().unwrap(), None);
    in1.borrow_mut().update().unwrap();
    in2.borrow_mut().update().unwrap();
    and.update().unwrap();
    assert_eq!(and.get().unwrap().unwrap().value, true);
    in1.borrow_mut().update().unwrap();
    in2.borrow_mut().update().unwrap();
    and.update().unwrap();
}
#[test]
fn or_stream() {
    struct In1 {
        index: u8,
    }
    impl In1 {
        fn new() -> Self {
            Self { index: 0 }
        }
    }
    impl Getter<bool, ()> for In1 {
        fn get(&self) -> Output<bool, ()> {
            Ok(match self.index {
                0 => Some(Datum::new(0, false)),
                1 => None,
                2 => Some(Datum::new(0, true)),
                3 => Some(Datum::new(0, false)),
                4 => None,
                5 => Some(Datum::new(0, true)),
                6 => Some(Datum::new(0, false)),
                7 => None,
                8 => Some(Datum::new(0, true)),
                _ => unimplemented!(),
            })
        }
    }
    impl Updatable<()> for In1 {
        fn update(&mut self) -> NothingOrError<()> {
            self.index += 1;
            Ok(())
        }
    }
    struct In2 {
        index: u8,
    }
    impl In2 {
        fn new() -> Self {
            Self { index: 0 }
        }
    }
    impl Getter<bool, ()> for In2 {
        fn get(&self) -> Output<bool, ()> {
            Ok(match self.index {
                0..=2 => Some(Datum::new(0, false)),
                3..=5 => None,
                6..=8 => Some(Datum::new(0, true)),
                _ => unimplemented!(),
            })
        }
    }
    impl Updatable<()> for In2 {
        fn update(&mut self) -> NothingOrError<()> {
            self.index += 1;
            Ok(())
        }
    }
    let in1 = make_input_getter(In1::new());
    let in2 = make_input_getter(In2::new());
    let mut and = OrStream::new(Rc::clone(&in1), Rc::clone(&in2));
    assert_eq!(and.get().unwrap().unwrap().value, false);
    in1.borrow_mut().update().unwrap();
    in2.borrow_mut().update().unwrap();
    and.update().unwrap();
    assert_eq!(and.get().unwrap(), None);
    in1.borrow_mut().update().unwrap();
    in2.borrow_mut().update().unwrap();
    and.update().unwrap();
    assert_eq!(and.get().unwrap().unwrap().value, true);
    in1.borrow_mut().update().unwrap();
    in2.borrow_mut().update().unwrap();
    and.update().unwrap();
    assert_eq!(and.get().unwrap(), None);
    in1.borrow_mut().update().unwrap();
    in2.borrow_mut().update().unwrap();
    and.update().unwrap();
    assert_eq!(and.get().unwrap(), None);
    in1.borrow_mut().update().unwrap();
    in2.borrow_mut().update().unwrap();
    and.update().unwrap();
    assert_eq!(and.get().unwrap().unwrap().value, true);
    in1.borrow_mut().update().unwrap();
    in2.borrow_mut().update().unwrap();
    and.update().unwrap();
    assert_eq!(and.get().unwrap().unwrap().value, true);
    in1.borrow_mut().update().unwrap();
    in2.borrow_mut().update().unwrap();
    and.update().unwrap();
    assert_eq!(and.get().unwrap().unwrap().value, true);
    in1.borrow_mut().update().unwrap();
    in2.borrow_mut().update().unwrap();
    and.update().unwrap();
    assert_eq!(and.get().unwrap().unwrap().value, true);
    in1.borrow_mut().update().unwrap();
    in2.borrow_mut().update().unwrap();
    and.update().unwrap();
}
#[test]
fn not_stream() {
    struct In {
        index: u8,
    }
    impl In {
        fn new() -> Self {
            Self {
                index: 0,
            }
        }
    }
    impl Getter<bool, ()> for In {
        fn get(&self) -> Output<bool, ()> {
            Ok(match self.index {
                0 => Some(Datum::new(0, false)),
                1 => None,
                2 => Some(Datum::new(0, true)),
                _ => unimplemented!(),
            })
        }
    }
    impl Updatable<()> for In {
        fn update(&mut self) -> NothingOrError<()> {
            self.index += 1;
            Ok(())
        }
    }
    let input = make_input_getter(In::new());
    let mut not = NotStream::new(Rc::clone(&input));
    assert_eq!(not.get().unwrap().unwrap().value, true);
    input.borrow_mut().update().unwrap();
    not.update().unwrap();
    assert_eq!(not.get().unwrap(), None);
    input.borrow_mut().update().unwrap();
    not.update().unwrap();
    assert_eq!(not.get().unwrap().unwrap().value, false);
}
