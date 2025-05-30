// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2024-2025 UxuginPython
//!Getters that do data processing and have other getters as inputs are called streams. These are
//!some helpful builtin streams for controlling your robot. See the `pid` example to learn more
//!about how to use the stream system.
use crate::*;
pub mod control;
pub mod converters;
pub mod flow;
pub mod logic;
pub mod math;
///Returns the output of whichever input has the latest time.
pub struct Latest<T, const C: usize, E: Copy + Debug> {
    inputs: [Reference<dyn Getter<T, E>>; C],
}
impl<T, const C: usize, E: Copy + Debug> Latest<T, C, E> {
    ///Constructor for [`Latest`].
    pub const fn new(inputs: [Reference<dyn Getter<T, E>>; C]) -> Self {
        if C < 1 {
            panic!("rrtk::streams::Latest C must be at least 1.");
        }
        Self { inputs: inputs }
    }
}
impl<T, const C: usize, E: Copy + Debug> Getter<T, E> for Latest<T, C, E> {
    fn get(&self) -> Output<T, E> {
        let mut output: Option<Datum<T>> = None;
        for i in &self.inputs {
            let gotten = i.borrow().get();
            match gotten {
                Ok(Some(gotten)) => match &output {
                    Some(thing) => {
                        if gotten.time > thing.time {
                            output = Some(gotten);
                        }
                    }
                    None => {
                        output = Some(gotten);
                    }
                },
                _ => {}
            }
        }
        Ok(output)
    }
}
impl<T, const C: usize, E: Copy + Debug> Updatable<E> for Latest<T, C, E> {
    fn update(&mut self) -> NothingOrError<E> {
        Ok(())
    }
}
///Expires data that are too old to be useful.
pub struct Expirer<T, G: Getter<T, E> + ?Sized, TG: TimeGetter<E> + ?Sized, E: Copy + Debug> {
    input: Reference<G>,
    time_getter: Reference<TG>,
    max_time_delta: Time,
    phantom_t: PhantomData<T>,
    phantom_e: PhantomData<E>,
}
impl<T, G: Getter<T, E> + ?Sized, TG: TimeGetter<E> + ?Sized, E: Copy + Debug>
    Expirer<T, G, TG, E>
{
    ///Constructor for [`Expirer`].
    pub const fn new(
        input: Reference<G>,
        time_getter: Reference<TG>,
        max_time_delta: Time,
    ) -> Self {
        Self {
            input: input,
            time_getter: time_getter,
            max_time_delta: max_time_delta,
            phantom_t: PhantomData,
            phantom_e: PhantomData,
        }
    }
}
impl<T, G: Getter<T, E> + ?Sized, TG: TimeGetter<E> + ?Sized, E: Copy + Debug> Getter<T, E>
    for Expirer<T, G, TG, E>
{
    fn get(&self) -> Output<T, E> {
        let output = match self.input.borrow().get()? {
            Some(datum) => datum,
            None => return Ok(None),
        };
        let time = self.time_getter.borrow().get()?;
        if time - output.time > self.max_time_delta {
            return Ok(None);
        }
        Ok(Some(output))
    }
}
impl<T, G: Getter<T, E> + ?Sized, TG: TimeGetter<E> + ?Sized, E: Copy + Debug> Updatable<E>
    for Expirer<T, G, TG, E>
{
    fn update(&mut self) -> NothingOrError<E> {
        Ok(())
    }
}
