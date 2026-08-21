// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2024-2026 UxuginPython
use core::fmt::Debug;
use rrtk::streams::control::*;
use rrtk::streams::converters::*;
use rrtk::streams::flow::*;
use rrtk::streams::logic::*;
use rrtk::streams::math::*;
use rrtk::streams::*;
use rrtk::*;
//TODO: Some of these PointerDereferencers probably aren't needed, nearly all unsafe blocks can be
//be shrunk, and some clone calls are unnecessary for Copy types.
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
        let expirer = Expirer::new(stream, time_getter, Time::from_nanoseconds(10));
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
            Ok(Some(Datum::new(Time::ZERO, 0.0)))
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
        let stream = NoneToError::new(input, Error::FromNone);
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
            Ok(Some(Datum::new(Time::ZERO, 1.0)))
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
        let stream = NoneToValue::new(input, time_getter, 2.0);
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
    impl Getter<MillimeterPerSecondSquared<f32>, ()> for AccGetter {
        fn get(&self) -> Output<MillimeterPerSecondSquared<f32>, ()> {
            Ok(Some(Datum::new(
                self.time,
                MillimeterPerSecondSquared::new(1.0),
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
        let acc_getter = PointerDereferencer::new(core::ptr::addr_of_mut!(ACC_GETTER));
        let mut state_getter = AccelerationToState::new(acc_getter);
        let output = state_getter.get();
        assert!(output.unwrap().is_none());
        state_getter.update().unwrap();
        let output = state_getter.get();
        assert!(output.unwrap().is_none());
        state_getter.update().unwrap();
        let output = state_getter.get();
        assert!(output.unwrap().is_none());
        state_getter.update().unwrap();
        let output = state_getter.get();
        assert_eq!(
            output.unwrap().unwrap(),
            Datum::new(
                Time::from_nanoseconds(3_000_000_000),
                LinearState::new(
                    Millimeter::new(1.5),
                    MillimeterPerSecond::new(2.0),
                    MillimeterPerSecondSquared::new(1.0)
                )
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
    impl Getter<MillimeterPerSecond<f32>, ()> for VelGetter {
        fn get(&self) -> Output<MillimeterPerSecond<f32>, ()> {
            Ok(Some(Datum::new(
                self.time,
                MillimeterPerSecond::new(self.time.as_seconds_f32()),
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
        let vel_getter = PointerDereferencer::new(core::ptr::addr_of_mut!(VEL_GETTER));
        let mut state_getter = VelocityToState::new(vel_getter);
        let output = state_getter.get();
        assert!(output.unwrap().is_none());
        state_getter.update().unwrap();
        let output = state_getter.get();
        assert!(output.unwrap().is_none());
        state_getter.update().unwrap();
        let output = state_getter.get();
        assert_eq!(
            output.unwrap().unwrap(),
            Datum::new(
                Time::from_nanoseconds(2_000_000_000),
                LinearState::new(
                    Millimeter::new(1.5),
                    MillimeterPerSecond::new(2.0),
                    MillimeterPerSecondSquared::new(1.0)
                )
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
    impl Getter<Millimeter<f32>, ()> for PosGetter {
        fn get(&self) -> Output<Millimeter<f32>, ()> {
            Ok(Some(Datum::new(
                self.time,
                Millimeter::new(self.time.as_seconds_f32()),
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
        let pos_getter = PointerDereferencer::new(core::ptr::addr_of_mut!(POS_GETTER));
        let mut state_getter = PositionToState::new(pos_getter);
        let output = state_getter.get();
        assert!(output.unwrap().is_none());
        state_getter.update().unwrap();
        let output = state_getter.get();
        assert!(output.unwrap().is_none());
        state_getter.update().unwrap();
        let output = state_getter.get();
        assert!(output.unwrap().is_none());
        state_getter.update().unwrap();
        let output = state_getter.get();
        assert_eq!(
            output.unwrap().unwrap(),
            Datum::new(
                Time::from_nanoseconds(3_000_000_000),
                LinearState::new(
                    Millimeter::new(3.0),
                    MillimeterPerSecond::new(1.0),
                    MillimeterPerSecondSquared::new(0.0)
                )
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
            match self.index {
                0 => Err(Error),
                1 => Ok(None),
                _ => Ok(Some(Datum::new(Time::from_nanoseconds(2), 1.0))),
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
        let erroring_binding = erroring;
        let normal_binding = normal;
        let stream = SumStream::new([
            erroring_binding.as_dyn_getter(),
            normal_binding.as_dyn_getter(),
        ]);
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
    let sum_stream = SumStream::new([Input]);
    assert_eq!(sum_stream.get(), Ok(None));
}
#[test]
#[should_panic]
fn empty_sum_stream() {
    let _: SumStream<0, NoneGetter> = SumStream::new([]);
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
                Err(Error)
            } else if self.index == 1 {
                Ok(None)
            } else {
                Ok(Some(Datum::new(Time::from_nanoseconds(2), 1.0)))
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
        let stream = Sum2::new(erroring, normal);
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
            Ok(Some(Datum::new(Time::from_nanoseconds(1), 10.0)))
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
            Ok(Some(Datum::new(Time::from_nanoseconds(2), 3.0)))
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
        let stream = DifferenceStream::new(stream1, stream2);
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
            match self.index {
                0 => Err(Error),
                1 => Ok(None),
                _ => Ok(Some(Datum::new(Time::from_nanoseconds(2), 3.0))),
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
        let erroring_binding = erroring;
        let normal_binding = normal;
        let stream = ProductStream::new([
            erroring_binding.as_dyn_getter(),
            normal_binding.as_dyn_getter(),
        ]);
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
    let product_stream = ProductStream::new([Input]);
    assert_eq!(product_stream.get(), Ok(None));
}
#[test]
#[should_panic]
fn empty_product_stream() {
    let _: ProductStream<0, NoneGetter> = ProductStream::new([]);
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
                Err(Error)
            } else if self.index == 1 {
                Ok(None)
            } else {
                Ok(Some(Datum::new(Time::from_nanoseconds(2), 3.0)))
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
        let stream = Product2::new(erroring, normal);
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
            Ok(Some(Datum::new(Time::from_nanoseconds(1), 12.0)))
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
            Ok(Some(Datum::new(Time::from_nanoseconds(2), 3.0)))
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
        let stream = QuotientStream::new(stream1, stream2);
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
//to fail even if the calculation is correct. Testing the accuracy of the other two is considered
//sufficient. The same applies to the ewma_stream and ewma_stream_quantity tests.
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
            match self.index {
                0..=2 => Err(DummyError),
                3..=5 => Ok(None),
                _ => Ok(Some(Datum::new(Time::from_nanoseconds(1), 5.0))),
            }
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
            match self.index {
                0 | 3 | 6 => Err(DummyError),
                1 | 4 | 7 => Ok(None),
                _ => Ok(Some(Datum::new(Time::from_nanoseconds(2), 3.0))),
            }
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
        let stream = ExponentStream::new(stream1, stream2);
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
            //This appears to be an instance of https://github.com/rust-lang/miri/issues/4208
            #[cfg(not(miri))]
            assert_eq!(x.value, 125.0);
            #[cfg(miri)]
            assert!(124.99995 < x.value && x.value < 125.00005);
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
    impl Getter<Second<f32>, DummyError> for DummyStream {
        fn get(&self) -> Output<Second<f32>, DummyError> {
            Ok(Some(Datum::new(
                self.time * DimensionlessInteger(2),
                (self.time * DimensionlessInteger(3)).as_seconds(),
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
        let input = PointerDereferencer::new(core::ptr::addr_of_mut!(INPUT));
        let mut stream = DerivativeStream::new(input);
        stream.update().unwrap();
        stream.update().unwrap();
        assert_eq!(
            stream.get().unwrap().unwrap().time,
            Time::from_nanoseconds(8_000_000_000)
        );
        assert_eq!(
            stream.get().unwrap().unwrap().value,
            Dimensionless::new(1.5) //Derivating time d time returns a dimensionless quantity.
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
    impl Getter<MillimeterPerSecond<f32>, DummyError> for DummyStream {
        fn get(&self) -> Output<MillimeterPerSecond<f32>, DummyError> {
            Ok(Some(Datum::new(self.time, MillimeterPerSecond::new(1.0))))
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
        let input = PointerDereferencer::new(core::ptr::addr_of_mut!(INPUT));
        let mut stream = IntegralStream::new(input);
        stream.update().unwrap();
        stream.update().unwrap();
        assert_eq!(
            stream.get().unwrap().unwrap().time,
            Time::from_nanoseconds(2_000_000_000)
        );
        assert_eq!(stream.get().unwrap().unwrap().value, Millimeter::new(1.0));
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
                (self.time / DimensionlessInteger(2)).as_seconds_f32(),
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
        let input = PointerDereferencer::new(core::ptr::addr_of_mut!(INPUT));
        let mut stream = PIDControllerStream::new(input, 5.0, PIDKValues::new(1.0, 0.01, 0.1));
        stream.update().unwrap();
        assert_eq!(
            stream.get().unwrap().unwrap().time,
            Time::from_nanoseconds(2_000_000_000)
        );
        assert_eq!(stream.get().unwrap().unwrap().value, 4.0);
        stream.update().unwrap();
        assert_eq!(
            stream.get().unwrap().unwrap().time,
            Time::from_nanoseconds(4_000_000_000)
        );
        assert_eq!(
            stream.get().unwrap().unwrap().value,
            3.0 + 7.0 * 0.01 - 0.5 * 0.1
        );
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
        let input = PointerDereferencer::new(core::ptr::addr_of_mut!(INPUT));
        let mut stream = EWMAStream::new(input, 0.25);
        stream.update().unwrap();
        assert_eq!(stream.get().unwrap().unwrap().value, 110.0);
        stream.update().unwrap();
        assert_eq!(stream.get().unwrap().unwrap().value, 110.4375);
        stream.update().unwrap();
        //Floating-point stuff gets a bit weird because of rounding, but it still appears to work
        //correctly.
        assert_eq!(stream.get().unwrap().unwrap().value, 112.87109375);
        stream.update().unwrap();
        assert_eq!(stream.get().unwrap().unwrap().value, 105.927490234375);
        stream.update().unwrap();
        assert_eq!(stream.get().unwrap().unwrap().value, 104.20921325683594);
        stream.update().unwrap();
        assert_eq!(stream.get().unwrap().unwrap().value, 107.18018245697021);
        stream.update().unwrap();
        assert_eq!(stream.get().unwrap().unwrap().value, 108.85135263204575);
        stream.update().unwrap();
        //Despite every other assert_eq! here working, this one does not because the way f32 works
        //means that it thinks it's off by 0.00001.
        //assert_eq!(stream.get().unwrap().unwrap().value, 104.97888585552573);
    }
}
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
    impl Getter<Millimeter<f32>, DummyError> for DummyStream {
        fn get(&self) -> Output<Millimeter<f32>, DummyError> {
            let value = Millimeter::new(match self.time.as_nanoseconds() {
                2_000_000_000 => 110.0,
                4_000_000_000 => 111.0,
                6_000_000_000 => 116.0,
                8_000_000_000 => 97.0,
                10_000_000_000 => 102.0,
                12_000_000_000 => 111.0,
                14_000_000_000 => 111.0,
                16_000_000_000 => 100.0,
                _ => 0.0,
            });
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
        let input = PointerDereferencer::new(core::ptr::addr_of_mut!(INPUT));
        let mut stream = EWMAStream::<Millimeter<f32>, _, _>::new(input, 0.25);
        stream.update().unwrap();
        assert_eq!(stream.get().unwrap().unwrap().value.into_inner(), 110.0);
        stream.update().unwrap();
        assert_eq!(stream.get().unwrap().unwrap().value.into_inner(), 110.4375);
        stream.update().unwrap();
        //Floating-point stuff gets a bit weird because of rounding, but it still appears to work
        //correctly.
        assert_eq!(
            stream.get().unwrap().unwrap().value.into_inner(),
            112.87109375
        );
        stream.update().unwrap();
        assert_eq!(
            stream.get().unwrap().unwrap().value.into_inner(),
            105.927490234375
        );
        stream.update().unwrap();
        assert_eq!(
            stream.get().unwrap().unwrap().value.into_inner(),
            104.20921325683594
        );
        stream.update().unwrap();
        assert_eq!(
            stream.get().unwrap().unwrap().value.into_inner(),
            107.18018245697021
        );
        stream.update().unwrap();
        assert_eq!(
            stream.get().unwrap().unwrap().value.into_inner(),
            108.85135263204575
        );
        stream.update().unwrap();
        //Despite every other assert_eq! here working, this one does not because the way f32 works
        //means that it thinks it's off by 0.00001.
        //assert_eq!(stream.get().unwrap().unwrap().value.into_inner(), 104.97888585552573);
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
        let input = PointerDereferencer::new(core::ptr::addr_of_mut!(INPUT));
        let mut stream = MovingAverageStream::new(input, Time::from_nanoseconds(5));
        stream.update().unwrap();
        assert_eq!(stream.get().unwrap().unwrap().value, 110.0);
        stream.update().unwrap();
        assert_eq!(stream.get().unwrap().unwrap().value, 110.4);
        stream.update().unwrap();
        //assert_eq!(stream.get().unwrap().unwrap().value, 112.8);
        stream.update().unwrap();
        assert_eq!(stream.get().unwrap().unwrap().value, 107.4);
        stream.update().unwrap();
        //assert_eq!(stream.get().unwrap().unwrap().value, 102.8);
        stream.update().unwrap();
        assert_eq!(stream.get().unwrap().unwrap().value, 104.6);
        stream.update().unwrap();
        assert_eq!(stream.get().unwrap().unwrap().value, 109.2);
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
    impl Getter<Millimeter<f32>, DummyError> for DummyStream {
        fn get(&self) -> Output<Millimeter<f32>, DummyError> {
            let value = Millimeter::new(match self.time.as_nanoseconds() {
                2 => 110.0,
                4 => 111.0,
                6 => 116.0,
                8 => 97.0,
                10 => 102.0,
                12 => 111.0,
                14 => 111.0,
                16 => 100.0,
                _ => 0.0,
            });
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
        let input = PointerDereferencer::new(core::ptr::addr_of_mut!(INPUT));
        let mut stream = MovingAverageStream::new(input, Time::from_nanoseconds(5));
        stream.update().unwrap();
        assert_eq!(stream.get().unwrap().unwrap().value.into_inner(), 110.0);
        stream.update().unwrap();
        assert_eq!(stream.get().unwrap().unwrap().value.into_inner(), 110.4);
        stream.update().unwrap();
        //assert_eq!(stream.get().unwrap().unwrap().value.into_inner(), 112.8);
        stream.update().unwrap();
        assert_eq!(stream.get().unwrap().unwrap().value.into_inner(), 107.4);
        stream.update().unwrap();
        //assert_eq!(stream.get().unwrap().unwrap().value.into_inner(), 102.8);
        stream.update().unwrap();
        assert_eq!(stream.get().unwrap().unwrap().value.into_inner(), 104.6);
        stream.update().unwrap();
        assert_eq!(stream.get().unwrap().unwrap().value.into_inner(), 109.2);
        stream.update().unwrap();
        assert_eq!(stream.get().unwrap().unwrap().value.into_inner(), 106.6);
    }
}
#[test]
fn latest() {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    struct Error(u8);
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
                6 => Err(Error(1)),                                      //Err,  Err
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
                3 => Err(Error(2)),                       //Some, Err
                4 => Ok(None),                            //None, None
                5 => Err(Error(3)),                       //None, Err
                6 => Err(Error(4)),                       //Err,  Err
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
        let stream1_binding = stream1;
        let stream2_binding = stream2;
        let mut latest = Latest::new([
            stream1_binding.as_dyn_getter(),
            stream2_binding.as_dyn_getter(),
        ]);
        assert_eq!(
            latest.get(),
            Ok(Some(Datum::new(Time::from_nanoseconds(1), 1)))
        );
        latest.update().unwrap();
        assert_eq!(
            latest.get(),
            Ok(Some(Datum::new(Time::from_nanoseconds(1), 2)))
        );
        latest.update().unwrap();
        assert_eq!(latest.get(), Ok(Some(Datum::new(Time::ZERO, 1))));
        latest.update().unwrap();
        assert_eq!(latest.get(), Err(Error(2)));
        latest.update().unwrap();
        assert_eq!(latest.get(), Ok(None));
        latest.update().unwrap();
        assert_eq!(latest.get(), Err(Error(3)));
        latest.update().unwrap();
        assert_eq!(latest.get(), Err(Error(1)));
    }
}
//This test has been slightly modified from the latest() test to test Latest2 instead.
#[test]
fn latest2() {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    struct Error(u8);
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
                6 => Err(Error(1)),                                      //Err,  Err
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
                3 => Err(Error(2)),                       //Some, Err
                4 => Ok(None),                            //None, None
                5 => Err(Error(3)),                       //None, Err
                6 => Err(Error(4)),                       //Err,  Err
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
    let mut latest = Latest2::new(Stream1::new(), Stream2::new());
    assert_eq!(
        latest.get(),
        Ok(Some(Datum::new(Time::from_nanoseconds(1), 1)))
    );
    latest.update().unwrap();
    assert_eq!(
        latest.get(),
        Ok(Some(Datum::new(Time::from_nanoseconds(1), 2)))
    );
    latest.update().unwrap();
    assert_eq!(latest.get(), Ok(Some(Datum::new(Time::ZERO, 1))));
    latest.update().unwrap();
    assert_eq!(latest.get(), Err(Error(2)));
    latest.update().unwrap();
    assert_eq!(latest.get(), Ok(None));
    latest.update().unwrap();
    assert_eq!(latest.get(), Err(Error(3)));
    latest.update().unwrap();
    assert_eq!(latest.get(), Err(Error(1)));
}
#[test]
fn inputless_gates() {
    struct GetBool;
    impl Updatable<()> for GetBool {
        fn update(&mut self) -> NothingOrError<()> {
            panic!("There should be no instances of GetBool to update.");
        }
    }
    impl Getter<bool, ()> for GetBool {
        fn get(&self) -> Output<bool, ()> {
            panic!("There should be no instances of GetBool to get from.");
        }
    }
    const INPUTS: [GetBool; 0] = [];
    let and = AndStream::new(INPUTS);
    assert_eq!(and.get(), Ok(None));
    let or = OrStream::new(INPUTS);
    assert_eq!(or.get(), Ok(None));
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
        let in1 = PointerDereferencer::new(core::ptr::addr_of_mut!(IN_1));
        static mut IN_2: In2 = In2::new();
        let in2 = PointerDereferencer::new(core::ptr::addr_of_mut!(IN_2));
        //TODO: Maybe revise this test to better suit the new AndStream. This is a pretty patchy
        //fix.
        let in1_binding = in1;
        let in2_binding = in2;
        let mut and = AndStream::new([in1_binding.as_dyn_getter(), in2_binding.as_dyn_getter()]);
        assert!(!and.get().unwrap().unwrap().value);
        and.update().unwrap();
        assert!(!and.get().unwrap().unwrap().value);
        and.update().unwrap();
        assert!(!and.get().unwrap().unwrap().value);
        and.update().unwrap();
        assert!(!and.get().unwrap().unwrap().value);
        and.update().unwrap();
        assert_eq!(and.get().unwrap(), None);
        and.update().unwrap();
        assert_eq!(and.get().unwrap(), None);
        and.update().unwrap();
        assert!(!and.get().unwrap().unwrap().value);
        and.update().unwrap();
        assert_eq!(and.get().unwrap(), None);
        and.update().unwrap();
        assert!(and.get().unwrap().unwrap().value);
        and.update().unwrap();
    }
}
#[test]
fn and2() {
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
        let in1 = PointerDereferencer::new(core::ptr::addr_of_mut!(IN_1));
        static mut IN_2: In2 = In2::new();
        let in2 = PointerDereferencer::new(core::ptr::addr_of_mut!(IN_2));
        let mut and = And2::new(in1, in2);
        assert!(!and.get().unwrap().unwrap().value);
        and.update().unwrap();
        assert!(!and.get().unwrap().unwrap().value);
        and.update().unwrap();
        assert!(!and.get().unwrap().unwrap().value);
        and.update().unwrap();
        assert!(!and.get().unwrap().unwrap().value);
        and.update().unwrap();
        assert_eq!(and.get().unwrap(), None);
        and.update().unwrap();
        assert_eq!(and.get().unwrap(), None);
        and.update().unwrap();
        assert!(!and.get().unwrap().unwrap().value);
        and.update().unwrap();
        assert_eq!(and.get().unwrap(), None);
        and.update().unwrap();
        assert!(and.get().unwrap().unwrap().value);
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
        let in1 = PointerDereferencer::new(core::ptr::addr_of_mut!(IN_1));
        static mut IN_2: In2 = In2::new();
        let in2 = PointerDereferencer::new(core::ptr::addr_of_mut!(IN_2));
        //TODO: Maybe revise this test to better suit the new OrStream. This is a pretty patchy
        //fix.
        let in1_binding = in1;
        let in2_binding = in2;
        let mut or = OrStream::new([in1_binding.as_dyn_getter(), in2_binding.as_dyn_getter()]);
        assert!(!or.get().unwrap().unwrap().value);
        or.update().unwrap();
        assert_eq!(or.get().unwrap(), None);
        or.update().unwrap();
        assert!(or.get().unwrap().unwrap().value);
        or.update().unwrap();
        assert_eq!(or.get().unwrap(), None);
        or.update().unwrap();
        assert_eq!(or.get().unwrap(), None);
        or.update().unwrap();
        assert!(or.get().unwrap().unwrap().value);
        or.update().unwrap();
        assert!(or.get().unwrap().unwrap().value);
        or.update().unwrap();
        assert!(or.get().unwrap().unwrap().value);
        or.update().unwrap();
        assert!(or.get().unwrap().unwrap().value);
        or.update().unwrap();
    }
}
#[test]
fn or2() {
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
        let in1 = PointerDereferencer::new(core::ptr::addr_of_mut!(IN_1));
        static mut IN_2: In2 = In2::new();
        let in2 = PointerDereferencer::new(core::ptr::addr_of_mut!(IN_2));
        let mut or = Or2::new(in1, in2);
        assert!(!or.get().unwrap().unwrap().value);
        or.update().unwrap();
        assert_eq!(or.get().unwrap(), None);
        or.update().unwrap();
        assert!(or.get().unwrap().unwrap().value);
        or.update().unwrap();
        assert_eq!(or.get().unwrap(), None);
        or.update().unwrap();
        assert_eq!(or.get().unwrap(), None);
        or.update().unwrap();
        assert!(or.get().unwrap().unwrap().value);
        or.update().unwrap();
        assert!(or.get().unwrap().unwrap().value);
        or.update().unwrap();
        assert!(or.get().unwrap().unwrap().value);
        or.update().unwrap();
        assert!(or.get().unwrap().unwrap().value);
        or.update().unwrap();
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
        let input = PointerDereferencer::new(core::ptr::addr_of_mut!(INPUT));
        let mut not = NotStream::new(input);
        assert!(not.get().unwrap().unwrap().value);
        not.update().unwrap();
        assert_eq!(not.get().unwrap(), None);
        not.update().unwrap();
        assert!(!not.get().unwrap().unwrap().value);
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
        let condition = PointerDereferencer::new(core::ptr::addr_of_mut!(CONDITION));
        static mut INPUT: Input = Input;
        let input = PointerDereferencer::new(core::ptr::addr_of_mut!(INPUT));
        let mut if_stream = IfStream::new(condition, input);
        assert_eq!(if_stream.get().unwrap(), None);
        if_stream.update().unwrap();
        assert_eq!(if_stream.get().unwrap(), None);
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
        let condition = PointerDereferencer::new(core::ptr::addr_of_mut!(CONDITION));
        static mut TRUE_INPUT: True = True;
        let true_input = PointerDereferencer::new(core::ptr::addr_of_mut!(TRUE_INPUT));
        static mut FALSE_INPUT: False = False;
        let false_input = PointerDereferencer::new(core::ptr::addr_of_mut!(FALSE_INPUT));
        let mut if_else_stream = IfElseStream::new(condition, true_input, false_input);
        assert_eq!(if_else_stream.get().unwrap().unwrap().value, 2);
        if_else_stream.update().unwrap();
        assert_eq!(if_else_stream.get().unwrap(), None);
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
        let condition = PointerDereferencer::new(core::ptr::addr_of_mut!(CONDITION));
        static mut INPUT: Input = Input { time: Time::ZERO };
        let input = PointerDereferencer::new(core::ptr::addr_of_mut!(INPUT));
        let mut freeze = FreezeStream::new(condition, input);
        freeze.update().unwrap();
        assert_eq!(freeze.get().unwrap().unwrap().value, 1);
        freeze.update().unwrap();
        assert_eq!(freeze.get().unwrap().unwrap().value, 1);
        freeze.update().unwrap();
        assert_eq!(freeze.get().unwrap().unwrap().value, 1);
        freeze.update().unwrap();
        assert_eq!(freeze.get().unwrap().unwrap().value, 4);
        freeze.update().unwrap();
        assert_eq!(freeze.get().unwrap().unwrap().value, 5);
        freeze.update().unwrap();
        assert_eq!(freeze.get().unwrap(), None);
        freeze.update().unwrap();
        assert_eq!(freeze.get().unwrap(), None);
        freeze.update().unwrap();
        assert_eq!(freeze.get().unwrap().unwrap().value, 8);
        freeze.update().unwrap();
        assert_eq!(freeze.get().unwrap().unwrap().value, 9);
    }
}
#[test]
fn command_pid() {
    struct Input {
        time: Time,
    }
    impl Getter<LinearState, ()> for Input {
        fn get(&self) -> Output<LinearState, ()> {
            Ok(Some(Datum::new(self.time, LinearState::default())))
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
            let input = PointerDereferencer::new(core::ptr::addr_of_mut!(INPUT));
            let mut pid = CommandPID::new(
                input,
                LinearCommand::new(PositionDerivative::Position, 5.0),
                kvals,
            );
            assert_eq!(pid.get().unwrap(), None);
            pid.update().unwrap();
            assert_eq!(pid.get().unwrap().unwrap().value, 5.0);
            pid.update().unwrap();
            assert_eq!(pid.get().unwrap().unwrap().value, 5.05);
            pid.update().unwrap();
            assert_eq!(pid.get().unwrap().unwrap().value, 5.1);
            pid.update().unwrap();
            assert_eq!(pid.get().unwrap().unwrap().value, 5.15);
        }

        {
            static mut INPUT: Input = Input { time: Time::ZERO };
            let input = PointerDereferencer::new(core::ptr::addr_of_mut!(INPUT));
            let mut pid = CommandPID::new(
                input,
                LinearCommand::new(PositionDerivative::Velocity, 5.0),
                kvals,
            );
            assert_eq!(pid.get().unwrap(), None);
            pid.update().unwrap();
            assert_eq!(pid.get().unwrap(), None);
            pid.update().unwrap();
            assert_eq!(pid.get().unwrap().unwrap().value, 5.025);
            pid.update().unwrap();
            assert_eq!(pid.get().unwrap().unwrap().value, 10.1);
            pid.update().unwrap();
            assert_eq!(pid.get().unwrap().unwrap().value, 15.225);
        }

        {
            static mut INPUT: Input = Input { time: Time::ZERO };
            let input = PointerDereferencer::new(core::ptr::addr_of_mut!(INPUT));
            let mut pid = CommandPID::new(
                input,
                LinearCommand::new(PositionDerivative::Acceleration, 5.0),
                kvals,
            );
            assert_eq!(pid.get().unwrap(), None);
            pid.update().unwrap();
            assert_eq!(pid.get().unwrap(), None);
            pid.update().unwrap();
            assert_eq!(pid.get().unwrap(), None);
            pid.update().unwrap();
            assert_eq!(pid.get().unwrap().unwrap().value, 7.5625);
            pid.update().unwrap();
            assert_eq!(pid.get().unwrap().unwrap().value, 20.225);
        }
    }
}
//XXX: This does not currently test the error handling behavior of NoneToDefault::update().
#[test]
fn none_to_default() {
    #[derive(Clone, Debug, PartialEq, Eq)]
    struct MyError(u8);
    struct MyGetter;
    static mut GETTER_UPDATE_CALLS: u8 = 0;
    impl Updatable<MyError> for MyGetter {
        fn update(&mut self) -> NothingOrError<MyError> {
            unsafe {
                GETTER_UPDATE_CALLS += 1;
            }
            Ok(())
        }
    }
    static mut GETTER_GET_CALLS: u8 = 0;
    impl Getter<u8, MyError> for MyGetter {
        fn get(&self) -> Output<u8, MyError> {
            unsafe {
                GETTER_GET_CALLS += 1;
            }
            match unsafe { GETTER_UPDATE_CALLS } {
                0 => panic!("missed update call"),
                1 => Ok(Some(Datum::new(Time::from_seconds_f32(2.0), 1))),
                2 => Ok(Some(Datum::new(Time::from_seconds_f32(2.1), 2))),
                3 => Ok(None),
                4 => Ok(None),
                5 => Ok(None),
                6 => Err(MyError(1)),
                _ => panic!("update called too many times"),
            }
        }
    }
    struct MyTimeGetter;
    static mut TIME_GETTER_UPDATE_CALLS: u8 = 0;
    impl Updatable<MyError> for MyTimeGetter {
        fn update(&mut self) -> NothingOrError<MyError> {
            unsafe {
                TIME_GETTER_UPDATE_CALLS += 1;
            }
            Ok(())
        }
    }
    static mut TIME_GETTER_GET_CALLS: u8 = 0;
    impl TimeGetter<MyError> for MyTimeGetter {
        fn get(&self) -> TimeOutput<MyError> {
            unsafe {
                TIME_GETTER_GET_CALLS += 1;
            }
            match unsafe { GETTER_UPDATE_CALLS } {
                0 => panic!("missed update call"),
                1 => panic!("TimeGetter is not needed here"),
                2 => panic!("TimeGetter is not needed here"),
                3 => Ok(Time::from_seconds_f32(2.2)),
                4 => Ok(Time::from_seconds_f32(2.3)),
                5 => Err(MyError(2)),
                6 => panic!("TimeGetter is not needed here"),
                _ => panic!("update called too many times"),
            }
        }
    }
    let mut test = NoneToDefault::new(MyGetter, MyTimeGetter);
    macro_rules! test_index {
        ($g_u_1: literal, $tg_u_1: literal, $g_g_1: literal, $tg_g_1: literal, $gotten_value: expr, $g_u_2: literal, $tg_u_2: literal, $g_g_2: literal, $tg_g_2: literal) => {
            test.update().unwrap();
            //This is not RRTK's fault: https://github.com/rust-lang/rust/issues/131443
            #[allow(static_mut_refs)]
            unsafe {
                assert_eq!(GETTER_UPDATE_CALLS, $g_u_1);
                assert_eq!(TIME_GETTER_UPDATE_CALLS, $tg_u_1);
                assert_eq!(GETTER_GET_CALLS, $g_g_1);
                assert_eq!(TIME_GETTER_GET_CALLS, $tg_g_1);
            }
            assert_eq!(test.get(), $gotten_value);
            #[allow(static_mut_refs)]
            unsafe {
                assert_eq!(GETTER_UPDATE_CALLS, $g_u_2);
                assert_eq!(TIME_GETTER_UPDATE_CALLS, $tg_u_2);
                assert_eq!(GETTER_GET_CALLS, $g_g_2);
                assert_eq!(TIME_GETTER_GET_CALLS, $tg_g_2);
            }
        };
    }

    #[rustfmt::skip]
    test_index!(1, 1, 0, 0, Ok(Some(Datum::new(Time::from_seconds_f32(2.0), 1))), 1, 1, 1, 0);
    #[rustfmt::skip]
    test_index!(2, 2, 1, 0, Ok(Some(Datum::new(Time::from_seconds_f32(2.1), 2))), 2, 2, 2, 0);
    #[rustfmt::skip]
    test_index!(3, 3, 2, 0, Ok(Some(Datum::new(Time::from_seconds_f32(2.2), 0))), 3, 3, 3, 1);
    #[rustfmt::skip]
    test_index!(4, 4, 3, 1, Ok(Some(Datum::new(Time::from_seconds_f32(2.3), 0))), 4, 4, 4, 2);
    #[rustfmt::skip]
    test_index!(5, 5, 4, 2, Err(error::PossibleDoubleError::B(MyError(2))), 5, 5, 5, 3);
    #[rustfmt::skip]
    test_index!(6, 6, 5, 3, Err(error::PossibleDoubleError::A(MyError(1))), 6, 6, 6, 3);
}
#[test]
fn into_converter() {
    #[derive(Clone, Debug, PartialEq, Eq)]
    struct MyError(u8);
    struct MyGetter;
    static mut GETTER_UPDATE_CALLS: u8 = 0;
    impl Updatable<MyError> for MyGetter {
        fn update(&mut self) -> NothingOrError<MyError> {
            unsafe {
                GETTER_UPDATE_CALLS += 1;
            }
            Ok(())
        }
    }
    static mut GETTER_GET_CALLS: u8 = 0;
    impl Getter<u8, MyError> for MyGetter {
        fn get(&self) -> Output<u8, MyError> {
            unsafe {
                GETTER_GET_CALLS += 1;
            }
            match unsafe { GETTER_UPDATE_CALLS } {
                0 => panic!("missed update call"),
                1 => Ok(Some(Datum::new(Time::from_seconds_f32(2.0), 1))),
                2 => Ok(None),
                3 => Err(MyError(1)),
                _ => panic!("update called too many times"),
            }
        }
    }
    let mut test = IntoConverter::new(MyGetter);
    macro_rules! test_index {
        ($g_u_1: literal, $g_g_1: literal, $gotten_value: expr, $g_u_2: literal, $g_g_2: literal) => {
            test.update().unwrap();
            //This is not RRTK's fault: https://github.com/rust-lang/rust/issues/131443
            #[allow(static_mut_refs)]
            unsafe {
                assert_eq!(GETTER_UPDATE_CALLS, $g_u_1);
                assert_eq!(GETTER_GET_CALLS, $g_g_1);
            }
            assert_eq!(test.get(), $gotten_value);
            #[allow(static_mut_refs)]
            unsafe {
                assert_eq!(GETTER_UPDATE_CALLS, $g_u_2);
                assert_eq!(GETTER_GET_CALLS, $g_g_2);
            }
        };
    }
    #[rustfmt::skip]
    test_index!(1, 0, Ok(Some(Datum::new(Time::from_seconds_f32(2.0), 1i16))), 1, 1);
    #[rustfmt::skip]
    test_index!(2, 1, Ok(None::<Datum<i16>>), 2, 2);
    #[rustfmt::skip]
    test_index!(3, 2, Err::<Option<Datum<i16>>, _>(MyError(1)), 3, 3);
}
#[test]
fn error_into_converter() {
    struct MyGetter;
    static mut GETTER_UPDATE_CALLS: u8 = 0;
    impl Updatable<i16> for MyGetter {
        fn update(&mut self) -> NothingOrError<i16> {
            unsafe {
                GETTER_UPDATE_CALLS += 1;
            }
            Ok(())
        }
    }
    static mut GETTER_GET_CALLS: u8 = 0;
    impl Getter<u8, i16> for MyGetter {
        fn get(&self) -> Output<u8, i16> {
            unsafe {
                GETTER_GET_CALLS += 1;
            }
            match unsafe { GETTER_UPDATE_CALLS } {
                0 => panic!("missed update call"),
                1 => Ok(Some(Datum::new(Time::from_seconds_f32(2.0), 1))),
                2 => Ok(None),
                3 => Err(100),
                _ => panic!("update called too many times"),
            }
        }
    }
    let mut test = ErrorIntoConverter::new(MyGetter);
    macro_rules! test_index {
        ($g_u_1: literal, $g_g_1: literal, $gotten_value: expr, $g_u_2: literal, $g_g_2: literal) => {
            <ErrorIntoConverter<_, _> as Updatable<i64>>::update(&mut test).unwrap();
            //This is not RRTK's fault: https://github.com/rust-lang/rust/issues/131443
            #[allow(static_mut_refs)]
            unsafe {
                assert_eq!(GETTER_UPDATE_CALLS, $g_u_1);
                assert_eq!(GETTER_GET_CALLS, $g_g_1);
            }
            assert_eq!(test.get(), $gotten_value);
            #[allow(static_mut_refs)]
            unsafe {
                assert_eq!(GETTER_UPDATE_CALLS, $g_u_2);
                assert_eq!(GETTER_GET_CALLS, $g_g_2);
            }
        };
    }
    #[rustfmt::skip]
    test_index!(1, 0, Ok::<_, i64>(Some(Datum::new(Time::from_seconds_f32(2.0), 1u8))), 1, 1);
    #[rustfmt::skip]
    test_index!(2, 1, Ok::<_, i64>(None), 2, 2);
    #[rustfmt::skip]
    test_index!(3, 2, Err(100i64), 3, 3);
}
#[test]
fn dimension_adder() {
    #[derive(Clone, Debug, PartialEq, Eq)]
    struct MyError(u8);
    struct MyGetter;
    static mut GETTER_UPDATE_CALLS: u8 = 0;
    impl Updatable<MyError> for MyGetter {
        fn update(&mut self) -> NothingOrError<MyError> {
            unsafe {
                GETTER_UPDATE_CALLS += 1;
            }
            Ok(())
        }
    }
    static mut GETTER_GET_CALLS: u8 = 0;
    impl Getter<f32, MyError> for MyGetter {
        fn get(&self) -> Output<f32, MyError> {
            unsafe {
                GETTER_GET_CALLS += 1;
            }
            match unsafe { GETTER_UPDATE_CALLS } {
                0 => panic!("missed update call"),
                1 => Ok(Some(Datum::new(Time::from_seconds_f32(2.0), 1.0))),
                2 => Ok(None),
                3 => Err(MyError(1)),
                _ => panic!("update called too many times"),
            }
        }
    }
    let mut test =
        DimensionAdder::<compile_time_integer::Pos1, compile_time_integer::Neg1, _>::new(MyGetter);
    macro_rules! test_index {
        ($g_u_1: literal, $g_g_1: literal, $gotten_value: expr, $g_u_2: literal, $g_g_2: literal) => {
            test.update().unwrap();
            //This is not RRTK's fault: https://github.com/rust-lang/rust/issues/131443
            #[allow(static_mut_refs)]
            unsafe {
                assert_eq!(GETTER_UPDATE_CALLS, $g_u_1);
                assert_eq!(GETTER_GET_CALLS, $g_g_1);
            }
            assert_eq!(test.get(), $gotten_value);
            #[allow(static_mut_refs)]
            unsafe {
                assert_eq!(GETTER_UPDATE_CALLS, $g_u_2);
                assert_eq!(GETTER_GET_CALLS, $g_g_2);
            }
        };
    }
    #[rustfmt::skip]
    test_index!(1, 0, Ok(Some(Datum::new(Time::from_seconds_f32(2.0), MillimeterPerSecond::new(1.0)))), 1, 1);
    #[rustfmt::skip]
    test_index!(2, 1, Ok(None), 2, 2);
    #[rustfmt::skip]
    test_index!(3, 2, Err(MyError(1)), 3, 3);
}
#[test]
fn dimension_remover() {
    #[derive(Clone, Debug, PartialEq, Eq)]
    struct MyError(u8);
    struct MyGetter;
    static mut GETTER_UPDATE_CALLS: u8 = 0;
    impl Updatable<MyError> for MyGetter {
        fn update(&mut self) -> NothingOrError<MyError> {
            unsafe {
                GETTER_UPDATE_CALLS += 1;
            }
            Ok(())
        }
    }
    static mut GETTER_GET_CALLS: u8 = 0;
    impl Getter<MillimeterPerSecond<f32>, MyError> for MyGetter {
        fn get(&self) -> Output<MillimeterPerSecond<f32>, MyError> {
            unsafe {
                GETTER_GET_CALLS += 1;
            }
            match unsafe { GETTER_UPDATE_CALLS } {
                0 => panic!("missed update call"),
                1 => Ok(Some(Datum::new(
                    Time::from_seconds_f32(2.0),
                    MillimeterPerSecond::new(1.0),
                ))),
                2 => Ok(None),
                3 => Err(MyError(1)),
                _ => panic!("update called too many times"),
            }
        }
    }
    let mut test = DimensionRemover::new(MyGetter);
    macro_rules! test_index {
        ($g_u_1: literal, $g_g_1: literal, $gotten_value: expr, $g_u_2: literal, $g_g_2: literal) => {
            test.update().unwrap();
            //This is not RRTK's fault: https://github.com/rust-lang/rust/issues/131443
            #[allow(static_mut_refs)]
            unsafe {
                assert_eq!(GETTER_UPDATE_CALLS, $g_u_1);
                assert_eq!(GETTER_GET_CALLS, $g_g_1);
            }
            assert_eq!(test.get(), $gotten_value);
            #[allow(static_mut_refs)]
            unsafe {
                assert_eq!(GETTER_UPDATE_CALLS, $g_u_2);
                assert_eq!(GETTER_GET_CALLS, $g_g_2);
            }
        };
    }
    #[rustfmt::skip]
    test_index!(1, 0, Ok(Some(Datum::new(Time::from_seconds_f32(2.0), 1.0))), 1, 1);
    #[rustfmt::skip]
    test_index!(2, 1, Ok(None), 2, 2);
    #[rustfmt::skip]
    test_index!(3, 2, Err(MyError(1)), 3, 3);
}
macro_rules! test_prioritize {
    ($test_name: ident, $stream_name: ident, $first_choice: expr, $second_choice: expr) => {
        #[test]
        fn $test_name() {
            use error::PossibleDoubleError;
            struct MyGetter;
            static mut GETTER_UPDATE_CALLS: u8 = 0;
            impl Updatable<PossibleDoubleError<u8>> for MyGetter {
                fn update(&mut self) -> NothingOrError<PossibleDoubleError<u8>> {
                    unsafe {
                        GETTER_UPDATE_CALLS += 1;
                    }
                    match unsafe { GETTER_UPDATE_CALLS } {
                        0 => unreachable!(),
                        1 => Err(PossibleDoubleError::A(1)),
                        2 => Err(PossibleDoubleError::B(2)),
                        3 => Err(PossibleDoubleError::AB(3, 4)),
                        4..=6 => Ok(()),
                        _ => panic!("update called too many times"),
                    }
                }
            }
            static mut GETTER_GET_CALLS: u8 = 0;
            impl Getter<u8, PossibleDoubleError<u8>> for MyGetter {
                fn get(&self) -> Output<u8, PossibleDoubleError<u8>> {
                    unsafe {
                        GETTER_GET_CALLS += 1;
                    }
                    match unsafe { GETTER_UPDATE_CALLS } {
                        0 => panic!("missed update call"),
                        1 => Ok(Some(Datum::new(Time::from_seconds_f32(1.5), 20))),
                        2 => Ok(None),
                        3 => Ok(None),
                        4 => Err(PossibleDoubleError::A(5)),
                        5 => Err(PossibleDoubleError::B(6)),
                        6 => Err(PossibleDoubleError::AB(7, 8)),
                        _ => panic!("update called too many times"),
                    }
                }
            }
            let mut test = $stream_name::new(MyGetter);
            macro_rules! test_index {
                ($update_value: expr, $g_u_1: literal, $g_g_1: literal, $gotten_value: expr, $g_u_2: literal, $g_g_2: literal) => {
                    assert_eq!(test.update(), $update_value);
                    //This is not RRTK's fault: https://github.com/rust-lang/rust/issues/131443
                    #[allow(static_mut_refs)]
                    unsafe {
                        assert_eq!(GETTER_UPDATE_CALLS, $g_u_1);
                        assert_eq!(GETTER_GET_CALLS, $g_g_1);
                    }
                    assert_eq!(test.get(), $gotten_value);
                    #[allow(static_mut_refs)]
                    unsafe {
                        assert_eq!(GETTER_UPDATE_CALLS, $g_u_2);
                        assert_eq!(GETTER_GET_CALLS, $g_g_2);
                    }
                };
            }
            test_index!(Err(1), 1, 0, Ok(Some(Datum::new(Time::from_seconds_f32(1.5), 20))), 1, 1);
            test_index!(Err(2), 2, 1, Ok(None), 2, 2);
            test_index!(Err($first_choice), 3, 2, Ok(None), 3, 3);
            test_index!(Ok(()), 4, 3, Err(5), 4, 4);
            test_index!(Ok(()), 5, 4, Err(6), 5, 5);
            test_index!(Ok(()), 6, 5, Err($second_choice), 6, 6);
        }
    }
}
test_prioritize!(prioritize_a, PrioritizeA, 3, 7);
test_prioritize!(prioritize_b, PrioritizeB, 4, 8);
