// SPDX-License-Identifier: LGPL-3.0-only
/*
Copyright 2024 UxuginPython on GitHub

     This file is part of Rust Robotics ToolKit.

    Rust Robotics ToolKit is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, version 3.

    Rust Robotics ToolKit is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Lesser General Public License for more details.

    You should have received a copy of the GNU Lesser General Public License along with Rust Robotics ToolKit. If not, see <https://www.gnu.org/licenses/>.
*/
//!Provided `Device` implementors that allow a raw `Getter` or `Settable` to work with the device
//!system.
use crate::*;
///Connect a `Settable<Command, E>` to a `Terminal<E>` for use as a servo motor in the device
///system.
pub struct ActuatorWrapper<'a, T: Settable<TerminalData, E>, E: Copy + Debug> {
    inner: T,
    terminal: RefCell<Terminal<'a, E>>,
}
impl<'a, T: Settable<TerminalData, E>, E: Copy + Debug> ActuatorWrapper<'a, T, E> {
    ///Constructor for `SettableCommandDeviceWrapper`.
    pub fn new(inner: T) -> Self {
        Self {
            inner: inner,
            terminal: Terminal::new(),
        }
    }
    ///Get a reference to this wrapper's terminal.
    pub fn get_terminal(&self) -> &'a RefCell<Terminal<'a, E>> {
        unsafe { &*(&self.terminal as *const RefCell<Terminal<'a, E>>) }
    }
}
impl<T: Settable<TerminalData, E>, E: Copy + Debug> Device<E>
    for ActuatorWrapper<'_, T, E>
{
    fn update_terminals(&mut self) -> NothingOrError<E> {
        self.terminal.borrow_mut().update()?;
        Ok(())
    }
}
impl<T: Settable<TerminalData, E>, E: Copy + Debug> Updatable<E>
    for ActuatorWrapper<'_, T, E>
{
    fn update(&mut self) -> NothingOrError<E> {
        self.update_terminals()?;
        let terminal_borrow = self.terminal.borrow();
        let command: Option<Datum<Command>> = terminal_borrow.get_last_request();
        let state: Option<Datum<State>> = terminal_borrow.get().expect("Terminal get always returns Ok");
        let (mut time, command) = match command {
            Some(datum_command) => (Some(datum_command.time), Some(datum_command.value)),
            None => (None, None),
        };
        //This syntax is a bit complex. Basically, we need to shadow the state variable, but that
        //won't work if we just have two separate statements in the branches. We also want to set
        //the time to the state's time if it has one, otherwise keeping the possible command time.
        //This is because the state's is likely to be newer.
        let state = match state {
            Some(datum_state) => {
                time = Some(datum_state.time);
                Some(datum_state.value)
            }
            None => None,
        };
        match time {
            Some(time) => {
                self.inner.set(TerminalData {
                    time: time,
                    command: command,
                    state: state,
                })?;
            }
            None => {}
        }
        self.inner.update()?;
        Ok(())
    }
}
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
    ///Get a reference to this wrapper's terminal.
    pub fn get_terminal(&self) -> &'a RefCell<Terminal<'a, E>> {
        unsafe { &*(&self.terminal as *const RefCell<Terminal<'a, E>>) }
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
