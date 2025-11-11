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
}
