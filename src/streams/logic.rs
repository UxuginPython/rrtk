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
///Performs a logical "and" operation on an arbitrary number of inputs. More specifically, follows
///these rules, starting at the top and proceeding as needed:
///1. If an input returns an error, return the error.
///2. If no input returns an error, if an input returns false, return false.
///3. If no input returns false, if an input returns None, return None.
///4. If no input returns None (all returned true), return true.
///
///Returns the latest timestamp of any input (if not Err or None).
///
///If you only need two inputs, you should probably use [`And2`] instead, which may be slightly
///faster and allows its inputs to have different types.
pub struct AndStream<const N: usize, G: Getter<bool, E>, E: Clone + Debug> {
    inputs: [G; N],
    phantom_e: PhantomData<E>,
}
impl<const N: usize, G: Getter<bool, E>, E: Clone + Debug> AndStream<N, G, E> {
    ///Constructor for `AndStream`.
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
///Performs a logical "and" operation on two input getters which can be of different types. More
///specifically, follows these rules, starting at the top and proceeding as needed:
///1. If an input returns an error, return the error.
///2. If neither input returns an error, if an input returns false, return false.
///3. If neither input returns false, if an input returns None, return None.
///4. If neither input returns None (both returned true), return true.
///
///Returns the later timestamp of the two inputs if they both return Some.
///
///If you need more than two inputs, you may consider using [`AndStream`] instead of a chain of
///`And2`, especially if the inputs are of the same type.
pub struct And2<G1: Getter<bool, E>, G2: Getter<bool, E>, E: Clone + Debug> {
    input1: G1,
    input2: G2,
    phantom_e: PhantomData<E>,
}
impl<G1: Getter<bool, E>, G2: Getter<bool, E>, E: Clone + Debug> And2<G1, G2, E> {
    ///Constructor for `And2`. Unlike [`AndStream`], its inputs can be of different types.
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
///Performs a logical "or" operation on an arbitrary number of inputs. More specifically, follows
///these rules, starting at the top and proceeding as needed:
///1. If an input returns an error, return the error.
///2. If no input returns an error, if an input returns true, return true.
///3. If no input returns true, if an input returns None, return None.
///4. If no input returns None (all returned false), return false.
///
///Returns the latest timestamp of any input (if not Err or None).
///
///If you only need two inputs, you should probably use [`Or2`] instead, which may be slightly
///faster and allows its inputs to have different types.
pub struct OrStream<const N: usize, G: Getter<bool, E>, E: Clone + Debug> {
    inputs: [G; N],
    phantom_e: PhantomData<E>,
}
impl<const N: usize, G: Getter<bool, E>, E: Clone + Debug> OrStream<N, G, E> {
    ///Constructor for `OrStream`.
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
///Performs a logical "or" operation on two input getters which can be of different types. More
///specifically, follows these rules, starting at the top and proceeding as needed:
///1. If an input returns an error, return the error.
///2. If neither input returns an error, if an input returns true, return true.
///3. If neither input returns true, if an input returns None, return None.
///4. If neither input returns None (both returned false), return false.
///
///Returns the later timestamp of the two inputs if they both return Some.
///
///If you need more than two inputs, you may consider using [`OrStream`] instead of a chain of
///`Or2`, especially if the inputs are of the same type.
pub struct Or2<G1: Getter<bool, E>, G2: Getter<bool, E>, E: Clone + Debug> {
    input1: G1,
    input2: G2,
    phantom_e: PhantomData<E>,
}
impl<G1: Getter<bool, E>, G2: Getter<bool, E>, E: Clone + Debug> Or2<G1, G2, E> {
    ///Constructor for `Or2`. Unlike [`OrStream`], its inputs can be of different types.
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
                if datum.value {
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
