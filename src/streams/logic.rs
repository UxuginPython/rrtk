// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2024-2025 UxuginPython
//!Logic operations for boolean getters.
use crate::streams::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum LogicState {
    ReturnableFalse,
    NeitherReturnable,
    ReturnableTrue,
}
impl LogicState {
    #[inline]
    fn not_returnable_true(&mut self) {
        if let Self::ReturnableTrue = self {
            *self = Self::NeitherReturnable;
        }
    }
    #[inline]
    fn not_returnable_false(&mut self) {
        if let Self::ReturnableFalse = self {
            *self = Self::NeitherReturnable;
        }
    }
}
pub struct GoodAndStream<const N: usize, G: Getter<bool, E>, E: Clone + Debug> {
    inputs: [G; N],
    phantom_e: PhantomData<E>,
}
impl<const N: usize, G: Getter<bool, E>, E: Clone + Debug> GoodAndStream<N, G, E> {
    pub const fn new(inputs: [G; N]) -> Self {
        Self {
            inputs,
            phantom_e: PhantomData,
        }
    }
}
impl<const N: usize, G: Getter<bool, E>, E: Clone + Debug> Updatable<E> for GoodAndStream<N, G, E> {
    fn update(&mut self) -> NothingOrError<E> {
        for getter in &mut self.inputs {
            getter.update()?;
        }
        Ok(())
    }
}
impl<const N: usize, G: Getter<bool, E>, E: Clone + Debug> Getter<bool, E>
    for GoodAndStream<N, G, E>
{
    fn get(&self) -> Output<bool, E> {
        let mut logic_state = LogicState::ReturnableTrue;
        let mut time = Time::ZERO;
        for getter in &self.inputs {
            match getter.get()? {
                None => logic_state.not_returnable_true(),
                Some(datum) => {
                    if datum.time > time {
                        time = datum.time;
                    }
                    if !datum.value {
                        logic_state = LogicState::ReturnableFalse;
                    }
                }
            }
        }
        match logic_state {
            LogicState::ReturnableTrue => Ok(Some(Datum::new(time, true))),
            LogicState::ReturnableFalse => Ok(Some(Datum::new(time, false))),
            LogicState::NeitherReturnable => Ok(None),
        }
    }
}
pub struct GoodOrStream<const N: usize, G: Getter<bool, E>, E: Clone + Debug> {
    inputs: [G; N],
    phantom_e: PhantomData<E>,
}
impl<const N: usize, G: Getter<bool, E>, E: Clone + Debug> GoodOrStream<N, G, E> {
    pub const fn new(inputs: [G; N]) -> Self {
        Self {
            inputs,
            phantom_e: PhantomData,
        }
    }
}
impl<const N: usize, G: Getter<bool, E>, E: Clone + Debug> Updatable<E> for GoodOrStream<N, G, E> {
    fn update(&mut self) -> NothingOrError<E> {
        for getter in &mut self.inputs {
            getter.update()?;
        }
        Ok(())
    }
}
impl<const N: usize, G: Getter<bool, E>, E: Clone + Debug> Getter<bool, E>
    for GoodOrStream<N, G, E>
{
    fn get(&self) -> Output<bool, E> {
        let mut logic_state = LogicState::ReturnableFalse;
        let mut time = Time::ZERO;
        for getter in &self.inputs {
            match getter.get()? {
                None => logic_state.not_returnable_false(),
                Some(datum) => {
                    if datum.time > time {
                        time = datum.time;
                    }
                    if datum.value {
                        logic_state = LogicState::ReturnableTrue;
                    }
                }
            }
        }
        match logic_state {
            LogicState::ReturnableTrue => Ok(Some(Datum::new(time, true))),
            LogicState::ReturnableFalse => Ok(Some(Datum::new(time, false))),
            LogicState::NeitherReturnable => Ok(None),
        }
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
