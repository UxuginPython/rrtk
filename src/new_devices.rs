// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2024-2025 UxuginPython
//!An experimental replacement for the previous device system (in the `devices` module). This
//!system uses a single struct for each group of devices to store the states at different
//!locations.
//TODO: review this documentation and see if there's anything else you need to say
#![allow(unused)]
use super::*;
use core::mem::MaybeUninit;
//There is a crate that does this, but the implementation is so simple that it is preferable to
//avoid the external dependency.
macro_rules! const_for {
    ($i: ident, $min: expr, $max: expr, $code: tt) => {
        let mut $i = $min;
        while $i < $max {
            $code
            $i += 1;
        }
    }
}
///A global identifier of a terminal, which is a place where two mechanical devices connect.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TerminalID {
    system: u8,
    terminal: usize,
}
struct IIdentifyAsAVec<T, const N: usize> {
    inner: [MaybeUninit<T>; N],
    length: usize,
}
impl<T: Copy, const N: usize> IIdentifyAsAVec<T, N> {
    #[inline]
    const fn new() -> Self {
        Self {
            inner: [MaybeUninit::uninit(); N],
            length: 0,
        }
    }
    #[inline]
    const fn push(&mut self, id: T) {
        if self.length >= N {
            panic!("You overflowed an IIdentifyAsAVec.");
        }
        self.inner[self.length].write(id);
        self.length += 1;
    }
    #[inline]
    const fn get(&self, index: usize) -> T {
        if index >= self.length {
            panic!("This index is out of range.");
        }
        unsafe { self.inner[index].assume_init() }
    }
    #[inline]
    const fn pop(&mut self) -> T {
        if self.length == 0 {
            panic!("You tried to pop from an empty IIdentifyAsAVec.");
        }
        let output = unsafe { self.inner[self.length - 1].assume_init() };
        self.length -= 1;
        output
    }
    #[inline]
    const fn as_array(&self) -> [T; N] {
        if self.length != N {
            panic!("You tried to convert a non-full IIdentifyAsAVec to an array.");
        }
        //core::mem::transmute doesn't work well with const generics, so this does the same thing
        //through pointers instead. This should be changed to use the transpose method if it's ever
        //stabilized.
        unsafe { self.inner.as_ptr().cast::<[T; N]>().read() }
    }
    #[inline]
    const fn len(&self) -> usize {
        self.length
    }
}
impl<const N: usize> IIdentifyAsAVec<TerminalID, N> {
    ///Although this takes `&self` because it's not technically necessary to consume `self`, it is
    ///strongly recommended that you drop all uninitialized `TerminalID`s. They are useless and
    ///weird stuff might happen if you try to use them since the same ID may be reused.
    const fn release_all<const Q: usize>(&self, system: &mut System<Q>) {
        const_for!(i, 0, self.length, {
            system.release_terminal(self.get(i));
        });
    }
}
#[derive(Clone, Copy, PartialEq)]
struct Terminal {
    measurement: Option<Datum<AngularState>>,
    root: Option<usize>,
}
impl Terminal {
    const fn new() -> Self {
        Self {
            measurement: None,
            root: None,
        }
    }
}
#[derive(Clone, Copy, Default, PartialEq)]
enum MaybeTerminal {
    #[default]
    Uninitialized,
    Connected(usize),
    Root(Datum<AngularState>),
}
static mut NEXT_SYSTEM_ID: u8 = 0;
///A collection of terminals used by a set of mechanical devices. `N` is the number of terminals
///the `System` can hold.
pub struct System<const N: usize> {
    terminals: [Option<Terminal>; N],
    global_id: u8,
}
impl<const N: usize> System<N> {
    ///Constructor for `System`.
    pub const fn new() -> Self {
        let id = unsafe { NEXT_SYSTEM_ID };
        unsafe {
            NEXT_SYSTEM_ID += 1;
        }
        Self {
            terminals: [None; N],
            global_id: id,
        }
    }
    ///Get the ID of a terminal not connected to anything if one is available.
    pub const fn initialize_terminal(&mut self) -> Option<TerminalID> {
        const_for!(i, 0, N, {
            if self.terminals[i].is_none() {
                self.terminals[i] = Some(Terminal::new());
                return Some(TerminalID {
                    system: self.global_id,
                    terminal: i,
                });
            }
        });
        None
    }
    ///Check if a terminal is a part of this system.
    #[inline]
    pub const fn has(&self, id: TerminalID) -> bool {
        self.global_id == id.system
    }
    #[inline]
    const fn verify_terminal_id(&self, id: TerminalID) {
        assert!(self.has(id), "This terminal is not a part of this system.");
    }
    #[inline]
    const fn get_root(&self, index: usize) -> usize {
        if let Some(root) = self.terminals[index].unwrap().root {
            //This checks that:
            //1. the root terminal is initialized (unwrap()), and
            //2. it itself does not have a root (asserting is_none()).
            //Root terminals should never have roots themselves.
            debug_assert!(self.terminals[root].unwrap().root.is_none());
            root
        } else {
            index
        }
    }
    //XXX: Should this just take the index rather than a TerminalID?
    const fn get_connected(&self, id: TerminalID) -> IIdentifyAsAVec<usize, N> {
        let root = self.get_root(id.terminal);
        let mut output = IIdentifyAsAVec::new();
        output.push(root);
        const_for!(i, 0, N, {
            if let Some(terminal) = self.terminals[i]
                && let Some(rooot) = terminal.root
                && root == rooot
            {
                output.push(i);
            }
        });
        output
    }
    ///Disconnect a terminal from every other terminal and allow it to be claimed again by
    ///[`initialize_terminal`](Self::initialize_terminal).
    pub const fn release_terminal(&mut self, id: TerminalID) {
        self.verify_terminal_id(id);
        if self.terminals[id.terminal]
            //XXX: Should this really panic or just return? (I made it panic initially just for
            //convenience. The MaybeTerminal version did not.)
            .expect("You tried to release an already released terminal.")
            .root
            .is_none()
        {
            let connected = self.get_connected(id);
            let new_root = connected.get(1);
            self.terminals[new_root].unwrap().root = None;
            const_for!(i, 2, connected.len(), {
                self.terminals[connected.get(i)].unwrap().root = Some(i);
            });
        }
        self.terminals[id.terminal] = None;
    }
    ///Connect two terminals together.
    pub const fn connect_terminals(&mut self, id_a: TerminalID, id_b: TerminalID) {
        self.verify_terminal_id(id_a);
        self.verify_terminal_id(id_b);
        let a_connected = self.get_connected(id_a);
        let b_connected = self.get_connected(id_b);
        let a_root = a_connected.get(0);
        let b_root = b_connected.get(0);
        if a_root > b_root {
            const_for!(i, 0, a_connected.len(), {
                self.terminals[a_connected.get(i)].unwrap().root = Some(b_root);
            });
        } else {
            const_for!(i, 0, b_connected.len(), {
                self.terminals[b_connected.get(i)].unwrap().root = Some(a_root);
            });
        }
    }
    ///Get the current [`AngularState`] of a terminal if it is known along with a timestamp.
    pub const fn get_terminal_state(&self, id: TerminalID) -> Option<Datum<AngularState>> {
        self.verify_terminal_id(id);
        let connected = self.get_connected(id);
        let mut state = Datum::new(Time::ZERO, AngularState::ZERO);
        let mut contributing = 0u8;
        const_for!(i, 0, connected.len(), {
            if let Some(addend_state) = self.terminals[connected.get(i)].unwrap().measurement {
                //This entire statement is the const equivalent of `state += addend_state`.
                state = Datum::new(
                    Time::from_nanoseconds(
                        if state.time.as_nanoseconds() > addend_state.time.as_nanoseconds() {
                            state.time.as_nanoseconds()
                        } else {
                            addend_state.time.as_nanoseconds()
                        },
                    ),
                    state.value.add_const(addend_state.value),
                );
                contributing += 1;
            }
        });
        let contributing_f32 = contributing as f32;
        //This entire statement is the const equivalent of
        //`state /= Dimensionless::new(contributing_f32)`
        state = Datum::new(
            state.time,
            AngularState::new(
                Dimensionless::new(state.value.position.2 / contributing_f32),
                InverseSecond::new(state.value.velocity.2 / contributing_f32),
                InverseSecondSquared::new(state.value.acceleration.2 / contributing_f32),
            ),
        );
        if contributing >= 1 { Some(state) } else { None }
    }
    ///Set the current state of a terminal including a timestamp.
    pub const fn set_terminal_state(&mut self, id: TerminalID, state: Datum<AngularState>) {
        self.verify_terminal_id(id);
        //unwrap does not work with mutating.
        if let Some(ref mut terminal) = self.terminals[id.terminal] {
            terminal.measurement = Some(state);
        } else {
            panic!();
        }
        self.terminals[id.terminal].unwrap().measurement = Some(state);
    }
    ///Returns an iterator returning uninitialized terminals until there are none remaining.
    pub const fn iter(&mut self) -> SystemIter<'_, N> {
        SystemIter {
            //self is an &mut reference.
            system: self,
        }
    }
    ///Returns `Some` if and only if all `Q` terminals were successfully initialized.
    pub const fn initialize_multiple_terminals<const Q: usize>(
        &mut self,
    ) -> Option<[TerminalID; Q]> {
        let mut ids = IIdentifyAsAVec::<TerminalID, Q>::new();
        const_for!(i, 0, Q, {
            if let Some(id) = self.initialize_terminal() {
                ids.push(id);
            } else {
                ids.release_all(self);
                return None;
            }
        });
        Some(ids.as_array())
    }
}
///Iterator returning uninitialized from a [`System`] terminals until there are none remaining.
///Constructed with [`System::iter`].
pub struct SystemIter<'a, const N: usize> {
    system: &'a mut System<N>,
}
impl<const N: usize> Iterator for SystemIter<'_, N> {
    type Item = TerminalID;
    fn next(&mut self) -> Option<TerminalID> {
        self.system.initialize_terminal()
    }
}
///This is a replacement for the [`Updatable`] trait that can be used by devices in a system. Since
///devices typically need mutable access to their system when updating, this provides that access.
pub trait DeviceUpdatable<E> {
    ///Update the device. After this method is called, the terminals' states should be mechanically
    ///valid, e.g., geared terminals maintain their ratio.
    fn update_device<const N: usize>(&mut self, system: &mut System<N>) -> NothingOrError<E>;
}
///This is a very basic proof of concept. Do not actually use it yet. All devices from the old
///system will be migrated before the stable release.
pub struct Differential {
    ///The terminal of one side of the differential.
    pub side_a: TerminalID,
    ///The terminal of the other side of the differential.
    pub side_b: TerminalID,
    ///The terminal of the side of the differential which adds the states of the two other sides.
    pub sum_side: TerminalID,
}
impl Differential {
    ///Constructor for `Differential`.
    pub const fn new<const N: usize>(system: &mut System<N>) -> Option<Self> {
        let terminals = if let Some(terminals) = system.initialize_multiple_terminals::<3>() {
            terminals
        } else {
            return None;
        };
        let [side_a, side_b, sum_side] = terminals;
        Some(Self {
            side_a,
            side_b,
            sum_side,
        })
    }
}
impl DeviceUpdatable<core::convert::Infallible> for Differential {
    fn update_device<const N: usize>(
        &mut self,
        system: &mut System<N>,
    ) -> NothingOrError<core::convert::Infallible> {
        //This is a pretty bad way of doing this.
        if let Some(a_state) = system.get_terminal_state(self.side_a)
            && let Some(b_state) = system.get_terminal_state(self.side_b)
        {
            system.set_terminal_state(self.sum_side, a_state + b_state);
        }
        Ok(())
    }
}
