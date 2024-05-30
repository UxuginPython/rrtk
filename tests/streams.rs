// SPDX-License-Identifier: LGPL-3.0-only
/*
Copyright 2024 UxuginPython on GitHub

     This file is part of Rust Robotics ToolKit.

    Rust Robotics ToolKit is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, version 3.

    Rust Robotics ToolKit is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Lesser General Public License for more details.

    You should have received a copy of the GNU Lesser General Public License along with Rust Robotics ToolKit. If not, see <https://www.gnu.org/licenses/>.
*/
use rrtk::getters::control::*;
use rrtk::getters::converters::*;
use rrtk::getters::math::*;
use rrtk::getters::*;
use rrtk::*;
#[cfg(feature = "std")]
use std::cell::RefCell;
#[cfg(feature = "std")]
use std::fmt::Debug;
#[cfg(feature = "std")]
use std::rc::Rc;
#[cfg(not(feature = "std"))]
extern crate alloc;
#[cfg(not(feature = "std"))]
use alloc::rc::Rc;
#[cfg(not(feature = "std"))]
use core::cell::RefCell;
#[cfg(not(feature = "std"))]
use core::fmt::Debug;
#[test]
fn time_getter_from_getter() {
    struct DummyGetter {
        time: f32,
    }
    impl DummyGetter {
        pub fn new() -> Self {
            Self { time: 0.0 }
        }
    }
    impl<E: Copy + Debug> Getter<f32, E> for DummyGetter {
        fn get(&self) -> Output<f32, E> {
            Ok(Some(Datum::new(self.time, 0.0)))
        }
    }
    impl<E: Copy + Debug> Updatable<E> for DummyGetter {
        fn update(&mut self) -> UpdateOutput<E> {
            self.time += 1.0;
            Ok(())
        }
    }
    let getter = make_input_getter!(DummyGetter::new(), f32, u8);
    let time_getter = TimeGetterFromGetter::new(Rc::clone(&getter));
    getter.borrow_mut().update().unwrap();
    assert_eq!(time_getter.get().unwrap(), 1.0);
}
#[test]
fn make_input_getter_() {
    #[derive(Clone, Copy, Debug)]
    struct Nothing;
    struct DummyGetter {
        time: f32,
    }
    impl DummyGetter {
        pub fn new() -> Self {
            Self { time: 0.0 }
        }
    }
    impl<E: Copy + Debug> Getter<f32, E> for DummyGetter {
        fn get(&self) -> Output<f32, E> {
            Ok(Some(Datum::new(self.time, 0.0)))
        }
    }
    impl<E: Copy + Debug> Updatable<E> for DummyGetter {
        fn update(&mut self) -> UpdateOutput<E> {
            self.time += 1.0;
            Ok(())
        }
    }
    let tg_getter = make_input_getter!(DummyGetter::new(), f32, Nothing);
    let time_getter =
        make_input_time_getter!(TimeGetterFromGetter::new(Rc::clone(&tg_getter)), Nothing);
    let getter = Constant::new(Rc::clone(&time_getter), 20u8);
    assert_eq!(getter.get().unwrap().unwrap().value, 20);
    tg_getter.borrow_mut().update().unwrap();
    assert_eq!(getter.get().unwrap().unwrap().value, 20);
}
#[test]
fn constant() {
    #[derive(Clone, Copy, Debug)]
    struct Nothing;
    struct DummyGetter {
        time: f32,
    }
    impl DummyGetter {
        pub fn new() -> Self {
            Self { time: 0.0 }
        }
    }
    impl<E: Copy + Debug> Getter<f32, E> for DummyGetter {
        fn get(&self) -> Output<f32, E> {
            Ok(Some(Datum::new(self.time, 0.0)))
        }
    }
    impl<E: Copy + Debug> Updatable<E> for DummyGetter {
        fn update(&mut self) -> UpdateOutput<E> {
            self.time += 1.0;
            Ok(())
        }
    }
    let tg_getter = make_input_getter!(DummyGetter::new(), f32, Nothing);
    let time_getter = Rc::new(RefCell::new(Box::new(TimeGetterFromGetter::new(Rc::clone(
        &tg_getter,
    ))) as Box<dyn TimeGetter<Nothing>>));
    let mut getter = Constant::new(Rc::clone(&time_getter), 20u8);
    assert_eq!(getter.get().unwrap().unwrap().value, 20);
    tg_getter.borrow_mut().update().unwrap();
    assert_eq!(getter.get().unwrap().unwrap().value, 20);
    getter.update().unwrap();
    assert_eq!(getter.get().unwrap().unwrap().value, 20);
}
#[test]
fn none_to_error() {
    #[derive(Clone, Copy, Debug)]
    struct Nothing;
    struct DummyGetter {
        index: u8,
    }
    impl DummyGetter {
        pub fn new() -> Self {
            Self { index: 0 }
        }
    }
    impl Getter<f32, Nothing> for DummyGetter {
        fn get(&self) -> Output<f32, Nothing> {
            if self.index == 1 {
                return Ok(None);
            } else if self.index == 2 {
                return Err(Error::Other(Nothing));
            }
            return Ok(Some(Datum::new(0.0, 0.0)));
        }
    }
    impl Updatable<Nothing> for DummyGetter {
        fn update(&mut self) -> UpdateOutput<Nothing> {
            self.index += 1;
            Ok(())
        }
    }
    let input = make_input_getter!(DummyGetter::new(), f32, Nothing);
    let getter = NoneToError::new(Rc::clone(&input));
    match getter.get() {
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
    match getter.get() {
        Ok(_) => {
            panic!("should return Err");
        }
        Err(Error::FromNone) => {}
        Err(_) => {
            panic!("should be FromNone");
        }
    }
    input.borrow_mut().update().unwrap();
    match getter.get() {
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
    struct DummyGetter {
        index: u8,
    }
    impl DummyGetter {
        pub fn new() -> Self {
            Self { index: 0 }
        }
    }
    impl Getter<f32, Nothing> for DummyGetter {
        fn get(&self) -> Output<f32, Nothing> {
            if self.index == 1 {
                return Ok(None);
            } else if self.index == 2 {
                return Err(Error::Other(Nothing));
            }
            return Ok(Some(Datum::new(0.0, 1.0)));
        }
    }
    impl Updatable<Nothing> for DummyGetter {
        fn update(&mut self) -> UpdateOutput<Nothing> {
            self.index += 1;
            Ok(())
        }
    }
    struct DummyTimeGetter {
        time: f32,
    }
    impl DummyTimeGetter {
        pub fn new() -> Self {
            Self { time: 0.0 }
        }
    }
    impl<E: Copy + Debug> TimeGetter<E> for DummyTimeGetter {
        fn get(&self) -> TimeOutput<E> {
            Ok(self.time)
        }
    }
    impl<E: Copy + Debug> Updatable<E> for DummyTimeGetter {
        fn update(&mut self) -> UpdateOutput<E> {
            self.time += 1.0;
            Ok(())
        }
    }
    let input = make_input_getter!(DummyGetter::new(), f32, Nothing);
    let getter = NoneToValue::new(
        Rc::clone(&input),
        make_input_time_getter!(DummyTimeGetter::new(), Nothing),
        2.0,
    );
    match getter.get() {
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
    match getter.get() {
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
    match getter.get() {
        Ok(_) => {
            panic!("should return Err(_), returned Ok(_)");
        }
        Err(_) => {}
    }
}
#[test]
fn sum_getter() {
    #[derive(Clone, Copy, Debug)]
    struct Nothing;
    struct ErroringGetter {
        index: u8,
    }
    impl ErroringGetter {
        pub fn new() -> Self {
            Self { index: 0 }
        }
    }
    impl Getter<f32, Nothing> for ErroringGetter {
        fn get(&self) -> Output<f32, Nothing> {
            if self.index == 0 {
                return Err(Error::Other(Nothing));
            } else if self.index == 1 {
                return Ok(None);
            } else {
                return Ok(Some(Datum::new(2.0, 1.0)));
            }
        }
    }
    impl Updatable<Nothing> for ErroringGetter {
        fn update(&mut self) -> UpdateOutput<Nothing> {
            self.index += 1;
            Ok(())
        }
    }
    struct NormalGetter;
    impl NormalGetter {
        pub fn new() -> Self {
            Self {}
        }
    }
    impl Getter<f32, Nothing> for NormalGetter {
        fn get(&self) -> Output<f32, Nothing> {
            Ok(Some(Datum::new(1.0, 1.0)))
        }
    }
    impl Updatable<Nothing> for NormalGetter {
        fn update(&mut self) -> UpdateOutput<Nothing> {
            Ok(())
        }
    }
    let erroring = make_input_getter!(ErroringGetter::new(), f32, Nothing);
    let normal = make_input_getter!(NormalGetter::new(), f32, Nothing);
    let getter = SumGetter::new([Rc::clone(&erroring), Rc::clone(&normal)]);
    match getter.get() {
        Ok(_) => {
            panic!("error not propagated")
        }
        Err(_) => {}
    }
    //normal does not need update
    erroring.borrow_mut().update().unwrap();
    assert_eq!(getter.get().unwrap().unwrap().time, 1.0);
    assert_eq!(getter.get().unwrap().unwrap().value, 1.0);
    erroring.borrow_mut().update().unwrap();
    assert_eq!(getter.get().unwrap().unwrap().time, 2.0);
    assert_eq!(getter.get().unwrap().unwrap().value, 2.0);
}
#[test]
fn difference_getter() {
    #[derive(Clone, Copy, Debug)]
    struct DummyError;
    struct Getter1 {
        index: u8,
    }
    impl Getter1 {
        pub fn new() -> Self {
            Self { index: 0 }
        }
    }
    impl Getter<f32, DummyError> for Getter1 {
        fn get(&self) -> Output<f32, DummyError> {
            if self.index == 0 || self.index == 1 || self.index == 2 {
                return Err(Error::Other(DummyError));
            } else if self.index == 3 || self.index == 4 || self.index == 5 {
                return Ok(None);
            }
            return Ok(Some(Datum::new(1.0, 10.0)));
        }
    }
    impl Updatable<DummyError> for Getter1 {
        fn update(&mut self) -> UpdateOutput<DummyError> {
            self.index += 1;
            Ok(())
        }
    }
    struct Getter2 {
        index: u8,
    }
    impl Getter2 {
        pub fn new() -> Self {
            Self { index: 0 }
        }
    }
    impl Getter<f32, DummyError> for Getter2 {
        fn get(&self) -> Output<f32, DummyError> {
            if self.index == 0 || self.index == 3 || self.index == 6 {
                return Err(Error::Other(DummyError));
            } else if self.index == 1 || self.index == 4 || self.index == 7 {
                return Ok(None);
            }
            return Ok(Some(Datum::new(2.0, 3.0)));
        }
    }
    impl Updatable<DummyError> for Getter2 {
        fn update(&mut self) -> UpdateOutput<DummyError> {
            self.index += 1;
            Ok(())
        }
    }
    let getter1 = make_input_getter!(Getter1::new(), f32, DummyError);
    let getter2 = make_input_getter!(Getter2::new(), f32, DummyError);
    let getter = DifferenceGetter::new(Rc::clone(&getter1), Rc::clone(&getter2));
    //Err, Err
    match getter.get() {
        Ok(_) => {
            panic!();
        }
        Err(_) => {}
    }
    getter1.borrow_mut().update().unwrap();
    getter2.borrow_mut().update().unwrap();
    //Err, None
    match getter.get() {
        Ok(_) => {
            panic!();
        }
        Err(_) => {}
    }
    getter1.borrow_mut().update().unwrap();
    getter2.borrow_mut().update().unwrap();
    //Err, Some
    match getter.get() {
        Ok(_) => {
            panic!();
        }
        Err(_) => {}
    }
    getter1.borrow_mut().update().unwrap();
    getter2.borrow_mut().update().unwrap();
    //None, Err
    match getter.get() {
        Ok(_) => {
            panic!();
        }
        Err(_) => {}
    }
    getter1.borrow_mut().update().unwrap();
    getter2.borrow_mut().update().unwrap();
    //None, None
    match getter.get() {
        Ok(Some(_)) => {
            panic!();
        }
        Ok(None) => {}
        Err(_) => {
            panic!();
        }
    }
    getter1.borrow_mut().update().unwrap();
    getter2.borrow_mut().update().unwrap();
    //None, Some
    match getter.get() {
        Ok(Some(_)) => {
            panic!();
        }
        Ok(None) => {}
        Err(_) => {
            panic!();
        }
    }
    getter1.borrow_mut().update().unwrap();
    getter2.borrow_mut().update().unwrap();
    //Some, Err
    match getter.get() {
        Ok(_) => {
            panic!();
        }
        Err(_) => {}
    }
    getter1.borrow_mut().update().unwrap();
    getter2.borrow_mut().update().unwrap();
    //Some, None
    match getter.get() {
        Ok(Some(x)) => {
            assert_eq!(x.time, 1.0);
            assert_eq!(x.value, 10.0);
        }
        Ok(None) => {
            panic!();
        }
        Err(_) => {
            panic!();
        }
    }
    getter1.borrow_mut().update().unwrap();
    getter2.borrow_mut().update().unwrap();
    //Some, Some
    match getter.get() {
        Ok(Some(x)) => {
            assert_eq!(x.time, 2.0);
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
fn product_getter() {
    #[derive(Clone, Copy, Debug)]
    struct Nothing;
    struct ErroringGetter {
        index: u8,
    }
    impl ErroringGetter {
        pub fn new() -> Self {
            Self { index: 0 }
        }
    }
    impl Getter<f32, Nothing> for ErroringGetter {
        fn get(&self) -> Output<f32, Nothing> {
            if self.index == 0 {
                return Err(Error::Other(Nothing));
            } else if self.index == 1 {
                return Ok(None);
            } else {
                return Ok(Some(Datum::new(2.0, 3.0)));
            }
        }
    }
    impl Updatable<Nothing> for ErroringGetter {
        fn update(&mut self) -> UpdateOutput<Nothing> {
            self.index += 1;
            Ok(())
        }
    }
    struct NormalGetter;
    impl NormalGetter {
        pub fn new() -> Self {
            Self {}
        }
    }
    impl Getter<f32, Nothing> for NormalGetter {
        fn get(&self) -> Output<f32, Nothing> {
            Ok(Some(Datum::new(1.0, 5.0)))
        }
    }
    impl Updatable<Nothing> for NormalGetter {
        fn update(&mut self) -> UpdateOutput<Nothing> {
            Ok(())
        }
    }
    let erroring = make_input_getter!(ErroringGetter::new(), f32, Nothing);
    let normal = make_input_getter!(NormalGetter::new(), f32, Nothing);
    let getter = ProductGetter::new([Rc::clone(&erroring), Rc::clone(&normal)]);
    match getter.get() {
        Ok(_) => {
            panic!("error not propagated")
        }
        Err(_) => {}
    }
    //normal does not need update
    erroring.borrow_mut().update().unwrap();
    assert_eq!(getter.get().unwrap().unwrap().time, 1.0);
    assert_eq!(getter.get().unwrap().unwrap().value, 5.0);
    erroring.borrow_mut().update().unwrap();
    assert_eq!(getter.get().unwrap().unwrap().time, 2.0);
    assert_eq!(getter.get().unwrap().unwrap().value, 15.0);
}
#[test]
fn quotient_getter() {
    #[derive(Clone, Copy, Debug)]
    struct DummyError;
    struct Getter1 {
        index: u8,
    }
    impl Getter1 {
        pub fn new() -> Self {
            Self { index: 0 }
        }
    }
    impl Getter<f32, DummyError> for Getter1 {
        fn get(&self) -> Output<f32, DummyError> {
            if self.index == 0 || self.index == 1 || self.index == 2 {
                return Err(Error::Other(DummyError));
            } else if self.index == 3 || self.index == 4 || self.index == 5 {
                return Ok(None);
            }
            return Ok(Some(Datum::new(1.0, 12.0)));
        }
    }
    impl Updatable<DummyError> for Getter1 {
        fn update(&mut self) -> UpdateOutput<DummyError> {
            self.index += 1;
            Ok(())
        }
    }
    struct Getter2 {
        index: u8,
    }
    impl Getter2 {
        pub fn new() -> Self {
            Self { index: 0 }
        }
    }
    impl Getter<f32, DummyError> for Getter2 {
        fn get(&self) -> Output<f32, DummyError> {
            if self.index == 0 || self.index == 3 || self.index == 6 {
                return Err(Error::Other(DummyError));
            } else if self.index == 1 || self.index == 4 || self.index == 7 {
                return Ok(None);
            }
            return Ok(Some(Datum::new(2.0, 3.0)));
        }
    }
    impl Updatable<DummyError> for Getter2 {
        fn update(&mut self) -> UpdateOutput<DummyError> {
            self.index += 1;
            Ok(())
        }
    }
    let getter1 = make_input_getter!(Getter1::new(), f32, DummyError);
    let getter2 = make_input_getter!(Getter2::new(), f32, DummyError);
    let getter = QuotientGetter::new(Rc::clone(&getter1), Rc::clone(&getter2));
    //Err, Err
    match getter.get() {
        Ok(_) => {
            panic!();
        }
        Err(_) => {}
    }
    getter1.borrow_mut().update().unwrap();
    getter2.borrow_mut().update().unwrap();
    //Err, None
    match getter.get() {
        Ok(_) => {
            panic!();
        }
        Err(_) => {}
    }
    getter1.borrow_mut().update().unwrap();
    getter2.borrow_mut().update().unwrap();
    //Err, Some
    match getter.get() {
        Ok(_) => {
            panic!();
        }
        Err(_) => {}
    }
    getter1.borrow_mut().update().unwrap();
    getter2.borrow_mut().update().unwrap();
    //None, Err
    match getter.get() {
        Ok(_) => {
            panic!();
        }
        Err(_) => {}
    }
    getter1.borrow_mut().update().unwrap();
    getter2.borrow_mut().update().unwrap();
    //None, None
    match getter.get() {
        Ok(Some(_)) => {
            panic!();
        }
        Ok(None) => {}
        Err(_) => {
            panic!();
        }
    }
    getter1.borrow_mut().update().unwrap();
    getter2.borrow_mut().update().unwrap();
    //None, Some
    match getter.get() {
        Ok(Some(_)) => {
            panic!();
        }
        Ok(None) => {}
        Err(_) => {
            panic!();
        }
    }
    getter1.borrow_mut().update().unwrap();
    getter2.borrow_mut().update().unwrap();
    //Some, Err
    match getter.get() {
        Ok(_) => {
            panic!();
        }
        Err(_) => {}
    }
    getter1.borrow_mut().update().unwrap();
    getter2.borrow_mut().update().unwrap();
    //Some, None
    match getter.get() {
        Ok(Some(x)) => {
            assert_eq!(x.time, 1.0);
            assert_eq!(x.value, 12.0);
        }
        Ok(None) => {
            panic!();
        }
        Err(_) => {
            panic!();
        }
    }
    getter1.borrow_mut().update().unwrap();
    getter2.borrow_mut().update().unwrap();
    //Some, Some
    match getter.get() {
        Ok(Some(x)) => {
            assert_eq!(x.time, 2.0);
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
fn exponent_getter() {
    #[derive(Clone, Copy, Debug)]
    struct DummyError;
    struct Getter1 {
        index: u8,
    }
    impl Getter1 {
        pub fn new() -> Self {
            Self { index: 0 }
        }
    }
    impl Getter<f32, DummyError> for Getter1 {
        fn get(&self) -> Output<f32, DummyError> {
            if self.index == 0 || self.index == 1 || self.index == 2 {
                return Err(Error::Other(DummyError));
            } else if self.index == 3 || self.index == 4 || self.index == 5 {
                return Ok(None);
            }
            return Ok(Some(Datum::new(1.0, 5.0)));
        }
    }
    impl Updatable<DummyError> for Getter1 {
        fn update(&mut self) -> UpdateOutput<DummyError> {
            self.index += 1;
            Ok(())
        }
    }
    struct Getter2 {
        index: u8,
    }
    impl Getter2 {
        pub fn new() -> Self {
            Self { index: 0 }
        }
    }
    impl Getter<f32, DummyError> for Getter2 {
        fn get(&self) -> Output<f32, DummyError> {
            if self.index == 0 || self.index == 3 || self.index == 6 {
                return Err(Error::Other(DummyError));
            } else if self.index == 1 || self.index == 4 || self.index == 7 {
                return Ok(None);
            }
            return Ok(Some(Datum::new(2.0, 3.0)));
        }
    }
    impl Updatable<DummyError> for Getter2 {
        fn update(&mut self) -> UpdateOutput<DummyError> {
            self.index += 1;
            Ok(())
        }
    }
    let getter1 = make_input_getter!(Getter1::new(), f32, DummyError);
    let getter2 = make_input_getter!(Getter2::new(), f32, DummyError);
    let getter = ExponentGetter::new(Rc::clone(&getter1), Rc::clone(&getter2));
    //Err, Err
    match getter.get() {
        Ok(_) => {
            panic!();
        }
        Err(_) => {}
    }
    getter1.borrow_mut().update().unwrap();
    getter2.borrow_mut().update().unwrap();
    //Err, None
    match getter.get() {
        Ok(_) => {
            panic!();
        }
        Err(_) => {}
    }
    getter1.borrow_mut().update().unwrap();
    getter2.borrow_mut().update().unwrap();
    //Err, Some
    match getter.get() {
        Ok(_) => {
            panic!();
        }
        Err(_) => {}
    }
    getter1.borrow_mut().update().unwrap();
    getter2.borrow_mut().update().unwrap();
    //None, Err
    match getter.get() {
        Ok(_) => {
            panic!();
        }
        Err(_) => {}
    }
    getter1.borrow_mut().update().unwrap();
    getter2.borrow_mut().update().unwrap();
    //None, None
    match getter.get() {
        Ok(Some(_)) => {
            panic!();
        }
        Ok(None) => {}
        Err(_) => {
            panic!();
        }
    }
    getter1.borrow_mut().update().unwrap();
    getter2.borrow_mut().update().unwrap();
    //None, Some
    match getter.get() {
        Ok(Some(_)) => {
            panic!();
        }
        Ok(None) => {}
        Err(_) => {
            panic!();
        }
    }
    getter1.borrow_mut().update().unwrap();
    getter2.borrow_mut().update().unwrap();
    //Some, Err
    match getter.get() {
        Ok(_) => {
            panic!();
        }
        Err(_) => {}
    }
    getter1.borrow_mut().update().unwrap();
    getter2.borrow_mut().update().unwrap();
    //Some, None
    match getter.get() {
        Ok(Some(x)) => {
            assert_eq!(x.time, 1.0);
            assert_eq!(x.value, 5.0);
        }
        Ok(None) => {
            panic!();
        }
        Err(_) => {
            panic!();
        }
    }
    getter1.borrow_mut().update().unwrap();
    getter2.borrow_mut().update().unwrap();
    //Some, Some
    match getter.get() {
        Ok(Some(x)) => {
            assert_eq!(x.time, 2.0);
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
fn derivative_getter() {
    #[derive(Clone, Copy, Debug)]
    struct DummyError;
    struct DummyGetter {
        time: f32,
    }
    impl DummyGetter {
        pub fn new() -> Self {
            Self { time: 0.0 }
        }
    }
    impl Getter<f32, DummyError> for DummyGetter {
        fn get(&self) -> Output<f32, DummyError> {
            Ok(Some(Datum::new(self.time * 2.0, self.time * 3.0)))
        }
    }
    impl Updatable<DummyError> for DummyGetter {
        fn update(&mut self) -> UpdateOutput<DummyError> {
            self.time += 2.0;
            Ok(())
        }
    }
    let input = make_input_getter!(DummyGetter::new(), f32, DummyError);
    let mut getter = DerivativeGetter::new(Rc::clone(&input));
    input.borrow_mut().update().unwrap();
    getter.update().unwrap();
    input.borrow_mut().update().unwrap();
    getter.update().unwrap();
    assert_eq!(getter.get().unwrap().unwrap().time, 8.0);
    assert_eq!(getter.get().unwrap().unwrap().value, 1.5);
}
#[test]
fn integral_getter() {
    #[derive(Clone, Copy, Debug)]
    struct DummyError;
    struct DummyGetter {
        time: f32,
    }
    impl DummyGetter {
        pub fn new() -> Self {
            Self { time: 0.0 }
        }
    }
    impl Getter<f32, DummyError> for DummyGetter {
        fn get(&self) -> Output<f32, DummyError> {
            Ok(Some(Datum::new(self.time, 1.0)))
        }
    }
    impl Updatable<DummyError> for DummyGetter {
        fn update(&mut self) -> UpdateOutput<DummyError> {
            self.time += 1.0;
            Ok(())
        }
    }
    let input = make_input_getter!(DummyGetter::new(), f32, DummyError);
    let mut getter = IntegralGetter::new(Rc::clone(&input));
    input.borrow_mut().update().unwrap();
    getter.update().unwrap();
    input.borrow_mut().update().unwrap();
    getter.update().unwrap();
    assert_eq!(getter.get().unwrap().unwrap().time, 2.0);
    assert_eq!(getter.get().unwrap().unwrap().value, 1.0);
}
#[test]
fn getter_pid() {
    #[derive(Clone, Copy, Debug)]
    struct DummyError;
    struct DummyGetter {
        time: f32,
    }
    impl DummyGetter {
        pub fn new() -> Self {
            Self { time: 0.0 }
        }
    }
    impl Getter<f32, DummyError> for DummyGetter {
        fn get(&self) -> Output<f32, DummyError> {
            Ok(Some(Datum::new(self.time, self.time / 2.0)))
        }
    }
    impl Updatable<DummyError> for DummyGetter {
        fn update(&mut self) -> UpdateOutput<DummyError> {
            self.time += 2.0;
            Ok(())
        }
    }
    let input = make_input_getter!(DummyGetter::new(), f32, DummyError);
    let mut getter = GetterPID::new(Rc::clone(&input), 5.0, 1.0, 0.01, 0.1);
    getter.update().unwrap();
    assert_eq!(getter.get().unwrap().unwrap().time, 0.0);
    assert_eq!(getter.get().unwrap().unwrap().value, 5.0);
    input.borrow_mut().update().unwrap();
    getter.update().unwrap();
    assert_eq!(getter.get().unwrap().unwrap().time, 2.0);
    assert_eq!(getter.get().unwrap().unwrap().value, 4.04);
}
#[test]
fn ewma_getter() {
    #[derive(Clone, Copy, Debug)]
    struct DummyError;
    struct DummyGetter {
        time: u8,
    }
    impl DummyGetter {
        pub fn new() -> Self {
            Self { time: 0 }
        }
    }
    impl Getter<f32, DummyError> for DummyGetter {
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
            Ok(Some(Datum::new(self.time as f32, value)))
        }
    }
    impl Updatable<DummyError> for DummyGetter {
        fn update(&mut self) -> UpdateOutput<DummyError> {
            self.time += 2;
            Ok(())
        }
    }
    let input = make_input_getter!(DummyGetter::new(), f32, DummyError);
    let mut getter = EWMAGetter::new(Rc::clone(&input), 0.25);
    input.borrow_mut().update().unwrap();
    getter.update().unwrap();
    assert_eq!(getter.get().unwrap().unwrap().value, 110.0);
    input.borrow_mut().update().unwrap();
    getter.update().unwrap();
    assert_eq!(getter.get().unwrap().unwrap().value, 110.5);
    input.borrow_mut().update().unwrap();
    getter.update().unwrap();
    assert_eq!(getter.get().unwrap().unwrap().value, 113.25);
    input.borrow_mut().update().unwrap();
    getter.update().unwrap();
    assert_eq!(getter.get().unwrap().unwrap().value, 105.125);
    input.borrow_mut().update().unwrap();
    getter.update().unwrap();
    assert_eq!(getter.get().unwrap().unwrap().value, 103.5625);
    input.borrow_mut().update().unwrap();
    getter.update().unwrap();
    //Floating-point stuff gets a bit weird after this because of rounding, but it still appears to
    //work correctly.
    assert_eq!(getter.get().unwrap().unwrap().value, 107.28125);
    input.borrow_mut().update().unwrap();
    getter.update().unwrap();
    assert_eq!(getter.get().unwrap().unwrap().value, 109.140625);
    input.borrow_mut().update().unwrap();
    getter.update().unwrap();
    assert_eq!(getter.get().unwrap().unwrap().value, 104.5703125);
}
#[test]
fn moving_average_getter() {
    #[derive(Clone, Copy, Debug)]
    struct DummyError;
    struct DummyGetter {
        time: u8,
    }
    impl DummyGetter {
        pub fn new() -> Self {
            Self { time: 0 }
        }
    }
    impl Getter<f32, DummyError> for DummyGetter {
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
            Ok(Some(Datum::new(self.time as f32, value)))
        }
    }
    impl Updatable<DummyError> for DummyGetter {
        fn update(&mut self) -> UpdateOutput<DummyError> {
            self.time += 2;
            Ok(())
        }
    }
    let input = make_input_getter!(DummyGetter::new(), f32, DummyError);
    let mut getter = MovingAverageGetter::new(Rc::clone(&input), 5.0);
    input.borrow_mut().update().unwrap();
    getter.update().unwrap();
    assert_eq!(getter.get().unwrap().unwrap().value, 110.0);
    input.borrow_mut().update().unwrap();
    getter.update().unwrap();
    assert_eq!(getter.get().unwrap().unwrap().value, 110.4);
    input.borrow_mut().update().unwrap();
    getter.update().unwrap();
    assert_eq!(getter.get().unwrap().unwrap().value, 112.8);
    input.borrow_mut().update().unwrap();
    getter.update().unwrap();
    assert_eq!(getter.get().unwrap().unwrap().value, 107.4);
    input.borrow_mut().update().unwrap();
    getter.update().unwrap();
    assert_eq!(getter.get().unwrap().unwrap().value, 102.8);
    input.borrow_mut().update().unwrap();
    getter.update().unwrap();
    assert_eq!(getter.get().unwrap().unwrap().value, 104.6);
    input.borrow_mut().update().unwrap();
    getter.update().unwrap();
    assert_eq!(getter.get().unwrap().unwrap().value, 109.2);
    input.borrow_mut().update().unwrap();
    getter.update().unwrap();
    assert_eq!(getter.get().unwrap().unwrap().value, 106.6);
}
