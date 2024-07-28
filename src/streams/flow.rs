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
