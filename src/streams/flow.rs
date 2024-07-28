// SPDX-License-Identifier: LGPL-3.0-only
/*
Copyright 2024 UxuginPython on GitHub

     This file is part of Rust Robotics ToolKit.

    Rust Robotics ToolKit is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, version 3.

    Rust Robotics ToolKit is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Lesser General Public License for more details.

    You should have received a copy of the GNU Lesser General Public License along with Rust Robotics ToolKit. If not, see <https://www.gnu.org/licenses/>.
*/
//!Streams for control flow.
use crate::streams::*;
///Propagates its input if a `Getter<bool, _>` returns `Ok(Some(true))`, otherwise returns
///`Ok(None)`.
pub struct IfStream<T, E: Copy + Debug> {
    condition: InputGetter<bool, E>,
    input: InputGetter<T, E>,
}
impl<T, E: Copy + Debug> IfStream<T, E> {
    ///Constructor for `IfStream`.
    pub fn new(condition: InputGetter<bool, E>, input: InputGetter<T, E>) -> Self {
        Self {
            condition: condition,
            input: input,
        }
    }
}
impl<T, E: Copy + Debug> Getter<T, E> for IfStream<T, E> {
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
impl<T, E: Copy + Debug> Updatable<E> for IfStream<T, E> {
    fn update(&mut self) -> NothingOrError<E> {
        Ok(())
    }
}
///Returns the output of one input if a `Getter<bool, _>` returns `Ok(Some(true))` and another if
///it returns `Ok(Some(false))`. Returns `Ok(None)` if the `Getter<bool, _>` does.
pub struct IfElseStream<T, E: Copy + Debug> {
    condition: InputGetter<bool, E>,
    true_output: InputGetter<T, E>,
    false_output: InputGetter<T, E>,
}
impl<T, E: Copy + Debug> IfElseStream<T, E> {
    ///Constructor for `IfElseStream`.
    pub fn new(condition: InputGetter<bool, E>, true_output: InputGetter<T, E>, false_output: InputGetter<T, E>) -> Self {
        Self {
            condition: condition,
            true_output: true_output,
            false_output: false_output,
        }
    }
}
impl<T, E: Copy + Debug> Getter<T, E> for IfElseStream<T, E> {
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
impl<T, E: Copy + Debug> Updatable<E> for IfElseStream<T, E> {
    fn update(&mut self) -> NothingOrError<E> {
        Ok(())
    }
}
