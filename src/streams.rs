// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2024-2026 UxuginPython
//!There are some special [`Getter`]s that hold other `Getter`s that they use as input for some
//!form of data processing. These are called *streams*.
//!
//!Streams are designed to be chained together for more complex operations. For example, to
//!multiply 5 by 3 and add 1, one could do this (of course, normally not using all
//!`ConstantGetter`s and dummy `TimeGetter`s):
//!```
//!use rrtk::*;
//!let five = ConstantGetter::<u8, Time>::new(Time::ZERO, 5);
//!let three = ConstantGetter::<u8, Time>::new(Time::ZERO, 3);
//!let one = ConstantGetter::<u8, Time>::new(Time::ZERO, 1);
//!let mul_by_3 = streams::math::Product2::new(five, three);
//!let add_1 = streams::math::Sum2::new(mul_by_3, one);
//!//Fully qualified syntax to avoid error handling
//!assert_eq!(<_ as Getter<_, ()>>::get(&add_1).unwrap().unwrap().value, 16);
//!```
//!See the "pid" example for a more complex demonstration of the stream system.
use crate::*;
pub mod control;
pub mod converters;
pub mod flow;
pub mod logic;
pub mod math;
///Returns the output of whichever input has the latest timestamp. This is almost identical to
///[`Latest2`] except that it can hold an arbitrary number of inputs and those inputs need to be of
///the same type.
///`Latest2` should be marginally faster than `Latest` if you only need 2 inputs.
pub struct Latest<const C: usize, G> {
    inputs: [G; C],
}
impl<const C: usize, G> Latest<C, G> {
    ///Constructor for [`Latest`].
    #[inline]
    pub const fn new(inputs: [G; C]) -> Self {
        Self { inputs }
    }
}
impl<T, const C: usize, G: Getter<T, E>, E: Clone + Debug> Getter<T, E> for Latest<C, G> {
    fn get(&self) -> Output<T, E> {
        let mut output: Option<Datum<T>> = None;
        for getter in &self.inputs {
            let gotten = getter.get()?;
            if let Some(gotten) = gotten {
                match &output {
                    Some(thing) => {
                        if gotten.time > thing.time {
                            output = Some(gotten);
                        }
                    }
                    None => {
                        output = Some(gotten);
                    }
                }
            }
        }
        Ok(output)
    }
}
impl<const C: usize, G: Updatable<E>, E: Clone + Debug> Updatable<E> for Latest<C, G> {
    fn update(&mut self) -> NothingOrError<E> {
        for getter in &mut self.inputs {
            getter.update()?;
        }
        Ok(())
    }
}
///Returns the output of whichever input has the latest timestamp. This is almost identical to
///[`Latest`] except that it can only hold 2 inputs and those inputs can be of different types.
///`Latest2` should be marginally faster than `Latest` if you only need 2 inputs.
pub struct Latest2<G1, G2> {
    input1: G1,
    input2: G2,
}
impl<G1, G2> Latest2<G1, G2> {
    ///Constructor for `Latest2`.
    #[inline]
    pub const fn new(input1: G1, input2: G2) -> Self {
        Self { input1, input2 }
    }
}
impl<G1: Updatable<E>, G2: Updatable<E>, E: Clone + Debug> Updatable<E> for Latest2<G1, G2> {
    fn update(&mut self) -> NothingOrError<E> {
        self.input1.update()?;
        self.input2.update()
    }
}
impl<T, G1: Getter<T, E>, G2: Getter<T, E>, E: Clone + Debug> Getter<T, E> for Latest2<G1, G2> {
    fn get(&self) -> Output<T, E> {
        let gotten_from_1 = match self.input1.get() {
            Err(error) => return Err(error),
            Ok(None) => return self.input2.get(),
            Ok(Some(datum)) => datum,
        };
        match self.input2.get() {
            Err(error) => Err(error),
            Ok(None) => Ok(Some(gotten_from_1)),
            Ok(Some(gotten_from_2)) => Ok(Some(latest(gotten_from_1, gotten_from_2))),
        }
    }
}
///Expires data that are too old to be useful. Timestamps of data from the input `Getter` are
///compared to the current time sourced from the `TimeGetter`, and `Ok(None)` is returned instead
///of the datum if the difference exceeds `max_time_delta`.
pub struct Expirer<G, TG> {
    input: G,
    time_getter: TG,
    max_time_delta: Time,
}
impl<G, TG> Expirer<G, TG> {
    ///Constructor for [`Expirer`].
    pub const fn new(input: G, time_getter: TG, max_time_delta: Time) -> Self {
        Self {
            input,
            time_getter,
            max_time_delta,
        }
    }
}
impl<T, G, TG, E> Getter<T, E> for Expirer<G, TG>
where
    G: Getter<T, E>,
    TG: TimeGetter<E>,
    E: Clone + Debug,
{
    fn get(&self) -> Output<T, E> {
        let output = match self.input.get()? {
            Some(datum) => datum,
            None => return Ok(None),
        };
        let time = self.time_getter.get()?;
        if time - output.time > self.max_time_delta {
            return Ok(None);
        }
        Ok(Some(output))
    }
}
impl<G, TG, E> Updatable<E> for Expirer<G, TG>
where
    G: Updatable<E>,
    TG: Updatable<E>,
    E: Clone + Debug,
{
    fn update(&mut self) -> NothingOrError<E> {
        self.time_getter.update()?;
        self.input.update()?;
        Ok(())
    }
}
///A stream for printing debug information upon `get`, `set`, and `update` calls. Except for that,
///it is transparent, passing all calls to those methods directly to its input.
#[cfg(feature = "std")]
pub struct DebugStream<G> {
    input: G,
    silent: bool,
    name: Option<String>,
}
#[cfg(feature = "std")]
impl<G> DebugStream<G> {
    ///Constructor for `DebugStream`.
    #[inline]
    pub const fn new(input: G) -> Self {
        Self {
            input,
            silent: false,
            name: None,
        }
    }
    ///Construct `DebugStream` with a name that will be prepended to all debug messages.
    #[inline]
    pub const fn with_name(input: G, name: String) -> Self {
        Self {
            input,
            silent: false,
            name: Some(name),
        }
    }
    ///Enable or disable the debug printing done by `DebugStream`.
    ///
    ///Set `true` to disable debug printing and `false` to reenable it. Debug printing is **on** by
    ///default.
    #[inline]
    pub const fn set_silent(&mut self, silent: bool) {
        self.silent = silent;
    }
    ///Change, add, or remove the name that is prepended to all debug messages.
    #[inline]
    pub fn change_name(&mut self, name: Option<String>) {
        self.name = name;
    }
    fn to_prepend(&self) -> String {
        if let Some(name) = &self.name {
            format!("{}: ", name)
        } else {
            String::new()
        }
    }
}
#[cfg(feature = "std")]
impl<G: Updatable<E>, E: Clone + Debug> Updatable<E> for DebugStream<G> {
    fn update(&mut self) -> NothingOrError<E> {
        if self.silent {
            self.input.update()
        } else {
            let update = self.input.update();
            eprintln!("{}rrtk::Updatable::update: {:?}", self.to_prepend(), update);
            update
        }
    }
}
#[cfg(feature = "std")]
impl<T: Debug, G: Getter<T, E>, E: Clone + Debug> Getter<T, E> for DebugStream<G> {
    fn get(&self) -> Output<T, E> {
        if self.silent {
            self.input.get()
        } else {
            let get = self.input.get();
            eprintln!("{}rrtk::Getter::get: {:?}", self.to_prepend(), get);
            get
        }
    }
}
#[cfg(feature = "std")]
impl<T: Debug, G: Settable<T, E>, E: Clone + Debug> Settable<T, E> for DebugStream<G> {
    fn set(&mut self, value: T) -> NothingOrError<E> {
        if self.silent {
            self.input.set(value)
        } else {
            let value_string = format!("{:?}", value);
            let set = self.input.set(value);
            eprintln!(
                r#"{}rrtk::Settable::set: setting to "{}" returned "{:?}""#,
                self.to_prepend(),
                value_string,
                set
            );
            set
        }
    }
}
#[cfg(feature = "std")]
impl<G: TimeGetter<E>, E: Clone + Debug> TimeGetter<E> for DebugStream<G> {
    fn get(&self) -> TimeOutput<E> {
        if self.silent {
            self.input.get()
        } else {
            let get = self.input.get();
            eprintln!("{}rrtk::TimeGetter::get: {:?}", self.to_prepend(), get);
            get
        }
    }
}
#[cfg(feature = "std")]
impl<T: Debug, G: Chronology<T>> Chronology<T> for DebugStream<G> {
    fn get(&self, time: Time) -> Option<Datum<T>> {
        if self.silent {
            self.input.get(time)
        } else {
            let get = self.input.get(time);
            eprintln!("{}rrtk::Chronology::get: {:?}", self.to_prepend(), get);
            get
        }
    }
}
#[cfg(all(feature = "std", feature = "devices"))]
impl<G: devices::DeviceUpdatable<E>, E: Debug> devices::DeviceUpdatable<E> for DebugStream<G> {
    fn device_update<const N: usize>(
        &mut self,
        system: &mut devices::System<N>,
    ) -> NothingOrError<E> {
        if self.silent {
            self.input.device_update(system)
        } else {
            let device_update = self.input.device_update(system);
            eprintln!(
                "{}rrtk::devices::DeviceUpdatable::device_update: {:?}",
                self.to_prepend(),
                device_update
            );
            device_update
        }
    }
}
