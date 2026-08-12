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
//!let five = ConstantGetter::<u8, Time, ()>::new(Time::ZERO, 5);
//!let three = ConstantGetter::<u8, Time, ()>::new(Time::ZERO, 3);
//!let one = ConstantGetter::<u8, Time, ()>::new(Time::ZERO, 1);
//!let mul_by_3 = streams::math::Product2::new(five, three);
//!let add_1 = streams::math::Sum2::new(mul_by_3, one);
//!assert_eq!(add_1.get().unwrap().unwrap().value, 16);
//!```
//!See the "pid" example for a more complex demonstration of the stream system.
use crate::*;
pub mod control;
pub mod converters;
pub mod flow;
pub mod logic;
pub mod math;
///Returns the output of whichever input has the latest timestamp.
pub struct Latest<const C: usize, G> {
    inputs: [G; C],
}
impl<const C: usize, G> Latest<C, G> {
    ///Constructor for [`Latest`].
    pub const fn new(inputs: [G; C]) -> Self {
        Self { inputs }
    }
}
impl<T, const C: usize, G: Getter<T, E>, E: Clone + Debug> Getter<T, E> for Latest<C, G> {
    fn get(&self) -> Output<T, E> {
        let mut output: Option<Datum<T>> = None;
        for getter in &self.inputs {
            let gotten = getter.get();
            if let Ok(Some(gotten)) = gotten {
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
///Expires data that are too old to be useful. Timestamps of data from the input `Getter` are
///compared to the current time sourced from the `TimeGetter`, and `Ok(None)` is returned instead
///of the datum if the difference exceeds `max_time_delta`.
pub struct Expirer<T, G, TG, E>
where
    G: Getter<T, E>,
    TG: TimeGetter<E>,
    E: Clone + Debug,
{
    input: G,
    time_getter: TG,
    max_time_delta: Time,
    phantom_t: PhantomData<T>,
    phantom_e: PhantomData<E>,
}
impl<T, G, TG, E> Expirer<T, G, TG, E>
where
    G: Getter<T, E>,
    TG: TimeGetter<E>,
    E: Clone + Debug,
{
    ///Constructor for [`Expirer`].
    pub const fn new(input: G, time_getter: TG, max_time_delta: Time) -> Self {
        Self {
            input,
            time_getter,
            max_time_delta,
            phantom_t: PhantomData,
            phantom_e: PhantomData,
        }
    }
}
impl<T, G, TG, E> Getter<T, E> for Expirer<T, G, TG, E>
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
impl<T, G, TG, E> Updatable<E> for Expirer<T, G, TG, E>
where
    G: Getter<T, E>,
    TG: TimeGetter<E>,
    E: Clone + Debug,
{
    fn update(&mut self) -> NothingOrError<E> {
        self.time_getter.update()?;
        self.input.update()?;
        Ok(())
    }
}
