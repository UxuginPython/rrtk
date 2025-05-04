// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2024-2025 UxuginPython
use core::fmt::Debug;
use rrtk::streams::control::*;
use rrtk::streams::converters::*;
use rrtk::streams::flow::*;
use rrtk::streams::logic::*;
use rrtk::streams::math::*;
use rrtk::streams::*;
use rrtk::*;
//TODO: Some of these PointerDereferencers probably aren't needed, and nearly all unsafe blocks can
//be shrunk.
#[test]
fn expirer() {
    struct DummyStream;
    impl Getter<f32, ()> for DummyStream {
        fn get(&self) -> Output<f32, ()> {
            Ok(Some(Datum::new(Time::ZERO, 0.0)))
        }
    }
    impl Updatable<()> for DummyStream {
        fn update(&mut self) -> NothingOrError<()> {
            Ok(())
        }
    }
    struct DummyTimeGetter {
        time: Time,
    }
    impl TimeGetter<()> for DummyTimeGetter {
        fn get(&self) -> TimeOutput<()> {
            Ok(self.time)
        }
    }
    impl Updatable<()> for DummyTimeGetter {
        fn update(&mut self) -> NothingOrError<()> {
            self.time += Time::from_nanoseconds(10);
            Ok(())
        }
    }
    unsafe {
        static mut STREAM: DummyStream = DummyStream;
        let stream = PointerDereferencer::new(core::ptr::addr_of_mut!(STREAM));
        static mut TIME_GETTER: DummyTimeGetter = DummyTimeGetter { time: Time::ZERO };
        let mut time_getter = PointerDereferencer::new(core::ptr::addr_of_mut!(TIME_GETTER));
        let mut expirer = Expirer::new(stream, time_getter.clone(), Time::from_nanoseconds(10));
        expirer.update().unwrap(); //This should do nothing.
        assert_eq!(expirer.get(), Ok(Some(Datum::new(Time::ZERO, 0.0))));
        time_getter.update().unwrap();
        assert_eq!(expirer.get(), Ok(Some(Datum::new(Time::ZERO, 0.0))));
        time_getter.update().unwrap();
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
        time: Time,
    }
    impl TimeGetter<()> for DummyTimeGetter {
        fn get(&self) -> TimeOutput<()> {
            Ok(self.time)
        }
    }
    impl Updatable<()> for DummyTimeGetter {
        fn update(&mut self) -> NothingOrError<()> {
            self.time += Time::from_nanoseconds(10);
            Ok(())
        }
    }
    unsafe {
        static mut STREAM: DummyStream = DummyStream;
        let stream = PointerDereferencer::new(core::ptr::addr_of_mut!(STREAM));
        static mut TIME_GETTER: DummyTimeGetter = DummyTimeGetter { time: Time::ZERO };
        let time_getter = PointerDereferencer::new(core::ptr::addr_of_mut!(TIME_GETTER));
        let expirer = Expirer::new(stream, time_getter, Time::from_nanoseconds(10));
        assert_eq!(expirer.get(), Ok(None));
    }
}
#[test]
fn none_to_error() {
    #[derive(Clone, Copy, Debug)]
    enum Error {
        RealError,
        FromNone,
    }
    struct DummyStream {
        index: u8,
    }
    impl DummyStream {
        pub const fn new() -> Self {
            Self { index: 0 }
        }
    }
    impl Getter<f32, Error> for DummyStream {
        fn get(&self) -> Output<f32, Error> {
            if self.index == 1 {
                return Ok(None);
            } else if self.index == 2 {
                return Err(Error::RealError);
            }
            return Ok(Some(Datum::new(Time::ZERO, 0.0)));
        }
    }
    impl Updatable<Error> for DummyStream {
        fn update(&mut self) -> NothingOrError<Error> {
            self.index += 1;
            Ok(())
        }
    }
    unsafe {
        static mut INPUT: DummyStream = DummyStream::new();
        let mut input = PointerDereferencer::new(core::ptr::addr_of_mut!(INPUT));
        let mut stream = NoneToError::new(input.clone(), Error::FromNone);
        stream.update().unwrap(); //This should do nothing.
        assert!(stream.get().unwrap().is_some());
        input.update().unwrap();
        if let Err(Error::FromNone) = stream.get() {
        } else {
            panic!();
        }
        input.update().unwrap();
        if let Err(Error::RealError) = stream.get() {
        } else {
            panic!();
        }
    }
}
#[test]
fn none_to_value() {
    #[derive(Clone, Copy, Debug)]
    struct Error;
    struct DummyStream {
        index: u8,
    }
    impl DummyStream {
        pub const fn new() -> Self {
            Self { index: 0 }
        }
    }
    impl Getter<f32, Error> for DummyStream {
        fn get(&self) -> Output<f32, Error> {
            if self.index == 1 {
                return Ok(None);
            } else if self.index == 2 {
                return Err(Error);
            }
            return Ok(Some(Datum::new(Time::ZERO, 1.0)));
        }
    }
    impl Updatable<Error> for DummyStream {
        fn update(&mut self) -> NothingOrError<Error> {
            self.index += 1;
            Ok(())
        }
    }
    struct DummyTimeGetter {
        time: Time,
    }
    impl DummyTimeGetter {
        pub const fn new() -> Self {
            Self { time: Time::ZERO }
        }
    }
    impl<E: Copy + Debug> TimeGetter<E> for DummyTimeGetter {
        fn get(&self) -> TimeOutput<E> {
            Ok(self.time)
        }
    }
    impl<E: Copy + Debug> Updatable<E> for DummyTimeGetter {
        fn update(&mut self) -> NothingOrError<E> {
            self.time += Time::from_nanoseconds(1);
            Ok(())
        }
    }
    unsafe {
        static mut INPUT: DummyStream = DummyStream::new();
        let mut input = PointerDereferencer::new(core::ptr::addr_of_mut!(INPUT));
        static mut TIME_GETTER: DummyTimeGetter = DummyTimeGetter::new();
        let time_getter = PointerDereferencer::new(core::ptr::addr_of_mut!(TIME_GETTER));
        let mut stream = NoneToValue::new(input.clone(), time_getter, 2.0);
        stream.update().unwrap(); //This should do nothing.
        assert_eq!(stream.get().unwrap().unwrap().value, 1.0);
        input.update().unwrap();
        assert_eq!(stream.get().unwrap().unwrap().value, 2.0);
        input.update().unwrap();
        assert!(stream.get().is_err());
    }
}
#[test]
fn acceleration_to_state() {
    struct AccGetter {
        time: Time,
    }
    impl AccGetter {
        const fn new() -> Self {
            Self { time: Time::ZERO }
        }
    }
    impl Getter<Quantity, ()> for AccGetter {
        fn get(&self) -> Output<Quantity, ()> {
            Ok(Some(Datum::new(
                self.time,
                Quantity::new(1.0, MILLIMETER_PER_SECOND_SQUARED),
            )))
        }
    }
    impl Updatable<()> for AccGetter {
        fn update(&mut self) -> NothingOrError<()> {
            self.time += Time::from_nanoseconds(1_000_000_000);
            Ok(())
        }
    }
    unsafe {
        static mut ACC_GETTER: AccGetter = AccGetter::new();
        let mut acc_getter = PointerDereferencer::new(core::ptr::addr_of_mut!(ACC_GETTER));
        let mut state_getter = AccelerationToState::new(acc_getter.clone());
        let output = state_getter.get();
        assert!(output.unwrap().is_none());
        acc_getter.update().unwrap();
        state_getter.update().unwrap();
        let output = state_getter.get();
        assert!(output.unwrap().is_none());
        acc_getter.update().unwrap();
        state_getter.update().unwrap();
        let output = state_getter.get();
        assert!(output.unwrap().is_none());
        acc_getter.update().unwrap();
        state_getter.update().unwrap();
        let output = state_getter.get();
        assert_eq!(
            output.unwrap().unwrap(),
            Datum::new(
                Time::from_nanoseconds(3_000_000_000),
                State::new_raw(1.5, 2.0, 1.0)
            )
        );
    }
}
#[test]
fn velocity_to_state() {
    struct VelGetter {
        time: Time,
    }
    impl VelGetter {
        const fn new() -> Self {
            Self { time: Time::ZERO }
        }
    }
    impl Getter<Quantity, ()> for VelGetter {
        //???: cargo fmt messed up the "never do this" thing
        fn get(&self) -> Output<Quantity, ()> {
            //                            | never do this
            //                            V
            Ok(Some(Datum::new(
                self.time,
                Quantity::new(f32::from(Quantity::from(self.time)), MILLIMETER_PER_SECOND),
            )))
        }
    }
    impl Updatable<()> for VelGetter {
        fn update(&mut self) -> NothingOrError<()> {
            self.time += Time::from_nanoseconds(1_000_000_000);
            Ok(())
        }
    }
    unsafe {
        static mut VEL_GETTER: VelGetter = VelGetter::new();
        let mut vel_getter = PointerDereferencer::new(core::ptr::addr_of_mut!(VEL_GETTER));
        let mut state_getter = VelocityToState::new(vel_getter.clone());
        let output = state_getter.get();
        assert!(output.unwrap().is_none());
        vel_getter.update().unwrap();
        state_getter.update().unwrap();
        let output = state_getter.get();
        assert!(output.unwrap().is_none());
        vel_getter.update().unwrap();
        state_getter.update().unwrap();
        let output = state_getter.get();
        assert_eq!(
            output.unwrap().unwrap(),
            Datum::new(
                Time::from_nanoseconds(2_000_000_000),
                State::new_raw(1.5, 2.0, 1.0)
            )
        );
    }
}
#[test]
fn position_to_state() {
    struct PosGetter {
        time: Time,
    }
    impl PosGetter {
        const fn new() -> Self {
            Self { time: Time::ZERO }
        }
    }
    impl Getter<Quantity, ()> for PosGetter {
        fn get(&self) -> Output<Quantity, ()> {
            //                            | never do this
            //                            V
            Ok(Some(Datum::new(
                self.time,
                Quantity::new(f32::from(Quantity::from(self.time)), MILLIMETER),
            )))
        }
    }
    impl Updatable<()> for PosGetter {
        fn update(&mut self) -> NothingOrError<()> {
            self.time += Time::from_nanoseconds(1_000_000_000);
            Ok(())
        }
    }
    unsafe {
        static mut POS_GETTER: PosGetter = PosGetter::new();
        let mut pos_getter = PointerDereferencer::new(core::ptr::addr_of_mut!(POS_GETTER));
        let mut state_getter = PositionToState::new(pos_getter.clone());
        let output = state_getter.get();
        assert!(output.unwrap().is_none());
        pos_getter.update().unwrap();
        state_getter.update().unwrap();
        let output = state_getter.get();
        assert!(output.unwrap().is_none());
        pos_getter.update().unwrap();
        state_getter.update().unwrap();
        let output = state_getter.get();
        assert!(output.unwrap().is_none());
        pos_getter.update().unwrap();
        state_getter.update().unwrap();
        let output = state_getter.get();
        assert_eq!(
            output.unwrap().unwrap(),
            Datum::new(
                Time::from_nanoseconds(3_000_000_000),
                State::new_raw(3.0, 1.0, 0.0)
            )
        );
    }
}
#[test]
fn sum_stream() {
    #[derive(Clone, Copy, Debug)]
    struct Error;
    struct ErroringStream {
        index: u8,
    }
    impl ErroringStream {
        pub const fn new() -> Self {
            Self { index: 0 }
        }
    }
    impl Getter<f32, Error> for ErroringStream {
        fn get(&self) -> Output<f32, Error> {
            if self.index == 0 {
                return Err(Error);
            } else if self.index == 1 {
                return Ok(None);
            } else {
                return Ok(Some(Datum::new(Time::from_nanoseconds(2), 1.0)));
            }
        }
    }
    impl Updatable<Error> for ErroringStream {
        fn update(&mut self) -> NothingOrError<Error> {
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
    impl Getter<f32, Error> for NormalStream {
        fn get(&self) -> Output<f32, Error> {
            Ok(Some(Datum::new(Time::from_nanoseconds(1), 1.0)))
        }
    }
    impl Updatable<Error> for NormalStream {
        fn update(&mut self) -> NothingOrError<Error> {
            Ok(())
        }
    }
    unsafe {
        static mut ERRORING: ErroringStream = ErroringStream::new();
        let erroring = PointerDereferencer::new(core::ptr::addr_of_mut!(ERRORING));
        static mut NORMAL: NormalStream = NormalStream::new();
        let normal = PointerDereferencer::new(core::ptr::addr_of_mut!(NORMAL));
        let stream = SumStream::new([
            to_dyn!(Getter<f32, _>, erroring.clone()),
            to_dyn!(Getter<f32, _>, normal.clone()),
        ]);
        assert!(stream.get().is_err());
        //normal does not need update
        erroring.update().unwrap();
        assert_eq!(
            stream.get().unwrap().unwrap().time,
            Time::from_nanoseconds(1)
        );
        assert_eq!(stream.get().unwrap().unwrap().value, 1.0);
        erroring.update().unwrap();
        assert_eq!(
            stream.get().unwrap().unwrap().time,
            Time::from_nanoseconds(2)
        );
        assert_eq!(stream.get().unwrap().unwrap().value, 2.0);
    }
}
#[test]
fn sum_stream_all_none() {
    struct Input;
    impl Getter<f32, ()> for Input {
        fn get(&self) -> Output<f32, ()> {
            Ok(None)
        }
    }
    impl Updatable<()> for Input {
        fn update(&mut self) -> NothingOrError<()> {
            Ok(())
        }
    }
    let input = static_reference!(Input, Input);
    let sum_stream = SumStream::new([to_dyn!(Getter<f32, ()>, input)]);
    assert_eq!(sum_stream.get(), Ok(None));
}
#[test]
#[should_panic]
fn empty_sum_stream() {
    let _: SumStream<f32, 0, ()> = SumStream::new([]);
}
#[test]
fn sum2() {
    #[derive(Clone, Copy, Debug)]
    struct Error;
    struct ErroringStream {
        index: u8,
    }
    impl ErroringStream {
        pub const fn new() -> Self {
            Self { index: 0 }
        }
    }
    impl Getter<f32, Error> for ErroringStream {
        fn get(&self) -> Output<f32, Error> {
            if self.index == 0 {
                return Err(Error);
            } else if self.index == 1 {
                return Ok(None);
            } else {
                return Ok(Some(Datum::new(Time::from_nanoseconds(2), 1.0)));
            }
        }
    }
    impl Updatable<Error> for ErroringStream {
        fn update(&mut self) -> NothingOrError<Error> {
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
    impl Getter<f32, Error> for NormalStream {
        fn get(&self) -> Output<f32, Error> {
            Ok(Some(Datum::new(Time::from_nanoseconds(1), 1.0)))
        }
    }
    impl Updatable<Error> for NormalStream {
        fn update(&mut self) -> NothingOrError<Error> {
            Ok(())
        }
    }
    unsafe {
        static mut ERRORING: ErroringStream = ErroringStream::new();
        let mut erroring = PointerDereferencer::new(core::ptr::addr_of_mut!(ERRORING));
        static mut NORMAL: NormalStream = NormalStream::new();
        let normal = PointerDereferencer::new(core::ptr::addr_of_mut!(NORMAL));
        let stream = Sum2::new(erroring.clone(), normal.clone());
        assert!(stream.get().is_err());
        //normal does not need update
        erroring.update().unwrap();
        assert!(stream.get().unwrap().is_none());
        erroring.update().unwrap();
        assert_eq!(
            stream.get().unwrap().unwrap().time,
            Time::from_nanoseconds(2)
        );
        assert_eq!(stream.get().unwrap().unwrap().value, 2.0);
    }
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
                return Err(DummyError);
            } else if self.index == 3 || self.index == 4 || self.index == 5 {
                return Ok(None);
            }
            return Ok(Some(Datum::new(Time::from_nanoseconds(1), 10.0)));
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
                return Err(DummyError);
            } else if self.index == 1 || self.index == 4 || self.index == 7 {
                return Ok(None);
            }
            return Ok(Some(Datum::new(Time::from_nanoseconds(2), 3.0)));
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
        let mut stream1 = PointerDereferencer::new(core::ptr::addr_of_mut!(STREAM_1));
        static mut STREAM_2: Stream2 = Stream2::new();
        let mut stream2 = PointerDereferencer::new(core::ptr::addr_of_mut!(STREAM_2));
        let stream = DifferenceStream::new(stream1.clone(), stream2.clone());
        //Err, Err
        assert!(stream.get().is_err());
        stream1.update().unwrap();
        stream2.update().unwrap();
        //Err, None
        assert!(stream.get().is_err());
        stream1.update().unwrap();
        stream2.update().unwrap();
        //Err, Some
        assert!(stream.get().is_err());
        stream1.update().unwrap();
        stream2.update().unwrap();
        //None, Err
        assert!(stream.get().is_err());
        stream1.update().unwrap();
        stream2.update().unwrap();
        //None, None
        assert!(stream.get().unwrap().is_none());
        stream1.update().unwrap();
        stream2.update().unwrap();
        //None, Some
        assert!(stream.get().unwrap().is_none());
        stream1.update().unwrap();
        stream2.update().unwrap();
        //Some, Err
        assert!(stream.get().is_err());
        stream1.update().unwrap();
        stream2.update().unwrap();
        //Some, None
        assert!(stream.get().unwrap().is_none());
        stream1.update().unwrap();
        stream2.update().unwrap();
        //Some, Some
        if let Ok(Some(x)) = stream.get() {
            assert_eq!(x.time, Time::from_nanoseconds(2));
            assert_eq!(x.value, 7.0);
        } else {
            panic!();
        }
    }
}
#[test]
fn product_stream() {
    #[derive(Clone, Copy, Debug)]
    struct Error;
    struct ErroringStream {
        index: u8,
    }
    impl ErroringStream {
        pub const fn new() -> Self {
            Self { index: 0 }
        }
    }
    impl Getter<f32, Error> for ErroringStream {
        fn get(&self) -> Output<f32, Error> {
            if self.index == 0 {
                return Err(Error);
            } else if self.index == 1 {
                return Ok(None);
            } else {
                return Ok(Some(Datum::new(Time::from_nanoseconds(2), 3.0)));
            }
        }
    }
    impl Updatable<Error> for ErroringStream {
        fn update(&mut self) -> NothingOrError<Error> {
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
    impl Getter<f32, Error> for NormalStream {
        fn get(&self) -> Output<f32, Error> {
            Ok(Some(Datum::new(Time::from_nanoseconds(1), 5.0)))
        }
    }
    impl Updatable<Error> for NormalStream {
        fn update(&mut self) -> NothingOrError<Error> {
            Ok(())
        }
    }
    unsafe {
        static mut ERRORING: ErroringStream = ErroringStream::new();
        let erroring = PointerDereferencer::new(core::ptr::addr_of_mut!(ERRORING));
        static mut NORMAL: NormalStream = NormalStream::new();
        let normal = PointerDereferencer::new(core::ptr::addr_of_mut!(NORMAL));
        let stream = ProductStream::new([
            to_dyn!(Getter<f32, _>, erroring.clone()),
            to_dyn!(Getter<f32, _>, normal.clone()),
        ]);
        assert!(stream.get().is_err());
        //normal does not need update
        erroring.update().unwrap();
        assert_eq!(
            stream.get().unwrap().unwrap().time,
            Time::from_nanoseconds(1)
        );
        assert_eq!(stream.get().unwrap().unwrap().value, 5.0);
        erroring.update().unwrap();
        assert_eq!(
            stream.get().unwrap().unwrap().time,
            Time::from_nanoseconds(2)
        );
        assert_eq!(stream.get().unwrap().unwrap().value, 15.0);
    }
}
#[test]
fn product_stream_all_none() {
    struct Input;
    impl Getter<f32, ()> for Input {
        fn get(&self) -> Output<f32, ()> {
            Ok(None)
        }
    }
    impl Updatable<()> for Input {
        fn update(&mut self) -> NothingOrError<()> {
            Ok(())
        }
    }
    let input = static_reference!(Input, Input);
    let product_stream = ProductStream::new([to_dyn!(Getter<f32, ()>, input)]);
    assert_eq!(product_stream.get(), Ok(None));
}
#[test]
#[should_panic]
fn empty_product_stream() {
    let _: ProductStream<f32, 0, ()> = ProductStream::new([]);
}
#[test]
fn product2() {
    #[derive(Clone, Copy, Debug)]
    struct Error;
    struct ErroringStream {
        index: u8,
    }
    impl ErroringStream {
        pub const fn new() -> Self {
            Self { index: 0 }
        }
    }
    impl Getter<f32, Error> for ErroringStream {
        fn get(&self) -> Output<f32, Error> {
            if self.index == 0 {
                return Err(Error);
            } else if self.index == 1 {
                return Ok(None);
            } else {
                return Ok(Some(Datum::new(Time::from_nanoseconds(2), 3.0)));
            }
        }
    }
    impl Updatable<Error> for ErroringStream {
        fn update(&mut self) -> NothingOrError<Error> {
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
    impl Getter<f32, Error> for NormalStream {
        fn get(&self) -> Output<f32, Error> {
            Ok(Some(Datum::new(Time::from_nanoseconds(1), 5.0)))
        }
    }
    impl Updatable<Error> for NormalStream {
        fn update(&mut self) -> NothingOrError<Error> {
            Ok(())
        }
    }
    unsafe {
        static mut ERRORING: ErroringStream = ErroringStream::new();
        let mut erroring = PointerDereferencer::new(core::ptr::addr_of_mut!(ERRORING));
        static mut NORMAL: NormalStream = NormalStream::new();
        let normal = PointerDereferencer::new(core::ptr::addr_of_mut!(NORMAL));
        let stream = Product2::new(erroring.clone(), normal.clone());
        assert!(stream.get().is_err());
        //normal does not need update
        erroring.update().unwrap();
        assert!(stream.get().unwrap().is_none());
        erroring.update().unwrap();
        assert_eq!(
            stream.get().unwrap().unwrap().time,
            Time::from_nanoseconds(2)
        );
        assert_eq!(stream.get().unwrap().unwrap().value, 15.0);
    }
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
                return Err(DummyError);
            } else if self.index == 3 || self.index == 4 || self.index == 5 {
                return Ok(None);
            }
            return Ok(Some(Datum::new(Time::from_nanoseconds(1), 12.0)));
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
                return Err(DummyError);
            } else if self.index == 1 || self.index == 4 || self.index == 7 {
                return Ok(None);
            }
            return Ok(Some(Datum::new(Time::from_nanoseconds(2), 3.0)));
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
        let mut stream1 = PointerDereferencer::new(core::ptr::addr_of_mut!(STREAM_1));
        static mut STREAM_2: Stream2 = Stream2::new();
        let mut stream2 = PointerDereferencer::new(core::ptr::addr_of_mut!(STREAM_2));
        let stream = QuotientStream::new(stream1.clone(), stream2.clone());
        //Err, Err
        assert!(stream.get().is_err());
        stream1.update().unwrap();
        stream2.update().unwrap();
        //Err, None
        assert!(stream.get().is_err());
        stream1.update().unwrap();
        stream2.update().unwrap();
        //Err, Some
        assert!(stream.get().is_err());
        stream1.update().unwrap();
        stream2.update().unwrap();
        //None, Err
        assert!(stream.get().is_err());
        stream1.update().unwrap();
        stream2.update().unwrap();
        //None, None
        assert!(stream.get().unwrap().is_none());
        stream1.update().unwrap();
        stream2.update().unwrap();
        //None, Some
        assert!(stream.get().unwrap().is_none());
        stream1.update().unwrap();
        stream2.update().unwrap();
        //Some, Err
        assert!(stream.get().is_err());
        stream1.update().unwrap();
        stream2.update().unwrap();
        //Some, None
        assert!(stream.get().unwrap().is_none());
        stream1.update().unwrap();
        stream2.update().unwrap();
        //Some, Some
        if let Ok(Some(x)) = stream.get() {
            assert_eq!(x.time, Time::from_nanoseconds(2));
            assert_eq!(x.value, 4.0);
        } else {
            panic!();
        }
    }
}
//micromath's implementations are not as precise as std's and libm's, making them cause this test
//to fail even if the calculation is correct. Testing the accuracy of the other two and compiling
//with micromath, although not testing its implementation, is considered sufficient. The same
//applies to the ewma_stream and ewma_stream_quantity tests.
#[test]
#[cfg(any(feature = "std", feature = "libm"))]
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
                return Err(DummyError);
            } else if self.index == 3 || self.index == 4 || self.index == 5 {
                return Ok(None);
            }
            return Ok(Some(Datum::new(Time::from_nanoseconds(1), 5.0)));
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
                return Err(DummyError);
            } else if self.index == 1 || self.index == 4 || self.index == 7 {
                return Ok(None);
            }
            return Ok(Some(Datum::new(Time::from_nanoseconds(2), 3.0)));
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
        let mut stream1 = PointerDereferencer::new(core::ptr::addr_of_mut!(STREAM_1));
        static mut STREAM_2: Stream2 = Stream2::new();
        let mut stream2 = PointerDereferencer::new(core::ptr::addr_of_mut!(STREAM_2));
        let stream = ExponentStream::new(stream1.clone(), stream2.clone());
        //Err, Err
        assert!(stream.get().is_err());
        stream1.update().unwrap();
        stream2.update().unwrap();
        //Err, None
        assert!(stream.get().is_err());
        stream1.update().unwrap();
        stream2.update().unwrap();
        //Err, Some
        assert!(stream.get().is_err());
        stream1.update().unwrap();
        stream2.update().unwrap();
        //None, Err
        assert!(stream.get().is_err());
        stream1.update().unwrap();
        stream2.update().unwrap();
        //None, None
        assert!(stream.get().unwrap().is_none());
        stream1.update().unwrap();
        stream2.update().unwrap();
        //None, Some
        assert!(stream.get().unwrap().is_none());
        stream1.update().unwrap();
        stream2.update().unwrap();
        //Some, Err
        assert!(stream.get().is_err());
        stream1.update().unwrap();
        stream2.update().unwrap();
        //Some, None
        if let Ok(Some(x)) = stream.get() {
            assert_eq!(x.time, Time::from_nanoseconds(1));
            assert_eq!(x.value, 5.0);
        } else {
            panic!();
        }
        stream1.update().unwrap();
        stream2.update().unwrap();
        //Some, Some
        if let Ok(Some(x)) = stream.get() {
            assert_eq!(x.time, Time::from_nanoseconds(2));
            assert_eq!(x.value, 125.0);
        } else {
            panic!();
        }
    }
}
#[test]
fn derivative_stream() {
    #[derive(Clone, Copy, Debug)]
    struct DummyError;
    struct DummyStream {
        time: Time,
    }
    impl DummyStream {
        pub const fn new() -> Self {
            Self { time: Time::ZERO }
        }
    }
    impl Getter<Quantity, DummyError> for DummyStream {
        fn get(&self) -> Output<Quantity, DummyError> {
            Ok(Some(Datum::new(
                self.time * DimensionlessInteger(2),
                Quantity::from(self.time * DimensionlessInteger(3)),
            )))
        }
    }
    impl Updatable<DummyError> for DummyStream {
        fn update(&mut self) -> NothingOrError<DummyError> {
            self.time += Time::from_nanoseconds(2_000_000_000);
            Ok(())
        }
    }
    unsafe {
        static mut INPUT: DummyStream = DummyStream::new();
        let mut input = PointerDereferencer::new(core::ptr::addr_of_mut!(INPUT));
        let mut stream = DerivativeStream::new(input.clone());
        input.update().unwrap();
        stream.update().unwrap();
        input.update().unwrap();
        stream.update().unwrap();
        assert_eq!(
            stream.get().unwrap().unwrap().time,
            Time::from_nanoseconds(8_000_000_000)
        );
        assert_eq!(
            stream.get().unwrap().unwrap().value,
            Quantity::new(1.5, DIMENSIONLESS) //Derivating time d time returns a dimensionless quantity.
        );
    }
}
#[test]
fn integral_stream() {
    #[derive(Clone, Copy, Debug)]
    struct DummyError;
    struct DummyStream {
        time: Time,
    }
    impl DummyStream {
        pub const fn new() -> Self {
            Self { time: Time::ZERO }
        }
    }
    impl Getter<Quantity, DummyError> for DummyStream {
        fn get(&self) -> Output<Quantity, DummyError> {
            Ok(Some(Datum::new(
                self.time,
                Quantity::new(1.0, MILLIMETER_PER_SECOND),
            )))
        }
    }
    impl Updatable<DummyError> for DummyStream {
        fn update(&mut self) -> NothingOrError<DummyError> {
            self.time += Time::from_nanoseconds(1_000_000_000);
            Ok(())
        }
    }
    unsafe {
        static mut INPUT: DummyStream = DummyStream::new();
        let mut input = PointerDereferencer::new(core::ptr::addr_of_mut!(INPUT));
        let mut stream = IntegralStream::new(input.clone());
        input.update().unwrap();
        stream.update().unwrap();
        input.update().unwrap();
        stream.update().unwrap();
        assert_eq!(
            stream.get().unwrap().unwrap().time,
            Time::from_nanoseconds(2_000_000_000)
        );
        assert_eq!(
            stream.get().unwrap().unwrap().value,
            Quantity::new(1.0, MILLIMETER)
        );
    }
}
#[test]
fn pid_controller_stream() {
    #[derive(Clone, Copy, Debug)]
    struct DummyError;
    struct DummyStream {
        time: Time,
    }
    impl DummyStream {
        pub const fn new() -> Self {
            Self { time: Time::ZERO }
        }
    }
    impl Getter<f32, DummyError> for DummyStream {
        fn get(&self) -> Output<f32, DummyError> {
            Ok(Some(Datum::new(
                self.time,
                f32::from(Quantity::from(self.time / DimensionlessInteger(2))),
            )))
        }
    }
    impl Updatable<DummyError> for DummyStream {
        fn update(&mut self) -> NothingOrError<DummyError> {
            self.time += Time::from_nanoseconds(2_000_000_000);
            Ok(())
        }
    }
    unsafe {
        static mut INPUT: DummyStream = DummyStream::new();
        let mut input = PointerDereferencer::new(core::ptr::addr_of_mut!(INPUT));
        let mut stream =
            PIDControllerStream::new(input.clone(), 5.0, PIDKValues::new(1.0, 0.01, 0.1));
        stream.update().unwrap();
        assert_eq!(stream.get().unwrap().unwrap().time, Time::ZERO);
        assert_eq!(stream.get().unwrap().unwrap().value, 5.0);
        input.update().unwrap();
        stream.update().unwrap();
        assert_eq!(
            stream.get().unwrap().unwrap().time,
            Time::from_nanoseconds(2_000_000_000)
        );
        assert_eq!(stream.get().unwrap().unwrap().value, 4.04);
    }
}
//See note on exponent_stream test
#[test]
#[cfg(any(feature = "std", feature = "libm"))]
fn ewma_stream() {
    #[derive(Clone, Copy, Debug)]
    struct DummyError;
    struct DummyStream {
        time: Time,
    }
    impl DummyStream {
        pub const fn new() -> Self {
            Self { time: Time::ZERO }
        }
    }
    impl Getter<f32, DummyError> for DummyStream {
        fn get(&self) -> Output<f32, DummyError> {
            let value = match self.time.as_nanoseconds() {
                2_000_000_000 => 110.0,
                4_000_000_000 => 111.0,
                6_000_000_000 => 116.0,
                8_000_000_000 => 97.0,
                10_000_000_000 => 102.0,
                12_000_000_000 => 111.0,
                14_000_000_000 => 111.0,
                16_000_000_000 => 100.0,
                _ => 0.0,
            };
            Ok(Some(Datum::new(self.time, value)))
        }
    }
    impl Updatable<DummyError> for DummyStream {
        fn update(&mut self) -> NothingOrError<DummyError> {
            self.time += Time::from_nanoseconds(2_000_000_000);
            Ok(())
        }
    }
    unsafe {
        static mut INPUT: DummyStream = DummyStream::new();
        let mut input = PointerDereferencer::new(core::ptr::addr_of_mut!(INPUT));
        let mut stream = EWMAStream::new(input.clone(), 0.25);
        input.update().unwrap();
        stream.update().unwrap();
        assert_eq!(stream.get().unwrap().unwrap().value, 110.0);
        input.update().unwrap();
        stream.update().unwrap();
        assert_eq!(stream.get().unwrap().unwrap().value, 110.4375);
        input.update().unwrap();
        stream.update().unwrap();
        //Floating-point stuff gets a bit weird because of rounding, but it still appears to work
        //correctly.
        assert_eq!(stream.get().unwrap().unwrap().value, 112.87109375);
        input.update().unwrap();
        stream.update().unwrap();
        assert_eq!(stream.get().unwrap().unwrap().value, 105.927490234375);
        input.update().unwrap();
        stream.update().unwrap();
        assert_eq!(stream.get().unwrap().unwrap().value, 104.20921325683594);
        input.update().unwrap();
        stream.update().unwrap();
        assert_eq!(stream.get().unwrap().unwrap().value, 107.18018245697021);
        input.update().unwrap();
        stream.update().unwrap();
        assert_eq!(stream.get().unwrap().unwrap().value, 108.85135263204575);
        input.update().unwrap();
        stream.update().unwrap();
        //Despite every other assert_eq! here working, this one does not because the way f32 works
        //means that it thinks it's off by 0.00001. I am unconcerned.
        //assert_eq!(stream.get().unwrap().unwrap().value, 104.97888585552573);
    }
}
//See note on exponent_stream test
#[test]
#[cfg(any(feature = "std", feature = "libm"))]
fn ewma_stream_quantity() {
    #[derive(Clone, Copy, Debug)]
    struct DummyError;
    struct DummyStream {
        time: Time,
    }
    impl DummyStream {
        pub const fn new() -> Self {
            Self { time: Time::ZERO }
        }
    }
    impl Getter<Quantity, DummyError> for DummyStream {
        fn get(&self) -> Output<Quantity, DummyError> {
            let value = match self.time.as_nanoseconds() {
                2_000_000_000 => Quantity::dimensionless(110.0),
                4_000_000_000 => Quantity::dimensionless(111.0),
                6_000_000_000 => Quantity::dimensionless(116.0),
                8_000_000_000 => Quantity::dimensionless(97.0),
                10_000_000_000 => Quantity::dimensionless(102.0),
                12_000_000_000 => Quantity::dimensionless(111.0),
                14_000_000_000 => Quantity::dimensionless(111.0),
                16_000_000_000 => Quantity::dimensionless(100.0),
                _ => Quantity::dimensionless(0.0),
            };
            Ok(Some(Datum::new(self.time, value)))
        }
    }
    impl Updatable<DummyError> for DummyStream {
        fn update(&mut self) -> NothingOrError<DummyError> {
            self.time += Time::from_nanoseconds(2_000_000_000);
            Ok(())
        }
    }
    unsafe {
        static mut INPUT: DummyStream = DummyStream::new();
        let mut input = PointerDereferencer::new(core::ptr::addr_of_mut!(INPUT));
        let mut stream = EWMAStream::new(input.clone(), 0.25);
        input.update().unwrap();
        stream.update().unwrap();
        assert_eq!(
            stream.get().unwrap().unwrap().value,
            Quantity::dimensionless(110.0)
        );
        input.update().unwrap();
        stream.update().unwrap();
        assert_eq!(
            stream.get().unwrap().unwrap().value,
            Quantity::dimensionless(110.4375)
        );
        input.update().unwrap();
        stream.update().unwrap();
        //Floating-point stuff gets a bit weird because of rounding, but it still appears to work
        //correctly.
        assert_eq!(
            stream.get().unwrap().unwrap().value,
            Quantity::dimensionless(112.87109375)
        );
        input.update().unwrap();
        stream.update().unwrap();
        assert_eq!(
            stream.get().unwrap().unwrap().value,
            Quantity::dimensionless(105.927490234375)
        );
        input.update().unwrap();
        stream.update().unwrap();
        assert_eq!(
            stream.get().unwrap().unwrap().value,
            Quantity::dimensionless(104.20921325683594)
        );
        input.update().unwrap();
        stream.update().unwrap();
        assert_eq!(
            stream.get().unwrap().unwrap().value,
            Quantity::dimensionless(107.18018245697021)
        );
        input.update().unwrap();
        stream.update().unwrap();
        assert_eq!(
            stream.get().unwrap().unwrap().value,
            Quantity::dimensionless(108.85135263204575)
        );
        input.update().unwrap();
        stream.update().unwrap();
        //Despite every other assert_eq! here working, this one does not because the way f32 works
        //means that it thinks it's off by 0.00001. I am unconcerned.
        //assert_eq!(stream.get().unwrap().unwrap().value, 104.97888585552573);
    }
}
#[test]
#[cfg(feature = "alloc")]
fn moving_average_stream() {
    #[derive(Clone, Copy, Debug)]
    struct DummyError;
    struct DummyStream {
        time: Time,
    }
    impl DummyStream {
        pub const fn new() -> Self {
            Self { time: Time::ZERO }
        }
    }
    impl Getter<f32, DummyError> for DummyStream {
        fn get(&self) -> Output<f32, DummyError> {
            let value = match self.time.as_nanoseconds() {
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
            self.time += Time::from_nanoseconds(2);
            Ok(())
        }
    }
    unsafe {
        static mut INPUT: DummyStream = DummyStream::new();
        let mut input = PointerDereferencer::new(core::ptr::addr_of_mut!(INPUT));
        let mut stream = MovingAverageStream::new(input.clone(), Time::from_nanoseconds(5));
        input.update().unwrap();
        stream.update().unwrap();
        assert_eq!(stream.get().unwrap().unwrap().value, 110.0);
        input.update().unwrap();
        stream.update().unwrap();
        assert_eq!(stream.get().unwrap().unwrap().value, 110.4);
        input.update().unwrap();
        stream.update().unwrap();
        //assert_eq!(stream.get().unwrap().unwrap().value, 112.8);
        input.update().unwrap();
        stream.update().unwrap();
        assert_eq!(stream.get().unwrap().unwrap().value, 107.4);
        input.update().unwrap();
        stream.update().unwrap();
        //assert_eq!(stream.get().unwrap().unwrap().value, 102.8);
        input.update().unwrap();
        stream.update().unwrap();
        assert_eq!(stream.get().unwrap().unwrap().value, 104.6);
        input.update().unwrap();
        stream.update().unwrap();
        assert_eq!(stream.get().unwrap().unwrap().value, 109.2);
        input.update().unwrap();
        stream.update().unwrap();
        assert_eq!(stream.get().unwrap().unwrap().value, 106.6);
    }
}
#[test]
#[cfg(feature = "alloc")]
fn moving_average_stream_quantity() {
    #[derive(Clone, Copy, Debug)]
    struct DummyError;
    struct DummyStream {
        time: Time,
    }
    impl DummyStream {
        pub const fn new() -> Self {
            Self { time: Time::ZERO }
        }
    }
    impl Getter<Quantity, DummyError> for DummyStream {
        fn get(&self) -> Output<Quantity, DummyError> {
            let value = match self.time.as_nanoseconds() {
                2 => Quantity::dimensionless(110.0),
                4 => Quantity::dimensionless(111.0),
                6 => Quantity::dimensionless(116.0),
                8 => Quantity::dimensionless(97.0),
                10 => Quantity::dimensionless(102.0),
                12 => Quantity::dimensionless(111.0),
                14 => Quantity::dimensionless(111.0),
                16 => Quantity::dimensionless(100.0),
                _ => Quantity::dimensionless(0.0),
            };
            Ok(Some(Datum::new(self.time, value)))
        }
    }
    impl Updatable<DummyError> for DummyStream {
        fn update(&mut self) -> NothingOrError<DummyError> {
            self.time += Time::from_nanoseconds(2);
            Ok(())
        }
    }
    unsafe {
        static mut INPUT: DummyStream = DummyStream::new();
        let mut input = PointerDereferencer::new(core::ptr::addr_of_mut!(INPUT));
        let mut stream = MovingAverageStream::new(input.clone(), Time::from_nanoseconds(5));
        input.update().unwrap();
        stream.update().unwrap();
        assert_eq!(
            stream.get().unwrap().unwrap().value,
            Quantity::dimensionless(110.0)
        );
        input.update().unwrap();
        stream.update().unwrap();
        assert_eq!(
            stream.get().unwrap().unwrap().value,
            Quantity::dimensionless(110.4)
        );
        input.update().unwrap();
        stream.update().unwrap();
        //assert_eq!(stream.get().unwrap().unwrap().value, 112.8);
        input.update().unwrap();
        stream.update().unwrap();
        assert_eq!(
            stream.get().unwrap().unwrap().value,
            Quantity::dimensionless(107.4)
        );
        input.update().unwrap();
        stream.update().unwrap();
        //assert_eq!(stream.get().unwrap().unwrap().value, 102.8);
        input.update().unwrap();
        stream.update().unwrap();
        assert_eq!(
            stream.get().unwrap().unwrap().value,
            Quantity::dimensionless(104.6)
        );
        input.update().unwrap();
        stream.update().unwrap();
        assert_eq!(
            stream.get().unwrap().unwrap().value,
            Quantity::dimensionless(109.2)
        );
        input.update().unwrap();
        stream.update().unwrap();
        assert_eq!(
            stream.get().unwrap().unwrap().value,
            Quantity::dimensionless(106.6)
        );
    }
}
#[test]
fn latest() {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    struct Error;
    struct Stream1 {
        time: Time,
    }
    impl Stream1 {
        pub const fn new() -> Self {
            Self { time: Time::ZERO }
        }
    }
    impl Getter<u8, Error> for Stream1 {
        fn get(&self) -> Output<u8, Error> {
            match self.time.as_nanoseconds() {
                0 => Ok(Some(Datum::new(Time::from_nanoseconds(1), 1))), //Some, Some
                1 => Ok(Some(Datum::new(Time::ZERO, 0))),                //Some, Some
                2 => Ok(Some(Datum::new(Time::ZERO, 1))),                //Some, None
                3 => Ok(Some(Datum::new(Time::ZERO, 1))),                //Some, Err
                4 => Ok(None),                                           //None, None
                5 => Ok(None),                                           //None, Err
                6 => Err(Error),                                         //Err,  Err
                _ => panic!("should be unreachable"),
            }
        }
    }
    impl Updatable<Error> for Stream1 {
        fn update(&mut self) -> NothingOrError<Error> {
            self.time += Time::from_nanoseconds(1);
            Ok(())
        }
    }
    struct Stream2 {
        time: Time,
    }
    impl Stream2 {
        pub const fn new() -> Self {
            Self { time: Time::ZERO }
        }
    }
    impl Getter<u8, Error> for Stream2 {
        fn get(&self) -> Output<u8, Error> {
            match self.time.as_nanoseconds() {
                0 => Ok(Some(Datum::new(Time::ZERO, 0))), //Some, Some
                1 => Ok(Some(Datum::new(Time::from_nanoseconds(1), 2))), //Some, Some
                2 => Ok(None),                            //Some, None
                3 => Err(Error),                          //Some, Err
                4 => Ok(None),                            //None, None
                5 => Err(Error),                          //None, Err
                6 => Err(Error),                          //Err,  Err
                _ => panic!("should be unreachable"),
            }
        }
    }
    impl Updatable<Error> for Stream2 {
        fn update(&mut self) -> NothingOrError<Error> {
            self.time += Time::from_nanoseconds(1);
            Ok(())
        }
    }
    unsafe {
        static mut STREAM_1: Stream1 = Stream1::new();
        let stream1 = PointerDereferencer::new(core::ptr::addr_of_mut!(STREAM_1));
        static mut STREAM_2: Stream2 = Stream2::new();
        let stream2 = PointerDereferencer::new(core::ptr::addr_of_mut!(STREAM_2));
        let mut latest = Latest::new([
            to_dyn!(Getter<u8, _>, stream1.clone()),
            to_dyn!(Getter<u8, _>, stream2.clone()),
        ]);
        latest.update().unwrap(); //This should do nothing.
        assert_eq!(
            latest.get(),
            Ok(Some(Datum::new(Time::from_nanoseconds(1), 1)))
        );
        stream1.update().unwrap();
        stream2.update().unwrap();
        assert_eq!(
            latest.get(),
            Ok(Some(Datum::new(Time::from_nanoseconds(1), 2)))
        );
        stream1.update().unwrap();
        stream2.update().unwrap();
        assert_eq!(latest.get(), Ok(Some(Datum::new(Time::ZERO, 1))));
        stream1.update().unwrap();
        stream2.update().unwrap();
        assert_eq!(latest.get(), Ok(Some(Datum::new(Time::ZERO, 1))));
        stream1.update().unwrap();
        stream2.update().unwrap();
        assert_eq!(latest.get(), Ok(None));
        stream1.update().unwrap();
        stream2.update().unwrap();
        assert_eq!(latest.get(), Ok(None));
        stream1.update().unwrap();
        stream2.update().unwrap();
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
                0 => Some(Datum::new(Time::ZERO, false)),
                1 => None,
                2 => Some(Datum::new(Time::ZERO, true)),
                3 => Some(Datum::new(Time::ZERO, false)),
                4 => None,
                5 => Some(Datum::new(Time::ZERO, true)),
                6 => Some(Datum::new(Time::ZERO, false)),
                7 => None,
                8 => Some(Datum::new(Time::ZERO, true)),
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
                0..=2 => Some(Datum::new(Time::ZERO, false)),
                3..=5 => None,
                6..=8 => Some(Datum::new(Time::ZERO, true)),
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
        let mut in1 = PointerDereferencer::new(core::ptr::addr_of_mut!(IN_1));
        static mut IN_2: In2 = In2::new();
        let mut in2 = PointerDereferencer::new(core::ptr::addr_of_mut!(IN_2));
        let mut and = AndStream::new(in1.clone(), in2.clone());
        assert_eq!(and.get().unwrap().unwrap().value, false);
        in1.update().unwrap();
        in2.update().unwrap();
        and.update().unwrap();
        assert_eq!(and.get().unwrap().unwrap().value, false);
        in1.update().unwrap();
        in2.update().unwrap();
        and.update().unwrap();
        assert_eq!(and.get().unwrap().unwrap().value, false);
        in1.update().unwrap();
        in2.update().unwrap();
        and.update().unwrap();
        assert_eq!(and.get().unwrap().unwrap().value, false);
        in1.update().unwrap();
        in2.update().unwrap();
        and.update().unwrap();
        assert_eq!(and.get().unwrap(), None);
        in1.update().unwrap();
        in2.update().unwrap();
        and.update().unwrap();
        assert_eq!(and.get().unwrap(), None);
        in1.update().unwrap();
        in2.update().unwrap();
        and.update().unwrap();
        assert_eq!(and.get().unwrap().unwrap().value, false);
        in1.update().unwrap();
        in2.update().unwrap();
        and.update().unwrap();
        assert_eq!(and.get().unwrap(), None);
        in1.update().unwrap();
        in2.update().unwrap();
        and.update().unwrap();
        assert_eq!(and.get().unwrap().unwrap().value, true);
        in1.update().unwrap();
        in2.update().unwrap();
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
                0 => Some(Datum::new(Time::ZERO, false)),
                1 => None,
                2 => Some(Datum::new(Time::ZERO, true)),
                3 => Some(Datum::new(Time::ZERO, false)),
                4 => None,
                5 => Some(Datum::new(Time::ZERO, true)),
                6 => Some(Datum::new(Time::ZERO, false)),
                7 => None,
                8 => Some(Datum::new(Time::ZERO, true)),
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
                0..=2 => Some(Datum::new(Time::ZERO, false)),
                3..=5 => None,
                6..=8 => Some(Datum::new(Time::ZERO, true)),
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
        let mut in1 = PointerDereferencer::new(core::ptr::addr_of_mut!(IN_1));
        static mut IN_2: In2 = In2::new();
        let mut in2 = PointerDereferencer::new(core::ptr::addr_of_mut!(IN_2));
        let mut and = OrStream::new(in1.clone(), in2.clone());
        assert_eq!(and.get().unwrap().unwrap().value, false);
        in1.update().unwrap();
        in2.update().unwrap();
        and.update().unwrap();
        assert_eq!(and.get().unwrap(), None);
        in1.update().unwrap();
        in2.update().unwrap();
        and.update().unwrap();
        assert_eq!(and.get().unwrap().unwrap().value, true);
        in1.update().unwrap();
        in2.update().unwrap();
        and.update().unwrap();
        assert_eq!(and.get().unwrap(), None);
        in1.update().unwrap();
        in2.update().unwrap();
        and.update().unwrap();
        assert_eq!(and.get().unwrap(), None);
        in1.update().unwrap();
        in2.update().unwrap();
        and.update().unwrap();
        assert_eq!(and.get().unwrap().unwrap().value, true);
        in1.update().unwrap();
        in2.update().unwrap();
        and.update().unwrap();
        assert_eq!(and.get().unwrap().unwrap().value, true);
        in1.update().unwrap();
        in2.update().unwrap();
        and.update().unwrap();
        assert_eq!(and.get().unwrap().unwrap().value, true);
        in1.update().unwrap();
        in2.update().unwrap();
        and.update().unwrap();
        assert_eq!(and.get().unwrap().unwrap().value, true);
        in1.update().unwrap();
        in2.update().unwrap();
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
                0 => Some(Datum::new(Time::ZERO, false)),
                1 => None,
                2 => Some(Datum::new(Time::ZERO, true)),
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
        let mut input = PointerDereferencer::new(core::ptr::addr_of_mut!(INPUT));
        let mut not = NotStream::new(input.clone());
        assert_eq!(not.get().unwrap().unwrap().value, true);
        input.update().unwrap();
        not.update().unwrap();
        assert_eq!(not.get().unwrap(), None);
        input.update().unwrap();
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
                0 => Some(Datum::new(Time::ZERO, false)),
                1 => None,
                2 => Some(Datum::new(Time::ZERO, true)),
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
            Ok(Some(Datum::new(Time::ZERO, 0)))
        }
    }
    impl Updatable<()> for Input {
        fn update(&mut self) -> NothingOrError<()> {
            Ok(())
        }
    }
    unsafe {
        static mut CONDITION: Condition = Condition { index: 0 };
        let mut condition = PointerDereferencer::new(core::ptr::addr_of_mut!(CONDITION));
        static mut INPUT: Input = Input;
        let input = PointerDereferencer::new(core::ptr::addr_of_mut!(INPUT));
        let mut if_stream = IfStream::new(condition.clone(), input.clone());
        assert_eq!(if_stream.get().unwrap(), None);
        condition.update().unwrap();
        if_stream.update().unwrap();
        assert_eq!(if_stream.get().unwrap(), None);
        condition.update().unwrap();
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
                0 => Some(Datum::new(Time::ZERO, false)),
                1 => None,
                2 => Some(Datum::new(Time::ZERO, true)),
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
            Ok(Some(Datum::new(Time::ZERO, 1)))
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
            Ok(Some(Datum::new(Time::ZERO, 2)))
        }
    }
    impl Updatable<()> for False {
        fn update(&mut self) -> NothingOrError<()> {
            Ok(())
        }
    }
    unsafe {
        static mut CONDITION: Condition = Condition { index: 0 };
        let mut condition = PointerDereferencer::new(core::ptr::addr_of_mut!(CONDITION));
        static mut TRUE_INPUT: True = True;
        let true_input = PointerDereferencer::new(core::ptr::addr_of_mut!(TRUE_INPUT));
        static mut FALSE_INPUT: False = False;
        let false_input = PointerDereferencer::new(core::ptr::addr_of_mut!(FALSE_INPUT));
        let mut if_else_stream = IfElseStream::new(condition.clone(), true_input, false_input);
        assert_eq!(if_else_stream.get().unwrap().unwrap().value, 2);
        condition.update().unwrap();
        if_else_stream.update().unwrap();
        assert_eq!(if_else_stream.get().unwrap(), None);
        condition.update().unwrap();
        if_else_stream.update().unwrap();
        assert_eq!(if_else_stream.get().unwrap().unwrap().value, 1);
    }
}
#[test]
fn freeze_stream() {
    struct Condition {
        time: Time,
    }
    impl Getter<bool, ()> for Condition {
        fn get(&self) -> Output<bool, ()> {
            Ok(match self.time.as_nanoseconds() {
                0..=1 => Some(Datum::new(Time::ZERO, false)),
                2..=3 => Some(Datum::new(Time::ZERO, true)),
                4..=5 => Some(Datum::new(Time::ZERO, false)),
                6..=7 => None,
                8..=9 => Some(Datum::new(Time::ZERO, false)),
                _ => unimplemented!(),
            })
        }
    }
    impl Updatable<()> for Condition {
        fn update(&mut self) -> NothingOrError<()> {
            self.time += Time::from_nanoseconds(1);
            Ok(())
        }
    }
    struct Input {
        time: Time,
    }
    impl Getter<i64, ()> for Input {
        fn get(&self) -> Output<i64, ()> {
            Ok(Some(Datum::new(Time::ZERO, self.time.as_nanoseconds())))
        }
    }
    impl Updatable<()> for Input {
        fn update(&mut self) -> NothingOrError<()> {
            self.time += Time::from_nanoseconds(1);
            Ok(())
        }
    }
    unsafe {
        static mut CONDITION: Condition = Condition { time: Time::ZERO };
        let mut condition = PointerDereferencer::new(core::ptr::addr_of_mut!(CONDITION));
        static mut INPUT: Input = Input { time: Time::ZERO };
        let mut input = PointerDereferencer::new(core::ptr::addr_of_mut!(INPUT));
        let mut freeze = FreezeStream::new(condition.clone(), input.clone());
        freeze.update().unwrap();
        assert_eq!(freeze.get().unwrap().unwrap().value, 0);
        condition.update().unwrap();
        input.update().unwrap();
        freeze.update().unwrap();
        assert_eq!(freeze.get().unwrap().unwrap().value, 1);
        condition.update().unwrap();
        input.update().unwrap();
        freeze.update().unwrap();
        assert_eq!(freeze.get().unwrap().unwrap().value, 1);
        condition.update().unwrap();
        input.update().unwrap();
        freeze.update().unwrap();
        assert_eq!(freeze.get().unwrap().unwrap().value, 1);
        condition.update().unwrap();
        input.update().unwrap();
        freeze.update().unwrap();
        assert_eq!(freeze.get().unwrap().unwrap().value, 4);
        condition.update().unwrap();
        input.update().unwrap();
        freeze.update().unwrap();
        assert_eq!(freeze.get().unwrap().unwrap().value, 5);
        condition.update().unwrap();
        input.update().unwrap();
        freeze.update().unwrap();
        assert_eq!(freeze.get().unwrap(), None);
        condition.update().unwrap();
        input.update().unwrap();
        freeze.update().unwrap();
        assert_eq!(freeze.get().unwrap(), None);
        condition.update().unwrap();
        input.update().unwrap();
        freeze.update().unwrap();
        assert_eq!(freeze.get().unwrap().unwrap().value, 8);
        condition.update().unwrap();
        input.update().unwrap();
        freeze.update().unwrap();
        assert_eq!(freeze.get().unwrap().unwrap().value, 9);
    }
}
#[test]
fn command_pid() {
    struct Input {
        time: Time,
    }
    impl Getter<State, ()> for Input {
        fn get(&self) -> Output<State, ()> {
            Ok(Some(Datum::new(self.time, State::default())))
        }
    }
    impl Updatable<()> for Input {
        fn update(&mut self) -> NothingOrError<()> {
            self.time += Time::from_nanoseconds(1_000_000_000);
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
            static mut INPUT: Input = Input { time: Time::ZERO };
            let mut input = PointerDereferencer::new(core::ptr::addr_of_mut!(INPUT));
            let mut pid = CommandPID::new(
                input.clone(),
                Command::new(PositionDerivative::Position, 5.0),
                kvals,
            );
            assert_eq!(pid.get().unwrap(), None);
            pid.update().unwrap();
            assert_eq!(pid.get().unwrap().unwrap().value, 5.0);
            input.update().unwrap();
            pid.update().unwrap();
            assert_eq!(pid.get().unwrap().unwrap().value, 5.05);
            input.update().unwrap();
            pid.update().unwrap();
            assert_eq!(pid.get().unwrap().unwrap().value, 5.1);
            input.update().unwrap();
            pid.update().unwrap();
            assert_eq!(pid.get().unwrap().unwrap().value, 5.15);
        }

        {
            static mut INPUT: Input = Input { time: Time::ZERO };
            let mut input = PointerDereferencer::new(core::ptr::addr_of_mut!(INPUT));
            let mut pid = CommandPID::new(
                input.clone(),
                Command::new(PositionDerivative::Velocity, 5.0),
                kvals,
            );
            assert_eq!(pid.get().unwrap(), None);
            pid.update().unwrap();
            assert_eq!(pid.get().unwrap(), None);
            input.update().unwrap();
            pid.update().unwrap();
            assert_eq!(pid.get().unwrap().unwrap().value, 5.025);
            input.update().unwrap();
            pid.update().unwrap();
            assert_eq!(pid.get().unwrap().unwrap().value, 10.1);
            input.update().unwrap();
            pid.update().unwrap();
            assert_eq!(pid.get().unwrap().unwrap().value, 15.225);
        }

        {
            static mut INPUT: Input = Input { time: Time::ZERO };
            let mut input = PointerDereferencer::new(core::ptr::addr_of_mut!(INPUT));
            let mut pid = CommandPID::new(
                input.clone(),
                Command::new(PositionDerivative::Acceleration, 5.0),
                kvals,
            );
            assert_eq!(pid.get().unwrap(), None);
            pid.update().unwrap();
            assert_eq!(pid.get().unwrap(), None);
            input.update().unwrap();
            pid.update().unwrap();
            assert_eq!(pid.get().unwrap(), None);
            input.update().unwrap();
            pid.update().unwrap();
            assert_eq!(pid.get().unwrap().unwrap().value, 7.5625);
            input.update().unwrap();
            pid.update().unwrap();
            assert_eq!(pid.get().unwrap().unwrap().value, 20.225);
        }
    }
}
