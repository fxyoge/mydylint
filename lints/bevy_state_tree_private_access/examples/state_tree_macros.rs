#![allow(dead_code)]
use state_tree::{StateTreeDomain, StateTreeInstance};

macro_rules! framework {
    ($tree:expr) => {
        $tree.data("generated")
    };
}

macro_rules! caller_expression {
    ($expr:expr) => {
        $expr
    };
}

macro_rules! caller_method {
    ($tree:expr, $method:ident) => {
        $tree.$method()
    };
}

fn example<D: StateTreeDomain>(tree: &StateTreeInstance<D>) {
    let _ = framework!(tree);
    let _ = caller_expression!(tree.data("placing"));
    let _ = caller_expression!(StateTreeInstance::data(tree, "placing"));
    let _ = caller_method!(tree, active_states);
}

fn main() {}
