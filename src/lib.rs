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
//!when `std` and `libm` are unavailable.
//!- `internal_enhanced_float` - Do not enable this yourself.
//!
//!RRTK prefers **`std`** over **`libm`** and `libm` over **`micromath`** when multiple are
//!available.
#![warn(missing_docs)]
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
use alloc::rc::Rc;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;
//There is nothing preventing this from being used without any features; we just don't currently,
//and it makes Cargo show a warning since there's an unused use.
#[cfg(any(feature = "alloc", feature = "devices"))]
use core::cell::RefCell;
use core::fmt;
use core::marker::PhantomData;
use core::ops::{
    Add, AddAssign, Deref, DerefMut, Div, DivAssign, Mul, MulAssign, Neg, Not, Sub, SubAssign,
};
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
pub mod reference;
mod state;
pub mod streams;
pub use command::*;
pub use datum::*;
#[cfg(feature = "internal_enhanced_float")]
use enhanced_float::*;
pub use motion_profile::*;
pub use reference::Reference;
#[cfg(feature = "alloc")]
pub use reference::rc_ref_cell_reference;
#[cfg(feature = "std")]
pub use reference::{arc_mutex_reference, arc_rw_lock_reference};
pub use state::*;
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
//TODO: figure out for to use the Error enum with this
#[cfg(any(
    feature = "dim_check_release",
    all(debug_assertions, feature = "dim_check_debug")
))]
impl TryFrom<Unit> for PositionDerivative {
    type Error = ();
    fn try_from(was: Unit) -> Result<Self, ()> {
        Ok(match was {
            MILLIMETER => PositionDerivative::Position,
            MILLIMETER_PER_SECOND => PositionDerivative::Velocity,
            MILLIMETER_PER_SECOND_SQUARED => PositionDerivative::Acceleration,
            _ => return Err(()),
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
    type Error = ();
    fn try_from(was: MotionProfilePiece) -> Result<Self, ()> {
        match was {
            MotionProfilePiece::BeforeStart | MotionProfilePiece::Complete => Err(()),
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
        Self {
            kp: kp,
            ki: ki,
            kd: kd,
        }
    }
    ///Calculate the control variable using the coefficients given error, its integral, and its
    ///derivative.
    #[inline]
    pub fn evaluate(&self, error: f32, error_integral: f32, error_derivative: f32) -> f32 {
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
            position: position,
            velocity: velocity,
            acceleration: acceleration,
        }
    }
    ///Get the k-values for a specific position derivative.
    #[inline]
    pub fn get_k_values(&self, position_derivative: PositionDerivative) -> PIDKValues {
        match position_derivative {
            PositionDerivative::Position => self.position,
            PositionDerivative::Velocity => self.velocity,
            PositionDerivative::Acceleration => self.acceleration,
        }
    }
    ///Calculate the control variable using the coefficients for a given position derivative given
    ///error, its integral, and its derivative.
    #[inline]
    pub fn evaluate(
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
pub trait TimeGetter<E: Copy + Debug>: Updatable<E> {
    ///Get the time.
    fn get(&self) -> TimeOutput<E>;
}
///An object that can return a value, like a [`Getter`], for a given time.
pub trait History<T, E: Copy + Debug>: Updatable<E> {
    ///Get a value at a time.
    fn get(&self, time: Time) -> Option<Datum<T>>;
}
///Something with an [`update`](Updatable::update) method. Mostly for subtraiting.
pub trait Updatable<E: Copy + Debug> {
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
pub trait Getter<G, E: Copy + Debug>: Updatable<E> {
    ///Get something.
    fn get(&self) -> Output<G, E>;
}
///Internal data needed for following a [`Getter`] with a [`Settable`].
pub struct SettableData<S, E: Copy + Debug> {
    following: Option<Reference<dyn Getter<S, E>>>,
    last_request: Option<S>,
}
impl<S, E: Copy + Debug> SettableData<S, E> {
    ///Constructor for [`SettableData`].
    pub const fn new() -> Self {
        Self {
            following: None,
            last_request: None,
        }
    }
}
///Something with a [`set`](Settable::set) method. Usually used for motors and other mechanical components and
///systems. This trait too is fairly broad.
pub trait Settable<S: Clone, E: Copy + Debug>: Updatable<E> {
    ///Set something, not updating the internal [`SettableData`]. Due to current limitations of the
    ///language, you must implement this but call [`set`](Settable::set). Do not call this directly as it will make
    ///[`get_last_request`](Settable::get_last_request) work incorrectly.
    fn impl_set(&mut self, value: S) -> NothingOrError<E>;
    ///Set something to a value. For example, this could set a motor to a voltage. You should call
    ///this and not [`impl_set`](Settable::impl_set).
    fn set(&mut self, value: S) -> NothingOrError<E> {
        self.impl_set(value.clone())?;
        let data = self.get_settable_data_mut();
        data.last_request = Some(value);
        Ok(())
    }
    ///As traits cannot have fields, get functions and separate types are required. All you have to
    ///do is make a field for a corresponding [`SettableData`], make this return an immutable
    ///reference to it, and make [`get_settable_data_mut`](Settable::get_settable_data_mut)
    ///return a mutable reference to it.
    fn get_settable_data_ref(&self) -> &SettableData<S, E>;
    ///As traits cannot have fields, get functions and separate types are required. All you have to
    ///do is make a field for a corresponding [`SettableData`], make this return a mutable
    ///reference to it, and make [`get_settable_data_ref`](Settable::get_settable_data_ref)
    ///return an immutable reference to it.
    fn get_settable_data_mut(&mut self) -> &mut SettableData<S, E>;
    ///Begin following a [`Getter`] of the same type. For this to work, you must have
    ///[`update_following_data`](Settable::update_following_data) in your [`Updatable`] implementation.
    fn follow(&mut self, getter: Reference<dyn Getter<S, E>>) {
        let data = self.get_settable_data_mut();
        data.following = Some(getter);
    }
    ///Stop following the [`Getter`].
    fn stop_following(&mut self) {
        let data = self.get_settable_data_mut();
        data.following = None;
    }
    ///Get a new value from the [`Getter`] we're following, if there is one, and call
    ///[`set`](Settable::set)
    ///accordingly. You must add this to your [`Updatable`] implementation if you are following
    ///[`Getter`]s. This is a current limitation of the Rust language. If specialization is ever
    ///stabilized, this will hopefully be done in a better way.
    fn update_following_data(&mut self) -> NothingOrError<E> {
        let data = self.get_settable_data_ref();
        match &data.following {
            None => {}
            Some(getter) => {
                let new_value = getter.borrow().get()?;
                match new_value {
                    None => {
                        return Ok(());
                    }
                    Some(datum) => {
                        self.set(datum.value)?;
                    }
                }
            }
        }
        Ok(())
    }
    ///Get the argument from the last time [`set`](Settable::set) was called.
    fn get_last_request(&self) -> Option<S> {
        let data = self.get_settable_data_ref();
        data.last_request.clone()
    }
}
//TODO: Update documentation to explain why error type is Option<E>.
///Because [`Getter`]s always return a timestamp (as long as they don't return `Err(_)` or
///`Ok(None)`), we can use this to treat them like [`TimeGetter`]s.
pub struct TimeGetterFromGetter<T: Clone, G: Getter<T, E>, E: Copy + Debug> {
    getter: G,
    phantom_t: PhantomData<T>,
    phantom_e: PhantomData<E>,
}
impl<T: Clone, G: Getter<T, E>, E: Copy + Debug> TimeGetterFromGetter<T, G, E> {
    ///Constructor for [`TimeGetterFromGetter`].
    pub const fn new(getter: G) -> Self {
        Self {
            getter: getter,
            phantom_t: PhantomData,
            phantom_e: PhantomData,
        }
    }
}
impl<T: Clone, G: Getter<T, E>, E: Copy + Debug> TimeGetter<Option<E>>
    for TimeGetterFromGetter<T, G, E>
{
    fn get(&self) -> TimeOutput<Option<E>> {
        match self.getter.get() {
            Err(error) => Err(Some(error)),
            Ok(None) => Err(None),
            Ok(Some(datum)) => Ok(datum.time),
        }
    }
}
impl<T: Clone, G: Getter<T, E>, E: Copy + Debug> Updatable<Option<E>>
    for TimeGetterFromGetter<T, G, E>
{
    fn update(&mut self) -> NothingOrError<Option<E>> {
        Ok(())
    }
}
///As histories return values at times, we can ask them to return values at the time of now or now
///with a delta. This makes that much easier and is the recommended way of following
///[`MotionProfile`]s.
pub struct GetterFromHistory<'a, G, TG: TimeGetter<E>, E: Copy + Debug> {
    history: &'a mut dyn History<G, E>,
    time_getter: Reference<TG>,
    time_delta: Time,
}
impl<'a, G, TG: TimeGetter<E>, E: Copy + Debug> GetterFromHistory<'a, G, TG, E> {
    ///Constructor such that the time in the request to the history will be directly that returned
    ///from the [`TimeGetter`] with no delta.
    pub fn new_no_delta(history: &'a mut impl History<G, E>, time_getter: Reference<TG>) -> Self {
        Self {
            history: history,
            time_getter: time_getter,
            time_delta: Time::default(),
        }
    }
    ///Constructor such that the times requested from the [`History`] will begin at zero where zero
    ///is the moment this constructor is called.
    pub fn new_start_at_zero(
        history: &'a mut impl History<G, E>,
        time_getter: Reference<TG>,
    ) -> Result<Self, E> {
        let time_delta = -time_getter.borrow().get()?;
        Ok(Self {
            history: history,
            time_getter: time_getter,
            time_delta: time_delta,
        })
    }
    ///Constructor such that the times requested from the [`History`] will start at a given time with
    ///that time defined as the moment of construction.
    pub fn new_custom_start(
        history: &'a mut impl History<G, E>,
        time_getter: Reference<TG>,
        start: Time,
    ) -> Result<Self, E> {
        let time_delta = start - time_getter.borrow().get()?;
        Ok(Self {
            history: history,
            time_getter: time_getter,
            time_delta: time_delta,
        })
    }
    ///Constructor with a custom time delta.
    pub fn new_custom_delta(
        history: &'a mut impl History<G, E>,
        time_getter: Reference<TG>,
        time_delta: Time,
    ) -> Self {
        Self {
            history: history,
            time_getter: time_getter,
            time_delta: time_delta,
        }
    }
    ///Set the time delta.
    pub fn set_delta(&mut self, time_delta: Time) {
        self.time_delta = time_delta;
    }
    ///Define now as a given time in the history. Mostly used when construction and use are far
    ///apart in time.
    pub fn set_time(&mut self, time: Time) -> NothingOrError<E> {
        let time_delta = time - self.time_getter.borrow().get()?;
        self.time_delta = time_delta;
        Ok(())
    }
}
impl<G, TG: TimeGetter<E>, E: Copy + Debug> Updatable<E> for GetterFromHistory<'_, G, TG, E> {
    fn update(&mut self) -> NothingOrError<E> {
        self.history.update()?;
        self.time_getter.borrow_mut().update()?;
        Ok(())
    }
}
impl<G, TG: TimeGetter<E>, E: Copy + Debug> Getter<G, E> for GetterFromHistory<'_, G, TG, E> {
    fn get(&self) -> Output<G, E> {
        let time = self.time_getter.borrow().get()?;
        Ok(match self.history.get(time + self.time_delta) {
            Some(datum) => Some(Datum::new(time, datum.value)),
            None => None,
        })
    }
}
///Getter for returning a constant value.
pub struct ConstantGetter<T: Clone, TG: TimeGetter<E>, E: Copy + Debug> {
    settable_data: SettableData<T, E>,
    time_getter: TG,
    value: T,
}
impl<T: Clone, TG: TimeGetter<E>, E: Copy + Debug> ConstantGetter<T, TG, E> {
    ///Constructor for [`ConstantGetter`].
    pub const fn new(time_getter: TG, value: T) -> Self {
        Self {
            settable_data: SettableData::new(),
            time_getter: time_getter,
            value: value,
        }
    }
}
impl<T: Clone, TG: TimeGetter<E>, E: Copy + Debug> Getter<T, E> for ConstantGetter<T, TG, E> {
    fn get(&self) -> Output<T, E> {
        let time = self.time_getter.get()?;
        Ok(Some(Datum::new(time, self.value.clone())))
    }
}
impl<T: Clone, TG: TimeGetter<E>, E: Copy + Debug> Settable<T, E> for ConstantGetter<T, TG, E> {
    fn get_settable_data_ref(&self) -> &SettableData<T, E> {
        &self.settable_data
    }
    fn get_settable_data_mut(&mut self) -> &mut SettableData<T, E> {
        &mut self.settable_data
    }
    fn impl_set(&mut self, value: T) -> NothingOrError<E> {
        self.value = value;
        Ok(())
    }
}
impl<T: Clone, TG: TimeGetter<E>, E: Copy + Debug> Updatable<E> for ConstantGetter<T, TG, E> {
    ///This does not need to be called.
    fn update(&mut self) -> NothingOrError<E> {
        self.update_following_data()?;
        Ok(())
    }
}
///Getter always returning `Ok(None)`.
pub struct NoneGetter;
impl NoneGetter {
    ///Constructor for [`NoneGetter`]. Since [`NoneGetter`] is a unit struct, you can use this or just
    ///the struct's name.
    pub const fn new() -> Self {
        Self
    }
}
impl<T, E: Copy + Debug> Getter<T, E> for NoneGetter {
    fn get(&self) -> Output<T, E> {
        Ok(None)
    }
}
impl<E: Copy + Debug> Updatable<E> for NoneGetter {
    fn update(&mut self) -> NothingOrError<E> {
        Ok(())
    }
}
impl<E: Copy + Debug> TimeGetter<E> for Time {
    fn get(&self) -> TimeOutput<E> {
        Ok(*self)
    }
}
impl<E: Copy + Debug> Updatable<E> for Time {
    fn update(&mut self) -> NothingOrError<E> {
        Ok(())
    }
}
///A place where a device can connect to another.
#[cfg(feature = "devices")]
pub struct Terminal<'a, E: Copy + Debug> {
    settable_data_state: SettableData<Datum<State>, E>,
    settable_data_command: SettableData<Datum<Command>, E>,
    other: Option<&'a RefCell<Terminal<'a, E>>>,
}
#[cfg(feature = "devices")]
impl<E: Copy + Debug> Terminal<'_, E> {
    ///Direct constructor for a [`Terminal`]. You almost always actually want [`RefCell<Terminal>`]
    ///however, in which case you should call [`new`](Terminal::new), which returns [`RefCell<Terminal>`].
    pub const fn new_raw() -> Self {
        Self {
            settable_data_state: SettableData::new(),
            settable_data_command: SettableData::new(),
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
impl<E: Copy + Debug> Settable<Datum<State>, E> for Terminal<'_, E> {
    fn get_settable_data_ref(&self) -> &SettableData<Datum<State>, E> {
        &self.settable_data_state
    }
    fn get_settable_data_mut(&mut self) -> &mut SettableData<Datum<State>, E> {
        &mut self.settable_data_state
    }
    //SettableData takes care of this for us.
    fn impl_set(&mut self, _state: Datum<State>) -> NothingOrError<E> {
        Ok(())
    }
}
#[cfg(feature = "devices")]
impl<E: Copy + Debug> Settable<Datum<Command>, E> for Terminal<'_, E> {
    fn get_settable_data_ref(&self) -> &SettableData<Datum<Command>, E> {
        &self.settable_data_command
    }
    fn get_settable_data_mut(&mut self) -> &mut SettableData<Datum<Command>, E> {
        &mut self.settable_data_command
    }
    fn impl_set(&mut self, _command: Datum<Command>) -> NothingOrError<E> {
        Ok(())
    }
}
#[cfg(feature = "devices")]
impl<E: Copy + Debug> Getter<State, E> for Terminal<'_, E> {
    fn get(&self) -> Output<State, E> {
        let mut addends: [core::mem::MaybeUninit<Datum<State>>; 2] =
            [core::mem::MaybeUninit::uninit(); 2];
        let mut addend_count = 0usize;
        match self.get_last_request() {
            Some(state) => {
                addends[0].write(state);
                addend_count += 1;
            }
            None => (),
        }
        match self.other {
            Some(other) => match other.borrow().get_last_request() {
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
impl<E: Copy + Debug> Getter<Command, E> for Terminal<'_, E> {
    fn get(&self) -> Output<Command, E> {
        let mut maybe_command: Option<Datum<Command>> = None;
        match self.get_last_request() {
            Some(command) => {
                maybe_command = Some(command);
            }
            None => {}
        }
        match self.other {
            Some(other) => {
                match <Terminal<'_, E> as Settable<Datum<Command>, E>>::get_last_request(
                    &other.borrow(),
                ) {
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
                }
            }
            None => (),
        }
        Ok(maybe_command)
    }
}
#[cfg(feature = "devices")]
impl<E: Copy + Debug> Getter<TerminalData, E> for Terminal<'_, E> {
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
impl<E: Copy + Debug> Updatable<E> for Terminal<'_, E> {
    fn update(&mut self) -> NothingOrError<E> {
        <Terminal<'_, E> as Settable<Datum<Command>, E>>::update_following_data(self)?;
        <Terminal<'_, E> as Settable<Datum<State>, E>>::update_following_data(self)?;
        Ok(())
    }
}
///Connect two terminals. Connected terminals should represent a physical connection between
///mechanical devices. This function will automatically disconnect the specified terminals if they
///are connected. You can manually disconnect terminals by calling the
///[`disconnect`](Terminal::disconnect) method on either of them.
#[cfg(feature = "devices")]
pub fn connect<'a, E: Copy + Debug>(
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
    type Error = ();
    fn try_from(value: TerminalData) -> Result<Datum<Command>, ()> {
        match value.command {
            Some(command) => Ok(Datum::new(value.time, command)),
            None => Err(()),
        }
    }
}
#[cfg(feature = "devices")]
impl TryFrom<TerminalData> for Datum<State> {
    type Error = ();
    fn try_from(value: TerminalData) -> Result<Datum<State>, ()> {
        match value.state {
            Some(state) => Ok(Datum::new(value.time, state)),
            None => Err(()),
        }
    }
}
///A mechanical device.
#[cfg(feature = "devices")]
pub trait Device<E: Copy + Debug>: Updatable<E> {
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
