// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2024-2025 UxuginPython
//!Provided [`Device`] implementors that allow a raw [`Getter`] or [`Settable`] to work with the device
//!system.
use crate::*;
///Connect a [`Settable<Command, E>`] to a [`Terminal<E>`] for use as a servo motor in the device
///system.
pub struct ActuatorWrapper<'a, T: Settable<TerminalData, E>, E: Clone + Debug> {
    inner: T,
    terminal: RefCell<Terminal<'a, E>>,
}
impl<'a, T: Settable<TerminalData, E>, E: Clone + Debug> ActuatorWrapper<'a, T, E> {
    ///Constructor for [`ActuatorWrapper`].
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
impl<T: Settable<TerminalData, E>, E: Clone + Debug> Device<E> for ActuatorWrapper<'_, T, E> {
    fn update_terminals(&mut self) -> NothingOrError<E> {
        self.terminal.borrow_mut().update()?;
        Ok(())
    }
}
impl<T: Settable<TerminalData, E>, E: Clone + Debug> Updatable<E> for ActuatorWrapper<'_, T, E> {
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
///Connect a [`Getter<State, E>`] to a [`Terminal<E>`] for use as an encoder in the device system.
pub struct GetterStateDeviceWrapper<'a, T: Getter<State, E>, E: Clone + Debug> {
    inner: T,
    terminal: RefCell<Terminal<'a, E>>,
}
impl<'a, T: Getter<State, E>, E: Clone + Debug> GetterStateDeviceWrapper<'a, T, E> {
    ///Constructor for [`GetterStateDeviceWrapper`].
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
impl<T: Getter<State, E>, E: Clone + Debug> Device<E> for GetterStateDeviceWrapper<'_, T, E> {
    fn update_terminals(&mut self) -> NothingOrError<E> {
        self.terminal.borrow_mut().update()?;
        Ok(())
    }
}
impl<T: Getter<State, E>, E: Clone + Debug> Updatable<E> for GetterStateDeviceWrapper<'_, T, E> {
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
//TODO: Hopefully figure out a way to make this less alloc-y.
///Connect a [`Settable<f32, E>`] motor to the device system through a
///[`CommandPID`](streams::control::CommandPID). See
///[`streams::control::CommandPID`] documentation for more information about how this works.
#[cfg(feature = "alloc")]
pub struct PIDWrapper<'a, T: Settable<f32, E>, E: Clone + Debug + 'static> {
    terminal: RefCell<Terminal<'a, E>>,
    time: Rc<RefCell<Time>>,
    state: Rc<RefCell<ConstantGetter<State, Rc<RefCell<Time>>, E>>>,
    command: Rc<RefCell<ConstantGetter<Command, Rc<RefCell<Time>>, E>>>,
    feeder: Feeder<
        f32,
        streams::control::CommandPID<Rc<RefCell<ConstantGetter<State, Rc<RefCell<Time>>, E>>>, E>,
        T,
        E,
    >,
}
#[cfg(feature = "alloc")]
impl<'a, T: Settable<f32, E>, E: Clone + Debug + 'static> PIDWrapper<'a, T, E> {
    ///Constructor for [`PIDWrapper`].
    pub fn new(
        inner: T,
        initial_time: Time,
        initial_state: State,
        initial_command: Command,
        kvalues: PositionDerivativeDependentPIDKValues,
    ) -> Self {
        let terminal = Terminal::new();
        let time = Rc::new(RefCell::new(initial_time));
        let state = Rc::new(RefCell::new(ConstantGetter::new(
            time.clone(),
            initial_state,
        )));
        let command = Rc::new(RefCell::new(ConstantGetter::new(
            time.clone(),
            initial_command,
        )));
        let pid = streams::control::CommandPID::new(state.clone(), initial_command, kvalues);
        let feeder = Feeder::new(pid, inner);
        Self {
            terminal: terminal,
            time: time,
            state: state,
            command: command,
            feeder: feeder,
        }
    }
    ///Get a reference to this wrapper's terminal.
    pub fn get_terminal(&self) -> &'a RefCell<Terminal<'a, E>> {
        unsafe { &*(&self.terminal as *const RefCell<Terminal<'a, E>>) }
    }
}
#[cfg(feature = "alloc")]
impl<T: Settable<f32, E>, E: Clone + Debug + 'static> Device<E> for PIDWrapper<'_, T, E> {
    fn update_terminals(&mut self) -> NothingOrError<E> {
        self.terminal.borrow_mut().update()?;
        Ok(())
    }
}
#[cfg(feature = "alloc")]
impl<T: Settable<f32, E>, E: Clone + Debug + 'static> Updatable<E> for PIDWrapper<'_, T, E> {
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
            }
            None => (),
        }
        self.feeder.update()?;
        Ok(())
    }
}
