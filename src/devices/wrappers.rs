use super::*;
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
pub struct GetterWrapper<G, E> {
    getter: G,
    node: NodeID,
    phantom_e: PhantomData<E>,
}
//FIXME: error handling
impl<G: Getter<AngularState, E>, E: Clone + Debug> DeviceUpdatable for GetterWrapper<G, E> {
    fn device_update<const N: usize>(&mut self, system: &mut System<N>) {
        self.getter.update();
        get_and_write_to_node(&self.getter, system, self.node);
    }
}
pub struct SettableWrapper<S, E> {
    settable: S,
    node: NodeID,
    phantom_e: PhantomData<E>,
}
impl<S: Settable<AngularState, E>, E: Clone + Debug> DeviceUpdatable for SettableWrapper<S, E> {
    fn device_update<const N: usize>(&mut self, system: &mut System<N>) {
        self.settable.update();
        set_to_node_state(&mut self.settable, system, self.node);
    }
}
pub struct GetterSettableWrapper<T, E> {
    getter_settable: T,
    node: NodeID,
    phantom_e: PhantomData<E>,
}
impl<T, E> DeviceUpdatable for GetterSettableWrapper<T, E>
where
    T: Getter<AngularState, E> + Settable<AngularState, E>,
    E: Clone + Debug,
{
    fn device_update<const N: usize>(&mut self, system: &mut System<N>) {
        self.getter_settable.update();
        set_to_node_state(&mut self.getter_settable, system, self.node);
        get_and_write_to_node(&self.getter_settable, system, self.node);
    }
}
