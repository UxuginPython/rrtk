// SPDX-License-Identifier: LGPL-3.0-only
/*
Copyright 2024 UxuginPython on GitHub

     This file is part of Rust Robotics ToolKit.

    Rust Robotics ToolKit is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, version 3.

    Rust Robotics ToolKit is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Lesser General Public License for more details.

    You should have received a copy of the GNU Lesser General Public License along with Rust Robotics ToolKit. If not, see <https://www.gnu.org/licenses/>.
*/
//!Getters that do data processing and have other getters as inputs are called streams. These are
//!some helpful builtin streams for controlling your robot. See the `pid` example to learn more
//!about how to use the stream system.
use crate::*;
#[cfg(feature = "alloc")]
use alloc::collections::vec_deque::VecDeque;
//pub mod control;
//pub mod converters;
//pub mod flow;
pub mod logic;
//pub mod math;
///Returns the output of whichever input has the latest time.
pub struct Latest<T, const C: usize, E: Copy + Debug> {
    inputs: [Reference<dyn Getter<T, E>>; C],
}
impl<T, const C: usize, E: Copy + Debug> Latest<T, C, E> {
    ///Constructor for `Latest`.
    pub fn new(inputs: [Reference<dyn Getter<T, E>>; C]) -> Self {
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
pub struct Expirer<T, G: Getter<T, E>, TG: TimeGetter<E>, E: Copy + Debug> {
    input: Reference<G>,
    time_getter: Reference<TG>,
    max_time_delta: i64,
    phantom_t: PhantomData<T>,
    phantom_e: PhantomData<E>,
}
impl<T, G: Getter<T, E>, TG: TimeGetter<E>, E: Copy + Debug> Expirer<T, G, TG, E> {
    ///Constructor for `Expirer`.
    pub fn new(input: Reference<G>, time_getter: Reference<TG>, max_time_delta: i64) -> Self {
        Self {
            input: input,
            time_getter: time_getter,
            max_time_delta: max_time_delta,
            phantom_t: PhantomData,
            phantom_e: PhantomData,
        }
    }
}
impl<T, G: Getter<T, E>, TG: TimeGetter<E>, E: Copy + Debug> Getter<T, E> for Expirer<T, G, TG, E> {
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
impl<T, G: Getter<T, E>, TG: TimeGetter<E>, E: Copy + Debug> Updatable<E> for Expirer<T, G, TG, E> {
    fn update(&mut self) -> NothingOrError<E> {
        Ok(())
    }
}
