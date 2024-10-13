// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2024 UxuginPython
//!Streams for control flow.
use crate::streams::*;
///Propagates its input if a `Getter<bool, _>` returns `Ok(Some(true))`, otherwise returns
///`Ok(None)`.
pub struct IfStream<T, GC: Getter<bool, E> + ?Sized, GI: Getter<T, E> + ?Sized, E: Copy + Debug> {
    condition: Reference<GC>,
    input: Reference<GI>,
    phantom_t: PhantomData<T>,
    phantom_e: PhantomData<E>,
}
impl<T, GC: Getter<bool, E> + ?Sized, GI: Getter<T, E> + ?Sized, E: Copy + Debug>
    IfStream<T, GC, GI, E>
{
    ///Constructor for `IfStream`.
    pub const fn new(condition: Reference<GC>, input: Reference<GI>) -> Self {
        Self {
            condition: condition,
            input: input,
            phantom_t: PhantomData,
            phantom_e: PhantomData,
        }
    }
}
impl<T, GC: Getter<bool, E> + ?Sized, GI: Getter<T, E> + ?Sized, E: Copy + Debug> Getter<T, E>
    for IfStream<T, GC, GI, E>
{
    fn get(&self) -> Output<T, E> {
        let condition = match self.condition.borrow().get()? {
            Some(output) => output.value,
            None => false,
        };
        if condition {
            self.input.borrow().get()
        } else {
            Ok(None)
        }
    }
}
impl<T, GC: Getter<bool, E> + ?Sized, GI: Getter<T, E> + ?Sized, E: Copy + Debug> Updatable<E>
    for IfStream<T, GC, GI, E>
{
    fn update(&mut self) -> NothingOrError<E> {
        Ok(())
    }
}
///Returns the output of one input if a `Getter<bool, _>` returns `Ok(Some(true))` and another if
///it returns `Ok(Some(false))`. Returns `Ok(None)` if the `Getter<bool, _>` does.
pub struct IfElseStream<
    T,
    GC: Getter<bool, E> + ?Sized,
    GT: Getter<T, E> + ?Sized,
    GF: Getter<T, E> + ?Sized,
    E: Copy + Debug,
> {
    condition: Reference<GC>,
    true_output: Reference<GT>,
    false_output: Reference<GF>,
    phantom_t: PhantomData<T>,
    phantom_e: PhantomData<E>,
}
impl<
        T,
        GC: Getter<bool, E> + ?Sized,
        GT: Getter<T, E> + ?Sized,
        GF: Getter<T, E> + ?Sized,
        E: Copy + Debug,
    > IfElseStream<T, GC, GT, GF, E>
{
    ///Constructor for `IfElseStream`.
    pub const fn new(
        condition: Reference<GC>,
        true_output: Reference<GT>,
        false_output: Reference<GF>,
    ) -> Self {
        Self {
            condition: condition,
            true_output: true_output,
            false_output: false_output,
            phantom_t: PhantomData,
            phantom_e: PhantomData,
        }
    }
}
impl<
        T,
        GC: Getter<bool, E> + ?Sized,
        GT: Getter<T, E> + ?Sized,
        GF: Getter<T, E> + ?Sized,
        E: Copy + Debug,
    > Getter<T, E> for IfElseStream<T, GC, GT, GF, E>
{
    fn get(&self) -> Output<T, E> {
        let condition = match self.condition.borrow().get()? {
            Some(output) => output.value,
            None => return Ok(None),
        };
        if condition {
            self.true_output.borrow().get()
        } else {
            self.false_output.borrow().get()
        }
    }
}
impl<
        T,
        GC: Getter<bool, E> + ?Sized,
        GT: Getter<T, E> + ?Sized,
        GF: Getter<T, E> + ?Sized,
        E: Copy + Debug,
    > Updatable<E> for IfElseStream<T, GC, GT, GF, E>
{
    fn update(&mut self) -> NothingOrError<E> {
        Ok(())
    }
}
///Returns the last value that a getter returned while another getter, a boolean, returned false.
///Passes the getter's value through if the boolean getter is false.
pub struct FreezeStream<
    T: Clone,
    GC: Getter<bool, E> + ?Sized,
    GI: Getter<T, E> + ?Sized,
    E: Copy + Debug,
> {
    condition: Reference<GC>,
    input: Reference<GI>,
    freeze_value: Output<T, E>,
}
impl<T: Clone, GC: Getter<bool, E> + ?Sized, GI: Getter<T, E> + ?Sized, E: Copy + Debug>
    FreezeStream<T, GC, GI, E>
{
    ///Constructor for `FreezeStream`.
    pub const fn new(condition: Reference<GC>, input: Reference<GI>) -> Self {
        Self {
            condition: condition,
            input: input,
            freeze_value: Ok(None),
        }
    }
}
impl<T: Clone, GC: Getter<bool, E> + ?Sized, GI: Getter<T, E> + ?Sized, E: Copy + Debug>
    Getter<T, E> for FreezeStream<T, GC, GI, E>
{
    fn get(&self) -> Output<T, E> {
        self.freeze_value.clone()
    }
}
impl<T: Clone, GC: Getter<bool, E> + ?Sized, GI: Getter<T, E> + ?Sized, E: Copy + Debug>
    Updatable<E> for FreezeStream<T, GC, GI, E>
{
    fn update(&mut self) -> NothingOrError<E> {
        let condition = match self.condition.borrow().get() {
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
            let gotten = self.input.borrow().get();
            self.freeze_value = gotten.clone();
            match gotten {
                Ok(_) => {}
                Err(error) => return Err(error),
            }
        }
        Ok(())
    }
}
