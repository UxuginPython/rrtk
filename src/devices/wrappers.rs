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
