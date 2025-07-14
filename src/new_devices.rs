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
struct IIdentifyAsAVec<const N: usize> {
    inner: [MaybeUninit<TerminalID>; N],
    length: usize,
}
impl<const N: usize> IIdentifyAsAVec<N> {
    fn new() -> Self {
        Self {
            inner: [MaybeUninit::uninit(); N],
            length: 0,
        }
    }
    fn push(&mut self, id: TerminalID) {
        if self.length >= N {
            panic!("You overflowed an IIdentifyAsAVec.");
        }
        self.inner[self.length].write(id);
        self.length += 1;
    }
    fn get(&self, index: usize) -> TerminalID {
        if index >= self.length {
            panic!("This index is out of range.");
        }
        unsafe { self.inner[index].assume_init() }
    }
    fn pop(&mut self) -> TerminalID {
        if self.length == 0 {
            panic!("You tried to pop from an empty IIdentifyAsAVec.");
        }
        let output = unsafe { self.inner[self.length - 1].assume_init() };
        self.length -= 1;
        output
    }
    fn as_array(&self) -> [TerminalID; N] {
        if self.length != N {
            panic!("You tried to convert a non-full IIdentifyAsAVec to an array.");
        }
        unsafe { self.inner.as_ptr().cast::<[TerminalID; N]>().read() }
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
    terminals: [MaybeTerminal; N],
    global_id: u8,
}
impl<const N: usize> System<N> {
    pub const fn new() -> Self {
        let id = unsafe { NEXT_SYSTEM_ID };
        unsafe {
            NEXT_SYSTEM_ID += 1;
        }
        Self {
            terminals: [MaybeTerminal::Uninitialized; N],
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
            if let MaybeTerminal::Uninitialized = self.terminals[i] {
                self.terminals[i] = MaybeTerminal::Root(Datum::new(
                    Time::ZERO,
                    AngularState::new(
                        Dimensionless::new(0.0),
                        InverseSecond::new(0.0),
                        InverseSecondSquared::new(0.0),
                    ),
                ));
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
    pub const fn release_terminal(&mut self, id: TerminalID) {
        self.verify_terminal_id(id);
        self.terminals[id.terminal] = MaybeTerminal::Uninitialized;
    }
    pub const fn connect_terminals(&mut self, id_a: TerminalID, id_b: TerminalID) {
        self.verify_terminal_id(id_a);
        self.verify_terminal_id(id_b);
        if id_a.terminal > id_b.terminal {
            self.terminals[id_a.terminal] = MaybeTerminal::Connected(id_b.terminal);
        } else if id_b.terminal > id_a.terminal {
            self.terminals[id_b.terminal] = MaybeTerminal::Connected(id_a.terminal);
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
        let mut ids = [core::mem::MaybeUninit::uninit(); Q];
        const_for!(i, 0, Q, {
            if let Some(id) = self.initialize_terminal() {
                ids[i].write(id);
            } else {
                const_for!(j, 0, i, {
                    self.release_terminal(unsafe { ids[j].assume_init() });
                });
                return None;
            }
        });
        //core::mem::transmute doesn't work well with const generics, so this does the same thing
        //through pointers instead. This should be changed to use the transpose method if it's ever
        //stabilized.
        Some(unsafe { ids.as_ptr().cast::<[TerminalID; Q]>().read() })
    }
}
pub struct SystemIter<'a, const N: usize> {
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
