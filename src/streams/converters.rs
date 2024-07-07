// SPDX-License-Identifier: LGPL-3.0-only
/*
Copyright 2024 UxuginPython on GitHub

     This file is part of Rust Robotics ToolKit.

    Rust Robotics ToolKit is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, version 3.

    Rust Robotics ToolKit is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Lesser General Public License for more details.

    You should have received a copy of the GNU Lesser General Public License along with Rust Robotics ToolKit. If not, see <https://www.gnu.org/licenses/>.
*/
//!Streams that convert from one type to another. Some of these also do keep the same type and are
//!for convenience in certain situations, for example when you do not want to handle a `None`
//!variant yourself.
use crate::streams::math::*;
use crate::streams::*;
///A stream converting all `Ok(None)` values from its input to `Err(_)` variants.
pub struct NoneToError<T: Clone, E> {
    input: InputGetter<T, E>,
}
impl<T: Clone, E> NoneToError<T, E> {
    ///Constructor for `NoneToError`.
    pub fn new(input: InputGetter<T, E>) -> Self {
        Self { input: input }
    }
}
impl<T: Clone, E: Copy + Debug> Getter<T, E> for NoneToError<T, E> {
    fn get(&self) -> Output<T, E> {
        let output = self.input.borrow().get()?;
        match output {
            Some(_) => {
                return Ok(output);
            }
            None => {
                return Err(Error::FromNone);
            }
        }
    }
}
impl<T: Clone, E: Copy + Debug> Updatable<E> for NoneToError<T, E> {
    ///This does not need to be called.
    fn update(&mut self) -> NothingOrError<E> {
        Ok(())
    }
}
///A stream converting all `Ok(None)` values from its input to a default `Ok(Some(_))` value.
pub struct NoneToValue<T, E> {
    input: InputGetter<T, E>,
    time_getter: InputTimeGetter<E>,
    none_value: T,
}
impl<T, E> NoneToValue<T, E> {
    ///Constructor for `NoneToValue`.
    pub fn new(input: InputGetter<T, E>, time_getter: InputTimeGetter<E>, none_value: T) -> Self {
        Self {
            input: input,
            time_getter: time_getter,
            none_value: none_value,
        }
    }
}
impl<T: Clone, E: Copy + Debug> Getter<T, E> for NoneToValue<T, E> {
    fn get(&self) -> Output<T, E> {
        let output = self.input.borrow().get()?;
        match output {
            Some(_) => {
                return Ok(output);
            }
            None => {
                return Ok(Some(Datum::new(
                    self.time_getter.borrow().get()?,
                    self.none_value.clone(),
                )))
            }
        }
    }
}
impl<T: Clone, E: Copy + Debug> Updatable<E> for NoneToValue<T, E> {
    fn update(&mut self) -> NothingOrError<E> {
        Ok(())
    }
}
///A stream that integrates an acceleration getter to construct a full state. Mostly useful for
///encoders.
pub struct AccelerationToState<E: Copy + Debug> {
    acc: InputGetter<f32, E>,
    vel: InputGetter<f32, E>,
    pos: InputGetter<f32, E>,
}
impl<E: Copy + Debug + 'static> AccelerationToState<E> {
    ///Constructor for `AccelerationToState`.
    pub fn new(acc: InputGetter<f32, E>) -> Self {
        let vel = make_input_getter!(IntegralStream::new(Rc::clone(&acc)), f32, E);
        let pos = make_input_getter!(IntegralStream::new(Rc::clone(&vel)), f32, E);
        Self {
            acc: acc,
            vel: vel,
            pos: pos,
        }
    }
}
impl<E: Copy + Debug> Getter<State, E> for AccelerationToState<E> {
    fn get(&self) -> Output<State, E> {
        let acc = self.acc.borrow().get()?;
        let vel = self.vel.borrow().get()?;
        let pos = self.pos.borrow().get()?;
        match acc {
            Some(_) => {}
            None => {
                return Ok(None);
            }
        }
        match vel {
            Some(_) => {}
            None => {
                return Ok(None);
            }
        }
        match pos {
            Some(_) => {}
            None => {
                return Ok(None);
            }
        }
        let acc = acc.unwrap();
        let vel = vel.unwrap();
        let pos = pos.unwrap();
        let time = acc.time;
        Ok(Some(Datum::new(
            time,
            State::new(pos.value, vel.value, acc.value),
        )))
    }
}
impl<E: Copy + Debug> Updatable<E> for AccelerationToState<E> {
    fn update(&mut self) -> NothingOrError<E> {
        self.vel.borrow_mut().update()?;
        self.pos.borrow_mut().update()?;
        Ok(())
    }
}
///A stream that integrates and derivates a velocity getter to construct a full state. Mostly
///useful for encoders.
pub struct VelocityToState<E: Copy + Debug> {
    acc: InputGetter<f32, E>,
    vel: InputGetter<f32, E>,
    pos: InputGetter<f32, E>,
}
impl<E: Copy + Debug + 'static> VelocityToState<E> {
    ///Constructor for `VelocityToState`.
    pub fn new(vel: InputGetter<f32, E>) -> Self {
        let acc = make_input_getter!(DerivativeStream::new(Rc::clone(&vel)), f32, E);
        let pos = make_input_getter!(IntegralStream::new(Rc::clone(&vel)), f32, E);
        Self {
            acc: acc,
            vel: vel,
            pos: pos,
        }
    }
}
impl<E: Copy + Debug> Getter<State, E> for VelocityToState<E> {
    fn get(&self) -> Output<State, E> {
        let acc = self.acc.borrow().get()?;
        let vel = self.vel.borrow().get()?;
        let pos = self.pos.borrow().get()?;
        match acc {
            Some(_) => {}
            None => {
                return Ok(None);
            }
        }
        match vel {
            Some(_) => {}
            None => {
                return Ok(None);
            }
        }
        match pos {
            Some(_) => {}
            None => {
                return Ok(None);
            }
        }
        let acc = acc.unwrap();
        let vel = vel.unwrap();
        let pos = pos.unwrap();
        let time = acc.time;
        Ok(Some(Datum::new(
            time,
            State::new(pos.value, vel.value, acc.value),
        )))
    }
}
impl<E: Copy + Debug> Updatable<E> for VelocityToState<E> {
    fn update(&mut self) -> NothingOrError<E> {
        self.acc.borrow_mut().update()?;
        self.pos.borrow_mut().update()?;
        Ok(())
    }
}
///A stream that derivates a position getter to construct a full state. Mostly useful for encoders.
pub struct PositionToState<E: Copy + Debug> {
    acc: InputGetter<f32, E>,
    vel: InputGetter<f32, E>,
    pos: InputGetter<f32, E>,
}
impl<E: Copy + Debug + 'static> PositionToState<E> {
    ///Constructor for `PositionToState`.
    pub fn new(pos: InputGetter<f32, E>) -> Self {
        let vel = make_input_getter!(DerivativeStream::new(Rc::clone(&pos)), f32, E);
        let acc = make_input_getter!(DerivativeStream::new(Rc::clone(&vel)), f32, E);
        Self {
            acc: acc,
            vel: vel,
            pos: pos,
        }
    }
}
impl<E: Copy + Debug> Getter<State, E> for PositionToState<E> {
    fn get(&self) -> Output<State, E> {
        let acc = self.acc.borrow().get()?;
        let vel = self.vel.borrow().get()?;
        let pos = self.pos.borrow().get()?;
        match acc {
            Some(_) => {}
            None => {
                return Ok(None);
            }
        }
        match vel {
            Some(_) => {}
            None => {
                return Ok(None);
            }
        }
        match pos {
            Some(_) => {}
            None => {
                return Ok(None);
            }
        }
        let acc = acc.unwrap();
        let vel = vel.unwrap();
        let pos = pos.unwrap();
        let time = acc.time;
        Ok(Some(Datum::new(
            time,
            State::new(pos.value, vel.value, acc.value),
        )))
    }
}
impl<E: Copy + Debug> Updatable<E> for PositionToState<E> {
    fn update(&mut self) -> NothingOrError<E> {
        self.vel.borrow_mut().update()?;
        self.acc.borrow_mut().update()?;
        Ok(())
    }
}
