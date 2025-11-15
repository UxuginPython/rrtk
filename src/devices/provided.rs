use super::*;
pub struct Clutch {
    connected: bool,
    node_a: NodeID,
    node_b: NodeID,
}
impl Clutch {
    #[inline]
    pub const fn new(node_a: NodeID, node_b: NodeID) -> Self {
        Self {
            connected: false,
            node_a,
            node_b,
        }
    }
    #[inline]
    pub const fn set_connected(&mut self, value: bool) {
        self.connected = value;
    }
}
impl DeviceUpdatable for Clutch {
    fn device_update<const N: usize>(&mut self, system: &mut System<N>) {
        if self.connected {
            system.set_state_local(self.node_a, system.get_state_connected(self.node_b));
            system.set_state_local(self.node_b, system.get_state_connected(self.node_a));
        } else {
            system.set_state_local(self.node_a, None);
            system.set_state_local(self.node_b, None);
        }
    }
}
pub struct Differential {
    node_left: NodeID,
    node_right: NodeID,
    node_sum: NodeID,
}
impl Differential {
    #[inline]
    pub const fn new(node_left: NodeID, node_right: NodeID, node_sum: NodeID) -> Self {
        Self {
            node_left,
            node_right,
            node_sum,
        }
    }
}
impl DeviceUpdatable for Differential {
    fn device_update<const N: usize>(&mut self, system: &mut System<N>) {
        let state_left = system.get_state_connected(self.node_left);
        let state_right = system.get_state_connected(self.node_right);
        let state_sum = system.get_state_connected(self.node_sum);
        system.set_state_local(
            self.node_sum,
            if let Some(state_left) = state_left
                && let Some(state_right) = state_right
            {
                Some(state_left + state_right)
            } else {
                None
            },
        );
        system.set_state_local(
            self.node_right,
            if let Some(state_left) = state_left
                && let Some(state_sum) = state_sum
            {
                Some(state_sum - state_left)
            } else {
                None
            },
        );
        system.set_state_local(
            self.node_left,
            if let Some(state_right) = state_right
                && let Some(state_sum) = state_sum
            {
                Some(state_sum - state_right)
            } else {
                None
            },
        );
    }
}
pub struct GearTrain {
    node_a: NodeID,
    node_b: NodeID,
    ratio: Dimensionless<f32>,
}
impl GearTrain {
    #[inline]
    pub const fn new(node_a: NodeID, node_b: NodeID, ratio: Dimensionless<f32>) -> Self {
        Self {
            node_a,
            node_b,
            ratio,
        }
    }
    pub const fn from_teeth<const N: usize>(
        node_a: NodeID,
        node_b: NodeID,
        teeth: [f32; N],
    ) -> Self {
        let ratio = teeth[0] / teeth[N - 1];
        let direction = if N % 2 == 0 { -1.0 } else { 1.0 };
        Self::new(node_a, node_b, Dimensionless::new(ratio * direction))
    }
}
impl DeviceUpdatable for GearTrain {
    fn device_update<const N: usize>(&mut self, system: &mut System<N>) {
        system.set_state_local(
            self.node_b,
            if let Some(a_state) = system.get_state_connected(self.node_a) {
                Some(a_state * self.ratio)
            } else {
                None
            },
        );
        system.set_state_local(
            self.node_a,
            if let Some(b_state) = system.get_state_connected(self.node_b) {
                Some(b_state / self.ratio)
            } else {
                None
            },
        );
    }
}
