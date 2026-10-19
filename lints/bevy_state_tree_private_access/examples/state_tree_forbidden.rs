#![allow(dead_code)]
use state_tree::{
    StateTreeActivityContext, StateTreeContext, StateTreeDomain, StateTreeEffectContext,
    StateTreeInstance,
};

fn instance<D: StateTreeDomain>(tree: &StateTreeInstance<D>, key: &str) {
    let _ = tree.data("placing");
    let _ = tree.data(key);
    let _ = tree.active_states();
    let _ = tree.active_state_names();
}

fn context<D: StateTreeDomain>(context: StateTreeContext<'_, D>) {
    let _ = context.data("placing");
}

fn effect<D: StateTreeDomain>(context: StateTreeEffectContext<'_, D>) {
    let _ = context.data("placing");
}

fn activity<D: StateTreeDomain>(context: StateTreeActivityContext<'_, D>) {
    let _ = context.data("placing");
}

fn query<D: StateTreeDomain>(instances: ecs::prelude::Query<&StateTreeInstance<D>>) {
    for tree in &instances {
        let _ = tree.data("placing");
    }
}

fn change_detection<D: StateTreeDomain>(tree: ecs::change_detection::Ref<StateTreeInstance<D>>) {
    let _ = tree.active_state_names();
}

fn main() {}
