// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2024-2026 UxuginPython
//!Streams for control flow.
use crate::streams::*;
///Propagates its input if a `Getter<bool, _>` returns `Ok(Some(true))`, otherwise returns
///`Ok(None)`.
pub struct IfStream<T, GC, GI, E>
where
    GC: Getter<bool, E>,
    GI: Getter<T, E>,
    E: Clone + Debug,
{
    condition: GC,
    input: GI,
    phantom_t: PhantomData<T>,
    phantom_e: PhantomData<E>,
}
impl<T, GC, GI, E> IfStream<T, GC, GI, E>
where
    GC: Getter<bool, E>,
    GI: Getter<T, E>,
    E: Clone + Debug,
{
    ///Constructor for [`IfStream`].
    pub const fn new(condition: GC, input: GI) -> Self {
        Self {
            condition,
            input,
            phantom_t: PhantomData,
            phantom_e: PhantomData,
        }
    }
}
impl<T, GC, GI, E> Getter<T, E> for IfStream<T, GC, GI, E>
where
    GC: Getter<bool, E>,
    GI: Getter<T, E>,
    E: Clone + Debug,
{
    fn get(&self) -> Output<T, E> {
        let condition = match self.condition.get()? {
            Some(output) => output.value,
            None => false,
        };
        if condition {
            self.input.get()
        } else {
            Ok(None)
        }
    }
}
impl<T, GC, GI, E> Updatable<E> for IfStream<T, GC, GI, E>
where
    GC: Getter<bool, E>,
    GI: Getter<T, E>,
    E: Clone + Debug,
{
    fn update(&mut self) -> NothingOrError<E> {
        self.input.update()?;
        self.condition.update()?;
        Ok(())
    }
}
///Returns the output of one input if a `Getter<bool, _>` returns `Ok(Some(true))` and another if
///it returns `Ok(Some(false))`. Returns `Ok(None)` if the `Getter<bool, _>` does.
pub struct IfElseStream<T, GC, GT, GF, E>
where
    GC: Getter<bool, E>,
    GT: Getter<T, E>,
    GF: Getter<T, E>,
    E: Clone + Debug,
{
    condition: GC,
    true_output: GT,
    false_output: GF,
    phantom_t: PhantomData<T>,
    phantom_e: PhantomData<E>,
}
impl<T, GC: Getter<bool, E>, GT: Getter<T, E>, GF: Getter<T, E>, E: Clone + Debug>
    IfElseStream<T, GC, GT, GF, E>
{
    ///Constructor for [`IfElseStream`].
    pub const fn new(condition: GC, true_output: GT, false_output: GF) -> Self {
        Self {
            condition,
            true_output,
            false_output,
            phantom_t: PhantomData,
            phantom_e: PhantomData,
        }
    }
}
impl<T, GC, GT, GF, E> Getter<T, E> for IfElseStream<T, GC, GT, GF, E>
where
    GC: Getter<bool, E>,
    GT: Getter<T, E>,
    GF: Getter<T, E>,
    E: Clone + Debug,
{
    fn get(&self) -> Output<T, E> {
        let condition = match self.condition.get()? {
            Some(output) => output.value,
            None => return Ok(None),
        };
        if condition {
            self.true_output.get()
        } else {
            self.false_output.get()
        }
    }
}
impl<T, GC, GT, GF, E> Updatable<E> for IfElseStream<T, GC, GT, GF, E>
where
    GC: Getter<bool, E>,
    GT: Getter<T, E>,
    GF: Getter<T, E>,
    E: Clone + Debug,
{
    fn update(&mut self) -> NothingOrError<E> {
        self.true_output.update()?;
        self.false_output.update()?;
        self.condition.update()?;
        Ok(())
    }
}
///"Freezes" the output of a `Getter` (the "input getter") based on whether a second, boolean
///`Getter` (the "condition getter") is returning true.
///
///Both `Getter`s are updated regardless of whether or not the stream is frozen.
///
///- If the boolean getter returns `Err(_)` or `Ok(None)`, that value is returned instead of that
///of the input getter.
///- If the boolean getter returns true, the last value that was being returned before the boolean
///getter was returning true is maintained. This is the frozen state.
///- If the boolean getter returns false, the value of the input getter is returned. This is the
///unfrozen state.
pub struct FreezeStream<T, GC, GI, E>
where
    T: Clone,
    GC: Getter<bool, E>,
    GI: Getter<T, E>,
    E: Clone + Debug,
{
    condition: GC,
    input: GI,
    freeze_value: Output<T, E>,
}
impl<T, GC, GI, E> FreezeStream<T, GC, GI, E>
where
    T: Clone,
    GC: Getter<bool, E>,
    GI: Getter<T, E>,
    E: Clone + Debug,
{
    ///Constructor for [`FreezeStream`].
    pub const fn new(condition: GC, input: GI) -> Self {
        Self {
            condition,
            input,
            freeze_value: Ok(None),
        }
    }
}
impl<T, GC, GI, E> Getter<T, E> for FreezeStream<T, GC, GI, E>
where
    T: Clone,
    GC: Getter<bool, E>,
    GI: Getter<T, E>,
    E: Clone + Debug,
{
    fn get(&self) -> Output<T, E> {
        self.freeze_value.clone()
    }
}
impl<T, GC, GI, E> Updatable<E> for FreezeStream<T, GC, GI, E>
where
    T: Clone,
    GC: Getter<bool, E>,
    GI: Getter<T, E>,
    E: Clone + Debug,
{
    fn update(&mut self) -> NothingOrError<E> {
        self.input.update()?;
        self.condition.update()?;
        let condition = match self.condition.get() {
            Err(error) => {
                //XXX: This may change when you standardize when Updatable::update errors.
                //Remove this clone if you don't return the error.
                self.freeze_value = Err(error.clone());
                return Err(error);
            }
            Ok(None) => {
                self.freeze_value = Ok(None);
                return Ok(());
            }
            Ok(Some(condition)) => condition.value,
        };
        if !condition {
            let gotten = self.input.get();
            self.freeze_value = gotten.clone();
            match gotten {
                Ok(_) => {}
                Err(error) => return Err(error),
            }
        }
        Ok(())
    }
}
