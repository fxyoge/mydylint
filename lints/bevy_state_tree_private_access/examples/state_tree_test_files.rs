#![allow(dead_code)]
use state_tree::{StateTreeDomain, StateTreeInstance};

mod directory {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/examples/support/tests/inspection.rs"
    ));
}
mod exact {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/examples/support/tests.rs"
    ));
}
mod production {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/examples/support/helper.rs"
    ));
}
mod suffix {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/examples/support/controller_tests.rs"
    ));
}

fn example<D: StateTreeDomain>(tree: &StateTreeInstance<D>) {
    let _ = tree.active_states();
}

fn main() {}
