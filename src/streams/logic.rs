// SPDX-License-Identifier: LGPL-3.0-only
/*
Copyright 2024 UxuginPython on GitHub

     This file is part of Rust Robotics ToolKit.

    Rust Robotics ToolKit is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, version 3.

    Rust Robotics ToolKit is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Lesser General Public License for more details.

    You should have received a copy of the GNU Lesser General Public License along with Rust Robotics ToolKit. If not, see <https://www.gnu.org/licenses/>.
*/
//!Logic operations for boolean getters.
use crate::streams::*;
//TODO: make these take arrays of inputs with generic lengths.
//TODO: document these better using that combination table thing
///Performs an and operation on two boolean getters.
pub struct AndStream<E: Copy + Debug> {
    input1: InputGetter<bool, E>,
    input2: InputGetter<bool, E>,
}
impl<E: Copy + Debug> AndStream<E> {
    ///Constructor for `AndStream`.
    pub fn new(input1: InputGetter<bool, E>, input2: InputGetter<bool, E>) -> Self {
        Self {
            input1: input1,
            input2: input2,
        }
    }
}
impl<E: Copy + Debug> Getter<bool, E> for AndStream<E> {
    fn get(&self) -> Output<bool, E> {
        let gotten1 = self.input1.borrow().get()?;
        let gotten2 = self.input2.borrow().get()?;
        let mut time = None;
        let mut can_be_true = true;
        let mut can_return_true = true;
        match gotten1 {
            Some(datum) => {
                time = Some(datum.time);
                if !datum.value {
                    can_be_true = false;
                    can_return_true = false;
                }
            }
            None => {
                can_return_true = false;
            }
        }
        match gotten2 {
            Some(datum) => {
                match time {
                    Some(existing) => {
                        if datum.time > existing {
                            time = Some(datum.time);
                        }
                    }
                    None => time = Some(datum.time),
                }
                if !datum.value {
                    can_be_true = false;
                    can_return_true = false;
                }
            }
            None => {
                can_return_true = false;
            }
        }
        //Never assume the boolean value of a None from an input:
        //To return true, we require that both inputs return true (not None).
        //To return false, we require that at least one input returns false (not None).
        //If neither of these is met, return None.
        let time = match time {
            Some(time) => time,
            None => return Ok(None),
        };
        match (can_be_true, can_return_true) {
            (false, false) => Ok(Some(Datum::new(time, false))),
            (true, false) => Ok(None),
            (true, true) => Ok(Some(Datum::new(time, true))),
            (false, true) => unimplemented!(),
        }
    }
}
impl<E: Copy + Debug> Updatable<E> for AndStream<E> {
    fn update(&mut self) -> NothingOrError<E> {
        Ok(())
    }
}
