#![allow(missing_docs)]
type SystemID = u16;
type LocalNodeID = usize;
static mut NEXT_SYSTEM_ID: SystemID = 0;
pub struct NodeID {
    system: SystemID,
    node: LocalNodeID,
}
pub struct Node {
    prev: Option<LocalNodeID>,
    next: Option<LocalNodeID>,
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
        if let Some(node) = &self.nodes[node_id] {
            if let Some(prev_id) = node.prev {
                if let Some(ref mut prev) = self.nodes[prev_id] {
                    prev.next = node.next;
                } else {
                    panic!("rrtk System invariant violated");
                }
            }
            if let Some(next_id) = node.next {
                if let Some(ref mut next) = self.nodes[next_id] {
                    next.prev = node.prev;
                } else {
                    panic!("rrtk System invariant violated");
                }
            }
        } else {
            panic!("rrtk System provided invalid NodeID");
        }
    }
}
