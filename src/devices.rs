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
pub struct Invert<'a, E: Copy + Debug> {
    term1: RefCell<Terminal<'a, E>>,
    term2: RefCell<Terminal<'a, E>>,
}
impl<'a, E: Copy + Debug> Invert<'a, E> {
    ///Constructor for `Invert`.
    pub fn new() -> Self {
        Self {
            term1: Terminal::new(),
            term2: Terminal::new(),
        }
    }
}
impl<E: Copy + Debug> Updatable<E> for Invert<'_, E> {
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
impl<E: Copy + Debug> Device<E> for Invert<'_, E> {
    fn update_terminals(&mut self) -> NothingOrError<E> {
        self.term1.borrow_mut().update()?;
        self.term2.borrow_mut().update()?;
        Ok(())
    }
}
//TODO: Test this.
//TODO: Add a constructor for this.
///A connection between terminals that are not directly connected, such as when three or more
///terminals are connected. Code-wise, this is almost exactly the same as directly connecting two
///terminals, but this type can connect more than two terminals. There is some freedom in exactly
///what you do with each of these ways of connecting terminals and what they represent physically,
///but the intention is that `connect` is for only two and `Axle` is for more. Using an `Axle` for
///only two terminals is possible but may have a slight performance cost. (The type even
///technically allows for only one or even zero connected terminals, but there is almost certainly
///no legitimate use for this.)
pub struct Axle<'a, const N: usize, E: Copy + Debug> {
    inputs: [&'a RefCell<Terminal<'a, E>>; N],
}
impl<'a, const N: usize, E: Copy + Debug> Axle<'a, N, E> {
    ///Constructor for `Axle`.
    pub fn new(inputs: [&'a RefCell<Terminal<'a, E>>; N]) -> Self {
        Self {
            inputs: inputs,
        }
    }
}
impl<const N: usize, E: Copy + Debug> Updatable<E> for Axle<'_, N, E> {
    fn update(&mut self) -> NothingOrError<E> {
        self.update_terminals()?;
        let mut count = 0u16;
        let mut datum = Datum::new(i64::MIN, State::default());
        for i in &self.inputs {
            match i.borrow().get()? {
                Some(gotten_datum) => {
                    datum += gotten_datum;
                    count += 1;
                },
                None => (),
            }
        }
        if count >= 1 {
            datum /= count as f32;
            for i in &self.inputs {
                i.borrow_mut().set(datum.clone())?;
            }
        }
        Ok(())
    }
}
impl<const N: usize, E: Copy + Debug> Device<E> for Axle<'_, N, E> {
    fn update_terminals(&mut self) -> NothingOrError<E> {
        for i in &self.inputs {
            i.borrow_mut().update()?;
        }
        Ok(())
    }
}
//TODO: test this
///Connect a `Settable<Command, E>` to a `Terminal<E>` for use as a servo motor in the device
///system.
pub struct SettableCommandDeviceWrapper<'a, T: Settable<Command, E>, E: Copy + Debug> {
    inner: T,
    terminal: RefCell<Terminal<'a, E>>,
}
impl<'a, T: Settable<Command, E>, E: Copy + Debug> SettableCommandDeviceWrapper<'a, T, E> {
    ///Constructor for `SettableCommandDeviceWrapper`.
    pub fn new(inner: T) -> Self {
        Self {
            inner: inner,
            terminal: Terminal::new(),
        }
    }
}
impl<T: Settable<Command, E>, E: Copy + Debug> Device<E>
    for SettableCommandDeviceWrapper<'_, T, E>
{
    fn update_terminals(&mut self) -> NothingOrError<E> {
        self.terminal.borrow_mut().update()?;
        Ok(())
    }
}
impl<T: Settable<Command, E>, E: Copy + Debug> Updatable<E>
    for SettableCommandDeviceWrapper<'_, T, E>
{
    fn update(&mut self) -> NothingOrError<E> {
        match <Terminal<E> as Settable<Datum<Command>, E>>::get_last_request(
            &self.terminal.borrow(),
        ) {
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
pub struct GetterStateDeviceWrapper<'a, T: Getter<State, E>, E: Copy + Debug> {
    inner: T,
    terminal: RefCell<Terminal<'a, E>>,
}
impl<'a, T: Getter<State, E>, E: Copy + Debug> GetterStateDeviceWrapper<'a, T, E> {
    ///Constructor for `GetterStateDeviceWrapper`.
    pub fn new(inner: T) -> Self {
        Self {
            inner: inner,
            terminal: Terminal::new(),
        }
    }
}
impl<T: Getter<State, E>, E: Copy + Debug> Device<E> for GetterStateDeviceWrapper<'_, T, E> {
    fn update_terminals(&mut self) -> NothingOrError<E> {
        self.terminal.borrow_mut().update()?;
        Ok(())
    }
}
impl<T: Getter<State, E>, E: Copy + Debug> Updatable<E> for GetterStateDeviceWrapper<'_, T, E> {
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
