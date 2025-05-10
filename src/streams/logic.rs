// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2024-2025 UxuginPython
//!Logic operations for boolean getters.
use crate::streams::*;
//TODO: make these take arrays of inputs with generic lengths.
enum AndState {
    DefinitelyFalse, //An input returned false.
    MaybeTrue,       //An input returned None and no input has returned false, so we can't assume an
    //output.
    ReturnableTrue, //No input has returned None or false.
}
impl AndState {
    #[inline]
    fn none(&mut self) {
        if let AndState::ReturnableTrue = self {
            *self = AndState::MaybeTrue
        }
    }
}
///Performs an and operation on two boolean getters. This will return [`None`] if it can't verify
///that the result should be [`true`] or [`false`]. This is caused by inputs returning [`None`]. It's a
///bit difficult to state exactly how this is determined, so here's a truth table:
///| Input 1         | Input 2         | [`AndStream`]   |
///|-----------------|-----------------|-----------------|
///| [`Some(false)`] | [`Some(false)`] | [`Some(false)`] |
///| [`None`]        | [`Some(false)`] | [`Some(false)`] |
///| [`Some(true)`]  | [`Some(false)`] | [`Some(false)`] |
///| [`Some(false)`] | [`None`]        | [`Some(false)`] |
///| [`None`]        | [`None`]        | [`None`]        |
///| [`Some(true)`]  | [`None`]        | [`None`]        |
///| [`Some(false)`] | [`Some(true)`]  | [`Some(false)`] |
///| [`None`]        | [`Some(true)`]  | [`None`]        |
///| [`Some(true)`]  | [`Some(true)`]  | [`Some(true)`]  |
pub struct AndStream<G1, G2, E>
where
    G1: Getter<bool, E>,
    G2: Getter<bool, E>,
    E: Clone + Debug,
{
    input1: G1,
    input2: G2,
    phantom_e: PhantomData<E>,
}
impl<G1, G2, E> AndStream<G1, G2, E>
where
    G1: Getter<bool, E>,
    G2: Getter<bool, E>,
    E: Clone + Debug,
{
    ///Constructor for [`AndStream`].
    pub const fn new(input1: G1, input2: G2) -> Self {
        Self {
            input1,
            input2,
            phantom_e: PhantomData,
        }
    }
}
impl<G1, G2, E> Getter<bool, E> for AndStream<G1, G2, E>
where
    G1: Getter<bool, E>,
    G2: Getter<bool, E>,
    E: Clone + Debug,
{
    fn get(&self) -> Output<bool, E> {
        let gotten1 = self.input1.get()?;
        let gotten2 = self.input2.get()?;
        //Never assume the boolean value of a None from an input:
        //To return true, we require that both inputs return true (not None).
        //To return false, we require that at least one input returns false (not None).
        //If neither of these is met, return None.
        let mut time = None;
        let mut and_state = AndState::ReturnableTrue;
        match gotten1 {
            Some(datum) => {
                time = Some(datum.time);
                if !datum.value {
                    and_state = AndState::DefinitelyFalse;
                }
            }
            None => {
                and_state.none();
            }
        }
        match gotten2 {
            Some(datum) => {
                match time {
                    Some(existing) => {
                        if datum.time > existing {
                            time = Some(datum.time);
                        }
                    }
                    None => time = Some(datum.time),
                }
                if !datum.value {
                    and_state = AndState::DefinitelyFalse;
                }
            }
            None => {
                and_state.none();
            }
        }
        let time = match time {
            Some(time) => time,
            None => return Ok(None),
        };
        match and_state {
            AndState::DefinitelyFalse => Ok(Some(Datum::new(time, false))),
            AndState::MaybeTrue => Ok(None),
            AndState::ReturnableTrue => Ok(Some(Datum::new(time, true))),
        }
    }
}
impl<G1, G2, E> Updatable<E> for AndStream<G1, G2, E>
where
    G1: Getter<bool, E>,
    G2: Getter<bool, E>,
    E: Clone + Debug,
{
    fn update(&mut self) -> NothingOrError<E> {
        self.input1.update()?;
        self.input2.update()?;
        Ok(())
    }
}
enum OrState {
    DefinitelyTrue, //An input returned true.
    MaybeFalse,     //An input returned None and no input has returned true, so we can't assume an
    //output.
    ReturnableFalse, //No input has returned None or true.
}
impl OrState {
    #[inline]
    fn none(&mut self) {
        if let OrState::ReturnableFalse = self {
            *self = OrState::MaybeFalse
        }
    }
}
///Performs an or operation on two boolean getters. This will return [`None`] if it can't verify that
///the result should be [`true`] or [`false`].
///| Input 1         | Input 2       | [`OrStream`]      |
///|-----------------|---------------|-------------------|
///| [`Some(false)`] | [`Some(false)`] | [`Some(false)`] |
///| [`None`]        | [`Some(false)`] | [`None`]        |
///| [`Some(true)`]  | [`Some(false)`] | [`Some(true)`]  |
///| [`Some(false)`] | [`None`]        | [`None`]        |
///| [`None`]        | [`None`]        | [`None`]        |
///| [`Some(true)`]  | [`None`]        | [`Some(true)`]  |
///| [`Some(false)`] | [`Some(true)`]  | [`Some(true)`]  |
///| [`None`]        | [`Some(true)`]  | [`Some(true)`]  |
///| [`Some(true)`]  | [`Some(true)`]  | [`Some(true)`]  |
pub struct OrStream<G1, G2, E>
where
    G1: Getter<bool, E>,
    G2: Getter<bool, E>,
    E: Clone + Debug,
{
    input1: G1,
    input2: G2,
    phantom_e: PhantomData<E>,
}
impl<G1, G2, E> OrStream<G1, G2, E>
where
    G1: Getter<bool, E>,
    G2: Getter<bool, E>,
    E: Clone + Debug,
{
    ///Constructor for [`OrStream`].
    pub const fn new(input1: G1, input2: G2) -> Self {
        Self {
            input1,
            input2,
            phantom_e: PhantomData,
        }
    }
}
impl<G1, G2, E> Getter<bool, E> for OrStream<G1, G2, E>
where
    G1: Getter<bool, E>,
    G2: Getter<bool, E>,
    E: Clone + Debug,
{
    fn get(&self) -> Output<bool, E> {
        let gotten1 = self.input1.get()?;
        let gotten2 = self.input2.get()?;
        let mut time = None;
        let mut or_state = OrState::ReturnableFalse;
        match gotten1 {
            Some(datum) => {
                time = Some(datum.time);
                if datum.value {
                    or_state = OrState::DefinitelyTrue;
                }
            }
            None => {
                or_state.none();
            }
        }
        match gotten2 {
            Some(datum) => {
                match time {
                    Some(existing) => {
                        if datum.time > existing {
                            time = Some(datum.time);
                        }
                    }
                    None => time = Some(datum.time),
                }
                if datum.value {
                    or_state = OrState::DefinitelyTrue;
                }
            }
            None => {
                or_state.none();
            }
        }
        let time = match time {
            Some(time) => time,
            None => return Ok(None),
        };
        match or_state {
            OrState::DefinitelyTrue => Ok(Some(Datum::new(time, true))),
            OrState::MaybeFalse => Ok(None),
            OrState::ReturnableFalse => Ok(Some(Datum::new(time, false))),
        }
    }
}
impl<G1, G2, E> Updatable<E> for OrStream<G1, G2, E>
where
    G1: Getter<bool, E>,
    G2: Getter<bool, E>,
    E: Clone + Debug,
{
    fn update(&mut self) -> NothingOrError<E> {
        self.input1.update()?;
        self.input2.update()?;
        Ok(())
    }
}
///Performs a not operation on a boolean getter.
pub struct NotStream<G: Getter<bool, E>, E: Clone + Debug> {
    input: G,
    phantom_e: PhantomData<E>,
}
impl<G: Getter<bool, E>, E: Clone + Debug> NotStream<G, E> {
    ///Constructor for [`NotStream`].
    pub const fn new(input: G) -> Self {
        Self {
            input,
            phantom_e: PhantomData,
        }
    }
}
impl<G: Getter<bool, E>, E: Clone + Debug> Getter<bool, E> for NotStream<G, E> {
    fn get(&self) -> Output<bool, E> {
        match self.input.get() {
            Ok(Some(datum)) => Ok(Some(!datum)),
            Ok(None) => Ok(None),
            Err(error) => Err(error),
        }
    }
}
impl<G: Getter<bool, E>, E: Clone + Debug> Updatable<E> for NotStream<G, E> {
    fn update(&mut self) -> NothingOrError<E> {
        self.input.update()?;
        Ok(())
    }
}
