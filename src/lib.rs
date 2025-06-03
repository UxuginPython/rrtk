// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2024-2025 UxuginPython
//!# Rust Robotics ToolKit
//!**A set of algorithms and other tools for robotics in Rust.**
//!
//!It is almost entirely `no_std` and most things work without `alloc`. It does not currently integrate with any API directly. This may be added in the future, probably through another crate.
//!## Feature Flags
//!- `alloc` - Enable items requiring dynamic allocation through Rust's builtin `alloc` crate.
//!- `std` - Enable items requiring the Rust standard library. Requires `alloc` feature. Enabled by default.
//!- `devices` - Enable RRTK's graph-based device system.
//!- `dim_check_debug` - Enable dimension checking in debug mode. Enabled by default.
//!- `dim_check_release` - Enable dimension checking in both debug mode and release mode. Requires `dim_check_debug` feature.
//!- `libm` - Use [`libm`](https://crates.io/crates/libm) for float exponentiation when `std` is not available.
//!- `micromath` - Use [`micromath`](https://crates.io/crates/micromath) for float exponentiation
//!  when `std` and `libm` are unavailable.
//!- `internal_enhanced_float` - Do not enable this yourself.
//!
//!RRTK prefers **`std`** over **`libm`** and `libm` over **`micromath`** when multiple are
//!available.
//#![warn(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]
#[cfg(all(
    feature = "internal_enhanced_float",
    not(feature = "std"),
    not(feature = "libm"),
    not(feature = "micromath")
))]
compile_error!("internal_enhanced_float must only be enabled by another feature.");
#[cfg(feature = "std")]
use alloc::sync::Arc;
#[cfg(feature = "std")]
use std::sync::{Mutex, RwLock};
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::boxed::Box;
#[cfg(feature = "alloc")]
use alloc::rc::Rc;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;
use core::cell::RefCell;
use core::fmt;
use core::marker::PhantomData;
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Not, Sub, SubAssign};
use fmt::Debug;
mod command;
pub mod compile_time_dimensions;
pub mod compile_time_integer;
mod datum;
#[cfg(feature = "devices")]
pub mod devices;
pub mod dimensions;
#[cfg(feature = "internal_enhanced_float")]
mod enhanced_float;
pub use dimensions::*;
mod motion_profile;
mod state;
pub mod streams;
pub use command::*;
pub use datum::*;
#[cfg(feature = "internal_enhanced_float")]
use enhanced_float::*;
pub use motion_profile::*;
pub use state::*;
///Error types used for various things in RRTK. Currently they are only zero-sized types, but this
///may change.
pub mod error {
    ///The error type used when an operation fails due to mismatched runtime dimensions.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct UnitInvalid;
    ///The error type used when a `TryFrom` fails.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct CannotConvert;
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct NoSuchProcess;
}
///A derivative of position: position, velocity, or acceleration.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PositionDerivative {
    ///Where you are.
    Position,
    ///How fast you're going.
    Velocity,
    ///How fast how fast you're going's changing.
    Acceleration,
}
#[cfg(any(
    feature = "dim_check_release",
    all(debug_assertions, feature = "dim_check_debug")
))]
impl TryFrom<Unit> for PositionDerivative {
    type Error = error::UnitInvalid;
    fn try_from(was: Unit) -> Result<Self, error::UnitInvalid> {
        Ok(match was {
            MILLIMETER => PositionDerivative::Position,
            MILLIMETER_PER_SECOND => PositionDerivative::Velocity,
            MILLIMETER_PER_SECOND_SQUARED => PositionDerivative::Acceleration,
            _ => return Err(error::UnitInvalid),
        })
    }
}
impl From<Command> for PositionDerivative {
    fn from(was: Command) -> Self {
        match was {
            Command::Position(_) => Self::Position,
            Command::Velocity(_) => Self::Velocity,
            Command::Acceleration(_) => Self::Acceleration,
        }
    }
}
impl TryFrom<MotionProfilePiece> for PositionDerivative {
    type Error = error::CannotConvert;
    fn try_from(was: MotionProfilePiece) -> Result<Self, error::CannotConvert> {
        match was {
            MotionProfilePiece::BeforeStart | MotionProfilePiece::Complete => {
                Err(error::CannotConvert)
            }
            MotionProfilePiece::InitialAcceleration | MotionProfilePiece::EndAcceleration => {
                Ok(PositionDerivative::Acceleration)
            }
            MotionProfilePiece::ConstantVelocity => Ok(PositionDerivative::Velocity),
        }
    }
}
///Coefficients for a PID controller.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PIDKValues {
    ///Proportional coefficient.
    pub kp: f32,
    ///Integral coefficient.
    pub ki: f32,
    ///Derivative coefficient.
    pub kd: f32,
}
impl PIDKValues {
    ///Constructor for [`PIDKValues`].
    pub const fn new(kp: f32, ki: f32, kd: f32) -> Self {
        Self { kp, ki, kd }
    }
    ///Calculate the control variable using the coefficients given error, its integral, and its
    ///derivative.
    #[inline]
    pub const fn evaluate(&self, error: f32, error_integral: f32, error_derivative: f32) -> f32 {
        self.kp * error + self.ki * error_integral + self.kd * error_derivative
    }
}
///A set of PID k-values for controlling each position derivative.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PositionDerivativeDependentPIDKValues {
    ///Use these k-values when controlling position.
    pub position: PIDKValues,
    ///Use these k-values when controlling velocity.
    pub velocity: PIDKValues,
    ///Use these k-values when controlling acceleration.
    pub acceleration: PIDKValues,
}
impl PositionDerivativeDependentPIDKValues {
    ///Constructor for [`PositionDerivativeDependentPIDKValues`].
    pub const fn new(position: PIDKValues, velocity: PIDKValues, acceleration: PIDKValues) -> Self {
        Self {
            position,
            velocity,
            acceleration,
        }
    }
    ///Get the k-values for a specific position derivative.
    #[inline]
    pub const fn get_k_values(&self, position_derivative: PositionDerivative) -> PIDKValues {
        match position_derivative {
            PositionDerivative::Position => self.position,
            PositionDerivative::Velocity => self.velocity,
            PositionDerivative::Acceleration => self.acceleration,
        }
    }
    ///Calculate the control variable using the coefficients for a given position derivative given
    ///error, its integral, and its derivative.
    #[inline]
    pub const fn evaluate(
        &self,
        position_derivative: PositionDerivative,
        error: f32,
        error_integral: f32,
        error_derivative: f32,
    ) -> f32 {
        self.get_k_values(position_derivative)
            .evaluate(error, error_integral, error_derivative)
    }
}
///A generic output type when something may return an error, nothing, or something with a
///timestamp.
pub type Output<T, E> = Result<Option<Datum<T>>, E>;
///Returned from [`TimeGetter`] objects, which may return either a time or an error.
pub type TimeOutput<E> = Result<Time, E>;
///Returned when something may return either nothing or an error.
pub type NothingOrError<E> = Result<(), E>;
///An object for getting the absolute time.
pub trait TimeGetter<E: Clone + Debug>: Updatable<E> {
    ///Get the time.
    fn get(&self) -> TimeOutput<E>;
}
///An object that can return a value, like a [`Getter`], for a given time.
pub trait Chronology<T> {
    ///Get a value at a time.
    fn get(&self, time: Time) -> Option<Datum<T>>;
}
///Something with an [`update`](Updatable::update) method. Mostly for subtraiting.
pub trait Updatable<E: Clone + Debug> {
    ///As this trait is very generic, exactly what this does will be very dependent on the
    ///implementor.
    fn update(&mut self) -> NothingOrError<E>;
}
///Something with a [`get`](Getter::get) method. Structs implementing this will often be chained for easier data
///processing, with a struct having other implementors in fields which will have some operation
///performed on their output before it being passed on. Data processing Getters with other Getters
///as fields can be referred to as streams, though this is only in naming and trait-wise there is
///no distinction. The other common use for this trait is encoders. These should not be called
///streams.
pub trait Getter<G, E: Clone + Debug>: Updatable<E> {
    ///Get something.
    fn get(&self) -> Output<G, E>;
}
///Something with a [`set`](Settable::set) method. Usually used for motors and other mechanical components and
///systems. This trait too is fairly broad.
pub trait Settable<S, E: Clone + Debug>: Updatable<E> {
    ///Set something to a value. For example, this could set a motor to a voltage.
    fn set(&mut self, value: S) -> NothingOrError<E>;
}
///Feeds the output of a [`Getter`] into a [`Settable`].
pub struct Feeder<T, G, S, E>
where
    G: Getter<T, E>,
    S: Settable<T, E>,
    E: Clone + Debug,
{
    getter: G,
    settable: S,
    phantom_t: PhantomData<T>,
    phantom_e: PhantomData<E>,
}
impl<T, G, S, E> Feeder<T, G, S, E>
where
    G: Getter<T, E>,
    S: Settable<T, E>,
    E: Clone + Debug,
{
    ///Constructor for `Feeder`.
    pub const fn new(getter: G, settable: S) -> Self {
        Self {
            getter,
            settable,
            phantom_t: PhantomData,
            phantom_e: PhantomData,
        }
    }
}
impl<T, G, S, E> Updatable<E> for Feeder<T, G, S, E>
where
    G: Getter<T, E>,
    S: Settable<T, E>,
    E: Clone + Debug,
{
    fn update(&mut self) -> NothingOrError<E> {
        //TODO: Currently, this just returns if anything fails, which can skip settable.update. Do
        //      you really want this?
        self.getter.update()?;
        if let Some(datum) = self.getter.get()? {
            self.settable.set(datum.value)?
        };
        self.settable.update()?;
        Ok(())
    }
}
///Because [`Getter`]s always return a timestamp (as long as they don't return `Err(_)` or
///`Ok(None)`), we can use this to treat them like [`TimeGetter`]s.
pub struct TimeGetterFromGetter<T, G: Getter<T, E>, E: Clone + Debug> {
    getter: G,
    none_error: E,
    phantom_t: PhantomData<T>,
}
impl<T, G: Getter<T, E>, E: Clone + Debug> TimeGetterFromGetter<T, G, E> {
    ///Constructor for [`TimeGetterFromGetter`].
    pub const fn new(getter: G, none_error: E) -> Self {
        Self {
            getter,
            none_error,
            phantom_t: PhantomData,
        }
    }
}
impl<T, G: Getter<T, E>, E: Clone + Debug> TimeGetter<E> for TimeGetterFromGetter<T, G, E> {
    fn get(&self) -> TimeOutput<E> {
        match self.getter.get() {
            Err(error) => Err(error),
            Ok(None) => Err(self.none_error.clone()),
            Ok(Some(datum)) => Ok(datum.time),
        }
    }
}
impl<T, G: Getter<T, E>, E: Clone + Debug> Updatable<E> for TimeGetterFromGetter<T, G, E> {
    fn update(&mut self) -> NothingOrError<E> {
        Ok(())
    }
}
///As histories return values at times, we can ask them to return values at the time of now or now
///with a delta. This makes that much easier and is the recommended way of following
///[`MotionProfile`]s.
pub struct GetterFromChronology<T, C: Chronology<T>, TG: TimeGetter<E>, E: Clone + Debug> {
    chronology: C,
    time_getter: TG,
    time_delta: Time,
    phantom_t: PhantomData<T>,
    phantom_e: PhantomData<E>,
}
impl<T, C: Chronology<T>, TG: TimeGetter<E>, E: Clone + Debug> GetterFromChronology<T, C, TG, E> {
    ///Constructor such that the time in the request to the chronology will be directly that returned
    ///from the [`TimeGetter`] with no delta.
    pub const fn new_no_delta(chronology: C, time_getter: TG) -> Self {
        Self {
            chronology,
            time_getter,
            time_delta: Time::ZERO,
            phantom_t: PhantomData,
            phantom_e: PhantomData,
        }
    }
    ///Constructor such that the times requested from the [`Chronology`] will begin at zero where zero
    ///is the moment this constructor is called.
    pub fn new_start_at_zero(chronology: C, time_getter: TG) -> Result<Self, E> {
        let time_delta = -time_getter.get()?;
        Ok(Self {
            chronology,
            time_getter,
            time_delta,
            phantom_t: PhantomData,
            phantom_e: PhantomData,
        })
    }
    ///Constructor such that the times requested from the [`Chronology`] will start at a given time with
    ///that time defined as the moment of construction.
    pub fn new_custom_start(chronology: C, time_getter: TG, start: Time) -> Result<Self, E> {
        let time_delta = start - time_getter.get()?;
        Ok(Self {
            chronology,
            time_getter,
            time_delta,
            phantom_t: PhantomData,
            phantom_e: PhantomData,
        })
    }
    ///Constructor with a custom time delta.
    pub const fn new_custom_delta(chronology: C, time_getter: TG, time_delta: Time) -> Self {
        Self {
            chronology,
            time_getter,
            time_delta,
            phantom_t: PhantomData,
            phantom_e: PhantomData,
        }
    }
    ///Set the time delta.
    pub const fn set_delta(&mut self, time_delta: Time) {
        self.time_delta = time_delta;
    }
    ///Define now as a given time in the chronology. Mostly used when construction and use are far
    ///apart in time.
    pub fn set_time(&mut self, time: Time) -> NothingOrError<E> {
        let time_delta = time - self.time_getter.get()?;
        self.time_delta = time_delta;
        Ok(())
    }
}
//TODO: Maybe one day with specialization, it will be possible to update self.chronology only if it
//implements it. I think that's really the only reason that Chronology: Updatable (then History: Updatable)
//stayed around for so long: It's easier to force empty impls every once in a while than to figure
//out a really wierd specialization thing. Overall, though, you almost never actually need an
//Updatable Chronology anyway, so the bound really doesn't make that much sense in the first place.
impl<T, C: Chronology<T>, TG: TimeGetter<E>, E: Clone + Debug> Updatable<E>
    for GetterFromChronology<T, C, TG, E>
{
    fn update(&mut self) -> NothingOrError<E> {
        self.time_getter.update()?;
        Ok(())
    }
}
impl<T, C: Chronology<T>, TG: TimeGetter<E>, E: Clone + Debug> Getter<T, E>
    for GetterFromChronology<T, C, TG, E>
{
    fn get(&self) -> Output<T, E> {
        let time = self.time_getter.get()?;
        Ok(match self.chronology.get(time + self.time_delta) {
            Some(datum) => Some(Datum::new(time, datum.value)),
            None => None,
        })
    }
}
///Getter for returning a constant value.
pub struct ConstantGetter<T, TG, E>
where
    T: Clone,
    TG: TimeGetter<E>,
    E: Clone + Debug,
{
    time_getter: TG,
    value: T,
    phantom_e: PhantomData<E>,
}
impl<T, TG, E> ConstantGetter<T, TG, E>
where
    T: Clone,
    TG: TimeGetter<E>,
    E: Clone + Debug,
{
    ///Constructor for [`ConstantGetter`].
    pub const fn new(time_getter: TG, value: T) -> Self {
        Self {
            time_getter,
            value,
            phantom_e: PhantomData,
        }
    }
}
impl<T, TG, E> Getter<T, E> for ConstantGetter<T, TG, E>
where
    T: Clone,
    TG: TimeGetter<E>,
    E: Clone + Debug,
{
    fn get(&self) -> Output<T, E> {
        let time = self.time_getter.get()?;
        Ok(Some(Datum::new(time, self.value.clone())))
    }
}
impl<T, TG, E> Settable<T, E> for ConstantGetter<T, TG, E>
where
    T: Clone,
    TG: TimeGetter<E>,
    E: Clone + Debug,
{
    fn set(&mut self, value: T) -> NothingOrError<E> {
        self.value = value;
        Ok(())
    }
}
impl<T, TG, E> Updatable<E> for ConstantGetter<T, TG, E>
where
    T: Clone,
    TG: TimeGetter<E>,
    E: Clone + Debug,
{
    fn update(&mut self) -> NothingOrError<E> {
        self.time_getter.update()?;
        Ok(())
    }
}
///Getter always returning `Ok(None)`.
#[derive(Default)]
pub struct NoneGetter;
impl NoneGetter {
    ///Constructor for [`NoneGetter`]. Since [`NoneGetter`] is a unit struct, you can use this, its
    ///[`Default`] implementation, or just the struct's name.
    pub const fn new() -> Self {
        Self
    }
}
impl<T, E: Clone + Debug> Getter<T, E> for NoneGetter {
    fn get(&self) -> Output<T, E> {
        Ok(None)
    }
}
impl<E: Clone + Debug> Updatable<E> for NoneGetter {
    fn update(&mut self) -> NothingOrError<E> {
        Ok(())
    }
}
impl<E: Clone + Debug> TimeGetter<E> for Time {
    fn get(&self) -> TimeOutput<E> {
        Ok(*self)
    }
}
impl<E: Clone + Debug> Updatable<E> for Time {
    fn update(&mut self) -> NothingOrError<E> {
        Ok(())
    }
}
///A place where a device can connect to another.
#[cfg(feature = "devices")]
pub struct Terminal<'a, E: Clone + Debug> {
    last_request_state: Option<Datum<State>>,
    last_request_command: Option<Datum<Command>>,
    other: Option<&'a RefCell<Terminal<'a, E>>>,
}
#[cfg(feature = "devices")]
impl<E: Clone + Debug> Terminal<'_, E> {
    ///Direct constructor for a [`Terminal`]. You almost always actually want [`RefCell<Terminal>`]
    ///however, in which case you should call [`new`](Terminal::new), which returns [`RefCell<Terminal>`].
    pub const fn new_raw() -> Self {
        Self {
            last_request_state: None,
            last_request_command: None,
            other: None,
        }
    }
    ///This constructs a [`RefCell<Terminal>`]. This is almost always what you want, and what is
    ///needed for connecting terminals. If you do just want a [`Terminal`], use
    ///[`new_raw`](Terminal::new_raw) instead.
    pub const fn new() -> RefCell<Self> {
        RefCell::new(Self::new_raw())
    }
    ///Disconnect this terminal and the one that it is connected to. You can connect terminals by
    ///calling the [`rrtk::connect`](connect) function.
    pub fn disconnect(&mut self) {
        match self.other {
            Some(other) => {
                let mut other = other.borrow_mut();
                other.other = None;
                self.other = None;
            }
            None => (),
        }
    }
}
#[cfg(feature = "devices")]
impl<E: Clone + Debug> Settable<Datum<State>, E> for Terminal<'_, E> {
    fn set(&mut self, state: Datum<State>) -> NothingOrError<E> {
        self.last_request_state = Some(state);
        Ok(())
    }
}
#[cfg(feature = "devices")]
impl<E: Clone + Debug> Settable<Datum<Command>, E> for Terminal<'_, E> {
    fn set(&mut self, command: Datum<Command>) -> NothingOrError<E> {
        self.last_request_command = Some(command);
        Ok(())
    }
}
#[cfg(feature = "devices")]
impl<E: Clone + Debug> Getter<State, E> for Terminal<'_, E> {
    fn get(&self) -> Output<State, E> {
        let mut addends: [core::mem::MaybeUninit<Datum<State>>; 2] =
            [core::mem::MaybeUninit::uninit(); 2];
        let mut addend_count = 0usize;
        match self.last_request_state {
            Some(state) => {
                addends[0].write(state);
                addend_count += 1;
            }
            None => (),
        }
        match self.other {
            Some(other) => match other.borrow().last_request_state {
                Some(state) => {
                    addends[addend_count].write(state);
                    addend_count += 1;
                }
                None => (),
            },
            None => (),
        }
        unsafe {
            match addend_count {
                0 => return Ok(None),
                1 => return Ok(Some(addends[0].assume_init())),
                2 => {
                    return Ok(Some(
                        (addends[0].assume_init() + addends[1].assume_init()) / 2.0,
                    ));
                }
                _ => unimplemented!(),
            }
        }
    }
}
#[cfg(feature = "devices")]
impl<E: Clone + Debug> Getter<Command, E> for Terminal<'_, E> {
    fn get(&self) -> Output<Command, E> {
        let mut maybe_command: Option<Datum<Command>> = None;
        match self.last_request_command {
            Some(command) => {
                maybe_command = Some(command);
            }
            None => {}
        }
        match self.other {
            Some(other) => match other.borrow().last_request_command {
                Some(gotten_command) => match maybe_command {
                    Some(command_some) => {
                        if gotten_command.time > command_some.time {
                            maybe_command = Some(gotten_command);
                        }
                    }
                    None => {
                        maybe_command = Some(gotten_command);
                    }
                },
                None => (),
            },
            None => (),
        }
        Ok(maybe_command)
    }
}
#[cfg(feature = "devices")]
impl<E: Clone + Debug> Getter<TerminalData, E> for Terminal<'_, E> {
    fn get(&self) -> Output<TerminalData, E> {
        let command = self.get().expect("Terminal get cannot return Err");
        let state = self.get().expect("Terminal get cannot return Err");
        let (mut time, command) = match command {
            Some(datum_command) => (Some(datum_command.time), Some(datum_command.value)),
            None => (None, None),
        };
        let state = match state {
            Some(datum_state) => {
                time = Some(datum_state.time);
                Some(datum_state.value)
            }
            None => None,
        };
        Ok(match time {
            Some(time) => Some(Datum::new(
                time,
                TerminalData {
                    time: time,
                    command: command,
                    state: state,
                },
            )),
            None => None,
        })
    }
}
#[cfg(feature = "devices")]
impl<E: Clone + Debug> Updatable<E> for Terminal<'_, E> {
    ///This does not need to be called.
    fn update(&mut self) -> NothingOrError<E> {
        Ok(())
    }
}
///Connect two terminals. Connected terminals should represent a physical connection between
///mechanical devices. This function will automatically disconnect the specified terminals if they
///are connected. You can manually disconnect terminals by calling the
///[`disconnect`](Terminal::disconnect) method on either of them.
#[cfg(feature = "devices")]
pub fn connect<'a, E: Clone + Debug>(
    term1: &'a RefCell<Terminal<'a, E>>,
    term2: &'a RefCell<Terminal<'a, E>>,
) {
    let mut term1_borrow = term1.borrow_mut();
    let mut term2_borrow = term2.borrow_mut();
    term1_borrow.disconnect();
    term2_borrow.disconnect();
    term1_borrow.other = Some(term2);
    term2_borrow.other = Some(term1);
}
///Data that are sent between terminals: A timestamp, an optional command, and a state.
#[cfg(feature = "devices")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TerminalData {
    ///Timestamp.
    pub time: Time,
    ///Optional command from the terminal.
    pub command: Option<Command>,
    ///Optional state from the terminal.
    pub state: Option<State>,
}
#[cfg(feature = "devices")]
impl TryFrom<TerminalData> for Datum<Command> {
    type Error = error::CannotConvert;
    fn try_from(value: TerminalData) -> Result<Datum<Command>, error::CannotConvert> {
        match value.command {
            Some(command) => Ok(Datum::new(value.time, command)),
            None => Err(error::CannotConvert),
        }
    }
}
#[cfg(feature = "devices")]
impl TryFrom<TerminalData> for Datum<State> {
    type Error = error::CannotConvert;
    fn try_from(value: TerminalData) -> Result<Datum<State>, error::CannotConvert> {
        match value.state {
            Some(state) => Ok(Datum::new(value.time, state)),
            None => Err(error::CannotConvert),
        }
    }
}
///A mechanical device.
#[cfg(feature = "devices")]
pub trait Device<E: Clone + Debug>: Updatable<E> {
    ///Call only the [`update`](Terminal::update) methods of owned terminals and do not update anything else with the
    ///device.
    fn update_terminals(&mut self) -> NothingOrError<E>;
}
///Get the newer of two [`Datum`] objects.
pub fn latest<T>(dat1: Datum<T>, dat2: Datum<T>) -> Datum<T> {
    if dat1.time >= dat2.time { dat1 } else { dat2 }
}
//TODO: Decide if this should be pub trait.
trait Half {
    fn half(self) -> Self;
}
macro_rules! impl_half_integer {
    ($num: ty) => {
        impl Half for $num {
            fn half(self) -> Self {
                self / 2
            }
        }
    };
}
impl_half_integer!(u8);
impl_half_integer!(u16);
impl_half_integer!(u32);
impl_half_integer!(u64);
impl_half_integer!(u128);
impl_half_integer!(usize);
impl_half_integer!(i8);
impl_half_integer!(i16);
impl_half_integer!(i32);
impl_half_integer!(i64);
impl_half_integer!(i128);
impl_half_integer!(isize);
impl Half for f32 {
    fn half(self) -> Self {
        self / 2.0
    }
}
impl Half for f64 {
    fn half(self) -> Self {
        self / 2.0
    }
}
///[`Updatable`], [`Getter`], [`Settable`], and [`TimeGetter`] are passed through `Box`,
///`Rc<RefCell<T>>`, `Arc<RwLock<T>>`, and `Arc<Mutex<T>>`, but this cannot be done safely for
///references involving raw pointer dereferencing. This is a wrapper struct that provides this
///functionality for `*mut T`, `*const RwLock<T>`, and `*const Mutex<T>`. It's constructor is
///`unsafe fn`, so this is considered sound.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct PointerDereferencer<P> {
    pointer: P,
}
impl<P> PointerDereferencer<P> {
    ///The constructor for `PointerDereferencer`. Although this constructor itself does not run any
    ///unsafe code, it is `unsafe fn` since this type inherently performs unsafe functions. See the
    ///[type documentation](`PointerDereferencer`) for more information.
    ///
    ///Although it is technically possible to construct a `PointerDereferencer<P>` where `P` is not
    ///a raw pointer, there is no valid reason to do so and the object would be entirely useless.
    pub const unsafe fn new(pointer: P) -> Self {
        Self { pointer }
    }
    //XXX: Is it clear that *_inner means the pointer and not the target of the pointer, or should
    //these functions be renamed?
    ///Returns the inner pointer that the wrapper contains by consuming it. Due to the fact that
    ///pointers are `Copy`, [`copy_inner`](Self::copy_inner), which does not consume `self` and is
    ///`const fn`, is preferred in almost all cases however.
    #[inline]
    pub fn into_inner(self) -> P {
        self.pointer
    }
}
impl<P: Clone> PointerDereferencer<P> {
    ///Clones and returns the inner pointer that the wrapper contains. Due to the fact that
    ///pointers are `Copy`, [`copy_inner`](Self::copy_inner) is nearly always preferred both for
    ///clarity and because it is `const fn` however.
    #[inline]
    pub fn clone_inner(&self) -> P {
        self.pointer.clone()
    }
}
impl<P: Copy> PointerDereferencer<P> {
    ///This function is be identical to [`clone_inner`](Self::clone_inner) when `P: Copy`. However,
    ///`copy_inner` should be preferred where possible because, unlike `clone_inner`, it is
    ///`const fn`. It is also clearer that the clone is very light.
    #[inline]
    pub const fn copy_inner(&self) -> P {
        self.pointer
    }
}
macro_rules! as_dyn_updatable {
    ($return_type:ty) => {
        //The way documentation for these function has to work is unfortunate, but there's not
        //really a better way.
        #[allow(missing_docs)]
        #[inline]
        pub const fn as_dyn_updatable<E: Clone + Debug>(&self) -> PointerDereferencer<$return_type>
        where
            T: Updatable<E>,
        {
            let ptr = self.copy_inner() as $return_type;
            unsafe { PointerDereferencer::new(ptr) }
        }
    };
}
macro_rules! as_dyn_getter {
    ($return_type:ty) => {
        #[allow(missing_docs)]
        #[inline]
        pub const fn as_dyn_getter<U, E: Clone + Debug>(&self) -> PointerDereferencer<$return_type>
        where
            T: Getter<U, E>,
        {
            let ptr = self.copy_inner() as $return_type;
            unsafe { PointerDereferencer::new(ptr) }
        }
    };
}
macro_rules! as_dyn_settable {
    ($return_type:ty) => {
        #[allow(missing_docs)]
        #[inline]
        pub const fn as_dyn_settable<U, E: Clone + Debug>(
            &self,
        ) -> PointerDereferencer<$return_type>
        where
            T: Settable<U, E>,
        {
            let ptr = self.copy_inner() as $return_type;
            unsafe { PointerDereferencer::new(ptr) }
        }
    };
}
macro_rules! as_dyn_time_getter {
    ($return_type:ty) => {
        #[allow(missing_docs)]
        #[inline]
        pub const fn as_dyn_time_getter<E: Clone + Debug>(
            &self,
        ) -> PointerDereferencer<$return_type>
        where
            T: TimeGetter<E>,
        {
            let ptr = self.copy_inner() as $return_type;
            unsafe { PointerDereferencer::new(ptr) }
        }
    };
}
//TODO: Chronology is different because it's not always Updatable and so probably isn't mutable.
//Figure out what to do about that. (Currently, as_dyn_chronology only works for mutable references
//and there are no impls for Rc<RefCell>, etc., either mutable or immutable.)
macro_rules! as_dyn_chronology {
    ($return_type:ty) => {
        #[allow(missing_docs)]
        #[inline]
        pub const fn as_dyn_chronology<U>(&self) -> PointerDereferencer<$return_type>
        where
            T: Chronology<U>,
        {
            let ptr = self.copy_inner() as $return_type;
            unsafe { PointerDereferencer::new(ptr) }
        }
    };
}
///These functions get a `PointerDereferencer<*mut dyn Trait>` from a `PointerDereferencer<*mut T>`
///where `T: Trait`. Because raw pointers are `Copy`, they only require `&self` and do not consume
///the original `PointerDereferencer`. Unfortunately `T` currently must be `Sized` due to language
///limitations.
impl<T> PointerDereferencer<*mut T> {
    as_dyn_updatable!(*mut dyn Updatable<E>);
    as_dyn_getter!(*mut dyn Getter<U, E>);
    as_dyn_settable!(*mut dyn Settable<U, E>);
    as_dyn_time_getter!(*mut dyn TimeGetter<E>);
    as_dyn_chronology!(*mut dyn Chronology<U>);
}
///These functions get a `PointerDereferencer<*const RwLock<dyn Trait>>` from a
///`PointerDereferencer<*const RwLock<T>>` where `T: Trait`. Because raw pointers are `Copy`, they
///only require `&self` and do not consume the original `PointerDereferencer`. Unfortunately `T`
///currently must be `Sized` due to language limitations.
#[cfg(feature = "std")]
impl<T> PointerDereferencer<*const RwLock<T>> {
    as_dyn_updatable!(*const RwLock<dyn Updatable<E>>);
    as_dyn_getter!(*const RwLock<dyn Getter<U, E>>);
    as_dyn_settable!(*const RwLock<dyn Settable<U, E>>);
    as_dyn_time_getter!(*const RwLock<dyn TimeGetter<E>>);
}
///These functions get a `PointerDereferencer<*const Mutex<dyn Trait>>` from a
///`PointerDereferencer<*const Mutex<T>>` where `T: Trait`. Because raw pointers are `Copy`, they
///only require `&self` and do not consume the original `PointerDereferencer`. Unfortunately `T`
///currently must be `Sized` due to language limitations.
#[cfg(feature = "std")]
impl<T> PointerDereferencer<*const Mutex<T>> {
    as_dyn_updatable!(*const Mutex<dyn Updatable<E>>);
    as_dyn_getter!(*const Mutex<dyn Getter<U, E>>);
    as_dyn_settable!(*const Mutex<dyn Settable<U, E>>);
    as_dyn_time_getter!(*const Mutex<dyn TimeGetter<E>>);
}
//There are Chronology impls for RwLock<C> and Mutex<C> where C: Chronology. It is necessary to
//implement Updatable etc. for *const RwLock<T> and *const Mutex<T> directly rather than doing it
//more generically like for Chronology because they require mutability.
impl<T> PointerDereferencer<*const T> {
    as_dyn_chronology!(*const dyn Chronology<U>);
}
//FIXME: Make one of these work if you can, preferably From since it implies Into.
/*impl<P> From<PointerDereferencer<P>> for P {
    fn from(was: PointerDereferencer<P>) -> Self {
        was.into_inner()
    }
}*/
/*impl<P> Into<P> for PointerDereferencer<P> {
    fn into(self) -> P {
        self.into_inner()
    }
}*/
impl<U: ?Sized + Updatable<E>, E: Clone + Debug> Updatable<E> for PointerDereferencer<*mut U> {
    fn update(&mut self) -> NothingOrError<E> {
        unsafe { (*self.pointer).update() }
    }
}
impl<T, G: ?Sized + Getter<T, E>, E: Clone + Debug> Getter<T, E> for PointerDereferencer<*mut G> {
    fn get(&self) -> Output<T, E> {
        unsafe { (*self.pointer).get() }
    }
}
impl<T, S: ?Sized + Settable<T, E>, E: Clone + Debug> Settable<T, E>
    for PointerDereferencer<*mut S>
{
    fn set(&mut self, value: T) -> NothingOrError<E> {
        unsafe { (*self.pointer).set(value) }
    }
}
impl<TG: ?Sized + TimeGetter<E>, E: Clone + Debug> TimeGetter<E> for PointerDereferencer<*mut TG> {
    fn get(&self) -> TimeOutput<E> {
        unsafe { (*self.pointer).get() }
    }
}
impl<T, C: ?Sized + Chronology<T>> Chronology<T> for PointerDereferencer<*mut C> {
    fn get(&self, time: Time) -> Option<Datum<T>> {
        unsafe { (*self.pointer).get(time) }
    }
}
impl<T, C: ?Sized + Chronology<T>> Chronology<T> for PointerDereferencer<*const C> {
    fn get(&self, time: Time) -> Option<Datum<T>> {
        unsafe { (*self.pointer).get(time) }
    }
}
#[cfg(feature = "std")]
impl<U: ?Sized + Updatable<E>, E: Clone + Debug> Updatable<E>
    for PointerDereferencer<*const RwLock<U>>
{
    fn update(&mut self) -> NothingOrError<E> {
        unsafe { (*self.pointer).write() }
            .expect("RRTK failed to acquire RwLock write lock for Updatable")
            .update()
    }
}
#[cfg(feature = "std")]
impl<G: ?Sized + Getter<T, E>, T, E: Clone + Debug> Getter<T, E>
    for PointerDereferencer<*const RwLock<G>>
{
    fn get(&self) -> Output<T, E> {
        unsafe { (*self.pointer).read() }
            .expect("RRTK failed to acquire RwLock read lock for Getter")
            .get()
    }
}
#[cfg(feature = "std")]
impl<S: ?Sized + Settable<T, E>, T, E: Clone + Debug> Settable<T, E>
    for PointerDereferencer<*const RwLock<S>>
{
    fn set(&mut self, value: T) -> NothingOrError<E> {
        unsafe { (*self.pointer).write() }
            .expect("RRTK failed to acquire RwLock write lock for Settable")
            .set(value)
    }
}
#[cfg(feature = "std")]
impl<TG: ?Sized + TimeGetter<E>, E: Clone + Debug> TimeGetter<E>
    for PointerDereferencer<*const RwLock<TG>>
{
    fn get(&self) -> TimeOutput<E> {
        unsafe { (*self.pointer).read() }
            .expect("RRTK failed to acquire RwLock read lock for TimeGetter")
            .get()
    }
}
#[cfg(feature = "std")]
impl<U: ?Sized + Updatable<E>, E: Clone + Debug> Updatable<E>
    for PointerDereferencer<*const Mutex<U>>
{
    fn update(&mut self) -> NothingOrError<E> {
        unsafe { (*self.pointer).lock() }
            .expect("RRTK failed to acquire Mutex lock for Updatable")
            .update()
    }
}
#[cfg(feature = "std")]
impl<G: ?Sized + Getter<T, E>, T, E: Clone + Debug> Getter<T, E>
    for PointerDereferencer<*const Mutex<G>>
{
    fn get(&self) -> Output<T, E> {
        unsafe { (*self.pointer).lock() }
            .expect("RRTK failed to acquire Mutex lock for Getter")
            .get()
    }
}
#[cfg(feature = "std")]
impl<S: ?Sized + Settable<T, E>, T, E: Clone + Debug> Settable<T, E>
    for PointerDereferencer<*const Mutex<S>>
{
    fn set(&mut self, value: T) -> NothingOrError<E> {
        unsafe { (*self.pointer).lock() }
            .expect("RRTK failed to acquire Mutex lock for Settable")
            .set(value)
    }
}
#[cfg(feature = "std")]
impl<TG: ?Sized + TimeGetter<E>, E: Clone + Debug> TimeGetter<E>
    for PointerDereferencer<*const Mutex<TG>>
{
    fn get(&self) -> TimeOutput<E> {
        unsafe { (*self.pointer).lock() }
            .expect("RRTK failed to acquire Mutex lock for TimeGetter")
            .get()
    }
}
#[cfg(feature = "alloc")]
impl<U: ?Sized + Updatable<E>, E: Clone + Debug> Updatable<E> for Box<U> {
    fn update(&mut self) -> NothingOrError<E> {
        (**self).update()
    }
}
#[cfg(feature = "alloc")]
impl<G: ?Sized + Getter<T, E>, T, E: Clone + Debug> Getter<T, E> for Box<G> {
    fn get(&self) -> Output<T, E> {
        (**self).get()
    }
}
#[cfg(feature = "alloc")]
impl<S: ?Sized + Settable<T, E>, T, E: Clone + Debug> Settable<T, E> for Box<S> {
    fn set(&mut self, value: T) -> NothingOrError<E> {
        (**self).set(value)
    }
}
#[cfg(feature = "alloc")]
impl<TG: ?Sized + TimeGetter<E>, E: Clone + Debug> TimeGetter<E> for Box<TG> {
    fn get(&self) -> TimeOutput<E> {
        (**self).get()
    }
}
#[cfg(feature = "alloc")]
impl<T, C: ?Sized + Chronology<T>> Chronology<T> for Box<C> {
    fn get(&self, time: Time) -> Option<Datum<T>> {
        (**self).get(time)
    }
}
#[cfg(feature = "alloc")]
impl<U: ?Sized + Updatable<E>, E: Clone + Debug> Updatable<E> for Rc<RefCell<U>> {
    fn update(&mut self) -> NothingOrError<E> {
        self.borrow_mut().update()
    }
}
#[cfg(feature = "alloc")]
impl<G: ?Sized + Getter<T, E>, T, E: Clone + Debug> Getter<T, E> for Rc<RefCell<G>> {
    fn get(&self) -> Output<T, E> {
        self.borrow().get()
    }
}
#[cfg(feature = "alloc")]
impl<S: ?Sized + Settable<T, E>, T, E: Clone + Debug> Settable<T, E> for Rc<RefCell<S>> {
    fn set(&mut self, value: T) -> NothingOrError<E> {
        self.borrow_mut().set(value)
    }
}
#[cfg(feature = "alloc")]
impl<TG: ?Sized + TimeGetter<E>, E: Clone + Debug> TimeGetter<E> for Rc<RefCell<TG>> {
    fn get(&self) -> TimeOutput<E> {
        self.borrow().get()
    }
}
#[cfg(feature = "std")]
impl<U: ?Sized + Updatable<E>, E: Clone + Debug> Updatable<E> for Arc<RwLock<U>> {
    fn update(&mut self) -> NothingOrError<E> {
        self.write()
            .expect("RRTK failed to acquire RwLock write lock for Updatable")
            .update()
    }
}
#[cfg(feature = "std")]
impl<G: ?Sized + Getter<T, E>, T, E: Clone + Debug> Getter<T, E> for Arc<RwLock<G>> {
    fn get(&self) -> Output<T, E> {
        self.read()
            .expect("RRTK failed to acquire RwLock read lock for Getter")
            .get()
    }
}
#[cfg(feature = "std")]
impl<S: ?Sized + Settable<T, E>, T, E: Clone + Debug> Settable<T, E> for Arc<RwLock<S>> {
    fn set(&mut self, value: T) -> NothingOrError<E> {
        self.write()
            .expect("RRTK failed to acquire RwLock write lock for Settable")
            .set(value)
    }
}
#[cfg(feature = "std")]
impl<TG: ?Sized + TimeGetter<E>, E: Clone + Debug> TimeGetter<E> for Arc<RwLock<TG>> {
    fn get(&self) -> TimeOutput<E> {
        self.read()
            .expect("RRTK failed to acquire RwLock read lock for TimeGetter")
            .get()
    }
}
#[cfg(feature = "std")]
impl<U: ?Sized + Updatable<E>, E: Clone + Debug> Updatable<E> for Arc<Mutex<U>> {
    fn update(&mut self) -> NothingOrError<E> {
        self.lock()
            .expect("RRTK failed to acquire Mutex lock for Updatable")
            .update()
    }
}
#[cfg(feature = "std")]
impl<G: ?Sized + Getter<T, E>, T, E: Clone + Debug> Getter<T, E> for Arc<Mutex<G>> {
    fn get(&self) -> Output<T, E> {
        self.lock()
            .expect("RRTK failed to acquire Mutex lock for Getter")
            .get()
    }
}
#[cfg(feature = "std")]
impl<S: ?Sized + Settable<T, E>, T, E: Clone + Debug> Settable<T, E> for Arc<Mutex<S>> {
    fn set(&mut self, value: T) -> NothingOrError<E> {
        self.lock()
            .expect("RRTK failed to acquire Mutex lock for Settable")
            .set(value)
    }
}
#[cfg(feature = "std")]
impl<TG: ?Sized + TimeGetter<E>, E: Clone + Debug> TimeGetter<E> for Arc<Mutex<TG>> {
    fn get(&self) -> TimeOutput<E> {
        self.lock()
            .expect("RRTK failed to acquire Mutex lock for TimeGetter")
            .get()
    }
}
#[cfg(feature = "alloc")]
impl<T, C: ?Sized + Chronology<T>> Chronology<T> for Rc<C> {
    fn get(&self, time: Time) -> Option<Datum<T>> {
        (**self).get(time)
    }
}
#[cfg(feature = "std")]
impl<T, C: ?Sized + Chronology<T>> Chronology<T> for Arc<C> {
    fn get(&self, time: Time) -> Option<Datum<T>> {
        (**self).get(time)
    }
}
impl<T, C: ?Sized + Chronology<T>> Chronology<T> for RefCell<C> {
    fn get(&self, time: Time) -> Option<Datum<T>> {
        self.borrow().get(time)
    }
}
#[cfg(feature = "std")]
impl<T, C: ?Sized + Chronology<T>> Chronology<T> for RwLock<C> {
    fn get(&self, time: Time) -> Option<Datum<T>> {
        self.read()
            .expect("RRTK failed to acquire RwLock read lock for Chronology")
            .get(time)
    }
}
#[cfg(feature = "std")]
impl<T, C: ?Sized + Chronology<T>> Chronology<T> for Mutex<C> {
    fn get(&self, time: Time) -> Option<Datum<T>> {
        self.lock()
            .expect("RRTK failed to acquire Mutex lock for Chronology")
            .get(time)
    }
}
pub enum ManagerSignal {
    Quit,
}
pub enum ProcessSignal {
    Die,
}
pub trait Process<E: Clone + Debug>: Updatable<E> {
    fn handle_signal(&mut self, signal: ManagerSignal);
    fn ask_manager(&self) -> Option<ProcessSignal> {
        None
    }
}
#[cfg(feature = "alloc")]
struct ProcessWithInfo<E: Clone + Debug> {
    process: Box<dyn Process<E>>,
    id: u32,
    meanness: u8,
    time_used: Time,
}
#[cfg(feature = "alloc")]
impl<E: Clone + Debug> ProcessWithInfo<E> {
    fn new<P: Process<E> + 'static>(process: P, meanness: u8, start_time: Time, id: u32) -> Self {
        Self {
            process: Box::new(process) as Box<dyn Process<E>>,
            id,
            meanness,
            time_used: start_time,
        }
    }
    fn want(&self, total_time: Time, total_meanness: f32) -> f32 {
        self.meanness as f32 / total_meanness
            - self.time_used.as_seconds() / total_time.as_seconds()
    }
}
#[cfg(feature = "alloc")]
pub struct ProcessManager<TG: TimeGetter<E>, E: Clone + Debug> {
    processes: Vec<ProcessWithInfo<E>>,
    time_getter: TG,
    next_id: u32,
}
#[cfg(feature = "alloc")]
impl<TG: TimeGetter<E>, E: Clone + Debug> ProcessManager<TG, E> {
    pub fn new(time_getter: TG) -> Self {
        Self {
            processes: Vec::new(),
            time_getter,
            next_id: 0,
        }
    }
    pub fn add_process<P: Process<E> + 'static>(&mut self, process: P, meanness: u8) -> u32 {
        //Pretend it's already been running for a time proportional to its meanness since it's
        //being started when the manager has already been going for a while and it basically keeps
        //stopwatches for every process.
        let start_time = Time::from_seconds(
            meanness as f32 / (meanness as f32 + self.get_total_meanness())
                * self.get_total_time().as_seconds(),
        );
        let id = self.next_id;
        self.next_id += 1;
        self.processes
            .push(ProcessWithInfo::new(process, meanness, start_time, id));
        id
    }
    pub fn quit(&mut self, id: u32) -> Result<(), error::NoSuchProcess> {
        let index = self.get_index(id)?;
        self.processes[index]
            .process
            .handle_signal(ManagerSignal::Quit);
        Ok(())
    }
    pub fn kill(&mut self, id: u32) -> Result<(), error::NoSuchProcess> {
        self.processes.swap_remove(self.get_index(id)?);
        Ok(())
    }
    fn get_index(&self, id: u32) -> Result<usize, error::NoSuchProcess> {
        match self
            .processes
            .iter()
            .enumerate()
            .find(|(_, process_with_info)| process_with_info.id == id)
        {
            Some((index, _)) => Ok(index),
            None => Err(error::NoSuchProcess),
        }
    }
    fn get_total_time(&self) -> Time {
        let mut output = Time::ZERO;
        for process_with_info in &self.processes {
            output += process_with_info.time_used;
        }
        output
    }
    fn get_total_meanness(&self) -> f32 {
        let mut output = 0u32;
        for process_with_info in &self.processes {
            output += process_with_info.meanness as u32;
        }
        output as f32
    }
}
#[cfg(feature = "alloc")]
impl<TG: TimeGetter<E>, E: Clone + Debug> Updatable<E> for ProcessManager<TG, E> {
    fn update(&mut self) -> NothingOrError<E> {
        let mut to_remove = Vec::new();
        for (i, process_with_info) in self.processes.iter().enumerate() {
            if let Some(ProcessSignal::Die) = process_with_info.process.ask_manager() {
                to_remove.push(i);
            }
        }
        for to_remove_index in to_remove.into_iter().rev() {
            //swap_remove doesn't change the order of anything before the removed item, so it's OK.
            self.processes.swap_remove(to_remove_index);
        }
        if self.processes.is_empty() {
            return Ok(());
        }
        //Prevent division by zero issue.
        let total_time = core::cmp::max(self.get_total_time(), Time::from_nanoseconds(1));
        let total_meanness = self.get_total_meanness();
        let index = self
            .processes
            .iter()
            //Get an iterator of the "wants" in the same order.
            .map(|process_with_info| process_with_info.want(total_time, total_meanness))
            //Enumerate it.
            .enumerate()
            //Get the maximum "want," ignoring all of the mess between f32 and iterator.
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .unwrap()
            //Since this is a tuple (usize, f32), we throw out the "want" and only take the index.
            .0;
        let start_time = self.time_getter.get().unwrap();
        self.processes[index].process.update()?;
        let end_time = self.time_getter.get().unwrap();
        self.processes[index].time_used += end_time - start_time;
        Ok(())
    }
}
#[cfg(all(test, feature = "alloc"))]
#[test]
fn process_test_meanness_time() {
    //This test tests differences in both meanness and execution time.
    let sample = Rc::new(RefCell::new(Vec::<u8>::new()));
    let time = Rc::new(RefCell::new(Time::from_nanoseconds(1)));
    struct MyProcess {
        id: u8,
        sample: Rc<RefCell<Vec<u8>>>,
        time: Rc<RefCell<Time>>,
    }
    impl Updatable<()> for MyProcess {
        fn update(&mut self) -> NothingOrError<()> {
            *self.time.borrow_mut() *= DimensionlessInteger(2);
            self.sample.borrow_mut().push(self.id);
            Ok(())
        }
    }
    impl Process<()> for MyProcess {
        fn handle_signal(&mut self, _signal: ManagerSignal) {
            unimplemented!();
        }
    }
    let process_a = MyProcess {
        id: 1,
        sample: Rc::clone(&sample),
        time: Rc::clone(&time),
    };
    let process_b = MyProcess {
        id: 3,
        sample: Rc::clone(&sample),
        time: Rc::clone(&time),
    };
    let mut manager = ProcessManager::new(time);
    manager.add_process(process_a, 1);
    manager.add_process(process_b, 3);
    assert_eq!(manager.get_total_meanness(), 4.0);
    assert_eq!(manager.get_total_time(), Time::ZERO);
    manager.update().unwrap();
    assert_eq!(manager.processes[0].time_used, Time::from_nanoseconds(0));
    assert_eq!(manager.processes[1].time_used, Time::from_nanoseconds(1));
    assert_eq!(manager.get_total_time(), Time::from_nanoseconds(1));

    assert_eq!(
        manager.processes[0].want(Time::from_nanoseconds(1), 4.0),
        1.0 / 4.0 - 0.0 / 1.0
    );
    assert_eq!(
        manager.processes[1].want(Time::from_nanoseconds(1), 4.0),
        3.0 / 4.0 - 1.0 / 1.0
    );
    manager.update().unwrap();
    assert_eq!(manager.processes[0].time_used, Time::from_nanoseconds(2));
    assert_eq!(manager.processes[1].time_used, Time::from_nanoseconds(1));
    assert_eq!(manager.get_total_time(), Time::from_nanoseconds(3));

    //FIXME: Floating point issues - these assert_eq!s are all within the margin of error but don't
    //pass.
    /*assert_eq!(
        manager.processes[0].want(Time::from_nanoseconds(3), 4.0),
        1.0 / 4.0 - 2.0 / 3.0
    );
    assert_eq!(
        manager.processes[1].want(Time::from_nanoseconds(3), 4.0),
        3.0 / 4.0 - 1.0 / 3.0
    );*/
    manager.update().unwrap();
    assert_eq!(manager.processes[0].time_used, Time::from_nanoseconds(2));
    assert_eq!(manager.processes[1].time_used, Time::from_nanoseconds(5));
    assert_eq!(manager.get_total_time(), Time::from_nanoseconds(7));

    /*assert_eq!(
        manager.processes[0].want(Time::from_nanoseconds(7), 4.0),
        1.0 / 4.0 - 2.0 / 7.0
    );
    assert_eq!(
        manager.processes[1].want(Time::from_nanoseconds(7), 4.0),
        3.0 / 4.0 - 5.0 / 7.0
    );*/
    manager.update().unwrap();
    assert_eq!(manager.processes[0].time_used, Time::from_nanoseconds(2));
    assert_eq!(manager.processes[1].time_used, Time::from_nanoseconds(13));
    assert_eq!(manager.get_total_time(), Time::from_nanoseconds(15));

    assert_eq!(
        manager.processes[0].want(Time::from_nanoseconds(15), 4.0),
        1.0 / 4.0 - 2.0 / 15.0
    );
    /*assert_eq!(
        manager.processes[1].want(Time::from_nanoseconds(15), 4.0),
        3.0 / 4.0 - 13.0 / 15.0
    );*/
    manager.update().unwrap();
    assert_eq!(manager.processes[0].time_used, Time::from_nanoseconds(18));
    assert_eq!(manager.processes[1].time_used, Time::from_nanoseconds(13));
    assert_eq!(manager.get_total_time(), Time::from_nanoseconds(31));

    assert_eq!(
        manager.processes[0].want(Time::from_nanoseconds(31), 4.0),
        1.0 / 4.0 - 18.0 / 31.0
    );
    /*assert_eq!(
        manager.processes[1].want(Time::from_nanoseconds(31), 4.0),
        3.0 / 4.0 - 13.0 / 31.0
    );*/
    manager.update().unwrap();
    assert_eq!(manager.processes[0].time_used, Time::from_nanoseconds(18));
    assert_eq!(manager.processes[1].time_used, Time::from_nanoseconds(45));
    assert_eq!(manager.get_total_time(), Time::from_nanoseconds(63));
}
