#![allow(missing_docs)]
use super::*;
type SystemID = u16;
type LocalNodeID = usize;
static mut NEXT_SYSTEM_ID: SystemID = 0;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
    state_local: Option<AngularState>,
}
impl Node {
    pub const fn new() -> Self {
        Self {
            prev: None,
            next: None,
            state_local: None,
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
            system_id,
            nodes: [const { None }; N],
        }
    }
    #[inline]
    pub const fn contains(&self, node_id: NodeID) -> bool {
        self.system_id == node_id.system
    }
    #[inline]
    const fn assert_contains(&self, node_id: NodeID) -> LocalNodeID {
        assert!(
            self.contains(node_id),
            "rrtk System does not contain provided node"
        );
        node_id.node
    }
    pub const fn get_state_local(&self, node_id: NodeID) -> Option<AngularState> {
        let node_id = self.assert_contains(node_id);
        if let Some(node) = &self.nodes[node_id] {
            node.state_local
        } else {
            //TODO: Should this have a more specific panic message?
            panic!("rrtk System invariant violated");
        }
    }
    pub const fn set_state_local(&mut self, node_id: NodeID, state: Option<AngularState>) {
        let node_id = self.assert_contains(node_id);
        if let Some(ref mut node) = self.nodes[node_id] {
            node.state_local = state;
        } else {
            panic!("rrtk System invariant violated");
        }
    }
    //TODO: Decide about #[inline] for this, get_state_connected, and get_state_true.
    fn get_average_state_over_iterator<I: Iterator<Item = LocalNodeID>>(
        &self,
        iterator: I,
    ) -> Option<AngularState> {
        let mut contributing = 0u16;
        let mut state = AngularState::ZERO;
        for node_id in iterator {
            if let Some(node) = &self.nodes[node_id] {
                if let Some(state_local) = node.state_local {
                    state += state_local;
                    contributing += 1;
                }
            } else {
                panic!("rrtk System invariant violated");
            }
        }
        if contributing >= 1 {
            Some(state / Dimensionless::new(contributing as f32))
        } else {
            None
        }
    }
    ///Use this in calculations (as opposed to get_state_local or get_state_true).
    pub fn get_state_connected(&self, node_id: NodeID) -> Option<AngularState> {
        let node_id = self.assert_contains(node_id);
        self.get_average_state_over_iterator(self.iter_connected(node_id))
    }
    pub fn get_state_true(&self, node_id: NodeID) -> Option<AngularState> {
        let node_id = self.assert_contains(node_id);
        self.get_average_state_over_iterator(
            self.iter_connected(node_id)
                .chain(core::iter::once(node_id)),
        )
    }
    pub const fn new_node(&mut self) -> Option<NodeID> {
        //A for loop over 0..N that works in a const context.
        let mut i = 0usize;
        while i < N {
            if self.nodes[i].is_none() {
                self.nodes[i] = Some(Node::new());
                return Some(NodeID::new(self.system_id, i));
            }
            i += 1;
        }
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
        let node_a_id = self.assert_contains(node_a_id);
        let node_b_id = self.assert_contains(node_b_id);
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
        let node_id = self.assert_contains(node_id);
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
///This iterator intentionally excludes the head node.
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
    fn size_hint(&self) -> (usize, Option<usize>) {
        if !matches!(self.state, ConnectedIteratorState::Done) {
            (1, None)
        } else {
            (0, Some(0))
        }
    }
}
#[cfg(test)]
mod tests {
    #![allow(unused)]
    use super::*;
    #[test]
    fn connected_iterator() {
        let mut system = System::<6>::new();
        let [n0, n1, n2, n3, n4, n5] = [
            system.new_node().unwrap(),
            system.new_node().unwrap(),
            system.new_node().unwrap(),
            system.new_node().unwrap(),
            system.new_node().unwrap(),
            system.new_node().unwrap(),
        ];
        system.connect(n1, n3);
        system.connect(n4, n3);
        let mut iter = system.iter_connected(3);
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(4));
        assert_eq!(iter.next(), None);
    }
    #[test]
    fn state_connected() {
        let mut system = System::<6>::new();
        let [n0, n1, n2, n3, n4, n5] = [
            system.new_node().unwrap(),
            system.new_node().unwrap(),
            system.new_node().unwrap(),
            system.new_node().unwrap(),
            system.new_node().unwrap(),
            system.new_node().unwrap(),
        ];
        system.connect(n1, n3);
        system.connect(n3, n2);
        system.connect(n4, n1);
        system.set_state_local(
            n2,
            Some(AngularState::new(
                Dimensionless::new(3.0),
                InverseSecond::new(9.0),
                InverseSecondSquared::new(1.0),
            )),
        );
        system.set_state_local(
            n3,
            Some(AngularState::new(
                Dimensionless::new(3.0),
                InverseSecond::new(1.0),
                InverseSecondSquared::new(3.0),
            )),
        );
        system.set_state_local(
            n4,
            Some(AngularState::new(
                Dimensionless::new(9.0),
                InverseSecond::new(1.0),
                InverseSecondSquared::new(3.0),
            )),
        );
        assert_eq!(
            system.get_state_connected(n3),
            Some(AngularState::new(
                Dimensionless::new(6.0),
                InverseSecond::new(5.0),
                InverseSecondSquared::new(2.0)
            ))
        );
    }
}
