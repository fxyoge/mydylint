use state_tree::{StateTreeContext, StateTreeDomain};

fn inspect<D: StateTreeDomain>(context: StateTreeContext<'_, D>) {
    let _ = context.data("placing");
}
