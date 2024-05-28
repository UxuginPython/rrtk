// SPDX-License-Identifier: LGPL-3.0-only
/*
Copyright 2024 UxuginPython on GitHub

     This file is part of Rust Robotics ToolKit.

    Rust Robotics ToolKit is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, version 3.

    Rust Robotics ToolKit is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Lesser General Public License for more details.

    You should have received a copy of the GNU Lesser General Public License along with Rust Robotics ToolKit. If not, see <https://www.gnu.org/licenses/>.
*/
use crate::*;
#[cfg(not(feature = "std"))]
use alloc::collections::vec_deque::VecDeque;
#[cfg(feature = "std")]
use std::collections::vec_deque::VecDeque;
pub mod control;
pub mod converters;
pub mod math;
pub struct Constant<T, E> {
    time_getter: InputTimeGetter<E>,
    value: T,
}
impl<T, E> Constant<T, E> {
    pub fn new(time_getter: InputTimeGetter<E>, value: T) -> Self {
        Self {
            time_getter: time_getter,
            value: value,
        }
    }
}
impl<T: Clone, E: Copy + Debug> Getter<T, E> for Constant<T, E> {
    fn get(&self) -> Output<T, E> {
        let time = self.time_getter.borrow().get()?;
        Ok(Some(Datum::new(time, self.value.clone())))
    }
}
impl<T: Clone, E: Copy + Debug> Updatable<E> for Constant<T, E> {
    fn update(&mut self) -> UpdateOutput<E> {
        Ok(())
    }
}
