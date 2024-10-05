// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2024 UxuginPython
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
    pub const fn new(inner: T) -> Self {
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
impl<T: Settable<TerminalData, E>, E: Copy + Debug> Device<E> for ActuatorWrapper<'_, T, E> {
    fn update_terminals(&mut self) -> NothingOrError<E> {
        self.terminal.borrow_mut().update()?;
        Ok(())
    }
}
impl<T: Settable<TerminalData, E>, E: Copy + Debug> Updatable<E> for ActuatorWrapper<'_, T, E> {
    fn update(&mut self) -> NothingOrError<E> {
        self.update_terminals()?;
        match self
            .terminal
            .borrow()
            .get()
            .expect("Terminal TerminalData get always returns Ok")
        {
            Some(terminal_data) => self.inner.set(terminal_data.value)?,
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
    pub const fn new(inner: T) -> Self {
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
        self.inner.update()?;
        self.update_terminals()?;
        let new_state_datum = match self.inner.get()? {
            None => return Ok(()),
            Some(state_datum) => state_datum,
        };
        self.terminal.borrow_mut().set(new_state_datum)?;
        Ok(())
    }
}
/*///Connect a `Settable<f32, E>` motor to the device system through a `CommandPID`. See
///`streams::control::CommandPID` documentation for more information about how this works.
pub struct PIDWrapper<'a, T: Settable<f32, E>, E: Copy + Debug + 'static> {
    terminal: RefCell<Terminal<'a, E>>,
    time: Rc<RefCell<i64>>,
    state: Rc<RefCell<ConstantGetter<State, E>>>,
    command: Rc<RefCell<ConstantGetter<Command, E>>>,
    pid: Rc<RefCell<streams::control::CommandPID<E>>>,
    inner: T,
}
impl<'a, T: Settable<f32, E>, E: Copy + Debug + 'static> PIDWrapper<'a, T, E> {
    ///Constructor for `PIDWrapper`.
    pub fn new( //TODO when this is updated to use Reference: Check if this can be a const fn
        mut inner: T,
        initial_time: i64,
        initial_state: State,
        initial_command: Command,
        kvalues: PositionDerivativeDependentPIDKValues,
    ) -> Self {
        let terminal = Terminal::new();
        let time = Rc::new(RefCell::new(initial_time));
        let state = Rc::new(RefCell::new(ConstantGetter::new(
            Rc::clone(&time) as InputTimeGetter<E>,
            initial_state,
        )));
        let command = Rc::new(RefCell::new(ConstantGetter::new(
            Rc::clone(&time) as InputTimeGetter<E>,
            initial_command,
        )));
        let pid = Rc::new(RefCell::new(streams::control::CommandPID::new(
            Rc::clone(&state) as InputGetter<State, E>,
            initial_command,
            kvalues,
        )));
        pid.borrow_mut()
            .follow(Rc::clone(&command) as InputGetter<Command, E>);
        inner.follow(Rc::clone(&pid) as InputGetter<f32, E>);
        Self {
            terminal: terminal,
            time: time,
            state: state,
            command: command,
            pid: pid,
            inner: inner,
        }
    }
    ///Get a reference to this wrapper's terminal.
    pub fn get_terminal(&self) -> &'a RefCell<Terminal<'a, E>> {
        unsafe { &*(&self.terminal as *const RefCell<Terminal<'a, E>>) }
    }
}
impl<T: Settable<f32, E>, E: Copy + Debug + 'static> Device<E> for PIDWrapper<'_, T, E> {
    fn update_terminals(&mut self) -> NothingOrError<E> {
        self.terminal.borrow_mut().update()?;
        Ok(())
    }
}
impl<T: Settable<f32, E>, E: Copy + Debug + 'static> Updatable<E> for PIDWrapper<'_, T, E> {
    fn update(&mut self) -> NothingOrError<E> {
        self.update_terminals()?;
        let terminal_data: Option<Datum<TerminalData>> =
            self.terminal.borrow().get().expect("This can't return Err");
        match terminal_data {
            Some(terminal_data) => {
                let terminal_data = terminal_data.value;
                *self.time.borrow_mut() = terminal_data.time;
                match terminal_data.state {
                    Some(state) => self.state.borrow_mut().set(state)?,
                    None => (),
                }
                match terminal_data.command {
                    Some(command) => self.command.borrow_mut().set(command)?,
                    None => (),
                }
                self.pid.borrow_mut().update()?;
            }
            None => (),
        }
        self.inner.update()?;
        Ok(())
    }
}*/
