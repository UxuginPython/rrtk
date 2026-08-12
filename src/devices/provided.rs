// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2024-2026 UxuginPython
//!A few common, simple mechanical devices that work with the device system.
//!
//!All implementations of [`DeviceUpdatable`] in this module are infallible. The source of this
//!module is simple enough to be treated as an example of how to implement your own devices.
use super::*;
///Device that either connects two axles or allows them to move independently depending on how it
///is set.
pub struct Clutch {
    connected: bool,
    node_a: NodeID,
    node_b: NodeID,
}
impl Clutch {
    ///Constructor. The clutch is set open by default.
    #[inline]
    pub const fn new(node_a: NodeID, node_b: NodeID) -> Self {
        Self {
            connected: false,
            node_a,
            node_b,
        }
    }
    ///Sets the clutch open or closed. `false` sets the clutch open; `true` sets it closed.
    #[inline]
    pub const fn set_connected(&mut self, value: bool) {
        self.connected = value;
    }
}
impl<E> DeviceUpdatable<E> for Clutch {
    fn device_update<const N: usize>(&mut self, system: &mut System<N>) -> NothingOrError<E> {
        if self.connected {
            system.set_state_local(self.node_a, system.get_state_connected(self.node_b));
            system.set_state_local(self.node_b, system.get_state_connected(self.node_a));
        } else {
            system.set_state_local(self.node_a, None);
            system.set_state_local(self.node_b, None);
        }
        Ok(())
    }
}
///Device that enforces that the position, velocity, and acceleration of the sum node will equal
///the sum of the left and right nodes. In other works,
///X<sub>sum</sub>=X<sub>left</sub>+X<sub>right</sub> where X<sub>sum</sub>, X<sub>left</sub>, and
///X<sub>right</sub> are the state vectors of their respective nodes.
pub struct Differential {
    node_left: NodeID,
    node_right: NodeID,
    node_sum: NodeID,
}
impl Differential {
    ///Constructor. See struct documentation for details.
    #[inline]
    pub const fn new(node_left: NodeID, node_right: NodeID, node_sum: NodeID) -> Self {
        Self {
            node_left,
            node_right,
            node_sum,
        }
    }
}
impl<E> DeviceUpdatable<E> for Differential {
    fn device_update<const N: usize>(&mut self, system: &mut System<N>) -> NothingOrError<E> {
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
        Ok(())
    }
}
///Device that enforces that two nodes' positions, velocities, and accelerations are related by a
///constant ratio.
pub struct GearTrain {
    node_a: NodeID,
    node_b: NodeID,
    ratio: Dimensionless<f32>,
}
impl GearTrain {
    ///Constructor. The device will enforce that node A's position, velocity, and acceleration
    ///multiplied by the ratio will equal the respective value of node B.
    ///In other works, rX<sub>a</sub>=X<sub>b</sub> where r=`ratio` and X<sub>a</sub> and
    ///X<sub>b</sub> are the state vectors of their respective nodes. Use a negative value for
    ///`ratio` if the states should be inverted relative to eachother.
    #[inline]
    pub const fn new(node_a: NodeID, node_b: NodeID, ratio: Dimensionless<f32>) -> Self {
        Self {
            node_a,
            node_b,
            ratio,
        }
    }
    ///Constructor that calculates the gear ratio from the numbers of teeth on the first and last
    ///gears. If there is an even number of gears, the states will be inverted relative to
    ///eachother.
    pub const fn from_teeth<const N: usize>(
        node_a: NodeID,
        node_b: NodeID,
        teeth: [f32; N],
    ) -> Self {
        if N < 2 {
            panic!("At least 2 gears are required to construct an RRTK GearTrain.");
        }
        let ratio = teeth[0] / teeth[N - 1];
        let direction = if N % 2 == 0 { -1.0 } else { 1.0 };
        Self::new(node_a, node_b, Dimensionless::new(ratio * direction))
    }
}
impl<E> DeviceUpdatable<E> for GearTrain {
    fn device_update<const N: usize>(&mut self, system: &mut System<N>) -> NothingOrError<E> {
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
        Ok(())
    }
}
#[test]
fn ratio_from_teeth() {
    let mut system = System::<2>::new();
    let node_a = system.new_node().unwrap();
    let node_b = system.new_node().unwrap();
    //You would never create two different gear trains between the same nodes in a real system. You
    //wouldn't even have devices sharing nodes. However, this is OK for the test since nothing is
    //ever updated.
    let gear_train_1 = GearTrain::from_teeth(node_a, node_b, [100.0, 50.0]);
    assert_eq!(gear_train_1.ratio.into_inner(), -2.0);
    let gear_train_2 = GearTrain::from_teeth(node_a, node_b, [20.0, 8.0, 60.0]);
    assert_eq!(gear_train_2.ratio.into_inner(), 1.0 / 3.0);
}
#[test]
#[should_panic]
fn not_enough_teeth() {
    let mut system = System::<2>::new();
    //We don't use unwrap() because, if it failed, it would make the test return ok.
    #[rustfmt::skip]
    let node_a = if let Some(node) = system.new_node() { node } else { return; };
    #[rustfmt::skip]
    let node_b = if let Some(node) = system.new_node() { node } else { return; };
    let _gear_train = GearTrain::from_teeth(node_a, node_b, [100.0]);
}
