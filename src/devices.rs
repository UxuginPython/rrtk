// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2024-2026 UxuginPython
//!A graph-based system for tracking the rotational states of mechanical components throughout your
//!robot.
//!
//!The system uses a set of *devices*, each of which is allowed to read and write to a specific set
//!of *nodes*. Nodes, their states, and connections between them are managed by the [`System`];
//!devices read and write to nodes using [`NodeID`]s provided by it. `NodeID`s issued by a `System`
//!must only be used with that same `System`. Using `NodeID`s with other `System`s will often
//!result in panics.
//!
//!Two devices should never read or write to the same node. Instead, each device should have its
//!own node, and those nodes should be connected through the `System`. This avoids issues such as
//!nodes oscillating between the states being written by different devices.
//!
//!For more information, see the "devices" example.
use super::*;
pub mod provided;
pub mod wrappers;
type SystemID = u16;
type LocalNodeID = usize;
static mut NEXT_SYSTEM_ID: SystemID = 0;
///A unique identifier for a node of a [`System`]. `NodeID` is used to interact with nodes through
///the `System`.
///
///The internal value is not accessible. `NodeID` can only be constructed by [`System::new_node`];
///it cannot be constructed directly.
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
    ///Checks whether two `NodeID`s were issued by the same [`System`]. This is different from the
    ///`PartialEq` implementation, which also checks if the `NodeID`s refer to the same node.
    #[inline]
    pub const fn same_system(&self, other: Self) -> bool {
        self.system == other.system
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
enum Previous {
    PreviousNode(LocalNodeID),
    BeginningNoCommand,
    BeginningWithCommand(AngularCommand),
}
struct Node {
    prev: Previous,
    next: Option<LocalNodeID>,
    state_local: Option<AngularState>,
}
impl Node {
    pub const fn new() -> Self {
        Self {
            prev: Previous::BeginningNoCommand,
            next: None,
            state_local: None,
        }
    }
}
///A struct that tracks the states of axles throughout your robot. It is based on a system of nodes
///that can be connected. `N` is the maximum number of nodes. The structure is implemented using a
///form of doubly linked list.
///
///See the [module documentation](self) for more information.
pub struct System<const N: usize> {
    system_id: SystemID,
    nodes: [Option<Node>; N],
}
impl<const N: usize> System<N> {
    ///Constructor.
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
    ///Returns true only if this system contains the provided node.
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
    #[inline]
    const fn node_ref_from_local_id(&self, node_id: LocalNodeID) -> &Node {
        if let Some(node) = &self.nodes[node_id] {
            node
        } else {
            panic!("rrtk System invariant violated");
        }
    }
    #[inline]
    const fn node_mut_from_local_id(&mut self, node_id: LocalNodeID) -> &mut Node {
        if let Some(ref mut node) = self.nodes[node_id] {
            node
        } else {
            panic!("rrtk System invariant violated");
        }
    }
    ///Returns the last state a node has been directly set to using
    ///[`set_state_local`](Self::set_state_local). This does not account for connected nodes.
    pub const fn get_state_local(&self, node_id: NodeID) -> Option<AngularState> {
        let node_id = self.assert_contains(node_id);
        let node = self.node_ref_from_local_id(node_id);
        node.state_local
    }
    ///Set the state of a node. The set value can be accessed by [`get_state_local`](Self::get_state_local).
    pub const fn set_state_local(&mut self, node_id: NodeID, state: Option<AngularState>) {
        let node_id = self.assert_contains(node_id);
        let node = self.node_mut_from_local_id(node_id);
        node.state_local = state;
    }
    fn get_average_state_over_iterator<I: Iterator<Item = LocalNodeID>>(
        &self,
        iterator: I,
    ) -> Option<AngularState> {
        let mut contributing = 0u16;
        let mut state = AngularState::ZERO;
        for node_id in iterator {
            let node = self.node_ref_from_local_id(node_id);
            if let Some(state_local) = node.state_local {
                state += state_local;
                contributing += 1;
            }
        }
        if contributing >= 1 {
            Some(state / Dimensionless::new(contributing as f32))
        } else {
            None
        }
    }
    ///Returns the average state of all nodes connected to the provided node, **excluding** the
    ///provided node itself. To avoid feedback loops, this is the recommended function to use in
    ///your calculations (as opposed to [`get_state_local`](Self::get_state_local) or
    ///[`get_state_true`](Self::get_state_true)).
    #[inline] //Inlining this function itself; not inlining get_average_state_over_iterator.
    pub fn get_state_connected(&self, node_id: NodeID) -> Option<AngularState> {
        let node_id = self.assert_contains(node_id);
        self.get_average_state_over_iterator(self.iter_connected(node_id))
    }
    ///Returns the average state of all nodes connected to the provided node, **including** the
    ///provided node itself. This value is mostly useful when displaying information and should
    ///generally not be used directly in calculations due to feedback loops.
    ///[`get_state_connected`](Self::get_state_connected) is recommended instead to avoid this
    ///issue.
    #[inline] //same as get_state_connected
    pub fn get_state_true(&self, node_id: NodeID) -> Option<AngularState> {
        let node_id = self.assert_contains(node_id);
        self.get_average_state_over_iterator(
            self.iter_connected(node_id)
                .chain(core::iter::once(node_id)),
        )
    }
    ///Returns the ID for a new node if the `System` has capacity for one.
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
            let node = self.node_ref_from_local_id(node_id);
            if let Previous::PreviousNode(prev_id) = node.prev {
                node_id = prev_id;
            } else {
                break;
            }
        }
        node_id
    }
    const fn end(&self, node_id: LocalNodeID) -> LocalNodeID {
        let mut node_id = node_id;
        loop {
            let node = self.node_ref_from_local_id(node_id);
            if let Some(next_id) = node.next {
                node_id = next_id;
            } else {
                break;
            }
        }
        node_id
    }
    #[inline]
    fn iter_connected(&self, node_id: LocalNodeID) -> ConnectedIterator<'_, N> {
        ConnectedIterator::new(self, node_id)
    }
    ///Connects two nodes. Connections between nodes are transitive (i.e. if A is connected to B and
    ///B is connected to C then A is connected to C) and bidirectional.
    ///
    ///If both Node A and Node B have received a Command, Node A's command is kept. Except for that,
    ///the order of the nodes has no effect.
    pub const fn connect(&mut self, node_a_id: NodeID, node_b_id: NodeID) {
        let node_a_id = self.assert_contains(node_a_id);
        let node_b_id = self.assert_contains(node_b_id);
        let a_end_id = self.end(node_a_id);
        let b_beginning_id = self.beginning(node_b_id);
        let a_end = self.node_mut_from_local_id(a_end_id);
        a_end.next = Some(b_beginning_id);
        let b_beginning = self.node_mut_from_local_id(b_beginning_id);
        b_beginning.prev = Previous::PreviousNode(a_end_id);
    }
    ///Disconnects a node from all other nodes connected to it. Connected nodes will stay connected
    ///to eachother. (e.g. if A is connected to B and B is connected to C, A will stay connected to
    ///C if B is disconnected.)
    pub const fn disconnect(&mut self, node_id: NodeID) {
        let node_id = self.assert_contains(node_id);
        let node = self.node_ref_from_local_id(node_id);
        let previous = node.prev;
        let maybe_next_id = node.next;
        if let Previous::PreviousNode(prev_id) = previous {
            let prev = self.node_mut_from_local_id(prev_id);
            prev.next = maybe_next_id;
        }
        if let Some(next_id) = maybe_next_id {
            let next = self.node_mut_from_local_id(next_id);
            //This can carry other nothing, the new previous node, or the command.
            next.prev = previous;
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
                let to_return_node = self.system.node_ref_from_local_id(to_return);
                if let Some(next_to_return) = to_return_node.next {
                    self.node_to_return = next_to_return;
                } else {
                    //Basically the same thing as in the constructor. Set it to go backward,
                    //set node_to_return to the head node, and then skip it.
                    self.state = ConnectedIteratorState::Backward;
                    self.node_to_return = self.head_node;
                    self.next();
                }
                Some(to_return)
            }
            ConnectedIteratorState::Backward => {
                let to_return = self.node_to_return;
                let to_return_node = self.system.node_ref_from_local_id(to_return);
                if let Previous::PreviousNode(next_to_return) = to_return_node.prev {
                    self.node_to_return = next_to_return;
                } else {
                    self.state = ConnectedIteratorState::Done;
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
///Very similar to the [`Updatable`] trait except that it requires a mutable reference to the
///system controlling the device's nodes.
pub trait DeviceUpdatable<E> {
    ///Update the states of the device's terminals based on the device's mechanical constraints.
    fn device_update<const N: usize>(&mut self, system: &mut System<N>) -> NothingOrError<E>;
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
                Dimensionless::new(6.0),        // (3 + 9) / 2
                InverseSecond::new(5.0),        // (9 + 1) / 2
                InverseSecondSquared::new(2.0)  // (1 + 3) / 2
            ))
        );
    }
}
