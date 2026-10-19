#![allow(dead_code)]
use state_tree::{StateTreeDomain, StateTreeInstance as Instance};

type Tree<D> = Instance<D>;
mod facade {
    pub use state_tree::StateTreeInstance as Controller;
}

fn qualified<D: StateTreeDomain>(tree: &Tree<D>, context: state_tree::StateTreeContext<'_, D>) {
    let _ = state_tree::StateTreeInstance::data(tree, "placing");
    let _ = Instance::active_states(tree);
    let _ = Tree::active_state_names(tree);
    let _ = facade::Controller::data(tree, "placing");
    let _ = state_tree::StateTreeEffectContext::data(&context, "placing");
}

fn function_values<D: StateTreeDomain>(tree: &Instance<D>) {
    let read = Instance::<D>::data;
    let _ = read(tree, "placing");
    let _ = Instance::<D>::active_states;
    let _ = Instance::<D>::active_state_names;
    let _ = state_tree::StateTreeContext::<D>::data;
    let _ = state_tree::StateTreeActivityContext::<D>::data;
}

fn reexported_receiver<D: StateTreeDomain>(tree: &facade::Controller<D>) {
    let _ = tree.data("placing");
}

fn main() {}
