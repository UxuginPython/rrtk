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
    pub fn new() -> Self {
        let id = unsafe { NEXT_SYSTEM_ID };
        unsafe {
            NEXT_SYSTEM_ID += 1;
        }
        Self {
            terminals: [MaybeTerminal::default(); N],
            global_id: id,
        }
    }
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
    pub fn release_terminal(&mut self, id: TerminalID) {
        assert_eq!(
            self.global_id, id.system,
            "This terminal is not a part of this system."
        );
        self.terminals[id.terminal] = MaybeTerminal::Uninitialized;
    }
    pub fn connect_terminals(&mut self, id_a: TerminalID, id_b: TerminalID) {
        assert_eq!(
            self.global_id, id_a.system,
            "id_a terminal is not a part of this system."
        );
        assert_eq!(
            self.global_id, id_b.system,
            "id_b terminal is not a part of this system."
        );
        if id_a.terminal > id_b.terminal {
            self.terminals[id_a.terminal] = MaybeTerminal::Connected(id_b.terminal);
        } else if id_b.terminal > id_a.terminal {
            self.terminals[id_b.terminal] = MaybeTerminal::Connected(id_a.terminal);
        }
    }
}
