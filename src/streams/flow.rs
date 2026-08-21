// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2024-2026 UxuginPython
//!Streams for control flow.
use crate::streams::*;
///Propagates its input if a `Getter<bool, _>` returns `Ok(Some(true))`, otherwise returns
///`Ok(None)`.
pub struct IfStream<GC, GI> {
    condition: GC,
    input: GI,
}
impl<GC, GI> IfStream<GC, GI> {
    ///Constructor for [`IfStream`].
    pub const fn new(condition: GC, input: GI) -> Self {
        Self { condition, input }
    }
}
impl<T, GC, GI, E> Getter<T, E> for IfStream<GC, GI>
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
impl<GC, GI, E> Updatable<E> for IfStream<GC, GI>
where
    GC: Updatable<E>,
    GI: Updatable<E>,
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
pub struct IfElseStream<GC, GT, GF> {
    condition: GC,
    true_output: GT,
    false_output: GF,
}
impl<GC, GT, GF> IfElseStream<GC, GT, GF> {
    ///Constructor for [`IfElseStream`].
    pub const fn new(condition: GC, true_output: GT, false_output: GF) -> Self {
        Self {
            condition,
            true_output,
            false_output,
        }
    }
}
impl<T, GC, GT, GF, E> Getter<T, E> for IfElseStream<GC, GT, GF>
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
impl<GC, GT, GF, E> Updatable<E> for IfElseStream<GC, GT, GF>
where
    GC: Updatable<E>,
    GT: Updatable<E>,
    GF: Updatable<E>,
    E: Clone + Debug,
{
    fn update(&mut self) -> NothingOrError<E> {
        self.true_output.update()?;
        self.false_output.update()?;
        self.condition.update()?;
        Ok(())
    }
}
///"Freezes" the output of a `Getter` (the "Input Getter") based on whether a second, boolean
///`Getter` (the "Condition Getter") is returning true.
///
///Both `Getter`s are updated regardless of whether or not the stream is frozen.
///
///- If the Condition Getter returns `Err(_)` or `Ok(None)`, that value is returned instead of that
///  of the Input Getter.
///- If the Condition Getter returns true, the last value that was being returned before the
///  Condition Getter was returning true is maintained. This is the frozen state.
///- If the Condition Getter returns false, the value of the Input Getter is returned. This is the
///  unfrozen state.
pub struct FreezeStream<T, GC, GI, E> {
    condition: GC,
    input: GI,
    freeze_value: Output<T, E>,
}
impl<T, GC, GI, E> FreezeStream<T, GC, GI, E> {
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
    Self: Updatable<E>,
    Output<T, E>: Clone,
    E: Clone + Debug,
{
    fn get(&self) -> Output<T, E> {
        self.freeze_value.clone()
    }
}
impl<T, GC, GI, E> Updatable<E> for FreezeStream<T, GC, GI, E>
where
    Output<T, E>: Clone,
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
