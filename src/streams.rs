// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2024-2026 UxuginPython
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
///Expires data that are too old to be useful.
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
