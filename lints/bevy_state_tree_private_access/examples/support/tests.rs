use state_tree::{StateTreeDomain, StateTreeInstance};

fn inspect<D: StateTreeDomain>(tree: &StateTreeInstance<D>) {
    let _ = tree.data("placing");
    let _ = tree.active_states();
}
