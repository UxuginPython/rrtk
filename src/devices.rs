// SPDX-License-Identifier: LGPL-3.0-only
/*
Copyright 2024 UxuginPython on GitHub

     This file is part of Rust Robotics ToolKit.

    Rust Robotics ToolKit is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, version 3.

    Rust Robotics ToolKit is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Lesser General Public License for more details.

    You should have received a copy of the GNU Lesser General Public License along with Rust Robotics ToolKit. If not, see <https://www.gnu.org/licenses/>.
*/
//!RRTK's device system works through a graph-like structure where each device holds `Rc`s to
//!objects called terminals. Terminals represent anywhere a device can connect to another.
//!Connected terminals hold `Weak` references to eachother. This module holds builtin devices.
use crate::*;
//TODO: test this
///A device such that positive for one terminal is negative for the other.
pub struct Invert<E: Copy + Debug> {
    term1: Rc<RefCell<Terminal<E>>>,
    term2: Rc<RefCell<Terminal<E>>>,
}
impl<E: Copy + Debug> Invert<E> {
    ///Constructor for `Invert`.
    pub fn new(term1: Rc<RefCell<Terminal<E>>>, term2: Rc<RefCell<Terminal<E>>>) -> Self {
        Self {
            term1: term1,
            term2: term2,
        }
    }
}
impl<E: Copy + Debug> Updatable<E> for Invert<E> {
    fn update(&mut self) -> NothingOrError<E> {
        self.update_terminals()?;
        let get1 = self
            .term1
            .borrow()
            .get()
            .expect("Terminal get will always return Ok");
        let get2 = self
            .term2
            .borrow()
            .get()
            .expect("Terminal get will always return Ok");
        match get1 {
            None => match get2 {
                None => {}
                Some(datum2) => {
                    let newdatum1 = Datum::new(datum2.time, -datum2.value);
                    self.term1.borrow_mut().set(newdatum1)?;
                }
            },
            Some(datum1) => match get2 {
                None => {
                    let newdatum2 = Datum::new(datum1.time, -datum1.value);
                    self.term2.borrow_mut().set(newdatum2)?;
                }
                Some(datum2) => {
                    let state1 = datum1.value;
                    let state2 = datum2.value;
                    let time = if datum1.time >= datum2.time {
                        datum1.time
                    } else {
                        datum2.time
                    };
                    //average with negative state2 as it is inverted from state1
                    let new_state = (state1 - state2) / 2.0;
                    self.term1.borrow_mut().set(Datum::new(time, new_state))?;
                    self.term2.borrow_mut().set(Datum::new(time, -new_state))?;
                }
            },
        }
        Ok(())
    }
}
impl<E: Copy + Debug> Device<E> for Invert<E> {
    fn update_terminals(&mut self) -> NothingOrError<E> {
        self.term1.borrow_mut().update()?;
        self.term2.borrow_mut().update()?;
        Ok(())
    }
}
//TODO: test this
///Connect a `Settable<Command, E>` to a `Terminal<E>` for use as a servo motor in the device
///system.
pub struct SettableCommandDeviceWrapper<T: Settable<Command, E>, E: Copy + Debug> {
    inner: T,
    terminal: Rc<RefCell<Terminal<E>>>,
}
impl<T: Settable<Command, E>, E: Copy + Debug> SettableCommandDeviceWrapper<T, E> {
    ///Constructor for `SettableCommandDeviceWrapper`.
    pub fn new(inner: T, terminal: Rc<RefCell<Terminal<E>>>) -> Self {
        Self {
            inner: inner,
            terminal: terminal,
        }
    }
}
impl<T: Settable<Command, E>, E: Copy + Debug> Device<E> for SettableCommandDeviceWrapper<T, E> {
    fn update_terminals(&mut self) -> NothingOrError<E> {
        self.terminal.borrow_mut().update()?;
        Ok(())
    }
}
impl<T: Settable<Command, E>, E: Copy + Debug> Updatable<E> for SettableCommandDeviceWrapper<T, E> {
    fn update(&mut self) -> NothingOrError<E> {
        match <Terminal<E> as Settable<Datum<Command>, E>>::get_last_request(&self.terminal.borrow()) {
            Some(command) => {
                self.inner.set(command.value)?;
            }
            None => {}
        }
        Ok(())
    }
}
//TODO: test this
///Connect a `Getter<State, E>` to a `Terminal<E>` for use as an encoder in the device system.
pub struct GetterStateDeviceWrapper<T: Getter<State, E>, E: Copy + Debug> {
    inner: T,
    terminal: Rc<RefCell<Terminal<E>>>,
}
impl<T: Getter<State, E>, E: Copy + Debug> GetterStateDeviceWrapper<T, E> {
    ///Constructor for `GetterStateDeviceWrapper`.
    pub fn new(inner: T, terminal: Rc<RefCell<Terminal<E>>>) -> Self {
        Self {
            inner: inner,
            terminal: terminal,
        }
    }
}
impl<T: Getter<State, E>, E: Copy + Debug> Device<E> for GetterStateDeviceWrapper<T, E> {
    fn update_terminals(&mut self) -> NothingOrError<E> {
        self.terminal.borrow_mut().update()?;
        Ok(())
    }
}
impl<T: Getter<State, E>, E: Copy + Debug> Updatable<E> for GetterStateDeviceWrapper<T, E> {
    fn update(&mut self) -> NothingOrError<E> {
        self.update_terminals()?;
        let new_state_datum = match self.inner.get()? {
            None => return Ok(()),
            Some(state_datum) => state_datum,
        };
        self.terminal.borrow_mut().set(new_state_datum)?;
        Ok(())
    }
}
