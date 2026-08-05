// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2024-2026 UxuginPython
//!Wrappers that connect [`Getter`]s and [`Settable`]s to the device system. There are two functions
//!and three wrappers that call the functions in their [`DeviceUpdatable`] implementations. The
//!three wrappers cannot be unified into one because that would require specialization.
use super::*;
///Call the getter's `get` method and, if it returns `Ok(Some(_))`, write the value to a node in the
///system.
///
///If `get` does not return a value (`Ok(None)` or `Err(_)`), the node's state is set to `None`.
///If `get` returns an error, it is returned.
pub fn get_and_write_to_node<
    G: Getter<AngularState, E> + ?Sized,
    const N: usize,
    E: Clone + Debug,
>(
    getter: &G,
    system: &mut System<N>,
    node: NodeID,
) -> NothingOrError<E> {
    let to_set = match getter.get() {
        Ok(option) => option.map(|datum| datum.value),
        Err(error) => {
            system.set_state_local(node, None);
            return Err(error);
        }
    };
    system.set_state_local(node, to_set);
    Ok(())
}
///Write the current state of a node in the system to a settable. This uses
///[`System::get_state_connected`].
pub fn set_to_node_state<
    S: Settable<AngularState, E> + ?Sized,
    const N: usize,
    E: Clone + Debug,
>(
    settable: &mut S,
    system: &mut System<N>,
    node: NodeID,
) -> NothingOrError<E> {
    if let Some(state) = system.get_state_connected(node) {
        settable.set(state)
    } else {
        Ok(())
    }
}
macro_rules! error_handle_update {
    ($updatable: expr, $system: expr, $node: expr) => {
        if let Err(error) = $updatable.update() {
            $system.set_state_local($node, None);
            return Err(error);
        }
    };
}
///Writes a state gotten from a [`Getter`] to a node. This uses [`get_and_write_to_node`]
///internally.
pub struct GetterWrapper<G, E> {
    getter: G,
    node: NodeID,
    phantom_e: PhantomData<E>,
}
impl<G, E> GetterWrapper<G, E> {
    ///Constructor for `GetterWrapper`.
    #[inline]
    pub const fn new(node: NodeID, getter: G) -> Self {
        Self {
            getter,
            node,
            phantom_e: PhantomData,
        }
    }
}
impl<G: Getter<AngularState, E>, E: Clone + Debug> DeviceUpdatable<E> for GetterWrapper<G, E> {
    fn device_update<const N: usize>(&mut self, system: &mut System<N>) -> NothingOrError<E> {
        error_handle_update!(self.getter, system, self.node);
        get_and_write_to_node(&self.getter, system, self.node)
    }
}
///Sets a [`Settable`] to the state of a node gotten using [`System::get_state_connected`].
///This uses [`set_to_node_state`] internally.
pub struct SettableWrapper<S, E> {
    settable: S,
    node: NodeID,
    phantom_e: PhantomData<E>,
}
impl<S, E> SettableWrapper<S, E> {
    ///Constructor for `SettableWrapper`.
    #[inline]
    pub const fn new(node: NodeID, settable: S) -> Self {
        Self {
            settable,
            node,
            phantom_e: PhantomData,
        }
    }
}
impl<S: Settable<AngularState, E>, E: Clone + Debug> DeviceUpdatable<E> for SettableWrapper<S, E> {
    fn device_update<const N: usize>(&mut self, system: &mut System<N>) -> NothingOrError<E> {
        error_handle_update!(self.settable, system, self.node);
        set_to_node_state(&mut self.settable, system, self.node)
    }
}
///Combines the functionality of [`GetterWrapper`] and [`SettableWrapper`]. The getter-settable is
///`set` to the state of the node from [`System::get_state_connected`] **before** the state to be
///written with [`System::set_state_local`] is gotten with `get`. This uses both
///[`set_to_node_state`] and [`get_and_write_to_node`] internally.
pub struct GetterSettableWrapper<T, E> {
    getter_settable: T,
    node: NodeID,
    phantom_e: PhantomData<E>,
}
impl<T, E> GetterSettableWrapper<T, E> {
    ///Constructor for `GetterSettableWrapper`.
    #[inline]
    pub const fn new(node: NodeID, getter_settable: T) -> Self {
        Self {
            getter_settable,
            node,
            phantom_e: PhantomData,
        }
    }
}
impl<T, E> DeviceUpdatable<E> for GetterSettableWrapper<T, E>
where
    T: Getter<AngularState, E> + Settable<AngularState, E>,
    E: Clone + Debug,
{
    fn device_update<const N: usize>(&mut self, system: &mut System<N>) -> NothingOrError<E> {
        error_handle_update!(self.getter_settable, system, self.node);
        if let Err(error) = set_to_node_state(&mut self.getter_settable, system, self.node) {
            system.set_state_local(self.node, None);
            return Err(error);
        }
        get_and_write_to_node(&self.getter_settable, system, self.node)
    }
}
