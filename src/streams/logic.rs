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
pub struct AndStream<const N: usize, G: Getter<bool, E>, E: Clone + Debug> {
    inputs: [G; N],
    phantom_e: PhantomData<E>,
}
impl<const N: usize, G: Getter<bool, E>, E: Clone + Debug> AndStream<N, G, E> {
    pub const fn new(inputs: [G; N]) -> Self {
        Self {
            inputs,
            phantom_e: PhantomData,
        }
    }
}
impl<const N: usize, G: Getter<bool, E>, E: Clone + Debug> Updatable<E> for AndStream<N, G, E> {
    fn update(&mut self) -> NothingOrError<E> {
        for getter in &mut self.inputs {
            getter.update()?;
        }
        Ok(())
    }
}
impl<const N: usize, G: Getter<bool, E>, E: Clone + Debug> Getter<bool, E> for AndStream<N, G, E> {
    //FIXME: Define what happens with 0 inputs.
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
        Ok(match logic_state {
            LogicState::ReturnableTrue => Some(Datum::new(time, true)),
            LogicState::ReturnableFalse => Some(Datum::new(time, false)),
            LogicState::NeitherReturnable => None,
        })
    }
}
pub struct And2<G1: Getter<bool, E>, G2: Getter<bool, E>, E: Clone + Debug> {
    input1: G1,
    input2: G2,
    phantom_e: PhantomData<E>,
}
impl<G1: Getter<bool, E>, G2: Getter<bool, E>, E: Clone + Debug> And2<G1, G2, E> {
    pub const fn new(input1: G1, input2: G2) -> Self {
        Self {
            input1,
            input2,
            phantom_e: PhantomData,
        }
    }
}
impl<G1: Getter<bool, E>, G2: Getter<bool, E>, E: Clone + Debug> Updatable<E> for And2<G1, G2, E> {
    fn update(&mut self) -> NothingOrError<E> {
        self.input1.update()?;
        self.input2.update()?;
        Ok(())
    }
}
impl<G1: Getter<bool, E>, G2: Getter<bool, E>, E: Clone + Debug> Getter<bool, E>
    for And2<G1, G2, E>
{
    fn get(&self) -> Output<bool, E> {
        let mut logic_state = LogicState::ReturnableTrue;
        let mut time = Time::ZERO;
        //TODO: See if there's a way to repeat less code.
        match self.input1.get()? {
            None => logic_state.not_returnable_true(),
            Some(datum) => {
                time = datum.time;
                if !datum.value {
                    logic_state = LogicState::ReturnableFalse;
                }
            }
        }
        match self.input2.get()? {
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
        Ok(match logic_state {
            LogicState::ReturnableTrue => Some(Datum::new(time, true)),
            LogicState::ReturnableFalse => Some(Datum::new(time, false)),
            LogicState::NeitherReturnable => None,
        })
    }
}
pub struct OrStream<const N: usize, G: Getter<bool, E>, E: Clone + Debug> {
    inputs: [G; N],
    phantom_e: PhantomData<E>,
}
impl<const N: usize, G: Getter<bool, E>, E: Clone + Debug> OrStream<N, G, E> {
    pub const fn new(inputs: [G; N]) -> Self {
        Self {
            inputs,
            phantom_e: PhantomData,
        }
    }
}
impl<const N: usize, G: Getter<bool, E>, E: Clone + Debug> Updatable<E> for OrStream<N, G, E> {
    fn update(&mut self) -> NothingOrError<E> {
        for getter in &mut self.inputs {
            getter.update()?;
        }
        Ok(())
    }
}
impl<const N: usize, G: Getter<bool, E>, E: Clone + Debug> Getter<bool, E> for OrStream<N, G, E> {
    //FIXME: Define what happens with 0 inputs.
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
        Ok(match logic_state {
            LogicState::ReturnableTrue => Some(Datum::new(time, true)),
            LogicState::ReturnableFalse => Some(Datum::new(time, false)),
            LogicState::NeitherReturnable => None,
        })
    }
}
pub struct Or2<G1: Getter<bool, E>, G2: Getter<bool, E>, E: Clone + Debug> {
    input1: G1,
    input2: G2,
    phantom_e: PhantomData<E>,
}
impl<G1: Getter<bool, E>, G2: Getter<bool, E>, E: Clone + Debug> Or2<G1, G2, E> {
    pub const fn new(input1: G1, input2: G2) -> Self {
        Self {
            input1,
            input2,
            phantom_e: PhantomData,
        }
    }
}
impl<G1: Getter<bool, E>, G2: Getter<bool, E>, E: Clone + Debug> Updatable<E> for Or2<G1, G2, E> {
    fn update(&mut self) -> NothingOrError<E> {
        self.input1.update()?;
        self.input2.update()?;
        Ok(())
    }
}
impl<G1: Getter<bool, E>, G2: Getter<bool, E>, E: Clone + Debug> Getter<bool, E>
    for Or2<G1, G2, E>
{
    fn get(&self) -> Output<bool, E> {
        let mut logic_state = LogicState::ReturnableFalse;
        let mut time = Time::ZERO;
        //TODO: See if there's a way to repeat less code.
        match self.input1.get()? {
            None => logic_state.not_returnable_false(),
            Some(datum) => {
                time = datum.time;
                if datum.value {
                    logic_state = LogicState::ReturnableTrue;
                }
            }
        }
        match self.input2.get()? {
            None => logic_state.not_returnable_false(),
            Some(datum) => {
                if datum.time > time {
                    time = datum.time;
                }
                if !datum.value {
                    logic_state = LogicState::ReturnableTrue;
                }
            }
        }
        Ok(match logic_state {
            LogicState::ReturnableTrue => Some(Datum::new(time, true)),
            LogicState::ReturnableFalse => Some(Datum::new(time, false)),
            LogicState::NeitherReturnable => None,
        })
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
        Ok(self.input.get()?.map(|datum| !datum))
    }
}
impl<G: Getter<bool, E>, E: Clone + Debug> Updatable<E> for NotStream<G, E> {
    fn update(&mut self) -> NothingOrError<E> {
        self.input.update()?;
        Ok(())
    }
}
