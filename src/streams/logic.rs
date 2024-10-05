// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2024 UxuginPython
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
        match self {
            AndState::ReturnableTrue => *self = AndState::MaybeTrue,
            _ => (),
        }
    }
}
///Performs an and operation on two boolean getters. This will return `None` if it can't verify
///that the result should be `true` or `false`. This is caused by inputs returning `None`. It's a
///bit difficult to state exactly how this is determined, so here's a truth table:
///```text
///Input 1 | Input 2 | AndStream
///--------+---------+----------
///false   | false   | false
///None    | false   | false
///true    | false   | false
///false   | None    | false
///None    | None    | None
///true    | None    | None
///false   | true    | false
///None    | true    | None
///true    | true    | true
///```
pub struct AndStream<G1: Getter<bool, E>, G2: Getter<bool, E>, E: Copy + Debug> {
    input1: Reference<G1>,
    input2: Reference<G2>,
    phantom_e: PhantomData<E>,
}
impl<G1: Getter<bool, E>, G2: Getter<bool, E>, E: Copy + Debug> AndStream<G1, G2, E> {
    ///Constructor for `AndStream`.
    pub const fn new(input1: Reference<G1>, input2: Reference<G2>) -> Self {
        Self {
            input1: input1,
            input2: input2,
            phantom_e: PhantomData,
        }
    }
}
impl<G1: Getter<bool, E>, G2: Getter<bool, E>, E: Copy + Debug> Getter<bool, E>
    for AndStream<G1, G2, E>
{
    fn get(&self) -> Output<bool, E> {
        let gotten1 = self.input1.borrow().get()?;
        let gotten2 = self.input2.borrow().get()?;
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
impl<G1: Getter<bool, E>, G2: Getter<bool, E>, E: Copy + Debug> Updatable<E>
    for AndStream<G1, G2, E>
{
    fn update(&mut self) -> NothingOrError<E> {
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
        match self {
            OrState::ReturnableFalse => *self = OrState::MaybeFalse,
            _ => (),
        }
    }
}
///Performs an or operation on two boolean getters. This will return `None` if it can't verify that
///the result should be `true` or `false`.
///```text
///Input 1 | Input 2 | OrStream
///--------+---------+---------
///false   | false   | false
///None    | false   | None
///true    | false   | true
///false   | None    | None
///None    | None    | None
///true    | None    | true
///false   | true    | true
///None    | true    | true
///true    | true    | true
///```
pub struct OrStream<G1: Getter<bool, E>, G2: Getter<bool, E>, E: Copy + Debug> {
    input1: Reference<G1>,
    input2: Reference<G2>,
    phantom_e: PhantomData<E>,
}
impl<G1: Getter<bool, E>, G2: Getter<bool, E>, E: Copy + Debug> OrStream<G1, G2, E> {
    ///Constructor for `OrStream`.
    pub const fn new(input1: Reference<G1>, input2: Reference<G2>) -> Self {
        Self {
            input1: input1,
            input2: input2,
            phantom_e: PhantomData,
        }
    }
}
impl<G1: Getter<bool, E>, G2: Getter<bool, E>, E: Copy + Debug> Getter<bool, E>
    for OrStream<G1, G2, E>
{
    fn get(&self) -> Output<bool, E> {
        let gotten1 = self.input1.borrow().get()?;
        let gotten2 = self.input2.borrow().get()?;
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
impl<G1: Getter<bool, E>, G2: Getter<bool, E>, E: Copy + Debug> Updatable<E>
    for OrStream<G1, G2, E>
{
    fn update(&mut self) -> NothingOrError<E> {
        Ok(())
    }
}
///Performs a not operation on a boolean getter.
pub struct NotStream<G: Getter<bool, E>, E: Copy + Debug> {
    input: Reference<G>,
    phantom_e: PhantomData<E>,
}
impl<G: Getter<bool, E>, E: Copy + Debug> NotStream<G, E> {
    ///Constructor for `NotStream`.
    pub const fn new(input: Reference<G>) -> Self {
        Self {
            input: input,
            phantom_e: PhantomData,
        }
    }
}
impl<G: Getter<bool, E>, E: Copy + Debug> Getter<bool, E> for NotStream<G, E> {
    fn get(&self) -> Output<bool, E> {
        match self.input.borrow().get() {
            Ok(Some(datum)) => Ok(Some(!datum)),
            Ok(None) => Ok(None),
            Err(error) => Err(error),
        }
    }
}
impl<G: Getter<bool, E>, E: Copy + Debug> Updatable<E> for NotStream<G, E> {
    fn update(&mut self) -> NothingOrError<E> {
        Ok(())
    }
}
