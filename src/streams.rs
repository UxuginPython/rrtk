// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2024 UxuginPython
//!Getters that do data processing and have other getters as inputs are called streams. These are
//!some helpful builtin streams for controlling your robot. See the `pid` example to learn more
//!about how to use the stream system.
use crate::*;
#[cfg(not(feature = "std"))]
use alloc::collections::vec_deque::VecDeque;
#[cfg(feature = "std")]
use std::collections::vec_deque::VecDeque;
pub mod control;
pub mod converters;
pub mod flow;
pub mod logic;
pub mod math;
///Returns the output of whichever input has the latest time.
pub struct Latest<T, const C: usize, E: Copy + Debug> {
    inputs: [InputGetter<T, E>; C],
}
impl<T, const C: usize, E: Copy + Debug> Latest<T, C, E> {
    ///Constructor for `Latest`.
    pub fn new(inputs: [InputGetter<T, E>; C]) -> Self {
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
pub struct Expirer<T, E: Copy + Debug> {
    input: InputGetter<T, E>,
    time_getter: InputTimeGetter<E>,
    max_time_delta: i64,
}
impl<T, E: Copy + Debug> Expirer<T, E> {
    ///Constructor for `Expirer`.
    pub fn new(
        input: InputGetter<T, E>,
        time_getter: InputTimeGetter<E>,
        max_time_delta: i64,
    ) -> Self {
        Self {
            input: input,
            time_getter: time_getter,
            max_time_delta: max_time_delta,
        }
    }
}
impl<T, E: Copy + Debug> Getter<T, E> for Expirer<T, E> {
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
impl<T, E: Copy + Debug> Updatable<E> for Expirer<T, E> {
    fn update(&mut self) -> NothingOrError<E> {
        Ok(())
    }
}
