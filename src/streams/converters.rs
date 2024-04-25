// SPDX-License-Identifier: LGPL-3.0-only
/*
Copyright 2024 UxuginPython on GitHub

     This file is part of Rust Robotics ToolKit.

    Rust Robotics ToolKit is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, version 3.

    Rust Robotics ToolKit is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Lesser General Public License for more details.

    You should have received a copy of the GNU Lesser General Public License along with Rust Robotics ToolKit. If not, see <https://www.gnu.org/licenses/>.
*/
use crate::streams::math::*;
use crate::streams::*;
use crate::*;
pub struct NoneToError<T: Clone, E> {
    input: InputStream<T, E>,
}
impl<T: Clone, E> NoneToError<T, E> {
    pub fn new(input: InputStream<T, E>) -> Self {
        Self { input: input }
    }
}
impl<T: Clone, E: Copy + Debug> Stream<T, E> for NoneToError<T, E> {
    fn get(&self) -> StreamOutput<T, E> {
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
    fn update(&mut self) {}
}
pub struct NoneToValue<T, E> {
    input: InputStream<T, E>,
    time_getter: InputTimeGetter<E>,
    none_value: T,
}
impl<T, E> NoneToValue<T, E> {
    pub fn new(input: InputStream<T, E>, time_getter: InputTimeGetter<E>, none_value: T) -> Self {
        Self {
            input: input,
            time_getter: time_getter,
            none_value: none_value,
        }
    }
}
impl<T: Clone, E: Copy + Debug> Stream<T, E> for NoneToValue<T, E> {
    fn get(&self) -> StreamOutput<T, E> {
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
    fn update(&mut self) {}
}
pub struct AccelerationToState<E: Copy + Debug> {
    acc: InputStream<f32, E>,
    vel: InputStream<f32, E>,
    pos: InputStream<f32, E>,
}
impl<E: Copy + Debug + 'static> AccelerationToState<E> {
    pub fn new(acc: InputStream<f32, E>) -> Self {
        let vel = make_stream_input!(IntegralStream::new(Rc::clone(&acc)), f32, E);
        let pos = make_stream_input!(IntegralStream::new(Rc::clone(&vel)), f32, E);
        Self {
            acc: acc,
            vel: vel,
            pos: pos,
        }
    }
}
impl<E: Copy + Debug> Stream<State, E> for AccelerationToState<E> {
    fn get(&self) -> StreamOutput<State, E> {
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
        let mut time = acc.time;
        if vel.time > time {
            time = vel.time;
        }
        if pos.time > time {
            time = pos.time;
        }
        Ok(Some(Datum::new(
            time,
            State::new(pos.value, vel.value, acc.value),
        )))
    }
    fn update(&mut self) {
        self.vel.borrow_mut().update();
        self.pos.borrow_mut().update();
    }
}
pub struct VelocityToState<E: Copy + Debug> {
    acc: InputStream<f32, E>,
    vel: InputStream<f32, E>,
    pos: InputStream<f32, E>,
}
impl<E: Copy + Debug + 'static> VelocityToState<E> {
    pub fn new(vel: InputStream<f32, E>) -> Self {
        let acc = make_stream_input!(DerivativeStream::new(Rc::clone(&vel)), f32, E);
        let pos = make_stream_input!(IntegralStream::new(Rc::clone(&vel)), f32, E);
        Self {
            acc: acc,
            vel: vel,
            pos: pos,
        }
    }
}
impl<E: Copy + Debug> Stream<State, E> for VelocityToState<E> {
    fn get(&self) -> StreamOutput<State, E> {
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
        let mut time = acc.time;
        if vel.time > time {
            time = vel.time;
        }
        if pos.time > time {
            time = pos.time;
        }
        Ok(Some(Datum::new(
            time,
            State::new(pos.value, vel.value, acc.value),
        )))
    }
    fn update(&mut self) {
        self.acc.borrow_mut().update();
        self.pos.borrow_mut().update();
    }
}
pub struct PositionToStream<E: Copy + Debug> {
    acc: InputStream<f32, E>,
    vel: InputStream<f32, E>,
    pos: InputStream<f32, E>,
}
impl<E: Copy + Debug + 'static> PositionToStream<E> {
    pub fn new(pos: InputStream<f32, E>) -> Self {
        let vel = make_stream_input!(DerivativeStream::new(Rc::clone(&pos)), f32, E);
        let acc = make_stream_input!(DerivativeStream::new(Rc::clone(&vel)), f32, E);
        Self {
            acc: acc,
            vel: vel,
            pos: pos,
        }
    }
}
impl<E: Copy + Debug> Stream<State, E> for PositionToStream<E> {
    fn get(&self) -> StreamOutput<State, E> {
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
        let mut time = acc.time;
        if vel.time > time {
            time = vel.time;
        }
        if pos.time > time {
            time = pos.time;
        }
        Ok(Some(Datum::new(
            time,
            State::new(pos.value, vel.value, acc.value),
        )))
    }
    fn update(&mut self) {
        self.vel.borrow_mut().update();
        self.acc.borrow_mut().update();
    }
}
