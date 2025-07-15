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
#[derive(Clone, Copy, PartialEq, Eq)]
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
/*impl<const N: usize> IIdentifyAsAVec<TerminalID, N> {
    ///Although this takes `&self` because it's not technically necessary to consume `self`, it is
    ///strongly recommended that you drop all uninitialized `TerminalID`s. They are useless and
    ///weird stuff might happen if you try to use them since the same ID may be reused.
    const fn release_all<const Q: usize>(&self, system: &mut System<Q>) {
        const_for!(i, 0, self.length, {
            system.release_terminal(self.get(i));
        });
    }
}*/
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
pub struct System<const N: usize> {
    terminals: [Option<Terminal>; N],
    global_id: u8,
}
impl<const N: usize> System<N> {
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
    //It would be very much preferable to write this with
    //```
    //for (i, maybe_terminal) in self.terminals.iter_mut().enumerate()
    //```
    //but that does not currently work in const. The same is true for using AngularState::default()
    //instead of constructing it like this.
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
    #[inline]
    pub const fn has(&self, id: TerminalID) -> bool {
        self.global_id == id.system
    }
    #[inline]
    const fn verify_terminal_id(&self, id: TerminalID) {
        assert!(self.has(id), "This terminal is not a part of this system.");
    }
    /*const fn get_connected(&self, id: TerminalID) -> IIdentifyAsAVec<usize, N> {
        let root = self.get_root(id);
        let mut output = IIdentifyAsAVec::new();
        output.push(root);
        const_for!(i, 0, N, {
            if let MaybeTerminal::Connected(rooot) = self.terminals[i]
                && root == rooot
            {
                output.push(i);
            }
        });
        output
    }
    pub const fn release_terminal(&mut self, id: TerminalID) {
        self.verify_terminal_id(id);
        if let MaybeTerminal::Root(state) = self.terminals[id.terminal] {
            let connected = self.get_connected(id);
            let new_root = connected.get(1);
            self.terminals[new_root] = MaybeTerminal::Root(state);
            const_for!(i, 2, connected.len(), {
                self.terminals[connected.get(i)] = MaybeTerminal::Connected(new_root);
            });
        }
        self.terminals[id.terminal] = MaybeTerminal::Uninitialized;
    }
    pub const fn connect_terminals(&mut self, id_a: TerminalID, id_b: TerminalID) {
        self.verify_terminal_id(id_a);
        self.verify_terminal_id(id_b);
        let a_connected = self.get_connected(id_a);
        let b_connected = self.get_connected(id_b);
        let a_root = a_connected.get(0);
        let b_root = b_connected.get(0);
        if a_root > b_root {
            const_for!(i, 0, a_connected.len(), {
                self.terminals[a_connected.get(i)] = MaybeTerminal::Connected(b_root);
            });
        } else {
            const_for!(i, 0, b_connected.len(), {
                self.terminals[b_connected.get(i)] = MaybeTerminal::Connected(a_root);
            });
        }
    }
    const fn get_root(&self, id: TerminalID) -> usize {
        //This is a private method, so we don't verify_terminal_id to improve performance.
        let mut eventually_root = id.terminal;
        loop {
            match self.terminals[eventually_root] {
                MaybeTerminal::Root(_) => break,
                MaybeTerminal::Connected(connected_index) => eventually_root = connected_index,
                MaybeTerminal::Uninitialized => panic!("This terminal has been released."),
            }
        }
        eventually_root
    }
    pub const fn get_terminal_state(&self, id: TerminalID) -> Datum<AngularState> {
        self.verify_terminal_id(id);
        let root = self.get_root(id);
        if let MaybeTerminal::Root(state) = self.terminals[root] {
            state
        } else {
            panic!();
        }
    }
    pub const fn set_terminal_state(&mut self, id: TerminalID, state: Datum<AngularState>) {
        self.verify_terminal_id(id);
        let root = self.get_root(id);
        if let MaybeTerminal::Root(ref mut terminal_state) = self.terminals[root] {
            //Time PartialOrd does not work in const fn, but i64 PartialOrd does.
            if terminal_state.time.as_nanoseconds() < state.time.as_nanoseconds() {
                *terminal_state = state;
            }
        } else {
            panic!();
        }
    }
    pub const fn iter(&mut self) -> SystemIter<N> {
        SystemIter {
            //self is an &mut reference.
            system: self,
        }
    }
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
    }*/
}
/*pub struct SystemIter<'a, const N: usize> {
    system: &'a mut System<N>,
}
impl<const N: usize> Iterator for SystemIter<'_, N> {
    type Item = TerminalID;
    fn next(&mut self) -> Option<TerminalID> {
        self.system.initialize_terminal()
    }
}
pub trait DeviceUpdatable<E> {
    fn update_device<const N: usize>(&mut self, system: &mut System<N>) -> NothingOrError<E>;
}
pub struct Differential {
    pub side_a: TerminalID,
    pub side_b: TerminalID,
    pub sum_side: TerminalID,
}
impl Differential {
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
        system.set_terminal_state(
            self.sum_side,
            system.get_terminal_state(self.side_a) + system.get_terminal_state(self.side_b),
        );
        Ok(())
    }
}*/
