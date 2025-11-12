#![allow(missing_docs)]
type SystemID = u16;
type LocalNodeID = usize;
static mut NEXT_SYSTEM_ID: SystemID = 0;
pub struct NodeID {
    system: SystemID,
    node: LocalNodeID,
}
impl NodeID {
    //This is intentionally not pub.
    #[inline]
    const fn new(system: SystemID, node: LocalNodeID) -> Self {
        Self { system, node }
    }
}
struct Node {
    prev: Option<LocalNodeID>,
    next: Option<LocalNodeID>,
}
impl Node {
    pub const fn new() -> Self {
        Self {
            prev: None,
            next: None,
        }
    }
}
//There is a crate that does this, but since it's so simple, we just do it here to avoid the
//mandatory dependency.
macro_rules! const_for {
    (for $i: ident in ($min: expr, $max: expr) => $code: tt) => {
        let mut $i = $min;
        while $i < $max {
            $code
            $i += 1;
        }
    }
}
pub struct System<const N: usize> {
    system_id: SystemID,
    nodes: [Option<Node>; N],
}
impl<const N: usize> System<N> {
    #[inline]
    pub const fn new() -> Self {
        let system_id;
        unsafe {
            system_id = NEXT_SYSTEM_ID;
            NEXT_SYSTEM_ID += 1;
        }
        Self {
            system_id: system_id,
            nodes: [const { None }; N],
        }
    }
    pub const fn new_node(&mut self) -> Option<NodeID> {
        const_for!(for i in (0, N) => {
            if self.nodes[i].is_none() {
                self.nodes[i] = Some(Node::new());
                return Some(NodeID::new(self.system_id, i));
            }
        });
        None
    }
    const fn beginning(&self, node_id: LocalNodeID) -> LocalNodeID {
        let mut node_id = node_id;
        loop {
            if let Some(node) = &self.nodes[node_id] {
                if let Some(prev_id) = node.prev {
                    node_id = prev_id;
                } else {
                    break;
                }
            } else {
                panic!("rrtk System invariant violated");
            }
        }
        node_id
    }
    const fn end(&self, node_id: LocalNodeID) -> LocalNodeID {
        let mut node_id = node_id;
        loop {
            if let Some(node) = &self.nodes[node_id] {
                if let Some(next_id) = node.next {
                    node_id = next_id;
                } else {
                    break;
                }
            } else {
                panic!("rrtk System invariant violated");
            }
        }
        node_id
    }
    #[inline]
    fn iter_connected(&self, node_id: LocalNodeID) -> ConnectedIterator<'_, N> {
        ConnectedIterator::new(self, node_id)
    }
    pub const fn connect(&mut self, node_a_id: NodeID, node_b_id: NodeID) {
        if !(node_a_id.system == self.system_id && node_b_id.system == self.system_id) {
            panic!("rrtk System does not contain provided node");
        }
        let node_a_id = node_a_id.node;
        let node_b_id = node_b_id.node;
        let a_end_id = self.end(node_a_id);
        let b_beginning_id = self.beginning(node_b_id);
        if let Some(ref mut a_end) = self.nodes[a_end_id] {
            a_end.next = Some(b_beginning_id);
        } else {
            panic!("rrtk System invariant violated");
        }
        if let Some(ref mut b_beginning) = self.nodes[b_beginning_id] {
            b_beginning.prev = Some(a_end_id);
        } else {
            panic!("rrtk System invariant violated");
        }
    }
    pub const fn disconnect(&mut self, node_id: NodeID) {
        if node_id.system != self.system_id {
            panic!("rrtk System does not contain provided node");
        }
        let node_id = node_id.node;
        let (maybe_prev_id, maybe_next_id);
        if let Some(node) = &self.nodes[node_id] {
            maybe_prev_id = node.prev;
            maybe_next_id = node.next;
        } else {
            panic!("rrtk System provided invalid NodeID");
        }
        if let Some(prev_id) = maybe_prev_id {
            if let Some(ref mut prev) = self.nodes[prev_id] {
                prev.next = maybe_next_id;
            } else {
                panic!("rrtk System invariant violated");
            }
        }
        if let Some(next_id) = maybe_next_id {
            if let Some(ref mut next) = self.nodes[next_id] {
                next.prev = maybe_prev_id;
            } else {
                panic!("rrtk System invariant violated");
            }
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
enum ConnectedIteratorState {
    Forward,
    Backward,
    Done,
}
struct ConnectedIterator<'a, const N: usize> {
    system: &'a System<N>,
    head_node: LocalNodeID,
    node_to_return: LocalNodeID,
    state: ConnectedIteratorState,
}
impl<'a, const N: usize> ConnectedIterator<'a, N> {
    fn new(system: &'a System<N>, node: LocalNodeID) -> Self {
        //We set node_to_return to the head node and then skip it.
        let mut new_self = Self {
            system,
            head_node: node,
            node_to_return: node,
            state: ConnectedIteratorState::Forward,
        };
        new_self.next();
        new_self
    }
}
impl<const N: usize> Iterator for ConnectedIterator<'_, N> {
    type Item = LocalNodeID;
    fn next(&mut self) -> Option<LocalNodeID> {
        match self.state {
            ConnectedIteratorState::Forward => {
                let to_return = self.node_to_return;
                if let Some(to_return_node) = &self.system.nodes[to_return] {
                    if let Some(next_to_return) = to_return_node.next {
                        self.node_to_return = next_to_return;
                    } else {
                        //Basically the same thing as in the constructor. Set it to go backward,
                        //set node_to_return to the head node, and then skip it.
                        self.state = ConnectedIteratorState::Backward;
                        self.node_to_return = self.head_node;
                        self.next();
                    }
                } else {
                    panic!("rrtk System invariant violated");
                }
                Some(to_return)
            }
            ConnectedIteratorState::Backward => {
                let to_return = self.node_to_return;
                if let Some(to_return_node) = &self.system.nodes[to_return] {
                    if let Some(next_to_return) = to_return_node.prev {
                        self.node_to_return = next_to_return;
                    } else {
                        self.state = ConnectedIteratorState::Done;
                    }
                } else {
                    panic!("rrtk System invariant violated");
                }
                Some(to_return)
            }
            ConnectedIteratorState::Done => None,
        }
    }
}
