// SPDX-License-Identifier: LGPL-3.0-only
/*
Copyright 2024 UxuginPython on GitHub

     This file is part of Rust Robotics ToolKit.

    Rust Robotics ToolKit is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, version 3.

    Rust Robotics ToolKit is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Lesser General Public License for more details.

    You should have received a copy of the GNU Lesser General Public License along with Rust Robotics ToolKit. If not, see <https://www.gnu.org/licenses/>.
*/
//!Rust Robotics ToolKit
//!A set of algorithms and other tools for robotics in Rust.
//!It is partially `no_std`. It does not currently integrate with any API directly, but this may be added in the future.
#![warn(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]
#[cfg(feature = "std")]
use std::cell::RefCell;
#[cfg(feature = "std")]
use std::fmt::Debug;
#[cfg(feature = "std")]
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
#[cfg(feature = "std")]
use std::rc::Rc;
#[cfg(not(feature = "std"))]
extern crate alloc;
#[cfg(not(feature = "std"))]
use alloc::boxed::Box;
#[cfg(not(feature = "std"))]
use alloc::rc::Rc;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;
#[cfg(not(feature = "std"))]
use core::cell::RefCell;
#[cfg(not(feature = "std"))]
use core::fmt::Debug;
#[cfg(not(feature = "std"))]
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
pub mod devices;
mod motion_profile;
pub mod streams;
pub use motion_profile::*;
///RRTK follows the enum style of error handling. This is the error type returned from nearly all
///RRTK types, but you can add your own custom error type using `Other(O)`. It is strongly
///recommended that you use a single `O` type across your crate.
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub enum Error<O: Copy + Debug> {
    ///Returned when a `None` is elevated to an error by a `NoneToError`.
    FromNone,
    ///Raised when two terminals must be connected, but we cannot get `Rc<RefCell<Terminal<O>>>`s
    ///for both.
    CannotConnectTerminals,
    ///A custom error of a user-defined type. Not created by any RRTK type but can be propagated by
    ///them.
    Other(O),
}
///A one-dimensional motion state with position, velocity, and acceleration.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct State {
    ///Where you are.
    pub position: f32,
    ///How fast you're going.
    pub velocity: f32,
    ///How fast how fast you're going's changing.
    pub acceleration: f32,
}
impl State {
    ///Constructor for `State`.
    pub fn new(position: f32, velocity: f32, acceleration: f32) -> State {
        State {
            position: position,
            velocity: velocity,
            acceleration: acceleration,
        }
    }
    ///Calculate the future state assuming a constant acceleration.
    pub fn update(&mut self, delta_time: i64) {
        let delta_time = delta_time as f32;
        let new_velocity = self.velocity + delta_time * self.acceleration;
        let new_position = self.position + delta_time * (self.velocity + new_velocity) / 2.0;
        self.position = new_position;
        self.velocity = new_velocity;
    }
    ///Set the acceleration.
    pub fn set_constant_acceleration(&mut self, acceleration: f32) {
        self.acceleration = acceleration;
    }
    ///Set the velocity to a given value and acceleration to zero.
    pub fn set_constant_velocity(&mut self, velocity: f32) {
        self.acceleration = 0.0;
        self.velocity = velocity;
    }
    ///Set the position to a given value and the velocity and acceleration to zero.
    pub fn set_constant_position(&mut self, position: f32) {
        self.acceleration = 0.0;
        self.velocity = 0.0;
        self.position = position;
    }
    ///State contains a position, velocity, and acceleration. This gets the respective field of a
    ///given position derivative.
    pub fn get_value(&self, position_derivative: PositionDerivative) -> f32 {
        match position_derivative {
            PositionDerivative::Position => self.position,
            PositionDerivative::Velocity => self.velocity,
            PositionDerivative::Acceleration => self.acceleration,
        }
    }
}
impl Neg for State {
    type Output = Self;
    fn neg(self) -> Self {
        State::new(-self.position, -self.velocity, -self.acceleration)
    }
}
impl Add for State {
    type Output = Self;
    fn add(self, other: State) -> Self {
        State::new(
            self.position + other.position,
            self.velocity + other.velocity,
            self.acceleration + other.acceleration,
        )
    }
}
impl Sub for State {
    type Output = Self;
    fn sub(self, other: State) -> Self {
        State::new(
            self.position - other.position,
            self.velocity - other.velocity,
            self.acceleration - other.acceleration,
        )
    }
}
impl Mul<f32> for State {
    type Output = Self;
    fn mul(self, coef: f32) -> Self {
        State::new(
            self.position * coef,
            self.velocity * coef,
            self.acceleration * coef,
        )
    }
}
impl Div<f32> for State {
    type Output = Self;
    fn div(self, dvsr: f32) -> Self {
        State::new(
            self.position / dvsr,
            self.velocity / dvsr,
            self.acceleration / dvsr,
        )
    }
}
impl AddAssign for State {
    fn add_assign(&mut self, other: State) {
        *self = *self + other;
    }
}
impl SubAssign for State {
    fn sub_assign(&mut self, other: State) {
        *self = *self - other;
    }
}
impl MulAssign<f32> for State {
    fn mul_assign(&mut self, coef: f32) {
        *self = *self * coef;
    }
}
impl DivAssign<f32> for State {
    fn div_assign(&mut self, dvsr: f32) {
        *self = *self / dvsr;
    }
}
///Get the newer of two `Datum` objects.
pub fn latest<T>(dat1: Datum<T>, dat2: Datum<T>) -> Datum<T> {
    if dat1.time >= dat2.time {
        dat1
    } else {
        dat2
    }
}
///A container for a time and something else, usually an `f32` or a `State`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Datum<T> {
    ///Timestamp for the datum. This should probably be absolute.
    pub time: i64,
    ///The thing with the timestamp.
    pub value: T,
}
impl<T> Datum<T> {
    ///Constructor for Datum type.
    pub fn new(time: i64, value: T) -> Datum<T> {
        Datum {
            time: time,
            value: value,
        }
    }
}
//Unfortunately implementing the ops traits is really awkward here and has unnecessary restrictions
//because of needing to provide implementations for T and Datum<T>. If we ever get negative trait
//bounds, it will be possible to provide a much better generic implementation for cases where the
//type of other is not necessarily the same as T. Additionally, I may just be doing it wrong.
//Regardless, for now, these are only implemented where other is T or Datum<T> and not a more
//generic type. A special case for State is provided due to its Mul<f32> and Div<f32>
//implementations.
impl<T: Neg<Output = O>, O> Neg for Datum<T> {
    type Output = Datum<O>;
    fn neg(self) -> Datum<O> {
        Datum::new(self.time, -self.value)
    }
}
impl<T: Add<Output = O>, O> Add for Datum<T> {
    type Output = Datum<O>;
    fn add(self, other: Self) -> Datum<O> {
        let output_value = self.value + other.value;
        let output_time = if self.time >= other.time {
            self.time
        } else {
            other.time
        };
        Datum::new(output_time, output_value)
    }
}
impl<T: AddAssign> AddAssign for Datum<T> {
    fn add_assign(&mut self, other: Self) {
        self.value += other.value;
        self.time = if self.time >= other.time {
            self.time
        } else {
            other.time
        };
    }
}
impl<T: Add<Output = O>, O> Add<T> for Datum<T> {
    type Output = Datum<O>;
    fn add(self, other: T) -> Datum<O> {
        let output_value = self.value + other;
        Datum::new(self.time, output_value)
    }
}
impl<T: AddAssign> AddAssign<T> for Datum<T> {
    fn add_assign(&mut self, other: T) {
        self.value += other;
    }
}
impl<T: Sub<Output = O>, O> Sub for Datum<T> {
    type Output = Datum<O>;
    fn sub(self, other: Self) -> Datum<O> {
        let output_value = self.value - other.value;
        let output_time = if self.time >= other.time {
            self.time
        } else {
            other.time
        };
        Datum::new(output_time, output_value)
    }
}
impl<T: SubAssign> SubAssign for Datum<T> {
    fn sub_assign(&mut self, other: Self) {
        self.value -= other.value;
        self.time = if self.time >= other.time {
            self.time
        } else {
            other.time
        };
    }
}
impl<T: Sub<Output = O>, O> Sub<T> for Datum<T> {
    type Output = Datum<O>;
    fn sub(self, other: T) -> Datum<O> {
        let output_value = self.value - other;
        Datum::new(self.time, output_value)
    }
}
impl<T: SubAssign> SubAssign<T> for Datum<T> {
    fn sub_assign(&mut self, other: T) {
        self.value -= other;
    }
}
impl<T: Mul<Output = O>, O> Mul for Datum<T> {
    type Output = Datum<O>;
    fn mul(self, other: Self) -> Datum<O> {
        let output_value = self.value * other.value;
        let output_time = if self.time >= other.time {
            self.time
        } else {
            other.time
        };
        Datum::new(output_time, output_value)
    }
}
impl<T: MulAssign> MulAssign for Datum<T> {
    fn mul_assign(&mut self, other: Self) {
        self.value *= other.value;
        self.time = if self.time >= other.time {
            self.time
        } else {
            other.time
        };
    }
}
impl<T: Mul<Output = O>, O> Mul<T> for Datum<T> {
    type Output = Datum<O>;
    fn mul(self, other: T) -> Datum<O> {
        let output_value = self.value * other;
        Datum::new(self.time, output_value)
    }
}
impl<T: MulAssign> MulAssign<T> for Datum<T> {
    fn mul_assign(&mut self, other: T) {
        self.value *= other;
    }
}
impl<T: Div<Output = O>, O> Div for Datum<T> {
    type Output = Datum<O>;
    fn div(self, other: Self) -> Datum<O> {
        let output_value = self.value / other.value;
        let output_time = if self.time >= other.time {
            self.time
        } else {
            other.time
        };
        Datum::new(output_time, output_value)
    }
}
impl<T: DivAssign> DivAssign for Datum<T> {
    fn div_assign(&mut self, other: Self) {
        self.value /= other.value;
        self.time = if self.time >= other.time {
            self.time
        } else {
            other.time
        };
    }
}
impl<T: Div<Output = O>, O> Div<T> for Datum<T> {
    type Output = Datum<O>;
    fn div(self, other: T) -> Datum<O> {
        let output_value = self.value / other;
        Datum::new(self.time, output_value)
    }
}
impl<T: DivAssign> DivAssign<T> for Datum<T> {
    fn div_assign(&mut self, other: T) {
        self.value /= other;
    }
}
impl Mul<Datum<f32>> for Datum<State> {
    type Output = Self;
    fn mul(self, other: Datum<f32>) -> Self {
        let output_value = self.value * other.value;
        let output_time = if self.time >= other.time {
            self.time
        } else {
            other.time
        };
        Datum::new(output_time, output_value)
    }
}
impl MulAssign<Datum<f32>> for Datum<State> {
    fn mul_assign(&mut self, other: Datum<f32>) {
        self.value *= other.value;
        self.time = if self.time >= other.time {
            self.time
        } else {
            other.time
        };
    }
}
impl Mul<f32> for Datum<State> {
    type Output = Self;
    fn mul(self, other: f32) -> Self {
        let output_value = self.value * other;
        Datum::new(self.time, output_value)
    }
}
impl MulAssign<f32> for Datum<State> {
    fn mul_assign(&mut self, other: f32) {
        self.value *= other;
    }
}
impl Div<Datum<f32>> for Datum<State> {
    type Output = Self;
    fn div(self, other: Datum<f32>) -> Self {
        let output_value = self.value / other.value;
        let output_time = if self.time >= other.time {
            self.time
        } else {
            other.time
        };
        Datum::new(output_time, output_value)
    }
}
impl DivAssign<Datum<f32>> for Datum<State> {
    fn div_assign(&mut self, other: Datum<f32>) {
        self.value /= other.value;
        self.time = if self.time >= other.time {
            self.time
        } else {
            other.time
        };
    }
}
impl Div<f32> for Datum<State> {
    type Output = Self;
    fn div(self, other: f32) -> Self {
        let output_value = self.value / other;
        Datum::new(self.time, output_value)
    }
}
impl DivAssign<f32> for Datum<State> {
    fn div_assign(&mut self, other: f32) {
        self.value /= other;
    }
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
    ///Constructor for `PIDKValues`.
    pub fn new(kp: f32, ki: f32, kd: f32) -> Self {
        Self {
            kp: kp,
            ki: ki,
            kd: kd,
        }
    }
    //TODO: test this lol
    ///Calculate the control variable using the coefficients given error, its integral, and its
    ///derivative.
    #[inline]
    pub fn evaluate(&self, error: f32, error_integral: f32, error_derivative: f32) -> f32 {
        self.kp * error + self.ki * error_integral + self.kd * error_derivative
    }
}
///A set of PID k-values for controlling each position derivative.
pub struct PositionDerivativeDependentPIDKValues {
    ///Use these k-values when controlling position.
    pub position: PIDKValues,
    ///Use these k-values when controlling velocity.
    pub velocity: PIDKValues,
    ///Use these k-values when controlling acceleration.
    pub acceleration: PIDKValues,
}
impl PositionDerivativeDependentPIDKValues {
    ///Constructor for `PositionDerivativeDependentPIDKValues`.
    pub fn new(position: PIDKValues, velocity: PIDKValues, acceleration: PIDKValues) -> Self {
        Self {
            position: position,
            velocity: velocity,
            acceleration: acceleration,
        }
    }
    //TODO: test this lol
    ///Get the k-values for a specific position derivative.
    #[inline]
    pub fn get_k_values(&self, position_derivative: PositionDerivative) -> PIDKValues {
        match position_derivative {
            PositionDerivative::Position => self.position,
            PositionDerivative::Velocity => self.velocity,
            PositionDerivative::Acceleration => self.acceleration,
        }
    }
    //TODO: test this lol
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
pub type Output<T, E> = Result<Option<Datum<T>>, Error<E>>;
///Returned from `TimeGetter` objects, which may return either a time or an error.
pub type TimeOutput<E> = Result<i64, Error<E>>;
///Makes `Getter`s easier to work with by containing them in an `Rc<RefCell<Box<_>>>`.
pub type InputGetter<T, E> = Rc<RefCell<Box<dyn Getter<T, E>>>>;
///Makes `TimeGetter`s easier to work with by containing them in an `Rc<RefCell<Box<_>>>`.
pub type InputTimeGetter<E> = Rc<RefCell<Box<dyn TimeGetter<E>>>>;
///Returned when something may return either nothing or an error.
pub type NothingOrError<E> = Result<(), Error<E>>;
///An object for getting the absolute time.
pub trait TimeGetter<E: Copy + Debug>: Updatable<E> {
    ///Get the time.
    fn get(&self) -> TimeOutput<E>;
}
///An object that can return a value, like a `Getter`, for a given time.
pub trait History<T: Clone, E: Copy + Debug>: Updatable<E> {
    ///Get a value at a time.
    fn get(&self, time: i64) -> Option<Datum<T>>;
}
///A command for a motor to perform: go to a position, run at a velocity, or accelerate at a rate.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Command {
    ///Controls whether you go to a position, run at a velocity, or accelerate at a rate.
    pub position_derivative: PositionDerivative,
    ///The position, velocity, or acceleration rate.
    pub value: f32,
}
impl Command {
    ///Constructor for `Command`.
    pub fn new(position_derivative: PositionDerivative, value: f32) -> Self {
        Self {
            position_derivative: position_derivative,
            value: value,
        }
    }
    ///Get the commanded constant position if there is one. If `position_derivative` is
    ///`PositionDerivative::Velocity` or `PositionDerivative::Acceleration`, this will return
    ///`None` as there is not a constant position.
    pub fn get_position(&self) -> Option<f32> {
        match self.position_derivative {
            PositionDerivative::Position => Some(self.value),
            _ => None,
        }
    }
    ///Get the commanded constant velocity if there is one. If `position_derivative` is
    ///`PositionDerivative::Acceleration`, this will return `None` as there is not a constant
    ///velocity. If `position_derivative` is `PositionDerivative::Position`, this will return 0 as
    ///velocity should be zero with a constant position.
    pub fn get_velocity(&self) -> Option<f32> {
        match self.position_derivative {
            PositionDerivative::Position => Some(0.0),
            PositionDerivative::Velocity => Some(self.value),
            PositionDerivative::Acceleration => None,
        }
    }
    ///Get the commanded constant acceleration if there is one. If `position_derivative` is not
    ///`PositionDerivative::Acceleration`, this will return `None` as there is not a constant
    ///acceleration.
    pub fn get_acceleration(&self) -> f32 {
        match self.position_derivative {
            PositionDerivative::Acceleration => self.value,
            _ => 0.0,
        }
    }
}
impl From<State> for Command {
    fn from(state: State) -> Self {
        if state.acceleration == 0.0 {
            if state.velocity == 0.0 {
                return Command::new(PositionDerivative::Position, state.position);
            } else {
                return Command::new(PositionDerivative::Velocity, state.velocity);
            }
        } else {
            return Command::new(PositionDerivative::Acceleration, state.acceleration);
        }
    }
}
///Something with an `update` method. Mostly for subtraiting.
pub trait Updatable<E: Copy + Debug> {
    ///As this trait is very generic, exactly what this does will be very dependent on the
    ///implementor.
    fn update(&mut self) -> Result<(), Error<E>>;
}
///Something with a `get` method. Structs implementing this will often be chained for easier data
///processing, with a struct having other implementors in fields which will have some operation
///performed on their output before it being passed on. Data processing Getters with other Getters
///as fields can be referred to as streams, though this is only in naming and trait-wise there is
///no distinction. The other common use for this trait is encoders. These should not be called
///streams.
pub trait Getter<G, E: Copy + Debug>: Updatable<E> {
    ///Get something.
    fn get(&self) -> Output<G, E>;
}
///Internal data needed for following a `Getter` with a `Settable`.
pub struct SettableData<S, E: Copy + Debug> {
    pub(crate) following: SettableFollowing<S, E>,
    pub(crate) last_request: Option<S>,
}
impl<S, E: Copy + Debug> SettableData<S, E> {
    ///Constructor for SettableData.
    pub fn new() -> Self {
        Self {
            following: SettableFollowing::Idle,
            last_request: None,
        }
    }
}
enum SettableFollowing<S, E: Copy + Debug> {
    Idle,
    Following(InputGetter<S, E>),
}
///Something with a `set` method. Usually used for motors and other mechanical components and
///systems. This trait too is fairly broad.
pub trait Settable<S: Clone, E: Copy + Debug>: Updatable<E> {
    ///Set something, not updating the internal `SettableData`. Due to current limitations of the
    ///language, you must implement this but call `set`. Do not call this directly as it will make
    ///`get_last_request` work incorrectly.
    fn direct_set(&mut self, value: S) -> Result<(), Error<E>>;
    ///Set something to a value. For example, this could set a motor to a voltage. You should call
    ///this and not `direct_set`.
    fn set(&mut self, value: S) -> Result<(), Error<E>> {
        self.direct_set(value.clone())?;
        let data = self.get_settable_data_mut();
        data.last_request = Some(value);
        Ok(())
    }
    ///As traits cannot have fields, get functions and separate types are required. All you have to
    ///do is make a field for a corresponding `SettableData` and make this return an immutable
    ///reference to it.
    fn get_settable_data_ref(&self) -> &SettableData<S, E>;
    ///As traits cannot have fields, get functions and separate types are required. All you have to
    ///do is make a field for a corresponding `SettableData` and make this return a mutable
    ///reference to it.
    fn get_settable_data_mut(&mut self) -> &mut SettableData<S, E>;
    ///Begin following a `Getter` of the same type.
    fn follow(&mut self, getter: InputGetter<S, E>) {
        let data = self.get_settable_data_mut();
        data.following = SettableFollowing::Following(getter);
    }
    ///Stop following the `Getter`.
    fn stop_following(&mut self) {
        let data = self.get_settable_data_mut();
        data.following = SettableFollowing::Idle;
    }
    ///Get a new value from the `Getter` we're following and update ourselves accordingly. Note
    ///that will call `self.update` regardless if we're following a `Getter`, so you can call with
    ///in lieu of the standard `update` if you're following stuff.
    fn following_update(&mut self) -> NothingOrError<E> {
        let data = self.get_settable_data_ref();
        match &data.following {
            SettableFollowing::Idle => {}
            SettableFollowing::Following(getter) => {
                let new_value = getter.borrow().get()?;
                match new_value {
                    None => {
                        self.update()?;
                        return Ok(());
                    }
                    Some(datum) => {
                        self.set(datum.value)?;
                    }
                }
            }
        }
        self.update()?;
        Ok(())
    }
    ///Get the argument from the last time `set` was called.
    fn get_last_request(&self) -> Option<S> {
        let data = self.get_settable_data_ref();
        data.last_request.clone()
    }
}
///Solely for subtraiting. Allows you to require that a type implements both `Getter` and
///`Settable` with a single trait. No methods and does nothing on its own.
pub trait GetterSettable<G, S: Clone, E: Copy + Debug>: Getter<G, E> + Settable<S, E> {}
///A fast way to turn anything implementing `Getter` into an `InputGetter`.
#[macro_export]
macro_rules! make_input_getter {
    ($stream:expr, $ttype:tt, $etype:tt) => {
        Rc::new(RefCell::new(
            Box::new($stream) as Box<dyn Getter<$ttype, $etype>>
        ))
    };
}
///A fast way to turn anything implementing `TimeGetter` into an `InputTimeGetter`.
#[macro_export]
macro_rules! make_input_time_getter {
    ($time_getter:expr, $etype:tt) => {
        Rc::new(RefCell::new(
            Box::new($time_getter) as Box<dyn TimeGetter<$etype>>
        ))
    };
}
///Because `Stream`s always return a timestamp (as long as they don't return `Err(_)` or
///`Ok(None)`), we can use this to treat them like `TimeGetter`s.
pub struct TimeGetterFromGetter<T: Clone, E> {
    elevator: streams::converters::NoneToError<T, E>,
}
impl<T: Clone, E> TimeGetterFromGetter<T, E> {
    ///Constructor for `TimeGetterFromGetter`.
    pub fn new(stream: InputGetter<T, E>) -> Self {
        Self {
            elevator: streams::converters::NoneToError::new(Rc::clone(&stream)),
        }
    }
}
impl<T: Clone, E: Copy + Debug> TimeGetter<E> for TimeGetterFromGetter<T, E> {
    fn get(&self) -> TimeOutput<E> {
        let output = self.elevator.get()?;
        let output = output.expect("`NoneToError` made all `Ok(None)`s into `Err(_)`s, and `?` returned all `Err(_)`s, so we're sure this is now an `Ok(Some(_))`.");
        return Ok(output.time);
    }
}
impl<T: Clone, E: Copy + Debug> Updatable<E> for TimeGetterFromGetter<T, E> {
    fn update(&mut self) -> NothingOrError<E> {
        Ok(())
    }
}
///As histories return values at times, we can ask them to return values at the time of now or now
///with a delta. This makes that much easier and is the recommended way of following
///`MotionProfile`s.
pub struct GetterFromHistory<G, E: Copy + Debug> {
    history: Box<dyn History<G, E>>,
    time_getter: InputTimeGetter<E>,
    time_delta: i64,
}
impl<G, E: Copy + Debug> GetterFromHistory<G, E> {
    ///Constructor such that the time in the request to the history will be directly that returned
    ///from the `TimeGetter` with no delta.
    pub fn new_no_delta(history: Box<dyn History<G, E>>, time_getter: InputTimeGetter<E>) -> Self {
        Self {
            history: history,
            time_getter: time_getter,
            time_delta: 0,
        }
    }
    ///Constructor such that the times requested from the `History` will begin at zero where zero
    ///is the moment this constructor is called.
    pub fn new_start_at_zero(
        history: Box<dyn History<G, E>>,
        time_getter: InputTimeGetter<E>,
    ) -> Result<Self, Error<E>> {
        let time_delta = -time_getter.borrow().get()?;
        Ok(Self {
            history: history,
            time_getter: time_getter,
            time_delta: time_delta,
        })
    }
    ///Constructor such that the times requested from the `History` will start at a given time with
    ///that time defined as the moment of construction.
    pub fn new_custom_start(
        history: Box<dyn History<G, E>>,
        time_getter: InputTimeGetter<E>,
        start: i64,
    ) -> Result<Self, Error<E>> {
        let time_delta = start - time_getter.borrow().get()?;
        Ok(Self {
            history: history,
            time_getter: time_getter,
            time_delta: time_delta,
        })
    }
    ///Constructor with a custom time delta.
    pub fn new_custom_delta(
        history: Box<dyn History<G, E>>,
        time_getter: InputTimeGetter<E>,
        time_delta: i64,
    ) -> Self {
        Self {
            history: history,
            time_getter: time_getter,
            time_delta: time_delta,
        }
    }
    ///Set the time delta.
    pub fn set_delta(&mut self, time_delta: i64) {
        self.time_delta = time_delta;
    }
    ///Define now as a given time in the history. Mostly used when construction and use are far
    ///apart in time.
    pub fn set_time(&mut self, time: i64) -> NothingOrError<E> {
        let time_delta = time - self.time_getter.borrow().get()?;
        self.time_delta = time_delta;
        Ok(())
    }
}
impl<E: Copy + Debug> GetterFromHistory<Command, E> {
    ///Shortcut to make following motion profiles easier. Calls `new_start_at_zero` internally.
    pub fn new_for_motion_profile(
        motion_profile: MotionProfile,
        time_getter: InputTimeGetter<E>,
    ) -> Result<Self, Error<E>> {
        Self::new_start_at_zero(
            Box::new(motion_profile) as Box<dyn History<Command, E>>,
            time_getter,
        )
    }
}
impl<G, E: Copy + Debug> Updatable<E> for GetterFromHistory<G, E> {
    fn update(&mut self) -> NothingOrError<E> {
        self.history.update()?;
        self.time_getter.borrow_mut().update()?;
        Ok(())
    }
}
impl<G: Clone, E: Copy + Debug> Getter<G, E> for GetterFromHistory<G, E> {
    fn get(&self) -> Output<G, E> {
        Ok(self
            .history
            .get(self.time_getter.borrow().get()? + self.time_delta))
    }
}
///Getter for returning a constant value.
pub struct ConstantGetter<T, E: Copy + Debug> {
    settable_data: SettableData<T, E>,
    time_getter: InputTimeGetter<E>,
    value: T,
}
impl<T, E: Copy + Debug> ConstantGetter<T, E> {
    ///Constructor for `ConstantGetter`.
    pub fn new(time_getter: InputTimeGetter<E>, value: T) -> Self {
        Self {
            settable_data: SettableData::new(),
            time_getter: time_getter,
            value: value,
        }
    }
}
impl<T: Clone, E: Copy + Debug> GetterSettable<T, T, E> for ConstantGetter<T, E> {}
impl<T: Clone, E: Copy + Debug> Getter<T, E> for ConstantGetter<T, E> {
    fn get(&self) -> Output<T, E> {
        let time = self.time_getter.borrow().get()?;
        Ok(Some(Datum::new(time, self.value.clone())))
    }
}
impl<T: Clone, E: Copy + Debug> Settable<T, E> for ConstantGetter<T, E> {
    fn get_settable_data_ref(&self) -> &SettableData<T, E> {
        &self.settable_data
    }
    fn get_settable_data_mut(&mut self) -> &mut SettableData<T, E> {
        &mut self.settable_data
    }
    fn direct_set(&mut self, value: T) -> Result<(), Error<E>> {
        self.value = value;
        Ok(())
    }
}
impl<T: Clone, E: Copy + Debug> Updatable<E> for ConstantGetter<T, E> {
    ///This does not need to be called.
    fn update(&mut self) -> NothingOrError<E> {
        Ok(())
    }
}
//TODO: test this
///A place where a device can connect to another.
pub struct Terminal<'a, E: Copy + Debug> {
    settable_data_state: SettableData<Datum<State>, E>,
    settable_data_command: SettableData<Datum<Command>, E>,
    other: Option<&'a RefCell<Terminal<'a, E>>>,
}
impl<E: Copy + Debug> Terminal<'_, E> {
    ///Direct constructor for a `Terminal`. You almost always actually want `RefCell<Terminal>`
    ///however, in which case you should call `new`, which returns `RefCell<Terminal>`.
    pub fn new_raw() -> Self {
        Self {
            settable_data_state: SettableData::new(),
            settable_data_command: SettableData::new(),
            other: None,
        }
    }
    ///This constructs a `RefCell<Terminal>`. This is almost always what you want, and what is
    ///needed for connecting terminals. If you do just want a `Terminal`, use `raw_get` instead.
    pub fn new() -> RefCell<Self> {
        RefCell::new(Self::new_raw())
    }
}
impl<E: Copy + Debug> Settable<Datum<State>, E> for Terminal<'_, E> {
    fn get_settable_data_ref(&self) -> &SettableData<Datum<State>, E> {
        &self.settable_data_state
    }
    fn get_settable_data_mut(&mut self) -> &mut SettableData<Datum<State>, E> {
        &mut self.settable_data_state
    }
    //SettableData takes care of this for us.
    fn direct_set(&mut self, _state: Datum<State>) -> NothingOrError<E> {
        Ok(())
    }
}
impl<E: Copy + Debug> Settable<Datum<Command>, E> for Terminal<'_, E> {
    fn get_settable_data_ref(&self) -> &SettableData<Datum<Command>, E> {
        &self.settable_data_command
    }
    fn get_settable_data_mut(&mut self) -> &mut SettableData<Datum<Command>, E> {
        &mut self.settable_data_command
    }
    //SettableData takes care of this for us.
    fn direct_set(&mut self, _command: Datum<Command>) -> NothingOrError<E> {
        Ok(())
    }
}
impl<E: Copy + Debug> Getter<State, E> for Terminal<'_, E> {
    fn get(&self) -> Output<State, E> {
        let mut addends = Vec::with_capacity(2);
        match self.get_last_request() {
            Some(state) => addends.push(state),
            None => (),
        }
        match self.other {
            Some(other) => match other.borrow().get_last_request() {
                Some(state) => addends.push(state),
                None => (),
            },
            None => (),
        }
        match addends.len() {
            0 => return Ok(None),
            1 => return Ok(Some(addends[0])),
            2 => return Ok(Some((addends[0] + addends[1]) / 2.0)),
            _ => unimplemented!(),
        }
    }
}
impl<E: Copy + Debug> Updatable<E> for Terminal<'_, E> {
    fn update(&mut self) -> NothingOrError<E> {
        Ok(())
    }
}
///Connect two terminals. Connected terminals should represent a physical connection between
///mechanical devices.
pub fn connect<'a, E: Copy + Debug>(
    term1: &'a RefCell<Terminal<'a, E>>,
    term2: &'a RefCell<Terminal<'a, E>>,
) {
    term1.borrow_mut().other = Some(term2);
    term2.borrow_mut().other = Some(term1);
}
//TODO; test this
///A mechanical device.
pub trait Device<E: Copy + Debug>: Updatable<E> {
    ///Call only the `update` methods of owned terminals and do not update anything else with the
    ///device.
    fn update_terminals(&mut self) -> NothingOrError<E>;
}
