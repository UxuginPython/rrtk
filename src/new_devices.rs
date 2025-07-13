use super::*;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct TerminalID {
    system: u8,
    terminal: usize,
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
    //This could probably be const if you enumerated manually.
    pub fn initialize_terminal(&mut self) -> Option<TerminalID> {
        for (i, maybe_terminal) in self.terminals.iter_mut().enumerate() {
            if *maybe_terminal == MaybeTerminal::Uninitialized {
                *maybe_terminal =
                    MaybeTerminal::Root(Datum::new(Time::ZERO, AngularState::default()));
                return Some(TerminalID {
                    system: self.global_id,
                    terminal: i,
                });
            }
        }
        None
    }
    #[inline]
    fn verify_terminal_id(&self, id: TerminalID) {
        assert_eq!(
            self.global_id, id.system,
            "This terminal is not a part of this system."
        );
    }
    pub fn release_terminal(&mut self, id: TerminalID) {
        self.verify_terminal_id(id);
        self.terminals[id.terminal] = MaybeTerminal::Uninitialized;
    }
    pub fn connect_terminals(&mut self, id_a: TerminalID, id_b: TerminalID) {
        self.verify_terminal_id(id_a);
        self.verify_terminal_id(id_b);
        if id_a.terminal > id_b.terminal {
            self.terminals[id_a.terminal] = MaybeTerminal::Connected(id_b.terminal);
        } else if id_b.terminal > id_a.terminal {
            self.terminals[id_b.terminal] = MaybeTerminal::Connected(id_a.terminal);
        }
    }
    pub fn set_terminal_state(&mut self, id: TerminalID, state: Datum<AngularState>) {
        self.verify_terminal_id(id);
        let mut to_set_id = id.terminal;
        loop {
            match self.terminals[to_set_id] {
                MaybeTerminal::Root(ref mut terminal_state) => {
                    *terminal_state = state;
                    break;
                }
                MaybeTerminal::Connected(connected_id) => to_set_id = connected_id,
                MaybeTerminal::Uninitialized => panic!(
                    "This terminal is uninitialized or is connected to an uninitialized terminal."
                ),
            }
        }
    }
}
