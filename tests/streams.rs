// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2024 UxuginPython
use core::fmt::Debug;
use rrtk::streams::control::*;
use rrtk::streams::converters::*;
use rrtk::streams::flow::*;
use rrtk::streams::logic::*;
use rrtk::streams::math::*;
use rrtk::streams::*;
use rrtk::*;
#[test]
fn time_getter_from_stream() {
    struct DummyStream {
        time: i64,
    }
    impl DummyStream {
        pub const fn new() -> Self {
            Self { time: 0 }
        }
    }
    impl Getter<f32, ()> for DummyStream {
        fn get(&self) -> Output<f32, ()> {
            Ok(Some(Datum::new(self.time, 0.0)))
        }
    }
    impl Updatable<()> for DummyStream {
        fn update(&mut self) -> NothingOrError<()> {
            self.time += 1;
            Ok(())
        }
    }
    unsafe {
        static mut STREAM: DummyStream = DummyStream::new();
        let stream = Reference::from_ptr(core::ptr::addr_of_mut!(STREAM));
        let time_getter = TimeGetterFromGetter::new(stream.clone());
        stream.borrow_mut().update().unwrap();
        assert_eq!(time_getter.get().unwrap(), 1);
    }
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
    unsafe {
        static mut STREAM: DummyStream = DummyStream;
        let stream = Reference::from_ptr(core::ptr::addr_of_mut!(STREAM));
        static mut TIME_GETTER: DummyTimeGetter = DummyTimeGetter { time: 0 };
        let time_getter = Reference::from_ptr(core::ptr::addr_of_mut!(TIME_GETTER));
        let mut expirer = Expirer::new(stream, time_getter.clone(), 10);
        expirer.update().unwrap(); //This should do nothing.
        assert_eq!(expirer.get(), Ok(Some(Datum::new(0, 0.0))));
        time_getter.borrow_mut().update().unwrap();
        assert_eq!(expirer.get(), Ok(Some(Datum::new(0, 0.0))));
        time_getter.borrow_mut().update().unwrap();
        assert_eq!(expirer.get(), Ok(None));
    }
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
    unsafe {
        static mut STREAM: DummyStream = DummyStream;
        let stream = Reference::from_ptr(core::ptr::addr_of_mut!(STREAM));
        static mut TIME_GETTER: DummyTimeGetter = DummyTimeGetter { time: 0 };
        let time_getter = Reference::from_ptr(core::ptr::addr_of_mut!(TIME_GETTER));
        let expirer = Expirer::new(stream, time_getter, 10);
        assert_eq!(expirer.get(), Ok(None));
    }
}
#[test]
fn none_to_error() {
    #[derive(Clone, Copy, Debug)]
    struct Nothing;
    struct DummyStream {
        index: u8,
    }
    impl DummyStream {
        pub const fn new() -> Self {
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
    unsafe {
        static mut INPUT: DummyStream = DummyStream::new();
        let input = Reference::from_ptr(core::ptr::addr_of_mut!(INPUT));
        let mut stream = NoneToError::new(input.clone());
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
}
#[test]
fn none_to_value() {
    #[derive(Clone, Copy, Debug)]
    struct Nothing;
    struct DummyStream {
        index: u8,
    }
    impl DummyStream {
        pub const fn new() -> Self {
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
        pub const fn new() -> Self {
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
    unsafe {
        static mut INPUT: DummyStream = DummyStream::new();
        let input = Reference::from_ptr(core::ptr::addr_of_mut!(INPUT));
        static mut TIME_GETTER: DummyTimeGetter = DummyTimeGetter::new();
        let time_getter = Reference::from_ptr(core::ptr::addr_of_mut!(TIME_GETTER));
        let mut stream = NoneToValue::new(input.clone(), time_getter, 2.0);
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
}
#[test]
fn acceleration_to_state() {
    struct AccGetter {
        time: i64,
    }
    impl AccGetter {
        const fn new() -> Self {
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
    unsafe {
        static mut ACC_GETTER: AccGetter = AccGetter::new();
        let acc_getter = Reference::from_ptr(core::ptr::addr_of_mut!(ACC_GETTER));
        let mut state_getter = AccelerationToState::new(acc_getter.clone());
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
}
#[test]
fn velocity_to_state() {
    struct VelGetter {
        time: i64,
    }
    impl VelGetter {
        const fn new() -> Self {
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
    unsafe {
        static mut VEL_GETTER: VelGetter = VelGetter::new();
        let vel_getter = Reference::from_ptr(core::ptr::addr_of_mut!(VEL_GETTER));
        let mut state_getter = VelocityToState::new(vel_getter.clone());
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
}
#[test]
fn position_to_state() {
    struct PosGetter {
        time: i64,
    }
    impl PosGetter {
        const fn new() -> Self {
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
    unsafe {
        static mut POS_GETTER: PosGetter = PosGetter::new();
        let pos_getter = Reference::from_ptr(core::ptr::addr_of_mut!(POS_GETTER));
        let mut state_getter = PositionToState::new(pos_getter.clone());
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
}
#[test]
#[cfg(feature = "alloc")]
fn sum_stream() {
    #[derive(Clone, Copy, Debug)]
    struct Nothing;
    struct ErroringStream {
        index: u8,
    }
    impl ErroringStream {
        pub const fn new() -> Self {
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
        pub const fn new() -> Self {
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
    unsafe {
        static mut ERRORING: ErroringStream = ErroringStream::new();
        let erroring = Reference::from_ptr(core::ptr::addr_of_mut!(ERRORING));
        static mut NORMAL: NormalStream = NormalStream::new();
        let normal = Reference::from_ptr(core::ptr::addr_of_mut!(NORMAL));
        let stream = SumStream::new([
            to_dyn!(Getter<f32, _>, erroring.clone()),
            to_dyn!(Getter<f32, _>, normal.clone()),
        ]);
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
}
#[test]
#[should_panic]
#[cfg(feature = "alloc")]
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
        pub const fn new() -> Self {
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
        pub const fn new() -> Self {
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
    unsafe {
        static mut STREAM_1: Stream1 = Stream1::new();
        let stream1 = Reference::from_ptr(core::ptr::addr_of_mut!(STREAM_1));
        static mut STREAM_2: Stream2 = Stream2::new();
        let stream2 = Reference::from_ptr(core::ptr::addr_of_mut!(STREAM_2));
        let stream = DifferenceStream::new(stream1.clone(), stream2.clone());
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
}
#[test]
fn product_stream() {
    #[derive(Clone, Copy, Debug)]
    struct Nothing;
    struct ErroringStream {
        index: u8,
    }
    impl ErroringStream {
        pub const fn new() -> Self {
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
        pub const fn new() -> Self {
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
    unsafe {
        static mut ERRORING: ErroringStream = ErroringStream::new();
        let erroring = Reference::from_ptr(core::ptr::addr_of_mut!(ERRORING));
        static mut NORMAL: NormalStream = NormalStream::new();
        let normal = Reference::from_ptr(core::ptr::addr_of_mut!(NORMAL));
        let stream = ProductStream::new([
            to_dyn!(Getter<f32, _>, erroring.clone()),
            to_dyn!(Getter<f32, _>, normal.clone()),
        ]);
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
        pub const fn new() -> Self {
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
        pub const fn new() -> Self {
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
    unsafe {
        static mut STREAM_1: Stream1 = Stream1::new();
        let stream1 = Reference::from_ptr(core::ptr::addr_of_mut!(STREAM_1));
        static mut STREAM_2: Stream2 = Stream2::new();
        let stream2 = Reference::from_ptr(core::ptr::addr_of_mut!(STREAM_2));
        let stream = QuotientStream::new(stream1.clone(), stream2.clone());
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
        pub const fn new() -> Self {
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
        pub const fn new() -> Self {
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
    unsafe {
        static mut STREAM_1: Stream1 = Stream1::new();
        let stream1 = Reference::from_ptr(core::ptr::addr_of_mut!(STREAM_1));
        static mut STREAM_2: Stream2 = Stream2::new();
        let stream2 = Reference::from_ptr(core::ptr::addr_of_mut!(STREAM_2));
        let stream = ExponentStream::new(stream1.clone(), stream2.clone());
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
}
#[test]
fn derivative_stream() {
    #[derive(Clone, Copy, Debug)]
    struct DummyError;
    struct DummyStream {
        time: i64,
    }
    impl DummyStream {
        pub const fn new() -> Self {
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
    unsafe {
        static mut INPUT: DummyStream = DummyStream::new();
        let input = Reference::from_ptr(core::ptr::addr_of_mut!(INPUT));
        let mut stream = DerivativeStream::new(input.clone());
        input.borrow_mut().update().unwrap();
        stream.update().unwrap();
        input.borrow_mut().update().unwrap();
        stream.update().unwrap();
        assert_eq!(stream.get().unwrap().unwrap().time, 8);
        assert_eq!(stream.get().unwrap().unwrap().value, 1.5);
    }
}
#[test]
fn integral_stream() {
    #[derive(Clone, Copy, Debug)]
    struct DummyError;
    struct DummyStream {
        time: i64,
    }
    impl DummyStream {
        pub const fn new() -> Self {
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
    unsafe {
        static mut INPUT: DummyStream = DummyStream::new();
        let input = Reference::from_ptr(core::ptr::addr_of_mut!(INPUT));
        let mut stream = IntegralStream::new(input.clone());
        input.borrow_mut().update().unwrap();
        stream.update().unwrap();
        input.borrow_mut().update().unwrap();
        stream.update().unwrap();
        assert_eq!(stream.get().unwrap().unwrap().time, 2);
        assert_eq!(stream.get().unwrap().unwrap().value, 1.0);
    }
}
#[test]
fn stream_pid() {
    #[derive(Clone, Copy, Debug)]
    struct DummyError;
    struct DummyStream {
        time: i64,
    }
    impl DummyStream {
        pub const fn new() -> Self {
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
    unsafe {
        static mut INPUT: DummyStream = DummyStream::new();
        let input = Reference::from_ptr(core::ptr::addr_of_mut!(INPUT));
        let mut stream =
            PIDControllerStream::new(input.clone(), 5.0, PIDKValues::new(1.0, 0.01, 0.1));
        stream.update().unwrap();
        assert_eq!(stream.get().unwrap().unwrap().time, 0);
        assert_eq!(stream.get().unwrap().unwrap().value, 5.0);
        input.borrow_mut().update().unwrap();
        stream.update().unwrap();
        assert_eq!(stream.get().unwrap().unwrap().time, 2);
        assert_eq!(stream.get().unwrap().unwrap().value, 4.04);
    }
}
#[test]
fn ewma_stream() {
    #[derive(Clone, Copy, Debug)]
    struct DummyError;
    struct DummyStream {
        time: i64,
    }
    impl DummyStream {
        pub const fn new() -> Self {
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
    unsafe {
        static mut INPUT: DummyStream = DummyStream::new();
        let input = Reference::from_ptr(core::ptr::addr_of_mut!(INPUT));
        let mut stream = EWMAStream::new(input.clone(), 0.25);
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
}
#[test]
fn moving_average_stream() {
    #[derive(Clone, Copy, Debug)]
    struct DummyError;
    struct DummyStream {
        time: i64,
    }
    impl DummyStream {
        pub const fn new() -> Self {
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
    unsafe {
        static mut INPUT: DummyStream = DummyStream::new();
        let input = Reference::from_ptr(core::ptr::addr_of_mut!(INPUT));
        let mut stream = MovingAverageStream::new(input.clone(), 5);
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
}
#[test]
fn latest() {
    struct Stream1 {
        time: i64,
    }
    impl Stream1 {
        pub const fn new() -> Self {
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
        pub const fn new() -> Self {
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
    unsafe {
        static mut STREAM_1: Stream1 = Stream1::new();
        let stream1 = Reference::from_ptr(core::ptr::addr_of_mut!(STREAM_1));
        static mut STREAM_2: Stream2 = Stream2::new();
        let stream2 = Reference::from_ptr(core::ptr::addr_of_mut!(STREAM_2));
        let mut latest = Latest::new([
            to_dyn!(Getter<u8, _>, stream1.clone()),
            to_dyn!(Getter<u8, _>, stream2.clone()),
        ]);
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
        const fn new() -> Self {
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
        const fn new() -> Self {
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
    unsafe {
        static mut IN_1: In1 = In1::new();
        let in1 = Reference::from_ptr(core::ptr::addr_of_mut!(IN_1));
        static mut IN_2: In2 = In2::new();
        let in2 = Reference::from_ptr(core::ptr::addr_of_mut!(IN_2));
        let mut and = AndStream::new(in1.clone(), in2.clone());
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
}
#[test]
fn or_stream() {
    struct In1 {
        index: u8,
    }
    impl In1 {
        const fn new() -> Self {
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
        const fn new() -> Self {
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
    unsafe {
        static mut IN_1: In1 = In1::new();
        let in1 = Reference::from_ptr(core::ptr::addr_of_mut!(IN_1));
        static mut IN_2: In2 = In2::new();
        let in2 = Reference::from_ptr(core::ptr::addr_of_mut!(IN_2));
        let mut and = OrStream::new(in1.clone(), in2.clone());
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
}
#[test]
fn not_stream() {
    struct In {
        index: u8,
    }
    impl In {
        const fn new() -> Self {
            Self { index: 0 }
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
    unsafe {
        static mut INPUT: In = In::new();
        let input = Reference::from_ptr(core::ptr::addr_of_mut!(INPUT));
        let mut not = NotStream::new(input.clone());
        assert_eq!(not.get().unwrap().unwrap().value, true);
        input.borrow_mut().update().unwrap();
        not.update().unwrap();
        assert_eq!(not.get().unwrap(), None);
        input.borrow_mut().update().unwrap();
        not.update().unwrap();
        assert_eq!(not.get().unwrap().unwrap().value, false);
    }
}
#[test]
fn if_stream() {
    struct Condition {
        index: u8,
    }
    impl Getter<bool, ()> for Condition {
        fn get(&self) -> Output<bool, ()> {
            Ok(match self.index {
                0 => Some(Datum::new(0, false)),
                1 => None,
                2 => Some(Datum::new(0, true)),
                _ => unimplemented!(),
            })
        }
    }
    impl Updatable<()> for Condition {
        fn update(&mut self) -> NothingOrError<()> {
            self.index += 1;
            Ok(())
        }
    }
    struct Input;
    impl Getter<u8, ()> for Input {
        fn get(&self) -> Output<u8, ()> {
            Ok(Some(Datum::new(0, 0)))
        }
    }
    impl Updatable<()> for Input {
        fn update(&mut self) -> NothingOrError<()> {
            Ok(())
        }
    }
    unsafe {
        static mut CONDITION: Condition = Condition { index: 0 };
        let condition = Reference::from_ptr(core::ptr::addr_of_mut!(CONDITION));
        static mut INPUT: Input = Input;
        let input = Reference::from_ptr(core::ptr::addr_of_mut!(INPUT));
        let mut if_stream = IfStream::new(condition.clone(), input.clone());
        assert_eq!(if_stream.get().unwrap(), None);
        condition.borrow_mut().update().unwrap();
        if_stream.update().unwrap();
        assert_eq!(if_stream.get().unwrap(), None);
        condition.borrow_mut().update().unwrap();
        if_stream.update().unwrap();
        assert_eq!(if_stream.get().unwrap().unwrap().value, 0);
    }
}
#[test]
fn if_else_stream() {
    struct Condition {
        index: u8,
    }
    impl Getter<bool, ()> for Condition {
        fn get(&self) -> Output<bool, ()> {
            Ok(match self.index {
                0 => Some(Datum::new(0, false)),
                1 => None,
                2 => Some(Datum::new(0, true)),
                _ => unimplemented!(),
            })
        }
    }
    impl Updatable<()> for Condition {
        fn update(&mut self) -> NothingOrError<()> {
            self.index += 1;
            Ok(())
        }
    }
    struct True;
    impl Getter<u8, ()> for True {
        fn get(&self) -> Output<u8, ()> {
            Ok(Some(Datum::new(0, 1)))
        }
    }
    impl Updatable<()> for True {
        fn update(&mut self) -> NothingOrError<()> {
            Ok(())
        }
    }
    struct False;
    impl Getter<u8, ()> for False {
        fn get(&self) -> Output<u8, ()> {
            Ok(Some(Datum::new(0, 2)))
        }
    }
    impl Updatable<()> for False {
        fn update(&mut self) -> NothingOrError<()> {
            Ok(())
        }
    }
    unsafe {
        static mut CONDITION: Condition = Condition { index: 0 };
        let condition = Reference::from_ptr(core::ptr::addr_of_mut!(CONDITION));
        static mut TRUE_INPUT: True = True;
        let true_input = Reference::from_ptr(core::ptr::addr_of_mut!(TRUE_INPUT));
        static mut FALSE_INPUT: False = False;
        let false_input = Reference::from_ptr(core::ptr::addr_of_mut!(FALSE_INPUT));
        let mut if_else_stream = IfElseStream::new(condition.clone(), true_input, false_input);
        assert_eq!(if_else_stream.get().unwrap().unwrap().value, 2);
        condition.borrow_mut().update().unwrap();
        if_else_stream.update().unwrap();
        assert_eq!(if_else_stream.get().unwrap(), None);
        condition.borrow_mut().update().unwrap();
        if_else_stream.update().unwrap();
        assert_eq!(if_else_stream.get().unwrap().unwrap().value, 1);
    }
}
#[test]
fn freeze_stream() {
    struct Condition {
        time: i64,
    }
    impl Getter<bool, ()> for Condition {
        fn get(&self) -> Output<bool, ()> {
            Ok(match self.time {
                0..=1 => Some(Datum::new(0, false)),
                2..=3 => Some(Datum::new(0, true)),
                4..=5 => Some(Datum::new(0, false)),
                6..=7 => None,
                8..=9 => Some(Datum::new(0, false)),
                _ => unimplemented!(),
            })
        }
    }
    impl Updatable<()> for Condition {
        fn update(&mut self) -> NothingOrError<()> {
            self.time += 1;
            Ok(())
        }
    }
    struct Input {
        time: i64,
    }
    impl Getter<i64, ()> for Input {
        fn get(&self) -> Output<i64, ()> {
            Ok(Some(Datum::new(0, self.time)))
        }
    }
    impl Updatable<()> for Input {
        fn update(&mut self) -> NothingOrError<()> {
            self.time += 1;
            Ok(())
        }
    }
    unsafe {
        static mut CONDITION: Condition = Condition { time: 0 };
        let condition = Reference::from_ptr(core::ptr::addr_of_mut!(CONDITION));
        static mut INPUT: Input = Input { time: 0 };
        let input = Reference::from_ptr(core::ptr::addr_of_mut!(INPUT));
        let mut freeze = FreezeStream::new(condition.clone(), input.clone());
        freeze.update().unwrap();
        assert_eq!(freeze.get().unwrap().unwrap().value, 0);
        condition.borrow_mut().update().unwrap();
        input.borrow_mut().update().unwrap();
        freeze.update().unwrap();
        assert_eq!(freeze.get().unwrap().unwrap().value, 1);
        condition.borrow_mut().update().unwrap();
        input.borrow_mut().update().unwrap();
        freeze.update().unwrap();
        assert_eq!(freeze.get().unwrap().unwrap().value, 1);
        condition.borrow_mut().update().unwrap();
        input.borrow_mut().update().unwrap();
        freeze.update().unwrap();
        assert_eq!(freeze.get().unwrap().unwrap().value, 1);
        condition.borrow_mut().update().unwrap();
        input.borrow_mut().update().unwrap();
        freeze.update().unwrap();
        assert_eq!(freeze.get().unwrap().unwrap().value, 4);
        condition.borrow_mut().update().unwrap();
        input.borrow_mut().update().unwrap();
        freeze.update().unwrap();
        assert_eq!(freeze.get().unwrap().unwrap().value, 5);
        condition.borrow_mut().update().unwrap();
        input.borrow_mut().update().unwrap();
        freeze.update().unwrap();
        assert_eq!(freeze.get().unwrap(), None);
        condition.borrow_mut().update().unwrap();
        input.borrow_mut().update().unwrap();
        freeze.update().unwrap();
        assert_eq!(freeze.get().unwrap(), None);
        condition.borrow_mut().update().unwrap();
        input.borrow_mut().update().unwrap();
        freeze.update().unwrap();
        assert_eq!(freeze.get().unwrap().unwrap().value, 8);
        condition.borrow_mut().update().unwrap();
        input.borrow_mut().update().unwrap();
        freeze.update().unwrap();
        assert_eq!(freeze.get().unwrap().unwrap().value, 9);
    }
}
#[test]
fn command_pid() {
    struct Input {
        time: i64,
    }
    impl Getter<State, ()> for Input {
        fn get(&self) -> Output<State, ()> {
            Ok(Some(Datum::new(self.time, State::default())))
        }
    }
    impl Updatable<()> for Input {
        fn update(&mut self) -> NothingOrError<()> {
            self.time += 1;
            Ok(())
        }
    }
    unsafe {
        let kvals = PositionDerivativeDependentPIDKValues::new(
            PIDKValues::new(1.0, 0.01, 0.1),
            PIDKValues::new(1.0, 0.01, 0.1),
            PIDKValues::new(1.0, 0.01, 0.1),
        );
        {
            static mut INPUT: Input = Input { time: 0 };
            let input = Reference::from_ptr(core::ptr::addr_of_mut!(INPUT));
            let mut pid = CommandPID::new(
                input.clone(),
                Command::new(PositionDerivative::Position, 5.0),
                kvals,
            );
            assert_eq!(pid.get().unwrap(), None);
            pid.update().unwrap();
            assert_eq!(pid.get().unwrap().unwrap().value, 5.0);
            input.borrow_mut().update().unwrap();
            pid.update().unwrap();
            assert_eq!(pid.get().unwrap().unwrap().value, 5.05);
            input.borrow_mut().update().unwrap();
            pid.update().unwrap();
            assert_eq!(pid.get().unwrap().unwrap().value, 5.1);
            input.borrow_mut().update().unwrap();
            pid.update().unwrap();
            assert_eq!(pid.get().unwrap().unwrap().value, 5.15);
        }

        {
            static mut INPUT: Input = Input { time: 0 };
            let input = Reference::from_ptr(core::ptr::addr_of_mut!(INPUT));
            let mut pid = CommandPID::new(
                input.clone(),
                Command::new(PositionDerivative::Velocity, 5.0),
                kvals,
            );
            assert_eq!(pid.get().unwrap(), None);
            pid.update().unwrap();
            assert_eq!(pid.get().unwrap(), None);
            input.borrow_mut().update().unwrap();
            pid.update().unwrap();
            assert_eq!(pid.get().unwrap().unwrap().value, 5.025);
            input.borrow_mut().update().unwrap();
            pid.update().unwrap();
            assert_eq!(pid.get().unwrap().unwrap().value, 10.1);
            input.borrow_mut().update().unwrap();
            pid.update().unwrap();
            assert_eq!(pid.get().unwrap().unwrap().value, 15.225);
        }

        {
            static mut INPUT: Input = Input { time: 0 };
            let input = Reference::from_ptr(core::ptr::addr_of_mut!(INPUT));
            let mut pid = CommandPID::new(
                input.clone(),
                Command::new(PositionDerivative::Acceleration, 5.0),
                kvals,
            );
            assert_eq!(pid.get().unwrap(), None);
            pid.update().unwrap();
            assert_eq!(pid.get().unwrap(), None);
            input.borrow_mut().update().unwrap();
            pid.update().unwrap();
            assert_eq!(pid.get().unwrap(), None);
            input.borrow_mut().update().unwrap();
            pid.update().unwrap();
            assert_eq!(pid.get().unwrap().unwrap().value, 7.5625);
            input.borrow_mut().update().unwrap();
            pid.update().unwrap();
            assert_eq!(pid.get().unwrap().unwrap().value, 20.225);
        }
    }
}
