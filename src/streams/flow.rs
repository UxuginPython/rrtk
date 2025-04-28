// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2024-2025 UxuginPython
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
            condition: condition,
            input: input,
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
            condition: condition,
            true_output: true_output,
            false_output: false_output,
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
        Ok(())
    }
}
///Returns the last value that a getter returned while another getter, a boolean, returned false.
///Passes the getter's value through if the boolean getter is false.
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
            condition: condition,
            input: input,
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
        let condition = match self.condition.get() {
            Err(error) => {
                self.freeze_value = Err(error);
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
